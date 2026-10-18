//! Adapter enumeration and DNS configuration reads.
//!
//! This crate owns the notion of *adapter identity* for the whole application.
//! Reads (enumeration, current DNS settings, routes) live here; writes live in
//! `forceconnect-privileged`, which builds on these types. One implementation,
//! one notion of identity — if the reading and the writing sides marshalled
//! adapters independently they could disagree about which adapter the user
//! picked, a bug that surfaces as "I set DNS and nothing happened".
//!
//! **This crate is read-only.** Nothing here changes system state, which is
//! what keeps the privileged surface down to one small crate a reviewer can
//! read in a sitting.
//!
//! # Platforms
//!
//! The public API — [`adapters`], [`ports`], [`route`] — is the same on every
//! platform. What sits underneath is not:
//!
//! - **Windows** uses the IP Helper API. `SetInterfaceDnsSettings` /
//!   `GetInterfaceDnsSettings` are documented as Windows 10 build 19041 (2004)
//!   or later, which is the supported floor for the app.
//! - **Linux** reads `getifaddrs`, sysfs and procfs for the adapters, and asks
//!   NetworkManager (`nmcli`) or systemd-resolved (`resolvectl`) about DNS.
//! - **macOS** reads `getifaddrs` for the interfaces and `networksetup` /
//!   `scutil` for the network services and DNS layered on top of them.
//!
//! The text parsers behind the Linux and macOS readers live in `linux` and
//! `macos`. They are pure, so they are compiled on every platform — the
//! parsing is the part most likely to break, and this way it is tested
//! wherever `cargo test` runs, not only on the platform it parses.

pub mod adapters;
pub mod ports;
pub mod route;

pub mod linux;
pub mod macos;
#[cfg(unix)]
mod unix;

use std::net::IpAddr;

use serde::{Deserialize, Serialize};

/// Which address family an operation applies to.
///
/// The two are always handled separately: `DNS_SETTING_IPV6` selects the stack
/// a call applies to, and a server list has to match the family it is written
/// to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Family {
    V4,
    V6,
}

/// Identifies an adapter by LUID rather than by name.
///
/// Interface indices are explicitly not persistent across adapter
/// disable/enable, and friendly names are localized, user-renameable, and may
/// contain spaces — all of which broke the previous `netsh`-based
/// implementation.
///
/// Linux and macOS have no LUID. There the stable identity of an interface is
/// its kernel name (`wlp3s0`, `en0`) — the index is no more persistent than on
/// Windows — so the id is derived from the name with [`Self::from_name`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InterfaceLuid(pub u64);

impl InterfaceLuid {
    /// A stable id for an interface known by name.
    ///
    /// FNV-1a: deterministic across runs and builds, which `std`'s hasher is
    /// explicitly not. The value is masked to 53 bits so it would survive a
    /// round trip through a JavaScript number, though it crosses the IPC
    /// boundary as a string regardless.
    pub fn from_name(name: &str) -> Self {
        const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
        const PRIME: u64 = 0x0000_0100_0000_01b3;

        let hash = name.bytes().fold(OFFSET, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(PRIME)
        });
        Self(hash & ((1 << 53) - 1))
    }
}

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
    /// The key DNS settings are written under.
    ///
    /// On Windows, the adapter GUID — what the DNS settings API is keyed on.
    /// Elsewhere it names whatever owns the adapter's DNS: see
    /// `linux::DnsTarget` and `macos::DnsTarget`.
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
    /// adapter Windows actually routes through. Linux reports the metric of
    /// the adapter's default route; macOS has no metric, so the network
    /// service order stands in for it.
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
    #[error("the system network API failed: {0}")]
    Api(String),

    #[error("no adapter matches {0:?}")]
    UnknownAdapter(InterfaceLuid),

    #[error("adapter GUID {0:?} could not be parsed")]
    BadGuid(String),

    #[error("this operation requires Windows 10 build 19041 or later")]
    Unsupported,

    /// Nothing on this machine manages the adapter's DNS in a way the app
    /// knows how to drive — no NetworkManager connection and no
    /// systemd-resolved on Linux, no network service on macOS.
    #[error("{0}")]
    Unmanaged(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_derived_ids_are_stable_and_distinct() {
        // Stability across runs is the whole point: the frontend holds on to
        // this id between calls, and the journal outlives the process.
        assert_eq!(
            InterfaceLuid::from_name("en0"),
            InterfaceLuid::from_name("en0")
        );
        assert_ne!(
            InterfaceLuid::from_name("en0"),
            InterfaceLuid::from_name("en1")
        );
        // The published FNV-1a test vector, masked to 53 bits.
        assert_eq!(
            InterfaceLuid::from_name("a").0,
            0xaf63_dc4c_8601_ec8c & ((1 << 53) - 1)
        );
        assert!(InterfaceLuid::from_name("wlp3s0").0 < 1 << 53);
    }

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
