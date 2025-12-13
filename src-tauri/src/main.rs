// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use omnibox::app;
use omnibox::commands;
use omnibox::core;
use omnibox::platform;
use omnibox::storage;
use omnibox::utils;

#[cfg(windows)]
use omnibox::everything_service;

use app::state::AppState;
use commands::*;
use commands::ai::AIState;
use tauri::{
    Manager,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter,
};
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
            // ═══════════════════════════════════════════════════════════════════
            // 1. Initialize AI state immediately (lightweight, no blocking)
            // ═══════════════════════════════════════════════════════════════════
            app.manage(AIState::new());
            
            // ═══════════════════════════════════════════════════════════════════
            // 2. Setup System Tray (Tauri 2.0 style)
            // ═══════════════════════════════════════════════════════════════════
            setup_system_tray(app)?;
            
            // ═══════════════════════════════════════════════════════════════════
            // 3. Register global shortcuts
            // ═══════════════════════════════════════════════════════════════════
            register_global_shortcuts(app)?;
            
            // ═══════════════════════════════════════════════════════════════════
            // 4. Initialize Everything Service (Windows only)
            // ═══════════════════════════════════════════════════════════════════
            #[cfg(windows)]
            {
                let app_handle = app.handle().clone();
                if let Err(e) = everything_service::init_everything(&app_handle) {
                    tracing::warn!("Everything search not available: {}", e);
                } else {
                    tracing::info!("Everything search initialized successfully");
                }
            }
            
            // ═══════════════════════════════════════════════════════════════════
            // 5. ASYNC: Initialize heavy state in background (DB, Indexer, etc.)
            // ═══════════════════════════════════════════════════════════════════
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match AppState::new(app_handle.clone()).await {
                    Ok(state) => {
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
                        
                        app_handle.manage(state);
                        tracing::info!("AppState initialized successfully");
                    }
                    Err(e) => {
                        tracing::error!("Failed to initialize AppState: {}", e);
                    }
                }
            });
            
            // ⚠️ DO NOT show window here - wait for frontend `app_ready` signal
            tracing::info!("Setup complete, waiting for frontend ready signal");
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Search commands (uses hybrid search: AppIndexer + Everything)
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
            ai::ai_quick_query,
            ai::ai_quick_stop,
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
            system::app_ready,
            // Capture commands
            capture::init_capture,
            capture::save_capture,
            capture::hide_capture_window,
            capture::create_pin_window,
            capture::close_pin_window,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            // Handle app exit event
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                // Allow exit to proceed normally
                api.prevent_exit();
            }
        });
}

// ═══════════════════════════════════════════════════════════════════════════════
// SYSTEM TRAY SETUP (Tauri 2.0)
// ═══════════════════════════════════════════════════════════════════════════════
fn setup_system_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Create tray menu items
    let show_item = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
    let capture_item = MenuItem::with_id(app, "capture", "屏幕截图", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let separator = MenuItem::with_id(app, "separator", "───────────", false, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    
    // Build menu
    let menu = Menu::with_items(app, &[&show_item, &capture_item, &settings_item, &separator, &quit_item])?;
    
    // Get app handle for event handlers
    let app_handle_for_menu = app.handle().clone();
    
    // Load tray icon - use the default window icon from tauri.conf.json
    // In Tauri 2.0, this is the most reliable way to get the bundled icon
    let tray_icon = app.default_window_icon()
        .cloned()
        .ok_or("No default window icon configured in tauri.conf.json")?;
    
    tracing::info!("Loading tray icon from default_window_icon");
    
    // Build tray icon
    let _tray = TrayIconBuilder::new()
        .icon(tray_icon)
        .tooltip("OmniBox - 按 Alt+Space 打开搜索")
        .menu(&menu)
        .on_tray_icon_event(move |tray, event| {
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    // Left click: Toggle main window
                    tracing::debug!("Tray icon left clicked");
                    toggle_window(tray.app_handle(), "main");
                }
                _ => {}
            }
        })
        .on_menu_event(move |_app, event| {
            let id = event.id.as_ref();
            tracing::info!("Tray menu clicked: {}", id);
            
            match id {
                "show" => {
                    tracing::info!("Tray menu: Show clicked");
                    if let Some(window) = app_handle_for_menu.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "capture" => {
                    tracing::info!("Tray menu: Capture clicked");
                    let app_handle = app_handle_for_menu.clone();
                    tauri::async_runtime::spawn(async move {
                        if let Err(e) = capture::init_capture(app_handle).await {
                            tracing::error!("Capture init from tray failed: {e}");
                        }
                    });
                }
                "settings" => {
                    tracing::info!("Tray menu: Settings clicked");
                    show_settings_window(&app_handle_for_menu);
                }
                "quit" => {
                    tracing::info!("Tray menu: Quit clicked - exiting application");
                    // Force exit - this MUST work
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        .build(app)?;
    
    tracing::info!("System tray initialized successfully");
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// GLOBAL SHORTCUTS
// ═══════════════════════════════════════════════════════════════════════════════
fn register_global_shortcuts(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle().clone();
    
    // Main window shortcut: Cmd+Space (macOS) or Alt+Space (Windows/Linux)
    #[cfg(target_os = "macos")]
    let main_shortcut = Shortcut::new(Some(Modifiers::META), Code::Space);
    #[cfg(not(target_os = "macos"))]
    let main_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);
    
    // Clipboard shortcut: Cmd+Shift+V (macOS) or Alt+V (Windows/Linux)
    #[cfg(target_os = "macos")]
    let clipboard_shortcut = Shortcut::new(Some(Modifiers::META | Modifiers::SHIFT), Code::KeyV);
    #[cfg(not(target_os = "macos"))]
    let clipboard_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyV);
    
    // Settings shortcut: Cmd+, (macOS) or Alt+, (Windows/Linux)
    #[cfg(target_os = "macos")]
    let settings_shortcut = Shortcut::new(Some(Modifiers::META), Code::Comma);
    #[cfg(not(target_os = "macos"))]
    let settings_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Comma);
    
    let app_handle_main = app_handle.clone();
    let app_handle_clipboard = app_handle.clone();
    let app_handle_settings = app_handle.clone();
    let app_handle_capture = app_handle.clone();
    
    // Register main window shortcut (Alt+Space)
    app.global_shortcut().on_shortcut(main_shortcut, move |_app, _shortcut, _event| {
        toggle_window(&app_handle_main, "main");
    })?;
    
    // Register clipboard shortcut (Ctrl+Shift+V)
    // This also emits an event to frontend for UI state change
    app.global_shortcut().on_shortcut(clipboard_shortcut, move |_app, _shortcut, _event| {
        tracing::debug!("Clipboard shortcut triggered");
        // Toggle clipboard window
        toggle_window(&app_handle_clipboard, "clipboard");
        // Also emit event to frontend
        let _ = app_handle_clipboard.emit("toggle-clipboard-history", ());
    })?;
    
    // Register settings shortcut (Ctrl+,)
    app.global_shortcut().on_shortcut(settings_shortcut, move |_app, _shortcut, _event| {
        show_settings_window(&app_handle_settings);
    })?;

    // Register capture shortcut: Ctrl+Alt+X (all platforms)
    let capture_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyX);
    app.global_shortcut().on_shortcut(capture_shortcut, move |_app, _shortcut, _event| {
        let app_handle_capture = app_handle_capture.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(e) = capture::init_capture(app_handle_capture).await {
                tracing::error!("Capture init failed: {e}");
            }
        });
    })?;
    
    tracing::info!("Global shortcuts registered: Alt+Space (main), Alt+V (clipboard), Alt+, (settings), Ctrl+Alt+X (capture)");
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// WINDOW MANAGEMENT HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Toggle window visibility with proper sizing for main window
fn toggle_window(app_handle: &tauri::AppHandle, label: &str) {
    if let Some(window) = app_handle.get_webview_window(label) {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            // For main window, ensure correct size before showing
            if label == "main" {
                // Reset to search-bar-only size (will expand when results appear)
                let _ = window.set_size(tauri::LogicalSize::new(680.0, 60.0));
            }
            let _ = window.center();
            let _ = window.show();
            let _ = window.set_focus();
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
