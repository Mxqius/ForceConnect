//! Flushing resolver caches on Linux.
//!
//! glibc does not cache lookups itself, so there is only a cache to flush if
//! something on the machine provides one: systemd-resolved on most desktops,
//! `nscd` on some older or enterprise installs. Each one present is flushed; a
//! machine with neither has nothing cached, and that is not an error.

use std::path::Path;

use crate::exec;

pub fn flush() -> Result<(), String> {
    if Path::new("/run/systemd/resolve").is_dir() && exec::locate("resolvectl").is_some() {
        exec::run("resolvectl", &["flush-caches"])?;
    }

    if exec::locate("nscd").is_some() {
        // Best-effort: nscd is commonly installed and not running, which is
        // the same as having no cache.
        if let Err(err) = exec::run("nscd", &["--invalidate=hosts"]) {
            tracing::debug!("nscd did not flush: {err}");
        }
    }

    Ok(())
}
