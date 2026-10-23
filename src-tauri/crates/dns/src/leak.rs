//! The leak test: are this machine's lookups going where the user thinks?
//!
//! Two questions, answered from the outside in.
//!
//! 1. *Does the system's resolver path reach ours?* A unique name is looked up
//!    through the operating system — the way any application's lookup goes —
//!    and the resolver's query log is checked for it. If it never arrived, the
//!    OS answered from somewhere else, whatever the adapter settings say.
//! 2. *Who answers the internet's side?* `whoami.akamai.net` resolves to the
//!    address of whichever recursive resolver asked Akamai's servers, so it
//!    names the egress the lookup really left by. Team Cymru's DNS whois then
//!    says who owns that address.
//!
//! The ownership lookups go to the connected server over its own protocol, not
//! through the OS, so running the test does not itself leak anything.

use std::net::IpAddr;
use std::time::Duration;

use futures_util::StreamExt;
use hickory_proto::op::{DnsRequest, DnsRequestOptions, Query};
use hickory_proto::rr::{Name, RData, RecordType};
use hickory_resolver::net::DnsHandle;
use hickory_resolver::net::runtime::TokioRuntimeProvider;

use crate::server::Endpoint;
use crate::upstream;

/// Answers with the address of the resolver that asked for it.
pub const WHOAMI: &str = "whoami.akamai.net";

const TIMEOUT: Duration = Duration::from_secs(5);

/// A name nobody has looked up before, so no cache anywhere can answer it and
/// the lookup has to travel the whole path.
pub fn nonce_name() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    let seed = (nanos as u64) ^ (u64::from(std::process::id()) << 32);
    format!("fc-leak-{seed:016x}.example.com")
}

/// Resolve through the operating system, as any application would.
///
/// Errors are not interesting here — the nonce name does not exist, so the
/// expected answer is NXDOMAIN — only whether the lookup happened, and for
/// [`WHOAMI`], what came back.
pub async fn system_lookup(name: &str) -> Vec<IpAddr> {
    match tokio::time::timeout(TIMEOUT, tokio::net::lookup_host((name, 0))).await {
        Ok(Ok(addrs)) => {
            let mut ips: Vec<IpAddr> = addrs.map(|addr| addr.ip()).collect();
            ips.sort();
            ips.dedup();
            ips
        }
        _ => Vec::new(),
    }
}

/// Who announces `ip`, e.g. "Cloudflare, Inc.", asked of `via`.
pub async fn owner_of(ip: IpAddr, via: &Endpoint) -> Option<String> {
    let opts = upstream::resolver_opts(TIMEOUT, 1);
    let pool = upstream::pool(via, opts, TokioRuntimeProvider::new()).ok()?;

    let origin = txt(&pool, &origin_name(ip)).await?;
    let asn = parse_origin(&origin)?;
    let record = txt(&pool, &format!("AS{asn}.asn.cymru.com")).await?;
    parse_as_name(&record)
}

/// One TXT lookup, tried twice: the test runs once, on demand, often on a
/// network that is already misbehaving, and a single lost exchange would
/// report "unknown owner" for an answer that was one retry away.
async fn txt(
    pool: &hickory_resolver::NameServerPool<TokioRuntimeProvider>,
    name: &str,
) -> Option<String> {
    for _ in 0..2 {
        if let Some(text) = txt_once(pool, name).await {
            return Some(text);
        }
    }
    None
}

async fn txt_once(
    pool: &hickory_resolver::NameServerPool<TokioRuntimeProvider>,
    name: &str,
) -> Option<String> {
    let query = Query::query(Name::from_ascii(name).ok()?, RecordType::TXT);
    let request = DnsRequest::from_query(query, DnsRequestOptions::default());

    let mut stream = pool.send(request);
    let reply = tokio::time::timeout(TIMEOUT, stream.next())
        .await
        .ok()??
        .ok()?
        .into_message();

    reply.answers.iter().find_map(|record| match &record.data {
        RData::TXT(txt) => Some(
            txt.txt_data
                .iter()
                .map(|part| String::from_utf8_lossy(part))
                .collect::<String>(),
        ),
        _ => None,
    })
}

/// `1.2.3.4` → `4.3.2.1.origin.asn.cymru.com`, and the nibble form for IPv6.
fn origin_name(ip: IpAddr) -> String {
    match ip {
        IpAddr::V4(v4) => {
            let [a, b, c, d] = v4.octets();
            format!("{d}.{c}.{b}.{a}.origin.asn.cymru.com")
        }
        IpAddr::V6(v6) => {
            let nibbles: Vec<String> = v6
                .octets()
                .iter()
                .rev()
                .flat_map(|byte| [byte & 0x0f, byte >> 4])
                .map(|nibble| format!("{nibble:x}"))
                .collect();
            format!("{}.origin6.asn.cymru.com", nibbles.join("."))
        }
    }
}

/// `"13335 | 172.64.0.0/13 | US | arin | 2010-07-14"` → `13335`. A prefix
/// announced by several networks lists them space-separated; the first will do.
fn parse_origin(record: &str) -> Option<u32> {
    record
        .split('|')
        .next()?
        .split_whitespace()
        .next()?
        .parse()
        .ok()
}

/// `"13335 | US | arin | 2010-07-14 | CLOUDFLARENET - Cloudflare, Inc., US"`
/// → `"Cloudflare, Inc."` — the registrant's own name, without the handle in
/// front or the country on the end.
fn parse_as_name(record: &str) -> Option<String> {
    let field = record.split('|').nth(4)?.trim();
    let name = field.split_once(" - ").map_or(field, |(_, name)| name);
    let name = match name.rsplit_once(", ") {
        Some((rest, country))
            if country.len() == 2 && country.chars().all(|c| c.is_ascii_uppercase()) =>
        {
            rest
        }
        _ => name,
    };
    let name = name.trim();
    (!name.is_empty()).then(|| name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonce_names_are_unique_and_lowercase() {
        let a = nonce_name();
        std::thread::sleep(Duration::from_millis(1));
        let b = nonce_name();
        assert_ne!(a, b);
        assert_eq!(a, a.to_ascii_lowercase());
        assert!(a.ends_with(".example.com"));
    }

    #[test]
    fn origin_names_reverse_the_address() {
        assert_eq!(
            origin_name("172.68.1.2".parse().unwrap()),
            "2.1.68.172.origin.asn.cymru.com"
        );
        let v6 = origin_name("2606:4700::1".parse().unwrap());
        assert!(v6.starts_with("1.0.0.0.0.0.0.0."));
        assert!(v6.ends_with(".0.0.7.4.6.0.6.2.origin6.asn.cymru.com"));
    }

    #[test]
    fn cymru_records_are_parsed() {
        assert_eq!(
            parse_origin("13335 | 172.64.0.0/13 | US | arin | 2010-07-14"),
            Some(13335)
        );
        assert_eq!(
            parse_origin("16509 14618 | 3.0.0.0/15 | US | arin | 2017-12-01"),
            Some(16509)
        );
        assert_eq!(parse_origin("garbage"), None);

        assert_eq!(
            parse_as_name("13335 | US | arin | 2010-07-14 | CLOUDFLARENET - Cloudflare, Inc., US")
                .as_deref(),
            Some("Cloudflare, Inc.")
        );
        assert_eq!(
            parse_as_name("19281 | US | arin | 2017-04-27 | QUAD9-AS-1, CH").as_deref(),
            Some("QUAD9-AS-1")
        );
        assert_eq!(parse_as_name("13335 | US"), None);
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore = "reaches the public internet"]
    async fn the_egress_and_its_owner_are_found_live() {
        let doh = Endpoint {
            protocol: crate::Protocol::Doh,
            addrs: vec!["https://cloudflare-dns.com/dns-query".into()],
            bootstrap: vec!["1.1.1.1".parse().unwrap()],
        };

        let owner = owner_of("1.1.1.1".parse().unwrap(), &doh).await;
        println!("1.1.1.1 belongs to {owner:?}");
        assert!(owner.is_some_and(|o| o.to_lowercase().contains("cloudflare")));

        let egress = system_lookup(WHOAMI).await;
        println!("this machine's lookups leave from {egress:?}");
        assert!(!egress.is_empty());
        if let Some(ip) = egress.first() {
            println!("owned by {:?}", owner_of(*ip, &doh).await);
        }
    }
}
