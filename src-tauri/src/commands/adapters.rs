use serde::{Deserialize, Serialize};

use crate::commands::{CommandError, CommandResult};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum AdapterKind {
    Ethernet,
    Wifi,
    /// VPN tunnels and virtual point-to-point adapters. Grouped separately
    /// because they routinely outrank the physical adapter carrying them.
    Tunnel,
    Loopback,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum AdapterStatus {
    Up,
    Down,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum DnsSource {
    /// Servers come from DHCP.
    Dhcp,
    /// Servers were configured explicitly — possibly by us.
    Static,
    /// The setting could not be read.
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Adapter {
    /// Interface LUID as a decimal string.
    ///
    /// A string rather than a number on purpose: a LUID is a genuine `u64` and
    /// can exceed 2^53, where JavaScript would silently round it. Rounding an
    /// adapter identifier means writing DNS to the wrong adapter.
    pub luid: String,
    pub guid: String,
    /// Not persistent across disable/enable — display and diagnostics only.
    pub index: u32,
    pub name: String,
    pub description: String,
    pub kind: AdapterKind,
    pub status: AdapterStatus,
    pub mac: Option<String>,
    /// Link speed in Mbps.
    ///
    /// `TransmitLinkSpeed` is in bits per second. The previous implementation
    /// ran it through a bytes-to-Mbps conversion and displayed a figure eight
    /// times too high.
    pub link_speed_mbps: u32,
    pub addresses: Vec<String>,
    pub gateways: Vec<String>,
    pub dns_v4: Vec<String>,
    pub dns_v6: Vec<String>,
    pub metric_v4: u32,
    pub metric_v6: u32,
    /// Whether this adapter is a plausible target for a DNS change.
    pub eligible: bool,
    /// Whether the routing table says traffic actually leaves by this adapter.
    pub is_active: bool,
    pub dns_source: DnsSource,
}

#[cfg(windows)]
fn collect() -> Result<Vec<Adapter>, CommandError> {
    use forceconnect_net::{
        AdapterKind as NetKind, DnsConfig, Family, OperStatus, adapters as net_adapters, route,
    };

    let raw = net_adapters::list()?;
    let active_luid = route::best_adapter(&raw, Family::V4).map(|a| a.luid);
    let eligible: Vec<_> = route::rank(&raw, Family::V4)
        .into_iter()
        .map(|a| a.luid)
        .collect();

    Ok(raw
        .iter()
        .map(|a| {
            let dns_source = if a.status == OperStatus::Up {
                match net_adapters::dns_config(&a.guid, Family::V4) {
                    Ok(DnsConfig::Dhcp) => DnsSource::Dhcp,
                    Ok(DnsConfig::Static(_)) => DnsSource::Static,
                    Err(_) => DnsSource::Unknown,
                }
            } else {
                DnsSource::Unknown
            };

            Adapter {
                luid: a.luid.0.to_string(),
                guid: a.guid.clone(),
                index: a.index,
                name: a.friendly_name.clone(),
                description: a.description.clone(),
                kind: match a.kind {
                    NetKind::Ethernet => AdapterKind::Ethernet,
                    NetKind::WiFi => AdapterKind::Wifi,
                    NetKind::Tunnel => AdapterKind::Tunnel,
                    NetKind::Loopback => AdapterKind::Loopback,
                    NetKind::Other => AdapterKind::Other,
                },
                status: match a.status {
                    OperStatus::Up => AdapterStatus::Up,
                    OperStatus::Down => AdapterStatus::Down,
                    _ => AdapterStatus::Other,
                },
                mac: a.mac.clone(),
                link_speed_mbps: u32::try_from(a.link_speed_bps / 1_000_000).unwrap_or(u32::MAX),
                addresses: a.unicast.iter().map(ToString::to_string).collect(),
                gateways: a.gateways.iter().map(ToString::to_string).collect(),
                dns_v4: a.dns_v4.iter().map(ToString::to_string).collect(),
                dns_v6: a.dns_v6.iter().map(ToString::to_string).collect(),
                metric_v4: a.metric_v4,
                metric_v6: a.metric_v6,
                eligible: eligible.contains(&a.luid),
                is_active: active_luid == Some(a.luid),
                dns_source,
            }
        })
        .collect())
}

#[cfg(not(windows))]
fn collect() -> Result<Vec<Adapter>, CommandError> {
    Err(CommandError::Internal(
        "adapter enumeration is only implemented for Windows".into(),
    ))
}

/// Every adapter the system reports, best-first among the eligible ones.
///
/// Everything here is an unprivileged read, so this never triggers a UAC
/// prompt — browsing the app costs the user nothing.
#[tauri::command]
#[specta::specta]
pub fn list_adapters() -> CommandResult<Vec<Adapter>> {
    let mut adapters = collect()?;

    // Eligible adapters first, then by the metric that decides routing, so the
    // list reads top-down in the order Windows would choose.
    adapters.sort_by(|a, b| {
        b.eligible
            .cmp(&a.eligible)
            .then_with(|| a.metric_v4.cmp(&b.metric_v4))
            .then_with(|| a.index.cmp(&b.index))
    });

    Ok(adapters)
}
