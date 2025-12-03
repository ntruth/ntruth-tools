use crate::app::error::AppResult;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

/// Manages the clipboard window
pub struct ClipboardWindowManager {
    app_handle: AppHandle,
}

impl ClipboardWindowManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    /// Show the clipboard window
    pub async fn show(&self) -> AppResult<()> {
        if let Some(window) = self.app_handle.get_webview_window("clipboard") {
            window.show()?;
            window.set_focus()?;
        } else {
            // Create window if it doesn't exist
            self.create_window().await?;
        }
        Ok(())
    }

    /// Hide the clipboard window
    pub async fn hide(&self) -> AppResult<()> {
        if let Some(window) = self.app_handle.get_webview_window("clipboard") {
            window.hide()?;
        }
        Ok(())
    }

    /// Toggle clipboard window visibility
    pub async fn toggle(&self) -> AppResult<()> {
        if let Some(window) = self.app_handle.get_webview_window("clipboard") {
            if window.is_visible()? {
                window.hide()?;
            } else {
                window.show()?;
                window.set_focus()?;
            }
        } else {
            self.create_window().await?;
        }
        Ok(())
    }

    /// Create the clipboard window
    async fn create_window(&self) -> AppResult<()> {
        let _window = WebviewWindowBuilder::new(
            &self.app_handle,
            "clipboard",
            WebviewUrl::App("/clipboard".into()),
        )
        .title("Clipboard History")
        .inner_size(400.0, 500.0)
        .center()
        .decorations(false)
        .transparent(true)
        .skip_taskbar(true)
        .always_on_top(true)
        .visible(true)
        .build()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_manager_creation() {
        // This test would require a Tauri app handle
        // Just verify the module compiles
    }
}
