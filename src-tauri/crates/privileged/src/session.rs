//! The application's one handle on the privileged verbs.
//!
//! [`Privileged`] has the same methods on every platform, so nothing above
//! this crate knows how the rights are held. On Windows it wraps a
//! [`DnsWriter`] in this process, which is elevated as a whole. On Linux and
//! macOS it is a connection to the [agent](crate::agent), started at launch.
//!
//! One handle for the whole process, with every call serialised through it.
//! That is what keeps two commands — a connect racing a disconnect, say — from
//! interleaving reads and writes and recording the wrong "previous"
//! configuration to restore later.

use std::net::IpAddr;
use std::sync::{Arc, Mutex, MutexGuard};

use forceconnect_net::{DnsConfig, Family, InterfaceLuid};

use crate::{PrivilegedError, ReleasePolicy};
#[cfg(windows)]
use crate::{DnsWriter, journal};

#[cfg(unix)]
use crate::agent::{self, CallError, Reply, Request};

/// Take a lock, ignoring poisoning.
///
/// A panic in one command must not turn every later DNS operation into an
/// error — the adapter would then be stuck pointing at our resolver with no
/// way to put it back, which is the exact failure the journal exists to avoid.
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(windows)]
pub struct Privileged {
    writer: Mutex<DnsWriter>,
}

#[cfg(windows)]
impl Default for Privileged {
    fn default() -> Self {
        Self {
            writer: Mutex::new(DnsWriter::default()),
        }
    }
}

#[cfg(windows)]
impl Privileged {
    /// Undo anything a previous run left behind.
    ///
    /// Called once, from the app's setup — after the single-instance check, so
    /// a second launch cannot revert the first one's changes — and before the
    /// UI can ask for anything. `on_recovered` hears how many adapter/family
    /// pairs were put back, so the caller can say so rather than silently
    /// repairing the machine.
    pub fn start(self: &Arc<Self>, on_recovered: impl FnOnce(usize) + Send + 'static) {
        on_recovered(crate::recover(&journal::default_path()));
        // After recovery, so the snapshot is the user's configuration and not
        // whatever a crashed run left on the adapter.
        lock(&self.writer).capture_baseline();
    }

    pub fn is_elevated(&self) -> bool {
        crate::is_elevated()
    }

    pub fn set_dns(&self, luid: InterfaceLuid, servers: &[IpAddr]) -> Result<(), PrivilegedError> {
        lock(&self.writer).set_dns(luid, servers)
    }

    pub fn clear_dns(&self, luid: InterfaceLuid) -> Result<(), PrivilegedError> {
        lock(&self.writer).clear_dns(luid)
    }

    pub fn flush_cache(&self) -> Result<(), PrivilegedError> {
        lock(&self.writer).flush_cache()
    }

    pub fn restore_all(&self) {
        lock(&self.writer).restore_all();
    }

    pub fn has_pending(&self) -> bool {
        lock(&self.writer).has_pending()
    }

    pub fn set_policy(&self, policy: ReleasePolicy) -> Result<(), PrivilegedError> {
        lock(&self.writer).set_policy(policy)
    }

    pub fn release(&self, luid: InterfaceLuid) -> Result<Vec<(Family, DnsConfig)>, PrivilegedError> {
        lock(&self.writer).release(luid)
    }

    /// Every adapter's DNS as captured at launch, keyed by adapter GUID.
    pub fn baseline(&self) -> Result<Vec<(String, Family, DnsConfig)>, PrivilegedError> {
        Ok(lock(&self.writer).baseline_entries())
    }
}

#[cfg(unix)]
enum State {
    NotStarted,
    /// The administrator prompt is up.
    Starting,
    Ready(agent::AgentConnection),
    /// Declined, impossible to ask, or the agent has since gone away.
    Unavailable,
}

#[cfg(unix)]
pub struct Privileged {
    state: Mutex<State>,
    changed: std::sync::Condvar,
}

#[cfg(unix)]
impl Default for Privileged {
    fn default() -> Self {
        Self {
            state: Mutex::new(State::NotStarted),
            changed: std::sync::Condvar::new(),
        }
    }
}

#[cfg(unix)]
impl Privileged {
    /// Ask for administrator rights and start the agent, then replay the
    /// journal through it.
    ///
    /// The prompt is shown from a background thread so the window stays alive
    /// behind it. Every call made in the meantime waits for the outcome
    /// rather than failing — the user is, at that moment, typing the password
    /// that will make it succeed.
    pub fn start(self: &Arc<Self>, on_recovered: impl FnOnce(usize) + Send + 'static) {
        {
            let mut state = lock(&self.state);
            if !matches!(*state, State::NotStarted) {
                return;
            }
            *state = State::Starting;
        }

        let this = Arc::clone(self);
        std::thread::spawn(move || {
            let next = match agent::launch() {
                Ok(mut connection) => match connection.call(&Request::Recover) {
                    Ok(Reply::Recovered(count)) => {
                        on_recovered(count);
                        State::Ready(connection)
                    }
                    Ok(_) => State::Ready(connection),
                    Err(CallError::Refused(err)) => {
                        tracing::warn!("journal recovery failed: {err}");
                        State::Ready(connection)
                    }
                    Err(CallError::Lost(err)) => {
                        tracing::error!("the privileged agent exited during startup: {err}");
                        State::Unavailable
                    }
                },
                Err(err) => {
                    tracing::warn!("{err}");
                    State::Unavailable
                }
            };

            // After recovery, so the snapshot is the user's configuration and
            // not whatever a crashed run left on the adapter.
            let next = match next {
                State::Ready(mut connection) => {
                    match connection.call(&Request::CaptureBaseline) {
                        Err(CallError::Lost(err)) => {
                            tracing::error!("the privileged agent exited during startup: {err}");
                            State::Unavailable
                        }
                        Err(CallError::Refused(err)) => {
                            tracing::warn!("could not capture the launch DNS baseline: {err}");
                            State::Ready(connection)
                        }
                        Ok(_) => State::Ready(connection),
                    }
                }
                other => other,
            };

            *lock(&this.state) = next;
            this.changed.notify_all();
        });
    }

    /// Whether the agent is running. Waits out the prompt if it is still up.
    pub fn is_elevated(&self) -> bool {
        matches!(*self.settled(), State::Ready(_))
    }

    pub fn set_dns(&self, luid: InterfaceLuid, servers: &[IpAddr]) -> Result<(), PrivilegedError> {
        self.call(Request::SetDns {
            luid,
            servers: servers.to_vec(),
        })
        .map(drop)
    }

    pub fn clear_dns(&self, luid: InterfaceLuid) -> Result<(), PrivilegedError> {
        self.call(Request::ClearDns { luid }).map(drop)
    }

    pub fn flush_cache(&self) -> Result<(), PrivilegedError> {
        self.call(Request::FlushCache).map(drop)
    }

    pub fn restore_all(&self) {
        if let Err(err) = self.call(Request::RestoreAll) {
            tracing::error!("restore failed: {err}");
        }
    }

    pub fn set_policy(&self, policy: ReleasePolicy) -> Result<(), PrivilegedError> {
        self.call(Request::SetPolicy { policy }).map(drop)
    }

    pub fn release(&self, luid: InterfaceLuid) -> Result<Vec<(Family, DnsConfig)>, PrivilegedError> {
        match self.call(Request::Release { luid })? {
            Reply::Released(written) => Ok(written),
            other => Err(unexpected(&other)),
        }
    }

    /// Every adapter's DNS as captured at launch, keyed by adapter GUID.
    pub fn baseline(&self) -> Result<Vec<(String, Family, DnsConfig)>, PrivilegedError> {
        match self.call(Request::Baseline)? {
            Reply::Baseline(entries) => Ok(entries),
            other => Err(unexpected(&other)),
        }
    }

    /// Does not wait for the prompt: this is asked on the way out, and an
    /// agent that never started is owed nothing.
    pub fn has_pending(&self) -> bool {
        let mut state = lock(&self.state);
        let State::Ready(connection) = &mut *state else {
            return false;
        };

        match connection.call(&Request::HasPending) {
            Ok(Reply::Pending(pending)) => pending,
            _ => false,
        }
    }

    fn settled(&self) -> MutexGuard<'_, State> {
        let mut state = lock(&self.state);
        while matches!(*state, State::Starting) {
            state = self
                .changed
                .wait(state)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
        state
    }

    fn call(&self, request: Request) -> Result<Reply, PrivilegedError> {
        let mut state = self.settled();
        let State::Ready(connection) = &mut *state else {
            return Err(PrivilegedError::NotElevated);
        };

        match connection.call(&request) {
            Ok(reply) => Ok(reply),
            Err(CallError::Refused(err)) => Err(err),
            Err(CallError::Lost(err)) => {
                // Nothing more can be sent; every later call says so the same
                // way an unelevated process would.
                tracing::error!("lost the privileged agent: {err}");
                *state = State::Unavailable;
                Err(PrivilegedError::NotElevated)
            }
        }
    }
}

/// A reply of the wrong shape means the two halves disagree about the
/// protocol, which only a mismatched build could produce.
#[cfg(unix)]
fn unexpected(reply: &Reply) -> PrivilegedError {
    PrivilegedError::Os(format!("the privileged agent answered unexpectedly: {reply:?}"))
}
