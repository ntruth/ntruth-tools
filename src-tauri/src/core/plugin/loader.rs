//! Plugin Loader
//! 插件加载器 - 负责从文件系统加载插件

use super::{InstalledPlugin, PluginMetadata, PluginStatus, PluginPermission, PluginError};
use std::path::PathBuf;
use chrono::Utc;

/// 插件加载器
pub struct PluginLoader;

impl PluginLoader {
    /// 创建新的加载器
    pub fn new() -> Self {
        Self
    }

    /// 从路径加载插件
    pub async fn load(&self, path: &PathBuf) -> Result<InstalledPlugin, PluginError> {
        // 读取 manifest.json
        let manifest_path = path.join("manifest.json");
        if !manifest_path.exists() {
            return Err(PluginError::InvalidManifest(
                format!("manifest.json not found in {:?}", path)
            ));
        }

        let manifest_content = std::fs::read_to_string(&manifest_path)
            .map_err(|e| PluginError::IoError(e.to_string()))?;

        let manifest: PluginManifest = serde_json::from_str(&manifest_content)
            .map_err(|e| PluginError::InvalidManifest(e.to_string()))?;

        // 读取状态文件（如果存在）
        let state_path = path.parent()
            .map(|p| p.join(format!("{}.state.json", manifest.id)))
            .unwrap_or_default();

        let (status, granted_permissions, config) = if state_path.exists() {
            let state_content = std::fs::read_to_string(&state_path)
                .map_err(|e| PluginError::IoError(e.to_string()))?;
            let state: PluginState = serde_json::from_str(&state_content)
                .unwrap_or_default();
            (state.status, state.granted_permissions, state.config)
        } else {
            (PluginStatus::Installed, vec![], std::collections::HashMap::new())
        };

        Ok(InstalledPlugin {
            metadata: PluginMetadata {
                id: manifest.id,
                name: manifest.name,
                version: manifest.version,
                description: manifest.description,
                author: manifest.author,
                homepage: manifest.homepage,
                repository: manifest.repository,
                license: manifest.license,
                icon: manifest.icon,
                keywords: manifest.keywords.unwrap_or_default(),
                category: manifest.category.unwrap_or_default(),
                min_app_version: manifest.min_app_version,
            },
            status,
            permissions: manifest.permissions.unwrap_or_default(),
            granted_permissions,
            installed_at: Utc::now(),
            updated_at: Utc::now(),
            config,
            error: None,
            path: Some(path.clone()),
        })
    }

    /// 下载并解压插件
    pub async fn download_and_extract(
        &self,
        url: &str,
        plugins_dir: &PathBuf,
    ) -> Result<PathBuf, PluginError> {
        // 下载插件包
        let response = reqwest::get(url)
            .await
            .map_err(|e| PluginError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(PluginError::NetworkError(
                format!("Failed to download plugin: {}", response.status())
            ));
        }

        let bytes = response.bytes()
            .await
            .map_err(|e| PluginError::NetworkError(e.to_string()))?;

        // 创建临时目录
        let temp_dir = std::env::temp_dir().join(format!("omnibox_plugin_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir)
            .map_err(|e| PluginError::IoError(e.to_string()))?;

        // 保存并解压
        let archive_path = temp_dir.join("plugin.zip");
        std::fs::write(&archive_path, &bytes)
            .map_err(|e| PluginError::IoError(e.to_string()))?;

        // 解压 ZIP
        let file = std::fs::File::open(&archive_path)
            .map_err(|e| PluginError::IoError(e.to_string()))?;
        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| PluginError::IoError(e.to_string()))?;

        // 读取 manifest 获取插件 ID
        let manifest_entry = archive.by_name("manifest.json")
            .map_err(|_| PluginError::InvalidManifest("manifest.json not found in archive".to_string()))?;
        
        let manifest: PluginManifest = serde_json::from_reader(manifest_entry)
            .map_err(|e| PluginError::InvalidManifest(e.to_string()))?;

        // 目标目录
        let plugin_dir = plugins_dir.join(&manifest.id);
        if plugin_dir.exists() {
            std::fs::remove_dir_all(&plugin_dir)
                .map_err(|e| PluginError::IoError(e.to_string()))?;
        }
        std::fs::create_dir_all(&plugin_dir)
            .map_err(|e| PluginError::IoError(e.to_string()))?;

        // 重新打开归档文件进行解压
        let file = std::fs::File::open(&archive_path)
            .map_err(|e| PluginError::IoError(e.to_string()))?;
        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| PluginError::IoError(e.to_string()))?;

        // 解压所有文件
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| PluginError::IoError(e.to_string()))?;
            
            let outpath = plugin_dir.join(file.name());

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)
                    .map_err(|e| PluginError::IoError(e.to_string()))?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)
                            .map_err(|e| PluginError::IoError(e.to_string()))?;
                    }
                }
                let mut outfile = std::fs::File::create(&outpath)
                    .map_err(|e| PluginError::IoError(e.to_string()))?;
                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| PluginError::IoError(e.to_string()))?;
            }
        }

        // 清理临时目录
        let _ = std::fs::remove_dir_all(&temp_dir);

        Ok(plugin_dir)
    }

    /// 验证插件完整性
    pub fn validate(&self, path: &PathBuf) -> Result<(), PluginError> {
        let manifest_path = path.join("manifest.json");
        if !manifest_path.exists() {
            return Err(PluginError::InvalidManifest("manifest.json not found".to_string()));
        }

        let manifest_content = std::fs::read_to_string(&manifest_path)
            .map_err(|e| PluginError::IoError(e.to_string()))?;

        let manifest: PluginManifest = serde_json::from_str(&manifest_content)
            .map_err(|e| PluginError::InvalidManifest(e.to_string()))?;

        // 验证必要字段
        if manifest.id.is_empty() {
            return Err(PluginError::InvalidManifest("Plugin ID is required".to_string()));
        }
        if manifest.name.is_empty() {
            return Err(PluginError::InvalidManifest("Plugin name is required".to_string()));
        }
        if manifest.version.is_empty() {
            return Err(PluginError::InvalidManifest("Plugin version is required".to_string()));
        }

        // 验证主入口文件
        let main_file = path.join(&manifest.main.unwrap_or_else(|| "index.js".to_string()));
        if !main_file.exists() {
            return Err(PluginError::InvalidManifest(
                format!("Main entry file not found: {:?}", main_file)
            ));
        }

        Ok(())
    }
}

/// 插件清单文件结构
#[derive(Debug, serde::Deserialize)]
struct PluginManifest {
    id: String,
    name: String,
    version: String,
    description: String,
    author: String,
    #[serde(default)]
    homepage: Option<String>,
    #[serde(default)]
    repository: Option<String>,
    #[serde(default)]
    license: Option<String>,
    #[serde(default)]
    icon: Option<String>,
    #[serde(default)]
    keywords: Option<Vec<String>>,
    #[serde(default)]
    category: Option<super::PluginCategory>,
    #[serde(default)]
    min_app_version: Option<String>,
    #[serde(default)]
    main: Option<String>,
    #[serde(default)]
    permissions: Option<Vec<PluginPermission>>,
}

/// 插件状态文件结构
#[derive(Debug, Default, serde::Deserialize)]
struct PluginState {
    #[serde(default)]
    status: PluginStatus,
    #[serde(default)]
    granted_permissions: Vec<PluginPermission>,
    #[serde(default)]
    config: std::collections::HashMap<String, serde_json::Value>,
}
