use crate::app::{error::AppResult, state::AppState};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub id: String,
    pub r#type: String,
    pub content: String,
    pub timestamp: i64,
    pub favorite: bool,
}

/// Get clipboard history
#[tauri::command]
pub async fn get_clipboard_history(
    state: State<'_, AppState>,
) -> AppResult<Vec<ClipboardItem>> {
    let storage = state.clipboard_storage().await?;
    let items = storage.get_history(100, 0).await?;
    
    let clipboard_items = items
        .into_iter()
        .map(|item| ClipboardItem {
            id: item.id,
            r#type: item.content_type,
            content: item.plain_text.unwrap_or_default(),
            timestamp: item.created_at.timestamp(),
            favorite: item.is_favorite,
        })
        .collect();
    
    Ok(clipboard_items)
}

/// Paste clipboard item
#[tauri::command]
pub async fn paste_clipboard_item(
    id: String,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let storage = state.clipboard_storage().await?;
    if let Some(item) = storage.get_by_id(&id).await? {
        // Write content to clipboard (synchronous operation)
        use tauri_plugin_clipboard_manager::ClipboardExt;
        if let Some(text) = item.plain_text {
            state.app_handle().clipboard().write_text(text)?;
        }
        
        // Update access count
        storage.increment_access_count(&id).await?;
    }
    Ok(())
}

/// Toggle clipboard favorite status
#[tauri::command]
pub async fn toggle_clipboard_favorite(
    id: String,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let storage = state.clipboard_storage().await?;
    storage.toggle_favorite(&id).await?;
    Ok(())
}

/// Delete clipboard item
#[tauri::command]
pub async fn delete_clipboard_item(
    id: String,
    state: State<'_, AppState>,
) -> AppResult<()> {
    let storage = state.clipboard_storage().await?;
    storage.delete(&id).await?;
    Ok(())
}

/// Show clipboard window
#[tauri::command]
pub async fn show_clipboard_window(state: State<'_, AppState>) -> AppResult<()> {
    let window_manager = state.clipboard_window_manager().await?;
    window_manager.show().await
}

/// Hide clipboard window
#[tauri::command]
pub async fn hide_clipboard_window(state: State<'_, AppState>) -> AppResult<()> {
    let window_manager = state.clipboard_window_manager().await?;
    window_manager.hide().await
}

