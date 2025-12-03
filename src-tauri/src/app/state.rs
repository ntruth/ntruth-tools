use super::config::AppConfig;
use super::error::AppResult;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;

/// Global application state
pub struct AppState {
    pub app_handle: AppHandle,
    pub config: Arc<RwLock<AppConfig>>,
}

impl AppState {
    pub fn new(app_handle: AppHandle) -> AppResult<Self> {
        let config = Arc::new(RwLock::new(AppConfig::default()));

        Ok(Self {
            app_handle,
            config,
        })
    }

    pub async fn get_config(&self) -> AppConfig {
        self.config.read().await.clone()
    }

    pub async fn update_config(&self, new_config: AppConfig) -> AppResult<()> {
        let mut config = self.config.write().await;
        *config = new_config;
        Ok(())
    }
}
