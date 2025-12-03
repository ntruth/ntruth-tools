// Sensitive content filtering
use regex::Regex;

pub struct ContentFilter {
    patterns: Vec<Regex>,
}

impl ContentFilter {
    pub fn new() -> Self {
        let patterns = vec![
            // Credit card numbers (basic pattern)
            Regex::new(r"\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b").unwrap(),
            // Social Security Numbers
            Regex::new(r"\b\d{3}[-\s]?\d{2}[-\s]?\d{4}\b").unwrap(),
            // Email addresses with common keywords
            Regex::new(r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b").unwrap(),
            // API Keys (common patterns)
            Regex::new(r#"(?i)(api[_-]?key|apikey|access[_-]?token|secret[_-]?key)['"]?\s*[:=]\s*['"]?[a-zA-Z0-9_-]{20,}"#).unwrap(),
            // Private keys
            Regex::new(r"-----BEGIN (RSA |EC )?PRIVATE KEY-----").unwrap(),
            // AWS keys
            Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),
            // Password patterns
            Regex::new(r#"(?i)(password|passwd|pwd)['"]?\s*[:=]\s*['"]?[^\s'"]{8,}"#).unwrap(),
        ];

        Self { patterns }
    }

    /// Check if the text contains sensitive content
    pub fn is_sensitive(&self, text: &str) -> bool {
        self.patterns.iter().any(|pattern| pattern.is_match(text))
    }

    /// Get sensitive matches in the text
    pub fn find_sensitive_patterns(&self, text: &str) -> Vec<String> {
        let mut matches = Vec::new();
        
        for pattern in &self.patterns {
            if let Some(mat) = pattern.find(text) {
                matches.push(mat.as_str().to_string());
            }
        }

        matches
    }

    /// Redact sensitive content from text
    pub fn redact(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        for pattern in &self.patterns {
            result = pattern.replace_all(&result, "[REDACTED]").to_string();
        }

        result
    }
}

impl Default for ContentFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_card_detection() {
        let filter = ContentFilter::new();
        assert!(filter.is_sensitive("My card is 1234 5678 9012 3456"));
        assert!(!filter.is_sensitive("Just some random numbers 123"));
    }

    #[test]
    fn test_api_key_detection() {
        let filter = ContentFilter::new();
        assert!(filter.is_sensitive("api_key=abcdefghijklmnopqrstuvwxyz123456"));
        assert!(!filter.is_sensitive("This is normal text"));
    }

    #[test]
    fn test_redaction() {
        let filter = ContentFilter::new();
        let text = "My card is 1234-5678-9012-3456 and key=abc123def456ghi789";
        let redacted = filter.redact(text);
        assert!(redacted.contains("[REDACTED]"));
        assert!(!redacted.contains("1234"));
    }
}
