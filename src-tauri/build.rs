fn main() {
    // ForceConnect writes DNS settings and flushes the resolver cache, both of
    // which need administrator rights. It asks for them the only way Windows
    // offers a plain desktop app: a `requireAdministrator` manifest, so the
    // consent prompt happens once at launch instead of interrupting the user's
    // first connect.
    //
    // That manifest goes on the `forceconnect` binary *specifically*, not on
    // every binary the package builds. `export-bindings` is a headless
    // developer tool CI runs; with `requireAdministrator` attached it fails to
    // start with ERROR_ELEVATION_REQUIRED (740) in any non-interactive shell.
    //
    // Which is why tauri-build is asked for `new_without_app_manifest`: it
    // embeds an `asInvoker` manifest into every binary by default, and two
    // manifests in one binary is a link error. Replacing it means carrying the
    // parts it provided — Common-Controls v6, per-monitor DPI awareness, the
    // supportedOS entries — into both manifest files here.
    let mut attributes = tauri_build::Attributes::new();

    #[cfg(windows)]
    {
        attributes = attributes
            .windows_attributes(tauri_build::WindowsAttributes::new_without_app_manifest());

        embed_manifest("forceconnect", "forceconnect.manifest");
        embed_manifest("export-bindings", "export-bindings.manifest");
    }

    tauri_build::try_build(attributes).expect("failed to run the Tauri build script");
}

/// Ask the MSVC linker to embed a manifest into one named binary.
///
/// `/MANIFEST:EMBED` plus `/MANIFESTINPUT:` is the documented pair; either
/// alone does nothing useful. `rustc-link-arg-bin` is what scopes it to a
/// single binary, and is the reason this is done by hand rather than through
/// tauri-build, which has no per-binary option.
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
#[cfg(windows)]
fn embed_manifest(binary: &str, manifest: &str) {
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

    println!("cargo:rustc-link-arg-bin={binary}=/MANIFEST:EMBED");
    println!("cargo:rustc-link-arg-bin={binary}=/MANIFESTUAC:NO");
    println!(
        "cargo:rustc-link-arg-bin={binary}=/MANIFESTINPUT:{}",
        path.display()
    );
}
