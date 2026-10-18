//! Flushing the resolver cache on macOS.
//!
//! Two caches, two calls — Apple's own documented remedy. `dscacheutil` clears
//! the Directory Services cache, and `mDNSResponder`, which does the actual
//! resolving and caching, drops its cache on `SIGHUP`. The second is the one
//! that needs root: `mDNSResponder` runs as its own user.

use crate::exec;

pub fn flush() -> Result<(), String> {
    exec::run("dscacheutil", &["-flushcache"])?;
    exec::run("killall", &["-HUP", "mDNSResponder"])?;
    Ok(())
}
