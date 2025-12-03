use super::config::AppConfig;
use super::error::AppResult;
use crate::core::indexer::{Indexer, ScanConfig};
use crate::storage::{Database, IconCache};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;

/// Global application state
#[derive(Clone)]
pub struct AppState {
    pub app_handle: AppHandle,
    pub config: Arc<RwLock<AppConfig>>,
    pub indexer: Arc<Indexer>,
    pub db: Arc<Database>,
    pub icon_cache: Arc<IconCache>,
}

impl AppState {
    pub async fn new(app_handle: AppHandle) -> AppResult<Self> {
        let config = Arc::new(RwLock::new(AppConfig::default()));
        
        // Create indexer with default configuration
        let indexer = Arc::new(Indexer::new(ScanConfig::default()));

        // Get app data directory
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| crate::app::error::AppError::Unknown(format!("Failed to get app data dir: {}", e)))?;

        // Initialize database
        let db_path = app_data_dir.join("omnibox.db");
        let db = Arc::new(
            Database::new(&db_path)
                .await
                .map_err(|e| crate::app::error::AppError::Database(format!("Failed to initialize database: {}", e)))?
        );

        // Initialize icon cache
        let cache_dir = app_data_dir.join("icon_cache");
        let icon_cache = Arc::new(
            IconCache::new(cache_dir)
                .await
                .map_err(|e| crate::app::error::AppError::Unknown(format!("Failed to initialize icon cache: {}", e)))?
        );

        Ok(Self {
            app_handle,
            config,
            indexer,
            db,
            icon_cache,
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
    
    /// Initialize file indexing for common directories
    pub async fn initialize_indexing(&self) -> AppResult<()> {
        // Get home directory
        if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
            let home_path = std::path::Path::new(&home);
            
            // Index common directories
            let dirs_to_index = vec![
                home_path.join("Documents"),
                home_path.join("Desktop"),
                home_path.join("Downloads"),
            ];
            
            for dir in dirs_to_index {
                if dir.exists() {
                    let _ = self.indexer.index_directory(&dir).await;
                }
            }
        }
        
        Ok(())
    }
}
