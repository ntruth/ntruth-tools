//! Plugin Commands
//! 插件相关的 Tauri 命令

use crate::app::state::AppState;
use crate::core::plugin::{
    InstalledPlugin, MarketplacePlugin, MarketplaceFilter, MarketplaceResponse,
    PluginUpdateInfo, PluginPermission, PluginError
};
use tauri::State;

/// 获取所有已安装的插件
#[tauri::command]
pub async fn get_installed_plugins(
    state: State<'_, AppState>,
) -> Result<Vec<InstalledPlugin>, String> {
    let plugin_manager = state.plugin_manager.read().await;
    Ok(plugin_manager.get_installed_plugins().await)
}

/// 获取单个插件信息
#[tauri::command]
pub async fn get_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
) -> Result<Option<InstalledPlugin>, String> {
    let plugin_manager = state.plugin_manager.read().await;
    Ok(plugin_manager.get_plugin(&plugin_id).await)
}

/// 安装插件
#[tauri::command]
pub async fn install_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
    version: Option<String>,
    permissions: Vec<PluginPermission>,
) -> Result<InstalledPlugin, String> {
    let plugin_manager = state.plugin_manager.read().await;
    plugin_manager
        .install_plugin(&plugin_id, version.as_deref(), permissions)
        .await
        .map_err(|e| e.to_string())
}

/// 卸载插件
#[tauri::command]
pub async fn uninstall_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
) -> Result<(), String> {
    let plugin_manager = state.plugin_manager.read().await;
    plugin_manager
        .uninstall_plugin(&plugin_id)
        .await
        .map_err(|e| e.to_string())
}

/// 启用插件
#[tauri::command]
pub async fn enable_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
) -> Result<(), String> {
    let plugin_manager = state.plugin_manager.read().await;
    plugin_manager
        .enable_plugin(&plugin_id)
        .await
        .map_err(|e| e.to_string())
}

/// 禁用插件
#[tauri::command]
pub async fn disable_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
) -> Result<(), String> {
    let plugin_manager = state.plugin_manager.read().await;
    plugin_manager
        .disable_plugin(&plugin_id)
        .await
        .map_err(|e| e.to_string())
}

/// 更新插件
#[tauri::command]
pub async fn update_plugin(
    state: State<'_, AppState>,
    plugin_id: String,
) -> Result<InstalledPlugin, String> {
    let plugin_manager = state.plugin_manager.read().await;
    plugin_manager
        .update_plugin(&plugin_id)
        .await
        .map_err(|e| e.to_string())
}

/// 检查插件更新
#[tauri::command]
pub async fn check_plugin_updates(
    state: State<'_, AppState>,
) -> Result<Vec<PluginUpdateInfo>, String> {
    let plugin_manager = state.plugin_manager.read().await;
    plugin_manager
        .check_updates()
        .await
        .map_err(|e| e.to_string())
}

/// 搜索插件市场
#[tauri::command]
pub async fn search_marketplace(
    state: State<'_, AppState>,
    filter: MarketplaceFilter,
) -> Result<MarketplaceResponse, String> {
    let plugin_manager = state.plugin_manager.read().await;
    plugin_manager
        .search_marketplace(filter)
        .await
        .map_err(|e| e.to_string())
}

/// 获取推荐插件
#[tauri::command]
pub async fn get_featured_plugins(
    state: State<'_, AppState>,
) -> Result<Vec<MarketplacePlugin>, String> {
    let plugin_manager = state.plugin_manager.read().await;
    plugin_manager.registry
        .get_featured()
        .await
        .map_err(|e| e.to_string())
}

/// 授予插件权限
#[tauri::command]
pub async fn grant_plugin_permission(
    state: State<'_, AppState>,
    plugin_id: String,
    permission: PluginPermission,
) -> Result<(), String> {
    let plugin_manager = state.plugin_manager.read().await;
    plugin_manager
        .grant_permission(&plugin_id, permission)
        .await
        .map_err(|e| e.to_string())
}

/// 撤销插件权限
#[tauri::command]
pub async fn revoke_plugin_permission(
    state: State<'_, AppState>,
    plugin_id: String,
    permission: PluginPermission,
) -> Result<(), String> {
    let plugin_manager = state.plugin_manager.read().await;
    plugin_manager
        .revoke_permission(&plugin_id, &permission)
        .await
        .map_err(|e| e.to_string())
}
