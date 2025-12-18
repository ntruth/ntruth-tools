// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use omnibox::app;
use omnibox::commands;
use omnibox::core;
use omnibox::platform;
use omnibox::storage;
use omnibox::utils;
use omnibox::automation;
use omnibox::ocr;

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
use tauri::path::BaseDirectory;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Default, Clone, Copy)]
struct MainShowState {
    shown_at: Option<Instant>,
    focused_at: Option<Instant>,
}

static MAIN_SHOW_STATE: Lazy<Mutex<MainShowState>> = Lazy::new(|| Mutex::new(MainShowState::default()));
const MAIN_SHOW_BLUR_GRACE_NO_FOCUS: Duration = Duration::from_millis(2000);
const MAIN_FOCUS_TO_BLUR_GRACE: Duration = Duration::from_millis(250);

static LAST_MAIN_SHORTCUT_AT: Lazy<Mutex<Option<Instant>>> = Lazy::new(|| Mutex::new(None));
const MAIN_SHORTCUT_DEBOUNCE: Duration = Duration::from_millis(350);

// Clipboard shortcut debounce to prevent double-trigger
static LAST_CLIPBOARD_SHORTCUT_AT: Lazy<Mutex<Option<Instant>>> = Lazy::new(|| Mutex::new(None));
const CLIPBOARD_SHORTCUT_DEBOUNCE: Duration = Duration::from_millis(400);

// Capture shortcut debounce - more aggressive due to rapid repeats observed
static LAST_CAPTURE_SHORTCUT_AT: Lazy<Mutex<Option<Instant>>> = Lazy::new(|| Mutex::new(None));
const CAPTURE_SHORTCUT_DEBOUNCE: Duration = Duration::from_millis(500);

fn main() {
    // Initialize logger
    utils::logger::init_simple_logger();

    fn launcher_autohide_enabled() -> bool {
        // Default ON (both dev & release) to match launcher UX.
        // Disable via: OMNIBOX_AUTOHIDE_ON_BLUR=0
        let v = std::env::var("OMNIBOX_AUTOHIDE_ON_BLUR").ok();
        let explicit = v.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty());

        match explicit {
            Some("0") | Some("false") | Some("FALSE") | Some("no") | Some("NO") => false,
            _ => true,
        }
    }

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
                    return;
                }
            }

            // Auto-hide launcher (main window only) when it loses focus.
            // DO NOT apply to clipboard or other windows!
            if let tauri::WindowEvent::Focused(focused) = event {
                let label = window.label();
                if label != "main" || !launcher_autohide_enabled() {
                    return;
                }

                if *focused {
                    if let Ok(mut st) = MAIN_SHOW_STATE.lock() {
                        st.focused_at = Some(Instant::now());
                    }
                    return;
                }

                // Focus lost.
                // On Windows (especially with transparent windows), focus can flap during show or
                // during programmatic resize. We only auto-hide once the window has actually been
                // focused at least once since it was shown.
                if let Ok(st) = MAIN_SHOW_STATE.lock() {
                    if let Some(shown_at) = st.shown_at {
                        if st.focused_at.is_none() && shown_at.elapsed() < MAIN_SHOW_BLUR_GRACE_NO_FOCUS {
                            // Never actually focused: ignore this transient blur.
                            return;
                        }
                        if let Some(focused_at) = st.focused_at {
                            if focused_at.elapsed() < MAIN_FOCUS_TO_BLUR_GRACE {
                                // Just focused then immediately blurred: likely transient.
                                return;
                            }
                        }
                    }
                }

                let _ = window.hide();
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

                    // If the DLL is missing (or couldn't be loaded), remind the user what to install/copy.
                    // Keep it non-fatal so the rest of the app still works.
                    let e_lower = e.to_lowercase();
                    if e_lower.contains("everything64.dll") {
                        let message = "未能加载 Everything 搜索组件（Everything64.dll）。\n\n请将 Everything64.dll 放到程序目录（与 OmniBox.exe 同级），然后重启 OmniBox。\n\n提示：开发环境也可放在 src-tauri/libs/。";
                        app_handle
                            .dialog()
                            .message(message)
                            .title("Everything 未就绪")
                            .show(|_| {});
                    }
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
            capture::capture_frontend_ready,
            capture::is_capture_ready,
            capture::save_capture,
            capture::save_capture_file,
            capture::copy_capture_base64,
            capture::hide_capture_window,
            capture::create_pin_window,
            capture::create_pin_window_from_selection,
            capture::close_pin_window,
            capture::get_pin_payload,

            // OCR (Windows native via WinRT)
            ocr::recognize_text,

            // UI Automation
            automation::get_element_rect_at,
            automation::get_element_info_at,
            automation::get_element_rects_batch,
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
    
    // Load tray icon.
    // Requirement: use tools.png as the tray/status-bar icon, but prefer tray-optimized sizes.
    // Order is platform + DPI aware, with safe fallbacks.
    let scale_factor = app
        .get_webview_window("main")
        .and_then(|w| w.scale_factor().ok())
        .unwrap_or(1.0);

    // Most trays are effectively 16px @ 1.0 scale, 32px @ 2.0 scale.
    let prefer_32 = scale_factor >= 1.5;

    // macOS status bar tends to look better with a larger source image (it will be downscaled).
    let prefer_32 = if cfg!(target_os = "macos") { true } else { prefer_32 };

    let mut candidates: Vec<&str> = Vec::new();
    if prefer_32 {
        candidates.push("icons/tools-32.png");
        candidates.push("icons/tools-16.png");
    } else {
        candidates.push("icons/tools-16.png");
        candidates.push("icons/tools-32.png");
    }
    candidates.push("icons/tools.png");
    candidates.push("icons/icon.ico");

    let tray_icon = candidates
        .into_iter()
        .find_map(|rel| {
            app.path()
                .resolve(rel, BaseDirectory::Resource)
                .ok()
                .and_then(|p| tauri::image::Image::from_path(p).ok())
                .map(|i| i.to_owned())
        })
        .or_else(|| app.default_window_icon().cloned())
        .ok_or("No tray icon available (missing bundled icon)")?;
    
    tracing::info!("Tray icon loaded");
    
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
    
    // Clipboard shortcut: Cmd+Shift+V (macOS) or Ctrl+Alt+V (Windows/Linux)
    #[cfg(target_os = "macos")]
    let clipboard_shortcut = Shortcut::new(Some(Modifiers::META | Modifiers::SHIFT), Code::KeyV);
    #[cfg(not(target_os = "macos"))]
    let clipboard_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyV);
    
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
        // On Windows, the hotkey can sometimes fire twice (repeat / key state quirks).
        // The launcher UX expects: hotkey always SHOWS (tray can toggle).
        if let Ok(mut last) = LAST_MAIN_SHORTCUT_AT.lock() {
            if let Some(t0) = *last {
                if t0.elapsed() < MAIN_SHORTCUT_DEBOUNCE {
                    return;
                }
            }
            *last = Some(Instant::now());
        }

        show_window(&app_handle_main, "main");
    })?;
    
    // Register clipboard shortcut (Ctrl+Alt+V)
    // This also emits an event to frontend for UI state change
    app.global_shortcut().on_shortcut(clipboard_shortcut, move |_app, _shortcut, _event| {
        // Debounce: Windows hotkey can fire twice rapidly
        if let Ok(mut last) = LAST_CLIPBOARD_SHORTCUT_AT.lock() {
            if let Some(t0) = *last {
                if t0.elapsed() < CLIPBOARD_SHORTCUT_DEBOUNCE {
                    tracing::debug!("Clipboard shortcut debounced");
                    return;
                }
            }
            *last = Some(Instant::now());
        }
        
        tracing::info!("Clipboard shortcut triggered");
        // Toggle clipboard window
        toggle_window(&app_handle_clipboard, "clipboard");
        // Also emit event to frontend
        let _ = app_handle_clipboard.emit("toggle-clipboard-history", ());
    })?;
    
    // Register settings shortcut (Ctrl+,)
    app.global_shortcut().on_shortcut(settings_shortcut, move |_app, _shortcut, _event| {
        show_settings_window(&app_handle_settings);
    })?;

    // Register multiple capture shortcuts for robustness (some combos may be occupied by system/other apps)
    let capture_shortcuts = vec![
        ("PrintScreen", Shortcut::new(None, Code::PrintScreen)),
        ("Ctrl+Alt+X", Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyX)),
        ("Ctrl+Shift+S", Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyS)),
    ];

    let mut capture_registered = false;
    for (label, sc) in capture_shortcuts {
        let app_handle_capture = app_handle_capture.clone();
        let label_owned = label.to_string();
        match app.global_shortcut().on_shortcut(sc, move |_app, _shortcut, _event| {
            // Aggressive debounce: Windows can fire global hotkeys many times per second
            if let Ok(mut last) = LAST_CAPTURE_SHORTCUT_AT.lock() {
                if let Some(t0) = *last {
                    if t0.elapsed() < CAPTURE_SHORTCUT_DEBOUNCE {
                        tracing::trace!("Capture shortcut debounced ({})", label_owned);
                        return;
                    }
                }
                *last = Some(Instant::now());
            }
            
            tracing::info!("Capture shortcut triggered ({})", label_owned);
            let app_handle_capture = app_handle_capture.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = capture::init_capture(app_handle_capture).await {
                    tracing::error!("Capture init failed: {e}");
                }
            });
        }) {
            Ok(_) => {
                capture_registered = true;
                tracing::info!("Capture shortcut registered: {}", label);
            }
            Err(e) => {
                tracing::warn!("Failed to register capture shortcut {}: {}", label, e);
            }
        }
    }
    if !capture_registered {
        tracing::error!("No capture shortcuts registered. Check for OS/global shortcut conflicts.");
    }
    
    tracing::info!("Global shortcuts registered: Alt+Space (main), Ctrl+Alt+V (clipboard), Alt+, (settings), Ctrl+Alt+X (capture)");

    // Warm up capture webview so the first hotkey can show immediately.
    {
        let app_handle = app.handle().clone();
        tauri::async_runtime::spawn(async move {
            omnibox::commands::capture::warmup_capture_window(&app_handle).await;
        });
    }
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// WINDOW MANAGEMENT HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Show window (no toggle) with proper sizing/focus for main window.
/// This is used for global shortcuts to avoid double-trigger toggling it back off.
fn show_window(app_handle: &tauri::AppHandle, label: &str) {
    if let Some(window) = app_handle.get_webview_window(label) {
        // For main window, ensure correct size before showing
        if label == "main" {
            let _ = window.set_size(tauri::LogicalSize::new(680.0, 60.0));
            if let Ok(mut st) = MAIN_SHOW_STATE.lock() {
                *st = MainShowState {
                    shown_at: Some(Instant::now()),
                    focused_at: None,
                };
            }
        }

        if label != "main" {
            let _ = window.center();
        }

        let _ = window.show();
        let _ = window.set_focus();

        if label == "main" {
            let win = window.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(50));
                let _ = win.set_focus();
                std::thread::sleep(Duration::from_millis(150));
                let _ = win.set_focus();
                std::thread::sleep(Duration::from_millis(300));
                let _ = win.set_focus();
            });
        }

        if label == "clipboard" {
            let win = window.clone();
            std::thread::spawn(move || {
                // Multiple focus attempts to combat Windows transparent window focus issues
                std::thread::sleep(Duration::from_millis(30));
                let _ = win.set_focus();
                std::thread::sleep(Duration::from_millis(80));
                let _ = win.set_focus();
                std::thread::sleep(Duration::from_millis(150));
                let _ = win.set_focus();
            });
        }
    }
}

/// Toggle window visibility with proper sizing for main window
fn toggle_window(app_handle: &tauri::AppHandle, label: &str) {
    if let Some(window) = app_handle.get_webview_window(label) {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
            if label == "main" {
                if let Ok(mut st) = MAIN_SHOW_STATE.lock() {
                    *st = MainShowState::default();
                }
            }
        } else {
            show_window(app_handle, label);
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
