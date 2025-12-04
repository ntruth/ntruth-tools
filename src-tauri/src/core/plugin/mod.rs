//! Plugin System Module
//! 插件系统模块 - 支持 SearchProvider, ActionHandler, WorkflowNode 三种插件类型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

pub mod loader;
pub mod sandbox;
pub mod registry;

// Re-exports
pub use loader::PluginLoader;
pub use sandbox::PluginSandbox;
pub use registry::PluginRegistry;

/// 插件分类
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PluginCategory {
    Search,
    Action,
    Workflow,
    Theme,
    Integration,
    Utility,
    Other,
}

impl Default for PluginCategory {
    fn default() -> Self {
        Self::Other
    }
}

/// 插件权限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PluginPermission {
    #[serde(rename = "clipboard:read")]
    ClipboardRead,
    #[serde(rename = "clipboard:write")]
    ClipboardWrite,
    #[serde(rename = "fs:read")]
    FsRead,
    #[serde(rename = "fs:write")]
    FsWrite,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "shell")]
    Shell,
    #[serde(rename = "notification")]
    Notification,
    #[serde(rename = "system")]
    System,
}

/// 插件状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PluginStatus {
    Installed,
    Enabled,
    Disabled,
    Error,
    Updating,
}

impl Default for PluginStatus {
    fn default() -> Self {
        Self::Disabled
    }
}

/// 插件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub category: PluginCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_app_version: Option<String>,
}

/// 已安装的插件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledPlugin {
    pub metadata: PluginMetadata,
    pub status: PluginStatus,
    pub permissions: Vec<PluginPermission>,
    pub granted_permissions: Vec<PluginPermission>,
    pub installed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub config: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// 插件安装路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
}

/// 插件市场项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplacePlugin {
    pub metadata: PluginMetadata,
    pub downloads: u64,
    pub rating: f32,
    pub rating_count: u32,
    pub last_updated: DateTime<Utc>,
    pub published_at: DateTime<Utc>,
    #[serde(default)]
    pub screenshots: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changelog: Option<String>,
    #[serde(default)]
    pub installed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_version: Option<String>,
    #[serde(default)]
    pub has_update: bool,
}

/// 插件市场筛选条件
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarketplaceFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<PluginCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(default = "default_sort")]
    pub sort: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_sort() -> String {
    "popular".to_string()
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

/// 插件市场响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceResponse {
    pub plugins: Vec<MarketplacePlugin>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

/// 插件更新信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginUpdateInfo {
    pub plugin_id: String,
    pub current_version: String,
    pub latest_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changelog: Option<String>,
    #[serde(default)]
    pub breaking: bool,
}

/// 搜索结果（供插件使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSearchResult {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub action: PluginAction,
    #[serde(default)]
    pub score: f32,
}

/// 插件动作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginAction {
    #[serde(rename = "type")]
    pub action_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}

/// 工作流端口定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPort {
    pub name: String,
    #[serde(rename = "type")]
    pub port_type: String,
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 工作流节点定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNodeDefinition {
    #[serde(rename = "type")]
    pub node_type: String,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub inputs: Vec<WorkflowPort>,
    pub outputs: Vec<WorkflowPort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_schema: Option<serde_json::Value>,
}

/// 插件管理器
pub struct PluginManager {
    /// 已安装的插件
    plugins: Arc<RwLock<HashMap<String, InstalledPlugin>>>,
    /// 插件加载器
    loader: PluginLoader,
    /// 插件沙箱
    sandbox: PluginSandbox,
    /// 插件注册表
    pub registry: PluginRegistry,
    /// 插件目录
    plugins_dir: PathBuf,
}

impl PluginManager {
    /// 创建新的插件管理器
    pub fn new(plugins_dir: PathBuf) -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            loader: PluginLoader::new(),
            sandbox: PluginSandbox::new(),
            registry: PluginRegistry::new(),
            plugins_dir,
        }
    }

    /// 初始化插件系统
    pub async fn init(&self) -> Result<(), PluginError> {
        // 确保插件目录存在
        if !self.plugins_dir.exists() {
            std::fs::create_dir_all(&self.plugins_dir)
                .map_err(|e| PluginError::IoError(e.to_string()))?;
        }

        // 加载所有已安装的插件
        self.load_installed_plugins().await?;
        
        Ok(())
    }

    /// 加载所有已安装的插件
    async fn load_installed_plugins(&self) -> Result<(), PluginError> {
        let entries = std::fs::read_dir(&self.plugins_dir)
            .map_err(|e| PluginError::IoError(e.to_string()))?;

        for entry in entries {
            let entry = entry.map_err(|e| PluginError::IoError(e.to_string()))?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Err(e) = self.load_plugin_from_path(&path).await {
                    tracing::warn!("Failed to load plugin from {:?}: {}", path, e);
                }
            }
        }

        Ok(())
    }

    /// 从路径加载插件
    async fn load_plugin_from_path(&self, path: &PathBuf) -> Result<InstalledPlugin, PluginError> {
        let plugin = self.loader.load(path).await?;
        
        let mut plugins = self.plugins.write().await;
        plugins.insert(plugin.metadata.id.clone(), plugin.clone());
        
        Ok(plugin)
    }

    /// 获取所有已安装的插件
    pub async fn get_installed_plugins(&self) -> Vec<InstalledPlugin> {
        let plugins = self.plugins.read().await;
        plugins.values().cloned().collect()
    }

    /// 获取单个插件
    pub async fn get_plugin(&self, id: &str) -> Option<InstalledPlugin> {
        let plugins = self.plugins.read().await;
        plugins.get(id).cloned()
    }

    /// 安装插件
    pub async fn install_plugin(
        &self,
        plugin_id: &str,
        version: Option<&str>,
        permissions: Vec<PluginPermission>,
    ) -> Result<InstalledPlugin, PluginError> {
        // 从市场获取插件信息
        let marketplace_plugin = self.registry.get_plugin(plugin_id).await?;
        
        // 下载插件
        let download_url = self.registry.get_download_url(plugin_id, version).await?;
        let plugin_path = self.loader.download_and_extract(&download_url, &self.plugins_dir).await?;
        
        // 加载插件
        let mut plugin = self.load_plugin_from_path(&plugin_path).await?;
        
        // 设置权限
        plugin.granted_permissions = permissions;
        plugin.status = PluginStatus::Enabled;
        
        // 更新插件列表
        let mut plugins = self.plugins.write().await;
        plugins.insert(plugin.metadata.id.clone(), plugin.clone());
        
        // 保存插件状态
        self.save_plugin_state(&plugin).await?;
        
        Ok(plugin)
    }

    /// 卸载插件
    pub async fn uninstall_plugin(&self, plugin_id: &str) -> Result<(), PluginError> {
        let mut plugins = self.plugins.write().await;
        
        if let Some(plugin) = plugins.remove(plugin_id) {
            // 删除插件目录
            if let Some(path) = &plugin.path {
                std::fs::remove_dir_all(path)
                    .map_err(|e| PluginError::IoError(e.to_string()))?;
            }
        } else {
            return Err(PluginError::NotFound(plugin_id.to_string()));
        }
        
        Ok(())
    }

    /// 启用插件
    pub async fn enable_plugin(&self, plugin_id: &str) -> Result<(), PluginError> {
        let mut plugins = self.plugins.write().await;
        
        if let Some(plugin) = plugins.get_mut(plugin_id) {
            plugin.status = PluginStatus::Enabled;
            self.save_plugin_state(plugin).await?;
            Ok(())
        } else {
            Err(PluginError::NotFound(plugin_id.to_string()))
        }
    }

    /// 禁用插件
    pub async fn disable_plugin(&self, plugin_id: &str) -> Result<(), PluginError> {
        let mut plugins = self.plugins.write().await;
        
        if let Some(plugin) = plugins.get_mut(plugin_id) {
            plugin.status = PluginStatus::Disabled;
            self.save_plugin_state(plugin).await?;
            Ok(())
        } else {
            Err(PluginError::NotFound(plugin_id.to_string()))
        }
    }

    /// 更新插件
    pub async fn update_plugin(&self, plugin_id: &str) -> Result<InstalledPlugin, PluginError> {
        // 获取当前插件
        let current = self.get_plugin(plugin_id).await
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;
        
        // 保存当前权限
        let permissions = current.granted_permissions.clone();
        
        // 卸载旧版本
        self.uninstall_plugin(plugin_id).await?;
        
        // 安装新版本
        self.install_plugin(plugin_id, None, permissions).await
    }

    /// 检查插件更新
    pub async fn check_updates(&self) -> Result<Vec<PluginUpdateInfo>, PluginError> {
        let plugins = self.plugins.read().await;
        let mut updates = Vec::new();
        
        for plugin in plugins.values() {
            if let Ok(info) = self.registry.check_update(&plugin.metadata.id, &plugin.metadata.version).await {
                if info.latest_version != plugin.metadata.version {
                    updates.push(info);
                }
            }
        }
        
        Ok(updates)
    }

    /// 搜索市场插件
    pub async fn search_marketplace(&self, filter: MarketplaceFilter) -> Result<MarketplaceResponse, PluginError> {
        self.registry.search(filter).await
    }

    /// 授予插件权限
    pub async fn grant_permission(
        &self,
        plugin_id: &str,
        permission: PluginPermission,
    ) -> Result<(), PluginError> {
        let mut plugins = self.plugins.write().await;
        
        if let Some(plugin) = plugins.get_mut(plugin_id) {
            if !plugin.granted_permissions.contains(&permission) {
                plugin.granted_permissions.push(permission);
                self.save_plugin_state(plugin).await?;
            }
            Ok(())
        } else {
            Err(PluginError::NotFound(plugin_id.to_string()))
        }
    }

    /// 撤销插件权限
    pub async fn revoke_permission(
        &self,
        plugin_id: &str,
        permission: &PluginPermission,
    ) -> Result<(), PluginError> {
        let mut plugins = self.plugins.write().await;
        
        if let Some(plugin) = plugins.get_mut(plugin_id) {
            plugin.granted_permissions.retain(|p| p != permission);
            self.save_plugin_state(plugin).await?;
            Ok(())
        } else {
            Err(PluginError::NotFound(plugin_id.to_string()))
        }
    }

    /// 保存插件状态
    async fn save_plugin_state(&self, plugin: &InstalledPlugin) -> Result<(), PluginError> {
        let state_file = self.plugins_dir.join(format!("{}.state.json", plugin.metadata.id));
        let content = serde_json::to_string_pretty(plugin)
            .map_err(|e| PluginError::SerializationError(e.to_string()))?;
        
        std::fs::write(&state_file, content)
            .map_err(|e| PluginError::IoError(e.to_string()))?;
        
        Ok(())
    }

    /// 检查插件是否有权限
    pub async fn has_permission(&self, plugin_id: &str, permission: &PluginPermission) -> bool {
        let plugins = self.plugins.read().await;
        
        if let Some(plugin) = plugins.get(plugin_id) {
            plugin.granted_permissions.contains(permission)
        } else {
            false
        }
    }

    /// 执行插件搜索（如果是搜索类插件）
    pub async fn execute_search(
        &self,
        plugin_id: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<PluginSearchResult>, PluginError> {
        let plugins = self.plugins.read().await;
        
        let plugin = plugins.get(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;
        
        if plugin.status != PluginStatus::Enabled {
            return Err(PluginError::PluginDisabled(plugin_id.to_string()));
        }
        
        // 在沙箱中执行搜索
        self.sandbox.execute_search(plugin, query, limit).await
    }

    /// 执行插件动作
    pub async fn execute_action(
        &self,
        plugin_id: &str,
        action: &PluginAction,
    ) -> Result<(), PluginError> {
        let plugins = self.plugins.read().await;
        
        let plugin = plugins.get(plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))?;
        
        if plugin.status != PluginStatus::Enabled {
            return Err(PluginError::PluginDisabled(plugin_id.to_string()));
        }
        
        // 在沙箱中执行动作
        self.sandbox.execute_action(plugin, action).await
    }
}

/// 插件错误类型
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    NotFound(String),
    
    #[error("Plugin is disabled: {0}")]
    PluginDisabled(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Invalid plugin manifest: {0}")]
    InvalidManifest(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Sandbox error: {0}")]
    SandboxError(String),
    
    #[error("Version mismatch: {0}")]
    VersionMismatch(String),
}
