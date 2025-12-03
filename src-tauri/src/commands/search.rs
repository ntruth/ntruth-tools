use crate::app::{error::AppResult, state::AppState};
use crate::core::parser::{Parser, ParseResult, Calculator};
use crate::core::indexer::FileEntry;
use serde::{Deserialize, Serialize};
use tauri::State;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub r#type: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub icon: Option<String>,
    pub path: Option<String>,
    pub action: SearchAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAction {
    pub r#type: String,
    pub payload: Option<String>,
}

/// Get app icon as base64 data URL (cached)
#[cfg(target_os = "macos")]
async fn get_app_icon(app_path: &Path, state: &State<'_, AppState>) -> Option<String> {
    // Try to get from cache first
    if let Some(cached) = state.icon_cache.get_icon(app_path).await {
        return Some(format!("data:image/png;base64,{}", cached));
    }
    
    // Extract and cache the icon
    match state.icon_cache.extract_and_cache_icon(app_path).await {
        Ok(base64_data) => Some(format!("data:image/png;base64,{}", base64_data)),
        Err(e) => {
            tracing::debug!("Failed to get icon for {:?}: {}", app_path, e);
            None
        }
    }
}

#[cfg(not(target_os = "macos"))]
async fn get_app_icon(_app_path: &Path, _state: &State<'_, AppState>) -> Option<String> {
    None
}

/// Search command
#[tauri::command]
pub async fn search(
    query: String,
    state: State<'_, AppState>,
) -> AppResult<Vec<SearchResult>> {
    let parser = Parser::new();
    let parse_result = parser.parse(&query);

    let results = match parse_result {
        ParseResult::Empty => Vec::new(),
        
        ParseResult::FileOrApp(q) => {
            // Search files using indexer
            let file_entries = state.indexer.search(&q).await;
            
            // Convert FileEntry to SearchResult with async icon loading
            let mut results = Vec::new();
            for entry in file_entries {
                // Check if this is an app (.app bundle on macOS)
                let is_app = entry.path.extension()
                    .map(|e| e == "app")
                    .unwrap_or(false);
                
                let (result_type, icon) = if is_app {
                    // Get app icon
                    let icon = get_app_icon(&entry.path, &state).await
                        .unwrap_or_else(|| "🚀".to_string());
                    ("app".to_string(), Some(icon))
                } else {
                    // Determine icon based on file extension
                    let icon = match entry.path.extension().and_then(|e| e.to_str()) {
                        Some("pdf") => "📄",
                        Some("doc") | Some("docx") => "📝",
                        Some("xls") | Some("xlsx") => "📊",
                        Some("ppt") | Some("pptx") => "📽️",
                        Some("txt") | Some("md") => "📃",
                        Some("jpg") | Some("jpeg") | Some("png") | Some("gif") => "🖼️",
                        Some("mp3") | Some("wav") | Some("m4a") => "🎵",
                        Some("mp4") | Some("mov") | Some("avi") => "🎬",
                        Some("zip") | Some("rar") | Some("7z") => "📦",
                        Some("html") | Some("css") | Some("js") => "💻",
                        Some("rs") | Some("py") | Some("ts") => "🔧",
                        _ => "📁",
                    };
                    ("file".to_string(), Some(icon.to_string()))
                };
                
                // For apps, show display name if available (e.g., "微信" instead of "WeChat")
                let title = if is_app {
                    entry.display_name.as_ref().unwrap_or(&entry.name).clone()
                } else {
                    entry.name.clone()
                };
                
                // Subtitle shows both names if different
                let subtitle = if is_app && entry.display_name.is_some() && entry.display_name.as_ref() != Some(&entry.name) {
                    Some(format!("{} - {}", entry.name, entry.path.to_string_lossy()))
                } else {
                    Some(entry.path.to_string_lossy().to_string())
                };
                
                results.push(SearchResult {
                    id: entry.id.to_string(),
                    r#type: result_type,
                    title,
                    subtitle,
                    icon,
                    path: Some(entry.path.to_string_lossy().to_string()),
                    action: SearchAction {
                        r#type: "open".to_string(),
                        payload: Some(entry.path.to_string_lossy().to_string()),
                    },
                });
            }
            results
        }
        
        ParseResult::Calculator(expr) => {
            // Evaluate calculator expression using new Calculator
            match evaluate_expression(&expr) {
                Ok(result) => vec![SearchResult {
                    id: "calc".to_string(),
                    r#type: "calculator".to_string(),
                    title: result.clone(),
                    subtitle: Some(format!("= {}", expr)),
                    icon: None,
                    path: None,
                    action: SearchAction {
                        r#type: "copy".to_string(),
                        payload: Some(result),
                    },
                }],
                Err(e) => vec![SearchResult {
                    id: "calc-error".to_string(),
                    r#type: "calculator".to_string(),
                    title: "Error".to_string(),
                    subtitle: Some(e),
                    icon: None,
                    path: None,
                    action: SearchAction {
                        r#type: "none".to_string(),
                        payload: None,
                    },
                }],
            }
        }
        
        ParseResult::WebSearch { engine, query, url } => {
            vec![SearchResult {
                id: "web-search".to_string(),
                r#type: "web-search".to_string(),
                title: format!("Search {} for '{}'", engine, query),
                subtitle: Some(url.clone()),
                icon: None,
                path: None,
                action: SearchAction {
                    r#type: "web-search".to_string(),
                    payload: Some(url),
                },
            }]
        }
        
        ParseResult::Url(url) => {
            vec![SearchResult {
                id: "url".to_string(),
                r#type: "web-search".to_string(),
                title: "Open URL".to_string(),
                subtitle: Some(url.clone()),
                icon: None,
                path: None,
                action: SearchAction {
                    r#type: "web-search".to_string(),
                    payload: Some(url),
                },
            }]
        }
        
        ParseResult::AI(query) => {
            vec![SearchResult {
                id: "ai".to_string(),
                r#type: "ai".to_string(),
                title: "Ask AI".to_string(),
                subtitle: Some(query.clone()),
                icon: None,
                path: None,
                action: SearchAction {
                    r#type: "ai-query".to_string(),
                    payload: Some(query),
                },
            }]
        }
        
        ParseResult::Clipboard(query) => {
            vec![SearchResult {
                id: "clipboard".to_string(),
                r#type: "clipboard".to_string(),
                title: "Search clipboard".to_string(),
                subtitle: Some(query.clone()),
                icon: None,
                path: None,
                action: SearchAction {
                    r#type: "clipboard".to_string(),
                    payload: Some(query),
                },
            }]
        }
        
        ParseResult::Bookmark(query) => {
            vec![SearchResult {
                id: "bookmark".to_string(),
                r#type: "web-search".to_string(),
                title: "Search bookmarks".to_string(),
                subtitle: Some(query.clone()),
                icon: None,
                path: None,
                action: SearchAction {
                    r#type: "bookmark".to_string(),
                    payload: Some(query),
                },
            }]
        }
        
        ParseResult::Command(cmd) => {
            vec![SearchResult {
                id: "command".to_string(),
                r#type: "command".to_string(),
                title: "Execute command".to_string(),
                subtitle: Some(cmd.clone()),
                icon: None,
                path: None,
                action: SearchAction {
                    r#type: "execute".to_string(),
                    payload: Some(cmd),
                },
            }]
        }
    };

    Ok(results)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculatorResult {
    pub expression: String,
    pub result: String,
    pub r#type: String,
}

/// Calculator command
#[tauri::command]
pub async fn calculate(
    expression: String,
    _state: State<'_, AppState>,
) -> AppResult<CalculatorResult> {
    let result = match evaluate_expression(&expression) {
        Ok(value) => value,
        Err(e) => e,
    };

    Ok(CalculatorResult {
        expression: expression.clone(),
        result,
        r#type: "number".to_string(),
    })
}

/// Evaluate a mathematical expression with unit conversion support
fn evaluate_expression(expr: &str) -> Result<String, String> {
    let calc = Calculator::new();
    
    match calc.evaluate(expr) {
        Ok(value) => Ok(calc.format_result(value)),
        Err(e) => Err(e),
    }
}
