use crate::app::error::AppResult;
use crate::core::clipboard::types::ClipboardContent;
use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tokio::sync::RwLock;
use tokio::time::sleep;

/// Clipboard monitor that watches for clipboard changes
pub struct ClipboardMonitor {
    app_handle: AppHandle,
    is_running: Arc<RwLock<bool>>,
    last_hash: Arc<RwLock<Option<String>>>,
}

impl ClipboardMonitor {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            is_running: Arc::new(RwLock::new(false)),
            last_hash: Arc::new(RwLock::new(None)),
        }
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

        tokio::spawn(async move {
            while *is_running.read().await {
                if let Ok(content) = Self::read_clipboard(&app_handle).await {
                    let content_hash = content.hash();
                    let mut last = last_hash.write().await;
                    
                    // Only process if content has changed
                    if last.as_ref() != Some(&content_hash) {
                        *last = Some(content_hash.clone());
                        
                        // Just log for now - events can be added later if needed
                        tracing::debug!("Clipboard content changed: {:?}", content.content_type());
                    }
                }

                // Poll every 500ms
                sleep(Duration::from_millis(500)).await;
            }
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

        // If no text, return empty text
        Ok(ClipboardContent::Text {
            content: String::new(),
            plain_text: String::new(),
        })
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
