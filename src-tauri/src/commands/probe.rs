use forceconnect_dns::probe::{ProbeSpec, Verdict, probe_endpoint};
use forceconnect_dns::{Catalog, Endpoint as CatalogEndpoint, Protocol as CatalogProtocol};
use futures_util::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::commands::servers::Protocol;
use crate::commands::{CommandError, CommandResult};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct LatencyResult {
    pub server_id: String,
    pub protocol: Protocol,
    /// `good` | `degraded` | `unreachable`.
    pub verdict: String,
    pub median_ms: Option<f64>,
    /// One-time connection setup cost, separate from steady-state latency.
    ///
    /// Surfaced rather than folded in because it is paid once per connection.
    /// Hiding it makes encrypted protocols look permanently slow when they are
    /// only slow on the first exchange.
    pub handshake_ms: Option<f64>,
    pub jitter_ms: Option<f64>,
    pub answered: u8,
    pub rounds: u8,
}

/// Probe every server that offers a protocol.
///
/// Concurrency is bounded: firing 20+ TLS and QUIC handshakes at once is both
/// slow and a distinctive network signature. Rounds within one server stay
/// sequential, since concurrent rounds would measure connection multiplexing
/// rather than round-trip time.
#[tauri::command]
#[specta::specta]
pub async fn probe_servers(protocol: Protocol) -> CommandResult<Vec<LatencyResult>> {
    let catalog = Catalog::builtin().map_err(CommandError::from)?;
    let wanted = to_catalog_protocol(protocol);

    let targets: Vec<_> = catalog
        .servers()
        .iter()
        .filter_map(|server| {
            server
                .endpoint(wanted)
                .map(|endpoint| (server.id.clone(), endpoint.clone()))
        })
        .collect();

    let results = stream::iter(targets)
        .map(|(server_id, endpoint)| async move {
            let result = probe_endpoint(&endpoint, ProbeSpec::default()).await;
            to_dto(server_id, protocol, result)
        })
        .buffer_unordered(8)
        .collect::<Vec<_>>()
        .await;

    Ok(results)
}

/// Probe one server, by id.
///
/// The connection screen needs a latency figure for the server the user is
/// actually on, and re-probing the whole catalogue to get one number would fire
/// twenty-odd handshakes for a single row. Custom servers are searched too — a
/// server the user added is exactly the one whose latency they cannot look up
/// anywhere else.
#[tauri::command]
#[specta::specta]
pub async fn probe_server(
    app: AppHandle,
    server_id: String,
    protocol: Protocol,
) -> CommandResult<LatencyResult> {
    let wanted = to_catalog_protocol(protocol);
    let endpoint = find_endpoint(&app, &server_id, wanted)?;

    let result = probe_endpoint(&endpoint, ProbeSpec::default()).await;
    Ok(to_dto(server_id, protocol, result))
}

/// Locate an endpoint in the built-in catalog, then among the user's own.
fn find_endpoint(
    app: &AppHandle,
    server_id: &str,
    protocol: CatalogProtocol,
) -> Result<CatalogEndpoint, CommandError> {
    let builtin = Catalog::builtin().map_err(CommandError::from)?;

    let server = builtin.get(server_id).cloned().map_or_else(
        || {
            crate::commands::custom::read_all(app)?
                .into_iter()
                .find(|s| s.id == server_id)
                .ok_or_else(|| CommandError::Catalog(format!("no server '{server_id}'")))
        },
        Ok,
    )?;

    server.endpoint(protocol).cloned().ok_or_else(|| {
        CommandError::Catalog(format!(
            "{} does not offer {}",
            server.display_name(),
            protocol.label()
        ))
    })
}

fn to_dto(
    server_id: String,
    protocol: Protocol,
    result: forceconnect_dns::probe::ProbeResult,
) -> LatencyResult {
    LatencyResult {
        server_id,
        protocol,
        verdict: match result.verdict {
            Verdict::Good => "good",
            Verdict::Degraded => "degraded",
            Verdict::Unreachable => "unreachable",
        }
        .to_owned(),
        median_ms: result.median_ms,
        handshake_ms: result.handshake_ms,
        jitter_ms: result.jitter_ms,
        answered: result.answered,
        rounds: result.rounds,
    }
}

fn to_catalog_protocol(protocol: Protocol) -> CatalogProtocol {
    match protocol {
        Protocol::Udp => CatalogProtocol::Udp,
        Protocol::Tcp => CatalogProtocol::Tcp,
        Protocol::Dot => CatalogProtocol::Dot,
        Protocol::Doh => CatalogProtocol::Doh,
        Protocol::Doq => CatalogProtocol::Doq,
        Protocol::Doh3 => CatalogProtocol::Doh3,
    }
}
