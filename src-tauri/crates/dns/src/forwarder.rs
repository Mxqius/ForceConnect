//! The local resolver.
//!
//! Operating systems only accept plain IP addresses as an adapter's DNS
//! servers, so reaching an encrypted provider means running a resolver here:
//! plain DNS in from the OS on `127.0.0.1:53`, encrypted DNS out to the
//! provider.
//!
//! On Linux that is compatible with systemd-resolved's stub listener, which
//! sits on `127.0.0.53:53`: the two addresses do not collide, and resolved
//! forwards to `127.0.0.1` like any other configured server.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use futures_util::StreamExt;
use hickory_proto::op::{DnsRequest, DnsRequestOptions, Header, Metadata, ResponseCode};
use hickory_resolver::NameServerPool;
use hickory_resolver::net::DnsHandle;
use hickory_resolver::net::runtime::{Time, TokioRuntimeProvider};
use hickory_server::server::{Request, RequestHandler, ResponseHandler, ResponseInfo};
use hickory_server::zone_handler::MessageResponseBuilder;

use crate::server::Endpoint;
use crate::upstream;

/// Where the forwarder listens. Not configurable: Windows, NetworkManager and
/// `networksetup` can only be pointed at port 53, so any other port would be
/// unreachable by the OS anyway.
pub const LISTEN_V4: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 53);

#[derive(Debug, thiserror::Error)]
pub enum ForwarderError {
    #[error("{0}")]
    Upstream(#[from] upstream::UpstreamError),

    #[error("port 53 is already in use by {holder}")]
    PortTaken { holder: String },

    #[error("could not bind {addr}: {source}")]
    Bind {
        addr: SocketAddr,
        source: std::io::Error,
    },
}

/// Counters the UI polls.
///
/// Polled rather than pushed: emitting an event per query would melt the
/// webview at any real query rate, and the UI only needs a rolling total.
#[derive(Debug, Default)]
pub struct Stats {
    pub total: AtomicU64,
    pub failed: AtomicU64,
}

impl Stats {
    pub fn snapshot(&self) -> (u64, u64) {
        (
            self.total.load(Ordering::Relaxed),
            self.failed.load(Ordering::Relaxed),
        )
    }
}

/// Forwards every query to one upstream pool.
pub struct Forwarder {
    pool: NameServerPool<TokioRuntimeProvider>,
    timeout: Duration,
    stats: Arc<Stats>,
}

impl Forwarder {
    pub fn new(endpoint: &Endpoint, timeout: Duration) -> Result<Self, ForwarderError> {
        let opts = upstream::resolver_opts(timeout, 2);
        let pool = upstream::pool(endpoint, opts, TokioRuntimeProvider::new())?;

        Ok(Self {
            pool,
            timeout,
            stats: Arc::new(Stats::default()),
        })
    }

    pub fn stats(&self) -> Arc<Stats> {
        Arc::clone(&self.stats)
    }
}

// The trait is declared with `#[async_trait]`, so the impl must carry it too —
// a bare `async fn` here fails to match the desugared lifetimes.
#[async_trait::async_trait]
impl RequestHandler for Forwarder {
    async fn handle_request<R: ResponseHandler, T: Time>(
        &self,
        request: &Request,
        mut response_handle: R,
    ) -> ResponseInfo {
        self.stats.total.fetch_add(1, Ordering::Relaxed);

        let Ok(info) = request.request_info() else {
            return reply_error(request, &mut response_handle, ResponseCode::FormErr).await;
        };
        let query = info.query.original().clone();

        let outbound = DnsRequest::from_query(query, DnsRequestOptions::default());
        let mut stream = self.pool.send(outbound);

        let reply = match tokio::time::timeout(self.timeout, stream.next()).await {
            Ok(Some(Ok(reply))) => reply,
            _ => {
                self.stats.failed.fetch_add(1, Ordering::Relaxed);
                return reply_error(request, &mut response_handle, ResponseCode::ServFail).await;
            }
        };

        let message = reply.into_message();

        // Restore the client's transaction id and flags: the upstream exchange
        // used its own, and returning those would break the client's matching.
        let mut metadata = Metadata::response_from_request(&request.metadata);
        metadata.response_code = message.response_code;
        metadata.authoritative = false;
        metadata.recursion_available = true;

        let builder = MessageResponseBuilder::from_message_request(request);
        let response = builder.build(
            metadata,
            message.answers.iter(),
            message.authorities.iter(),
            [],
            message.additionals.iter(),
        );

        match response_handle.send_response(response).await {
            Ok(info) => info,
            Err(err) => {
                tracing::debug!("could not send a response: {err}");
                fallback_info(request)
            }
        }
    }
}

async fn reply_error<R: ResponseHandler>(
    request: &Request,
    handle: &mut R,
    code: ResponseCode,
) -> ResponseInfo {
    let builder = MessageResponseBuilder::from_message_request(request);
    let response = builder.error_msg(&request.metadata, code);

    handle
        .send_response(response)
        .await
        .unwrap_or_else(|_| fallback_info(request))
}

/// Something to return when even sending a response failed.
fn fallback_info(request: &Request) -> ResponseInfo {
    ResponseInfo::from(Header {
        metadata: Metadata::response_from_request(&request.metadata),
        counts: Default::default(),
    })
}

/// Check whether port 53 can be bound, and say who has it if not.
///
/// Called before starting so the failure carries a remedy, rather than
/// surfacing as an opaque bind error after the UI has said "connecting".
pub fn check_port_available() -> Result<(), ForwarderError> {
    use forceconnect_net::ports;

    match ports::udp_port_owner(LISTEN_V4.port()) {
        Some(owner) => Err(ForwarderError::PortTaken {
            holder: owner.describe(),
        }),
        None => Ok(()),
    }
}

/// Whether a name should be answered with NXDOMAIN rather than forwarded.
///
/// Firefox reads NXDOMAIN for this name as "the network operator disabled
/// DoH" and stops upgrading. Without it, a user running a local filtering
/// resolver gets silently bypassed by their browser and the filtering does
/// nothing.
pub fn is_doh_canary(name: &str) -> bool {
    name.trim_end_matches('.')
        .eq_ignore_ascii_case("use-application-dns.net")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn listens_only_on_loopback() {
        // Binding anywhere else would expose the resolver to the network.
        assert!(LISTEN_V4.ip().is_loopback());
        assert_eq!(LISTEN_V4.port(), 53);
    }

    #[test]
    fn the_firefox_canary_is_recognised() {
        assert!(is_doh_canary("use-application-dns.net"));
        assert!(is_doh_canary("use-application-dns.net."));
        assert!(is_doh_canary("USE-APPLICATION-DNS.NET"));
        assert!(!is_doh_canary("example.com"));
    }

    #[test]
    fn stats_count_totals_and_failures_separately() {
        let stats = Stats::default();
        assert_eq!(stats.snapshot(), (0, 0));
        stats.total.fetch_add(3, Ordering::Relaxed);
        stats.failed.fetch_add(1, Ordering::Relaxed);
        assert_eq!(stats.snapshot(), (3, 1));
    }
}
