//! Adapter enumeration on Linux.
//!
//! Addresses and flags come from `getifaddrs`; everything the kernel knows
//! about the link itself — type, state, speed, driver — from sysfs; default
//! routes from procfs. DNS is the one thing the kernel has no opinion on, so it
//! is read from whichever of NetworkManager or systemd-resolved owns it (see
//! [`crate::linux`]).

use std::collections::HashMap;
use std::net::IpAddr;
use std::path::Path;

use crate::linux::{
    self as parse, DefaultRoute, DnsTarget, NmDevice, SysfsKind, nm_key, resolved_key,
    unmanaged_key,
};
use crate::unix::{self, Interface};
use crate::{Adapter, DnsConfig, Family, InterfaceLuid, NetError};

/// Metric for an adapter with no default route of a family.
///
/// Linux's own default for a route added without one, which is also where it
/// would land in a ranking.
const NO_ROUTE_METRIC: u32 = 1024;

pub fn list() -> Result<Vec<Adapter>, NetError> {
    let interfaces = unix::interfaces()?;

    let v4_routes = read_proc("/proc/net/route", parse::parse_proc_route);
    let v6_routes = read_proc("/proc/net/ipv6_route", parse::parse_proc_ipv6_route);
    let managers = Managers::detect();

    Ok(interfaces
        .into_iter()
        .map(|iface| build(iface, &v4_routes, &v6_routes, &managers))
        .collect())
}

fn read_proc(path: &str, parse: fn(&str) -> Vec<DefaultRoute>) -> Vec<DefaultRoute> {
    std::fs::read_to_string(path)
        .map(|body| parse(&body))
        .unwrap_or_default()
}

/// What the DNS managers say, gathered once per enumeration rather than once
/// per adapter.
struct Managers {
    nm: HashMap<String, NmDevice>,
    /// `None` when systemd-resolved is not running.
    resolved: Option<HashMap<String, Vec<IpAddr>>>,
}

impl Managers {
    fn detect() -> Self {
        // `nmcli` failing — not installed, or NetworkManager not running — is
        // the ordinary case on a server, and means "manages nothing".
        let nm = unix::run(
            "nmcli",
            &[
                "-t",
                "-f",
                "GENERAL.DEVICE,GENERAL.STATE,IP4.DNS,IP6.DNS",
                "device",
                "show",
            ],
        )
        .map(|body| parse::parse_nmcli_devices(&body))
        .unwrap_or_default();

        let resolved = resolved_running()
            .then(|| unix::run("resolvectl", &["dns"]).ok())
            .flatten()
            .map(|body| parse::parse_resolvectl_dns(&body));

        Self { nm, resolved }
    }

    fn target_key(&self, device: &str) -> String {
        if self.nm.get(device).is_some_and(|d| d.managed) {
            nm_key(device)
        } else if self.resolved.is_some() {
            resolved_key(device)
        } else {
            unmanaged_key(device)
        }
    }

    fn servers(&self, device: &str) -> Vec<IpAddr> {
        let from_nm = self
            .nm
            .get(device)
            .map(|d| d.dns.clone())
            .unwrap_or_default();
        let from_resolved = self
            .resolved
            .as_ref()
            .and_then(|links| links.get(device).cloned())
            .unwrap_or_default();

        // Prefer whoever owns the device; the other may only be relaying.
        if self.nm.get(device).is_some_and(|d| d.managed) && !from_nm.is_empty() {
            from_nm
        } else if !from_resolved.is_empty() {
            from_resolved
        } else {
            from_nm
        }
    }
}

/// systemd-resolved creates its runtime directory when it starts; checking for
/// it is cheaper than a D-Bus round trip and does not hang when the service is
/// absent.
pub(crate) fn resolved_running() -> bool {
    Path::new("/run/systemd/resolve").is_dir() && unix::locate("resolvectl").is_some()
}

fn build(
    iface: Interface,
    v4_routes: &[DefaultRoute],
    v6_routes: &[DefaultRoute],
    managers: &Managers,
) -> Adapter {
    let sys = Path::new("/sys/class/net").join(&iface.name);
    let read = |file: &str| std::fs::read_to_string(sys.join(file)).ok();

    let arphrd = read("type")
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or_default();
    let uevent = read("uevent").unwrap_or_default();
    let devtype = uevent
        .lines()
        .find_map(|line| line.strip_prefix("DEVTYPE="))
        .map(str::trim);

    let kind = parse::classify(SysfsKind {
        arphrd,
        wireless: sys.join("wireless").exists() || sys.join("phy80211").exists(),
        tun: sys.join("tun_flags").exists(),
        hardware: sys.join("device").exists(),
        devtype,
    });

    let status = parse::map_operstate(
        &read("operstate").unwrap_or_default(),
        iface.is_up_and_running(),
    );

    // Mbps, or -1 / an I/O error for links that have no notion of speed.
    let link_speed_bps = read("speed")
        .and_then(|s| s.trim().parse::<i64>().ok())
        .filter(|mbps| *mbps > 0)
        .map_or(0, |mbps| mbps as u64 * 1_000_000);

    let driver = std::fs::read_link(sys.join("device/driver"))
        .ok()
        .and_then(|path| path.file_name().map(|n| n.to_string_lossy().into_owned()));
    let description = driver
        .or_else(|| devtype.map(str::to_owned))
        .unwrap_or_else(|| "virtual interface".into());

    let routes = |table: &[DefaultRoute]| -> (Vec<IpAddr>, u32) {
        let mine: Vec<&DefaultRoute> = table.iter().filter(|r| r.device == iface.name).collect();
        let metric = mine
            .iter()
            .map(|r| r.metric)
            .min()
            .unwrap_or(NO_ROUTE_METRIC);
        (mine.iter().map(|r| r.gateway).collect(), metric)
    };
    let (mut gateways, metric_v4) = routes(v4_routes);
    let (v6_gateways, metric_v6) = routes(v6_routes);
    gateways.extend(v6_gateways);

    let servers = managers.servers(&iface.name);

    Adapter {
        luid: InterfaceLuid::from_name(&iface.name),
        guid: managers.target_key(&iface.name),
        index: iface.index,
        friendly_name: iface.name.clone(),
        description,
        kind,
        status,
        mac: iface.mac,
        link_speed_bps,
        unicast: iface.unicast,
        gateways,
        dns_v4: servers.iter().copied().filter(IpAddr::is_ipv4).collect(),
        dns_v6: servers.iter().copied().filter(IpAddr::is_ipv6).collect(),
        metric_v4,
        metric_v6,
    }
}

/// Read how an adapter is currently configured for one address family.
///
/// For a NetworkManager device this is the connection profile: a non-empty
/// `ipv4.dns` is what "statically configured" means there, exactly as a
/// non-empty `NameServer` does on Windows. For a bare systemd-resolved link
/// there is no such distinction to read — the link's servers are whatever was
/// last pushed to it — so any servers at all count as static, and restoring
/// them puts back exactly what was there.
pub fn dns_config(guid: &str, family: Family) -> Result<DnsConfig, NetError> {
    let servers = match DnsTarget::parse(guid)? {
        DnsTarget::NetworkManager(device) => nm_profile_servers(device, family)?,
        DnsTarget::Resolved(device) => {
            let body = unix::run("resolvectl", &["dns", device])?;
            parse::parse_resolvectl_dns(&body)
                .remove(device)
                .unwrap_or_default()
        }
    };

    let wanted_v6 = family == Family::V6;
    let servers: Vec<IpAddr> = servers
        .into_iter()
        .filter(|ip| ip.is_ipv6() == wanted_v6)
        .collect();

    Ok(if servers.is_empty() {
        DnsConfig::Dhcp
    } else {
        DnsConfig::Static(servers)
    })
}

fn nm_profile_servers(device: &str, family: Family) -> Result<Vec<IpAddr>, NetError> {
    let connections = unix::run("nmcli", &["-t", "-f", "DEVICE,CON-UUID", "device"])?;
    let Some(uuid) = parse::parse_nmcli_connections(&connections).remove(device) else {
        return Ok(Vec::new());
    };

    let field = match family {
        Family::V4 => "ipv4.dns",
        Family::V6 => "ipv6.dns",
    };
    let body = unix::run("nmcli", &["-g", field, "connection", "show", &uuid])?;
    Ok(parse::parse_nmcli_server_list(&body))
}
