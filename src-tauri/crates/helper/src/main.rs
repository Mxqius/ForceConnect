//! Thin entry point. Everything testable lives in the library — see `lib.rs`
//! for why.

fn main() -> std::process::ExitCode {
    forceconnect_helper::run()
}
