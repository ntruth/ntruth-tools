//! Application Indexer with Pinyin Support
//! 
//! Provides fast app search with:
//! - Chinese pinyin matching (微信 -> weixin)
//! - Fuzzy matching (Idea -> IntelliJ IDEA)
//! - Abbreviation matching (wx -> 微信)

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::env;

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use parking_lot::RwLock;
use pinyin::ToPinyin;
use serde::Serialize;
use walkdir::WalkDir;

// ═══════════════════════════════════════════════════════════════════════════════
// Data Structures
// ═══════════════════════════════════════════════════════════════════════════════

/// Represents an indexed application entry
#[derive(Debug, Clone, Serialize)]
pub struct AppEntry {
    /// Display name (e.g., "微信", "Google Chrome")
    pub name: String,
    /// Pinyin representation for Chinese characters (e.g., "weixin")
    pub pinyin_full: String,
    /// Pinyin initials (e.g., "wx" for "微信")
    pub pinyin_initials: String,
    /// Full path to the shortcut or executable
    pub path: String,
    /// File extension (lnk, exe, etc.)
    pub extension: String,
    /// Is this from Start Menu (higher priority)
    pub is_start_menu: bool,
}

/// Search result with relevance score
#[derive(Debug, Clone, Serialize)]
pub struct AppSearchResult {
    pub entry: AppEntry,
    pub score: i64,
    pub match_type: MatchType,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum MatchType {
    /// Exact name match
    ExactName,
    /// Fuzzy name match
    FuzzyName,
    /// Pinyin full match (weixin -> 微信)
    PinyinFull,
    /// Pinyin initials match (wx -> 微信)
    PinyinInitials,
}

// ═══════════════════════════════════════════════════════════════════════════════
// App Indexer
// ═══════════════════════════════════════════════════════════════════════════════

pub struct AppIndexer {
    /// Cached app entries
    entries: Arc<RwLock<Vec<AppEntry>>>,
    /// Fuzzy matcher
    matcher: SkimMatcherV2,
}

impl Default for AppIndexer {
    fn default() -> Self {
        Self::new()
    }
}

impl AppIndexer {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
            matcher: SkimMatcherV2::default().smart_case(),
        }
    }

    /// Initialize indexer and scan for apps
    pub async fn init(&self) -> Result<usize, String> {
        let entries = tokio::task::spawn_blocking(|| {
            Self::scan_apps()
        })
        .await
        .map_err(|e| format!("Spawn blocking failed: {}", e))?;
        
        let count = entries.len();
        *self.entries.write() = entries;
        
        tracing::info!("AppIndexer initialized with {} apps", count);
        Ok(count)
    }

    /// Scan all application directories
    fn scan_apps() -> Vec<AppEntry> {
        let mut entries = Vec::new();
        
        // Start Menu paths
        let start_menu_paths = Self::get_start_menu_paths();
        
        for (path, is_start_menu) in start_menu_paths {
            if path.exists() {
                tracing::debug!("Scanning: {:?}", path);
                Self::scan_directory(&path, is_start_menu, &mut entries);
            }
        }
        
        // Remove duplicates (same name, keep Start Menu version)
        entries.sort_by(|a, b| {
            let name_cmp = a.name.to_lowercase().cmp(&b.name.to_lowercase());
            if name_cmp == std::cmp::Ordering::Equal {
                // Prefer Start Menu entries
                b.is_start_menu.cmp(&a.is_start_menu)
            } else {
                name_cmp
            }
        });
        entries.dedup_by(|a, b| a.name.to_lowercase() == b.name.to_lowercase());
        
        tracing::info!("Scanned {} unique apps", entries.len());
        entries
    }

    /// Get Start Menu paths for Windows
    fn get_start_menu_paths() -> Vec<(PathBuf, bool)> {
        let mut paths = Vec::new();
        
        // System Start Menu
        paths.push((
            PathBuf::from(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"),
            true,
        ));
        
        // User Start Menu
        if let Ok(appdata) = env::var("APPDATA") {
            paths.push((
                PathBuf::from(appdata).join(r"Microsoft\Windows\Start Menu\Programs"),
                true,
            ));
        }
        
        // Desktop (lower priority)
        if let Ok(userprofile) = env::var("USERPROFILE") {
            paths.push((
                PathBuf::from(&userprofile).join("Desktop"),
                false,
            ));
            // Public Desktop
            paths.push((
                PathBuf::from(r"C:\Users\Public\Desktop"),
                false,
            ));
        }
        
        paths
    }

    /// Scan a directory for app shortcuts
    fn scan_directory(dir: &Path, is_start_menu: bool, entries: &mut Vec<AppEntry>) {
        for entry in WalkDir::new(dir)
            .follow_links(true)
            .max_depth(5)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Only process .lnk and .exe files
            let extension = path.extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_lowercase())
                .unwrap_or_default();
            
            if extension != "lnk" && extension != "exe" {
                continue;
            }
            
            // Get file name without extension
            let name = path.file_stem()
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
                .unwrap_or_default();
            
            // Skip empty names and uninstallers
            if name.is_empty() {
                continue;
            }
            let name_lower = name.to_lowercase();
            if name_lower.contains("uninstall") 
                || name_lower.contains("卸载")
                || name_lower.contains("readme")
                || name_lower.contains("help")
            {
                continue;
            }
            
            // Generate pinyin
            let (pinyin_full, pinyin_initials) = Self::to_pinyin(&name);
            
            entries.push(AppEntry {
                name,
                pinyin_full,
                pinyin_initials,
                path: path.to_string_lossy().to_string(),
                extension,
                is_start_menu,
            });
        }
    }

    /// Convert Chinese characters to pinyin
    fn to_pinyin(text: &str) -> (String, String) {
        let mut full = String::new();
        let mut initials = String::new();
        
        for c in text.chars() {
            if let Some(pinyin) = c.to_pinyin() {
                // Full pinyin (e.g., "wei" for 微)
                full.push_str(pinyin.plain());
                // Initial only (e.g., "w" for 微)
                if let Some(first) = pinyin.plain().chars().next() {
                    initials.push(first);
                }
            } else if c.is_alphanumeric() {
                // Keep alphanumeric characters
                full.push(c.to_ascii_lowercase());
                initials.push(c.to_ascii_lowercase());
            }
            // Skip spaces and special characters in pinyin
        }
        
        (full, initials)
    }

    /// Search for apps matching the query
    pub fn search(&self, query: &str, max_results: usize) -> Vec<AppSearchResult> {
        if query.is_empty() {
            return Vec::new();
        }
        
        let query_lower = query.to_lowercase();
        let entries = self.entries.read();
        let mut results: Vec<AppSearchResult> = Vec::new();
        
        for entry in entries.iter() {
            let mut best_score: i64 = 0;
            let mut best_match_type = MatchType::FuzzyName;
            
            // 1. Exact name match (highest priority)
            let name_lower = entry.name.to_lowercase();
            if name_lower == query_lower {
                best_score = 10000;
                best_match_type = MatchType::ExactName;
            } else if name_lower.starts_with(&query_lower) {
                best_score = 8000 + (100 - name_lower.len() as i64).max(0);
                best_match_type = MatchType::ExactName;
            } else if name_lower.contains(&query_lower) {
                best_score = 6000 + (100 - name_lower.len() as i64).max(0);
                best_match_type = MatchType::ExactName;
            }
            
            // 2. Fuzzy name match
            if let Some(score) = self.matcher.fuzzy_match(&name_lower, &query_lower) {
                let adjusted_score = score + 1000; // Base boost for name match
                if adjusted_score > best_score {
                    best_score = adjusted_score;
                    best_match_type = MatchType::FuzzyName;
                }
            }
            
            // 3. Pinyin full match (weixin -> 微信)
            if !entry.pinyin_full.is_empty() {
                if entry.pinyin_full == query_lower {
                    let score = 9000; // Very high for exact pinyin match
                    if score > best_score {
                        best_score = score;
                        best_match_type = MatchType::PinyinFull;
                    }
                } else if entry.pinyin_full.starts_with(&query_lower) {
                    let score = 7000 + (100 - entry.pinyin_full.len() as i64).max(0);
                    if score > best_score {
                        best_score = score;
                        best_match_type = MatchType::PinyinFull;
                    }
                } else if let Some(score) = self.matcher.fuzzy_match(&entry.pinyin_full, &query_lower) {
                    let adjusted_score = score + 500;
                    if adjusted_score > best_score {
                        best_score = adjusted_score;
                        best_match_type = MatchType::PinyinFull;
                    }
                }
            }
            
            // 4. Pinyin initials match (wx -> 微信)
            if !entry.pinyin_initials.is_empty() {
                if entry.pinyin_initials == query_lower {
                    let score = 8500; // High for exact initials match
                    if score > best_score {
                        best_score = score;
                        best_match_type = MatchType::PinyinInitials;
                    }
                } else if entry.pinyin_initials.starts_with(&query_lower) {
                    let score = 6500 + (100 - entry.pinyin_initials.len() as i64).max(0);
                    if score > best_score {
                        best_score = score;
                        best_match_type = MatchType::PinyinInitials;
                    }
                }
            }
            
            // Only include if there's a match
            if best_score > 0 {
                // Boost Start Menu entries
                if entry.is_start_menu {
                    best_score += 200;
                }
                // Boost .lnk over .exe (shortcuts are usually what users want)
                if entry.extension == "lnk" {
                    best_score += 100;
                }
                
                results.push(AppSearchResult {
                    entry: entry.clone(),
                    score: best_score,
                    match_type: best_match_type,
                });
            }
        }
        
        // Sort by score descending
        results.sort_by(|a, b| b.score.cmp(&a.score));
        
        // Limit results
        results.truncate(max_results);
        
        results
    }

    /// Get number of indexed apps
    pub fn app_count(&self) -> usize {
        self.entries.read().len()
    }

    /// Refresh the index
    pub async fn refresh(&self) -> Result<usize, String> {
        self.init().await
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinyin_conversion() {
        let (full, initials) = AppIndexer::to_pinyin("微信");
        assert_eq!(full, "weixin");
        assert_eq!(initials, "wx");
        
        let (full, initials) = AppIndexer::to_pinyin("QQ音乐");
        assert_eq!(full, "qqyinyue");
        assert_eq!(initials, "qqyy");
    }

    #[test]
    fn test_pinyin_mixed() {
        let (full, initials) = AppIndexer::to_pinyin("Chrome 浏览器");
        assert_eq!(full, "chromeliulanqi");
        assert_eq!(initials, "chromellq");
    }
}
