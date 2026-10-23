//! The local resolver's command surface.
//!
//! Encrypted protocols cannot be handed to the OS — it takes plain IP
//! addresses and nothing else — so connecting to one means starting the
//! resolver here and pointing the adapter at loopback. The frontend drives the
//! order: start (or switch) the resolver, then write the addresses this
//! returns to the adapter; on disconnect, release the adapter, then stop.
//! Doing it the other way round would leave the adapter pointing at a port
//! nothing answers on.

use std::time::UNIX_EPOCH;

use forceconnect_dns::querylog::Outcome;
use forceconnect_dns::{Endpoint as CatalogEndpoint, ForwarderError, LocalResolver, leak};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

use crate::commands::probe::{find_endpoint, to_catalog_protocol};
use crate::commands::servers::Protocol;
use crate::commands::{CommandError, CommandResult};
use crate::state::{AppState, RunningResolver};

impl From<ForwarderError> for CommandError {
    fn from(value: ForwarderError) -> Self {
        match value {
            // Same treatment as an adapter write without rights: the UI
            // explains it rather than offering a retry that cannot work.
            ForwarderError::NeedsPrivilege => Self::Elevation(value.to_string()),
            other => Self::Network(other.to_string()),
        }
    }
}

/// A server and one of its protocols, as the frontend names them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpstreamRef {
    pub server_id: String,
    pub protocol: Protocol,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ResolverStatus {
    pub running: bool,
    pub server_id: Option<String>,
    pub protocol: Option<Protocol>,
    /// The server asked when the chosen one does not answer, if one is set.
    pub backup: Option<UpstreamRef>,
    /// The chosen server has stopped answering and lookups are going to the
    /// backup first, until it is tried again.
    pub using_backup: bool,
    /// The addresses the adapter should be pointed at — `127.0.0.1`, and `::1`
    /// when the machine has IPv6.
    pub listen: Vec<String>,
    /// Counters since the resolver started. `u32` and saturating rather than
    /// the `u64` underneath: it crosses into JavaScript as a plain number,
    /// and four billion queries is more than any session will see.
    pub queries: u32,
    pub failed: u32,
    /// Queries answered from the cache without going upstream.
    pub cached: u32,
    /// Queries the backup answered.
    pub answered_by_backup: u32,
    pub cache_entries: u32,
    /// Lookups no server answered in a row, reset by the next answer and by a
    /// switch of server. What the UI warns on: a growing streak means nothing
    /// is reachable — blocked, down, or the network gone — while the adapter
    /// still points at us, so nothing resolves.
    pub failing_streak: u32,
}

fn clamp(value: u64) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

impl ResolverStatus {
    fn stopped() -> Self {
        Self {
            running: false,
            server_id: None,
            protocol: None,
            backup: None,
            using_backup: false,
            listen: Vec::new(),
            queries: 0,
            failed: 0,
            cached: 0,
            answered_by_backup: 0,
            cache_entries: 0,
            failing_streak: 0,
        }
    }

    fn of(running: &RunningResolver) -> Self {
        let forwarder = running.resolver.forwarder();
        let stats = forwarder.stats().snapshot();

        Self {
            running: true,
            server_id: Some(running.server_id.clone()),
            protocol: Some(running.protocol),
            backup: running.backup.clone(),
            using_backup: forwarder.using_backup(),
            listen: running
                .resolver
                .listening()
                .iter()
                .map(|addr| addr.ip().to_string())
                .collect(),
            queries: clamp(stats.total),
            failed: clamp(stats.failed),
            cached: clamp(stats.cached),
            answered_by_backup: clamp(stats.backup),
            cache_entries: u32::try_from(forwarder.cache_len()).unwrap_or(u32::MAX),
            failing_streak: clamp(stats.failing_streak),
        }
    }
}

/// Forward to a server through the local resolver, starting it if needed.
///
/// Already running, this swaps the upstream in place: the listener stays up,
/// so a switch between two encrypted servers never has a moment where the
/// adapter points at a port nothing answers on.
#[tauri::command]
#[specta::specta]
pub async fn start_resolver(
    app: AppHandle,
    state: State<'_, AppState>,
    server_id: String,
    protocol: Protocol,
    backup: Option<UpstreamRef>,
) -> CommandResult<ResolverStatus> {
    let endpoint = find_endpoint(&app, &server_id, to_catalog_protocol(protocol))?;
    let (backup, backup_endpoint) = resolve_backup(&app, &server_id, backup);
    let mut slot = state.resolver().lock().await;

    match slot.as_mut() {
        Some(running) => {
            running
                .resolver
                .forwarder()
                .switch(&endpoint, backup_endpoint.as_ref())?;
            running.server_id = server_id;
            running.protocol = protocol;
            running.backup = backup;
            tracing::info!(server = %running.server_id, ?protocol, "local resolver switched upstream");
        }
        None => {
            let resolver = start(&state, &endpoint, backup_endpoint.as_ref()).await?;
            *slot = Some(RunningResolver {
                resolver,
                server_id,
                protocol,
                backup,
            });
        }
    }

    Ok(slot
        .as_ref()
        .map_or_else(ResolverStatus::stopped, ResolverStatus::of))
}

/// The backup's endpoint, or none.
///
/// A backup that cannot be found — a custom server deleted since it was
/// chosen — or that is the primary itself is dropped rather than failing the
/// connect: the connection the user asked for does not depend on it.
fn resolve_backup(
    app: &AppHandle,
    primary_id: &str,
    backup: Option<UpstreamRef>,
) -> (Option<UpstreamRef>, Option<CatalogEndpoint>) {
    let Some(backup) = backup.filter(|backup| backup.server_id != primary_id) else {
        return (None, None);
    };

    match find_endpoint(app, &backup.server_id, to_catalog_protocol(backup.protocol)) {
        Ok(endpoint) => (Some(backup), Some(endpoint)),
        Err(err) => {
            tracing::warn!("ignoring the backup server: {err}");
            (None, None)
        }
    }
}

/// Bind and start. Where port 53 needs rights this process lacks — Linux —
/// the privileged agent binds it and hands the sockets over.
async fn start(
    state: &AppState,
    endpoint: &CatalogEndpoint,
    backup: Option<&CatalogEndpoint>,
) -> Result<LocalResolver, CommandError> {
    match LocalResolver::start(endpoint, backup).await {
        #[cfg(unix)]
        Err(ForwarderError::NeedsPrivilege) => {
            let privileged = state.dns();
            let sockets =
                tauri::async_runtime::spawn_blocking(move || privileged.open_resolver_sockets())
                    .await
                    .map_err(|err| CommandError::Internal(err.to_string()))??;

            Ok(LocalResolver::start_with(
                endpoint,
                backup,
                bound_sockets(sockets)?,
            )?)
        }
        other => {
            let _ = state;
            Ok(other?)
        }
    }
}

/// Sort the agent's sockets into the shape the resolver takes.
///
/// Compiled everywhere, though only Linux and macOS call it, so the build that
/// runs on every platform checks it too.
#[cfg_attr(not(unix), allow(dead_code))]
fn bound_sockets(
    sockets: Vec<forceconnect_privileged::BoundSocket>,
) -> Result<forceconnect_dns::BoundSockets, CommandError> {
    use forceconnect_privileged::BoundSocket;

    let (mut udp_v4, mut tcp_v4, mut udp_v6, mut tcp_v6) = (None, None, None, None);
    for socket in sockets {
        let spec = socket
            .spec()
            .map_err(|err| CommandError::Internal(err.to_string()))?;
        match (socket, spec.addr.is_ipv4()) {
            (BoundSocket::Udp(udp), true) => udp_v4 = Some(udp),
            (BoundSocket::Tcp(tcp), true) => tcp_v4 = Some(tcp),
            (BoundSocket::Udp(udp), false) => udp_v6 = Some(udp),
            (BoundSocket::Tcp(tcp), false) => tcp_v6 = Some(tcp),
        }
    }

    let missing = || CommandError::Internal("the agent did not hand over the IPv4 sockets".into());
    Ok(forceconnect_dns::BoundSockets {
        udp_v4: udp_v4.ok_or_else(missing)?,
        tcp_v4: tcp_v4.ok_or_else(missing)?,
        v6: udp_v6.zip(tcp_v6),
    })
}

/// Stop the local resolver. A no-op when it is not running.
#[tauri::command]
#[specta::specta]
pub async fn stop_resolver(state: State<'_, AppState>) -> CommandResult<()> {
    let running = state.resolver().lock().await.take();
    if let Some(running) = running {
        running.resolver.stop().await;
    }
    Ok(())
}

/// What the resolver is doing, for the connection screen to poll.
#[tauri::command]
#[specta::specta]
pub async fn resolver_status(state: State<'_, AppState>) -> CommandResult<ResolverStatus> {
    let slot = state.resolver().lock().await;
    Ok(slot
        .as_ref()
        .map_or_else(ResolverStatus::stopped, ResolverStatus::of))
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "lowercase")]
pub enum QueryOutcome {
    Answered,
    Backup,
    Cached,
    Failed,
    Blocked,
}

/// One lookup the resolver handled.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct QueryLogEntry {
    pub id: u32,
    /// Milliseconds since the Unix epoch. A float, not a `u64`, so it crosses
    /// into JavaScript as an ordinary number.
    pub at: f64,
    pub name: String,
    pub record_type: String,
    pub outcome: QueryOutcome,
    pub response_code: String,
    pub elapsed_ms: f64,
}

/// Lookups newer than `after`, oldest first. Empty when the resolver is not
/// running: the log is memory only and goes with it.
#[tauri::command]
#[specta::specta]
pub async fn resolver_log(
    state: State<'_, AppState>,
    after: u32,
) -> CommandResult<Vec<QueryLogEntry>> {
    let slot = state.resolver().lock().await;
    let Some(running) = slot.as_ref() else {
        return Ok(Vec::new());
    };

    Ok(running
        .resolver
        .forwarder()
        .log()
        .since(u64::from(after))
        .into_iter()
        .map(|entry| QueryLogEntry {
            id: clamp(entry.id),
            at: entry
                .at
                .duration_since(UNIX_EPOCH)
                .map_or(0.0, |d| d.as_secs_f64() * 1000.0),
            name: entry.name,
            record_type: entry.record_type,
            outcome: match entry.outcome {
                Outcome::Answered => QueryOutcome::Answered,
                Outcome::Backup => QueryOutcome::Backup,
                Outcome::Cached => QueryOutcome::Cached,
                Outcome::Failed => QueryOutcome::Failed,
                Outcome::Blocked => QueryOutcome::Blocked,
            },
            response_code: entry.response_code,
            elapsed_ms: entry.elapsed.as_secs_f64() * 1000.0,
        })
        .collect())
}

#[tauri::command]
#[specta::specta]
pub async fn clear_resolver_log(state: State<'_, AppState>) -> CommandResult<()> {
    if let Some(running) = state.resolver().lock().await.as_ref() {
        running.resolver.forwarder().log().clear();
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LeakReport {
    /// Whether a lookup made through the operating system reached the local
    /// resolver. `None` when the connection is plain DNS, which does not pass
    /// through anything of ours to check against.
    pub through_resolver: Option<bool>,
    /// The resolver address the internet saw the lookup come from.
    pub egress: Vec<String>,
    /// Who that address belongs to, when it could be found out.
    pub egress_owner: Option<String>,
}

/// Check where this machine's lookups actually go.
///
/// `server_id` and `protocol` name the connected server: the ownership
/// lookups are sent to it directly, over its own protocol, so the test adds no
/// lookups of its own to whatever path is being tested.
#[tauri::command]
#[specta::specta]
pub async fn leak_test(
    app: AppHandle,
    state: State<'_, AppState>,
    server_id: String,
    protocol: Protocol,
) -> CommandResult<LeakReport> {
    let endpoint = find_endpoint(&app, &server_id, to_catalog_protocol(protocol))?;

    // Without this, the system cache could answer `whoami` from before the
    // connection changed, and report the old resolver.
    let privileged = state.dns();
    let _ = tauri::async_runtime::spawn_blocking(move || privileged.flush_cache()).await;

    let forwarder = state
        .resolver()
        .lock()
        .await
        .as_ref()
        .map(|running| running.resolver.forwarder().clone());

    let nonce = leak::nonce_name();
    leak::system_lookup(&nonce).await;
    let through_resolver = forwarder.map(|forwarder| forwarder.log().saw(&nonce));

    let egress = leak::system_lookup(leak::WHOAMI).await;
    let egress_owner = match egress.first() {
        Some(ip) => leak::owner_of(*ip, &endpoint).await,
        None => None,
    };

    Ok(LeakReport {
        through_resolver,
        egress: egress.iter().map(ToString::to_string).collect(),
        egress_owner,
    })
}
