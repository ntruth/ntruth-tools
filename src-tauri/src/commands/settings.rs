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
    // Validate and canonicalize path to prevent directory traversal
    let canonical_path = path.canonicalize().unwrap_or(path);
    
    // Additional security: ensure we're writing to a safe location
    if let Some(parent) = canonical_path.parent() {
        if !parent.exists() {
            return Err(crate::app::error::AppError::Config(
                "Parent directory does not exist".to_string()
            ));
        }
    }
    
    let config = state.get_config().await;
    let yaml = serde_yaml::to_string(&config)?;
    std::fs::write(canonical_path, yaml)?;
    Ok(())
}

/// Import config from file
#[tauri::command]
pub async fn import_config(path: PathBuf, state: State<'_, AppState>) -> AppResult<AppConfig> {
    // Validate and canonicalize path to prevent directory traversal
    let canonical_path = path.canonicalize().map_err(|_| {
        crate::app::error::AppError::Config("Invalid file path".to_string())
    })?;
    
    // Check file size to prevent resource exhaustion (limit to 1MB)
    let metadata = std::fs::metadata(&canonical_path)?;
    if metadata.len() > 1_048_576 {
        return Err(crate::app::error::AppError::Config(
            "Config file too large (max 1MB)".to_string()
        ));
    }
    
    let yaml = std::fs::read_to_string(canonical_path)?;
    let config: AppConfig = serde_yaml::from_str(&yaml)?;
    state.update_config(config.clone()).await?;
    Ok(config)
}
