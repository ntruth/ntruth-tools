// Input parser module
mod calculator;
pub mod web_search;

pub use calculator::Calculator;
pub use web_search::{SearchEngine, builtin_engines, parse_search_trigger, validate_url_template};
pub use web_search::is_url as is_web_url;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parser {
    web_engines: HashMap<String, WebSearchEngine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchEngine {
    pub name: String,
    pub url_template: String,
}

impl Parser {
    pub fn new() -> Self {
        let mut web_engines = HashMap::new();

        // Initialize built-in search engines
        web_engines.insert(
            "gg".to_string(),
            WebSearchEngine {
                name: "Google".to_string(),
                url_template: "https://www.google.com/search?q={query}".to_string(),
            },
        );
        web_engines.insert(
            "bd".to_string(),
            WebSearchEngine {
                name: "Baidu".to_string(),
                url_template: "https://www.baidu.com/s?wd={query}".to_string(),
            },
        );
        web_engines.insert(
            "bi".to_string(),
            WebSearchEngine {
                name: "Bing".to_string(),
                url_template: "https://www.bing.com/search?q={query}".to_string(),
            },
        );
        web_engines.insert(
            "ddg".to_string(),
            WebSearchEngine {
                name: "DuckDuckGo".to_string(),
                url_template: "https://duckduckgo.com/?q={query}".to_string(),
            },
        );
        web_engines.insert(
            "gh".to_string(),
            WebSearchEngine {
                name: "GitHub".to_string(),
                url_template: "https://github.com/search?q={query}".to_string(),
            },
        );
        web_engines.insert(
            "so".to_string(),
            WebSearchEngine {
                name: "Stack Overflow".to_string(),
                url_template: "https://stackoverflow.com/search?q={query}".to_string(),
            },
        );
        web_engines.insert(
            "yt".to_string(),
            WebSearchEngine {
                name: "YouTube".to_string(),
                url_template: "https://www.youtube.com/results?search_query={query}".to_string(),
            },
        );
        web_engines.insert(
            "tw".to_string(),
            WebSearchEngine {
                name: "Twitter".to_string(),
                url_template: "https://twitter.com/search?q={query}".to_string(),
            },
        );
        web_engines.insert(
            "npm".to_string(),
            WebSearchEngine {
                name: "NPM".to_string(),
                url_template: "https://www.npmjs.com/search?q={query}".to_string(),
            },
        );
        web_engines.insert(
            "crate".to_string(),
            WebSearchEngine {
                name: "Crates.io".to_string(),
                url_template: "https://crates.io/search?q={query}".to_string(),
            },
        );

        Self { web_engines }
    }

    pub fn parse(&self, input: &str) -> ParseResult {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return ParseResult::Empty;
        }

        // Check for calculator (starts with = or looks like math)
        if trimmed.starts_with('=') {
            return ParseResult::Calculator(trimmed[1..].trim().to_string());
        }

        // Check if it's a math expression
        if is_math_expression(trimmed) {
            return ParseResult::Calculator(trimmed.to_string());
        }

        // Check for AI query
        if trimmed.starts_with("ai ") {
            return ParseResult::AI(trimmed[3..].trim().to_string());
        }

        // Check for clipboard search
        if trimmed.starts_with("cb ") {
            return ParseResult::Clipboard(trimmed[3..].trim().to_string());
        }

        // Check for bookmark search
        if trimmed.starts_with("bm ") {
            return ParseResult::Bookmark(trimmed[3..].trim().to_string());
        }

        // Check for system command
        if trimmed.starts_with("> ") {
            return ParseResult::Command(trimmed[2..].trim().to_string());
        }

        // Check for web search with keyword
        for (keyword, engine) in &self.web_engines {
            let prefix = format!("{} ", keyword);
            if trimmed.starts_with(&prefix) {
                let query = trimmed[prefix.len()..].trim().to_string();
                let url = engine.url_template.replace("{query}", &urlencoding::encode(&query));
                return ParseResult::WebSearch {
                    engine: engine.name.clone(),
                    query,
                    url,
                };
            }
        }

        // Check for URL
        if is_url(trimmed) {
            return ParseResult::Url(normalize_url(trimmed));
        }

        // Default: file/app search
        ParseResult::FileOrApp(trimmed.to_string())
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ParseResult {
    Empty,
    FileOrApp(String),
    Calculator(String),
    WebSearch {
        engine: String,
        query: String,
        url: String,
    },
    Url(String),
    AI(String),
    Clipboard(String),
    Bookmark(String),
    Command(String),
}

/// Check if input looks like a math expression
fn is_math_expression(input: &str) -> bool {
    // Simple heuristic: contains mostly numbers, operators, and math functions
    let math_chars = input.chars().all(|c| {
        c.is_ascii_digit()
            || c.is_whitespace()
            || matches!(c, '+' | '-' | '*' | '/' | '(' | ')' | '.' | '^' | '%')
    });

    // Must contain at least one operator
    let has_operator = input.contains('+')
        || input.contains('-')
        || input.contains('*')
        || input.contains('/')
        || input.contains('^')
        || input.contains('%');

    // Check for math functions
    let has_function = input.contains("sin")
        || input.contains("cos")
        || input.contains("tan")
        || input.contains("sqrt")
        || input.contains("log")
        || input.contains("ln")
        || input.contains("abs");

    (math_chars && has_operator) || has_function
}

/// Check if input is a URL
fn is_url(input: &str) -> bool {
    input.starts_with("http://")
        || input.starts_with("https://")
        || input.starts_with("www.")
        || (input.contains('.') && !input.contains(' ') && input.split('.').count() >= 2)
}

/// Normalize URL by adding protocol if missing
fn normalize_url(input: &str) -> String {
    if input.starts_with("http://") || input.starts_with("https://") {
        input.to_string()
    } else if input.starts_with("www.") {
        format!("https://{}", input)
    } else {
        format!("https://{}", input)
    }
}
