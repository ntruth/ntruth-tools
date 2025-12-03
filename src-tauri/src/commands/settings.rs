use crate::app::{config::AppConfig, error::AppResult, state::AppState};
use tauri::State;

/// Get application config
#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> AppResult<AppConfig> {
    Ok(state.get_config().await)
}

/// Update application config
#[tauri::command]
pub async fn update_config(
    config: AppConfig,
    state: State<'_, AppState>,
) -> AppResult<()> {
    state.update_config(config).await
}
