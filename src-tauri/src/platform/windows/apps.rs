// Windows application scanner
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
    /// Scan for installed applications on Windows
    pub async fn scan_apps() -> Vec<AppInfo> {
        let mut apps = Vec::new();

        // Scan Start Menu (Common)
        let program_data = std::env::var("ProgramData")
            .unwrap_or_else(|_| "C:\\ProgramData".to_string());
        let common_start_menu = format!("{}\\Microsoft\\Windows\\Start Menu\\Programs", program_data);
        if let Ok(common_apps) = Self::scan_start_menu(&common_start_menu).await {
            apps.extend(common_apps);
        }

        // Scan Start Menu (User)
        if let Ok(appdata) = std::env::var("APPDATA") {
            let user_start_menu = format!("{}\\Microsoft\\Windows\\Start Menu\\Programs", appdata);
            if let Ok(user_apps) = Self::scan_start_menu(&user_start_menu).await {
                apps.extend(user_apps);
            }
        }

        // Scan Program Files
        if let Ok(program_files) = std::env::var("ProgramFiles") {
            if let Ok(pf_apps) = Self::scan_program_files(&program_files).await {
                apps.extend(pf_apps);
            }
        }

        // Scan Program Files (x86)
        if let Ok(program_files_x86) = std::env::var("ProgramFiles(x86)") {
            if let Ok(pf_apps) = Self::scan_program_files(&program_files_x86).await {
                apps.extend(pf_apps);
            }
        }

        apps
    }

    async fn scan_start_menu(path: &str) -> Result<Vec<AppInfo>, std::io::Error> {
        let mut apps = Vec::new();
        Self::scan_directory_recursive(path, &mut apps).await?;
        Ok(apps)
    }

    async fn scan_directory_recursive(
        path: &str,
        apps: &mut Vec<AppInfo>,
    ) -> Result<(), std::io::Error> {
        Box::pin(async move {
            let mut entries = fs::read_dir(path).await?;

            while let Ok(Some(entry)) = entries.next_entry().await {
                let entry_path = entry.path();
                let metadata = entry.metadata().await?;

                if metadata.is_dir() {
                    // Recursively scan subdirectories
                    if let Some(path_str) = entry_path.to_str() {
                        let _ = Self::scan_directory_recursive(path_str, apps).await;
                    }
                } else if let Some(extension) = entry_path.extension() {
                    // Check for .lnk (shortcut) files
                    if extension == "lnk" {
                        if let Some(app_info) = Self::parse_shortcut(&entry_path).await {
                            apps.push(app_info);
                        }
                    }
                }
            }

            Ok(())
        }).await
    }

    async fn scan_program_files(path: &str) -> Result<Vec<AppInfo>, std::io::Error> {
        let mut apps = Vec::new();
        let mut entries = fs::read_dir(path).await?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            let entry_path = entry.path();

            if entry_path.is_dir() {
                // Look for .exe files in the directory
                if let Ok(mut sub_entries) = fs::read_dir(&entry_path).await {
                    while let Ok(Some(sub_entry)) = sub_entries.next_entry().await {
                        let sub_path = sub_entry.path();
                        if let Some(extension) = sub_path.extension() {
                            if extension == "exe" {
                                if let Some(app_info) = Self::parse_executable(&sub_path).await {
                                    apps.push(app_info);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(apps)
    }

    async fn parse_shortcut(shortcut_path: &PathBuf) -> Option<AppInfo> {
        // For parsing .lnk files, we would need a Windows-specific library
        // For now, we'll just extract the name from the filename
        let app_name = shortcut_path
            .file_stem()?
            .to_string_lossy()
            .to_string();

        // Skip system shortcuts and folders
        if app_name.to_lowercase().contains("uninstall") 
            || app_name.to_lowercase().contains("help")
            || app_name.to_lowercase().contains("readme") {
            return None;
        }

        Some(AppInfo {
            id: format!("app-{}", app_name.to_lowercase().replace(' ', "-")),
            name: app_name,
            path: shortcut_path.clone(),
            bundle_id: None,
            version: None,
            icon_path: None,
        })
    }

    async fn parse_executable(exe_path: &PathBuf) -> Option<AppInfo> {
        let app_name = exe_path
            .file_stem()?
            .to_string_lossy()
            .to_string();

        // Skip common system executables (compute lowercase once)
        let lowercase_name = app_name.to_lowercase();
        if lowercase_name.contains("uninstall")
            || lowercase_name.contains("setup")
            || lowercase_name.contains("installer")
            || lowercase_name.contains("updater") {
            return None;
        }

        Some(AppInfo {
            id: format!("app-{}", lowercase_name.replace(' ', "-")),
            name: app_name,
            path: exe_path.clone(),
            bundle_id: None,
            version: None,
            icon_path: Some(exe_path.clone()), // .exe files contain their own icons
        })
    }
}
