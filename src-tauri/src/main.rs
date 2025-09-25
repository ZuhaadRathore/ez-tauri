#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Entry point for the Tauri application.
fn main() {
    ez_tauri_lib::run()
}
