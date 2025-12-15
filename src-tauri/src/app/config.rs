use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub features: FeaturesConfig,
    pub appearance: AppearanceConfig,
    pub shortcuts: ShortcutsConfig,
    pub indexer: IndexerConfig,
    pub clipboard: ClipboardConfig,
    pub screenshot: ScreenshotConfig,
    pub ai: AIConfig,
    pub web_search: WebSearchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub language: String,
    pub auto_start: bool,
    pub check_updates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub file_search: bool,
    pub app_search: bool,
    pub calculator: bool,
    pub web_search: bool,
    pub clipboard: bool,
    pub screenshot: bool,
    pub ai: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub theme: String,
    pub accent_color: String,
    pub transparency: f32,
    pub window_radius: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutsConfig {
    pub main: String,
    pub clipboard: String,
    pub screenshot: String,
    pub ai_chat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexerConfig {
    pub enabled: bool,
    pub index_paths: Vec<PathBuf>,
    pub exclude_paths: Vec<PathBuf>,
    pub file_types: Vec<String>,
    pub max_file_size: u64,
    pub index_hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardConfig {
    pub enabled: bool,
    pub history_limit: usize,
    pub retention_days: usize,
    pub filter_sensitive: bool,
    pub exclude_apps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotConfig {
    pub format: String,
    pub quality: u8,
    pub save_dir: PathBuf,
    pub auto_save: bool,

    #[serde(default)]
    pub ocr_auto_copy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub provider: String,
    pub api_key: String,
    pub api_url: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchConfig {
    pub default_engine: String,
    pub engines: Vec<SearchEngine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngine {
    pub name: String,
    pub keyword: String,
    pub url: String,
    pub icon: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                language: "en".to_string(),
                auto_start: false,
                check_updates: true,
            },
            features: FeaturesConfig {
                file_search: true,
                app_search: true,
                calculator: true,
                web_search: true,
                clipboard: true,
                screenshot: true,
                ai: false,
            },
            appearance: AppearanceConfig {
                theme: "auto".to_string(),
                accent_color: "#007AFF".to_string(),
                transparency: 0.95,
                window_radius: 8,
            },
            shortcuts: ShortcutsConfig {
                main: "CommandOrControl+Space".to_string(),
                clipboard: "CommandOrControl+Shift+V".to_string(),
                screenshot: "CommandOrControl+Shift+S".to_string(),
                ai_chat: "CommandOrControl+Shift+A".to_string(),
            },
            indexer: IndexerConfig {
                enabled: true,
                index_paths: vec![],
                exclude_paths: vec![],
                file_types: vec![],
                max_file_size: 100 * 1024 * 1024, // 100MB
                index_hidden: false,
            },
            clipboard: ClipboardConfig {
                enabled: true,
                history_limit: 1000,
                retention_days: 30,
                filter_sensitive: true,
                exclude_apps: vec![],
            },
            screenshot: ScreenshotConfig {
                format: "png".to_string(),
                quality: 90,
                save_dir: PathBuf::new(),
                auto_save: false,
                ocr_auto_copy: false,
            },
            ai: AIConfig {
                provider: "openai".to_string(),
                api_key: String::new(),
                api_url: String::new(),
                model: "gpt-4".to_string(),
                temperature: 0.7,
                max_tokens: 2000,
            },
            web_search: WebSearchConfig {
                default_engine: "google".to_string(),
                engines: vec![
                    SearchEngine {
                        name: "Google".to_string(),
                        keyword: "gg".to_string(),
                        url: "https://www.google.com/search?q={query}".to_string(),
                        icon: None,
                    },
                    SearchEngine {
                        name: "Baidu".to_string(),
                        keyword: "bd".to_string(),
                        url: "https://www.baidu.com/s?wd={query}".to_string(),
                        icon: None,
                    },
                    SearchEngine {
                        name: "GitHub".to_string(),
                        keyword: "gh".to_string(),
                        url: "https://github.com/search?q={query}".to_string(),
                        icon: None,
                    },
                ],
            },
        }
    }
}
