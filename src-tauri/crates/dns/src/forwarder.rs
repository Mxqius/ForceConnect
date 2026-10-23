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

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

use futures_util::StreamExt;
use hickory_proto::op::{
    DnsRequest, DnsRequestOptions, Edns, Header, Message, MessageType, Metadata, OpCode, Query,
    ResponseCode,
};
use hickory_proto::rr::Record;
use hickory_resolver::NameServerPool;
use hickory_resolver::net::runtime::{Time, TokioRuntimeProvider};
use hickory_resolver::net::{DnsError, DnsHandle, NetError};
use hickory_server::Server;
use hickory_server::server::{Request, RequestHandler, ResponseHandler, ResponseInfo};
use hickory_server::zone_handler::MessageResponseBuilder;
use tokio::net::{TcpListener, UdpSocket};

use crate::cache::{Cache, QueryFlags};
use crate::querylog::{Outcome, QueryLog};
use crate::server::Endpoint;
use crate::upstream;

/// Where the forwarder listens. Not configurable: Windows, NetworkManager and
/// `networksetup` can only be pointed at port 53, so any other port would be
/// unreachable by the OS anyway.
pub const LISTEN_V4: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 53);

/// The IPv6 half. Without it, an adapter's IPv6 DNS stays on whatever the
/// network hands out and the OS resolves around us — see ARCHITECTURE.md,
/// "IPv6 will bypass you".
pub const LISTEN_V6: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 53);

/// How long an upstream exchange may take before the client gets SERVFAIL.
pub const UPSTREAM_TIMEOUT: Duration = Duration::from_secs(4);

/// How long an idle TCP client is kept. Clients fall back to TCP only for
/// truncated answers and close straight after; this just stops a stuck one
/// holding a task forever.
const TCP_IDLE: Duration = Duration::from_secs(10);

/// Outstanding responses buffered per TCP connection.
const TCP_RESPONSE_BUFFER: usize = 32;

#[derive(Debug, thiserror::Error)]
pub enum ForwarderError {
    #[error("{0}")]
    Upstream(#[from] upstream::UpstreamError),

    #[error("port 53 is already in use by {holder}")]
    PortTaken { holder: String },

    #[error(
        "port 53 on {addr} is reserved by the system or held exclusively by another program{}",
        holder.as_deref().map(|h| format!(" ({h})")).unwrap_or_default()
    )]
    Reserved {
        addr: SocketAddr,
        holder: Option<String>,
    },

    #[error("listening on port 53 needs administrator rights, which this process does not have")]
    NeedsPrivilege,

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
    /// Answered from the cache without going upstream.
    pub cached: AtomicU64,
    /// Answered by the backup server.
    pub backup: AtomicU64,
    /// Lookups no upstream answered — primary or backup — since the last one
    /// that was.
    ///
    /// The signal the UI warns on. A running total cannot tell "one timeout an
    /// hour ago" from "nothing has answered for a minute"; a streak can, and a
    /// blocked or dead upstream shows up as one that only grows. Cache hits do
    /// not reset it — they say nothing about whether the server is reachable.
    pub failing_streak: AtomicU64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatsSnapshot {
    pub total: u64,
    pub failed: u64,
    pub cached: u64,
    pub backup: u64,
    pub failing_streak: u64,
}

impl Stats {
    pub fn snapshot(&self) -> StatsSnapshot {
        StatsSnapshot {
            total: self.total.load(Ordering::Relaxed),
            failed: self.failed.load(Ordering::Relaxed),
            cached: self.cached.load(Ordering::Relaxed),
            backup: self.backup.load(Ordering::Relaxed),
            failing_streak: self.failing_streak.load(Ordering::Relaxed),
        }
    }

    fn upstream_failed(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
        self.failing_streak.fetch_add(1, Ordering::Relaxed);
    }

    fn upstream_answered(&self) {
        self.failing_streak.store(0, Ordering::Relaxed);
    }
}

/// The upstreams in use, and how many times they have been replaced.
///
/// The generation is what keeps a reply that was in flight across a switch
/// out of the cache: it comes from the old server, and the cache was emptied
/// precisely so nothing from the old server would be served afterwards.
struct Upstream {
    primary: Arc<Pool>,
    /// Asked when the primary does not answer. Optional, and the user's
    /// choice: a second server sees the lookups the first one misses.
    backup: Option<Arc<Pool>>,
    generation: u64,
}

type Pool = NameServerPool<TokioRuntimeProvider>;

/// Primary failures in a row before the backup is asked first.
///
/// Without this, a primary that has gone dark costs every lookup the full
/// upstream timeout before the backup is even tried — seconds per page.
const PRIMARY_BYPASS_AFTER: u64 = 3;

/// How long the primary is passed over once it has been, before live traffic
/// gives it another chance.
const PRIMARY_RETRY_AFTER: Duration = Duration::from_secs(30);

struct Inner {
    upstream: RwLock<Upstream>,
    cache: Cache,
    log: QueryLog,
    timeout: Duration,
    stats: Arc<Stats>,
    primary_failures: AtomicU64,
    primary_bypassed_until: Mutex<Option<Instant>>,
}

/// Forwards every query to its upstream, answering repeats from cache.
///
/// Cheap to clone: the server owns one copy and [`LocalResolver`] keeps
/// another, which is how the upstream can be swapped while the listener keeps
/// running.
#[derive(Clone)]
pub struct Forwarder {
    inner: Arc<Inner>,
}

impl Forwarder {
    pub fn new(
        primary: &Endpoint,
        backup: Option<&Endpoint>,
        timeout: Duration,
    ) -> Result<Self, ForwarderError> {
        Ok(Self {
            inner: Arc::new(Inner {
                upstream: RwLock::new(Upstream {
                    primary: Arc::new(build_pool(primary, timeout)?),
                    backup: build_backup(backup, timeout)?,
                    generation: 0,
                }),
                cache: Cache::default(),
                log: QueryLog::default(),
                timeout,
                stats: Arc::new(Stats::default()),
                primary_failures: AtomicU64::new(0),
                primary_bypassed_until: Mutex::new(None),
            }),
        })
    }

    pub fn stats(&self) -> Arc<Stats> {
        Arc::clone(&self.inner.stats)
    }

    pub fn log(&self) -> &QueryLog {
        &self.inner.log
    }

    /// Point at different upstreams without dropping the listener.
    ///
    /// The OS keeps sending to `127.0.0.1` throughout, so a switch between two
    /// encrypted servers has no moment where lookups fail or fall back to the
    /// network's resolver. The cache is emptied: its answers came from the
    /// previous server, which may well be one that filters differently.
    pub fn switch(
        &self,
        primary: &Endpoint,
        backup: Option<&Endpoint>,
    ) -> Result<(), ForwarderError> {
        let pool = Arc::new(build_pool(primary, self.inner.timeout)?);
        let backup = build_backup(backup, self.inner.timeout)?;

        let mut upstream = self
            .inner
            .upstream
            .write()
            .unwrap_or_else(|err| err.into_inner());
        upstream.primary = pool;
        upstream.backup = backup;
        upstream.generation += 1;
        drop(upstream);

        self.inner.cache.clear();
        // The streaks described the old servers; the new ones start clean.
        self.inner.stats.upstream_answered();
        self.primary_answered();
        Ok(())
    }

    pub fn clear_cache(&self) {
        self.inner.cache.clear();
    }

    pub fn cache_len(&self) -> usize {
        self.inner.cache.len()
    }

    pub fn has_backup(&self) -> bool {
        self.read_upstream().backup.is_some()
    }

    /// Whether lookups are currently going to the backup first, because the
    /// primary stopped answering.
    pub fn using_backup(&self) -> bool {
        self.has_backup() && self.primary_bypassed()
    }

    fn read_upstream(&self) -> std::sync::RwLockReadGuard<'_, Upstream> {
        self.inner
            .upstream
            .read()
            .unwrap_or_else(|err| err.into_inner())
    }

    fn upstream(&self) -> (Arc<Pool>, Option<Arc<Pool>>, u64) {
        let upstream = self.read_upstream();
        (
            Arc::clone(&upstream.primary),
            upstream.backup.clone(),
            upstream.generation,
        )
    }

    fn generation(&self) -> u64 {
        self.read_upstream().generation
    }

    fn bypass_slot(&self) -> std::sync::MutexGuard<'_, Option<Instant>> {
        self.inner
            .primary_bypassed_until
            .lock()
            .unwrap_or_else(|err| err.into_inner())
    }

    fn primary_bypassed(&self) -> bool {
        self.bypass_slot()
            .is_some_and(|until| Instant::now() < until)
    }

    fn primary_answered(&self) {
        self.inner.primary_failures.store(0, Ordering::Relaxed);
        *self.bypass_slot() = None;
    }

    fn primary_failed(&self, has_backup: bool) {
        let failures = self.inner.primary_failures.fetch_add(1, Ordering::Relaxed) + 1;
        if has_backup && failures >= PRIMARY_BYPASS_AFTER {
            *self.bypass_slot() = Some(Instant::now() + PRIMARY_RETRY_AFTER);
        }
    }

    /// Ask the primary, then the backup if there is one and the primary did
    /// not answer — or the backup straight away, while the primary is being
    /// passed over.
    async fn resolve(
        &self,
        primary: &Pool,
        backup: Option<&Pool>,
        query: &Query,
        flags: QueryFlags,
    ) -> Option<(Message, Outcome)> {
        if backup.is_none() || !self.primary_bypassed() {
            if let Some(message) = self.exchange(primary, query, flags).await {
                self.primary_answered();
                return Some((message, Outcome::Answered));
            }
            self.primary_failed(backup.is_some());
        }

        let message = self.exchange(backup?, query, flags).await?;
        Some((message, Outcome::Backup))
    }

    async fn exchange(&self, pool: &Pool, query: &Query, flags: QueryFlags) -> Option<Message> {
        let mut options = DnsRequestOptions::default();
        // Passed through rather than fixed, so a validating client gets the
        // signatures it asked for and a plain one is not sent bytes it ignores.
        options.edns_set_dnssec_ok = flags.dnssec_ok;
        let mut outbound = DnsRequest::from_query(query.clone(), options);
        outbound.metadata.checking_disabled = flags.checking_disabled;

        let mut stream = pool.send(outbound);
        match tokio::time::timeout(self.inner.timeout, stream.next()).await {
            Ok(Some(Ok(reply))) => Some(reply.into_message()),
            Ok(Some(Err(err))) => answered_negatively(err),
            _ => None,
        }
    }
}

/// The server's reply, when hickory has turned it into an error.
///
/// A reply with no answer records — NXDOMAIN, a name with nothing of the type
/// asked for, a REFUSED — comes back from the pool as an error, but it is
/// still the server's answer and the client is owed it as one. Reported as
/// SERVFAIL instead, a typo would send the client retrying, skip straight to
/// the backup, and count against the server's health as if it had gone dark.
fn answered_negatively(err: NetError) -> Option<Message> {
    let mut message = Message::new(0, MessageType::Response, OpCode::Query);
    match err {
        NetError::Dns(DnsError::NoRecordsFound(none)) => {
            message.metadata.response_code = none.response_code;
            // The authority section carries the SOA that sets how long the
            // answer may be cached, and any NSEC proof a validator needs.
            message.authorities = match (none.authorities, none.soa) {
                (Some(authorities), _) => authorities.to_vec(),
                (None, Some(soa)) => vec![soa.into_record_of_rdata()],
                (None, None) => Vec::new(),
            };
        }
        NetError::Dns(DnsError::ResponseCode(code)) => message.metadata.response_code = code,
        _ => return None,
    }
    Some(message)
}

fn build_pool(endpoint: &Endpoint, timeout: Duration) -> Result<Pool, ForwarderError> {
    let opts = upstream::resolver_opts(timeout, 2);
    Ok(upstream::pool(endpoint, opts, TokioRuntimeProvider::new())?)
}

fn build_backup(
    endpoint: Option<&Endpoint>,
    timeout: Duration,
) -> Result<Option<Arc<Pool>>, ForwarderError> {
    endpoint
        .map(|endpoint| build_pool(endpoint, timeout).map(Arc::new))
        .transpose()
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
        let started = Instant::now();
        let stats = &self.inner.stats;
        stats.total.fetch_add(1, Ordering::Relaxed);

        let Ok(info) = request.request_info() else {
            return reply_error(request, &mut response_handle, ResponseCode::FormErr, None).await;
        };
        let query = info.query.original().clone();
        let flags = QueryFlags {
            dnssec_ok: request
                .edns
                .as_ref()
                .is_some_and(|edns| edns.flags().dnssec_ok),
            checking_disabled: request.metadata.checking_disabled,
        };
        let edns = response_edns(request, flags);

        let name = query.name().to_ascii();
        let log = |outcome, code: Option<ResponseCode>| {
            self.inner.log.push(
                &name,
                query.query_type().to_string(),
                outcome,
                code.map(|code| format!("{code:?}")).unwrap_or_default(),
                started.elapsed(),
            );
        };

        if is_doh_canary(&name) {
            log(Outcome::Blocked, Some(ResponseCode::NXDomain));
            return reply_error(
                request,
                &mut response_handle,
                ResponseCode::NXDomain,
                edns.as_ref(),
            )
            .await;
        }

        if let Some(hit) = self.inner.cache.get(&query, flags, Instant::now()) {
            stats.cached.fetch_add(1, Ordering::Relaxed);
            log(Outcome::Cached, Some(hit.response_code));
            let answer = Answer {
                code: hit.response_code,
                authentic_data: hit.authentic_data,
                truncated: false,
                answers: &hit.answers,
                authorities: &hit.authorities,
                additionals: &hit.additionals,
            };
            return reply(request, &mut response_handle, answer, edns.as_ref()).await;
        }

        let (primary, backup, generation) = self.upstream();
        let Some((message, outcome)) = self
            .resolve(&primary, backup.as_deref(), &query, flags)
            .await
        else {
            stats.upstream_failed();
            log(Outcome::Failed, None);
            return reply_error(
                request,
                &mut response_handle,
                ResponseCode::ServFail,
                edns.as_ref(),
            )
            .await;
        };
        stats.upstream_answered();
        if outcome == Outcome::Backup {
            stats.backup.fetch_add(1, Ordering::Relaxed);
        }

        if self.generation() == generation {
            self.inner
                .cache
                .insert(&query, flags, &message, Instant::now());
        }
        log(outcome, Some(message.metadata.response_code));

        let answer = Answer {
            code: message.metadata.response_code,
            authentic_data: message.metadata.authentic_data,
            truncated: message.metadata.truncation,
            answers: &message.answers,
            authorities: &message.authorities,
            additionals: &message.additionals,
        };
        reply(request, &mut response_handle, answer, edns.as_ref()).await
    }
}

/// The OPT record for a response, when the request carried one.
///
/// RFC 6891 requires it: a client that sent EDNS and gets none back assumes
/// the server does not speak it, and drops to 512-byte answers — which pushes
/// anything larger onto a TCP retry it did not need.
fn response_edns(request: &Request, flags: QueryFlags) -> Option<Edns> {
    let asked = request.edns.as_ref()?;
    let mut edns = Edns::new();
    edns.set_max_payload(asked.max_payload().max(512));
    edns.set_dnssec_ok(flags.dnssec_ok);
    Some(edns)
}

/// What goes back to the client, from upstream or from the cache.
struct Answer<'a> {
    code: ResponseCode,
    /// The upstream validated it (DNSSEC). Passed through, not asserted by us.
    authentic_data: bool,
    truncated: bool,
    answers: &'a [Record],
    authorities: &'a [Record],
    additionals: &'a [Record],
}

async fn reply<R: ResponseHandler>(
    request: &Request,
    handle: &mut R,
    answer: Answer<'_>,
    edns: Option<&Edns>,
) -> ResponseInfo {
    // Restore the client's transaction id and flags: the upstream exchange
    // used its own, and returning those would break the client's matching.
    let mut metadata = Metadata::response_from_request(&request.metadata);
    metadata.response_code = answer.code;
    metadata.authoritative = false;
    metadata.recursion_available = true;
    metadata.authentic_data = answer.authentic_data;
    metadata.truncation = answer.truncated;

    let mut builder = MessageResponseBuilder::from_message_request(request);
    if let Some(edns) = edns {
        builder.edns(edns);
    }
    let response = builder.build(
        metadata,
        answer.answers.iter(),
        answer.authorities.iter(),
        [],
        answer.additionals.iter(),
    );

    match handle.send_response(response).await {
        Ok(info) => info,
        Err(err) => {
            tracing::debug!("could not send a response: {err}");
            fallback_info(request)
        }
    }
}

async fn reply_error<R: ResponseHandler>(
    request: &Request,
    handle: &mut R,
    code: ResponseCode,
    edns: Option<&Edns>,
) -> ResponseInfo {
    let mut builder = MessageResponseBuilder::from_message_request(request);
    if let Some(edns) = edns {
        builder.edns(edns);
    }
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

/// A running resolver: the listeners on loopback port 53, and the forwarder
/// behind them.
pub struct LocalResolver {
    server: Server<Forwarder>,
    forwarder: Forwarder,
    listening: Vec<SocketAddr>,
}

impl LocalResolver {
    /// Bind loopback port 53 and start answering.
    ///
    /// Must be called from inside a Tokio runtime: the listeners are spawned
    /// onto it. IPv4 is required; IPv6 is taken when the machine has it, and
    /// its absence is not an error — a machine with IPv6 disabled has no IPv6
    /// DNS for the OS to resolve around us with.
    pub async fn start(
        primary: &Endpoint,
        backup: Option<&Endpoint>,
    ) -> Result<Self, ForwarderError> {
        // Built first: a bad endpoint should fail before port 53 is touched.
        let forwarder = Forwarder::new(primary, backup, UPSTREAM_TIMEOUT)?;

        // No "is anyone on port 53" check first: a holder on the wildcard
        // address does not necessarily block a loopback bind. Internet
        // Connection Sharing sits on `0.0.0.0:53` and a `127.0.0.1:53` bind
        // beside it succeeds and receives loopback traffic — refusing on sight
        // of it would turn away a machine that works. The bind is the test,
        // and only its failure looks the holder up to name it.
        let v4 = bind_pair(LISTEN_V4).await?;
        let v6 = match bind_pair(LISTEN_V6).await {
            Ok(pair) => Some(pair),
            Err(err) => {
                tracing::info!("IPv6 loopback unavailable, listening on IPv4 only: {err}");
                None
            }
        };

        Ok(Self::serve(forwarder, v4, v6))
    }

    /// Start on sockets bound by someone else.
    ///
    /// On Linux, binding port 53 needs root, which the interface process does
    /// not have; the privileged agent binds them and hands them over (see
    /// `forceconnect_privileged::agent`). Their addresses are checked rather
    /// than trusted, so a socket on anything but loopback port 53 is refused
    /// instead of quietly exposing the resolver.
    pub fn start_with(
        primary: &Endpoint,
        backup: Option<&Endpoint>,
        sockets: BoundSockets,
    ) -> Result<Self, ForwarderError> {
        let forwarder = Forwarder::new(primary, backup, UPSTREAM_TIMEOUT)?;

        let v4 = adopt(LISTEN_V4, sockets.udp_v4, sockets.tcp_v4)?;
        let v6 = sockets
            .v6
            .map(|(udp, tcp)| adopt(LISTEN_V6, udp, tcp))
            .transpose()?;

        Ok(Self::serve(forwarder, v4, v6))
    }

    fn serve(
        forwarder: Forwarder,
        v4: (UdpSocket, TcpListener),
        v6: Option<(UdpSocket, TcpListener)>,
    ) -> Self {
        let mut server = Server::new(forwarder.clone());
        let mut listening = Vec::with_capacity(2);

        for (addr, (udp, tcp)) in [(LISTEN_V4, Some(v4)), (LISTEN_V6, v6)]
            .into_iter()
            .filter_map(|(addr, pair)| pair.map(|pair| (addr, pair)))
        {
            server.register_socket(udp);
            server.register_listener(tcp, TCP_IDLE, TCP_RESPONSE_BUFFER);
            listening.push(addr);
        }

        tracing::info!(?listening, "local resolver started");
        Self {
            server,
            forwarder,
            listening,
        }
    }

    /// The addresses the OS should be pointed at.
    pub fn listening(&self) -> &[SocketAddr] {
        &self.listening
    }

    pub fn forwarder(&self) -> &Forwarder {
        &self.forwarder
    }

    /// Close the listeners and wait for in-flight queries to finish.
    ///
    /// Bounded: a query stuck on a dead upstream should not hold up a
    /// disconnect. Anything still running is dropped with the server.
    pub async fn stop(mut self) {
        if tokio::time::timeout(Duration::from_secs(3), self.server.shutdown_gracefully())
            .await
            .is_err()
        {
            tracing::warn!("local resolver did not shut down in time; dropping it");
        }
        tracing::info!("local resolver stopped");
    }
}

/// Listening sockets bound elsewhere, for [`LocalResolver::start_with`].
pub struct BoundSockets {
    pub udp_v4: std::net::UdpSocket,
    pub tcp_v4: std::net::TcpListener,
    pub v6: Option<(std::net::UdpSocket, std::net::TcpListener)>,
}

fn adopt(
    expected: SocketAddr,
    udp: std::net::UdpSocket,
    tcp: std::net::TcpListener,
) -> Result<(UdpSocket, TcpListener), ForwarderError> {
    let wrong = |found: std::io::Result<SocketAddr>| ForwarderError::Bind {
        addr: expected,
        source: std::io::Error::other(format!(
            "a handed-over socket is bound to {found:?}, not {expected}"
        )),
    };

    let udp_addr = udp.local_addr();
    if udp_addr.as_ref().ok() != Some(&expected) {
        return Err(wrong(udp_addr));
    }
    let tcp_addr = tcp.local_addr();
    if tcp_addr.as_ref().ok() != Some(&expected) {
        return Err(wrong(tcp_addr));
    }

    let io = |source| ForwarderError::Bind {
        addr: expected,
        source,
    };
    udp.set_nonblocking(true).map_err(io)?;
    tcp.set_nonblocking(true).map_err(io)?;
    Ok((
        UdpSocket::from_std(udp).map_err(io)?,
        TcpListener::from_std(tcp).map_err(io)?,
    ))
}

async fn bind_pair(addr: SocketAddr) -> Result<(UdpSocket, TcpListener), ForwarderError> {
    Ok((bind_udp(addr).await?, bind_tcp(addr).await?))
}

async fn bind_udp(addr: SocketAddr) -> Result<UdpSocket, ForwarderError> {
    UdpSocket::bind(addr)
        .await
        .map_err(|err| bind_error(addr, err))
}

async fn bind_tcp(addr: SocketAddr) -> Result<TcpListener, ForwarderError> {
    TcpListener::bind(addr)
        .await
        .map_err(|err| bind_error(addr, err))
}

/// Turn a bind failure into something with a remedy.
fn bind_error(addr: SocketAddr, source: std::io::Error) -> ForwarderError {
    use std::io::ErrorKind;

    let holder =
        || forceconnect_net::ports::udp_port_owner(addr.port()).map(|owner| describe_owner(&owner));

    match source.kind() {
        ErrorKind::AddrInUse => ForwarderError::PortTaken {
            holder: holder().unwrap_or_else(|| "another program".into()),
        },
        // Windows reports an exclusive or reserved port as WSAEACCES, which
        // surfaces as "permission denied" even when running elevated: Hyper-V,
        // WSL and Docker reserve port ranges, and some services bind with
        // SO_EXCLUSIVEADDRUSE.
        ErrorKind::PermissionDenied if cfg!(windows) => ForwarderError::Reserved {
            addr,
            holder: holder(),
        },
        ErrorKind::PermissionDenied => ForwarderError::NeedsPrivilege,
        _ => ForwarderError::Bind { addr, source },
    }
}

fn describe_owner(owner: &forceconnect_net::ports::PortOwner) -> String {
    if owner.is_ics() {
        // The one holder common enough to name its remedy outright.
        format!(
            "Internet Connection Sharing ({}) — stop the SharedAccess service or turn off sharing to use encrypted DNS",
            owner.describe()
        )
    } else {
        owner.describe()
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
        for addr in [LISTEN_V4, LISTEN_V6] {
            assert!(addr.ip().is_loopback());
            assert_eq!(addr.port(), 53);
        }
    }

    #[test]
    fn the_firefox_canary_is_recognised() {
        assert!(is_doh_canary("use-application-dns.net"));
        assert!(is_doh_canary("use-application-dns.net."));
        assert!(is_doh_canary("USE-APPLICATION-DNS.NET"));
        assert!(!is_doh_canary("example.com"));
    }

    #[test]
    fn stats_count_totals_failures_and_hits_separately() {
        let stats = Stats::default();
        assert_eq!(stats.snapshot().total, 0);
        stats.total.fetch_add(3, Ordering::Relaxed);
        stats.upstream_failed();
        stats.cached.fetch_add(2, Ordering::Relaxed);
        assert_eq!(
            stats.snapshot(),
            StatsSnapshot {
                total: 3,
                failed: 1,
                cached: 2,
                backup: 0,
                failing_streak: 1,
            }
        );
    }

    #[test]
    fn an_answer_ends_a_failing_streak_but_not_the_failure_count() {
        let stats = Stats::default();
        for _ in 0..4 {
            stats.upstream_failed();
        }
        assert_eq!(stats.snapshot().failing_streak, 4);

        stats.upstream_answered();
        let snapshot = stats.snapshot();
        assert_eq!(snapshot.failing_streak, 0);
        assert_eq!(snapshot.failed, 4, "the total is history and stays");
    }

    #[test]
    fn a_switch_empties_the_cache_and_moves_the_generation() {
        let endpoint = |addr: &str| Endpoint {
            protocol: crate::Protocol::Udp,
            addrs: vec![addr.into()],
            bootstrap: vec![],
        };

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            let forwarder = Forwarder::new(&endpoint("192.0.2.1"), None, UPSTREAM_TIMEOUT).unwrap();
            assert_eq!(forwarder.generation(), 0);

            forwarder
                .switch(&endpoint("192.0.2.2"), Some(&endpoint("192.0.2.3")))
                .unwrap();
            assert_eq!(forwarder.generation(), 1);
            assert_eq!(forwarder.cache_len(), 0);
            assert!(forwarder.has_backup());
        });
    }

    #[test]
    fn a_switch_to_a_loopback_upstream_is_refused() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            let good = Endpoint {
                protocol: crate::Protocol::Udp,
                addrs: vec!["192.0.2.1".into()],
                bootstrap: vec![],
            };
            let forwarder = Forwarder::new(&good, None, UPSTREAM_TIMEOUT).unwrap();

            let looped = Endpoint {
                addrs: vec!["127.0.0.1".into()],
                ..good.clone()
            };
            assert!(forwarder.switch(&looped, None).is_err());
            assert!(
                forwarder.switch(&good, Some(&looped)).is_err(),
                "a looped backup is refused like a looped primary"
            );
            assert_eq!(
                forwarder.generation(),
                0,
                "a refused switch changes nothing"
            );
        });
    }

    #[test]
    fn a_dead_primary_is_passed_over_only_when_there_is_a_backup() {
        let endpoint = |addr: &str| Endpoint {
            protocol: crate::Protocol::Udp,
            addrs: vec![addr.into()],
            bootstrap: vec![],
        };
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            let alone = Forwarder::new(&endpoint("192.0.2.1"), None, UPSTREAM_TIMEOUT).unwrap();
            for _ in 0..PRIMARY_BYPASS_AFTER {
                alone.primary_failed(false);
            }
            // With nowhere else to go, the primary keeps being asked.
            assert!(!alone.using_backup());

            let paired = Forwarder::new(
                &endpoint("192.0.2.1"),
                Some(&endpoint("192.0.2.2")),
                UPSTREAM_TIMEOUT,
            )
            .unwrap();
            for _ in 1..PRIMARY_BYPASS_AFTER {
                paired.primary_failed(true);
            }
            assert!(!paired.using_backup(), "not before the threshold");
            paired.primary_failed(true);
            assert!(paired.using_backup());

            paired.primary_answered();
            assert!(!paired.using_backup(), "an answer restores the primary");
        });
    }

    #[test]
    fn handed_over_sockets_must_be_on_loopback_port_53() {
        // Bound to an ephemeral port: exactly the kind of socket `start_with`
        // must refuse rather than serve on.
        let udp = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
        let tcp = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let _guard = runtime.enter();

        assert!(matches!(
            adopt(LISTEN_V4, udp, tcp),
            Err(ForwarderError::Bind { .. })
        ));
    }

    // Needs port 53 free and the right to bind it — true in an elevated
    // Windows shell and on macOS, not in most CI.
    #[test]
    #[ignore = "binds port 53 and reaches the public internet"]
    fn the_resolver_answers_from_loopback() {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            let endpoint = |protocol, addr: &str, bootstrap: &str| Endpoint {
                protocol,
                addrs: vec![addr.into()],
                bootstrap: vec![bootstrap.parse().unwrap()],
            };
            let upstreams = [
                endpoint(
                    crate::Protocol::Doh,
                    "https://cloudflare-dns.com/dns-query",
                    "1.1.1.1",
                ),
                endpoint(crate::Protocol::Dot, "one.one.one.one", "1.1.1.1"),
                endpoint(crate::Protocol::Doq, "dns.adguard-dns.com", "94.140.14.14"),
                // Last: networks that filter HTTP/3 are common, and a failure
                // here should not hide whether the others work.
                endpoint(
                    crate::Protocol::Doh3,
                    "https://dns.adguard-dns.com/dns-query",
                    "94.140.14.14",
                ),
            ];

            let resolver = LocalResolver::start(&upstreams[0], None)
                .await
                .expect("start");

            // A repeat is answered from the cache, not upstream.
            ask("iana.org.").await;
            let before = resolver.forwarder().stats().snapshot().cached;
            ask("iana.org.").await;
            assert_eq!(resolver.forwarder().stats().snapshot().cached, before + 1);

            // A client that asks for DNSSEC gets signatures and an OPT record
            // back; one that does not, gets neither.
            let signed = ask_with("cloudflare.com.", true).await;
            assert!(signed.edns.as_ref().is_some_and(|e| e.flags().dnssec_ok));
            assert!(
                signed
                    .answers
                    .iter()
                    .any(|r| r.record_type() == hickory_proto::rr::RecordType::RRSIG),
                "no RRSIG in a DO answer"
            );
            let plain = ask("cloudflare.com.").await;
            assert!(plain.edns.is_none());
            assert!(
                !plain
                    .answers
                    .iter()
                    .any(|r| r.record_type() == hickory_proto::rr::RecordType::RRSIG)
            );

            // A primary that never answers: TEST-NET-1 is unroutable by
            // definition. The backup carries every lookup, and after a few the
            // primary is skipped so lookups stop paying its timeout.
            let dead = Endpoint {
                protocol: crate::Protocol::Udp,
                addrs: vec!["192.0.2.1".into()],
                bootstrap: vec![],
            };
            resolver
                .forwarder()
                .switch(&dead, Some(&upstreams[0]))
                .unwrap();
            for name in ["one.example.", "two.example.", "three.example."] {
                ask(name).await;
            }
            assert!(resolver.forwarder().using_backup());
            let started = Instant::now();
            let reply = ask("wikipedia.org.").await;
            assert!(!reply.answers.is_empty());
            assert!(
                started.elapsed() < Duration::from_secs(2),
                "the bypassed primary still cost {:?}",
                started.elapsed()
            );
            assert_eq!(resolver.forwarder().stats().snapshot().backup, 4);

            let names = ["example.com.", "example.org.", "example.net.", "iana.org."];
            for (upstream, name) in upstreams.iter().zip(names) {
                // Switched in place, as a server change while connected is:
                // the listener stays up throughout.
                resolver.forwarder().switch(upstream, None).unwrap();
                let reply = ask(name).await;
                assert!(
                    !reply.answers.is_empty(),
                    "{:?} gave no answer for {name}",
                    upstream.protocol
                );
            }

            resolver.stop().await;
        });

        // The self-referential guard refuses loopback upstreams, so the
        // listener is queried with a bare socket rather than a pool.
        async fn ask(name: &str) -> hickory_proto::op::Message {
            ask_with(name, false).await
        }

        async fn ask_with(name: &str, dnssec_ok: bool) -> hickory_proto::op::Message {
            let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
            let mut query = hickory_proto::op::Message::query();
            query.metadata.recursion_desired = true;
            if dnssec_ok {
                let mut edns = Edns::new();
                edns.set_max_payload(1232);
                edns.set_dnssec_ok(true);
                query.edns = Some(edns);
            }
            query.add_query(hickory_proto::op::Query::query(
                name.parse().unwrap(),
                hickory_proto::rr::RecordType::A,
            ));
            socket
                .send_to(&query.to_vec().unwrap(), LISTEN_V4)
                .await
                .unwrap();

            let mut buf = [0u8; 4096];
            let len = tokio::time::timeout(Duration::from_secs(6), socket.recv(&mut buf))
                .await
                .expect("reply in time")
                .unwrap();
            let reply = hickory_proto::op::Message::from_vec(&buf[..len]).unwrap();
            assert_eq!(reply.metadata.id, query.metadata.id);
            reply
        }
    }
}
