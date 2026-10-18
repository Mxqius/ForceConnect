fn main() {
    // ForceConnect writes DNS settings and flushes the resolver cache, both of
    // which need administrator rights. On Windows it asks for them the only way
    // Windows offers a plain desktop app: a `requireAdministrator` manifest, so
    // the consent prompt happens once at launch instead of interrupting the
    // user's first connect. (Linux and macOS have no such thing; there the app
    // starts a privileged agent at launch instead — nothing to do at build
    // time.)
    //
    // That manifest goes on the `forceconnect` binary *specifically*, not on
    // everything the package links. `export-bindings` is a headless developer
    // tool CI runs; with `requireAdministrator` attached it fails to start
    // with ERROR_ELEVATION_REQUIRED (740) in any non-interactive shell.
    //
    // The exporter is a Cargo *example*, not a second `[[bin]]`, because the
    // bundler ships every binary in the package: as a bin it was installed
    // next to the app on every platform, and on a universal macOS build —
    // where only the main binary is merged across architectures — it broke
    // bundling outright.
    //
    // Which is why tauri-build is asked for `new_without_app_manifest`: it
    // embeds an `asInvoker` manifest into every binary by default, and two
    // manifests in one binary is a link error. Replacing it means carrying the
    // parts it provided — Common-Controls v6, per-monitor DPI awareness, the
    // supportedOS entries — into both manifest files here.
    //
    // The *target* OS, not `cfg(windows)`: a build script is compiled for the
    // machine running the build, so `cfg` here describes the host, and a
    // cross-compile from Windows would otherwise try to embed a Windows
    // manifest into a macOS binary.
    let mut attributes = tauri_build::Attributes::new();

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        attributes = attributes
            .windows_attributes(tauri_build::WindowsAttributes::new_without_app_manifest());

        embed_manifest("bin=forceconnect", "forceconnect.manifest");
        // The exporter is the package's only example, so the examples-wide
        // flag reaches exactly it.
        embed_manifest("examples", "export-bindings.manifest");
    }

    tauri_build::try_build(attributes).expect("failed to run the Tauri build script");
}

/// Ask the MSVC linker to embed a manifest into one link target.
///
/// `scope` completes the `cargo:rustc-link-arg-…` directive: `bin=NAME` for one
/// binary, `examples` for the examples. `/MANIFEST:EMBED` plus
/// `/MANIFESTINPUT:` is the documented pair; either alone does nothing useful.
/// Scoping the flags per target is the reason this is done by hand rather than
/// through tauri-build, which has no per-binary option.
///
/// `/MANIFESTUAC:NO` is not optional. With `/MANIFEST:EMBED` the linker
/// contributes a `requestedExecutionLevel` of its own — `asInvoker` — and
/// `mt.exe` then refuses to merge two snippets that disagree:
///
/// ```text
/// manifest authoring error c1010001: Values of attribute "level" not equal
/// ```
///
/// Suppressing the linker's fragment leaves the file's as the only one.
fn embed_manifest(scope: &str, manifest: &str) {
    println!("cargo:rerun-if-changed={manifest}");

    // The GNU toolchain takes a manifest as a compiled resource, not as a
    // linker flag. Tauri on Windows builds with MSVC; saying so is better than
    // emitting flags that would fail the link with a confusing message.
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() != Ok("msvc") {
        println!(
            "cargo:warning=not an MSVC target: {manifest} was not embedded, so this build will \
             run unelevated and every DNS action will refuse"
        );
        return;
    }

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(manifest);

    println!("cargo:rustc-link-arg-{scope}=/MANIFEST:EMBED");
    println!("cargo:rustc-link-arg-{scope}=/MANIFESTUAC:NO");
    println!(
        "cargo:rustc-link-arg-{scope}=/MANIFESTINPUT:{}",
        path.display()
    );
}
