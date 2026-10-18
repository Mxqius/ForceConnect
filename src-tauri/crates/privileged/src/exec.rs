//! Running the system's own DNS tools, on Linux and macOS.
//!
//! The same discipline as everywhere else in this crate: callers pass a fixed
//! program and arguments built from validated values — an IP address, an
//! interface or service name read back from live system state — never a
//! string that reaches a shell. `Command` execs the program directly, so there
//! is no shell to inject into.

use std::path::PathBuf;
use std::process::Command;

/// Where the tools are looked for. Not `$PATH`: this runs as root, and a
/// `$PATH` inherited from the user's session is theirs to point anywhere.
const SYSTEM_DIRS: [&str; 4] = ["/usr/bin", "/bin", "/usr/sbin", "/sbin"];

pub fn locate(program: &str) -> Option<PathBuf> {
    SYSTEM_DIRS
        .iter()
        .map(|dir| PathBuf::from(dir).join(program))
        .find(|path| path.is_file())
}

/// Run a tool to completion and return its standard output.
pub fn run(program: &str, args: &[&str]) -> Result<String, String> {
    let path = locate(program).ok_or_else(|| format!("{program} was not found"))?;

    let output = Command::new(path)
        .args(args)
        .env("LC_ALL", "C")
        .output()
        .map_err(|err| format!("could not run {program}: {err}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    if output.status.success() {
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = if stderr.trim().is_empty() {
            stdout.trim()
        } else {
            stderr.trim()
        };
        Err(format!("{program} failed ({}): {detail}", output.status))
    }
}
