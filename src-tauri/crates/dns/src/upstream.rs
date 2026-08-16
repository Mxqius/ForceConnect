//! Building hickory upstream pools from catalog endpoints.

use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;

use hickory_resolver::config::{ConnectionConfig, NameServerConfig, ResolverOpts};
use hickory_resolver::net::runtime::TokioRuntimeProvider;
use hickory_resolver::{NameServerPool, PoolContext, TlsConfig};

use crate::server::{Endpoint, Protocol};

#[derive(Debug, thiserror::Error)]
pub enum UpstreamError {
    #[error("endpoint has no addresses")]
    NoAddresses,

    #[error("'{0}' is not a usable address for this protocol")]
    BadAddress(String),

    #[error("'{0}' resolves to a loopback or unspecified address")]
    SelfReferential(String),

    #[error("tls setup failed: {0}")]
    Tls(String),
}

/// Whether an address would point the resolver back at us.
///
/// A guard rather than a nicety: once the OS points at 127.0.0.1, an upstream
/// that also resolves to loopback turns every query into an infinite loop
/// through our own forwarder. It also closes a DNS-rebinding path back into it.
pub fn is_self_referential(ip: &IpAddr) -> bool {
    ip.is_loopback() || ip.is_unspecified()
}

/// Resolve an endpoint to the concrete IPs hickory should dial.
///
/// For plain protocols the addresses *are* IPs. For encrypted ones the address
/// carries a hostname used for TLS validation, and the IPs come from the
/// endpoint's bootstrap list — which is why the catalog requires one.
pub fn dial_addresses(endpoint: &Endpoint) -> Result<Vec<IpAddr>, UpstreamError> {
    if endpoint.addrs.is_empty() {
        return Err(UpstreamError::NoAddresses);
    }

    let addresses = match endpoint.protocol {
        Protocol::Udp | Protocol::Tcp => endpoint
            .addrs
            .iter()
            .map(|addr| {
                let host = addr.rsplit_once(':').map_or(addr.as_str(), |(h, _)| h);
                host.parse::<IpAddr>()
                    .map_err(|_| UpstreamError::BadAddress(addr.clone()))
            })
            .collect::<Result<Vec<_>, _>>()?,

        // Hostname endpoints never trigger a lookup: hickory dials the IP and
        // uses the hostname only for SNI and certificate validation, so the
        // bootstrap list is the whole story.
        _ => {
            if endpoint.bootstrap.is_empty() {
                // Unless the endpoint is addressed by IP literal already.
                literal_host(endpoint)
                    .map(|ip| vec![ip])
                    .ok_or(UpstreamError::NoAddresses)?
            } else {
                endpoint.bootstrap.clone()
            }
        }
    };

    if let Some(bad) = addresses.iter().find(|ip| is_self_referential(ip)) {
        return Err(UpstreamError::SelfReferential(bad.to_string()));
    }

    Ok(addresses)
}

fn literal_host(endpoint: &Endpoint) -> Option<IpAddr> {
    let addr = endpoint.addrs.first()?;
    let host = host_of(addr)?;
    host.trim_matches(['[', ']']).parse().ok()
}

/// Extract the host portion, whether the address is a URL or a bare hostname.
pub fn host_of(addr: &str) -> Option<&str> {
    let rest = addr.strip_prefix("https://").unwrap_or(addr);
    let host = rest.split('/').next()?;
    let host = match host.rsplit_once(':') {
        // Only strip a trailing `:port`, not part of an IPv6 literal.
        Some((h, port)) if port.chars().all(|c| c.is_ascii_digit()) && !h.ends_with(']') => h,
        _ => host,
    };

    (!host.is_empty()).then_some(host)
}

/// Path component of a DoH/DoH3 URL.
pub fn path_of(addr: &str) -> String {
    addr.strip_prefix("https://")
        .and_then(|rest| rest.find('/').map(|i| rest[i..].to_owned()))
        .unwrap_or_else(|| "/dns-query".to_owned())
}

/// Explicit port, if the address carries one.
pub fn port_of(addr: &str) -> Option<u16> {
    let rest = addr.strip_prefix("https://").unwrap_or(addr);
    let host = rest.split('/').next()?;
    let (h, port) = host.rsplit_once(':')?;
    if h.ends_with(']') || !port.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    port.parse().ok()
}

/// Build a connection config for one protocol against one IP.
fn connection(endpoint: &Endpoint) -> Result<ConnectionConfig, UpstreamError> {
    let addr = endpoint
        .addrs
        .first()
        .ok_or(UpstreamError::NoAddresses)?
        .clone();

    let server_name: Arc<str> = Arc::from(
        host_of(&addr)
            .ok_or_else(|| UpstreamError::BadAddress(addr.clone()))?
            .to_owned(),
    );

    let mut config = match endpoint.protocol {
        Protocol::Udp => ConnectionConfig::udp(),
        Protocol::Tcp => ConnectionConfig::tcp(),
        Protocol::Dot => ConnectionConfig::tls(server_name),
        Protocol::Doh => ConnectionConfig::https(server_name, Some(Arc::from(path_of(&addr)))),
        Protocol::Doq => ConnectionConfig::quic(server_name),
        Protocol::Doh3 => ConnectionConfig::h3(server_name, Some(Arc::from(path_of(&addr)))),
    };

    if let Some(port) = port_of(&addr) {
        config.port = port;
    }

    Ok(config)
}

/// Options shared by the prober and the forwarder.
pub fn resolver_opts(timeout: Duration, attempts: usize) -> ResolverOpts {
    let mut opts = ResolverOpts::default();
    opts.timeout = timeout;
    opts.attempts = attempts;
    // Caching happens at the message level in the forwarder, and must not
    // happen at all in the prober — a cached answer would measure nothing.
    opts.cache_size = 0;
    opts.try_tcp_on_error = true;
    // 1232 avoids IPv6 fragmentation blackholes, which present as intermittent
    // timeouts that are miserable to diagnose.
    opts.edns0 = true;
    opts
}

/// Build a pool for one endpoint.
pub fn pool(
    endpoint: &Endpoint,
    opts: ResolverOpts,
    provider: TokioRuntimeProvider,
) -> Result<NameServerPool<TokioRuntimeProvider>, UpstreamError> {
    let addresses = dial_addresses(endpoint)?;
    let connection = connection(endpoint)?;

    let tls = TlsConfig::new().map_err(|err| UpstreamError::Tls(err.to_string()))?;
    let context = Arc::new(PoolContext::new(opts, tls));

    let servers: Vec<NameServerConfig> = addresses
        .into_iter()
        .map(|ip| NameServerConfig::new(ip, true, vec![connection.clone()]))
        .collect();

    Ok(NameServerPool::from_config(servers, context, provider))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn endpoint(protocol: Protocol, addr: &str, bootstrap: &[&str]) -> Endpoint {
        Endpoint {
            protocol,
            addrs: vec![addr.to_owned()],
            bootstrap: bootstrap.iter().map(|s| s.parse().unwrap()).collect(),
        }
    }

    #[test]
    fn plain_endpoints_dial_their_own_addresses() {
        let e = endpoint(Protocol::Udp, "1.1.1.1", &[]);
        assert_eq!(
            dial_addresses(&e).unwrap(),
            vec!["1.1.1.1".parse::<IpAddr>().unwrap()]
        );
    }

    #[test]
    fn encrypted_endpoints_dial_their_bootstrap() {
        // The hostname is for certificate validation only; hickory connects to
        // the IP, so no lookup happens and no loop is possible.
        let e = endpoint(
            Protocol::Doh,
            "https://cloudflare-dns.com/dns-query",
            &["1.1.1.1", "1.0.0.1"],
        );
        assert_eq!(dial_addresses(&e).unwrap().len(), 2);
    }

    #[test]
    fn an_ip_addressed_endpoint_needs_no_bootstrap() {
        let e = endpoint(Protocol::Doh, "https://9.9.9.9/dns-query", &[]);
        assert_eq!(
            dial_addresses(&e).unwrap(),
            vec!["9.9.9.9".parse::<IpAddr>().unwrap()]
        );
    }

    #[test]
    fn loopback_upstreams_are_refused() {
        // Pointing an upstream at ourselves would make every query recurse
        // through the forwarder forever.
        let e = endpoint(Protocol::Udp, "127.0.0.1", &[]);
        assert!(matches!(
            dial_addresses(&e),
            Err(UpstreamError::SelfReferential(_))
        ));

        let bootstrapped = endpoint(
            Protocol::Doh,
            "https://evil.example/dns-query",
            &["127.0.0.1"],
        );
        assert!(matches!(
            dial_addresses(&bootstrapped),
            Err(UpstreamError::SelfReferential(_))
        ));
    }

    #[test]
    fn host_path_and_port_are_extracted() {
        assert_eq!(host_of("https://dns.google/dns-query"), Some("dns.google"));
        assert_eq!(path_of("https://dns.google/dns-query"), "/dns-query");
        assert_eq!(
            host_of("dns.adguard-dns.com:853"),
            Some("dns.adguard-dns.com")
        );
        assert_eq!(port_of("dns.adguard-dns.com:853"), Some(853));
        assert_eq!(port_of("dns.google"), None);
        assert_eq!(
            host_of("https://corp.example:8443/dns-query"),
            Some("corp.example")
        );
        assert_eq!(port_of("https://corp.example:8443/dns-query"), Some(8443));
    }

    #[test]
    fn an_ipv6_literal_is_not_mistaken_for_a_port() {
        assert_eq!(host_of("[2606:4700::1111]"), Some("[2606:4700::1111]"));
        assert_eq!(port_of("[2606:4700::1111]"), None);
    }
}
