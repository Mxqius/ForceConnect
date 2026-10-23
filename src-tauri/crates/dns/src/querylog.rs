//! The resolver's recent lookups, for the Activity screen and the leak test.
//!
//! Memory only, and bounded: it exists so a user can see what their machine is
//! asking and whether it is being answered, not as a history. Nothing here is
//! written to disk, and it goes when the resolver stops.

use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

/// Entries kept. Enough to scroll back through a page load or two; old ones
/// fall off the front.
pub const CAPACITY: usize = 500;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Answered by the chosen server.
    Answered,
    /// Answered by the backup server, because the chosen one did not.
    Backup,
    /// Answered from the cache without going upstream.
    Cached,
    /// No server answered; the client got SERVFAIL.
    Failed,
    /// Answered locally on purpose — Firefox's DoH canary.
    Blocked,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    /// Increasing, so a poller can ask for only what is new.
    pub id: u64,
    pub at: SystemTime,
    /// Lower-case, without the trailing dot.
    pub name: String,
    pub record_type: String,
    pub outcome: Outcome,
    /// `NoError`, `NXDomain`, … — empty when nothing answered.
    pub response_code: String,
    pub elapsed: Duration,
}

#[derive(Debug)]
pub struct QueryLog {
    entries: Mutex<VecDeque<LogEntry>>,
    next_id: AtomicU64,
    capacity: usize,
}

impl Default for QueryLog {
    fn default() -> Self {
        Self::with_capacity(CAPACITY)
    }
}

impl QueryLog {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Mutex::new(VecDeque::with_capacity(capacity)),
            next_id: AtomicU64::new(1),
            capacity: capacity.max(1),
        }
    }

    pub fn push(
        &self,
        name: &str,
        record_type: String,
        outcome: Outcome,
        response_code: String,
        elapsed: Duration,
    ) {
        let entry = LogEntry {
            id: self.next_id.fetch_add(1, Ordering::Relaxed),
            at: SystemTime::now(),
            name: normalize(name),
            record_type,
            outcome,
            response_code,
            elapsed,
        };

        let mut entries = self.lock();
        if entries.len() >= self.capacity {
            entries.pop_front();
        }
        entries.push_back(entry);
    }

    /// Entries newer than `after`, oldest first.
    pub fn since(&self, after: u64) -> Vec<LogEntry> {
        self.lock()
            .iter()
            .filter(|entry| entry.id > after)
            .cloned()
            .collect()
    }

    /// Whether a name was asked about at all — what the leak test checks.
    pub fn saw(&self, name: &str) -> bool {
        let name = normalize(name);
        self.lock().iter().any(|entry| entry.name == name)
    }

    pub fn clear(&self) {
        self.lock().clear();
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, VecDeque<LogEntry>> {
        self.entries.lock().unwrap_or_else(|err| err.into_inner())
    }
}

fn normalize(name: &str) -> String {
    name.trim_end_matches('.').to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push(log: &QueryLog, name: &str) {
        log.push(
            name,
            "A".into(),
            Outcome::Answered,
            "NoError".into(),
            Duration::from_millis(12),
        );
    }

    #[test]
    fn a_poller_gets_only_what_is_new() {
        let log = QueryLog::default();
        push(&log, "a.example.");
        push(&log, "b.example.");
        let first = log.since(0);
        assert_eq!(first.len(), 2);

        push(&log, "c.example.");
        let next = log.since(first.last().unwrap().id);
        assert_eq!(next.len(), 1);
        assert_eq!(next[0].name, "c.example");
    }

    #[test]
    fn the_oldest_entries_fall_off() {
        let log = QueryLog::with_capacity(2);
        push(&log, "a.example.");
        push(&log, "b.example.");
        push(&log, "c.example.");

        let names: Vec<_> = log.since(0).into_iter().map(|e| e.name).collect();
        assert_eq!(names, ["b.example", "c.example"]);
    }

    #[test]
    fn names_match_regardless_of_case_and_trailing_dot() {
        let log = QueryLog::default();
        push(&log, "FC-Leak-1234.Example.COM.");
        assert!(log.saw("fc-leak-1234.example.com"));
        assert!(!log.saw("fc-leak-9999.example.com"));
    }
}
