use crate::app::{config::AppConfig, error::AppResult, state::AppState};
use std::path::PathBuf;
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

/// Reset config to defaults
#[tauri::command]
pub async fn reset_config(state: State<'_, AppState>) -> AppResult<AppConfig> {
    let default_config = AppConfig::default();
    state.update_config(default_config.clone()).await?;
    Ok(default_config)
}

/// Export config to file
#[tauri::command]
pub async fn export_config(path: PathBuf, state: State<'_, AppState>) -> AppResult<()> {
    let config = state.get_config().await;
    let yaml = serde_yaml::to_string(&config)?;
    std::fs::write(path, yaml)?;
    Ok(())
}

/// Import config from file
#[tauri::command]
pub async fn import_config(path: PathBuf, state: State<'_, AppState>) -> AppResult<AppConfig> {
    let yaml = std::fs::read_to_string(path)?;
    let config: AppConfig = serde_yaml::from_str(&yaml)?;
    state.update_config(config.clone()).await?;
    Ok(config)
}
