//! Writing an adapter's DNS servers.
//!
//! This lives here rather than in `forceconnect-net` so that the read side of
//! the application has no code path to it: `forceconnect-net` cannot change
//! anything, and everything that can is in this crate.
//!
//! An earlier version gated the writes behind a `privileged` feature on
//! `forceconnect-net` instead. That does not work: Cargo unifies features
//! across workspace members sharing a dependency, so a `--workspace` build
//! enabled the feature everywhere and the gate compiled away. Crate boundaries
//! hold where feature flags do not.
//!
//! The contract is the same on every platform — set, clear and restore, one
//! address family at a time, keyed by the adapter's `guid` — and so is the
//! meaning of "clear": the adapter goes back to taking DNS from wherever the
//! system would give it, rather than to an empty static list. How each
//! platform gets there is in its own module.

#[cfg(any(target_os = "linux", test))]
mod linux;
#[cfg(any(target_os = "macos", test))]
mod macos;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::{clear_dns, restore, set_dns};
#[cfg(target_os = "macos")]
pub use macos::{clear_dns, restore, set_dns};
#[cfg(windows)]
pub use windows::{clear_dns, restore, set_dns, verify_cleared};

use std::net::IpAddr;

use forceconnect_net::{Family, NetError};

/// Refuse a list that does not match the family it is written to.
#[cfg_attr(not(unix), allow(dead_code))]
fn check_family(family: Family, servers: &[IpAddr]) -> Result<(), NetError> {
    let wanted_v6 = family == Family::V6;
    if servers.iter().any(|ip| ip.is_ipv6() != wanted_v6) {
        return Err(NetError::Api(format!(
            "server list does not match the {family:?} address family"
        )));
    }
    Ok(())
}

/// Combine one family's new servers with the other family's current ones.
///
/// Windows keeps a list per family; `networksetup` and `resolvectl` keep one
/// list for both. Writing "the IPv4 servers" there means writing the whole
/// list, so the IPv6 half has to be carried over — or the IPv4 write silently
/// wipes it, and the next IPv6 write wipes the IPv4 one back. IPv4 goes first,
/// which is the order every resolver here tries them in.
#[cfg_attr(not(unix), allow(dead_code))]
fn merge(family: Family, servers: &[IpAddr], other_family: &[IpAddr]) -> Vec<IpAddr> {
    let (v4, v6) = match family {
        Family::V4 => (servers, other_family),
        Family::V6 => (other_family, servers),
    };

    v4.iter()
        .filter(|ip| ip.is_ipv4())
        .chain(v6.iter().filter(|ip| ip.is_ipv6()))
        .copied()
        .collect()
}

#[cfg_attr(not(unix), allow(dead_code))]
const fn other(family: Family) -> Family {
    match family {
        Family::V4 => Family::V6,
        Family::V6 => Family::V4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ips(values: &[&str]) -> Vec<IpAddr> {
        values.iter().map(|v| v.parse().unwrap()).collect()
    }

    #[test]
    fn a_write_to_one_family_keeps_the_other() {
        let v6 = ips(&["2606:4700:4700::1111"]);
        assert_eq!(
            merge(Family::V4, &ips(&["1.1.1.1", "1.0.0.1"]), &v6),
            ips(&["1.1.1.1", "1.0.0.1", "2606:4700:4700::1111"])
        );

        // The other order produces the same list, IPv4 first.
        let v4 = ips(&["9.9.9.9"]);
        assert_eq!(
            merge(Family::V6, &ips(&["2620:fe::fe"]), &v4),
            ips(&["9.9.9.9", "2620:fe::fe"])
        );
    }

    #[test]
    fn clearing_a_family_is_merging_nothing_into_it() {
        assert_eq!(
            merge(Family::V4, &[], &ips(&["fe80::1"])),
            ips(&["fe80::1"])
        );
        assert!(merge(Family::V6, &[], &[]).is_empty());
    }

    #[test]
    fn stray_addresses_of_the_wrong_family_are_dropped() {
        // A "current IPv6 servers" read that somehow returned an IPv4 address
        // must not duplicate it into the IPv4 half.
        assert_eq!(
            merge(Family::V4, &ips(&["1.1.1.1"]), &ips(&["1.1.1.1", "::1"])),
            ips(&["1.1.1.1", "::1"])
        );
    }

    #[test]
    fn family_mismatches_are_refused() {
        assert!(check_family(Family::V4, &ips(&["1.1.1.1"])).is_ok());
        assert!(check_family(Family::V4, &ips(&["::1"])).is_err());
        assert!(check_family(Family::V6, &ips(&["1.1.1.1"])).is_err());
    }
}
