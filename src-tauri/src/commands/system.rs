use crate::app::{error::AppResult, state::AppState};
use tauri::{Manager, State};
use tauri_plugin_shell::ShellExt;

/// Open path in system file manager or default application
#[tauri::command]
pub async fn open_path(path: String, state: State<'_, AppState>) -> AppResult<()> {
    tracing::info!("Opening path: {}", path);
    
    // Use tauri-plugin-shell to open the path
    let shell = state.app_handle().shell();
    
    #[cfg(target_os = "macos")]
    {
        shell.command("open").arg(&path).spawn()?;
    }
    
    #[cfg(target_os = "windows")]
    {
        shell.command("explorer").arg(&path).spawn()?;
    }
    
    #[cfg(target_os = "linux")]
    {
        shell.command("xdg-open").arg(&path).spawn()?;
    }
    
    Ok(())
}

/// Open URL in default browser
#[tauri::command]
pub async fn open_url(url: String, state: State<'_, AppState>) -> AppResult<()> {
    tracing::info!("Opening URL: {}", url);
    
    let shell = state.app_handle().shell();
    
    #[cfg(target_os = "macos")]
    {
        shell.command("open").arg(&url).spawn()?;
    }
    
    #[cfg(target_os = "windows")]
    {
        shell.command("cmd").args(["/C", "start", "", &url]).spawn()?;
    }
    
    #[cfg(target_os = "linux")]
    {
        shell.command("xdg-open").arg(&url).spawn()?;
    }
    
    Ok(())
}

/// Show window (with smart window management)
/// When opening settings/ai/clipboard windows, automatically hide the main launcher window
#[tauri::command]
pub async fn show_window(label: String, state: State<'_, AppState>) -> AppResult<()> {
    let app_handle = state.app_handle();
    
    // Windows that should hide the main launcher when opened
    let exclusive_windows = ["settings", "ai", "clipboard"];
    
    // If opening an exclusive window, hide the main window first
    if exclusive_windows.contains(&label.as_str()) {
        if let Some(main_window) = app_handle.get_webview_window("main") {
            let _ = main_window.hide();
            tracing::debug!("Main window hidden before showing {}", label);
        }
    }
    
    // Show the target window
    if let Some(window) = app_handle.get_webview_window(&label) {
        window.center()?;
        window.show()?;
        window.set_focus()?;
        tracing::info!("Window '{}' shown and focused", label);
    }
    
    Ok(())
}

/// Hide window
#[tauri::command]
pub async fn hide_window(label: String, state: State<'_, AppState>) -> AppResult<()> {
    if let Some(window) = state.app_handle().get_webview_window(&label) {
        window.hide()?;
    }
    Ok(())
}

/// Toggle main window visibility
#[tauri::command]
pub async fn toggle_main_window(state: State<'_, AppState>) -> AppResult<()> {
    if let Some(window) = state.app_handle().get_webview_window("main") {
        if window.is_visible()? {
            window.hide()?;
        } else {
            window.show()?;
            window.set_focus()?;
            window.center()?;
        }
    }
    Ok(())
}

/// Called by frontend when the UI is fully rendered and ready to be shown
/// This implements the "ready-to-show" pattern to eliminate white flash
/// 
/// IMPORTANT: Only the "main" window is shown on startup.
/// Other windows (settings, clipboard, ai) are pre-created but stay hidden
/// until explicitly triggered by user action (shortcuts, tray menu, etc.)
#[tauri::command]
pub async fn app_ready(window: tauri::Window) -> Result<(), String> {
    let label = window.label();
    tracing::info!("Frontend signaled ready for window: {}", label);
    
    // Only show the main launcher window on startup
    // Other windows should remain hidden until user explicitly opens them
    if label == "main" {
        // Center the window first
        window.center().map_err(|e| e.to_string())?;
        
        // Now show the window - UI is guaranteed to be rendered
        window.show().map_err(|e| e.to_string())?;
        
        // Focus the window
        window.set_focus().map_err(|e| e.to_string())?;
        
        tracing::info!("Main window shown after frontend ready");
    } else {
        // For non-main windows, just log that they're ready but keep them hidden
        tracing::debug!("Window '{}' ready but staying hidden (will show on user action)", label);
    }
    
    Ok(())
}
