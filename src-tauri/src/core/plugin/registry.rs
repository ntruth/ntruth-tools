//! Plugin Registry
//! 插件注册表 - 与插件市场交互

use super::{
    MarketplaceFilter, MarketplacePlugin, MarketplaceResponse, 
    PluginUpdateInfo, PluginError, PluginMetadata, PluginCategory
};
use chrono::Utc;

/// 插件市场 API URL（可配置）
const MARKETPLACE_API_URL: &str = "https://plugins.omnibox.app/api/v1";

/// 插件注册表
pub struct PluginRegistry {
    /// HTTP 客户端
    client: reqwest::Client,
    /// API 基础 URL
    api_url: String,
}

impl PluginRegistry {
    /// 创建新的注册表
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            api_url: MARKETPLACE_API_URL.to_string(),
        }
    }

    /// 设置自定义 API URL
    pub fn with_api_url(mut self, url: &str) -> Self {
        self.api_url = url.to_string();
        self
    }

    /// 搜索插件市场
    pub async fn search(&self, filter: MarketplaceFilter) -> Result<MarketplaceResponse, PluginError> {
        // TODO: 实际调用 API
        // 目前返回模拟数据
        let mock_plugins = self.get_mock_plugins();
        
        // 应用筛选
        let mut filtered: Vec<_> = mock_plugins.into_iter()
            .filter(|p| {
                // 分类筛选
                if let Some(ref cat) = filter.category {
                    if &p.metadata.category != cat {
                        return false;
                    }
                }
                // 搜索筛选
                if let Some(ref search) = filter.search {
                    let search_lower = search.to_lowercase();
                    if !p.metadata.name.to_lowercase().contains(&search_lower)
                        && !p.metadata.description.to_lowercase().contains(&search_lower)
                        && !p.metadata.keywords.iter().any(|k| k.to_lowercase().contains(&search_lower))
                    {
                        return false;
                    }
                }
                true
            })
            .collect();

        // 排序
        match filter.sort.as_str() {
            "popular" => filtered.sort_by(|a, b| b.downloads.cmp(&a.downloads)),
            "newest" => filtered.sort_by(|a, b| b.published_at.cmp(&a.published_at)),
            "updated" => filtered.sort_by(|a, b| b.last_updated.cmp(&a.last_updated)),
            "rating" => filtered.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal)),
            _ => {}
        }

        // 分页
        let total = filtered.len() as u64;
        let start = ((filter.page - 1) * filter.page_size) as usize;
        let end = (start + filter.page_size as usize).min(filtered.len());
        let plugins = filtered[start..end].to_vec();

        Ok(MarketplaceResponse {
            plugins,
            total,
            page: filter.page,
            page_size: filter.page_size,
        })
    }

    /// 获取单个插件信息
    pub async fn get_plugin(&self, plugin_id: &str) -> Result<MarketplacePlugin, PluginError> {
        // TODO: 实际调用 API
        let mock_plugins = self.get_mock_plugins();
        mock_plugins.into_iter()
            .find(|p| p.metadata.id == plugin_id)
            .ok_or_else(|| PluginError::NotFound(plugin_id.to_string()))
    }

    /// 获取插件下载 URL
    pub async fn get_download_url(&self, plugin_id: &str, version: Option<&str>) -> Result<String, PluginError> {
        // TODO: 实际调用 API
        let version = version.unwrap_or("latest");
        Ok(format!("{}/plugins/{}/download/{}", self.api_url, plugin_id, version))
    }

    /// 检查插件更新
    pub async fn check_update(&self, plugin_id: &str, current_version: &str) -> Result<PluginUpdateInfo, PluginError> {
        // TODO: 实际调用 API
        let plugin = self.get_plugin(plugin_id).await?;
        
        Ok(PluginUpdateInfo {
            plugin_id: plugin_id.to_string(),
            current_version: current_version.to_string(),
            latest_version: plugin.metadata.version,
            changelog: plugin.changelog,
            breaking: false,
        })
    }

    /// 获取推荐插件
    pub async fn get_featured(&self) -> Result<Vec<MarketplacePlugin>, PluginError> {
        let mock_plugins = self.get_mock_plugins();
        Ok(mock_plugins.into_iter().take(6).collect())
    }

    /// 获取分类列表
    pub fn get_categories(&self) -> Vec<CategoryInfo> {
        vec![
            CategoryInfo {
                id: PluginCategory::Search,
                name: "Search Providers".to_string(),
                description: "Add new search sources and providers".to_string(),
                icon: "🔍".to_string(),
            },
            CategoryInfo {
                id: PluginCategory::Action,
                name: "Action Handlers".to_string(),
                description: "Custom actions for search results".to_string(),
                icon: "⚡".to_string(),
            },
            CategoryInfo {
                id: PluginCategory::Workflow,
                name: "Workflow Nodes".to_string(),
                description: "Extend workflow capabilities".to_string(),
                icon: "🔄".to_string(),
            },
            CategoryInfo {
                id: PluginCategory::Theme,
                name: "Themes".to_string(),
                description: "Custom themes and appearances".to_string(),
                icon: "🎨".to_string(),
            },
            CategoryInfo {
                id: PluginCategory::Integration,
                name: "Integrations".to_string(),
                description: "Third-party service integrations".to_string(),
                icon: "🔗".to_string(),
            },
            CategoryInfo {
                id: PluginCategory::Utility,
                name: "Utilities".to_string(),
                description: "Useful tools and utilities".to_string(),
                icon: "🛠".to_string(),
            },
        ]
    }

    /// 获取模拟插件数据
    fn get_mock_plugins(&self) -> Vec<MarketplacePlugin> {
        vec![
            MarketplacePlugin {
                metadata: PluginMetadata {
                    id: "github-search".to_string(),
                    name: "GitHub Search".to_string(),
                    version: "1.2.0".to_string(),
                    description: "Search GitHub repositories, issues, and pull requests directly from OmniBox".to_string(),
                    author: "OmniBox Team".to_string(),
                    homepage: Some("https://github.com/omnibox/github-search".to_string()),
                    repository: Some("https://github.com/omnibox/github-search".to_string()),
                    license: Some("MIT".to_string()),
                    icon: Some("🐙".to_string()),
                    keywords: vec!["github".to_string(), "search".to_string(), "repository".to_string(), "code".to_string()],
                    category: PluginCategory::Search,
                    min_app_version: Some("0.5.0".to_string()),
                },
                downloads: 15420,
                rating: 4.8,
                rating_count: 128,
                last_updated: Utc::now(),
                published_at: Utc::now(),
                screenshots: vec![],
                readme: Some("# GitHub Search Plugin\n\nSearch GitHub directly from OmniBox.\n\n## Usage\n\nType `gh` followed by your search query.".to_string()),
                changelog: Some("## 1.2.0\n- Added support for searching pull requests\n- Improved search speed".to_string()),
                installed: false,
                installed_version: None,
                has_update: false,
            },
            MarketplacePlugin {
                metadata: PluginMetadata {
                    id: "notion-search".to_string(),
                    name: "Notion Search".to_string(),
                    version: "1.0.0".to_string(),
                    description: "Search your Notion workspace pages and databases".to_string(),
                    author: "Community".to_string(),
                    homepage: None,
                    repository: Some("https://github.com/community/notion-search".to_string()),
                    license: Some("MIT".to_string()),
                    icon: Some("📝".to_string()),
                    keywords: vec!["notion".to_string(), "search".to_string(), "notes".to_string(), "workspace".to_string()],
                    category: PluginCategory::Search,
                    min_app_version: Some("0.5.0".to_string()),
                },
                downloads: 8320,
                rating: 4.5,
                rating_count: 64,
                last_updated: Utc::now(),
                published_at: Utc::now(),
                screenshots: vec![],
                readme: Some("# Notion Search Plugin\n\nIntegrate Notion with OmniBox.".to_string()),
                changelog: None,
                installed: false,
                installed_version: None,
                has_update: false,
            },
            MarketplacePlugin {
                metadata: PluginMetadata {
                    id: "clipboard-formatter".to_string(),
                    name: "Clipboard Formatter".to_string(),
                    version: "2.1.0".to_string(),
                    description: "Format and transform clipboard content with various actions".to_string(),
                    author: "OmniBox Team".to_string(),
                    homepage: None,
                    repository: None,
                    license: Some("MIT".to_string()),
                    icon: Some("📋".to_string()),
                    keywords: vec!["clipboard".to_string(), "format".to_string(), "transform".to_string()],
                    category: PluginCategory::Action,
                    min_app_version: Some("0.5.0".to_string()),
                },
                downloads: 12500,
                rating: 4.7,
                rating_count: 95,
                last_updated: Utc::now(),
                published_at: Utc::now(),
                screenshots: vec![],
                readme: Some("# Clipboard Formatter\n\nTransform your clipboard content.".to_string()),
                changelog: Some("## 2.1.0\n- Added JSON formatting\n- Added base64 encode/decode".to_string()),
                installed: false,
                installed_version: None,
                has_update: false,
            },
            MarketplacePlugin {
                metadata: PluginMetadata {
                    id: "http-request-node".to_string(),
                    name: "HTTP Request Node".to_string(),
                    version: "1.0.0".to_string(),
                    description: "Make HTTP requests in your workflows".to_string(),
                    author: "OmniBox Team".to_string(),
                    homepage: None,
                    repository: None,
                    license: Some("MIT".to_string()),
                    icon: Some("🌐".to_string()),
                    keywords: vec!["http".to_string(), "request".to_string(), "api".to_string(), "workflow".to_string()],
                    category: PluginCategory::Workflow,
                    min_app_version: Some("0.8.0".to_string()),
                },
                downloads: 5600,
                rating: 4.6,
                rating_count: 42,
                last_updated: Utc::now(),
                published_at: Utc::now(),
                screenshots: vec![],
                readme: Some("# HTTP Request Node\n\nAdd HTTP requests to your workflows.".to_string()),
                changelog: None,
                installed: false,
                installed_version: None,
                has_update: false,
            },
            MarketplacePlugin {
                metadata: PluginMetadata {
                    id: "slack-integration".to_string(),
                    name: "Slack Integration".to_string(),
                    version: "1.1.0".to_string(),
                    description: "Search Slack messages and send messages from OmniBox".to_string(),
                    author: "Community".to_string(),
                    homepage: None,
                    repository: None,
                    license: Some("MIT".to_string()),
                    icon: Some("💬".to_string()),
                    keywords: vec!["slack".to_string(), "chat".to_string(), "messages".to_string(), "integration".to_string()],
                    category: PluginCategory::Integration,
                    min_app_version: Some("0.5.0".to_string()),
                },
                downloads: 9800,
                rating: 4.4,
                rating_count: 78,
                last_updated: Utc::now(),
                published_at: Utc::now(),
                screenshots: vec![],
                readme: Some("# Slack Integration\n\nIntegrate Slack with OmniBox.".to_string()),
                changelog: None,
                installed: false,
                installed_version: None,
                has_update: false,
            },
            MarketplacePlugin {
                metadata: PluginMetadata {
                    id: "dracula-theme".to_string(),
                    name: "Dracula Theme".to_string(),
                    version: "1.0.0".to_string(),
                    description: "Dark theme inspired by Dracula color scheme".to_string(),
                    author: "Community".to_string(),
                    homepage: None,
                    repository: None,
                    license: Some("MIT".to_string()),
                    icon: Some("🧛".to_string()),
                    keywords: vec!["theme".to_string(), "dark".to_string(), "dracula".to_string()],
                    category: PluginCategory::Theme,
                    min_app_version: Some("0.5.0".to_string()),
                },
                downloads: 18200,
                rating: 4.9,
                rating_count: 156,
                last_updated: Utc::now(),
                published_at: Utc::now(),
                screenshots: vec![],
                readme: Some("# Dracula Theme\n\nBeautiful dark theme for OmniBox.".to_string()),
                changelog: None,
                installed: false,
                installed_version: None,
                has_update: false,
            },
            MarketplacePlugin {
                metadata: PluginMetadata {
                    id: "color-picker".to_string(),
                    name: "Color Picker".to_string(),
                    version: "1.3.0".to_string(),
                    description: "Pick colors from screen and convert between formats".to_string(),
                    author: "OmniBox Team".to_string(),
                    homepage: None,
                    repository: None,
                    license: Some("MIT".to_string()),
                    icon: Some("🎨".to_string()),
                    keywords: vec!["color".to_string(), "picker".to_string(), "hex".to_string(), "rgb".to_string()],
                    category: PluginCategory::Utility,
                    min_app_version: Some("0.5.0".to_string()),
                },
                downloads: 11300,
                rating: 4.6,
                rating_count: 89,
                last_updated: Utc::now(),
                published_at: Utc::now(),
                screenshots: vec![],
                readme: Some("# Color Picker\n\nPick colors and convert between formats.".to_string()),
                changelog: Some("## 1.3.0\n- Added HSL support\n- Added color history".to_string()),
                installed: false,
                installed_version: None,
                has_update: false,
            },
            MarketplacePlugin {
                metadata: PluginMetadata {
                    id: "translator".to_string(),
                    name: "Translator".to_string(),
                    version: "2.0.0".to_string(),
                    description: "Translate text between languages using multiple providers".to_string(),
                    author: "Community".to_string(),
                    homepage: None,
                    repository: None,
                    license: Some("MIT".to_string()),
                    icon: Some("🌍".to_string()),
                    keywords: vec!["translate".to_string(), "language".to_string(), "i18n".to_string()],
                    category: PluginCategory::Utility,
                    min_app_version: Some("0.5.0".to_string()),
                },
                downloads: 14700,
                rating: 4.7,
                rating_count: 112,
                last_updated: Utc::now(),
                published_at: Utc::now(),
                screenshots: vec![],
                readme: Some("# Translator\n\nTranslate text between 100+ languages.".to_string()),
                changelog: Some("## 2.0.0\n- Added DeepL support\n- Added auto-detect language".to_string()),
                installed: false,
                installed_version: None,
                has_update: false,
            },
        ]
    }
}

/// 分类信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct CategoryInfo {
    pub id: PluginCategory,
    pub name: String,
    pub description: String,
    pub icon: String,
}
