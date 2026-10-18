//! The main window's two sizes: the full panel and the compact one.
//!
//! One window that changes size rather than a second window. A second webview
//! would be a second renderer process — exactly the cost compact mode exists
//! to avoid — and two windows sharing one connection store would need a
//! cross-window sync for state that currently lives in a single place.

use tauri::{AppHandle, LogicalSize, Manager as _, PhysicalPosition, Runtime, WebviewWindow};

/// Every full-size screen is laid out for exactly this.
const FULL: LogicalSize<f64> = LogicalSize::new(1000.0, 700.0);
/// The compact panel: status, power button, server, latency.
const COMPACT: LogicalSize<f64> = LogicalSize::new(360.0, 450.0);

/// Switch the main window between full and compact.
///
/// Order matters because the window enforces a minimum size: shrinking lowers
/// the floor before the size, growing raises the size before the floor.
pub fn apply_mode<R: Runtime>(
    window: &WebviewWindow<R>,
    compact: bool,
    always_on_top: bool,
) -> tauri::Result<()> {
    if compact {
        window.set_min_size(Some(COMPACT))?;
        window.set_size(COMPACT)?;
    } else {
        window.set_size(FULL)?;
        window.set_min_size(Some(FULL))?;
        // A compact window parked in a screen corner would otherwise grow off
        // the edge of the display.
        keep_on_screen(window);
    }

    window.set_always_on_top(compact && always_on_top)?;
    set_memory_target(window, compact);
    Ok(())
}

/// Apply the saved mode before the user sees the window.
///
/// The preference is the frontend's, in the same store file as the rest of
/// them, but waiting for the frontend to hydrate and resize would show the
/// full layout squeezed into a compact window, or the reverse, for a frame.
/// Reading the file here is a plain read of a JSON document, not a second
/// owner of it: nothing in Rust ever writes it.
pub fn restore_saved_mode<R: Runtime>(app: &AppHandle<R>) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    let prefs = app
        .path()
        .app_data_dir()
        .ok()
        .and_then(|dir| std::fs::read(dir.join("preferences.json")).ok())
        .and_then(|body| serde_json::from_slice::<serde_json::Value>(&body).ok());

    let flag = |key: &str| {
        prefs
            .as_ref()
            .and_then(|p| p.get(key))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };

    if flag("compact")
        && let Err(err) = apply_mode(&window, true, flag("compactOnTop"))
    {
        log::warn!("could not restore compact mode: {err}");
    }
}

/// Nudge the window back inside the work area of the monitor it is on.
fn keep_on_screen<R: Runtime>(window: &WebviewWindow<R>) {
    let (Ok(Some(monitor)), Ok(position), Ok(size)) = (
        window.current_monitor(),
        window.outer_position(),
        window.outer_size(),
    ) else {
        return;
    };

    let area = monitor.work_area();
    let max_x = area.position.x + area.size.width as i32 - size.width as i32;
    let max_y = area.position.y + area.size.height as i32 - size.height as i32;
    let x = position.x.min(max_x).max(area.position.x);
    let y = position.y.min(max_y).max(area.position.y);

    if (x, y) != (position.x, position.y) {
        let _ = window.set_position(PhysicalPosition::new(x, y));
    }
}

/// Ask WebView2 to keep its footprint small while the window is compact.
///
/// `MemoryUsageTargetLevel::Low` makes the renderer trim its caches and
/// working set aggressively. Worth it for a panel that shows a handful of
/// values; not for the full window, where it would cost smoothness on screens
/// that animate. Best-effort: a runtime older than the interface simply keeps
/// its normal behaviour.
///
/// This is the one direct platform call in the shell crate. It acts on this
/// process's own webview, not on the system, so it does not belong in
/// `forceconnect-privileged`, and it needs Tauri's webview handle, which keeps
/// it out of the other crates too.
#[cfg(windows)]
fn set_memory_target<R: Runtime>(window: &WebviewWindow<R>, low: bool) {
    use webview2_com::Microsoft::Web::WebView2::Win32::{
        COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_LOW, COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_NORMAL,
        ICoreWebView2_19,
    };
    use windows_core::Interface as _;

    let level = if low {
        COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_LOW
    } else {
        COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_NORMAL
    };

    let _ = window.with_webview(move |webview| {
        // SAFETY: the controller is live for the duration of this callback,
        // and both calls are plain COM property accessors on it.
        unsafe {
            let Ok(core) = webview.controller().CoreWebView2() else {
                return;
            };
            if let Ok(core) = core.cast::<ICoreWebView2_19>() {
                let _ = core.SetMemoryUsageTargetLevel(level);
            }
        }
    });
}

#[cfg(not(windows))]
fn set_memory_target<R: Runtime>(_window: &WebviewWindow<R>, _low: bool) {}
