//! Generate `src/lib/bindings.ts` without starting the app.
//!
//! An example rather than a second binary: the bundler ships every binary in
//! the package, and this is a developer tool, not something to install.
//!
//! `pnpm tauri dev` regenerates the bindings as a side effect of launching, but
//! CI has no display and should not need one to typecheck the frontend. Run
//! this first, then `tsc`.

fn main() -> std::process::ExitCode {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| forceconnect_lib::BINDINGS_PATH.to_owned());

    match forceconnect_lib::export_bindings(&path) {
        Ok(()) => {
            println!("wrote {path}");
            std::process::ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("failed to export bindings: {err}");
            std::process::ExitCode::FAILURE
        }
    }
}
