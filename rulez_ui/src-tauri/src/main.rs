// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::{config, debug};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            config::list_config_files,
            config::read_config,
            config::write_config,
            debug::run_debug,
            debug::validate_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
