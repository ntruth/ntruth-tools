// Search filter for advanced file searching
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilter {
    /// Filter by file extensions (e.g., ["txt", "md"])
    pub extensions: Option<Vec<String>>,
    /// Filter by path patterns (glob-like patterns)
    pub path_patterns: Option<Vec<String>>,
    /// Minimum file size in bytes
    pub min_size: Option<u64>,
    /// Maximum file size in bytes
    pub max_size: Option<u64>,
    /// Filter files modified after this time
    pub modified_after: Option<DateTime<Utc>>,
    /// Filter files modified before this time
    pub modified_before: Option<DateTime<Utc>>,
}

impl SearchFilter {
    pub fn new() -> Self {
        Self {
            extensions: None,
            path_patterns: None,
            min_size: None,
            max_size: None,
            modified_after: None,
            modified_before: None,
        }
    }

    /// Check if a file matches the filter criteria
    pub fn matches(&self, path: &Path, size: u64, modified: DateTime<Utc>) -> bool {
        // Check extension filter
        if let Some(ref exts) = self.extensions {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_lower = ext.to_lowercase();
                if !exts.iter().any(|e| e.to_lowercase() == ext_lower) {
                    return false;
                }
            } else {
                // File has no extension but filter requires one
                return false;
            }
        }

        // Check path patterns
        if let Some(ref patterns) = self.path_patterns {
            let path_str = path.to_string_lossy().to_lowercase();
            if !patterns.iter().any(|pattern| {
                let pattern_lower = pattern.to_lowercase();
                path_str.contains(&pattern_lower)
            }) {
                return false;
            }
        }

        // Check size filters
        if let Some(min) = self.min_size {
            if size < min {
                return false;
            }
        }

        if let Some(max) = self.max_size {
            if size > max {
                return false;
            }
        }

        // Check modified time filters
        if let Some(after) = self.modified_after {
            if modified < after {
                return false;
            }
        }

        if let Some(before) = self.modified_before {
            if modified > before {
                return false;
            }
        }

        true
    }

    /// Create a filter for documents only
    pub fn documents_only() -> Self {
        Self {
            extensions: Some(vec![
                "txt".to_string(),
                "md".to_string(),
                "pdf".to_string(),
                "doc".to_string(),
                "docx".to_string(),
            ]),
            ..Default::default()
        }
    }

    /// Create a filter for images only
    pub fn images_only() -> Self {
        Self {
            extensions: Some(vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "gif".to_string(),
                "bmp".to_string(),
                "svg".to_string(),
            ]),
            ..Default::default()
        }
    }

    /// Create a filter for code files only
    pub fn code_only() -> Self {
        Self {
            extensions: Some(vec![
                "rs".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "tsx".to_string(),
                "py".to_string(),
                "java".to_string(),
                "c".to_string(),
                "cpp".to_string(),
                "h".to_string(),
                "go".to_string(),
            ]),
            ..Default::default()
        }
    }
}

impl Default for SearchFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_extension_filter() {
        let filter = SearchFilter {
            extensions: Some(vec!["txt".to_string(), "md".to_string()]),
            ..Default::default()
        };

        let path1 = PathBuf::from("file.txt");
        let path2 = PathBuf::from("file.rs");
        let now = Utc::now();

        assert!(filter.matches(&path1, 100, now));
        assert!(!filter.matches(&path2, 100, now));
    }

    #[test]
    fn test_size_filter() {
        let filter = SearchFilter {
            min_size: Some(100),
            max_size: Some(1000),
            ..Default::default()
        };

        let path = PathBuf::from("file.txt");
        let now = Utc::now();

        assert!(filter.matches(&path, 500, now));
        assert!(!filter.matches(&path, 50, now));
        assert!(!filter.matches(&path, 2000, now));
    }

    #[test]
    fn test_preset_filters() {
        let doc_filter = SearchFilter::documents_only();
        let code_filter = SearchFilter::code_only();
        
        assert!(doc_filter.extensions.is_some());
        assert!(code_filter.extensions.is_some());
    }
}
