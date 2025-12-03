// macOS-specific implementations
pub mod apps;

pub use apps::{AppScanner, AppInfo};

use std::path::{Path, PathBuf};
use tokio::process::Command;

/// Extract icon from macOS app bundle and convert to PNG
/// Uses multiple methods to extract icons including from Asset Catalogs
pub async fn extract_app_icon(app_path: &Path) -> Option<Vec<u8>> {
    // Method 1: Try to find and convert .icns file
    if let Some(icns_path) = find_icns_file(app_path).await {
        if let Some(data) = convert_icns_to_png(&icns_path).await {
            return Some(data);
        }
    }
    
    // Method 2: Use iconutil to extract from the app bundle directly
    // This works for apps that use Asset Catalogs (Assets.car)
    if let Some(data) = extract_icon_via_qlmanage(app_path).await {
        return Some(data);
    }
    
    None
}

/// Use qlmanage (Quick Look) to generate an icon for any file/app
/// This is the most reliable method as it uses macOS's built-in icon rendering
async fn extract_icon_via_qlmanage(app_path: &Path) -> Option<Vec<u8>> {
    let temp_dir = std::env::temp_dir();
    let output_dir = temp_dir.join(format!("omnibox_icon_{}", uuid::Uuid::new_v4()));
    
    // Create temp directory
    if tokio::fs::create_dir_all(&output_dir).await.is_err() {
        return None;
    }
    
    // Use qlmanage to generate icon preview
    let output = Command::new("qlmanage")
        .args([
            "-t",                                    // Generate thumbnail
            "-s", "64",                             // Size 64x64
            "-o", output_dir.to_string_lossy().as_ref(),
            app_path.to_string_lossy().as_ref(),
        ])
        .output()
        .await
        .ok()?;
    
    if !output.status.success() {
        let _ = tokio::fs::remove_dir_all(&output_dir).await;
        return None;
    }
    
    // Find the generated PNG file
    let mut entries = tokio::fs::read_dir(&output_dir).await.ok()?;
    let mut png_data = None;
    
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "png" {
                png_data = tokio::fs::read(&path).await.ok();
                break;
            }
        }
    }
    
    // Cleanup
    let _ = tokio::fs::remove_dir_all(&output_dir).await;
    
    png_data
}

/// Find the .icns icon file in an app bundle
async fn find_icns_file(app_path: &Path) -> Option<std::path::PathBuf> {
    let resources_path = app_path.join("Contents/Resources");
    
    // First, try to read CFBundleIconFile from Info.plist
    let info_plist_path = app_path.join("Contents/Info.plist");
    if info_plist_path.exists() {
        if let Some(icon_name) = read_icon_name_from_plist(&info_plist_path).await {
            // Icon name might be with or without .icns extension
            let icon_filename = if icon_name.ends_with(".icns") {
                icon_name
            } else {
                format!("{}.icns", icon_name)
            };
            let icon_path = resources_path.join(&icon_filename);
            if icon_path.exists() {
                return Some(icon_path);
            }
        }
    }

    // Fallback: Common icon filenames
    let icon_names = vec![
        "AppIcon.icns",
        "app.icns",
        "icon.icns",
        "Icon.icns",
    ];

    for icon_name in icon_names {
        let icon_path = resources_path.join(icon_name);
        if icon_path.exists() {
            return Some(icon_path);
        }
    }

    // Last resort: find any .icns file
    if let Ok(mut entries) = tokio::fs::read_dir(&resources_path).await {
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

/// Read CFBundleIconFile from Info.plist
async fn read_icon_name_from_plist(plist_path: &Path) -> Option<String> {
    let content = tokio::fs::read_to_string(plist_path).await.ok()?;
    
    // Simple XML parsing for CFBundleIconFile
    let key_tag = "<key>CFBundleIconFile</key>";
    if let Some(key_pos) = content.find(key_tag) {
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

/// Convert .icns to PNG using macOS sips command
async fn convert_icns_to_png(icns_path: &Path) -> Option<Vec<u8>> {
    // Create a temporary file for the PNG output
    let temp_dir = std::env::temp_dir();
    let temp_png = temp_dir.join(format!("omnibox_icon_{}.png", uuid::Uuid::new_v4()));
    
    // Use sips to convert icns to png (48x48 is good for UI display)
    let output = Command::new("sips")
        .args([
            "-s", "format", "png",
            "-z", "64", "64",  // Resize to 64x64
            icns_path.to_string_lossy().as_ref(),
            "--out", temp_png.to_string_lossy().as_ref(),
        ])
        .output()
        .await
        .ok()?;
    
    if !output.status.success() {
        tracing::debug!("sips conversion failed for {:?}: {:?}", icns_path, String::from_utf8_lossy(&output.stderr));
        return None;
    }
    
    // Read the converted PNG
    let png_data = tokio::fs::read(&temp_png).await.ok();
    
    // Clean up temp file
    let _ = tokio::fs::remove_file(&temp_png).await;
    
    png_data
}

/// Get app icon as base64 data URL
pub async fn get_app_icon_base64(app_path: &Path) -> Option<String> {
    let png_data = extract_app_icon(app_path).await?;
    let base64_data = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png_data);
    Some(format!("data:image/png;base64,{}", base64_data))
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

/// Application info with localized display name
#[derive(Debug, Clone)]
pub struct AppDisplayInfo {
    pub path: PathBuf,
    pub name: String,           // File system name (e.g., "WeChat")
    pub display_name: String,   // Localized display name (e.g., "微信")
}

/// Search applications using mdfind with Chinese/localized name support
pub async fn search_apps_with_mdfind(query: &str) -> Vec<AppDisplayInfo> {
    use tokio::process::Command;
    
    // Build mdfind query to search both file name and display name
    // kMDItemFSName: File system name
    // kMDItemDisplayName: Localized display name
    let mdfind_query = format!(
        "kMDItemContentType == 'com.apple.application-bundle' && (kMDItemFSName == '*{}*'c || kMDItemDisplayName == '*{}*'c)",
        query, query
    );
    
    let output = match Command::new("mdfind")
        .arg(&mdfind_query)
        .output()
        .await
    {
        Ok(o) => o,
        Err(e) => {
            tracing::error!("Failed to run mdfind: {}", e);
            return Vec::new();
        }
    };
    
    if !output.status.success() {
        return Vec::new();
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();
    
    for line in stdout.lines() {
        let path = PathBuf::from(line.trim());
        if path.exists() {
            // Get display name using mdls
            let (name, display_name) = get_app_names(&path).await;
            results.push(AppDisplayInfo {
                path,
                name,
                display_name,
            });
        }
    }
    
    // Limit results
    results.truncate(20);
    results
}

/// Get both file system name and localized display name for an app
async fn get_app_names(app_path: &Path) -> (String, String) {
    use tokio::process::Command;
    
    // Get file system name
    let fs_name = app_path
        .file_stem()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    
    // Try to get display name from mdls
    let display_name = match Command::new("mdls")
        .args(["-name", "kMDItemDisplayName", "-raw", app_path.to_string_lossy().as_ref()])
        .output()
        .await
    {
        Ok(output) if output.status.success() => {
            let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if name.is_empty() || name == "(null)" {
                fs_name.clone()
            } else {
                name
            }
        }
        _ => fs_name.clone(),
    };
    
    (fs_name, display_name)
}

/// Index all applications and return their display names for the indexer
pub async fn scan_apps_with_display_names() -> Vec<AppDisplayInfo> {
    use tokio::process::Command;
    
    // Use mdfind to get all applications
    let output = match Command::new("mdfind")
        .arg("kMDItemContentType == 'com.apple.application-bundle'")
        .output()
        .await
    {
        Ok(o) => o,
        Err(e) => {
            tracing::error!("Failed to scan applications: {}", e);
            return Vec::new();
        }
    };
    
    if !output.status.success() {
        return Vec::new();
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();
    
    for line in stdout.lines() {
        let path = PathBuf::from(line.trim());
        
        // Only include apps from standard locations
        let path_str = path.to_string_lossy();
        if !path_str.starts_with("/Applications") 
            && !path_str.starts_with("/System/Applications")
            && !path_str.contains("/Applications/") 
        {
            continue;
        }
        
        if path.exists() {
            let (name, display_name) = get_app_names(&path).await;
            results.push(AppDisplayInfo {
                path,
                name,
                display_name,
            });
        }
    }
    
    tracing::info!("Scanned {} applications with display names", results.len());
    results
}

