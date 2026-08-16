//! Flushing the system resolver cache.
//!
//! This is what `ipconfig /flushdns` does underneath: a single call into
//! `dnsapi.dll` that tells the DNS Client service to drop every cached record.
//! It matters after a DNS change, because entries resolved through the previous
//! server stay valid for their TTL — which is exactly the window in which a user
//! switches server, sees the same blocked or stale answer, and concludes the app
//! did nothing.
//!
//! # Why `GetProcAddress` rather than a link-time import
//!
//! `DnsFlushResolverCache` is exported by `dnsapi.dll` but absent from the
//! Windows SDK headers, so the `windows` crate has no binding for it. Resolving
//! it at runtime keeps that fact contained here, and means a Windows build that
//! somehow lacks the export fails with a clear message instead of refusing to
//! start the process at all.
//!
//! Spawning `ipconfig.exe` would be the other option and is rejected for the
//! same reasons as `netsh` elsewhere in this crate: a process spawn from an
//! elevated process, no structured errors, and a pattern endpoint security
//! tooling increasingly flags.

use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::core::s;

/// `BOOL` spelled as its underlying `i32`: the export has no SDK header, so
/// there is no binding to borrow the type alias from.
type FlushFn = unsafe extern "system" fn() -> i32;

/// Drop every entry in the resolver cache.
///
/// Requires elevation, which is why it lives in this crate: the call is an RPC
/// into the `Dnscache` service, and the service rejects it from an unelevated
/// caller.
pub fn flush() -> Result<(), String> {
    // SAFETY: a constant, NUL-terminated module name; the handle is not freed,
    // since `dnsapi.dll` stays loaded for the (short) life of this process.
    let module = unsafe { LoadLibraryA(s!("dnsapi.dll")) }
        .map_err(|err| format!("loading dnsapi.dll: {err}"))?;

    // SAFETY: `module` is a live handle from the call above.
    let Some(symbol) = (unsafe { GetProcAddress(module, s!("DnsFlushResolverCache")) }) else {
        return Err("dnsapi.dll does not export DnsFlushResolverCache".into());
    };

    // SAFETY: the export takes no arguments and returns BOOL — the signature
    // `ipconfig` has called for two decades. Transmuting the raw address to
    // that signature is the documented use of a `GetProcAddress` result.
    let flush: FlushFn = unsafe { std::mem::transmute(symbol) };

    // SAFETY: no arguments, no buffers, nothing to outlive the call.
    if unsafe { flush() } != 0 {
        Ok(())
    } else {
        Err("the DNS Client service refused to flush its cache".into())
    }
}
