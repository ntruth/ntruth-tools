// File indexer module
mod trie;
mod trigram;
mod scanner;
mod ranker;
mod watcher;
mod filter;

pub use scanner::{FileScanner, ScanConfig, FileEntry};
pub use ranker::{Ranker, FileScore};
pub use watcher::FileWatcher;
pub use filter::SearchFilter;

use trie::Trie;
use trigram::TrigramIndex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Check if a character is CJK (Chinese, Japanese, Korean)
fn is_cjk(c: char) -> bool {
    matches!(c,
        '\u{4E00}'..='\u{9FFF}' |    // CJK Unified Ideographs
        '\u{3400}'..='\u{4DBF}' |    // CJK Unified Ideographs Extension A
        '\u{20000}'..='\u{2A6DF}' |  // CJK Unified Ideographs Extension B
        '\u{F900}'..='\u{FAFF}' |    // CJK Compatibility Ideographs
        '\u{3000}'..='\u{303F}' |    // CJK Symbols and Punctuation
        '\u{3040}'..='\u{309F}' |    // Hiragana
        '\u{30A0}'..='\u{30FF}' |    // Katakana
        '\u{AC00}'..='\u{D7AF}'      // Hangul Syllables
    )
}

/// Main file indexer that combines Trie and Trigram indexing
pub struct Indexer {
    /// Trie for prefix matching
    trie: Arc<RwLock<Trie>>,
    /// Trigram index for fuzzy search
    trigram: Arc<RwLock<TrigramIndex>>,
    /// File entries by ID
    files: Arc<RwLock<HashMap<usize, FileEntry>>>,
    /// Path to ID mapping for quick lookup
    path_to_id: Arc<RwLock<HashMap<PathBuf, usize>>>,
    /// File scanner
    scanner: FileScanner,
    /// Ranking algorithm
    ranker: Arc<RwLock<Ranker>>,
    /// File watcher for incremental updates
    watcher: Arc<RwLock<Option<FileWatcher>>>,
    /// Next available ID
    next_id: Arc<RwLock<usize>>,
}

impl Indexer {
    pub fn new(config: ScanConfig) -> Self {
        Self {
            trie: Arc::new(RwLock::new(Trie::new())),
            trigram: Arc::new(RwLock::new(TrigramIndex::new())),
            files: Arc::new(RwLock::new(HashMap::new())),
            path_to_id: Arc::new(RwLock::new(HashMap::new())),
            scanner: FileScanner::new(config),
            ranker: Arc::new(RwLock::new(Ranker::new())),
            watcher: Arc::new(RwLock::new(None)),
            next_id: Arc::new(RwLock::new(1)),
        }
    }

    /// Index a directory
    pub async fn index_directory(&self, path: &Path) -> Result<usize, String> {
        let entries = self.scanner.scan_directory(path).await;
        let count = entries.len();

        let mut trie = self.trie.write().await;
        let mut trigram = self.trigram.write().await;
        let mut files = self.files.write().await;
        let mut path_to_id = self.path_to_id.write().await;

        for entry in entries {
            let file_id = entry.id;
            let file_name = entry.name.clone();
            let file_path = entry.path.clone();

            // Add to trie (word-by-word)
            for word in file_name.split(|c: char| !c.is_alphanumeric()) {
                if !word.is_empty() {
                    trie.insert(word, file_id);
                }
            }

            // Add to trigram index
            trigram.add_file(&file_name, file_id);

            // Store file entry and path mapping
            path_to_id.insert(file_path, file_id);
            files.insert(file_id, entry);
        }

        Ok(count)
    }

    /// Add a single file to the index
    pub async fn add_file(&self, path: &Path) -> Result<usize, String> {
        self.add_file_with_display_name(path, None).await
    }
    
    /// Add a single file to the index with an optional display name
    pub async fn add_file_with_display_name(&self, path: &Path, display_name: Option<String>) -> Result<usize, String> {
        if !path.exists() {
            return Err("Path does not exist".to_string());
        }

        // Check if already indexed
        {
            let path_to_id = self.path_to_id.read().await;
            if path_to_id.contains_key(path) {
                return Err("File already indexed".to_string());
            }
        }

        // Get next ID
        let file_id = {
            let mut next_id = self.next_id.write().await;
            let id = *next_id;
            *next_id += 1;
            id
        };

        // For .app bundles, use the bundle name without extension
        let file_name = if path.extension().map(|e| e == "app").unwrap_or(false) {
            path.file_stem()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default()
        } else {
            path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default()
        };

        let metadata = tokio::fs::metadata(path).await
            .map_err(|e| e.to_string())?;

        let entry = FileEntry {
            id: file_id,
            name: file_name.clone(),
            display_name: display_name.clone(),
            path: path.to_path_buf(),
            size: metadata.len(),
            modified: metadata.modified().ok(),
        };

        // Add to indexes
        let mut trie = self.trie.write().await;
        let mut trigram = self.trigram.write().await;
        let mut files = self.files.write().await;
        let mut path_to_id = self.path_to_id.write().await;

        // Add to trie - index both full name and individual words
        // For apps like "Google Chrome", we want to match "goo", "chr", "chrome", etc.
        let name_lower = file_name.to_lowercase();
        
        // Add full name
        trie.insert(&name_lower, file_id);
        
        // Add individual words from file name
        for word in file_name.split(|c: char| !c.is_alphanumeric()) {
            if !word.is_empty() && word.len() >= 2 {
                trie.insert(&word.to_lowercase(), file_id);
            }
        }
        
        // Also index the display name (for Chinese/localized names)
        if let Some(ref disp_name) = display_name {
            let disp_lower = disp_name.to_lowercase();
            trie.insert(&disp_lower, file_id);
            
            // Add individual words/characters from display name
            for word in disp_name.split(|c: char| !c.is_alphanumeric() && !is_cjk(c)) {
                if !word.is_empty() {
                    trie.insert(&word.to_lowercase(), file_id);
                }
            }
            
            // For CJK text, index each character
            for ch in disp_name.chars() {
                if is_cjk(ch) {
                    trie.insert(&ch.to_string(), file_id);
                }
            }
            
            // Add display name to trigram index
            trigram.add_file(disp_name, file_id);
        }

        // Add to trigram index
        trigram.add_file(&file_name, file_id);

        // Store entry
        path_to_id.insert(path.to_path_buf(), file_id);
        files.insert(file_id, entry);

        tracing::debug!("Added to index: {:?} as '{}' (display: {:?})", path, file_name, display_name);
        Ok(file_id)
    }

    /// Remove a file from the index
    pub async fn remove_file(&self, path: &Path) -> Result<(), String> {
        let file_id = {
            let path_to_id = self.path_to_id.read().await;
            match path_to_id.get(path) {
                Some(&id) => id,
                None => return Ok(()), // File not in index
            }
        };

        let mut trie = self.trie.write().await;
        let mut trigram = self.trigram.write().await;
        let mut files = self.files.write().await;
        let mut path_to_id = self.path_to_id.write().await;

        // Get file name for removal from trie
        if let Some(entry) = files.get(&file_id) {
            let file_name = entry.name.clone();
            
            // Remove from trie
            for word in file_name.split(|c: char| !c.is_alphanumeric()) {
                if !word.is_empty() {
                    trie.remove(word, file_id);
                }
            }

            // Remove from trigram index
            trigram.remove_file(file_id);
        }

        // Remove from storage
        path_to_id.remove(path);
        files.remove(&file_id);

        tracing::debug!("Removed file from index: {:?}", path);
        Ok(())
    }

    /// Update a file in the index (re-index)
    pub async fn update_file(&self, path: &Path) -> Result<(), String> {
        self.remove_file(path).await?;
        if path.exists() {
            self.add_file(path).await?;
        }
        Ok(())
    }

    /// Start watching directories for changes
    pub async fn start_watching(&self, paths: Vec<PathBuf>) -> Result<(), String> {
        let mut watcher = FileWatcher::new();
        
        for path in &paths {
            watcher.add_path(path.clone());
        }

        // Clone Arcs for the callback
        let trie = self.trie.clone();
        let trigram = self.trigram.clone();
        let files = self.files.clone();
        let path_to_id = self.path_to_id.clone();
        let next_id = self.next_id.clone();

        watcher.start_watching(move |changed_path| {
            let trie = trie.clone();
            let trigram = trigram.clone();
            let files = files.clone();
            let path_to_id = path_to_id.clone();
            let next_id = next_id.clone();

            // Spawn async task to handle the change
            tokio::spawn(async move {
                if changed_path.exists() {
                    // File was created or modified
                    let file_id = {
                        let path_to_id_read = path_to_id.read().await;
                        path_to_id_read.get(&changed_path).copied()
                    };

                    if let Some(file_id) = file_id {
                        // Update existing file
                        tracing::debug!("File modified: {:?}", changed_path);
                        // For simplicity, just update the entry
                        if let Ok(metadata) = tokio::fs::metadata(&changed_path).await {
                            let mut files_write = files.write().await;
                            if let Some(entry) = files_write.get_mut(&file_id) {
                                entry.size = metadata.len();
                                entry.modified = metadata.modified().ok();
                            }
                        }
                    } else if changed_path.is_file() {
                        // New file
                        tracing::debug!("New file detected: {:?}", changed_path);
                        
                        let file_id = {
                            let mut next = next_id.write().await;
                            let id = *next;
                            *next += 1;
                            id
                        };

                        let file_name = changed_path.file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();

                        if let Ok(metadata) = tokio::fs::metadata(&changed_path).await {
                            let entry = FileEntry {
                                id: file_id,
                                name: file_name.clone(),
                                display_name: None,
                                path: changed_path.clone(),
                                size: metadata.len(),
                                modified: metadata.modified().ok(),
                            };

                            let mut trie_write = trie.write().await;
                            let mut trigram_write = trigram.write().await;
                            let mut files_write = files.write().await;
                            let mut path_to_id_write = path_to_id.write().await;

                            // Add to indexes
                            for word in file_name.split(|c: char| !c.is_alphanumeric()) {
                                if !word.is_empty() {
                                    trie_write.insert(word, file_id);
                                }
                            }
                            trigram_write.add_file(&file_name, file_id);
                            
                            path_to_id_write.insert(changed_path, file_id);
                            files_write.insert(file_id, entry);
                        }
                    }
                } else {
                    // File was deleted
                    tracing::debug!("File deleted: {:?}", changed_path);
                    
                    let file_id = {
                        let mut path_to_id_write = path_to_id.write().await;
                        path_to_id_write.remove(&changed_path)
                    };

                    if let Some(file_id) = file_id {
                        let mut files_write = files.write().await;
                        if let Some(entry) = files_write.remove(&file_id) {
                            let mut trie_write = trie.write().await;
                            let mut trigram_write = trigram.write().await;

                            // Remove from trie
                            for word in entry.name.split(|c: char| !c.is_alphanumeric()) {
                                if !word.is_empty() {
                                    trie_write.remove(word, file_id);
                                }
                            }
                            
                            // Remove from trigram
                            trigram_write.remove_file(file_id);
                        }
                    }
                }
            });
        }).await.map_err(|e| e.to_string())?;

        // Store the watcher
        let mut watcher_lock = self.watcher.write().await;
        *watcher_lock = Some(watcher);

        tracing::info!("File watcher started for {:?}", paths);
        Ok(())
    }

    /// Search for files matching the query
    pub async fn search(&self, query: &str) -> Vec<FileEntry> {
        if query.is_empty() {
            return Vec::new();
        }

        let trie = self.trie.read().await;
        let trigram = self.trigram.read().await;
        let files = self.files.read().await;
        let ranker = self.ranker.read().await;

        // Get candidates from both indexes
        let mut candidate_ids = std::collections::HashSet::new();

        // Trie prefix search
        let trie_results = trie.search_prefix(query);
        candidate_ids.extend(trie_results);

        // Trigram fuzzy search
        let trigram_results = trigram.search(query);
        candidate_ids.extend(trigram_results.into_iter().map(|(id, _)| id));

        // Create file name map for ranking
        let file_names: HashMap<usize, String> = files
            .iter()
            .map(|(id, entry)| (*id, entry.name.clone()))
            .collect();

        // Rank results
        let ranked = ranker.rank_results(candidate_ids.into_iter().collect(), query, &file_names);

        // Return top results
        ranked
            .into_iter()
            .take(20)
            .filter_map(|score| files.get(&score.file_id).cloned())
            .collect()
    }

    /// Record that a file was accessed
    pub async fn record_access(&self, file_id: usize) {
        let mut ranker = self.ranker.write().await;
        ranker.record_access(file_id);
    }

    /// Get file entry by ID
    pub async fn get_file(&self, file_id: usize) -> Option<FileEntry> {
        let files = self.files.read().await;
        files.get(&file_id).cloned()
    }

    /// Get total number of indexed files
    pub async fn file_count(&self) -> usize {
        let files = self.files.read().await;
        files.len()
    }
}

impl Default for Indexer {
    fn default() -> Self {
        Self::new(ScanConfig::default())
    }
}
