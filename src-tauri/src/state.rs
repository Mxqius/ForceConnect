use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use forceconnect_privileged::Privileged;

/// Application state shared across commands.
///
/// One privileged handle for the whole process. It is the only thing in the
/// application that changes system DNS settings, and it serialises every call
/// through it — see `forceconnect_privileged::Privileged` for why that matters
/// and for how the rights behind it are held on each platform.
pub struct AppState {
    dns: Arc<Privileged>,
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
            dns: Arc::new(Privileged::default()),
            minimize_to_tray: AtomicBool::new(false),
        }
    }
}

impl AppState {
    /// A handle to the privileged side, cloneable into a blocking task.
    ///
    /// Every call behind it is synchronous — a Win32 call, or a round trip to
    /// the agent — and may wait on a lock, so commands run it on a blocking
    /// thread rather than across an await point.
    pub fn dns(&self) -> Arc<Privileged> {
        Arc::clone(&self.dns)
    }

    /// Put every adapter back the way it was, on the way out of the process.
    pub fn restore_all(&self) {
        self.dns.restore_all();
    }

    pub fn has_pending_changes(&self) -> bool {
        self.dns.has_pending()
    }

    pub fn minimize_to_tray(&self) -> bool {
        self.minimize_to_tray.load(Ordering::Relaxed)
    }

    pub fn set_minimize_to_tray(&self, enabled: bool) {
        self.minimize_to_tray.store(enabled, Ordering::Relaxed);
    }
}
