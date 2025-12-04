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
use commands::ai::AIState;
use tauri::{Manager, RunEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

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
        .on_window_event(|window, event| {
            // Handle window close event - hide instead of close for main window
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    // Prevent the window from closing, just hide it
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
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
            
            // Start clipboard monitoring
            let state_for_clipboard = state.clone();
            tauri::async_runtime::spawn(async move {
                if let Ok(monitor) = state_for_clipboard.clipboard_monitor().await {
                    if let Err(e) = monitor.start().await {
                        tracing::error!("Failed to start clipboard monitor: {}", e);
                    }
                }
            });
            
            app.manage(state);
            
            // Initialize AI state
            app.manage(AIState::new());
            
            // Register global shortcuts
            register_global_shortcuts(app)?;
            
            // 🚀 Show main window on startup (like Alfred)
            if let Some(main_window) = app.get_webview_window("main") {
                // Ensure window is centered
                let _ = main_window.center();
                // Show the window
                let _ = main_window.show();
                // Force focus on the window
                let _ = main_window.set_focus();
                tracing::info!("Main window shown and focused");
            } else {
                tracing::error!("Failed to get main window");
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Search commands
            search::search,
            search::calculate,
            // Clipboard commands
            clipboard::get_clipboard_history,
            clipboard::paste_clipboard_item,
            clipboard::toggle_clipboard_favorite,
            clipboard::delete_clipboard_item,
            clipboard::show_clipboard_window,
            clipboard::hide_clipboard_window,
            // AI commands
            ai::ai_create_conversation,
            ai::ai_get_conversation,
            ai::ai_get_conversations,
            ai::ai_delete_conversation,
            ai::ai_clear_conversations,
            ai::ai_chat,
            ai::ai_chat_stream,
            ai::ai_save_response,
            ai::ai_get_presets,
            ai::ai_add_preset,
            ai::ai_delete_preset,
            ai::ai_get_models,
            ai::get_ai_conversations,
            // Plugin commands
            plugin::get_installed_plugins,
            plugin::get_plugin,
            plugin::install_plugin,
            plugin::uninstall_plugin,
            plugin::enable_plugin,
            plugin::disable_plugin,
            plugin::update_plugin,
            plugin::check_plugin_updates,
            plugin::search_marketplace,
            plugin::get_featured_plugins,
            plugin::grant_plugin_permission,
            plugin::revoke_plugin_permission,
            // Settings commands
            settings::get_config,
            settings::update_config,
            settings::reset_config,
            settings::export_config,
            settings::import_config,
            // System commands
            system::open_path,
            system::open_url,
            system::show_window,
            system::hide_window,
            system::toggle_main_window,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            // Handle macOS Dock icon click (reopen event)
            if let RunEvent::Reopen { has_visible_windows, .. } = event {
                if !has_visible_windows {
                    // Show main window when Dock icon is clicked and no windows visible
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.center();
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        });
}

/// Register global shortcuts for the application
fn register_global_shortcuts(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle().clone();
    
    // Main window shortcut: Cmd+Space (macOS) or Alt+Space (Windows/Linux)
    #[cfg(target_os = "macos")]
    let main_shortcut = Shortcut::new(Some(Modifiers::META), Code::Space);
    #[cfg(not(target_os = "macos"))]
    let main_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);
    
    // Clipboard shortcut: Cmd+Shift+V (macOS) or Ctrl+Shift+V (Windows/Linux)
    #[cfg(target_os = "macos")]
    let clipboard_shortcut = Shortcut::new(Some(Modifiers::META | Modifiers::SHIFT), Code::KeyV);
    #[cfg(not(target_os = "macos"))]
    let clipboard_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);
    
    // Settings shortcut: Cmd+, (macOS) or Ctrl+, (Windows/Linux)
    #[cfg(target_os = "macos")]
    let settings_shortcut = Shortcut::new(Some(Modifiers::META), Code::Comma);
    #[cfg(not(target_os = "macos"))]
    let settings_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::Comma);
    
    let app_handle_main = app_handle.clone();
    let app_handle_clipboard = app_handle.clone();
    let app_handle_settings = app_handle.clone();
    
    // Register main window shortcut
    app.global_shortcut().on_shortcut(main_shortcut, move |_app, _shortcut, _event| {
        toggle_window(&app_handle_main, "main");
    })?;
    
    // Register clipboard shortcut
    app.global_shortcut().on_shortcut(clipboard_shortcut, move |_app, _shortcut, _event| {
        toggle_window(&app_handle_clipboard, "clipboard");
    })?;
    
    // Register settings shortcut
    app.global_shortcut().on_shortcut(settings_shortcut, move |_app, _shortcut, _event| {
        show_settings_window(&app_handle_settings);
    })?;
    
    tracing::info!("Global shortcuts registered successfully");
    Ok(())
}

/// Toggle window visibility
fn toggle_window(app_handle: &tauri::AppHandle, label: &str) {
    if let Some(window) = app_handle.get_webview_window(label) {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
            // Center the window on show
            let _ = window.center();
        }
    }
}

/// Show settings window (always show, never toggle)
/// Also hides the main launcher window to avoid overlap
fn show_settings_window(app_handle: &tauri::AppHandle) {
    // First, hide the main window if visible
    if let Some(main_window) = app_handle.get_webview_window("main") {
        let _ = main_window.hide();
        tracing::debug!("Main window hidden before showing settings");
    }
    
    // Then show the settings window
    if let Some(window) = app_handle.get_webview_window("settings") {
        let _ = window.center();
        let _ = window.show();
        let _ = window.set_focus();
        tracing::info!("Settings window opened");
    }
}
