// macOS-specific implementations
pub mod apps;

pub use apps::{AppScanner, AppInfo};

/// Extract icon from macOS app bundle
pub async fn extract_app_icon(app_path: &std::path::Path) -> Option<Vec<u8>> {
    // Look for icon in Resources folder
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
            if let Ok(data) = tokio::fs::read(&icon_path).await {
                // For now, return raw .icns data
                // In production, convert to PNG using image processing library
                return Some(data);
            }
        }
    }

    // Try to find any .icns file
    if let Ok(mut entries) = tokio::fs::read_dir(&resources_path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "icns" {
                    if let Ok(data) = tokio::fs::read(&path).await {
                        return Some(data);
                    }
                }
            }
        }
    }

    None
}

/// Launch an application
pub async fn launch_app(app_path: &std::path::Path) -> Result<(), String> {
    use tokio::process::Command;

    let output = Command::new("open")
        .arg(app_path)
        .output()
        .await
        .map_err(|e| format!("Failed to launch app: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to launch app: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

