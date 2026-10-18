//! The privileged command surface.
//!
//! These are the only commands that change anything about the machine. Each one
//! validates its arguments here, then hands off to `forceconnect-privileged`,
//! which validates again against live system state — the frontend's idea of
//! which adapter exists is a cache, and a cache can be stale.
//!
//! There is no administrator prompt at this layer. The application asks once,
//! at launch — the Windows manifest, or the agent's prompt on Linux and macOS —
//! so by the time a user reaches these commands the rights are already held,
//! or every one of them refuses with an elevation error the UI explains.

use std::net::IpAddr;

use forceconnect_net::InterfaceLuid;
use forceconnect_privileged::PrivilegedError;
use tauri::State;

use crate::commands::{CommandError, CommandResult};
use crate::state::AppState;

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
/// alone while IPv6 DNS stays on DHCP lets the OS resolve around the change.
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

    blocking(move || writer.set_dns(luid, &servers)).await
}

/// Return an adapter to DHCP-assigned DNS for both families.
#[tauri::command]
#[specta::specta]
pub async fn clear_dns(state: State<'_, AppState>, luid: String) -> CommandResult<()> {
    let luid = parse_luid(&luid)?;
    let writer = state.dns();

    blocking(move || writer.clear_dns(luid)).await
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

    blocking(move || writer.flush_cache()).await
}

/// Run a privileged operation off the async runtime.
///
/// The calls underneath are synchronous and can take tens of milliseconds —
/// enumerating adapters is a full system query, and on Linux and macOS each
/// write spawns a system tool — which is long enough to be felt as a stutter
/// if it runs on a runtime worker. During launch they can also wait out the
/// administrator prompt.
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
