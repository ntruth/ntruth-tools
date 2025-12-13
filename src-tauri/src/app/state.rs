use super::config::AppConfig;
use super::error::AppResult;
use crate::core::clipboard::{ClipboardMonitor, ClipboardStorage, ClipboardWindowManager};
use crate::core::indexer::{Indexer, ScanConfig};
use crate::core::plugin::PluginManager;
use crate::storage::{Database, IconCache};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;

#[cfg(windows)]
use crate::app_indexer::AppIndexer;

/// Global application state
#[derive(Clone)]
pub struct AppState {
    app_handle: AppHandle,
    pub config: Arc<RwLock<AppConfig>>,
    pub indexer: Arc<Indexer>,
    pub db: Arc<Database>,
    pub icon_cache: Arc<IconCache>,
    pub plugin_manager: Arc<RwLock<PluginManager>>,
    clipboard_storage: Arc<RwLock<Option<Arc<ClipboardStorage>>>>,
    clipboard_monitor: Arc<RwLock<Option<Arc<ClipboardMonitor>>>>,
    clipboard_window_manager: Arc<RwLock<Option<Arc<ClipboardWindowManager>>>>,
    #[cfg(windows)]
    pub app_indexer: Arc<AppIndexer>,
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

        // Create app data directory if it doesn't exist
        if !app_data_dir.exists() {
            std::fs::create_dir_all(&app_data_dir)?;
        }

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

        // Initialize plugin manager
        let plugins_dir = app_data_dir.join("plugins");
        let plugin_manager = PluginManager::new(plugins_dir);
        if let Err(e) = plugin_manager.init().await {
            tracing::warn!("Failed to initialize plugin manager: {}", e);
        }

        // Initialize app indexer (Windows only)
        #[cfg(windows)]
        let app_indexer = {
            let indexer = Arc::new(AppIndexer::new());
            // Initialize synchronously to ensure apps are available on first search
            tracing::info!("Initializing AppIndexer...");
            match indexer.init().await {
                Ok(count) => tracing::info!("AppIndexer initialized with {} apps", count),
                Err(e) => tracing::error!("Failed to initialize AppIndexer: {}", e),
            }
            indexer
        };

        Ok(Self {
            app_handle,
            config,
            indexer,
            db,
            icon_cache,
            plugin_manager: Arc::new(RwLock::new(plugin_manager)),
            clipboard_storage: Arc::new(RwLock::new(None)),
            clipboard_monitor: Arc::new(RwLock::new(None)),
            clipboard_window_manager: Arc::new(RwLock::new(None)),
            #[cfg(windows)]
            app_indexer,
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
        
        // Index macOS Applications with display names (for Chinese search support)
        #[cfg(target_os = "macos")]
        {
            use crate::platform::macos::scan_apps_with_display_names;
            
            // Scan all applications with their display names
            let apps = scan_apps_with_display_names().await;
            let mut indexed_count = 0;
            
            for app in apps {
                // Use display_name if different from file name
                let display_name = if app.display_name != app.name && !app.display_name.is_empty() {
                    Some(app.display_name)
                } else {
                    None
                };
                
                if let Err(e) = self.indexer.add_file_with_display_name(&app.path, display_name).await {
                    // Ignore "already indexed" errors
                    if !e.contains("already indexed") {
                        tracing::debug!("Failed to index app {:?}: {}", app.path, e);
                    }
                } else {
                    indexed_count += 1;
                }
            }
            
            tracing::info!("Indexed {} applications with display names", indexed_count);
        }
        
        tracing::info!("Indexing completed, total files: {}", self.indexer.file_count().await);
        Ok(())
    }
    
    /// Index applications from a directory (macOS .app bundles) - legacy method
    #[cfg(target_os = "macos")]
    async fn index_applications(&self, dir: &std::path::Path) -> AppResult<()> {
        use tokio::fs;
        
        let mut entries = fs::read_dir(dir).await?;
        let mut count = 0;
        
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            
            // Only process .app bundles
            if let Some(ext) = path.extension() {
                if ext == "app" {
                    // Add the .app bundle itself to the index
                    if let Err(e) = self.indexer.add_file(&path).await {
                        // Ignore "already indexed" errors
                        if !e.contains("already indexed") {
                            tracing::debug!("Failed to index app {:?}: {}", path, e);
                        }
                    } else {
                        count += 1;
                    }
                }
            }
        }
        
        tracing::info!("Indexed {} applications from {:?}", count, dir);
        Ok(())
    }

    /// Get or create clipboard storage
    pub async fn clipboard_storage(&self) -> AppResult<Arc<ClipboardStorage>> {
        let mut storage = self.clipboard_storage.write().await;
        if storage.is_none() {
            let pool = self.db.pool().clone();
            let clipboard_storage = Arc::new(ClipboardStorage::new(pool).await?);
            *storage = Some(clipboard_storage.clone());
        }
        Ok(storage.as_ref().unwrap().clone())
    }

    /// Get or create clipboard monitor
    pub async fn clipboard_monitor(&self) -> AppResult<Arc<ClipboardMonitor>> {
        let mut monitor = self.clipboard_monitor.write().await;
        if monitor.is_none() {
            let clipboard_monitor = Arc::new(ClipboardMonitor::new(self.app_handle.clone()));
            
            // Set storage for the monitor
            let storage = self.clipboard_storage().await?;
            clipboard_monitor.set_storage(storage).await;
            
            *monitor = Some(clipboard_monitor.clone());
        }
        Ok(monitor.as_ref().unwrap().clone())
    }

    /// Get or create clipboard window manager
    pub async fn clipboard_window_manager(&self) -> AppResult<Arc<ClipboardWindowManager>> {
        let mut manager = self.clipboard_window_manager.write().await;
        if manager.is_none() {
            let window_manager = Arc::new(ClipboardWindowManager::new(self.app_handle.clone()));
            *manager = Some(window_manager.clone());
        }
        Ok(manager.as_ref().unwrap().clone())
    }

    /// Get app handle for Tauri operations
    pub fn app_handle(&self) -> &AppHandle {
        &self.app_handle
    }
}
