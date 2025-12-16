use crate::app::{error::AppResult, state::AppState};
use crate::core::parser::{Parser, ParseResult, Calculator};
use base64::Engine;
use serde::{Deserialize, Serialize};
use tauri::State;
use std::path::Path;
use std::collections::HashSet;

#[cfg(windows)]
use crate::app_indexer::AppIndexer;
#[cfg(windows)]
use crate::everything_service;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub r#type: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub icon: Option<String>,
    pub path: Option<String>,
    pub category: String,  // "Application" or "File" for grouping
    pub score: i32,        // Relevance score for debugging
    pub action: SearchAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAction {
    pub r#type: String,
    pub payload: Option<String>,
}

/// Get app icon as base64 data URL (cached)
#[cfg(any(target_os = "macos", target_os = "windows"))]
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

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
async fn get_app_icon(_app_path: &Path, _state: &State<'_, AppState>) -> Option<String> {
    None
}

/// Get Windows system (Explorer) icon as base64 data URL (cached)
#[cfg(windows)]
async fn get_system_icon(path: &Path, state: &State<'_, AppState>) -> Option<String> {
    if let Some(cached) = state.icon_cache.get_icon(path).await {
        return Some(format!("data:image/png;base64,{}", cached));
    }

    match crate::platform::windows::extract_file_icon(path).await {
        Some(icon_data) => {
            let _ = state.icon_cache.cache_icon(path, &icon_data).await;
            Some(format!(
                "data:image/png;base64,{}",
                base64::engine::general_purpose::STANDARD.encode(&icon_data)
            ))
        }
        None => None,
    }
}

#[cfg(not(windows))]
async fn get_system_icon(_path: &Path, _state: &State<'_, AppState>) -> Option<String> {
    None
}

/// Check if a file is an application based on path and extension
/// Uses smart classification that considers the file location
fn is_application_smart(path: &str, extension: &str) -> bool {
    let path_lower = path.to_lowercase();
    let ext_lower = extension.to_lowercase();
    
    match ext_lower.as_str() {
        "exe" => true,
        "lnk" => {
            // .lnk in Recent folder is NOT an app
            if path_lower.contains("\\recent\\")
                || path_lower.contains("microsoft\\windows\\recent")
            {
                return false;
            }
            // .lnk in Start Menu or Desktop IS an app
            path_lower.contains("start menu")
                || path_lower.contains("\\desktop\\")
                || path_lower.contains("\\programs\\")
        }
        "msi" => true,
        _ => false,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Hybrid Search Engine (Windows)
// ═══════════════════════════════════════════════════════════════════════════════

/// Search apps using AppIndexer (Rust indexer with pinyin support)
#[cfg(windows)]
async fn search_apps_with_indexer(query: &str, indexer: &AppIndexer, state: &State<'_, AppState>) -> Vec<SearchResult> {
    let app_results = indexer.search(query, 20);

    let mut out = Vec::with_capacity(app_results.len());
    for (idx, result) in app_results.into_iter().enumerate() {
        let path_buf = std::path::PathBuf::from(&result.entry.path);
        let icon_data_url = get_app_icon(&path_buf, state).await;
        let fallback = if result.entry.extension == "lnk" { "🔗" } else { "🚀" };

        out.push(SearchResult {
            id: format!("app-{}", idx),
            r#type: "app".to_string(),
            title: result.entry.name.clone(),
            subtitle: Some(result.entry.path.clone()),
            icon: Some(icon_data_url.unwrap_or_else(|| fallback.to_string())),
            path: Some(result.entry.path.clone()),
            category: "Application".to_string(),
            score: result.score as i32,
            action: SearchAction {
                r#type: "open".to_string(),
                payload: Some(result.entry.path.clone()),
            },
        });
    }

    out
}

/// Search files using Everything (file search engine)
#[cfg(windows)]
async fn search_files_with_everything(query: &str, state: &State<'_, AppState>) -> Result<Vec<SearchResult>, String> {
    tracing::debug!("Searching files with Everything: {}", query);
    
    match everything_service::search_files(query.to_string(), Some(50)).await {
        Ok(file_results) => {
            tracing::debug!("Everything returned {} results", file_results.len());
            
            let mut out = Vec::with_capacity(file_results.len());
            for (idx, result) in file_results.into_iter().enumerate() {
                let path = std::path::Path::new(&result.path);

                // Use the smart category from Everything service
                let is_app = result.category == "Application";

                let result_type = if is_app {
                    "app"
                } else if result.is_folder {
                    "folder"
                } else {
                    "file"
                };

                // Use file stem for apps, full filename for others
                let title = if is_app {
                    path.file_stem()
                        .and_then(|n| n.to_str())
                        .unwrap_or(&result.filename)
                        .to_string()
                } else {
                    result.filename.clone()
                };

                let icon = if is_app {
                    get_app_icon(path, state).await
                } else {
                    get_system_icon(path, state).await
                };

                out.push(SearchResult {
                    id: format!("file-{}", idx),
                    r#type: result_type.to_string(),
                    title,
                    subtitle: Some(result.display_path.clone()),
                    icon,
                    path: Some(result.path.clone()),
                    category: result.category.clone(),
                    score: if is_app { 2000 - idx as i32 } else { 1000 - idx as i32 },
                    action: SearchAction {
                        r#type: "open".to_string(),
                        payload: Some(result.path.clone()),
                    },
                });
            }

            Ok(out)
        }
        Err(e) => {
            tracing::error!("Everything search failed: {}", e);
            Err(e)
        }
    }
}

/// Fallback search for Desktop items when Everything is unavailable.
/// This is intentionally shallow (non-recursive) and limited to a small number of results.
#[cfg(windows)]
async fn fallback_search_desktop(query: &str, state: &State<'_, AppState>) -> Vec<SearchResult> {
    use tokio::fs;

    let q = query.trim();
    if q.is_empty() {
        return Vec::new();
    }
    let q_lower = q.to_lowercase();

    let mut candidates: Vec<std::path::PathBuf> = Vec::new();

    // Standard desktop
    if let Some(user_profile) = std::env::var_os("USERPROFILE") {
        candidates.push(std::path::PathBuf::from(user_profile).join("Desktop"));
    }
    // Public desktop
    if let Some(public) = std::env::var_os("PUBLIC") {
        candidates.push(std::path::PathBuf::from(public).join("Desktop"));
    }
    // OneDrive redirected desktop (common on Win11)
    if let Some(onedrive) = std::env::var_os("OneDrive") {
        candidates.push(std::path::PathBuf::from(onedrive).join("Desktop"));
    }
    if let Some(onedrive) = std::env::var_os("OneDriveConsumer") {
        candidates.push(std::path::PathBuf::from(onedrive).join("Desktop"));
    }

    // De-dup
    candidates.sort();
    candidates.dedup();

    let mut out: Vec<SearchResult> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for dir in candidates {
        let mut rd = match fs::read_dir(&dir).await {
            Ok(v) => v,
            Err(_) => continue,
        };

        while let Ok(Some(entry)) = rd.next_entry().await {
            if out.len() >= 30 {
                break;
            }
            let path = entry.path();

            let name = match path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };

            if !name.to_lowercase().contains(&q_lower) {
                continue;
            }

            let full = path.to_string_lossy().to_string();
            let key = full.to_lowercase();
            if !seen.insert(key) {
                continue;
            }

            let is_folder = entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false);
            let icon = get_system_icon(&path, state).await;

            out.push(SearchResult {
                id: format!("desktop-{}", out.len()),
                r#type: if is_folder { "folder".to_string() } else { "file".to_string() },
                title: name,
                subtitle: Some(full.clone()),
                icon,
                path: Some(full.clone()),
                category: "File".to_string(),
                score: 900 - out.len() as i32,
                action: SearchAction {
                    r#type: "open".to_string(),
                    payload: Some(full),
                },
            });
        }
    }

    out
}

/// Hybrid search: Apps (Rust indexer) + Files (Everything)
/// Apps always appear before files, with deduplication
#[cfg(windows)]
async fn hybrid_search(query: &str, state: &State<'_, AppState>) -> Vec<SearchResult> {
    tracing::info!("Hybrid search for: '{}'", query);
    
    // Run both searches
    let app_results = search_apps_with_indexer(query, &state.app_indexer, state).await;
    tracing::debug!("AppIndexer returned {} results", app_results.len());
    
    let mut file_results = Vec::new();
    let mut everything_failed = false;
    
    match search_files_with_everything(query, state).await {
        Ok(v) => {
            tracing::debug!("Everything returned {} file results", v.len());
            file_results = v;
        }
        Err(e) => {
            tracing::warn!("Everything search failed: {}", e);
            everything_failed = true;
        }
    };

    // Desktop fallback when Everything fails OR returns empty results
    // This helps when Everything.exe isn't running, indexing isn't ready, or file is only on Desktop.
    if file_results.is_empty() || everything_failed {
        let desktop = fallback_search_desktop(query, state).await;
        if !desktop.is_empty() {
            tracing::debug!("Desktop fallback returned {} results", desktop.len());
            // Merge desktop results (avoid duplicates)
            let existing_paths: std::collections::HashSet<String> = file_results
                .iter()
                .filter_map(|r| r.path.clone())
                .map(|p| p.to_lowercase())
                .collect();
            for r in desktop {
                if let Some(ref p) = r.path {
                    if !existing_paths.contains(&p.to_lowercase()) {
                        file_results.push(r);
                    }
                }
            }
        }
    }

    // For Everything results that are classified as applications, try to replace emoji with real icons.
    // Keep it lightweight: only attempt for the first few app-like results.
    let mut upgraded = 0usize;
    for r in file_results.iter_mut() {
        if upgraded >= 12 {
            break;
        }
        if r.category != "Application" {
            continue;
        }
        if r
            .icon
            .as_deref()
            .is_some_and(|v| v.starts_with("data:image/"))
        {
            continue;
        }
        if let Some(ref p) = r.path {
            let pb = std::path::PathBuf::from(p);
            if let Some(icon) = get_app_icon(&pb, state).await {
                r.icon = Some(icon);
                upgraded += 1;
            }
        }
    }
    tracing::debug!("Everything returned {} results", file_results.len());
    
    // Collect paths from app results for deduplication
    let app_paths: HashSet<String> = app_results
        .iter()
        .filter_map(|r| r.path.clone())
        .map(|p| p.to_lowercase())
        .collect();
    
    // Deduplicate: remove files that are already in app results
    let deduplicated_files: Vec<SearchResult> = file_results
        .into_iter()
        .filter(|r| {
            if let Some(ref path) = r.path {
                !app_paths.contains(&path.to_lowercase())
            } else {
                true
            }
        })
        .collect();
    
    // Merge: Apps first, then Files
    let mut results = app_results;
    results.extend(deduplicated_files);
    
    // Sort by score descending
    results.sort_by(|a, b| b.score.cmp(&a.score));
    
    tracing::info!("Hybrid search returned {} total results", results.len());
    
    results
}

/// Search using indexer (fallback for non-Windows)
#[cfg(not(windows))]
async fn search_with_indexer(query: &str, state: &State<'_, AppState>) -> Vec<SearchResult> {
    use crate::core::indexer::FileEntry;
    
    let file_entries = state.indexer.search(query).await;
    
    let mut results = Vec::new();
    for (idx, entry) in file_entries.iter().enumerate() {
        let is_app = entry.path.extension()
            .map(|e| e == "app")
            .unwrap_or(false);
        
        let (result_type, icon) = if is_app {
            let icon = get_app_icon(&entry.path, state).await
                .unwrap_or_else(|| "🚀".to_string());
            ("app".to_string(), Some(icon))
        } else {
            let icon = get_file_icon(&entry.path);
            ("file".to_string(), Some(icon.to_string()))
        };
        
        let title = if is_app {
            entry.display_name.as_ref().unwrap_or(&entry.name).clone()
        } else {
            entry.name.clone()
        };
        
        let subtitle = Some(entry.path.to_string_lossy().to_string());
        let category = if is_app { "Application".to_string() } else { "File".to_string() };
        
        results.push(SearchResult {
            id: entry.id.to_string(),
            r#type: result_type,
            title,
            subtitle,
            icon,
            path: Some(entry.path.to_string_lossy().to_string()),
            category,
            score: idx as i32,
            action: SearchAction {
                r#type: "open".to_string(),
                payload: Some(entry.path.to_string_lossy().to_string()),
            },
        });
    }
    results
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
            // Use hybrid search on Windows (App Indexer + Everything)
            #[cfg(windows)]
            {
                hybrid_search(&q, &state).await
            }
            
            #[cfg(not(windows))]
            {
                // Fallback to indexer search on non-Windows platforms
                search_with_indexer(&q, &state).await
            }
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
                    category: "Utility".to_string(),
                    score: 0,
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
                    category: "Utility".to_string(),
                    score: 0,
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
                category: "Web".to_string(),
                score: 0,
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
                category: "Web".to_string(),
                score: 0,
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
                category: "AI".to_string(),
                score: 0,
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
                category: "Utility".to_string(),
                score: 0,
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
                category: "Web".to_string(),
                score: 0,
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
                category: "Command".to_string(),
                score: 0,
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
