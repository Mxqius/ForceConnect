use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use forceconnect_privileged::DnsWriter;

/// Application state shared across commands.
///
/// One writer for the whole process. It is the only thing in the application
/// that changes system DNS settings, and serialising access through a single
/// mutex is what keeps two commands — a connect racing a disconnect, say — from
/// interleaving reads and writes and recording the wrong "previous"
/// configuration to restore later.
pub struct AppState {
    dns: Arc<Mutex<DnsWriter>>,
    /// Whether closing the main window should hide it to the tray instead of
    /// exiting the process.
    ///
    /// The setting itself lives in the frontend's preferences store (so it
    /// persists across launches); this is just the copy the window-close
    /// handler can read synchronously, kept in sync by
    /// `set_minimize_to_tray` on hydrate and on every toggle.
    minimize_to_tray: AtomicBool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            dns: Arc::new(Mutex::new(DnsWriter::default())),
            minimize_to_tray: AtomicBool::new(false),
        }
    }
}

impl AppState {
    /// A handle to the writer, cloneable into a blocking task.
    ///
    /// The Win32 calls behind it are synchronous, so commands move the locked
    /// section onto a blocking thread rather than holding a lock across an
    /// await point.
    pub fn dns(&self) -> Arc<Mutex<DnsWriter>> {
        Arc::clone(&self.dns)
    }

    /// Put every adapter back the way it was, on the way out of the process.
    pub fn restore_all(&self) {
        lock(&self.dns).restore_all();
    }

    /// Snapshot every adapter's DNS as the "before ForceConnect" state.
    pub fn capture_baseline(&self) {
        lock(&self.dns).capture_baseline();
    }

    pub fn has_pending_changes(&self) -> bool {
        lock(&self.dns).has_pending()
    }

    pub fn minimize_to_tray(&self) -> bool {
        self.minimize_to_tray.load(Ordering::Relaxed)
    }

    pub fn set_minimize_to_tray(&self, enabled: bool) {
        self.minimize_to_tray.store(enabled, Ordering::Relaxed);
    }
}

/// Take the lock, ignoring poisoning.
///
/// A panic in one command must not turn every later DNS operation into an
/// error — the adapter would then be stuck pointing at our resolver with no
/// way to put it back, which is the exact failure the journal exists to avoid.
pub fn lock(dns: &Mutex<DnsWriter>) -> std::sync::MutexGuard<'_, DnsWriter> {
    dns.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
