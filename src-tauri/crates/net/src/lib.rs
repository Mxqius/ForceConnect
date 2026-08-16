//! Windows adapter enumeration and DNS configuration.
//!
//! Shared by both processes on purpose. The UI reads adapters to display them;
//! the helper writes DNS settings to one of them. If each side marshalled
//! adapter identity independently, the two could disagree about which adapter
//! the user selected — a bug that would show up as "I set DNS and nothing
//! happened". One implementation, one notion of identity.
//!
//! **This crate is read-only.** Writing DNS settings lives in the helper crate,
//! which the UI does not depend on, so there is no code path from the UI to a
//! privileged call.
//!
//! An earlier version gated the writes behind a `privileged` feature here
//! instead. That does not work: Cargo unifies features across workspace members
//! sharing a dependency, so a `--workspace` build enabled the feature for the
//! UI crate too and the "gate" compiled away. Crate boundaries hold where
//! feature flags do not.
//!
//! # Platform floor
//!
//! `SetInterfaceDnsSettings` / `GetInterfaceDnsSettings` are documented as
//! Windows 10 build 19041 (2004) or later. That is the supported floor for the
//! app; the `DNS_INTERFACE_SETTINGS` struct itself dates to 18362, but the
//! functions that consume it do not.

#[cfg(windows)]
pub mod adapters;
#[cfg(windows)]
pub mod route;

use std::net::IpAddr;

use serde::{Deserialize, Serialize};

pub use forceconnect_ipc::{Family, InterfaceLuid};

/// How an adapter is currently getting its DNS servers for one address family.
///
/// The distinction matters for restore: putting a DHCP adapter back means
/// clearing the static list, not writing the previously observed servers into
/// it as a static configuration.
///
/// Note that this is *not* the same question as whether the adapter's IP
/// address came from DHCP. An adapter routinely has a DHCP address with
/// statically configured DNS — that is exactly the state this app creates — so
/// `IP_ADAPTER_DHCP_ENABLED` is the wrong thing to test. The correct test is
/// whether `GetInterfaceDnsSettings` reports a non-empty `NameServer`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsConfig {
    /// Servers are supplied by DHCP.
    Dhcp,
    /// Servers were explicitly configured.
    Static(Vec<IpAddr>),
}

impl DnsConfig {
    pub fn servers(&self) -> &[IpAddr] {
        match self {
            Self::Dhcp => &[],
            Self::Static(servers) => servers,
        }
    }

    pub const fn is_dhcp(&self) -> bool {
        matches!(self, Self::Dhcp)
    }
}

/// Operational state, mirroring `IF_OPER_STATUS`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperStatus {
    Up,
    Down,
    Testing,
    Unknown,
    Dormant,
    NotPresent,
    LowerLayerDown,
}

/// Broad adapter classification, used for grouping and ranking in the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdapterKind {
    Ethernet,
    WiFi,
    /// VPN tunnels and other virtual point-to-point adapters. These routinely
    /// outrank physical adapters, so they cannot be filtered out the way the
    /// previous implementation did.
    Tunnel,
    Loopback,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Adapter {
    /// Stable across disable/enable, unlike the interface index.
    pub luid: InterfaceLuid,
    /// Adapter GUID string. This is what the DNS settings API is keyed on.
    pub guid: String,
    /// Index is *not* persistent; kept only for display and diagnostics.
    pub index: u32,
    pub friendly_name: String,
    pub description: String,
    pub kind: AdapterKind,
    pub status: OperStatus,
    pub mac: Option<String>,
    pub link_speed_bps: u64,
    pub unicast: Vec<IpAddr>,
    pub gateways: Vec<IpAddr>,
    pub dns_v4: Vec<IpAddr>,
    pub dns_v6: Vec<IpAddr>,
    /// Lower is preferred. This, not adapter type, is what decides which
    /// adapter Windows actually routes through.
    pub metric_v4: u32,
    pub metric_v6: u32,
}

impl Adapter {
    /// Whether this adapter is a plausible target for a DNS change.
    pub fn is_eligible(&self) -> bool {
        self.status == OperStatus::Up
            && self.kind != AdapterKind::Loopback
            && (!self.gateways.is_empty() || self.kind == AdapterKind::Tunnel)
    }

    /// Unicast addresses ordered by how useful they are to show a person.
    ///
    /// The OS reports IPv6 before IPv4 and does not sort by scope, so the raw
    /// first entry is usually a link-local address — `fe80::…` tells a user
    /// nothing about which network they are on.
    pub fn display_addresses(&self) -> Vec<IpAddr> {
        let mut sorted = self.unicast.clone();
        sorted.sort_by_key(address_rank);
        sorted
    }
}

/// Lower sorts first: routable IPv4, routable IPv6, then link-local of either.
fn address_rank(ip: &IpAddr) -> u8 {
    let link_local = match ip {
        IpAddr::V4(v4) => v4.is_link_local(),
        IpAddr::V6(v6) => (v6.segments()[0] & 0xffc0) == 0xfe80,
    };

    match (link_local, ip.is_ipv4()) {
        (false, true) => 0,
        (false, false) => 1,
        (true, true) => 2,
        (true, false) => 3,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NetError {
    #[error("windows api call failed: {0}")]
    Api(String),

    #[error("no adapter matches {0:?}")]
    UnknownAdapter(InterfaceLuid),

    #[error("adapter GUID {0:?} could not be parsed")]
    BadGuid(String),

    #[error("this operation requires Windows 10 build 19041 or later")]
    Unsupported,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dhcp_and_empty_static_are_distinct() {
        // Collapsing these is how a disconnect turns a DHCP adapter into one
        // with an empty static server list and no working DNS.
        assert!(DnsConfig::Dhcp.is_dhcp());
        assert!(!DnsConfig::Static(vec![]).is_dhcp());
        assert_eq!(
            DnsConfig::Dhcp.servers(),
            DnsConfig::Static(vec![]).servers()
        );
    }

    #[test]
    fn loopback_and_gatewayless_adapters_are_ineligible() {
        let base = Adapter {
            luid: InterfaceLuid(1),
            guid: "{00000000-0000-0000-0000-000000000000}".into(),
            index: 1,
            friendly_name: "Test".into(),
            description: "Test".into(),
            kind: AdapterKind::Ethernet,
            status: OperStatus::Up,
            mac: None,
            link_speed_bps: 0,
            unicast: vec![],
            gateways: vec![IpAddr::from([192, 168, 1, 1])],
            dns_v4: vec![],
            dns_v6: vec![],
            metric_v4: 25,
            metric_v6: 25,
        };

        assert!(base.is_eligible());

        let no_gateway = Adapter {
            gateways: vec![],
            ..base.clone()
        };
        assert!(!no_gateway.is_eligible());

        let loopback = Adapter {
            kind: AdapterKind::Loopback,
            ..base.clone()
        };
        assert!(!loopback.is_eligible());

        let down = Adapter {
            status: OperStatus::Down,
            ..base
        };
        assert!(!down.is_eligible());
    }

    #[test]
    fn display_addresses_puts_the_useful_one_first() {
        // Real ordering as returned by GetAdaptersAddresses: IPv6 link-local
        // ahead of the routable IPv4 address. Showing `fe80::…` as an
        // adapter's address tells a user nothing.
        let adapter = Adapter {
            luid: InterfaceLuid(1),
            guid: String::new(),
            index: 1,
            friendly_name: "Wi-Fi".into(),
            description: String::new(),
            kind: AdapterKind::WiFi,
            status: OperStatus::Up,
            mac: None,
            link_speed_bps: 0,
            unicast: vec![
                "fe80::b543:5d0b:f125:18d3".parse().unwrap(),
                "192.168.16.241".parse().unwrap(),
            ],
            gateways: vec![],
            dns_v4: vec![],
            dns_v6: vec![],
            metric_v4: 0,
            metric_v6: 0,
        };

        let sorted = adapter.display_addresses();
        assert_eq!(sorted[0].to_string(), "192.168.16.241");
        assert_eq!(sorted[1].to_string(), "fe80::b543:5d0b:f125:18d3");
    }

    #[test]
    fn routable_ipv6_outranks_link_local_ipv4() {
        let adapter = Adapter {
            luid: InterfaceLuid(1),
            guid: String::new(),
            index: 1,
            friendly_name: String::new(),
            description: String::new(),
            kind: AdapterKind::Ethernet,
            status: OperStatus::Up,
            mac: None,
            link_speed_bps: 0,
            unicast: vec![
                "169.254.11.136".parse().unwrap(),
                "2606:4700::1111".parse().unwrap(),
            ],
            gateways: vec![],
            dns_v4: vec![],
            dns_v6: vec![],
            metric_v4: 0,
            metric_v6: 0,
        };

        assert_eq!(
            adapter.display_addresses()[0].to_string(),
            "2606:4700::1111"
        );
    }
}
