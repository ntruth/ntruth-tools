use crate::app::{error::AppResult, state::AppState};
use crate::core::parser::{Parser, ParseResult};
use crate::core::indexer::FileEntry;
use serde::{Deserialize, Serialize};
use tauri::State;

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
            
            // Convert FileEntry to SearchResult
            file_entries
                .into_iter()
                .map(|entry| SearchResult {
                    id: entry.id.to_string(),
                    r#type: "file".to_string(),
                    title: entry.name.clone(),
                    subtitle: Some(entry.path.to_string_lossy().to_string()),
                    icon: None,
                    path: Some(entry.path.to_string_lossy().to_string()),
                    action: SearchAction {
                        r#type: "open".to_string(),
                        payload: Some(entry.path.to_string_lossy().to_string()),
                    },
                })
                .collect()
        }
        
        ParseResult::Calculator(expr) => {
            // Evaluate calculator expression
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

/// Evaluate a mathematical expression
fn evaluate_expression(expr: &str) -> Result<String, String> {
    // Try to evaluate using meval
    match meval::eval_str(expr) {
        Ok(value) => Ok(format!("{}", value)),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
