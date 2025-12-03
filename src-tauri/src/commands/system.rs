use crate::app::{error::AppResult, state::AppState};
use tauri::{Manager, State};

/// Open path in system file manager or default application
#[tauri::command]
pub async fn open_path(path: String, _state: State<'_, AppState>) -> AppResult<()> {
    // TODO: Implement path opening
    println!("Opening path: {}", path);
    Ok(())
}

/// Show window
#[tauri::command]
pub async fn show_window(label: String, state: State<'_, AppState>) -> AppResult<()> {
    if let Some(window) = state.app_handle.get_webview_window(&label) {
        window.show()?;
        window.set_focus()?;
    }
    Ok(())
}

/// Hide window
#[tauri::command]
pub async fn hide_window(label: String, state: State<'_, AppState>) -> AppResult<()> {
    if let Some(window) = state.app_handle.get_webview_window(&label) {
        window.hide()?;
    }
    Ok(())
}
