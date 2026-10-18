//! The Linux text formats the readers depend on, and how an adapter's DNS is
//! owned.
//!
//! Everything here is a pure function of its input, so it compiles on every
//! platform and its tests run wherever `cargo test` does. The code that
//! actually runs `nmcli` or reads `/proc` lives in the `adapters`, `ports` and
//! `route` modules and is Linux only.
//!
//! # Who owns an adapter's DNS
//!
//! Linux has no single per-adapter DNS setting the way Windows does. On a
//! desktop it is one of:
//!
//! - **NetworkManager**, on almost every desktop distribution. It owns the
//!   devices it manages and re-pushes their DNS on every connection or DHCP
//!   event, so writing to anything underneath it is overwritten the next time
//!   the lease renews. A device NetworkManager manages is therefore written
//!   *through* NetworkManager, even when systemd-resolved is what answers
//!   queries.
//! - **systemd-resolved** on its own, for links nothing else configures — or
//!   that systemd-networkd does, which `resolvectl` routes to correctly.
//!
//! [`DnsTarget`] records which of the two applies, in the adapter's `guid`, so
//! the write side never has to re-derive it — and so a journal entry written
//! before a crash still says how to undo it after the reboot.

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::{AdapterKind, NetError, OperStatus};

/// What an adapter's DNS is written through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsTarget<'a> {
    /// A device NetworkManager has an active connection on. Holds the device
    /// name.
    NetworkManager(&'a str),
    /// A link systemd-resolved knows about. Holds the link name.
    Resolved(&'a str),
}

const NM_PREFIX: &str = "nm:";
const RESOLVED_PREFIX: &str = "resolved:";
const UNMANAGED_PREFIX: &str = "unmanaged:";

impl<'a> DnsTarget<'a> {
    /// Read a target back out of an adapter's `guid`.
    pub fn parse(guid: &'a str) -> Result<Self, NetError> {
        if let Some(device) = guid.strip_prefix(NM_PREFIX) {
            return Ok(Self::NetworkManager(device));
        }
        if let Some(device) = guid.strip_prefix(RESOLVED_PREFIX) {
            return Ok(Self::Resolved(device));
        }

        let device = guid.strip_prefix(UNMANAGED_PREFIX).unwrap_or(guid);
        Err(NetError::Unmanaged(format!(
            "neither NetworkManager nor systemd-resolved manages DNS for {device}"
        )))
    }

    pub fn device(&self) -> &'a str {
        match self {
            Self::NetworkManager(device) | Self::Resolved(device) => device,
        }
    }
}

/// The `guid` for a device NetworkManager manages.
pub fn nm_key(device: &str) -> String {
    format!("{NM_PREFIX}{device}")
}

/// The `guid` for a link systemd-resolved manages.
pub fn resolved_key(device: &str) -> String {
    format!("{RESOLVED_PREFIX}{device}")
}

/// The `guid` for an adapter nothing manages DNS for.
pub fn unmanaged_key(device: &str) -> String {
    format!("{UNMANAGED_PREFIX}{device}")
}

/// A default route out of one interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultRoute {
    pub device: String,
    pub gateway: IpAddr,
    pub metric: u32,
}

/// `RTF_UP`, `RTF_GATEWAY` and `RTF_REJECT` from `<linux/route.h>`.
const RTF_UP: u32 = 0x0001;
const RTF_GATEWAY: u32 = 0x0002;
const RTF_REJECT: u32 = 0x0200;

/// IPv4 default routes from `/proc/net/route`.
///
/// Addresses there are the raw network-order `u32` printed as host-order hex,
/// so on a little-endian machine `192.168.1.1` reads `0101A8C0`.
/// `to_ne_bytes` undoes exactly that on whatever machine this runs on.
pub fn parse_proc_route(body: &str) -> Vec<DefaultRoute> {
    body.lines()
        .skip(1)
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let [device, destination, gateway, flags, _, _, metric, mask, ..] = fields[..] else {
                return None;
            };

            let hex = |s: &str| u32::from_str_radix(s, 16).ok();
            let flags = hex(flags)?;
            let is_default = hex(destination)? == 0 && hex(mask)? == 0;

            (is_default && flags & RTF_UP != 0 && flags & RTF_GATEWAY != 0).then(|| DefaultRoute {
                device: device.to_owned(),
                gateway: IpAddr::V4(Ipv4Addr::from(hex(gateway).unwrap_or(0).to_ne_bytes())),
                metric: metric.parse().unwrap_or(0),
            })
        })
        .collect()
}

/// IPv6 default routes from `/proc/net/ipv6_route`.
///
/// Unlike the IPv4 file this one prints addresses as plain big-endian hex. The
/// kernel also lists `lo` "unreachable" defaults here; they carry
/// `RTF_REJECT` and no gateway, and are not a way out of anything.
pub fn parse_proc_ipv6_route(body: &str) -> Vec<DefaultRoute> {
    body.lines()
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let [
                destination,
                prefix,
                _,
                _,
                next_hop,
                metric,
                _,
                _,
                flags,
                device,
            ] = fields[..]
            else {
                return None;
            };

            let flags = u32::from_str_radix(flags, 16).ok()?;
            let is_default = u128::from_str_radix(destination, 16).ok()? == 0 && prefix == "00";
            let gateway = Ipv6Addr::from(u128::from_str_radix(next_hop, 16).ok()?);

            (is_default
                && flags & RTF_UP != 0
                && flags & RTF_GATEWAY != 0
                && flags & RTF_REJECT == 0
                && !gateway.is_unspecified())
            .then(|| DefaultRoute {
                device: device.to_owned(),
                gateway: IpAddr::V6(gateway),
                metric: u32::from_str_radix(metric, 16).unwrap_or(0),
            })
        })
        .collect()
}

/// The output device from `ip -o route get <destination>`.
///
/// `ip route get` is used rather than reading the main table because it
/// applies policy routing: WireGuard's full-tunnel setup lives in a separate
/// table selected by an `ip rule`, and is invisible in `/proc/net/route`.
pub fn parse_ip_route_get(body: &str) -> Option<&str> {
    let mut tokens = body.split_whitespace();
    while let Some(token) = tokens.next() {
        if token == "dev" {
            return tokens.next();
        }
    }
    None
}

/// Strip what `resolvectl` and `nmcli` decorate a server with: a `%zone`
/// suffix, a `#server-name` for DNS-over-TLS, a non-default port.
fn parse_server(raw: &str) -> Option<IpAddr> {
    let raw = raw.split('#').next()?;

    // `[fe80::1%3]:5353` and `1.1.1.1:5353` — a port means brackets for IPv6.
    let host = if let Some(rest) = raw.strip_prefix('[') {
        rest.split(']').next()?
    } else if raw.matches(':').count() == 1 {
        raw.split(':').next()?
    } else {
        raw
    };

    host.split('%').next()?.parse().ok()
}

/// Per-link servers from `resolvectl dns`, with or without a link argument.
///
/// ```text
/// Global:
/// Link 2 (enp0s3): 10.0.2.3
/// Link 3 (wlp3s0): 192.168.1.1 fe80::1%3
/// ```
pub fn parse_resolvectl_dns(body: &str) -> HashMap<String, Vec<IpAddr>> {
    body.lines()
        .filter_map(|line| {
            let rest = line.trim().strip_prefix("Link ")?;
            let open = rest.find('(')?;
            let close = rest[open..].find("):")? + open;

            let device = rest[open + 1..close].to_owned();
            let servers = rest[close + 2..]
                .split_whitespace()
                .filter_map(parse_server)
                .collect();
            Some((device, servers))
        })
        .collect()
}

/// One device, as `nmcli device show` describes it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NmDevice {
    /// NetworkManager configured this device and has a connection active on
    /// it. A device it merely observes — `connected (externally)` — was set
    /// up by something else, and NetworkManager's DNS for it means nothing.
    pub managed: bool,
    pub dns: Vec<IpAddr>,
}

/// Undo `nmcli -t` escaping, which writes a literal `:` as `\:` (so every IPv6
/// address) and `\` as `\\`.
pub fn nm_unescape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next) = chars.next() {
                out.push(next);
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Devices from
/// `nmcli -t -f GENERAL.DEVICE,GENERAL.STATE,IP4.DNS,IP6.DNS device show`.
///
/// Must be run with `LC_ALL=C`: the state text is translated otherwise.
pub fn parse_nmcli_devices(body: &str) -> HashMap<String, NmDevice> {
    let mut devices = HashMap::new();
    let mut current: Option<(String, NmDevice)> = None;

    for line in body.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let value = nm_unescape(value);

        if key == "GENERAL.DEVICE" {
            if let Some((name, device)) = current.take() {
                devices.insert(name, device);
            }
            current = Some((value, NmDevice::default()));
            continue;
        }

        let Some((_, device)) = current.as_mut() else {
            continue;
        };

        if key == "GENERAL.STATE" {
            // `100 (connected)` is NetworkManager's own activated connection;
            // `100 (connected (externally))` is one it only watches.
            device.managed = value.starts_with("100 ") && !value.contains("externally");
        } else if key.starts_with("IP4.DNS") || key.starts_with("IP6.DNS") {
            device.dns.extend(parse_server(value.trim()));
        }
    }

    if let Some((name, device)) = current {
        devices.insert(name, device);
    }
    devices
}

/// The device → connection-UUID map from `nmcli -t -f DEVICE,CON-UUID device`.
pub fn parse_nmcli_connections(body: &str) -> HashMap<String, String> {
    body.lines()
        .filter_map(|line| {
            let (device, uuid) = line.split_once(':')?;
            let uuid = uuid.trim();
            (!uuid.is_empty()).then(|| (nm_unescape(device), uuid.to_owned()))
        })
        .collect()
}

/// A server list from `nmcli -g ipv4.dns connection show …`: comma-separated,
/// colons escaped.
pub fn parse_nmcli_server_list(body: &str) -> Vec<IpAddr> {
    nm_unescape(body.trim())
        .split([',', ' '])
        .filter_map(|s| parse_server(s.trim()))
        .collect()
}

/// What sysfs says about an interface, for [`classify`].
#[derive(Debug, Clone, Copy, Default)]
pub struct SysfsKind<'a> {
    /// `/sys/class/net/<if>/type`: an `ARPHRD_*` value.
    pub arphrd: u32,
    /// `/sys/class/net/<if>/wireless` exists.
    pub wireless: bool,
    /// `/sys/class/net/<if>/tun_flags` exists: a TUN or TAP device.
    pub tun: bool,
    /// `/sys/class/net/<if>/device` exists: backed by real hardware.
    pub hardware: bool,
    /// `DEVTYPE=` from `/sys/class/net/<if>/uevent`.
    pub devtype: Option<&'a str>,
}

/// `ARPHRD_*` values from `<linux/if_arp.h>`.
mod arphrd {
    pub const ETHER: u32 = 1;
    pub const PPP: u32 = 512;
    pub const TUNNEL: u32 = 768;
    pub const TUNNEL6: u32 = 769;
    pub const LOOPBACK: u32 = 772;
    pub const SIT: u32 = 776;
    pub const IPGRE: u32 = 778;
    pub const IP6GRE: u32 = 823;
    /// What WireGuard and TUN devices report: no link layer at all.
    pub const NONE: u32 = 65534;
}

/// Broad adapter classification from sysfs.
///
/// Bridges, veth pairs and VLANs are all `ARPHRD_ETHER` too, which is why
/// "is there hardware behind it" decides between `Ethernet` and `Other`:
/// Docker's `docker0` and every `veth…` would otherwise be listed alongside
/// the real network card.
pub fn classify(info: SysfsKind<'_>) -> AdapterKind {
    if info.arphrd == arphrd::LOOPBACK {
        return AdapterKind::Loopback;
    }
    if info.wireless || info.devtype == Some("wlan") {
        return AdapterKind::WiFi;
    }
    if info.tun || info.devtype == Some("wireguard") {
        return AdapterKind::Tunnel;
    }

    match info.arphrd {
        arphrd::PPP
        | arphrd::TUNNEL
        | arphrd::TUNNEL6
        | arphrd::SIT
        | arphrd::IPGRE
        | arphrd::IP6GRE
        | arphrd::NONE => AdapterKind::Tunnel,
        arphrd::ETHER if info.hardware && info.devtype.is_none() => AdapterKind::Ethernet,
        _ => AdapterKind::Other,
    }
}

/// `/sys/class/net/<if>/operstate`, with the flags as a tie-break.
///
/// TUN and WireGuard devices report `unknown` for their whole working life —
/// the kernel has no carrier to sense — so for those the interface flags are
/// the only signal. Treating `unknown` as down would hide the VPN that is
/// carrying every packet.
pub fn map_operstate(operstate: &str, up_and_running: bool) -> OperStatus {
    match operstate.trim() {
        "up" => OperStatus::Up,
        "down" => OperStatus::Down,
        "dormant" => OperStatus::Dormant,
        "lowerlayerdown" => OperStatus::LowerLayerDown,
        "notpresent" => OperStatus::NotPresent,
        "testing" => OperStatus::Testing,
        _ if up_and_running => OperStatus::Up,
        _ => OperStatus::Unknown,
    }
}

/// One UDP socket from `/proc/net/udp` or `/proc/net/udp6`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdpSocket {
    pub local: IpAddr,
    pub port: u16,
    pub inode: u64,
}

/// Sockets from `/proc/net/udp` or `/proc/net/udp6` — the format is the same
/// but for the address width. As in `/proc/net/route`, each 32-bit word of
/// the address is printed in host order.
pub fn parse_proc_udp(body: &str) -> Vec<UdpSocket> {
    body.lines()
        .skip(1)
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let (address, port) = fields.get(1)?.split_once(':')?;
            let inode = fields.get(9)?.parse().ok()?;

            let words: Option<Vec<[u8; 4]>> = (0..address.len() / 8)
                .map(|i| {
                    u32::from_str_radix(address.get(i * 8..i * 8 + 8)?, 16)
                        .ok()
                        .map(u32::to_ne_bytes)
                })
                .collect();
            let bytes: Vec<u8> = words?.concat();

            let local = match bytes.len() {
                4 => IpAddr::V4(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3])),
                16 => IpAddr::V6(Ipv6Addr::from(<[u8; 16]>::try_from(bytes).ok()?)),
                _ => return None,
            };

            Some(UdpSocket {
                local,
                port: u16::from_str_radix(port, 16).ok()?,
                inode,
            })
        })
        .collect()
}

/// Whether a socket bound here keeps the forwarder off `127.0.0.1`.
///
/// The wildcard does, on either family — a dual-stack `[::]` socket takes the
/// IPv4 wildcard with it, and `/proc` does not say whether `IPV6_V6ONLY` was
/// set, so it is assumed not to be. Another loopback address does *not*,
/// which is the case that matters: systemd-resolved's stub on `127.0.0.53`
/// is present on nearly every desktop and is no obstacle at all.
pub fn blocks_loopback(local: IpAddr) -> bool {
    match local {
        IpAddr::V4(v4) => v4.is_unspecified() || v4 == Ipv4Addr::LOCALHOST,
        IpAddr::V6(v6) => v6.is_unspecified() || v6.to_ipv4_mapped() == Some(Ipv4Addr::LOCALHOST),
    }
}

/// The systemd system unit a process belongs to, from `/proc/<pid>/cgroup`.
///
/// Only system services are named. A process in a user session lives under a
/// `user@1000.service`, which says nothing about what it is.
pub fn systemd_unit(cgroup: &str) -> Option<String> {
    cgroup.lines().find_map(|line| {
        let path = line.rsplit(':').next()?;
        let rest = path.split("/system.slice/").nth(1)?;
        let unit = rest.split('/').next()?;
        unit.ends_with(".service").then(|| unit.to_owned())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    #[test]
    fn targets_round_trip_through_the_guid() {
        assert_eq!(
            DnsTarget::parse(&nm_key("wlp3s0")).unwrap(),
            DnsTarget::NetworkManager("wlp3s0")
        );
        assert_eq!(
            DnsTarget::parse(&resolved_key("wg0")).unwrap(),
            DnsTarget::Resolved("wg0")
        );

        // The refusal names the device, so the user knows which one.
        let err = DnsTarget::parse(&unmanaged_key("eth1")).unwrap_err();
        assert!(err.to_string().contains("eth1"), "{err}");
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn ipv4_default_routes_come_out_of_proc() {
        let body = "\
Iface\tDestination\tGateway \tFlags\tRefCnt\tUse\tMetric\tMask\t\tMTU\tWindow\tIRTT
wlp3s0\t00000000\t0101A8C0\t0003\t0\t0\t600\t00000000\t0\t0\t0
wlp3s0\t0001A8C0\t00000000\t0001\t0\t0\t600\t00FFFFFF\t0\t0\t0
docker0\t000011AC\t00000000\t0001\t0\t0\t0\t0000FFFF\t0\t0\t0
";
        let routes = parse_proc_route(body);
        assert_eq!(
            routes,
            vec![DefaultRoute {
                device: "wlp3s0".into(),
                gateway: ip("192.168.1.1"),
                metric: 600,
            }]
        );
    }

    #[test]
    fn ipv6_default_routes_skip_the_unreachable_loopback_entries() {
        let body = "\
fe800000000000000000000000000000 40 00000000000000000000000000000000 00 00000000000000000000000000000000 00000100 00000001 00000000 00000001 wlp3s0
00000000000000000000000000000000 00 00000000000000000000000000000000 00 fe80000000000000021122fffe334455 00000400 00000001 00000000 00000003 wlp3s0
00000000000000000000000000000000 00 00000000000000000000000000000000 00 00000000000000000000000000000000 ffffffff 00000001 00000000 00200200       lo
";
        let routes = parse_proc_ipv6_route(body);
        assert_eq!(routes.len(), 1);
        assert_eq!(routes[0].device, "wlp3s0");
        assert_eq!(routes[0].gateway, ip("fe80::211:22ff:fe33:4455"));
        assert_eq!(routes[0].metric, 1024);
    }

    #[test]
    fn route_get_names_the_output_device() {
        let body = "1.1.1.1 via 192.168.1.1 dev wlp3s0 src 192.168.1.20 uid 1000 \\    cache ";
        assert_eq!(parse_ip_route_get(body), Some("wlp3s0"));

        // WireGuard full tunnel: no `via`, and the device is the tunnel.
        let wg = "1.1.1.1 dev wg0 table 51820 src 10.2.0.2 uid 1000 \\    cache ";
        assert_eq!(parse_ip_route_get(wg), Some("wg0"));

        assert_eq!(
            parse_ip_route_get("RTNETLINK answers: Network is unreachable"),
            None
        );
    }

    #[test]
    fn resolvectl_servers_are_read_per_link_and_undecorated() {
        let body = "\
Global:
Link 2 (enp0s3): 10.0.2.3
Link 3 (wlp3s0): 192.168.1.1 fe80::1%3 1.1.1.1#cloudflare-dns.com
Link 4 (wg0):
Link 5 (tun0): [fd00::53]:5353 10.8.0.1:53
";
        let links = parse_resolvectl_dns(body);
        assert_eq!(links["enp0s3"], vec![ip("10.0.2.3")]);
        assert_eq!(
            links["wlp3s0"],
            vec![ip("192.168.1.1"), ip("fe80::1"), ip("1.1.1.1")]
        );
        assert!(links["wg0"].is_empty());
        assert_eq!(links["tun0"], vec![ip("fd00::53"), ip("10.8.0.1")]);
        assert!(!links.contains_key("Global"));
    }

    #[test]
    fn nmcli_devices_distinguish_managed_from_observed() {
        let body = "\
GENERAL.DEVICE:wlp3s0
GENERAL.STATE:100 (connected)
IP4.DNS[1]:192.168.1.1
IP6.DNS[1]:fe80\\:\\:1
GENERAL.DEVICE:docker0
GENERAL.STATE:100 (connected (externally))
GENERAL.DEVICE:enp0s31f6
GENERAL.STATE:20 (unavailable)
GENERAL.DEVICE:lo
GENERAL.STATE:10 (unmanaged)
";
        let devices = parse_nmcli_devices(body);
        assert_eq!(
            devices["wlp3s0"],
            NmDevice {
                managed: true,
                dns: vec![ip("192.168.1.1"), ip("fe80::1")],
            }
        );
        assert!(!devices["docker0"].managed);
        assert!(!devices["enp0s31f6"].managed);
        assert!(!devices["lo"].managed);
    }

    #[test]
    fn nmcli_escaping_is_undone() {
        assert_eq!(nm_unescape(r"2606\:4700\:\:1111"), "2606:4700::1111");
        assert_eq!(nm_unescape(r"a\\b"), r"a\b");

        assert_eq!(
            parse_nmcli_server_list("1.1.1.1,2606\\:4700\\:\\:1111\n"),
            vec![ip("1.1.1.1"), ip("2606:4700::1111")]
        );
        assert!(parse_nmcli_server_list("\n").is_empty());

        let connections =
            parse_nmcli_connections("wlp3s0:6b0e4c9e-3a35-4c3e-9b5b-4d6b2f0e1a2b\nlo:\n");
        assert_eq!(connections.len(), 1);
        assert_eq!(
            connections["wlp3s0"],
            "6b0e4c9e-3a35-4c3e-9b5b-4d6b2f0e1a2b"
        );
    }

    #[test]
    fn classification_keeps_virtual_ethernet_out_of_the_ethernet_group() {
        let nic = SysfsKind {
            arphrd: arphrd::ETHER,
            hardware: true,
            ..Default::default()
        };
        assert_eq!(classify(nic), AdapterKind::Ethernet);

        let wifi = SysfsKind {
            wireless: true,
            ..nic
        };
        assert_eq!(classify(wifi), AdapterKind::WiFi);

        let bridge = SysfsKind {
            arphrd: arphrd::ETHER,
            devtype: Some("bridge"),
            ..Default::default()
        };
        assert_eq!(classify(bridge), AdapterKind::Other);

        let veth = SysfsKind {
            arphrd: arphrd::ETHER,
            ..Default::default()
        };
        assert_eq!(classify(veth), AdapterKind::Other);

        let wireguard = SysfsKind {
            arphrd: arphrd::NONE,
            devtype: Some("wireguard"),
            ..Default::default()
        };
        assert_eq!(classify(wireguard), AdapterKind::Tunnel);

        let tap = SysfsKind {
            arphrd: arphrd::ETHER,
            tun: true,
            ..Default::default()
        };
        assert_eq!(classify(tap), AdapterKind::Tunnel);

        let lo = SysfsKind {
            arphrd: arphrd::LOOPBACK,
            ..Default::default()
        };
        assert_eq!(classify(lo), AdapterKind::Loopback);
    }

    #[test]
    fn an_unknown_operstate_with_a_running_link_is_up() {
        // The shape of every WireGuard interface ever.
        assert_eq!(map_operstate("unknown\n", true), OperStatus::Up);
        assert_eq!(map_operstate("unknown", false), OperStatus::Unknown);
        assert_eq!(map_operstate("up\n", false), OperStatus::Up);
        assert_eq!(map_operstate("down", true), OperStatus::Down);
    }

    #[test]
    #[cfg(target_endian = "little")]
    fn udp_sockets_come_out_of_proc_on_both_families() {
        let v4 = "\
  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode ref pointer drops
 1031: 3500007F:0035 00000000:0000 07 00000000:00000000 00:00000000 00000000   991        0 21305 2 0000000000000000 0
 1032: 0100007F:0035 00000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 31337 2 0000000000000000 0
";
        let sockets = parse_proc_udp(v4);
        assert_eq!(
            sockets,
            vec![
                UdpSocket {
                    local: ip("127.0.0.53"),
                    port: 53,
                    inode: 21305
                },
                UdpSocket {
                    local: ip("127.0.0.1"),
                    port: 53,
                    inode: 31337
                },
            ]
        );

        let v6 = "\
  sl  local_address                         remote_address                        st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode ref pointer drops
  496: 00000000000000000000000000000000:0035 00000000000000000000000000000000:0000 07 00000000:00000000 00:00000000 00000000     0        0 4242 2 0000000000000000 0
";
        let sockets = parse_proc_udp(v6);
        assert_eq!(sockets[0].local, ip("::"));
        assert_eq!(sockets[0].port, 53);
        assert_eq!(sockets[0].inode, 4242);
    }

    #[test]
    fn the_resolved_stub_does_not_block_the_forwarder() {
        assert!(!blocks_loopback(ip("127.0.0.53")));
        assert!(!blocks_loopback(ip("127.0.0.54")));
        assert!(!blocks_loopback(ip("192.168.122.1")));
        assert!(blocks_loopback(ip("127.0.0.1")));
        assert!(blocks_loopback(ip("0.0.0.0")));
        assert!(blocks_loopback(ip("::")));
        assert!(blocks_loopback(ip("::ffff:127.0.0.1")));
    }

    #[test]
    fn only_system_units_are_named() {
        assert_eq!(
            systemd_unit("0::/system.slice/dnsmasq.service\n").as_deref(),
            Some("dnsmasq.service")
        );
        assert_eq!(
            systemd_unit("12:pids:/system.slice/systemd-resolved.service\n0::/\n").as_deref(),
            Some("systemd-resolved.service")
        );
        assert_eq!(
            systemd_unit("0::/user.slice/user-1000.slice/user@1000.service/app.slice/x.scope"),
            None
        );
    }
}
