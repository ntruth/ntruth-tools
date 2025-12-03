// macOS application scanner
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub bundle_id: Option<String>,
    pub version: Option<String>,
    pub icon_path: Option<PathBuf>,
}

pub struct AppScanner;

impl AppScanner {
    /// Scan for installed applications on macOS
    pub async fn scan_apps() -> Vec<AppInfo> {
        let mut apps = Vec::new();

        // Scan /Applications directory
        if let Ok(system_apps) = Self::scan_directory("/Applications").await {
            apps.extend(system_apps);
        }

        // Scan user Applications directory
        if let Ok(home) = std::env::var("HOME") {
            let user_apps_path = format!("{}/Applications", home);
            if let Ok(user_apps) = Self::scan_directory(&user_apps_path).await {
                apps.extend(user_apps);
            }
        }

        apps
    }

    async fn scan_directory(path: &str) -> Result<Vec<AppInfo>, std::io::Error> {
        let mut apps = Vec::new();
        let mut entries = fs::read_dir(path).await?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            let entry_path = entry.path();

            // Check if it's a .app bundle
            if let Some(extension) = entry_path.extension() {
                if extension == "app" {
                    if let Some(app_info) = Self::parse_app_bundle(&entry_path).await {
                        apps.push(app_info);
                    }
                }
            }
        }

        Ok(apps)
    }

    async fn parse_app_bundle(app_path: &PathBuf) -> Option<AppInfo> {
        let app_name = app_path
            .file_stem()?
            .to_string_lossy()
            .to_string();

        // Read Info.plist for bundle information
        let info_plist_path = app_path.join("Contents/Info.plist");
        let (bundle_id, version) = if info_plist_path.exists() {
            Self::read_info_plist(&info_plist_path).await
        } else {
            (None, None)
        };

        // Find icon
        let icon_path = Self::find_app_icon(app_path).await;

        Some(AppInfo {
            id: format!("app-{}", app_name.to_lowercase().replace(' ', "-")),
            name: app_name,
            path: app_path.clone(),
            bundle_id,
            version,
            icon_path,
        })
    }

    async fn read_info_plist(plist_path: &PathBuf) -> (Option<String>, Option<String>) {
        // Simple plist parsing (for full support, use plist crate)
        // This is a basic implementation
        match fs::read_to_string(plist_path).await {
            Ok(content) => {
                let bundle_id = Self::extract_plist_value(&content, "CFBundleIdentifier");
                let version = Self::extract_plist_value(&content, "CFBundleShortVersionString");
                (bundle_id, version)
            }
            Err(_) => (None, None),
        }
    }

    fn extract_plist_value(content: &str, key: &str) -> Option<String> {
        // Very basic XML parsing - in production, use proper plist parser
        let key_tag = format!("<key>{}</key>", key);
        if let Some(key_pos) = content.find(&key_tag) {
            let after_key = &content[key_pos + key_tag.len()..];
            if let Some(string_start) = after_key.find("<string>") {
                let after_string_start = &after_key[string_start + 8..];
                if let Some(string_end) = after_string_start.find("</string>") {
                    return Some(after_string_start[..string_end].trim().to_string());
                }
            }
        }
        None
    }

    async fn find_app_icon(app_path: &PathBuf) -> Option<PathBuf> {
        let resources_path = app_path.join("Contents/Resources");
        
        // Common icon filenames
        let icon_names = vec![
            "AppIcon.icns",
            "app.icns",
            "icon.icns",
        ];

        for icon_name in icon_names {
            let icon_path = resources_path.join(icon_name);
            if icon_path.exists() {
                return Some(icon_path);
            }
        }

        // Try to find any .icns file
        if let Ok(mut entries) = fs::read_dir(&resources_path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "icns" {
                        return Some(path);
                    }
                }
            }
        }

        None
    }
}
