//! Adapter enumeration via the IP Helper API.
//!
//! All of this is unprivileged — `GetAdaptersAddresses` and
//! `GetInterfaceDnsSettings` are read-only calls any user can make. Nothing
//! here belongs in the elevated helper.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use windows::Win32::Foundation::{ERROR_BUFFER_OVERFLOW, ERROR_SUCCESS, NO_ERROR};
use windows::Win32::NetworkManagement::IpHelper::{
    DNS_INTERFACE_SETTINGS, FreeInterfaceDnsSettings, GAA_FLAG_INCLUDE_GATEWAYS,
    GAA_FLAG_SKIP_ANYCAST, GAA_FLAG_SKIP_MULTICAST, GetAdaptersAddresses, GetInterfaceDnsSettings,
    IP_ADAPTER_ADDRESSES_LH,
};
use windows::Win32::Networking::WinSock::{
    AF_INET, AF_INET6, AF_UNSPEC, SOCKADDR, SOCKADDR_IN, SOCKADDR_IN6,
};
use windows::core::{GUID, PCWSTR, PWSTR};

use crate::{Adapter, AdapterKind, DnsConfig, Family, InterfaceLuid, NetError, OperStatus};

/// `IANA ifType` values we care about. Defined here rather than imported so the
/// mapping below reads as a single table.
mod if_type {
    pub const ETHERNET: u32 = 6;
    pub const PPP: u32 = 23;
    pub const SOFTWARE_LOOPBACK: u32 = 24;
    /// What WinTUN, TAP, and OpenVPN DCO adapters actually report — *not*
    /// `TUNNEL`. Verified against a machine running ExpressVPN and OpenVPN:
    /// every VPN adapter present came back as 53.
    pub const PROP_VIRTUAL: u32 = 53;
    pub const IEEE80211: u32 = 71;
    pub const TUNNEL: u32 = 131;
}

/// `DNS_INTERFACE_SETTINGS_VERSION1`. Later versions carry DoH profile fields we
/// do not use — VERSION1 has the widest support.
const DNS_INTERFACE_SETTINGS_VERSION1: u32 = 1;

/// Enumerate every adapter the system knows about.
pub fn list() -> Result<Vec<Adapter>, NetError> {
    let buffer = fetch_adapter_buffer()?;

    let mut adapters = Vec::new();
    // SAFETY: `buffer` holds a valid linked list written by the OS and outlives
    // this loop; every pointer dereferenced below is checked non-null first.
    unsafe {
        let mut current = buffer.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;
        while !current.is_null() {
            adapters.push(read_adapter(&*current));
            current = (*current).Next;
        }
    }

    Ok(adapters)
}

/// Call `GetAdaptersAddresses` into a right-sized buffer.
///
/// The size is probed first, then the call is retried: the adapter set can
/// change between the two calls, which is exactly what `ERROR_BUFFER_OVERFLOW`
/// on the second call means. Three attempts is plenty — more than that and
/// something else is wrong.
fn fetch_adapter_buffer() -> Result<Vec<u8>, NetError> {
    const FLAGS: u32 =
        GAA_FLAG_INCLUDE_GATEWAYS.0 | GAA_FLAG_SKIP_ANYCAST.0 | GAA_FLAG_SKIP_MULTICAST.0;

    let mut size: u32 = 16 * 1024;

    for _ in 0..3 {
        // Over-align for IP_ADAPTER_ADDRESSES_LH by allocating u64s.
        let mut buffer = vec![0u64; (size as usize).div_ceil(8)];
        let ptr = buffer.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;

        // SAFETY: `ptr` addresses at least `size` writable bytes, and `size` is
        // updated by the call to reflect what was needed.
        let result = unsafe {
            GetAdaptersAddresses(
                AF_UNSPEC.0 as u32,
                windows::Win32::NetworkManagement::IpHelper::GET_ADAPTERS_ADDRESSES_FLAGS(FLAGS),
                None,
                Some(ptr),
                &mut size,
            )
        };

        match result {
            r if r == ERROR_SUCCESS.0 || r == NO_ERROR.0 => {
                let bytes = unsafe {
                    std::slice::from_raw_parts(buffer.as_ptr() as *const u8, buffer.len() * 8)
                };
                return Ok(bytes.to_vec());
            }
            r if r == ERROR_BUFFER_OVERFLOW.0 => continue,
            r => {
                return Err(NetError::Api(format!(
                    "GetAdaptersAddresses failed with {r}"
                )));
            }
        }
    }

    Err(NetError::Api(
        "GetAdaptersAddresses kept reporting a too-small buffer".into(),
    ))
}

/// # Safety
///
/// `raw` must be a valid entry from a buffer that outlives this call.
unsafe fn read_adapter(raw: &IP_ADAPTER_ADDRESSES_LH) -> Adapter {
    unsafe {
        let luid = InterfaceLuid(raw.Luid.Value);
        let index = raw.Anonymous1.Anonymous.IfIndex;

        Adapter {
            luid,
            guid: pstr_to_string(raw.AdapterName.0),
            index,
            friendly_name: pwstr_to_string(raw.FriendlyName),
            description: pwstr_to_string(raw.Description),
            kind: classify(raw.IfType),
            status: map_status(raw.OperStatus.0),
            mac: read_mac(&raw.PhysicalAddress, raw.PhysicalAddressLength),
            link_speed_bps: raw.TransmitLinkSpeed,
            unicast: collect_unicast(raw),
            gateways: collect_gateways(raw),
            dns_v4: collect_dns(raw, Family::V4),
            dns_v6: collect_dns(raw, Family::V6),
            metric_v4: raw.Ipv4Metric,
            metric_v6: raw.Ipv6Metric,
        }
    }
}

fn classify(if_type: u32) -> AdapterKind {
    match if_type {
        if_type::ETHERNET => AdapterKind::Ethernet,
        if_type::IEEE80211 => AdapterKind::WiFi,
        if_type::SOFTWARE_LOOPBACK => AdapterKind::Loopback,
        // VPN clients present as any of these, and they must stay visible: a
        // tunnel routinely outranks the physical adapter carrying it.
        if_type::TUNNEL | if_type::PPP | if_type::PROP_VIRTUAL => AdapterKind::Tunnel,
        _ => AdapterKind::Other,
    }
}

fn map_status(raw: i32) -> OperStatus {
    match raw {
        1 => OperStatus::Up,
        2 => OperStatus::Down,
        3 => OperStatus::Testing,
        5 => OperStatus::Dormant,
        6 => OperStatus::NotPresent,
        7 => OperStatus::LowerLayerDown,
        _ => OperStatus::Unknown,
    }
}

fn read_mac(bytes: &[u8; 8], len: u32) -> Option<String> {
    let len = (len as usize).min(bytes.len());
    if len == 0 {
        return None;
    }

    Some(
        bytes[..len]
            .iter()
            .map(|b| format!("{b:02X}"))
            .collect::<Vec<_>>()
            .join(":"),
    )
}

unsafe fn collect_unicast(raw: &IP_ADAPTER_ADDRESSES_LH) -> Vec<IpAddr> {
    let mut out = Vec::new();
    unsafe {
        let mut node = raw.FirstUnicastAddress;
        while !node.is_null() {
            if let Some(ip) = sockaddr_to_ip((*node).Address.lpSockaddr) {
                out.push(ip);
            }
            node = (*node).Next;
        }
    }
    out
}

unsafe fn collect_gateways(raw: &IP_ADAPTER_ADDRESSES_LH) -> Vec<IpAddr> {
    let mut out = Vec::new();
    unsafe {
        let mut node = raw.FirstGatewayAddress;
        while !node.is_null() {
            if let Some(ip) = sockaddr_to_ip((*node).Address.lpSockaddr) {
                out.push(ip);
            }
            node = (*node).Next;
        }
    }
    out
}

unsafe fn collect_dns(raw: &IP_ADAPTER_ADDRESSES_LH, family: Family) -> Vec<IpAddr> {
    let mut out = Vec::new();
    unsafe {
        let mut node = raw.FirstDnsServerAddress;
        while !node.is_null() {
            if let Some(ip) = sockaddr_to_ip((*node).Address.lpSockaddr) {
                let matches = match family {
                    Family::V4 => ip.is_ipv4(),
                    Family::V6 => ip.is_ipv6(),
                };
                if matches {
                    out.push(ip);
                }
            }
            node = (*node).Next;
        }
    }
    out
}

/// # Safety
///
/// `addr` must be null or point to a `SOCKADDR` whose `sa_family` correctly
/// describes the storage behind it.
unsafe fn sockaddr_to_ip(addr: *const SOCKADDR) -> Option<IpAddr> {
    if addr.is_null() {
        return None;
    }

    unsafe {
        match (*addr).sa_family {
            AF_INET => {
                let v4 = &*(addr as *const SOCKADDR_IN);
                Some(IpAddr::V4(Ipv4Addr::from(
                    v4.sin_addr.S_un.S_addr.to_ne_bytes(),
                )))
            }
            AF_INET6 => {
                let v6 = &*(addr as *const SOCKADDR_IN6);
                Some(IpAddr::V6(Ipv6Addr::from(v6.sin6_addr.u.Byte)))
            }
            _ => None,
        }
    }
}

unsafe fn pwstr_to_string(value: PWSTR) -> String {
    if value.is_null() {
        return String::new();
    }
    unsafe { value.to_string().unwrap_or_default() }
}

unsafe fn pstr_to_string(value: *mut u8) -> String {
    if value.is_null() {
        return String::new();
    }

    unsafe {
        let mut len = 0usize;
        while *value.add(len) != 0 {
            len += 1;
        }
        String::from_utf8_lossy(std::slice::from_raw_parts(value, len)).into_owned()
    }
}

/// Read how an adapter is currently configured for one address family.
///
/// This is the *only* correct way to tell DHCP-assigned DNS from statically
/// configured DNS. `IP_ADAPTER_DHCP_ENABLED` answers a different question — it
/// describes the IP address, and an adapter with a DHCP address and static DNS
/// is exactly the state this app creates.
pub fn dns_config(guid: &str, family: Family) -> Result<DnsConfig, NetError> {
    let guid = parse_guid(guid)?;

    let mut settings = DNS_INTERFACE_SETTINGS {
        Version: DNS_INTERFACE_SETTINGS_VERSION1,
        ..Default::default()
    };

    // SAFETY: `settings` is a correctly initialized VERSION1 struct with only
    // `Version` set, as the API requires for a read.
    let status = unsafe { GetInterfaceDnsSettings(guid, &mut settings) };
    if status != NO_ERROR {
        return Err(NetError::Api(format!(
            "GetInterfaceDnsSettings failed with {}",
            status.0
        )));
    }

    let raw = unsafe { pcwstr_to_string(PCWSTR(settings.NameServer.0)) };
    // The struct owns heap memory the API allocated; it must be handed back.
    unsafe { FreeInterfaceDnsSettings(&mut settings) };

    let wanted_v6 = family == Family::V6;
    let servers: Vec<IpAddr> = raw
        .split([',', ' '])
        .filter(|s| !s.trim().is_empty())
        .filter_map(|s| s.trim().parse::<IpAddr>().ok())
        .filter(|ip| ip.is_ipv6() == wanted_v6)
        .collect();

    if servers.is_empty() {
        Ok(DnsConfig::Dhcp)
    } else {
        Ok(DnsConfig::Static(servers))
    }
}

unsafe fn pcwstr_to_string(value: PCWSTR) -> String {
    if value.is_null() {
        return String::new();
    }
    unsafe { value.to_string().unwrap_or_default() }
}

/// Parse the `{XXXXXXXX-XXXX-...}` form `AdapterName` reports.
pub fn parse_guid(value: &str) -> Result<GUID, NetError> {
    let trimmed = value.trim_matches(['{', '}']);
    GUID::try_from(trimmed).map_err(|_| NetError::BadGuid(value.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guid_parses_with_or_without_braces() {
        let bare = "0d8d0e2a-1e2f-4b3c-9a7d-1f2e3d4c5b6a";
        let braced = format!("{{{bare}}}");
        assert_eq!(parse_guid(bare).unwrap(), parse_guid(&braced).unwrap());
        assert!(parse_guid("not-a-guid").is_err());
    }

    #[test]
    fn mac_formatting_respects_the_reported_length() {
        // The buffer is always 8 bytes; only PhysicalAddressLength of it is
        // real. Formatting all 8 would append phantom `00` octets.
        let bytes = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0, 0];
        assert_eq!(read_mac(&bytes, 6).unwrap(), "AA:BB:CC:DD:EE:FF");
        assert_eq!(read_mac(&bytes, 0), None);
        // A length longer than the buffer must not panic.
        assert_eq!(read_mac(&bytes, 99).unwrap().split(':').count(), 8);
    }

    /// Enumerates the machine this runs on, so it is opt-in:
    /// `cargo test -p forceconnect-net -- --ignored --nocapture`
    #[test]
    #[ignore = "reads live system state"]
    fn enumerate_this_machine() {
        let adapters = list().expect("enumeration should succeed");
        assert!(
            !adapters.is_empty(),
            "every machine has at least a loopback"
        );

        for a in &adapters {
            println!(
                "[{:>3}] {:<38} {:?} {:?} metric v4={} v6={}",
                a.index, a.friendly_name, a.kind, a.status, a.metric_v4, a.metric_v6
            );
            println!(
                "      guid={} mac={:?}",
                a.guid,
                a.mac.as_deref().unwrap_or("-")
            );
            if !a.unicast.is_empty() {
                println!("      addrs={:?}", a.unicast);
            }
            if !a.gateways.is_empty() {
                println!("      gateways={:?}", a.gateways);
            }
            if !a.dns_v4.is_empty() || !a.dns_v6.is_empty() {
                println!("      dns v4={:?} v6={:?}", a.dns_v4, a.dns_v6);
            }
            if a.status == OperStatus::Up {
                match dns_config(&a.guid, Family::V4) {
                    Ok(cfg) => println!("      v4 config={cfg:?}"),
                    Err(err) => println!("      v4 config error: {err}"),
                }
            }
        }

        let best = crate::route::best_adapter(&adapters, Family::V4);
        println!("\nbest v4 adapter: {:?}", best.map(|a| &a.friendly_name));
    }
}
