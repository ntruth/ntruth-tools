// Icon cache module
use crate::app::error::{AppError, AppResult};
use std::path::{Path, PathBuf};
use tokio::fs;

/// Icon cache manager
pub struct IconCache {
    cache_dir: PathBuf,
}

impl IconCache {
    /// Create a new icon cache
    pub async fn new(cache_dir: PathBuf) -> AppResult<Self> {
        // Ensure cache directory exists
        fs::create_dir_all(&cache_dir).await?;

        Ok(Self { cache_dir })
    }

    /// Get cached icon as Base64 string
    pub async fn get_icon(&self, app_path: &Path) -> Option<String> {
        let cache_path = self.get_cache_path(app_path);

        if cache_path.exists() {
            if let Ok(data) = fs::read(&cache_path).await {
                return Some(base64::encode(&data));
            }
        }

        None
    }

    /// Cache an icon from binary data
    pub async fn cache_icon(&self, app_path: &Path, icon_data: &[u8]) -> AppResult<()> {
        let cache_path = self.get_cache_path(app_path);

        // Ensure parent directory exists
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Write icon data to cache
        fs::write(&cache_path, icon_data).await?;

        Ok(())
    }

    /// Cache an icon as Base64 string
    pub async fn cache_icon_base64(&self, app_path: &Path, base64_data: &str) -> AppResult<()> {
        let icon_data = base64::decode(base64_data)
            .map_err(|e| AppError::Unknown(format!("Base64 decode error: {}", e)))?;
        self.cache_icon(app_path, &icon_data).await
    }

    /// Clear expired cache entries
    pub async fn clear_expired(&self, max_age_days: u32) -> AppResult<usize> {
        let mut cleared = 0;
        let max_age_secs = max_age_days as i64 * 86400;
        let now = std::time::SystemTime::now();

        let mut entries = fs::read_dir(&self.cache_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if let Ok(metadata) = fs::metadata(&path).await {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(age) = now.duration_since(modified) {
                        if age.as_secs() > max_age_secs as u64 {
                            let _ = fs::remove_file(&path).await;
                            cleared += 1;
                        }
                    }
                }
            }
        }

        Ok(cleared)
    }

    /// Get cache path for an app
    fn get_cache_path(&self, app_path: &Path) -> PathBuf {
        // Create a hash of the app path for the cache filename
        let hash = format!("{:x}", md5::compute(app_path.to_string_lossy().as_bytes()));
        self.cache_dir.join(format!("{}.png", hash))
    }

    /// Extract and cache icon from app path (platform-specific)
    #[cfg(target_os = "macos")]
    pub async fn extract_and_cache_icon(&self, app_path: &Path) -> AppResult<String> {
        use crate::platform::macos;

        if let Some(cached) = self.get_icon(app_path).await {
            return Ok(cached);
        }

        // Extract icon from macOS app bundle
        if let Some(icon_data) = macos::extract_app_icon(app_path).await {
            self.cache_icon(app_path, &icon_data).await?;
            return Ok(base64::encode(&icon_data));
        }

        Err(AppError::Unknown("Failed to extract icon".to_string()))
    }

    /// Extract and cache icon from app path (platform-specific)
    #[cfg(target_os = "windows")]
    pub async fn extract_and_cache_icon(&self, app_path: &Path) -> AppResult<String> {
        use crate::platform::windows;

        if let Some(cached) = self.get_icon(app_path).await {
            return Ok(cached);
        }

        // Extract icon from Windows executable or shortcut
        if let Some(icon_data) = windows::extract_app_icon(app_path).await {
            self.cache_icon(app_path, &icon_data).await?;
            return Ok(base64::encode(&icon_data));
        }

        Err(AppError::Unknown("Failed to extract icon".to_string()))
    }

    /// Extract and cache icon from app path (platform-specific)
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    pub async fn extract_and_cache_icon(&self, app_path: &Path) -> AppResult<String> {
        Err(AppError::Unknown(
            "Icon extraction not supported on this platform".to_string(),
        ))
    }
}

// Re-export base64 encode/decode (we need to add base64 and md5 to Cargo.toml)
