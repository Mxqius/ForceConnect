//! Writing an adapter's DNS servers on Windows.
//!
//! These calls write under `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip` and
//! fail with access denied from an unelevated process.
//!
//! # Why `SetInterfaceDnsSettings` and not the alternatives
//!
//! Against `netsh`: a process spawn per call, locale-dependent output that has
//! to be scraped, no structured error codes, and a pattern increasingly flagged
//! by endpoint security tooling. It is what this rewrite exists to leave behind.
//!
//! Against writing the registry directly: that works further back, but it makes
//! notifying the DNS Client service our problem. Registry-based tools generally
//! end up restarting `Dnscache` to force a refresh, which is heavy-handed and
//! discards NRPT rules and other resolver state that VPN clients and corporate
//! agents depend on. This API notifies the stack internally.

use std::net::IpAddr;

use windows::Win32::Foundation::NO_ERROR;
use windows::Win32::NetworkManagement::IpHelper::{
    DNS_INTERFACE_SETTINGS, DNS_INTERFACE_SETTINGS_VERSION1, DNS_SETTING_IPV6,
    DNS_SETTING_NAMESERVER, SetInterfaceDnsSettings,
};
use windows::core::PWSTR;

use forceconnect_net::adapters::{dns_config, parse_guid};
use forceconnect_net::{DnsConfig, Family, NetError};

/// Point an adapter at a fixed list of servers for one address family.
///
/// IPv4 and IPv6 are separate calls: `DNS_SETTING_IPV6` selects which stack the
/// call applies to, and the server list must match that family. Setting only
/// IPv4 while IPv6 DNS stays on DHCP is a well-known way to have Windows
/// quietly resolve around you.
pub fn set_dns(guid: &str, family: Family, servers: &[IpAddr]) -> Result<(), NetError> {
    let wanted_v6 = family == Family::V6;
    if servers.iter().any(|ip| ip.is_ipv6() != wanted_v6) {
        return Err(NetError::Api(format!(
            "server list does not match the {family:?} address family"
        )));
    }

    let list = servers
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",");

    // This binding is load-bearing. Writing
    // `PWSTR(list.encode_utf16().collect::<Vec<_>>().as_mut_ptr())` compiles
    // and produces a dangling pointer into a dropped temporary — which shows up
    // as garbage DNS servers on the user's adapter, about the worst outcome
    // this code has. The buffer must outlive the call.
    let mut wide: Vec<u16> = list.encode_utf16().chain(std::iter::once(0)).collect();

    apply(guid, family, PWSTR(wide.as_mut_ptr()))
}

/// Return an adapter to DHCP-assigned DNS for one address family.
///
/// A null `NameServer` with `DNS_SETTING_NAMESERVER` set is the documented-by-
/// convention way to clear the static list; it is what `netsh … set dns dhcp`
/// and `Set-DnsClientServerAddress -ResetServerAddresses` do underneath. The
/// reference states what the flag selects but never spells out null semantics,
/// so [`verify_cleared`] exists to confirm the effect rather than assume it.
pub fn clear_dns(guid: &str, family: Family) -> Result<(), NetError> {
    apply(guid, family, PWSTR::null())
}

fn apply(guid: &str, family: Family, name_server: PWSTR) -> Result<(), NetError> {
    let guid = parse_guid(guid)?;

    let mut flags = DNS_SETTING_NAMESERVER as u64;
    if family == Family::V6 {
        flags |= DNS_SETTING_IPV6 as u64;
    }

    // The reference is explicit: set only the fields whose option is present in
    // `Flags` and zero everything else. `..Default::default()` does that;
    // hand-filling the remaining fields would not.
    let settings = DNS_INTERFACE_SETTINGS {
        Version: DNS_INTERFACE_SETTINGS_VERSION1,
        Flags: flags,
        NameServer: name_server,
        ..Default::default()
    };

    // SAFETY: `settings` is a correctly initialized VERSION1 struct, and any
    // buffer `name_server` points at is owned by the caller for the duration of
    // this call.
    let status = unsafe { SetInterfaceDnsSettings(guid, &settings) };

    if status == NO_ERROR {
        Ok(())
    } else {
        Err(NetError::Api(format!(
            "SetInterfaceDnsSettings failed with {}",
            status.0
        )))
    }
}

/// Restore a previously captured configuration.
///
/// The distinction that matters: a `Dhcp` snapshot is restored by *clearing*
/// the static list, not by writing the servers DHCP happened to be handing out
/// at the time. Writing them back would convert a DHCP adapter to a static one
/// pinned to whatever the router said that day, and the user would never know.
pub fn restore(guid: &str, family: Family, previous: &DnsConfig) -> Result<(), NetError> {
    match previous {
        DnsConfig::Dhcp => clear_dns(guid, family),
        DnsConfig::Static(servers) => set_dns(guid, family, servers),
    }
}

/// Read back what the adapter reports after a change.
///
/// Used to confirm a write took effect rather than trusting the return code —
/// the ordering discipline for connect and disconnect depends on knowing the
/// change actually landed.
pub fn verify_cleared(guid: &str, family: Family) -> Result<bool, NetError> {
    Ok(dns_config(guid, family)?.is_dhcp())
}
