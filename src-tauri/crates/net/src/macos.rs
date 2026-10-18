//! The macOS text formats the readers depend on, and how an adapter's DNS is
//! owned.
//!
//! Everything here is a pure function of its input, so it compiles on every
//! platform and its tests run wherever `cargo test` does. The code that runs
//! `networksetup` and `scutil` lives in the `adapters`, `ports` and `route`
//! modules and is macOS only.
//!
//! # Interfaces and network services
//!
//! macOS does not configure DNS on an interface. It configures it on a
//! *network service* — "Wi-Fi", "USB 10/100/1000 LAN" — which is bound to an
//! interface (`en0`) and is what System Settings shows. `networksetup` takes
//! service names, so the write key for an adapter is its service.
//!
//! Interfaces with no service — the system's own `utun` tunnels, `awdl0`,
//! bridges — are still listed, because routing decisions involve them, but
//! their DNS cannot be set through any supported interface and the write side
//! says so rather than guessing.

use std::collections::HashMap;
use std::net::IpAddr;

use crate::{AdapterKind, NetError};

/// What an adapter's DNS is written through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsTarget<'a> {
    /// A network service, by name.
    Service(&'a str),
}

const SERVICE_PREFIX: &str = "service:";
const DEVICE_PREFIX: &str = "device:";

impl<'a> DnsTarget<'a> {
    /// Read a target back out of an adapter's `guid`.
    pub fn parse(guid: &'a str) -> Result<Self, NetError> {
        if let Some(service) = guid.strip_prefix(SERVICE_PREFIX) {
            return Ok(Self::Service(service));
        }

        let device = guid.strip_prefix(DEVICE_PREFIX).unwrap_or(guid);
        Err(NetError::Unmanaged(format!(
            "{device} is not part of any macOS network service, so its DNS cannot be set"
        )))
    }
}

/// The `guid` for an interface bound to a network service.
pub fn service_key(service: &str) -> String {
    format!("{SERVICE_PREFIX}{service}")
}

/// The `guid` for an interface with no network service.
pub fn device_key(device: &str) -> String {
    format!("{DEVICE_PREFIX}{device}")
}

/// One entry from `networksetup -listnetworkserviceorder`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    pub name: String,
    pub hardware_port: String,
    /// Empty for services with no interface of their own — most VPN
    /// configurations.
    pub device: Option<String>,
    pub enabled: bool,
}

/// Services in priority order.
///
/// ```text
/// An asterisk (*) denotes that a network service is disabled.
/// (1) Wi-Fi
/// (Hardware Port: Wi-Fi, Device: en0)
///
/// (*) Bluetooth PAN
/// (Hardware Port: Bluetooth PAN, Device: en6)
/// ```
///
/// The order is what macOS uses to pick the primary service, so it stands in
/// for the interface metric Windows and Linux have and macOS does not.
pub fn parse_service_order(body: &str) -> Vec<Service> {
    let mut services = Vec::new();
    let mut pending: Option<(String, bool)> = None;

    for line in body.lines() {
        let line = line.trim();

        if let Some(rest) = line.strip_prefix("(Hardware Port: ") {
            let Some((name, enabled)) = pending.take() else {
                continue;
            };
            let rest = rest.strip_suffix(')').unwrap_or(rest);
            let (port, device) = match rest.rfind(", Device: ") {
                Some(at) => (&rest[..at], rest[at + ", Device: ".len()..].trim()),
                None => (rest, ""),
            };

            services.push(Service {
                name,
                hardware_port: port.to_owned(),
                device: (!device.is_empty()).then(|| device.to_owned()),
                enabled,
            });
        } else if let Some(rest) = line.strip_prefix('(')
            && let Some((marker, name)) = rest.split_once(") ")
            && (marker == "*" || marker.chars().all(|c| c.is_ascii_digit()))
        {
            pending = Some((name.to_owned(), marker != "*"));
        }
    }

    services
}

/// A service's statically configured servers from
/// `networksetup -getdnsservers <service>`.
///
/// No servers is reported as a sentence rather than as empty output —
/// "There aren't any DNS Servers set on Wi-Fi." — and means the service takes
/// whatever DHCP hands it.
pub fn parse_getdnsservers(body: &str) -> Vec<IpAddr> {
    body.lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect()
}

/// The servers each interface is actually using, from `scutil --dns`.
///
/// Read from the scoped section, which has one resolver per interface; the
/// unscoped section mixes in `mdns` and `.local` resolvers that are not
/// anybody's configured DNS.
pub fn parse_scutil_dns(body: &str) -> HashMap<String, Vec<IpAddr>> {
    let scoped = body
        .find("DNS configuration (for scoped queries)")
        .map_or(body, |at| &body[at..]);

    let mut out: HashMap<String, Vec<IpAddr>> = HashMap::new();
    let mut servers: Vec<IpAddr> = Vec::new();

    let mut flush = |servers: &mut Vec<IpAddr>, device: Option<&str>| {
        if let Some(device) = device {
            let entry = out.entry(device.to_owned()).or_default();
            for server in servers.drain(..) {
                if !entry.contains(&server) {
                    entry.push(server);
                }
            }
        }
        servers.clear();
    };

    let mut device: Option<String> = None;
    for line in scoped.lines() {
        let line = line.trim();
        if line.starts_with("resolver #") {
            flush(&mut servers, device.as_deref());
            device = None;
            continue;
        }

        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let (key, value) = (key.trim(), value.trim());

        if key.starts_with("nameserver[") {
            if let Ok(ip) = value.split('%').next().unwrap_or(value).parse() {
                servers.push(ip);
            }
        } else if key == "if_index" {
            // `6 (en0)`
            device = value
                .split_once('(')
                .and_then(|(_, rest)| rest.strip_suffix(')'))
                .map(str::to_owned);
        }
    }
    flush(&mut servers, device.as_deref());

    out
}

/// Default gateways per interface from `netstat -rn -f inet` (or `inet6`).
///
/// ```text
/// Destination        Gateway            Flags               Netif Expire
/// default            192.168.1.1        UGScg                 en0
/// default            link#18            UCSIg               utun3
/// ```
///
/// `link#N` gateways are interface routes with no next hop and are skipped.
/// A link-local IPv6 gateway carries its scope, `fe80::1%en0`, which is
/// dropped.
pub fn parse_netstat_defaults(body: &str) -> Vec<(String, IpAddr)> {
    body.lines()
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            let [destination, gateway, _flags, netif, ..] = fields[..] else {
                return None;
            };
            if destination != "default" {
                return None;
            }

            let gateway: IpAddr = gateway.split('%').next()?.parse().ok()?;
            Some((netif.to_owned(), gateway))
        })
        .collect()
}

/// The `interface:` line from `route -n get <destination>`.
pub fn parse_route_get(body: &str) -> Option<&str> {
    body.lines().find_map(|line| {
        line.trim()
            .strip_prefix("interface:")
            .map(str::trim)
            .filter(|s| !s.is_empty())
    })
}

/// The first process holding a conflicting UDP socket, from
/// `lsof -nP -iUDP:<port> -Fpcn`.
///
/// `-iUDP:53` also matches sockets whose *remote* end is port 53 — every
/// stub resolver mid-query — so only the local half of each name is
/// considered, and only the addresses that block a loopback bind.
pub fn parse_lsof(body: &str, port: u16) -> Option<(u32, String)> {
    let mut pid = None;
    let mut command = String::new();

    for line in body.lines() {
        let Some((tag, value)) = line.split_at_checked(1) else {
            continue;
        };
        match tag {
            "p" => {
                pid = value.parse().ok();
                command.clear();
            }
            "c" => command = value.to_owned(),
            "n" => {
                let local = value.split("->").next().unwrap_or(value);
                let Some((host, held)) = local.rsplit_once(':') else {
                    continue;
                };
                let blocks = matches!(host, "*" | "127.0.0.1" | "0.0.0.0" | "[::]" | "[::1]");
                if blocks && held.parse() == Ok(port) {
                    return pid.map(|pid| (pid, command.clone()));
                }
            }
            _ => {}
        }
    }

    None
}

/// Broad adapter classification from the hardware port and BSD name.
pub fn classify(hardware_port: Option<&str>, device: &str) -> AdapterKind {
    const TUNNELS: [&str; 7] = ["utun", "ipsec", "ppp", "gif", "stf", "tun", "tap"];

    if device.starts_with("lo") {
        return AdapterKind::Loopback;
    }
    if TUNNELS.iter().any(|prefix| device.starts_with(prefix)) {
        return AdapterKind::Tunnel;
    }

    match hardware_port {
        Some("Wi-Fi" | "AirPort") => AdapterKind::WiFi,
        Some(port) if port.contains("Bridge") => AdapterKind::Other,
        Some(port) if port.contains("Ethernet") || port.contains("LAN") => AdapterKind::Ethernet,
        _ => AdapterKind::Other,
    }
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
            DnsTarget::parse(&service_key("USB 10/100/1000 LAN")).unwrap(),
            DnsTarget::Service("USB 10/100/1000 LAN")
        );

        let err = DnsTarget::parse(&device_key("utun3")).unwrap_err();
        assert!(err.to_string().contains("utun3"), "{err}");
    }

    #[test]
    fn service_order_keeps_order_devices_and_the_disabled_marker() {
        let body = "\
An asterisk (*) denotes that a network service is disabled.
(1) Wi-Fi
(Hardware Port: Wi-Fi, Device: en0)

(2) Thunderbolt Bridge
(Hardware Port: Thunderbolt Bridge, Device: bridge0)

(*) Bluetooth PAN
(Hardware Port: Bluetooth PAN, Device: en6)

(3) Work VPN (IKEv2)
(Hardware Port: com.example.vpn, Device: )
";
        let services = parse_service_order(body);
        assert_eq!(services.len(), 4);

        assert_eq!(services[0].name, "Wi-Fi");
        assert_eq!(services[0].device.as_deref(), Some("en0"));
        assert!(services[0].enabled);

        assert_eq!(services[1].hardware_port, "Thunderbolt Bridge");

        assert_eq!(services[2].name, "Bluetooth PAN");
        assert!(!services[2].enabled);

        // Parentheses in a service name are not a marker.
        assert_eq!(services[3].name, "Work VPN (IKEv2)");
        assert_eq!(services[3].device, None);
    }

    #[test]
    fn no_static_servers_is_a_sentence_not_an_empty_list() {
        assert!(parse_getdnsservers("There aren't any DNS Servers set on Wi-Fi.\n").is_empty());
        assert_eq!(
            parse_getdnsservers("1.1.1.1\n2606:4700:4700::1111\n"),
            vec![ip("1.1.1.1"), ip("2606:4700:4700::1111")]
        );
    }

    #[test]
    fn scutil_servers_come_from_the_scoped_section() {
        let body = "\
DNS configuration

resolver #1
  search domain[0] : lan
  nameserver[0] : 192.168.1.1
  if_index : 6 (en0)
  flags    : Request A records
  reach    : 0x00020002 (Reachable,Directly Reachable Address)

resolver #2
  domain   : local
  options  : mdns
  timeout  : 5
  order    : 300000

DNS configuration (for scoped queries)

resolver #1
  search domain[0] : lan
  nameserver[0] : 192.168.1.1
  nameserver[1] : fe80::1%en0
  if_index : 6 (en0)
  flags    : Scoped, Request A records

resolver #2
  nameserver[0] : 100.100.100.100
  if_index : 21 (utun4)
  flags    : Scoped, Request A records
";
        let dns = parse_scutil_dns(body);
        assert_eq!(dns["en0"], vec![ip("192.168.1.1"), ip("fe80::1")]);
        assert_eq!(dns["utun4"], vec![ip("100.100.100.100")]);
        assert_eq!(dns.len(), 2);
    }

    #[test]
    fn netstat_defaults_skip_interface_routes() {
        let v4 = "\
Routing tables

Internet:
Destination        Gateway            Flags               Netif Expire
default            192.168.1.1        UGScg                 en0
default            link#18            UCSIg               utun3
127                127.0.0.1          UCS                   lo0
";
        assert_eq!(
            parse_netstat_defaults(v4),
            vec![("en0".to_owned(), ip("192.168.1.1"))]
        );

        let v6 = "\
Internet6:
Destination                             Gateway                                 Flags               Netif Expire
default                                 fe80::1%en0                             UGcg                  en0
";
        assert_eq!(
            parse_netstat_defaults(v6),
            vec![("en0".to_owned(), ip("fe80::1"))]
        );
    }

    #[test]
    fn route_get_names_the_interface() {
        let body = "\
   route to: 1.1.1.1
destination: default
       mask: default
    gateway: 192.168.1.1
  interface: en0
      flags: <UP,GATEWAY,DONE,STATIC,PRCLONING,GLOBAL>
";
        assert_eq!(parse_route_get(body), Some("en0"));
        assert_eq!(
            parse_route_get("route: writing to routing socket: not in table"),
            None
        );
    }

    #[test]
    fn lsof_reports_only_local_holders() {
        let body = "\
p311
f12
cmDNSResponder
n192.168.1.20:61234->1.1.1.1:53
p912
f5
cdnsmasq
n127.0.0.1:53
";
        assert_eq!(parse_lsof(body, 53), Some((912, "dnsmasq".to_owned())));

        let only_clients = "p311\nf12\ncmDNSResponder\nn192.168.1.20:61234->1.1.1.1:53\n";
        assert_eq!(parse_lsof(only_clients, 53), None);

        assert_eq!(
            parse_lsof("p77\nccom.docker.backend\nn*:53\n", 53),
            Some((77, "com.docker.backend".to_owned()))
        );
    }

    #[test]
    fn classification_uses_the_hardware_port() {
        assert_eq!(classify(Some("Wi-Fi"), "en0"), AdapterKind::WiFi);
        assert_eq!(
            classify(Some("USB 10/100/1000 LAN"), "en7"),
            AdapterKind::Ethernet
        );
        assert_eq!(
            classify(Some("Thunderbolt Bridge"), "bridge0"),
            AdapterKind::Other
        );
        assert_eq!(classify(None, "utun3"), AdapterKind::Tunnel);
        assert_eq!(classify(None, "lo0"), AdapterKind::Loopback);
        assert_eq!(classify(None, "awdl0"), AdapterKind::Other);
    }
}
