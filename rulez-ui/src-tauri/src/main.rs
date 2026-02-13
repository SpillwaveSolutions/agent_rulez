// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::{config, debug, logs};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            config::list_config_files,
            config::read_config,
            config::write_config,
            debug::run_debug,
            debug::validate_config,
            debug::check_binary,
            logs::read_logs,
            logs::get_log_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
