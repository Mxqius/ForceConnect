//! Writing DNS on Linux, through whichever manager owns the adapter.
//!
//! See `forceconnect_net::linux` for how that is decided. The short version:
//! NetworkManager when it manages the device, systemd-resolved otherwise —
//! because writing underneath NetworkManager is overwritten at its next DHCP
//! event, with nothing to tell the user why their DNS changed back.
//!
//! # NetworkManager
//!
//! Changes go through `nmcli device modify`, which alters the *applied*
//! connection on the device and leaves the saved profile alone. That choice
//! carries most of the design:
//!
//! - Nothing the app does is persisted. A reboot, a reconnect, or a crash
//!   followed by a power cycle all land the device back on the user's own
//!   profile without any help from the journal.
//! - Undoing a change is `nmcli device reapply`, which re-applies the saved
//!   profile — exactly the user's configuration, static servers and
//!   `ignore-auto-dns` included, rather than an approximation rebuilt from
//!   what was observed. So clear and restore are the same operation there.
//!
//! # systemd-resolved
//!
//! `resolvectl dns` sets one list per link for both families, so a write to
//! one family carries the other's current servers over (see `super::merge`).
//! An empty result is `resolvectl revert`, which hands the link back to
//! whatever configured it — systemd-networkd re-supplies its DHCP servers, and
//! `resolvectl` routes both calls through networkd for links it manages.

#![cfg_attr(not(target_os = "linux"), allow(dead_code))]

use std::net::IpAddr;

use forceconnect_net::adapters::dns_config;
use forceconnect_net::linux::DnsTarget;
use forceconnect_net::{DnsConfig, Family, NetError};

use super::{check_family, merge, other};
use crate::exec;

/// Point an adapter at a fixed list of servers for one address family.
pub fn set_dns(guid: &str, family: Family, servers: &[IpAddr]) -> Result<(), NetError> {
    check_family(family, servers)?;

    match DnsTarget::parse(guid)? {
        DnsTarget::NetworkManager(device) => {
            match nmcli(&nm_modify_args(device, family, servers)) {
                // A connection with IPv6 turned off rejects IPv6 DNS outright.
                // There is no IPv6 resolver for it to route around us with, so
                // there is nothing to set.
                Err(NetError::Api(err)) if family == Family::V6 && ipv6_is_off(&err) => Ok(()),
                result => result,
            }
        }
        DnsTarget::Resolved(device) => {
            let carried = current(guid, other(family));
            resolvectl(&resolved_args(device, &merge(family, servers, &carried)))
        }
    }
}

/// Return an adapter to the DNS its manager would give it, for one family.
pub fn clear_dns(guid: &str, family: Family) -> Result<(), NetError> {
    match DnsTarget::parse(guid)? {
        DnsTarget::NetworkManager(device) => {
            nmcli(&["device".to_owned(), "reapply".to_owned(), device.to_owned()])
        }
        DnsTarget::Resolved(device) => {
            let carried = current(guid, other(family));
            resolvectl(&resolved_args(device, &merge(family, &[], &carried)))
        }
    }
}

/// Restore a previously captured configuration.
///
/// For NetworkManager the capture is not needed: the saved profile *is* the
/// configuration, and reapplying it is lossless.
pub fn restore(guid: &str, family: Family, previous: &DnsConfig) -> Result<(), NetError> {
    match (DnsTarget::parse(guid)?, previous) {
        (DnsTarget::NetworkManager(_), _) | (_, DnsConfig::Dhcp) => clear_dns(guid, family),
        (_, DnsConfig::Static(servers)) => set_dns(guid, family, servers),
    }
}

/// The other family's servers as they stand, to carry across a write.
///
/// A failed read is treated as "none": the worst case is that the other
/// family falls back to its manager's defaults, which is the safe direction.
fn current(guid: &str, family: Family) -> Vec<IpAddr> {
    dns_config(guid, family)
        .map(|config| config.servers().to_vec())
        .unwrap_or_default()
}

fn nm_modify_args(device: &str, family: Family, servers: &[IpAddr]) -> Vec<String> {
    let prefix = match family {
        Family::V4 => "ipv4",
        Family::V6 => "ipv6",
    };
    let list = servers
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",");

    vec![
        "device".into(),
        "modify".into(),
        device.into(),
        format!("{prefix}.dns"),
        list,
        // Without this the DHCP servers stay on the list alongside ours, and
        // the resolver is free to use either.
        format!("{prefix}.ignore-auto-dns"),
        "yes".into(),
    ]
}

fn ipv6_is_off(err: &str) -> bool {
    err.contains("method=disabled") || err.contains("method=ignore")
}

fn resolved_args(device: &str, servers: &[IpAddr]) -> Vec<String> {
    if servers.is_empty() {
        return vec!["revert".into(), device.into()];
    }

    ["dns".to_owned(), device.to_owned()]
        .into_iter()
        .chain(servers.iter().map(ToString::to_string))
        .collect()
}

fn nmcli(args: &[String]) -> Result<(), NetError> {
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    exec::run("nmcli", &args).map(drop).map_err(NetError::Api)
}

fn resolvectl(args: &[String]) -> Result<(), NetError> {
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    exec::run("resolvectl", &args)
        .map(drop)
        .map_err(NetError::Api)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ips(values: &[&str]) -> Vec<IpAddr> {
        values.iter().map(|v| v.parse().unwrap()).collect()
    }

    #[test]
    fn networkmanager_changes_are_applied_not_saved() {
        let args = nm_modify_args("wlp3s0", Family::V4, &ips(&["1.1.1.1", "1.0.0.1"]));
        assert_eq!(
            args,
            [
                "device",
                "modify",
                "wlp3s0",
                "ipv4.dns",
                "1.1.1.1,1.0.0.1",
                "ipv4.ignore-auto-dns",
                "yes"
            ]
        );

        let v6 = nm_modify_args("wlp3s0", Family::V6, &ips(&["2606:4700:4700::1111"]));
        assert_eq!(v6[3], "ipv6.dns");
        assert_eq!(v6[4], "2606:4700:4700::1111");
        assert_eq!(v6[5], "ipv6.ignore-auto-dns");
    }

    #[test]
    fn a_connection_without_ipv6_is_not_an_error() {
        assert!(ipv6_is_off(
            "Error: Reapplying connection to device 'wlp3s0' failed: ipv6.dns: this property is \
             not allowed for 'method=disabled'"
        ));
        assert!(!ipv6_is_off("Error: Device 'wlp3s0' not found."));
    }

    #[test]
    fn an_empty_resolved_list_reverts_the_link() {
        assert_eq!(resolved_args("wg0", &[]), ["revert", "wg0"]);
        assert_eq!(
            resolved_args("wg0", &ips(&["1.1.1.1", "2606:4700:4700::1111"])),
            ["dns", "wg0", "1.1.1.1", "2606:4700:4700::1111"]
        );
    }
}
