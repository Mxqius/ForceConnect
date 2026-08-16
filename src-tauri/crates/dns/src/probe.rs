//! Measuring how fast a server actually answers.
//!
//! Not ICMP. Pinging a resolver measures almost nothing relevant: it skips the
//! TLS or QUIC handshake and the HTTP framing entirely, many resolvers
//! deprioritise or rate-limit ICMP, and anycast means the host answering the
//! echo may not be the one that serves DNS. This issues a real query over the
//! server's real protocol.

use std::time::{Duration, Instant};

use futures_util::StreamExt;
use hickory_proto::op::{DnsRequest, DnsRequestOptions, Query};
use hickory_proto::rr::{Name, RecordType};
// `xfer` moved out of hickory-proto into hickory-net in 0.26; reached here
// through the resolver's re-export so this crate needs no direct dependency.
use hickory_resolver::net::DnsHandle;
use hickory_resolver::net::runtime::TokioRuntimeProvider;

use crate::server::{Endpoint, Protocol};
use crate::upstream;

/// What a probe concluded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    /// Every round answered.
    Good,
    /// Some rounds answered. Usually packet loss or aggressive rate limiting.
    Degraded,
    /// Nothing answered.
    Unreachable,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct ProbeResult {
    pub protocol: Protocol,
    pub verdict: Verdict,
    /// Median round-trip in milliseconds, over the rounds that answered.
    ///
    /// Median rather than mean on purpose: a single timeout in a four-sample
    /// mean turns a 20ms resolver into a 515ms one, which is a lie that would
    /// send users to a worse server.
    pub median_ms: Option<f64>,
    pub min_ms: Option<f64>,
    /// Mean absolute difference between consecutive samples.
    pub jitter_ms: Option<f64>,
    /// Cost of the first exchange minus the steady-state median.
    ///
    /// Reported separately because it is paid once per connection, not per
    /// query. It is why DoH looks slow on a cold pick and fine thereafter, and
    /// hiding it would make encrypted protocols look worse than they are.
    pub handshake_ms: Option<f64>,
    pub answered: u8,
    pub rounds: u8,
    pub error: Option<String>,
}

impl ProbeResult {
    pub fn unreachable(protocol: Protocol, rounds: u8, error: String) -> Self {
        Self {
            protocol,
            verdict: Verdict::Unreachable,
            median_ms: None,
            min_ms: None,
            jitter_ms: None,
            handshake_ms: None,
            answered: 0,
            rounds,
            error: Some(error),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProbeSpec {
    pub rounds: u8,
    pub timeout: Duration,
    /// Discard a first exchange so the handshake is not counted as latency.
    pub warmup: bool,
}

impl Default for ProbeSpec {
    fn default() -> Self {
        Self {
            rounds: 4,
            timeout: Duration::from_secs(2),
            warmup: true,
        }
    }
}

/// Summarise raw samples into a result.
///
/// Split out from the I/O so the statistics are testable without a network.
pub fn summarize(
    protocol: Protocol,
    samples: &[Option<Duration>],
    warmup: Option<Duration>,
) -> ProbeResult {
    let rounds = samples.len() as u8;
    let mut answered: Vec<f64> = samples
        .iter()
        .filter_map(|s| s.map(|d| d.as_secs_f64() * 1000.0))
        .collect();

    if answered.is_empty() {
        return ProbeResult::unreachable(protocol, rounds, "no response".into());
    }

    let count = answered.len() as u8;
    let jitter = mean_absolute_difference(&answered);

    answered.sort_by(|a, b| a.partial_cmp(b).expect("no NaN in measured durations"));
    let median = percentile(&answered, 0.5);
    let min = answered.first().copied();

    let handshake = warmup
        .map(|d| d.as_secs_f64() * 1000.0)
        .and_then(|w| median.map(|m| (w - m).max(0.0)));

    ProbeResult {
        protocol,
        verdict: if count == rounds {
            Verdict::Good
        } else {
            Verdict::Degraded
        },
        median_ms: median,
        min_ms: min,
        jitter_ms: jitter,
        handshake_ms: handshake,
        answered: count,
        rounds,
        error: None,
    }
}

fn percentile(sorted: &[f64], p: f64) -> Option<f64> {
    if sorted.is_empty() {
        return None;
    }
    let index = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted.get(index).copied()
}

fn mean_absolute_difference(values: &[f64]) -> Option<f64> {
    if values.len() < 2 {
        return None;
    }
    let total: f64 = values.windows(2).map(|w| (w[1] - w[0]).abs()).sum();
    Some(total / (values.len() - 1) as f64)
}

/// A cache-busting name, so nothing on the path can answer from a cache and
/// make a slow resolver look instant.
pub fn probe_name(seed: u64) -> String {
    format!("fc{seed:016x}.example.com.")
}

/// Whether an endpoint is worth probing at all.
pub fn is_probeable(endpoint: &Endpoint) -> bool {
    !endpoint.addrs.is_empty()
}

/// Probe one endpoint.
///
/// Builds a throwaway pool rather than borrowing a live one: probing through
/// the production pool would warm its connections and let its cache answer,
/// hiding the very latency being measured.
///
/// Rounds are sequential. Firing them concurrently would measure HTTP/2 or QUIC
/// multiplexing rather than round-trip time.
pub async fn probe_endpoint(endpoint: &Endpoint, spec: ProbeSpec) -> ProbeResult {
    let opts = upstream::resolver_opts(spec.timeout, 1);
    let provider = TokioRuntimeProvider::new();

    let pool = match upstream::pool(endpoint, opts, provider) {
        Ok(pool) => pool,
        Err(err) => {
            return ProbeResult::unreachable(endpoint.protocol, spec.rounds, err.to_string());
        }
    };

    let mut seed = seed_from_clock();

    let warmup = if spec.warmup {
        seed = seed.wrapping_add(1);
        exchange(&pool, seed, spec.timeout).await
    } else {
        None
    };

    let mut samples = Vec::with_capacity(spec.rounds as usize);
    for _ in 0..spec.rounds {
        seed = seed.wrapping_add(1);
        samples.push(exchange(&pool, seed, spec.timeout).await);
    }

    summarize(endpoint.protocol, &samples, warmup)
}

/// One query, timed. `None` means no usable answer arrived in time.
async fn exchange(
    pool: &hickory_resolver::NameServerPool<TokioRuntimeProvider>,
    seed: u64,
    timeout: Duration,
) -> Option<Duration> {
    let name = Name::from_ascii(probe_name(seed)).ok()?;
    let query = Query::query(name, RecordType::A);
    let request = DnsRequest::from_query(query, DnsRequestOptions::default());
    let started = Instant::now();

    // NXDOMAIN and REFUSED count as answers: the server responded, which is
    // exactly what is being timed. Only silence and transport failures do not.
    let mut stream = pool.send(request);
    match tokio::time::timeout(timeout, stream.next()).await {
        Ok(Some(Ok(_))) => Some(started.elapsed()),
        Ok(Some(Err(err))) if is_negative_answer(&err) => Some(started.elapsed()),
        _ => None,
    }
}

/// Hickory surfaces NXDOMAIN as an error; for timing purposes it is a success.
fn is_negative_answer(err: &hickory_resolver::net::NetError) -> bool {
    let text = err.to_string();
    text.contains("no records found") || text.contains("NXDomain") || text.contains("Refused")
}

fn seed_from_clock() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x5eed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms(value: f64) -> Option<Duration> {
        Some(Duration::from_secs_f64(value / 1000.0))
    }

    #[test]
    fn a_single_timeout_does_not_wreck_the_figure() {
        // The reason for median over mean: the mean here is ~515ms, which
        // would rank a fast resolver below a mediocre one.
        let samples = [ms(20.0), ms(21.0), ms(19.0), None];
        let result = summarize(Protocol::Udp, &samples, None);

        assert_eq!(result.verdict, Verdict::Degraded);
        assert_eq!(result.answered, 3);
        assert_eq!(result.rounds, 4);
        assert!(result.median_ms.unwrap() < 25.0);
    }

    #[test]
    fn all_answered_is_good() {
        let samples = [ms(10.0), ms(12.0), ms(11.0), ms(13.0)];
        let result = summarize(Protocol::Doh, &samples, None);
        assert_eq!(result.verdict, Verdict::Good);
        assert_eq!(result.answered, 4);
        assert_eq!(result.min_ms, Some(10.0));
    }

    #[test]
    fn nothing_answered_is_unreachable_not_slow() {
        // Distinct from a high latency: an unreachable server needs a different
        // response from the user than a slow one.
        let result = summarize(Protocol::Doq, &[None, None], None);
        assert_eq!(result.verdict, Verdict::Unreachable);
        assert_eq!(result.median_ms, None);
        assert!(result.error.is_some());
    }

    #[test]
    fn handshake_is_reported_apart_from_steady_state() {
        // A cold DoH exchange pays the TLS handshake; later ones do not.
        let samples = [ms(20.0), ms(20.0), ms(20.0)];
        let result = summarize(Protocol::Doh, &samples, Some(Duration::from_millis(120)));

        assert_eq!(result.median_ms, Some(20.0));
        assert_eq!(result.handshake_ms, Some(100.0));
    }

    #[test]
    fn a_faster_warmup_than_median_does_not_go_negative() {
        let samples = [ms(50.0), ms(50.0)];
        let result = summarize(Protocol::Udp, &samples, Some(Duration::from_millis(10)));
        assert_eq!(result.handshake_ms, Some(0.0));
    }

    #[test]
    fn jitter_is_the_mean_step_between_samples() {
        let samples = [ms(10.0), ms(20.0), ms(15.0)];
        let result = summarize(Protocol::Udp, &samples, None);
        // |20-10| + |15-20| = 15, over 2 gaps.
        assert_eq!(result.jitter_ms, Some(7.5));
    }

    #[test]
    fn probe_names_are_unique_per_seed() {
        assert_ne!(probe_name(1), probe_name(2));
        assert!(probe_name(0xdead).ends_with(".example.com."));
    }

    /// Talks to real resolvers, so it is opt-in:
    /// `cargo test -p forceconnect-dns -- --ignored --nocapture`
    #[tokio::test]
    #[ignore = "reaches the public internet"]
    async fn probe_every_protocol_against_live_servers() {
        let catalog = crate::Catalog::builtin().unwrap();
        let spec = ProbeSpec::default();

        // One representative per protocol, so a regression in any transport
        // shows up rather than hiding behind plain UDP working.
        for (id, protocol) in [
            ("cloudflare", Protocol::Udp),
            ("cloudflare", Protocol::Dot),
            ("cloudflare", Protocol::Doh),
            ("cloudflare", Protocol::Doh3),
            ("adguard", Protocol::Doq),
        ] {
            let server = catalog.get(id).expect("server exists");
            let endpoint = server.endpoint(protocol).expect("endpoint exists");

            let result = probe_endpoint(endpoint, spec).await;
            println!(
                "{id:<12} {protocol:<6?} {:?} median={:?} handshake={:?} {}/{}",
                result.verdict,
                result.median_ms.map(|v| v.round()),
                result.handshake_ms.map(|v| v.round()),
                result.answered,
                result.rounds,
            );
        }
    }
}
