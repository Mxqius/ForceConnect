use std::net::IpAddr;

use forceconnect_ipc::{Family, InterfaceLuid, Request, Response};
use serde::{Deserialize, Serialize};
use tauri::{Manager, State, WebviewWindow};

use crate::commands::{CommandError, CommandResult};
use crate::elevate::{ElevationError, helper_path};
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct HelperStatus {
    /// Whether a helper session is live. `false` means the next privileged
    /// action will show a UAC prompt.
    pub connected: bool,
    pub version: Option<String>,
    /// Process id of the elevated helper, so a user can verify in Task Manager
    /// exactly which process is running with administrator rights.
    pub pid: Option<u32>,
}

impl From<ElevationError> for CommandError {
    fn from(value: ElevationError) -> Self {
        match value {
            // The user declining is a decision, not a fault, and the UI should
            // treat it as such — no error styling, no retry nagging.
            ElevationError::Declined => Self::Elevation("declined".into()),
            other => Self::Elevation(other.to_string()),
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn helper_status(state: State<'_, AppState>) -> CommandResult<HelperStatus> {
    let info = state.helper_info().await;

    Ok(HelperStatus {
        connected: info.is_some(),
        version: info.as_ref().map(|(version, _)| version.clone()),
        pid: info.map(|(_, pid)| pid),
    })
}

/// Point an adapter at a list of DNS servers.
///
/// Triggers a UAC prompt if no helper session exists yet. Both address families
/// present in `servers` are applied, because setting IPv4 alone while IPv6 DNS
/// stays on DHCP lets Windows resolve around the change entirely.
#[tauri::command]
#[specta::specta]
pub async fn set_dns(
    window: WebviewWindow,
    state: State<'_, AppState>,
    luid: String,
    servers: Vec<String>,
) -> CommandResult<()> {
    let luid = parse_luid(&luid)?;
    let parsed = parse_servers(&servers)?;
    let session = session(&window, &state).await?;

    for (family, list) in split_by_family(&parsed) {
        if list.is_empty() {
            continue;
        }

        let response = session
            .call(Request::SetDns {
                luid,
                family,
                servers: list,
            })
            .await;

        check(response, &state).await?;
    }

    Ok(())
}

/// Return an adapter to DHCP-assigned DNS for both families.
#[tauri::command]
#[specta::specta]
pub async fn clear_dns(
    window: WebviewWindow,
    state: State<'_, AppState>,
    luid: String,
) -> CommandResult<()> {
    let luid = parse_luid(&luid)?;
    let session = session(&window, &state).await?;

    for family in [Family::V4, Family::V6] {
        let response = session.call(Request::ClearDns { luid, family }).await;
        check(response, &state).await?;
    }

    Ok(())
}

async fn session(
    window: &WebviewWindow,
    state: &State<'_, AppState>,
) -> Result<std::sync::Arc<crate::elevate::client::HelperSession>, CommandError> {
    let path = helper_path(
        window
            .app_handle()
            .path()
            .resource_dir()
            .ok()
            .map(|dir| dir.to_path_buf()),
    )?;

    // Parenting the consent dialog to our window keeps it from opening behind
    // the app and merely flashing in the taskbar.
    let hwnd = window.hwnd().map(|h| h.0 as isize).unwrap_or(0);

    Ok(state.helper(path, hwnd).await?)
}

/// Map a helper reply to a command result, dropping a dead session so the next
/// call re-establishes rather than failing forever against a closed pipe.
async fn check(
    response: Result<Response, ElevationError>,
    state: &State<'_, AppState>,
) -> Result<(), CommandError> {
    match response {
        Ok(_) => Ok(()),
        Err(err @ (ElevationError::Io(_) | ElevationError::Connect(_))) => {
            state.drop_helper().await;
            Err(err.into())
        }
        Err(err) => Err(err.into()),
    }
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
        .map(|s| {
            s.trim()
                .parse::<IpAddr>()
                .map_err(|_| CommandError::Internal(format!("'{s}' is not an IP address")))
        })
        .collect()
}

/// Group servers by address family, since each family is a separate call.
fn split_by_family(servers: &[IpAddr]) -> [(Family, Vec<IpAddr>); 2] {
    [
        (
            Family::V4,
            servers.iter().copied().filter(IpAddr::is_ipv4).collect(),
        ),
        (
            Family::V6,
            servers.iter().copied().filter(IpAddr::is_ipv6).collect(),
        ),
    ]
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
    fn families_are_split_for_separate_calls() {
        let servers: Vec<IpAddr> = ["1.1.1.1", "2606:4700:4700::1111", "1.0.0.1"]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();

        let [(v4_family, v4), (v6_family, v6)] = split_by_family(&servers);
        assert_eq!(v4_family, Family::V4);
        assert_eq!(v4.len(), 2);
        assert_eq!(v6_family, Family::V6);
        assert_eq!(v6.len(), 1);
    }

    #[test]
    fn malformed_server_lists_are_rejected_before_elevation() {
        // Validating here means a bad list never reaches the point of showing
        // the user a UAC prompt.
        assert!(parse_servers(&[]).is_err());
        assert!(parse_servers(&["1.1.1.1".into(), "nope".into()]).is_err());
        assert_eq!(parse_servers(&[" 9.9.9.9 ".into()]).unwrap().len(), 1);
    }
}
