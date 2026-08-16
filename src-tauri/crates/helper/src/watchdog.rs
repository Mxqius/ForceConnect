//! Watching the process that launched us.
//!
//! The watch runs in this direction — high integrity observing medium — because
//! the reverse does not work. To put the helper in a job object the parent would
//! need `PROCESS_SET_QUOTA | PROCESS_TERMINATE` on it, and Mandatory Integrity
//! Control denies a medium-integrity process write access to a high-integrity
//! one. Reading *down* is always permitted, so the elevated side does the
//! watching.
//!
//! This is what makes "the app crashed" a recoverable situation rather than a
//! machine with no DNS.

use std::sync::mpsc::Sender;

use windows::Win32::Foundation::{CloseHandle, FILETIME, WAIT_OBJECT_0};
use windows::Win32::System::Threading::{
    GetProcessTimes, INFINITE, OpenProcess, PROCESS_ACCESS_RIGHTS,
    PROCESS_QUERY_LIMITED_INFORMATION, WaitForSingleObject,
};

/// Standard `SYNCHRONIZE` right. The `windows` crate only surfaces it typed as
/// a file access right, which does not compose with `PROCESS_ACCESS_RIGHTS`.
const SYNCHRONIZE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(0x0010_0000);

use crate::HelperError;

/// Process creation time as a single integer, for comparing against the value
/// the parent passed on our command line.
pub fn creation_time(pid: u32) -> Result<u64, HelperError> {
    // SAFETY: the handle is closed on both paths; the FILETIME out-params are
    // valid zeroed structs.
    unsafe {
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid)
            .map_err(|e| HelperError::Security(format!("opening process {pid}: {e}")))?;

        let mut created = FILETIME::default();
        let mut exited = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();

        let result = GetProcessTimes(process, &mut created, &mut exited, &mut kernel, &mut user);
        let _ = CloseHandle(process);

        result.map_err(|e| HelperError::Security(format!("reading process times: {e}")))?;
        Ok(to_u64(created))
    }
}

const fn to_u64(time: FILETIME) -> u64 {
    ((time.dwHighDateTime as u64) << 32) | time.dwLowDateTime as u64
}

/// Block until the given process exits, then signal.
///
/// Intended for a dedicated thread. The PID is re-checked against
/// `expected_start` first: PIDs are recycled, and waiting on the wrong process
/// would mean either never reverting or reverting at random.
pub fn watch(pid: u32, expected_start: u64, notify: Sender<()>) -> Result<(), HelperError> {
    let actual = creation_time(pid)?;
    if actual != expected_start {
        return Err(HelperError::Rejected(format!(
            "process {pid} started at {actual}, expected {expected_start} — PID reuse"
        )));
    }

    // SAFETY: handle is closed after the wait returns.
    unsafe {
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION | SYNCHRONIZE, false, pid)
            .map_err(|e| HelperError::Security(format!("opening process {pid} to wait: {e}")))?;

        let reason = WaitForSingleObject(process, INFINITE);
        let _ = CloseHandle(process);

        if reason != WAIT_OBJECT_0 {
            tracing::warn!(?reason, "unexpected wait result while watching the parent");
        }
    }

    tracing::info!(pid, "parent exited");
    let _ = notify.send(());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_time_is_readable_and_stable() {
        let me = std::process::id();
        let first = creation_time(me).expect("own creation time is readable");
        assert!(first > 0);
        assert_eq!(first, creation_time(me).unwrap(), "must not drift");
    }

    #[test]
    fn a_mismatched_start_time_is_rejected() {
        // The guard against PID reuse: same PID, wrong creation time.
        let (tx, _rx) = std::sync::mpsc::channel();
        let err = watch(std::process::id(), 1, tx).expect_err("should reject");
        assert!(matches!(err, HelperError::Rejected(_)), "got {err:?}");
    }
}
