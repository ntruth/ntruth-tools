use crate::app::error::AppResult;
use crate::core::clipboard::types::ClipboardContent;
use crate::core::clipboard::storage::{ClipboardStorage, ClipboardHistoryItem};
use crate::core::clipboard::filter::ContentFilter;
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashSet;
use tauri::AppHandle;
use tokio::sync::RwLock;
use tokio::time::sleep;
use chrono::Utc;

/// Clipboard monitor that watches for clipboard changes
pub struct ClipboardMonitor {
    app_handle: AppHandle,
    is_running: Arc<RwLock<bool>>,
    last_hash: Arc<RwLock<Option<String>>>,
    storage: Arc<RwLock<Option<Arc<ClipboardStorage>>>>,
    content_filter: ContentFilter,
    excluded_apps: Arc<RwLock<HashSet<String>>>,
}

impl ClipboardMonitor {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            is_running: Arc::new(RwLock::new(false)),
            last_hash: Arc::new(RwLock::new(None)),
            storage: Arc::new(RwLock::new(None)),
            content_filter: ContentFilter::new(),
            excluded_apps: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Set the storage for saving clipboard history
    pub async fn set_storage(&self, storage: Arc<ClipboardStorage>) {
        let mut s = self.storage.write().await;
        *s = Some(storage);
    }

    /// Set excluded apps list
    pub async fn set_excluded_apps(&self, apps: Vec<String>) {
        let mut excluded = self.excluded_apps.write().await;
        excluded.clear();
        for app in apps {
            excluded.insert(app.to_lowercase());
        }
    }

    /// Check if an app is excluded
    async fn is_app_excluded(&self, app_name: &Option<String>) -> bool {
        if let Some(name) = app_name {
            let excluded = self.excluded_apps.read().await;
            return excluded.contains(&name.to_lowercase());
        }
        false
    }

    /// Start monitoring clipboard changes
    pub async fn start(&self) -> AppResult<()> {
        let mut running = self.is_running.write().await;
        if *running {
            return Ok(());
        }
        *running = true;
        drop(running);

        let is_running = self.is_running.clone();
        let last_hash = self.last_hash.clone();
        let app_handle = self.app_handle.clone();
        let storage = self.storage.clone();
        let content_filter = self.content_filter.clone();
        let excluded_apps = self.excluded_apps.clone();

        tokio::spawn(async move {
            tracing::info!("Clipboard monitor started");
            
            while *is_running.read().await {
                if let Ok(content) = Self::read_clipboard(&app_handle).await {
                    let content_hash = content.hash();
                    let mut last = last_hash.write().await;
                    
                    // Only process if content has changed and is not empty
                    if last.as_ref() != Some(&content_hash) && !content.is_empty() {
                        *last = Some(content_hash.clone());
                        drop(last);
                        
                        // Get source app
                        let source_app = Self::get_active_app();
                        
                        // Check if app is excluded
                        if let Some(ref app_name) = source_app {
                            let excluded = excluded_apps.read().await;
                            if excluded.contains(&app_name.to_lowercase()) {
                                tracing::debug!("Skipping clipboard from excluded app: {}", app_name);
                                continue;
                            }
                        }
                        
                        tracing::debug!("Clipboard content changed: {:?}", content.content_type());
                        
                        // Check if content should be filtered (sensitive)
                        let plain_text = content.plain_text();
                        let is_sensitive = content_filter.is_sensitive(&plain_text);
                        
                        // Save to storage if available
                        if let Some(ref storage) = *storage.read().await {
                            // Check if this content already exists
                            if !storage.exists_by_hash(&content_hash).await.unwrap_or(false) {
                                let item = ClipboardHistoryItem {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    content_type: content.content_type().to_string(),
                                    content_hash,
                                    plain_text: Some(plain_text),
                                    data: content.data(),
                                    source_app,
                                    source_window: None,
                                    is_favorite: false,
                                    is_sensitive,
                                    created_at: Utc::now(),
                                    accessed_at: None,
                                    access_count: 0,
                                };
                                
                                if let Err(e) = storage.add_item(&item).await {
                                    tracing::error!("Failed to save clipboard item: {}", e);
                                } else {
                                    tracing::debug!("Clipboard item saved: {}", item.id);
                                    
                                    // Emit event to frontend
                                    let _ = tauri::Emitter::emit(&app_handle, "clipboard-changed", &item.id);
                                }
                            }
                        }
                    }
                }

                // Poll every 500ms
                sleep(Duration::from_millis(500)).await;
            }
            
            tracing::info!("Clipboard monitor stopped");
        });

        Ok(())
    }

    /// Stop monitoring clipboard changes
    pub async fn stop(&self) -> AppResult<()> {
        let mut running = self.is_running.write().await;
        *running = false;
        Ok(())
    }

    /// Check if monitor is running
    pub async fn is_running(&self) -> bool {
        *self.is_running.read().await
    }

    /// Read current clipboard content
    async fn read_clipboard(app_handle: &AppHandle) -> AppResult<ClipboardContent> {
        use tauri_plugin_clipboard_manager::ClipboardExt;

        // Try to read text (synchronous operation)
        if let Ok(text) = app_handle.clipboard().read_text() {
            if !text.is_empty() {
                return Ok(ClipboardContent::Text {
                    content: text.clone(),
                    plain_text: text,
                });
            }
        }

        // Try to read image
        // Note: Image reading might not be supported on all platforms
        // We'll handle this gracefully

        // If no content, return empty
        Ok(ClipboardContent::Text {
            content: String::new(),
            plain_text: String::new(),
        })
    }

    /// Get the name of the currently active application
    #[cfg(target_os = "macos")]
    fn get_active_app() -> Option<String> {
        // Use AppleScript to get frontmost application
        // This is a simple implementation - for production use cocoa bindings
        None
    }

    #[cfg(target_os = "windows")]
    fn get_active_app() -> Option<String> {
        // Use Windows API to get foreground window process name
        // This is a simple implementation - for production use winapi
        None
    }

    #[cfg(target_os = "linux")]
    fn get_active_app() -> Option<String> {
        // Use xdotool or similar to get active window
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_creation() {
        // This test would require a Tauri app handle, so we skip actual testing
        // Just verify the module compiles
    }
}
