// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod commands;
mod core;
mod platform;
mod storage;
mod utils;

use app::state::AppState;
use commands::*;
use tauri::Manager;

fn main() {
    // Initialize logger
    utils::logger::init_simple_logger();

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize app state (now async)
            let app_handle = app.handle().clone();
            let state = tauri::async_runtime::block_on(async move {
                AppState::new(app_handle).await
            })?;
            
            // Start background indexing task
            let state_clone = state.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = state_clone.initialize_indexing().await {
                    tracing::error!("Failed to initialize indexing: {}", e);
                }
            });
            
            app.manage(state);
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Search commands
            search::search,
            search::calculate,
            // Clipboard commands
            clipboard::get_clipboard_history,
            clipboard::paste_clipboard_item,
            // AI commands
            ai::ai_chat,
            ai::get_ai_conversations,
            // Settings commands
            settings::get_config,
            settings::update_config,
            settings::reset_config,
            settings::export_config,
            settings::import_config,
            // System commands
            system::open_path,
            system::show_window,
            system::hide_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
