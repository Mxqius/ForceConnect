//! Crash-safe record of DNS changes awaiting revert.
//!
//! The app reverts what it changed on a clean exit. If it is killed instead —
//! a hard crash, a forced shutdown, a power loss — the adapter is left pointing
//! at a resolver that is no longer running, and the machine has no working DNS
//! until the user finds the setting themselves.
//!
//! So before applying anything, the pending revert is written to disk and
//! flushed. On the next launch [`crate::recover`] looks for that file: if it
//! exists, a revert is owed and is performed before anything else runs.

use std::io::Write;
use std::path::{Path, PathBuf};

use forceconnect_net::{DnsConfig, Family};
use serde::{Deserialize, Serialize};

/// One adapter/family pair we changed, and what it looked like before.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingRevert {
    pub guid: String,
    pub family: Family,
    /// What to put back on recovery. Named for its original meaning; it now
    /// holds the release policy's destination, which is the pre-change
    /// configuration unless the user chose DHCP or a default DNS instead.
    pub previous: DnsConfig,
    /// Unix seconds, for diagnostics when a stale file turns up.
    pub applied_at: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Journal {
    pub entries: Vec<PendingRevert>,
}

impl Journal {
    /// Record a change, replacing any earlier entry for the same adapter and
    /// family.
    ///
    /// Replacing rather than appending is the point: the *first* recorded
    /// state is the user's real configuration, so switching servers three times
    /// must still revert to what they started with, not to the second server.
    pub fn record(&mut self, guid: &str, family: Family, previous: DnsConfig) {
        if self
            .entries
            .iter()
            .any(|e| e.guid == guid && e.family == family)
        {
            return;
        }

        self.entries.push(PendingRevert {
            guid: guid.to_owned(),
            family,
            previous,
            applied_at: now_unix(),
        });
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default()
}

/// Default location, under `%PROGRAMDATA%` so it survives a user profile and is
/// visible to an elevated process on the next run.
pub fn default_path() -> PathBuf {
    let base = std::env::var_os("PROGRAMDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\ProgramData"));
    base.join("ForceConnect").join("pending-revert.json")
}

/// Write the journal, flushing before returning.
///
/// Written to a sibling temporary file and renamed, so a crash midway through
/// leaves either the old journal or the new one — never a truncated file that
/// would fail to parse and lose the revert entirely.
pub fn save(path: &Path, journal: &Journal) -> std::io::Result<()> {
    if journal.is_empty() {
        return clear(path);
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let temp = path.with_extension("json.tmp");
    let body = serde_json::to_vec_pretty(journal)?;

    {
        let mut file = std::fs::File::create(&temp)?;
        file.write_all(&body)?;
        file.sync_all()?;
    }

    std::fs::rename(&temp, path)
}

pub fn load(path: &Path) -> Option<Journal> {
    let body = std::fs::read(path).ok()?;
    serde_json::from_slice(&body).ok()
}

pub fn clear(path: &Path) -> std::io::Result<()> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("fc-journal-{name}-{}.json", std::process::id()))
    }

    #[test]
    fn the_first_recorded_state_wins() {
        // Switching servers repeatedly must still revert to the configuration
        // the user actually started with.
        let mut journal = Journal::default();
        journal.record("{a}", Family::V4, DnsConfig::Dhcp);
        journal.record(
            "{a}",
            Family::V4,
            DnsConfig::Static(vec!["1.1.1.1".parse::<IpAddr>().unwrap()]),
        );

        assert_eq!(journal.entries.len(), 1);
        assert_eq!(journal.entries[0].previous, DnsConfig::Dhcp);
    }

    #[test]
    fn families_are_tracked_separately() {
        // One adapter can be mid-change on IPv4 and untouched on IPv6, so the
        // pair — not the adapter — is the unit a revert is owed for.
        let mut journal = Journal::default();
        journal.record("{a}", Family::V4, DnsConfig::Dhcp);
        journal.record("{a}", Family::V6, DnsConfig::Dhcp);
        assert_eq!(journal.entries.len(), 2);
    }

    #[test]
    fn round_trips_through_disk() {
        let path = temp_path("roundtrip");
        let mut journal = Journal::default();
        journal.record(
            "{b}",
            Family::V4,
            DnsConfig::Static(vec!["9.9.9.9".parse::<IpAddr>().unwrap()]),
        );

        save(&path, &journal).expect("saves");
        assert_eq!(load(&path).expect("loads"), journal);

        clear(&path).expect("clears");
        assert!(load(&path).is_none());
    }

    #[test]
    fn saving_an_empty_journal_removes_the_file() {
        // A leftover empty file on next launch would look like an owed revert
        // and prompt the user for elevation over nothing.
        let path = temp_path("empty");
        let mut journal = Journal::default();
        journal.record("{c}", Family::V4, DnsConfig::Dhcp);
        save(&path, &journal).expect("saves");
        assert!(path.exists());

        save(&path, &Journal::default()).expect("saves empty");
        assert!(!path.exists());
    }

    #[test]
    fn clearing_a_missing_file_is_not_an_error() {
        assert!(clear(&temp_path("absent")).is_ok());
    }
}
