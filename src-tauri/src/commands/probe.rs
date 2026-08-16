use forceconnect_dns::probe::{ProbeSpec, Verdict, probe_endpoint};
use forceconnect_dns::{Catalog, Protocol as CatalogProtocol};
use futures_util::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};

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
        })
        .buffer_unordered(8)
        .collect::<Vec<_>>()
        .await;

    Ok(results)
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
