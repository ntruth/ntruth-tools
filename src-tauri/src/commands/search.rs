use crate::app::{error::AppResult, state::AppState};
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
}

/// Search command
#[tauri::command]
pub async fn search(
    query: String,
    _state: State<'_, AppState>,
) -> AppResult<Vec<SearchResult>> {
    // TODO: Implement actual search logic
    // For now, return empty results
    Ok(vec![])
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
    // TODO: Implement calculator logic
    // For now, return a simple result
    
    // Try to parse and evaluate simple expressions
    let result = match parse_expression(&expression) {
        Ok(value) => value.to_string(),
        Err(e) => e,
    };

    Ok(CalculatorResult {
        expression: expression.clone(),
        result,
        r#type: "number".to_string(),
    })
}

// Simple expression parser for basic arithmetic
fn parse_expression(expr: &str) -> Result<f64, String> {
    // Remove whitespace
    let expr = expr.trim();
    
    // Try to evaluate using meval (we'll need to add this dependency later)
    // For now, just try to parse as a number
    expr.parse::<f64>()
        .map_err(|_| "Invalid expression".to_string())
}
