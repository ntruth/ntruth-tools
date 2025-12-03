// Web search functionality
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngine {
    pub id: String,
    pub name: String,
    pub keyword: String,
    pub url_template: String,
    pub icon: Option<String>,
}

impl SearchEngine {
    /// Build the search URL for a query
    pub fn build_url(&self, query: &str) -> String {
        self.url_template.replace("{query}", &urlencoding::encode(query))
    }
}

/// Check if input is a URL
pub fn is_url(input: &str) -> bool {
    let input = input.trim();
    
    // Check for common URL patterns
    if input.starts_with("http://") || input.starts_with("https://") {
        return true;
    }
    
    // Check for domain-like pattern (e.g., "example.com")
    if input.contains('.') {
        let parts: Vec<&str> = input.split('.').collect();
        if parts.len() >= 2 {
            let last_part = parts.last().unwrap();
            // Common TLDs
            if last_part.len() >= 2 && last_part.chars().all(|c| c.is_ascii_alphabetic()) {
                return true;
            }
        }
    }
    
    false
}

/// Parse web search trigger from input
pub fn parse_search_trigger(input: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
    
    if parts.len() == 2 {
        let keyword = parts[0];
        let query = parts[1];
        
        // Check if it looks like a search trigger (2-3 letters)
        if keyword.len() >= 2 && keyword.len() <= 3 && keyword.chars().all(|c| c.is_ascii_alphabetic()) {
            return Some((keyword.to_string(), query.to_string()));
        }
    }
    
    None
}

/// Get builtin search engines
pub fn builtin_engines() -> Vec<SearchEngine> {
    vec![
        SearchEngine {
            id: "google".to_string(),
            name: "Google".to_string(),
            keyword: "gg".to_string(),
            url_template: "https://www.google.com/search?q={query}".to_string(),
            icon: None,
        },
        SearchEngine {
            id: "baidu".to_string(),
            name: "Baidu".to_string(),
            keyword: "bd".to_string(),
            url_template: "https://www.baidu.com/s?wd={query}".to_string(),
            icon: None,
        },
        SearchEngine {
            id: "github".to_string(),
            name: "GitHub".to_string(),
            keyword: "gh".to_string(),
            url_template: "https://github.com/search?q={query}".to_string(),
            icon: None,
        },
        SearchEngine {
            id: "stackoverflow".to_string(),
            name: "Stack Overflow".to_string(),
            keyword: "so".to_string(),
            url_template: "https://stackoverflow.com/search?q={query}".to_string(),
            icon: None,
        },
        SearchEngine {
            id: "bing".to_string(),
            name: "Bing".to_string(),
            keyword: "bg".to_string(),
            url_template: "https://www.bing.com/search?q={query}".to_string(),
            icon: None,
        },
        SearchEngine {
            id: "duckduckgo".to_string(),
            name: "DuckDuckGo".to_string(),
            keyword: "dd".to_string(),
            url_template: "https://duckduckgo.com/?q={query}".to_string(),
            icon: None,
        },
        SearchEngine {
            id: "youtube".to_string(),
            name: "YouTube".to_string(),
            keyword: "yt".to_string(),
            url_template: "https://www.youtube.com/results?search_query={query}".to_string(),
            icon: None,
        },
        SearchEngine {
            id: "twitter".to_string(),
            name: "Twitter/X".to_string(),
            keyword: "tw".to_string(),
            url_template: "https://twitter.com/search?q={query}".to_string(),
            icon: None,
        },
    ]
}

/// Validate a URL template
pub fn validate_url_template(template: &str) -> bool {
    template.contains("{query}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_url() {
        assert!(is_url("https://example.com"));
        assert!(is_url("http://example.com"));
        assert!(is_url("example.com"));
        assert!(is_url("www.example.com"));
        assert!(!is_url("just text"));
        assert!(!is_url("single"));
    }

    #[test]
    fn test_parse_search_trigger() {
        let result = parse_search_trigger("gg hello world");
        assert!(result.is_some());
        let (keyword, query) = result.unwrap();
        assert_eq!(keyword, "gg");
        assert_eq!(query, "hello world");

        assert!(parse_search_trigger("just text").is_none());
        assert!(parse_search_trigger("toolong search").is_none());
    }

    #[test]
    fn test_builtin_engines() {
        let engines = builtin_engines();
        assert!(!engines.is_empty());
        
        let google = engines.iter().find(|e| e.id == "google");
        assert!(google.is_some());
        assert_eq!(google.unwrap().keyword, "gg");
    }

    #[test]
    fn test_build_url() {
        let engine = SearchEngine {
            id: "test".to_string(),
            name: "Test".to_string(),
            keyword: "t".to_string(),
            url_template: "https://example.com/search?q={query}".to_string(),
            icon: None,
        };
        
        let url = engine.build_url("hello world");
        assert!(url.contains("hello"));
        assert!(url.contains("example.com"));
    }

    #[test]
    fn test_validate_url_template() {
        assert!(validate_url_template("https://example.com?q={query}"));
        assert!(!validate_url_template("https://example.com"));
    }
}
