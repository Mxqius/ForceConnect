//! The privileged command surface.
//!
//! These are the only commands that change anything about the machine. Each one
//! validates its arguments here, then hands off to `forceconnect-privileged`,
//! which validates again against live system state — the frontend's idea of
//! which adapter exists is a cache, and a cache can be stale.
//!
//! There is no UAC prompt at this layer any more. The application requests
//! administrator rights once, at launch, so by the time a user reaches these
//! commands the rights are already held or the app never started.

use std::net::IpAddr;

use forceconnect_net::{DnsConfig, Family, InterfaceLuid};
use forceconnect_privileged::PrivilegedError;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::commands::adapters::DnsSource;
use crate::commands::{CommandError, CommandResult};
use crate::state::{AppState, lock};

impl From<PrivilegedError> for CommandError {
    fn from(value: PrivilegedError) -> Self {
        match value {
            // Distinct from an ordinary failure: nothing the user does in the
            // app will fix it, and the UI says so rather than offering a retry.
            PrivilegedError::NotElevated => Self::Elevation(value.to_string()),
            other => Self::Network(other.to_string()),
        }
    }
}

/// Point an adapter at a list of DNS servers.
///
/// Both address families present in `servers` are applied, because setting IPv4
/// alone while IPv6 DNS stays on DHCP lets Windows resolve around the change.
#[tauri::command]
#[specta::specta]
pub async fn set_dns(
    state: State<'_, AppState>,
    luid: String,
    servers: Vec<String>,
) -> CommandResult<()> {
    let luid = parse_luid(&luid)?;
    let servers = parse_servers(&servers)?;
    let writer = state.dns();

    blocking(move || lock(&writer).set_dns(luid, &servers)).await
}

/// Return an adapter to DHCP-assigned DNS for both families.
#[tauri::command]
#[specta::specta]
pub async fn clear_dns(state: State<'_, AppState>, luid: String) -> CommandResult<()> {
    let luid = parse_luid(&luid)?;
    let writer = state.dns();

    blocking(move || lock(&writer).clear_dns(luid)).await
}

/// One address family's DNS, as the UI shows it.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct FamilyDns {
    pub source: DnsSource,
    pub servers: Vec<String>,
}

impl From<&DnsConfig> for FamilyDns {
    fn from(value: &DnsConfig) -> Self {
        Self {
            source: if value.is_dhcp() {
                DnsSource::Dhcp
            } else {
                DnsSource::Static
            },
            servers: value.servers().iter().map(ToString::to_string).collect(),
        }
    }
}

/// Where a released adapter went.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ReleasedDns {
    pub v4: Option<FamilyDns>,
    pub v6: Option<FamilyDns>,
}

/// Disconnect: hand an adapter back according to the release policy.
///
/// Distinct from `clear_dns`, which always means DHCP and stays as the "Reset
/// to automatic" tool. This is what the power button does.
#[tauri::command]
#[specta::specta]
pub async fn release_dns(state: State<'_, AppState>, luid: String) -> CommandResult<ReleasedDns> {
    let luid = parse_luid(&luid)?;
    let writer = state.dns();

    let written = tauri::async_runtime::spawn_blocking(move || lock(&writer).release(luid))
        .await
        .map_err(|err| CommandError::Internal(err.to_string()))??;

    let pick = |family| {
        written
            .iter()
            .find(|(f, _)| *f == family)
            .map(|(_, config)| FamilyDns::from(config))
    };

    Ok(ReleasedDns {
        v4: pick(Family::V4),
        v6: pick(Family::V6),
    })
}

/// Where adapters go on disconnect, on exit, and after a crash.
///
/// The choice is persisted in the frontend's preferences store, like the tray
/// setting; this is the copy the exit handler and the journal can see, pushed
/// across on hydrate and on every change.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "mode", rename_all = "camelCase")]
pub enum ReleasePolicy {
    /// Put back the DNS Windows had before ForceConnect started.
    Restore,
    /// Hand the adapter to DHCP.
    Automatic,
    /// Switch to the user's own default servers.
    Fallback { servers: Vec<String> },
}

#[tauri::command]
#[specta::specta]
pub fn set_release_policy(state: State<'_, AppState>, policy: ReleasePolicy) -> CommandResult<()> {
    let policy = match policy {
        ReleasePolicy::Restore => forceconnect_privileged::ReleasePolicy::Restore,
        ReleasePolicy::Automatic => forceconnect_privileged::ReleasePolicy::Automatic,
        ReleasePolicy::Fallback { servers } => {
            forceconnect_privileged::ReleasePolicy::Fallback(parse_servers(&servers)?)
        }
    };

    lock(&state.dns())
        .set_policy(policy)
        .map_err(CommandError::from)
}

/// One adapter's DNS as it stood when ForceConnect started.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AdapterBaseline {
    pub luid: String,
    pub name: String,
    pub v4: Option<FamilyDns>,
    pub v6: Option<FamilyDns>,
}

/// The DNS every adapter had at launch — what "Restore previous DNS" puts back.
///
/// Captured once at startup, after crash recovery, so it is the user's own
/// configuration rather than anything a previous run left behind.
#[tauri::command]
#[specta::specta]
pub fn dns_baseline(state: State<'_, AppState>) -> CommandResult<Vec<AdapterBaseline>> {
    let adapters = forceconnect_net::adapters::list()?;
    let writer = state.dns();
    let writer = lock(&writer);

    Ok(adapters
        .iter()
        .filter_map(|adapter| {
            let v4 = writer
                .baseline(&adapter.guid, Family::V4)
                .map(FamilyDns::from);
            let v6 = writer
                .baseline(&adapter.guid, Family::V6)
                .map(FamilyDns::from);
            (v4.is_some() || v6.is_some()).then(|| AdapterBaseline {
                luid: adapter.luid.0.to_string(),
                name: adapter.friendly_name.clone(),
                v4,
                v6,
            })
        })
        .collect())
}

/// Drop every entry in the system resolver cache.
///
/// Worth its own action rather than being folded into `set_dns`: records
/// resolved through the previous server stay valid for their TTL, so a user who
/// switches server and immediately reloads a page can see the old answer and
/// conclude nothing happened. It is also useful on its own, with no DNS change
/// involved at all.
#[tauri::command]
#[specta::specta]
pub async fn flush_dns_cache(state: State<'_, AppState>) -> CommandResult<()> {
    let writer = state.dns();

    blocking(move || lock(&writer).flush_cache()).await
}

/// Run a privileged operation off the async runtime.
///
/// The Win32 calls underneath are synchronous and can take tens of
/// milliseconds — enumerating adapters is a full system query — which is long
/// enough to be felt as a stutter if it runs on a runtime worker.
async fn blocking<F>(work: F) -> CommandResult<()>
where
    F: FnOnce() -> Result<(), PrivilegedError> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(work)
        .await
        .map_err(|err| CommandError::Internal(err.to_string()))?
        .map_err(CommandError::from)
}

fn parse_luid(value: &str) -> Result<InterfaceLuid, CommandError> {
    value
        .parse::<u64>()
        .map(InterfaceLuid)
        .map_err(|_| CommandError::Internal(format!("'{value}' is not a valid interface id")))
}

fn parse_servers(servers: &[String]) -> Result<Vec<IpAddr>, CommandError> {
    if servers.is_empty() {
        return Err(CommandError::Internal("no servers were given".into()));
    }

    servers
        .iter()
        .map(|server| {
            server
                .trim()
                .parse::<IpAddr>()
                .map_err(|_| CommandError::Internal(format!("'{server}' is not an IP address")))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luids_round_trip_as_strings() {
        // Values beyond 2^53 are exactly why these cross as strings: JavaScript
        // would round them, and a rounded LUID targets a different adapter.
        let big = 0x8000_0000_0000_0001u64;
        assert_eq!(parse_luid(&big.to_string()).unwrap(), InterfaceLuid(big));
        assert!(parse_luid("not-a-number").is_err());
    }

    #[test]
    fn malformed_server_lists_are_rejected_at_the_boundary() {
        assert!(parse_servers(&[]).is_err());
        assert!(parse_servers(&["1.1.1.1".into(), "nope".into()]).is_err());
        assert_eq!(parse_servers(&[" 9.9.9.9 ".into()]).unwrap().len(), 1);
    }
}
