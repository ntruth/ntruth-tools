// File scanner for indexing
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::task;

#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// Maximum depth to scan
    pub max_depth: Option<usize>,
    /// Paths to exclude (glob patterns)
    pub exclude_patterns: Vec<String>,
    /// Extensions to exclude
    pub exclude_extensions: Vec<String>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            max_depth: Some(10),
            exclude_patterns: vec![
                "**/node_modules/**".to_string(),
                "**/.git/**".to_string(),
                "**/target/**".to_string(),
                "**/dist/**".to_string(),
                "**/build/**".to_string(),
                "**/.cache/**".to_string(),
                "**/Library/**".to_string(), // macOS
                "**/AppData/**".to_string(), // Windows
            ],
            exclude_extensions: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub id: usize,
    pub path: PathBuf,
    pub name: String,
    pub display_name: Option<String>,  // Localized display name (for apps)
    pub size: u64,
    pub modified: Option<std::time::SystemTime>,
}

pub struct FileScanner {
    config: ScanConfig,
}

impl FileScanner {
    pub fn new(config: ScanConfig) -> Self {
        Self { config }
    }

    /// Scan a directory recursively
    pub async fn scan_directory(&self, path: &Path) -> Vec<FileEntry> {
        let mut entries = Vec::new();
        self.scan_recursive(path, 0, &mut entries, 0).await;
        entries
    }

    fn scan_recursive<'a>(
        &'a self,
        path: &'a Path,
        depth: usize,
        entries: &'a mut Vec<FileEntry>,
        next_id: usize,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = usize> + Send + 'a>> {
        Box::pin(async move {
            let mut current_id = next_id;

            // Check max depth
            if let Some(max_depth) = self.config.max_depth {
                if depth > max_depth {
                    return current_id;
                }
            }

            // Check if path should be excluded
            if self.should_exclude(path) {
                return current_id;
            }

            // Read directory
            let mut read_dir = match fs::read_dir(path).await {
                Ok(rd) => rd,
                Err(_) => return current_id,
            };

            while let Ok(Some(entry)) = read_dir.next_entry().await {
                let entry_path = entry.path();

                // Skip hidden files (starting with .)
                if let Some(file_name) = entry_path.file_name() {
                    if file_name.to_string_lossy().starts_with('.') {
                        continue;
                    }
                }

                let metadata = match entry.metadata().await {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                if metadata.is_dir() {
                    // Recursively scan subdirectory
                    current_id = self.scan_recursive(&entry_path, depth + 1, entries, current_id).await;
                } else if metadata.is_file() {
                    // Check extension exclusion
                    if let Some(ext) = entry_path.extension() {
                        let ext_str = ext.to_string_lossy().to_string();
                        if self.config.exclude_extensions.contains(&ext_str) {
                            continue;
                        }
                    }

                    // Add file entry
                    if let Some(name) = entry_path.file_name() {
                        entries.push(FileEntry {
                            id: current_id,
                            path: entry_path.clone(),
                            name: name.to_string_lossy().to_string(),
                            display_name: None,
                            size: metadata.len(),
                            modified: metadata.modified().ok(),
                        });
                        current_id += 1;
                    }
                }
            }

            current_id
        })
    }

    fn should_exclude(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        // Check against exclude patterns
        for pattern in &self.config.exclude_patterns {
            // Simple glob matching (can be improved with glob crate)
            if self.matches_pattern(&path_str, pattern) {
                return true;
            }
        }

        false
    }

    fn matches_pattern(&self, path: &str, pattern: &str) -> bool {
        // Simple pattern matching (can be improved)
        if pattern.starts_with("**/") && pattern.ends_with("/**") {
            let segment = &pattern[3..pattern.len() - 3];
            path.contains(&format!("/{}/", segment)) || path.contains(&format!("\\{}\\", segment))
        } else {
            path.contains(&pattern.replace("**", ""))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ScanConfig::default();
        assert!(config.max_depth.is_some());
        assert!(!config.exclude_patterns.is_empty());
    }
}
