//! ForceConnect — application entry point and Tauri wiring.

mod commands;

use tauri_specta::{Builder, collect_commands};

/// Assemble the typed command surface.
///
/// Kept in its own function so `build.rs`-adjacent tooling and the test below
/// can both reach it. The generated TypeScript is what the frontend imports,
/// which means a renamed command or a changed argument breaks the frontend
/// build rather than failing at runtime.
fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![
        commands::app::app_info,
        commands::adapters::list_adapters,
        commands::servers::list_servers,
        commands::servers::list_protocols,
    ])
}

/// Where the generated bindings live.
///
/// Absolute, resolved from the manifest directory at compile time. A path
/// relative to the working directory would land somewhere different depending
/// on whether the exporter was run from `src-tauri` or from the repository
/// root — and would silently create the file outside the project in the second
/// case.
pub const BINDINGS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/lib/bindings.ts");

/// Write the TypeScript bindings without starting the app.
///
/// Exposed so CI can generate bindings headlessly and then typecheck the
/// frontend against them — see `src/bin/export-bindings.rs`. Without this the
/// only way to produce the file would be to launch the GUI.
pub fn export_bindings(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use specta_typescript::Typescript;

    // Bigints are deliberately *not* cast to `number`. Interface LUIDs are
    // genuine u64 values that can exceed 2^53, and silently losing precision on
    // an adapter identifier would mean writing DNS to the wrong adapter.
    // Anything u64-shaped crosses the boundary as a string instead.
    specta_builder().export(Typescript::new(), path)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    // Regenerated on every debug run so the bindings can never drift from the
    // Rust signatures. The file is gitignored for the same reason.
    #[cfg(debug_assertions)]
    export_bindings(BINDINGS_PATH).expect("failed to export TypeScript bindings");

    let mut app = tauri::Builder::default();

    // Must be registered before anything that touches shared state: a second
    // instance racing the first over port 53 and the system DNS setting is not
    // a recoverable situation.
    #[cfg(desktop)]
    {
        app = app.plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            use tauri::Manager as _;
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }));
    }

    app = app
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(if cfg!(debug_assertions) {
                    log::LevelFilter::Debug
                } else {
                    log::LevelFilter::Info
                })
                .build(),
        );

    #[cfg(desktop)]
    {
        app = app
            .plugin(tauri_plugin_window_state::Builder::default().build())
            .plugin(tauri_plugin_autostart::init(
                tauri_plugin_autostart::MacosLauncher::LaunchAgent,
                None,
            ));
    }

    // The updater is deliberately not registered yet. It refuses to initialize
    // without a `plugins.updater.pubkey`, and that keypair has to be generated
    // by whoever owns the release process — the private key belongs in CI
    // secrets, never in this repository. It gets wired up alongside the release
    // pipeline. A placeholder key would be worse than none: the plugin would
    // look configured while silently rejecting every update.

    app.invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running ForceConnect");
}

// No `#[cfg(test)]` module here on purpose. A test harness linked against the
// Tauri app crate fails to load on Windows with STATUS_ENTRYPOINT_NOT_FOUND
// before any test runs. That is one more reason this crate stays a thin shell:
// everything worth testing lives in `crates/`, which tests normally.
//
// The command surface is still verified on every debug run — `run()` exports
// the TypeScript bindings, and `tsc` then typechecks the frontend against
// them, so a renamed command or changed argument breaks the build.
