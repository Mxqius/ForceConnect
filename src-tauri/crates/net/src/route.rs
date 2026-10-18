//! Which adapter is actually carrying traffic.
//!
//! The previous implementation took the first `Up` adapter that was Ethernet or
//! Wi-Fi and had an IPv4 gateway. That is wrong on any machine with a VPN: the
//! tunnel is neither of those interface types, yet it is where the packets go.
//! It is also non-deterministic, since adapter enumeration order is not stable.
//!
//! Two mechanisms, in order of authority:
//!
//! 1. Ask the routing table, which is what the stack itself consults:
//!    `GetBestRoute2` on Windows, `ip route get` on Linux (which also honours
//!    the policy rules WireGuard uses), `route -n get` on macOS.
//! 2. Fall back to a deterministic ranking over the enumerated adapters.
//!
//! Note that scanning for a `0.0.0.0/0` route is *not* a valid substitute for
//! (1). VPN clients routinely install the split pair `0.0.0.0/1` +
//! `128.0.0.0/1`, which outranks the real default route without replacing it,
//! so a default-route scan misses them entirely.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::{Adapter, AdapterKind, Family, OperStatus};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::best_route_luid;
#[cfg(target_os = "macos")]
pub use macos::best_route_luid;
#[cfg(windows)]
pub use windows::best_route_luid;

/// Destinations used to ask the stack "if I sent a packet here, which adapter
/// would it leave by?". They are never contacted — this is a routing table
/// lookup, not a probe.
pub(crate) const PROBE_V4: Ipv4Addr = Ipv4Addr::new(1, 1, 1, 1);
pub(crate) const PROBE_V6: Ipv6Addr = Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111);

/// Rank eligible adapters best-first.
///
/// Ordering, in priority order:
///
/// 1. Interface metric ascending — this, not adapter type, is what Windows
///    itself uses to choose. (On Linux it is the default route's metric, and
///    on macOS the network service order; see [`Adapter::metric_v4`].)
/// 2. Physical media ahead of virtual, as a tie-break only.
/// 3. Link speed descending.
/// 4. Interface index ascending, purely so the result is stable.
pub fn rank(adapters: &[Adapter], family: Family) -> Vec<&Adapter> {
    let mut eligible: Vec<&Adapter> = adapters
        .iter()
        .filter(|a| a.status == OperStatus::Up)
        .filter(|a| a.kind != AdapterKind::Loopback)
        .filter(|a| {
            let wants_v6 = family == Family::V6;
            a.unicast
                .iter()
                .any(|ip| ip.is_ipv6() == wants_v6 && !is_link_local(ip))
        })
        // A gateway is the usual sign an adapter reaches beyond its own subnet,
        // but point-to-point VPN tunnels legitimately have none — verified on
        // an ExpressVPN WinTUN adapter that was carrying all traffic while
        // reporting no gateway at all. Requiring one here would drop exactly
        // the adapter that matters most.
        .filter(|a| !a.gateways.is_empty() || a.kind == AdapterKind::Tunnel)
        .collect();

    eligible.sort_by(|a, b| {
        let metric = |x: &Adapter| match family {
            Family::V4 => x.metric_v4,
            Family::V6 => x.metric_v6,
        };

        metric(a)
            .cmp(&metric(b))
            .then_with(|| media_rank(a.kind).cmp(&media_rank(b.kind)))
            .then_with(|| b.link_speed_bps.cmp(&a.link_speed_bps))
            .then_with(|| a.index.cmp(&b.index))
    });

    eligible
}

const fn media_rank(kind: AdapterKind) -> u8 {
    match kind {
        AdapterKind::Ethernet => 0,
        AdapterKind::WiFi => 1,
        AdapterKind::Tunnel => 2,
        AdapterKind::Other => 3,
        AdapterKind::Loopback => 4,
    }
}

fn is_link_local(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_link_local(),
        // `Ipv6Addr::is_unicast_link_local` is still unstable, so test the
        // fe80::/10 prefix directly.
        IpAddr::V6(v6) => (v6.segments()[0] & 0xffc0) == 0xfe80,
    }
}

/// The adapter traffic is actually leaving by.
///
/// Prefers the routing table and falls back to ranking. When the two disagree
/// that is a signal worth logging — it usually means a split-tunnel VPN.
pub fn best_adapter(adapters: &[Adapter], family: Family) -> Option<&Adapter> {
    let ranked = rank(adapters, family);

    match best_route_luid(family) {
        Ok(luid) => {
            if let Some(found) = adapters.iter().find(|a| a.luid == luid) {
                if ranked.first().map(|a| a.luid) != Some(luid) {
                    tracing::debug!(
                        route = %found.friendly_name,
                        ranked = ranked.first().map(|a| a.friendly_name.as_str()).unwrap_or("none"),
                        "routing table and metric ranking disagree; trusting the route"
                    );
                }
                return Some(found);
            }
            ranked.first().copied()
        }
        Err(err) => {
            tracing::debug!(%err, "routing table lookup unavailable; falling back to ranking");
            ranked.first().copied()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InterfaceLuid;

    fn adapter(name: &str, kind: AdapterKind, metric: u32, speed: u64, index: u32) -> Adapter {
        Adapter {
            luid: InterfaceLuid(index as u64),
            guid: format!("{{{index:08x}-0000-0000-0000-000000000000}}"),
            index,
            friendly_name: name.to_owned(),
            description: name.to_owned(),
            kind,
            status: OperStatus::Up,
            mac: None,
            link_speed_bps: speed,
            unicast: vec![IpAddr::from([10, 0, 0, index as u8])],
            gateways: vec![IpAddr::from([10, 0, 0, 1])],
            dns_v4: vec![],
            dns_v6: vec![],
            metric_v4: metric,
            metric_v6: metric,
        }
    }

    #[test]
    fn metric_beats_media_type() {
        // The shape of this developer's machine: a VPN tunnel at metric 3 and
        // Wi-Fi at 55. The old "first Ethernet or Wi-Fi with a gateway" rule
        // picks Wi-Fi, and setting DNS there does nothing observable.
        let adapters = vec![
            adapter("Wi-Fi", AdapterKind::WiFi, 55, 72_200_000, 13),
            adapter("ExpressVPN", AdapterKind::Tunnel, 3, 100_000_000_000, 83),
        ];

        let ranked = rank(&adapters, Family::V4);
        assert_eq!(ranked[0].friendly_name, "ExpressVPN");
    }

    #[test]
    fn a_gatewayless_tunnel_is_still_eligible() {
        // Regression test for real observed state: the ExpressVPN WinTUN
        // adapter was Up, carrying every packet, at metric 3 — and reported no
        // gateway. An earlier version filtered on gateway presence and dropped
        // it, falling back to Wi-Fi, where setting DNS would have changed
        // nothing the user could observe.
        let mut tunnel = adapter("ExpressVPN", AdapterKind::Tunnel, 3, 100_000_000_000, 89);
        tunnel.gateways.clear();
        tunnel.unicast = vec![IpAddr::from([100, 64, 100, 6])];

        let wifi = adapter("Wi-Fi", AdapterKind::WiFi, 55, 72_200_000, 15);

        let adapters = [tunnel, wifi];
        let ranked = rank(&adapters, Family::V4);
        assert_eq!(ranked.len(), 2);
        assert_eq!(ranked[0].friendly_name, "ExpressVPN");
    }

    #[test]
    fn a_gatewayless_virtual_switch_is_not_eligible() {
        // The mirror case: Hyper-V's Default Switch is Up with a routable
        // address and no gateway, but it goes nowhere. Only tunnels get the
        // gateway exemption.
        let mut switch = adapter(
            "vEthernet (Default Switch)",
            AdapterKind::Ethernet,
            15,
            10_000_000_000,
            27,
        );
        switch.gateways.clear();
        switch.unicast = vec![IpAddr::from([172, 23, 64, 1])];

        let wifi = adapter("Wi-Fi", AdapterKind::WiFi, 55, 72_200_000, 15);

        let adapters = [switch, wifi];
        let ranked = rank(&adapters, Family::V4);
        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0].friendly_name, "Wi-Fi");
    }

    #[test]
    fn media_type_breaks_metric_ties() {
        let adapters = vec![
            adapter("Wi-Fi", AdapterKind::WiFi, 25, 1_000_000_000, 13),
            adapter("Ethernet", AdapterKind::Ethernet, 25, 1_000_000_000, 14),
        ];

        assert_eq!(rank(&adapters, Family::V4)[0].friendly_name, "Ethernet");
    }

    #[test]
    fn ordering_is_stable_when_everything_ties() {
        let adapters = vec![
            adapter("B", AdapterKind::Ethernet, 25, 1_000, 20),
            adapter("A", AdapterKind::Ethernet, 25, 1_000, 10),
        ];

        assert_eq!(rank(&adapters, Family::V4)[0].friendly_name, "A");
    }

    #[test]
    fn ineligible_adapters_are_excluded() {
        let mut down = adapter("Down", AdapterKind::Ethernet, 1, 1_000, 1);
        down.status = OperStatus::Down;

        let mut gatewayless = adapter("NoGateway", AdapterKind::Ethernet, 1, 1_000, 2);
        gatewayless.gateways.clear();

        let loopback = adapter("Loopback", AdapterKind::Loopback, 1, 1_000, 3);

        let mut link_local_only = adapter("LinkLocal", AdapterKind::Ethernet, 1, 1_000, 4);
        link_local_only.unicast = vec![IpAddr::from([169, 254, 1, 5])];

        let good = adapter("Wi-Fi", AdapterKind::WiFi, 50, 1_000, 5);

        let adapters = vec![down, gatewayless, loopback, link_local_only, good];
        let ranked = rank(&adapters, Family::V4);

        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0].friendly_name, "Wi-Fi");
    }
}
