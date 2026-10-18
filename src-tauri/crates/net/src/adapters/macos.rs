//! Adapter enumeration on macOS.
//!
//! Interfaces, addresses and flags come from `getifaddrs`. The network
//! services layered over them — which carry the names a user recognises, the
//! priority order, and the DNS setting — come from `networksetup`; the servers
//! actually in use from `scutil --dns`; gateways from `netstat`.
//!
//! Every tool here is in the base system and works unprivileged. See
//! [`crate::macos`] for why the write key is the service and not the
//! interface.

use std::net::IpAddr;

use crate::macos::{self as parse, DnsTarget, Service, device_key, service_key};
use crate::unix;
use crate::{Adapter, DnsConfig, Family, InterfaceLuid, NetError, OperStatus};

/// Metric for an interface with no network service: after every service.
const NO_SERVICE_METRIC: u32 = 1024;

pub fn list() -> Result<Vec<Adapter>, NetError> {
    let interfaces = unix::interfaces()?;

    // Each of these failing degrades the listing rather than failing it: an
    // adapter with no gateway or no service is still worth showing.
    let services = unix::run("networksetup", &["-listnetworkserviceorder"])
        .map(|body| parse::parse_service_order(&body))
        .unwrap_or_default();
    let mut gateways = Vec::new();
    for family in ["inet", "inet6"] {
        if let Ok(body) = unix::run("netstat", &["-rn", "-f", family]) {
            gateways.extend(parse::parse_netstat_defaults(&body));
        }
    }
    let dns = unix::run("scutil", &["--dns"])
        .map(|body| parse::parse_scutil_dns(&body))
        .unwrap_or_default();

    Ok(interfaces
        .into_iter()
        .map(|iface| {
            let position = services
                .iter()
                .position(|s| s.enabled && s.device.as_deref() == Some(iface.name.as_str()));
            let service: Option<&Service> = position.map(|at| &services[at]);

            let status = if iface.is_up_and_running() {
                OperStatus::Up
            } else {
                OperStatus::Down
            };

            // Ten apart, so the numbers read like interface metrics rather
            // than list positions.
            let metric = position.map_or(NO_SERVICE_METRIC, |at| (at as u32 + 1) * 10);
            let servers = dns.get(&iface.name).cloned().unwrap_or_default();

            Adapter {
                luid: InterfaceLuid::from_name(&iface.name),
                guid: service.map_or_else(|| device_key(&iface.name), |s| service_key(&s.name)),
                index: iface.index,
                friendly_name: service.map_or_else(|| iface.name.clone(), |s| s.name.clone()),
                description: service
                    .map_or_else(|| iface.name.clone(), |s| s.hardware_port.clone()),
                kind: parse::classify(service.map(|s| s.hardware_port.as_str()), &iface.name),
                status,
                mac: iface.mac,
                // macOS reports negotiated media only through `ifconfig`'s
                // free-form text; not worth parsing for a display figure.
                link_speed_bps: 0,
                unicast: iface.unicast,
                gateways: gateways
                    .iter()
                    .filter(|(device, _)| *device == iface.name)
                    .map(|(_, gateway)| *gateway)
                    .collect(),
                dns_v4: servers.iter().copied().filter(IpAddr::is_ipv4).collect(),
                dns_v6: servers.iter().copied().filter(IpAddr::is_ipv6).collect(),
                metric_v4: metric,
                metric_v6: metric,
            }
        })
        .collect())
}

/// Read how a service is currently configured for one address family.
///
/// `networksetup -getdnsservers` reports only the statically configured list,
/// which is exactly the distinction [`DnsConfig`] draws: no servers there means
/// the service takes whatever DHCP hands it.
pub fn dns_config(guid: &str, family: Family) -> Result<DnsConfig, NetError> {
    let DnsTarget::Service(service) = DnsTarget::parse(guid)?;

    let body = unix::run("networksetup", &["-getdnsservers", service])?;
    let wanted_v6 = family == Family::V6;
    let servers: Vec<IpAddr> = parse::parse_getdnsservers(&body)
        .into_iter()
        .filter(|ip| ip.is_ipv6() == wanted_v6)
        .collect();

    Ok(if servers.is_empty() {
        DnsConfig::Dhcp
    } else {
        DnsConfig::Static(servers)
    })
}
