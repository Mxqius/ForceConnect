//! The forwarder's answer cache.
//!
//! Encrypted transports cost a round trip that plain DNS does not, and the OS
//! asks the same handful of names over and over — the system resolver cache
//! sits in front of us, but browsers and runtimes that resolve on their own do
//! not use it. Answering repeats locally is what makes an encrypted upstream
//! feel no slower than the network's own resolver.
//!
//! Message-level, keyed by question: the cached thing is the upstream's whole
//! reply, with TTLs counted down on the way out so a client never holds an
//! answer longer than the authority allowed.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use hickory_proto::op::{Message, Query, ResponseCode};
use hickory_proto::rr::{DNSClass, Name, RData, Record, RecordType};

/// Entries held at most. A desktop resolves a few hundred distinct names an
/// hour; this is room for a long session without growing without bound.
pub const CAPACITY: usize = 4096;

/// Longest any answer is kept, whatever its TTL says. Bounds how long a
/// change at the authority — or a switch of upstream we did not see — can be
/// masked by us.
const MAX_TTL: u32 = 3600;

/// How long a negative answer with no SOA to go by is kept. Short: these are
/// usually typos, and a name that starts existing should not stay missing.
const NEGATIVE_TTL: u32 = 60;

/// The request bits that change what an answer contains.
///
/// With DO set the reply carries RRSIGs, and with CD set it may carry data a
/// validating resolver would have rejected. Serving one kind to a client that
/// asked for the other is wrong both ways, so they are cached apart.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct QueryFlags {
    pub dnssec_ok: bool,
    pub checking_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    name: Name,
    record_type: RecordType,
    class: DNSClass,
    flags: QueryFlags,
}

impl Key {
    fn of(query: &Query, flags: QueryFlags) -> Self {
        Self {
            // DNS names compare case-insensitively; clients randomise case
            // (0x20 encoding), and keying on the raw name would miss every time.
            name: query.name().to_lowercase(),
            record_type: query.query_type(),
            class: query.query_class(),
            flags,
        }
    }
}

#[derive(Debug, Clone)]
struct Entry {
    response_code: ResponseCode,
    authentic_data: bool,
    answers: Vec<Record>,
    authorities: Vec<Record>,
    additionals: Vec<Record>,
    stored: Instant,
    ttl: u32,
}

impl Entry {
    fn expires(&self) -> Instant {
        self.stored + Duration::from_secs(u64::from(self.ttl))
    }
}

/// A cached reply, with TTLs already reduced by the time it spent here.
#[derive(Debug, Clone)]
pub struct Cached {
    pub response_code: ResponseCode,
    pub authentic_data: bool,
    pub answers: Vec<Record>,
    pub authorities: Vec<Record>,
    pub additionals: Vec<Record>,
}

#[derive(Debug)]
pub struct Cache {
    entries: Mutex<HashMap<Key, Entry>>,
    capacity: usize,
}

impl Default for Cache {
    fn default() -> Self {
        Self::with_capacity(CAPACITY)
    }
}

impl Cache {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
            capacity: capacity.max(1),
        }
    }

    pub fn len(&self) -> usize {
        self.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&self) {
        self.lock().clear();
    }

    pub fn get(&self, query: &Query, flags: QueryFlags, now: Instant) -> Option<Cached> {
        let key = Key::of(query, flags);
        let mut entries = self.lock();

        let entry = entries.get(&key)?;
        if now >= entry.expires() {
            entries.remove(&key);
            return None;
        }

        let elapsed = u32::try_from(now.duration_since(entry.stored).as_secs()).unwrap_or(u32::MAX);
        let age = |records: &[Record]| {
            records
                .iter()
                .map(|record| {
                    let mut record = record.clone();
                    record.decrement_ttl(elapsed);
                    record
                })
                .collect()
        };

        Some(Cached {
            response_code: entry.response_code,
            authentic_data: entry.authentic_data,
            answers: age(&entry.answers),
            authorities: age(&entry.authorities),
            additionals: age(&entry.additionals),
        })
    }

    /// Keep an upstream reply, if it is the kind that can be kept.
    pub fn insert(&self, query: &Query, flags: QueryFlags, reply: &Message, now: Instant) {
        let Some(ttl) = cacheable_ttl(reply) else {
            return;
        };

        let mut entries = self.lock();
        if entries.len() >= self.capacity {
            entries.retain(|_, entry| now < entry.expires());
        }
        if entries.len() >= self.capacity {
            // Still full of live answers: drop whichever would have gone first.
            if let Some(soonest) = entries
                .iter()
                .min_by_key(|(_, entry)| entry.expires())
                .map(|(key, _)| key.clone())
            {
                entries.remove(&soonest);
            }
        }

        entries.insert(
            Key::of(query, flags),
            Entry {
                response_code: reply.metadata.response_code,
                authentic_data: reply.metadata.authentic_data,
                answers: reply.answers.clone(),
                authorities: reply.authorities.clone(),
                additionals: reply.additionals.clone(),
                stored: now,
                ttl,
            },
        );
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<Key, Entry>> {
        // A poisoned lock only means another query panicked mid-insert; the map
        // itself is still a valid map, and losing the cache over it is worse.
        self.entries.lock().unwrap_or_else(|err| err.into_inner())
    }
}

/// How long a reply may be kept, or `None` if it must not be.
///
/// Only definitive answers are cached: success, and NXDOMAIN. A SERVFAIL or a
/// REFUSED is a statement about the upstream at that moment, and caching it
/// would turn one dropped packet into a minute of failures. Truncated replies
/// are incomplete by definition.
fn cacheable_ttl(reply: &Message) -> Option<u32> {
    if reply.metadata.truncation {
        return None;
    }

    let ttl = match reply.metadata.response_code {
        ResponseCode::NoError if !reply.answers.is_empty() => {
            reply.answers.iter().map(|record| record.ttl).min()?
        }
        // NODATA and NXDOMAIN: RFC 2308 says the negative TTL is the lesser of
        // the SOA's own TTL and its MINIMUM field.
        ResponseCode::NoError | ResponseCode::NXDomain => negative_ttl(reply),
        _ => return None,
    };

    (ttl > 0).then_some(ttl.min(MAX_TTL))
}

fn negative_ttl(reply: &Message) -> u32 {
    reply
        .authorities
        .iter()
        .find_map(|record| match &record.data {
            RData::SOA(soa) => Some(record.ttl.min(soa.minimum)),
            _ => None,
        })
        .unwrap_or(NEGATIVE_TTL)
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use std::str::FromStr;

    use hickory_proto::op::{Message, MessageType, OpCode};
    use hickory_proto::rr::rdata::A;

    use super::*;

    const NONE: QueryFlags = QueryFlags {
        dnssec_ok: false,
        checking_disabled: false,
    };

    fn query(name: &str) -> Query {
        Query::query(Name::from_str(name).unwrap(), RecordType::A)
    }

    fn reply(name: &str, ttl: u32, code: ResponseCode) -> Message {
        let mut message = Message::new(1, MessageType::Response, OpCode::Query);
        message.metadata.response_code = code;
        if code == ResponseCode::NoError {
            message.answers.push(Record::from_rdata(
                Name::from_str(name).unwrap(),
                ttl,
                RData::A(A(Ipv4Addr::new(192, 0, 2, 1))),
            ));
        }
        message
    }

    #[test]
    fn answers_come_back_with_their_ttl_counted_down() {
        let cache = Cache::default();
        let now = Instant::now();
        cache.insert(
            &query("example.com."),
            NONE,
            &reply("example.com.", 300, ResponseCode::NoError),
            now,
        );

        let hit = cache
            .get(&query("example.com."), NONE, now + Duration::from_secs(100))
            .expect("should be cached");
        assert_eq!(hit.answers[0].ttl, 200);
    }

    #[test]
    fn lookups_ignore_case() {
        let cache = Cache::default();
        let now = Instant::now();
        cache.insert(
            &query("Example.COM."),
            NONE,
            &reply("example.com.", 300, ResponseCode::NoError),
            now,
        );

        assert!(cache.get(&query("eXaMpLe.com."), NONE, now).is_some());
    }

    #[test]
    fn expired_entries_are_not_served() {
        let cache = Cache::default();
        let now = Instant::now();
        cache.insert(
            &query("example.com."),
            NONE,
            &reply("example.com.", 10, ResponseCode::NoError),
            now,
        );

        assert!(
            cache
                .get(&query("example.com."), NONE, now + Duration::from_secs(10))
                .is_none()
        );
        assert!(cache.is_empty(), "an expired hit should be evicted");
    }

    #[test]
    fn failures_and_zero_ttls_are_not_kept() {
        let cache = Cache::default();
        let now = Instant::now();
        cache.insert(
            &query("a.example."),
            NONE,
            &reply("a.example.", 300, ResponseCode::ServFail),
            now,
        );
        cache.insert(
            &query("b.example."),
            NONE,
            &reply("b.example.", 0, ResponseCode::NoError),
            now,
        );

        assert!(cache.is_empty());
    }

    #[test]
    fn nxdomain_without_an_soa_is_kept_briefly() {
        let cache = Cache::default();
        let now = Instant::now();
        cache.insert(
            &query("nope.example."),
            NONE,
            &reply("nope.example.", 0, ResponseCode::NXDomain),
            now,
        );

        let later = now + Duration::from_secs(u64::from(NEGATIVE_TTL) - 1);
        assert_eq!(
            cache
                .get(&query("nope.example."), NONE, later)
                .unwrap()
                .response_code,
            ResponseCode::NXDomain
        );
        assert!(
            cache
                .get(
                    &query("nope.example."),
                    NONE,
                    now + Duration::from_secs(u64::from(NEGATIVE_TTL))
                )
                .is_none()
        );
    }

    #[test]
    fn long_ttls_are_capped() {
        let cache = Cache::default();
        let now = Instant::now();
        cache.insert(
            &query("example.com."),
            NONE,
            &reply("example.com.", 86_400, ResponseCode::NoError),
            now,
        );

        let past_cap = now + Duration::from_secs(u64::from(MAX_TTL));
        assert!(cache.get(&query("example.com."), NONE, past_cap).is_none());
    }

    #[test]
    fn a_full_cache_drops_the_soonest_to_expire() {
        let cache = Cache::with_capacity(2);
        let now = Instant::now();
        cache.insert(
            &query("short.example."),
            NONE,
            &reply("short.example.", 30, ResponseCode::NoError),
            now,
        );
        cache.insert(
            &query("long.example."),
            NONE,
            &reply("long.example.", 600, ResponseCode::NoError),
            now,
        );
        cache.insert(
            &query("new.example."),
            NONE,
            &reply("new.example.", 300, ResponseCode::NoError),
            now,
        );

        assert_eq!(cache.len(), 2);
        assert!(cache.get(&query("short.example."), NONE, now).is_none());
        assert!(cache.get(&query("long.example."), NONE, now).is_some());
        assert!(cache.get(&query("new.example."), NONE, now).is_some());
    }

    #[test]
    fn a_dnssec_answer_is_not_served_to_a_plain_query() {
        let cache = Cache::default();
        let now = Instant::now();
        let signed = QueryFlags {
            dnssec_ok: true,
            checking_disabled: false,
        };
        cache.insert(
            &query("example.com."),
            signed,
            &reply("example.com.", 300, ResponseCode::NoError),
            now,
        );

        assert!(cache.get(&query("example.com."), NONE, now).is_none());
        assert!(cache.get(&query("example.com."), signed, now).is_some());
    }
}
