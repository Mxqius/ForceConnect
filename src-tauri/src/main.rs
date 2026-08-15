// Keeps a console window from appearing alongside the app in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    forceconnect_lib::run()
}
