//! Flushing the system resolver cache.
//!
//! It matters after a DNS change, because entries resolved through the
//! previous server stay valid for their TTL — which is exactly the window in
//! which a user switches server, sees the same blocked or stale answer, and
//! concludes the app did nothing.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::flush;
#[cfg(target_os = "macos")]
pub use macos::flush;
#[cfg(windows)]
pub use windows::flush;
