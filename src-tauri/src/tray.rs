//! The tray icon: built once in `setup`, then kept live from
//! `commands::app::set_tray_status` as the frontend's connection state
//! changes.
//!
//! The icon itself carries no state of its own — "what server, if any" lives
//! in the frontend's connection store, same as it always has — so every
//! update here is the frontend pushing a snapshot across, not the tray
//! deriving anything.

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, WindowEvent};

use crate::commands::app::TrayStatus;
use crate::state::AppState;

/// Emitted at the frontend when "Disconnect" is chosen from the tray menu.
///
/// The tray has no adapter handle of its own to disconnect with — only the
/// frontend's connection store knows which adapter is live — so the menu
/// click is relayed as an event rather than acted on here.
const DISCONNECT_EVENT: &str = "tray://disconnect";

/// The pieces of the tray that change after it is built.
struct TrayHandles {
    tray: TrayIcon<tauri::Wry>,
    status_item: MenuItem<tauri::Wry>,
    disconnect_item: MenuItem<tauri::Wry>,
}

/// Build the tray icon and wire the main window's close button to hide it
/// there instead of exiting, when the user has opted into that.
///
/// Minimizing to tray never touches DNS: only "Quit" (or closing the window
/// with the preference off) reaches `RunEvent::Exit` and restores it.
pub fn setup(app: &AppHandle) -> tauri::Result<()> {
    let status_item = MenuItem::with_id(app, "status", "Not connected", false, None::<&str>)?;
    let show_item = MenuItem::with_id(app, "show", "Show ForceConnect", true, None::<&str>)?;
    let disconnect_item =
        MenuItem::with_id(app, "disconnect", "Disconnect", false, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &status_item,
            &PredefinedMenuItem::separator(app)?,
            &show_item,
            &disconnect_item,
            &PredefinedMenuItem::separator(app)?,
            &quit_item,
        ],
    )?;

    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().cloned().expect("app icon"))
        .tooltip("ForceConnect · Not connected")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => show_main_window(app),
            "disconnect" => {
                let _ = app.emit(DISCONNECT_EVENT, ());
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main_window(tray.app_handle());
            }
        })
        .build(app)?;

    app.manage(TrayHandles {
        tray,
        status_item,
        disconnect_item,
    });

    if let Some(window) = app.get_webview_window("main") {
        let app_handle = app.clone();
        window.on_window_event(move |event| {
            if let WindowEvent::CloseRequested { api, .. } = event
                && let Some(state) = app_handle.try_state::<AppState>()
                && state.minimize_to_tray()
            {
                api.prevent_close();
                show_main_window_hidden(&app_handle);
            }
        });
    }

    Ok(())
}

fn show_main_window_hidden(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}

pub fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

/// Reflect the frontend's connection state in the tray tooltip and menu.
///
/// Called from `set_tray_status` on every connect/disconnect, so the tray
/// always shows what the app itself last knew — not a guess made by polling
/// adapter state, which cannot tell "connected to X" from "DNS happens to be
/// static".
pub fn update_status(app: &AppHandle, status: &TrayStatus) {
    let Some(handles) = app.try_state::<TrayHandles>() else {
        return;
    };

    let label = status_label(status);
    let _ = handles.status_item.set_text(&label);
    let _ = handles.disconnect_item.set_enabled(status.connected);
    let _ = handles
        .tray
        .set_tooltip(Some(format!("ForceConnect · {label}")));
}

fn status_label(status: &TrayStatus) -> String {
    if !status.connected {
        return "Not connected".to_owned();
    }

    match (&status.server_name, &status.protocol_label) {
        (Some(server), Some(protocol)) => format!("Connected · {server} ({protocol})"),
        (Some(server), None) => format!("Connected · {server}"),
        _ => "Connected".to_owned(),
    }
}
