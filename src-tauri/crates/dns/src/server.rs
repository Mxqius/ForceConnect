//! The server record every other part of the app hangs off.

use std::net::{IpAddr, SocketAddr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    /// Classic unencrypted DNS over UDP/53, falling back to TCP on truncation.
    Udp,
    /// Unencrypted DNS forced over TCP/53.
    Tcp,
    /// DNS over TLS, RFC 7858.
    Dot,
    /// DNS over HTTPS, RFC 8484.
    Doh,
    /// DNS over QUIC, RFC 9250.
    Doq,
    /// DNS over HTTP/3.
    Doh3,
}

impl Protocol {
    /// Whether traffic to this endpoint is encrypted in transit.
    pub const fn is_encrypted(self) -> bool {
        matches!(self, Self::Dot | Self::Doh | Self::Doq | Self::Doh3)
    }

    /// Encrypted protocols cannot be handed to Windows directly — the OS only
    /// accepts plain IP addresses — so they require the local forwarder.
    pub const fn requires_forwarder(self) -> bool {
        self.is_encrypted()
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Udp => "DNS",
            Self::Tcp => "DNS/TCP",
            Self::Dot => "DoT",
            Self::Doh => "DoH",
            Self::Doq => "DoQ",
            Self::Doh3 => "DoH3",
        }
    }

    pub const fn default_port(self) -> u16 {
        match self {
            Self::Udp | Self::Tcp => 53,
            Self::Dot => 853,
            Self::Doh | Self::Doh3 => 443,
            Self::Doq => 853,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "lowercase")]
pub enum Source {
    Builtin,
    Custom,
}

/// One way to reach a provider. A provider variant usually offers several.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Endpoint {
    pub protocol: Protocol,

    /// Plain IPs for `Udp`/`Tcp`, a URL for `Doh`/`Doh3`, a hostname for
    /// `Dot`/`Doq`. Multiple entries are alternates, tried in order.
    pub addrs: Vec<String>,

    /// IPs used to resolve the endpoint's hostname. These are queried directly
    /// over plain UDP, never through our own forwarder — otherwise pointing
    /// Windows at 127.0.0.1 would deadlock the first DoH lookup on itself.
    #[serde(default)]
    pub bootstrap: Vec<IpAddr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DnsServer {
    /// Stable slug. Unlike the previous implementation, which regenerated a
    /// random ID on every read, this survives restarts so selections and
    /// latency history can reference it.
    pub id: String,
    pub name: String,

    /// Distinguishes a provider's filtering tiers, e.g. "Family".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,

    /// Icon key, resolved to a bundled SVG by the UI.
    pub provider: String,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,

    pub endpoints: Vec<Endpoint>,

    #[serde(default = "default_source")]
    pub source: Source,
}

fn default_source() -> Source {
    Source::Builtin
}

impl DnsServer {
    /// Display name including the variant, e.g. "Quad9 — Unsecured".
    pub fn display_name(&self) -> String {
        match &self.variant {
            Some(variant) => format!("{} — {}", self.name, variant),
            None => self.name.clone(),
        }
    }

    pub fn supports(&self, protocol: Protocol) -> bool {
        self.endpoints.iter().any(|e| e.protocol == protocol)
    }

    pub fn endpoint(&self, protocol: Protocol) -> Option<&Endpoint> {
        self.endpoints.iter().find(|e| e.protocol == protocol)
    }

    /// Case-insensitive match against name, variant, tags, and addresses —
    /// what the server browser's search box runs.
    pub fn matches(&self, needle: &str) -> bool {
        let needle = needle.trim().to_ascii_lowercase();
        if needle.is_empty() {
            return true;
        }

        let in_name = self.name.to_ascii_lowercase().contains(&needle)
            || self
                .variant
                .as_deref()
                .is_some_and(|v| v.to_ascii_lowercase().contains(&needle));

        in_name
            || self
                .tags
                .iter()
                .any(|t| t.to_ascii_lowercase().contains(&needle))
            || self
                .endpoints
                .iter()
                .flat_map(|e| &e.addrs)
                .any(|a| a.to_ascii_lowercase().contains(&needle))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ValidationError {
    #[error("server id must not be empty")]
    EmptyId,

    #[error("server '{0}' has no endpoints")]
    NoEndpoints(String),

    #[error("server '{id}' lists {protocol:?} twice")]
    DuplicateProtocol { id: String, protocol: Protocol },

    #[error("server '{id}': {protocol:?} endpoint has no addresses")]
    NoAddresses { id: String, protocol: Protocol },

    #[error("server '{id}': '{addr}' is not a valid address for {protocol:?}")]
    BadAddress {
        id: String,
        protocol: Protocol,
        addr: String,
    },

    #[error("server '{id}': {protocol:?} endpoint '{addr}' needs bootstrap addresses")]
    MissingBootstrap {
        id: String,
        protocol: Protocol,
        addr: String,
    },
}

impl DnsServer {
    /// Reject records that would fail at connect time, so a bad catalog entry
    /// or a malformed custom server surfaces immediately rather than as a
    /// mysterious resolution failure later.
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.id.trim().is_empty() {
            return Err(ValidationError::EmptyId);
        }
        if self.endpoints.is_empty() {
            return Err(ValidationError::NoEndpoints(self.id.clone()));
        }

        let mut seen: Vec<Protocol> = Vec::with_capacity(self.endpoints.len());
        for endpoint in &self.endpoints {
            if seen.contains(&endpoint.protocol) {
                return Err(ValidationError::DuplicateProtocol {
                    id: self.id.clone(),
                    protocol: endpoint.protocol,
                });
            }
            seen.push(endpoint.protocol);

            if endpoint.addrs.is_empty() {
                return Err(ValidationError::NoAddresses {
                    id: self.id.clone(),
                    protocol: endpoint.protocol,
                });
            }

            for addr in &endpoint.addrs {
                validate_addr(&self.id, endpoint, addr)?;
            }
        }

        Ok(())
    }
}

fn validate_addr(id: &str, endpoint: &Endpoint, addr: &str) -> Result<(), ValidationError> {
    let bad = || ValidationError::BadAddress {
        id: id.to_owned(),
        protocol: endpoint.protocol,
        addr: addr.to_owned(),
    };

    match endpoint.protocol {
        Protocol::Udp | Protocol::Tcp => {
            // Bare IP or IP:port. Hostnames are rejected: resolving them would
            // need DNS, which is the thing being configured.
            if addr.parse::<IpAddr>().is_err() && addr.parse::<SocketAddr>().is_err() {
                return Err(bad());
            }
        }

        Protocol::Doh | Protocol::Doh3 => {
            let rest = addr.strip_prefix("https://").ok_or_else(bad)?;
            let host = rest.split('/').next().unwrap_or_default();
            if host.is_empty() || !rest.contains('/') {
                return Err(bad());
            }
            require_bootstrap(id, endpoint, addr, host)?;
        }

        Protocol::Dot | Protocol::Doq => {
            // Hostname, optionally with a port. The name matters: it is what
            // the server certificate is validated against.
            let host = addr.rsplit_once(':').map_or(addr, |(h, _)| h);
            if host.is_empty() || host.contains('/') {
                return Err(bad());
            }
            require_bootstrap(id, endpoint, addr, host)?;
        }
    }

    Ok(())
}

/// An encrypted endpoint named by hostname is unreachable without bootstrap
/// IPs once we are the system resolver.
fn require_bootstrap(
    id: &str,
    endpoint: &Endpoint,
    addr: &str,
    host: &str,
) -> Result<(), ValidationError> {
    let host = host.split(':').next().unwrap_or(host);
    let is_literal_ip = host.parse::<IpAddr>().is_ok()
        || host
            .strip_prefix('[')
            .and_then(|h| h.strip_suffix(']'))
            .is_some_and(|h| h.parse::<IpAddr>().is_ok());

    if !is_literal_ip && endpoint.bootstrap.is_empty() {
        return Err(ValidationError::MissingBootstrap {
            id: id.to_owned(),
            protocol: endpoint.protocol,
            addr: addr.to_owned(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn endpoint(protocol: Protocol, addr: &str) -> Endpoint {
        Endpoint {
            protocol,
            addrs: vec![addr.to_owned()],
            bootstrap: vec![IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))],
        }
    }

    fn server(endpoints: Vec<Endpoint>) -> DnsServer {
        DnsServer {
            id: "test".into(),
            name: "Test".into(),
            variant: None,
            provider: "test".into(),
            tags: vec!["fast".into()],
            homepage: None,
            endpoints,
            source: Source::Builtin,
        }
    }

    #[test]
    fn accepts_well_formed_endpoints() {
        let s = server(vec![
            Endpoint {
                protocol: Protocol::Udp,
                addrs: vec!["1.1.1.1".into(), "1.0.0.1".into()],
                bootstrap: vec![],
            },
            endpoint(Protocol::Doh, "https://cloudflare-dns.com/dns-query"),
            endpoint(Protocol::Dot, "1dot1dot1dot1.cloudflare-dns.com"),
            endpoint(Protocol::Doq, "dns.adguard-dns.com:853"),
        ]);

        assert_eq!(s.validate(), Ok(()));
    }

    #[test]
    fn rejects_hostname_for_plain_dns() {
        // Accepting this would require resolving a name in order to configure
        // the resolver.
        let s = server(vec![endpoint(Protocol::Udp, "dns.google")]);
        assert!(matches!(
            s.validate(),
            Err(ValidationError::BadAddress { .. })
        ));
    }

    #[test]
    fn rejects_doh_without_a_path() {
        let s = server(vec![endpoint(Protocol::Doh, "https://cloudflare-dns.com")]);
        assert!(matches!(
            s.validate(),
            Err(ValidationError::BadAddress { .. })
        ));
    }

    #[test]
    fn rejects_encrypted_hostname_without_bootstrap() {
        let s = server(vec![Endpoint {
            protocol: Protocol::Doh,
            addrs: vec!["https://dns.quad9.net/dns-query".into()],
            bootstrap: vec![],
        }]);

        assert!(matches!(
            s.validate(),
            Err(ValidationError::MissingBootstrap { .. })
        ));
    }

    #[test]
    fn allows_encrypted_endpoint_addressed_by_ip() {
        // No name to resolve means no bootstrap needed.
        let s = server(vec![Endpoint {
            protocol: Protocol::Doh,
            addrs: vec!["https://9.9.9.9/dns-query".into()],
            bootstrap: vec![],
        }]);

        assert_eq!(s.validate(), Ok(()));
    }

    #[test]
    fn rejects_duplicate_protocols() {
        let s = server(vec![
            endpoint(Protocol::Doh, "https://a.example/dns-query"),
            endpoint(Protocol::Doh, "https://b.example/dns-query"),
        ]);

        assert!(matches!(
            s.validate(),
            Err(ValidationError::DuplicateProtocol { .. })
        ));
    }

    #[test]
    fn search_matches_name_tag_and_address() {
        let s = server(vec![Endpoint {
            protocol: Protocol::Udp,
            addrs: vec!["9.9.9.9".into()],
            bootstrap: vec![],
        }]);

        assert!(s.matches("test"));
        assert!(s.matches("FAST"));
        assert!(s.matches("9.9.9"));
        assert!(s.matches(""));
        assert!(!s.matches("cloudflare"));
    }

    #[test]
    fn only_encrypted_protocols_need_the_forwarder() {
        assert!(!Protocol::Udp.requires_forwarder());
        assert!(!Protocol::Tcp.requires_forwarder());
        for p in [Protocol::Dot, Protocol::Doh, Protocol::Doq, Protocol::Doh3] {
            assert!(p.requires_forwarder(), "{p:?} should need the forwarder");
        }
    }
}
