//! Writing DNS on macOS, with `networksetup`.
//!
//! `networksetup -setdnsservers` writes the service's DNS into the system
//! configuration store the same way System Settings does, and `configd` pushes
//! the change to the resolver itself — the macOS counterpart of
//! `SetInterfaceDnsSettings` notifying the DNS Client service.
//!
//! The alternative, `SCPreferences` with an authorization reference, is the
//! API `networksetup` is built on. It would trade one process spawn per write
//! for several hundred lines of CoreFoundation dictionary handling, and the
//! output being read back here is a line long.
//!
//! One list covers both families, so a write to one carries the other's
//! current servers over (see `super::merge`). An empty list is spelled
//! `Empty`, which returns the service to DHCP-supplied servers.

#![cfg_attr(not(target_os = "macos"), allow(dead_code))]

use std::net::IpAddr;

use forceconnect_net::adapters::dns_config;
use forceconnect_net::macos::DnsTarget;
use forceconnect_net::{DnsConfig, Family, NetError};

use super::{check_family, merge, other};
use crate::exec;

/// Point a service at a fixed list of servers for one address family.
pub fn set_dns(guid: &str, family: Family, servers: &[IpAddr]) -> Result<(), NetError> {
    check_family(family, servers)?;
    let DnsTarget::Service(service) = DnsTarget::parse(guid)?;

    let carried = current(guid, other(family));
    setdnsservers(service, &merge(family, servers, &carried))
}

/// Return a service to DHCP-supplied servers for one address family.
pub fn clear_dns(guid: &str, family: Family) -> Result<(), NetError> {
    let DnsTarget::Service(service) = DnsTarget::parse(guid)?;

    let carried = current(guid, other(family));
    setdnsservers(service, &merge(family, &[], &carried))
}

/// Restore a previously captured configuration.
pub fn restore(guid: &str, family: Family, previous: &DnsConfig) -> Result<(), NetError> {
    match previous {
        DnsConfig::Dhcp => clear_dns(guid, family),
        DnsConfig::Static(servers) => set_dns(guid, family, servers),
    }
}

fn current(guid: &str, family: Family) -> Vec<IpAddr> {
    dns_config(guid, family)
        .map(|config| config.servers().to_vec())
        .unwrap_or_default()
}

fn setdnsservers_args(service: &str, servers: &[IpAddr]) -> Vec<String> {
    let mut args = vec!["-setdnsservers".to_owned(), service.to_owned()];
    if servers.is_empty() {
        args.push("Empty".into());
    } else {
        args.extend(servers.iter().map(ToString::to_string));
    }
    args
}

/// `networksetup` exits 0 for most failures and says so on standard output,
/// as `** Error: …`.
fn reported_error(stdout: &str) -> Option<&str> {
    stdout
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("** Error"))
}

fn setdnsservers(service: &str, servers: &[IpAddr]) -> Result<(), NetError> {
    let args = setdnsservers_args(service, servers);
    let args: Vec<&str> = args.iter().map(String::as_str).collect();

    let stdout = exec::run("networksetup", &args).map_err(NetError::Api)?;
    match reported_error(&stdout) {
        Some(err) => Err(NetError::Api(format!("networksetup: {err}"))),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_list_is_spelled_empty() {
        assert_eq!(
            setdnsservers_args("Wi-Fi", &[]),
            ["-setdnsservers", "Wi-Fi", "Empty"]
        );

        let servers: Vec<IpAddr> = vec!["1.1.1.1".parse().unwrap(), "::1".parse().unwrap()];
        assert_eq!(
            setdnsservers_args("USB 10/100/1000 LAN", &servers),
            ["-setdnsservers", "USB 10/100/1000 LAN", "1.1.1.1", "::1"]
        );
    }

    #[test]
    fn errors_on_stdout_are_errors() {
        assert_eq!(
            reported_error("** Error: The parameters were not valid.\n"),
            Some("** Error: The parameters were not valid.")
        );
        assert_eq!(reported_error("\n"), None);
    }
}
