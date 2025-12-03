// File indexer module
mod trie;
mod trigram;
mod scanner;
mod ranker;

pub use scanner::{FileScanner, ScanConfig, FileEntry};
pub use ranker::{Ranker, FileScore};

use trie::Trie;
use trigram::TrigramIndex;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main file indexer that combines Trie and Trigram indexing
pub struct Indexer {
    /// Trie for prefix matching
    trie: Arc<RwLock<Trie>>,
    /// Trigram index for fuzzy search
    trigram: Arc<RwLock<TrigramIndex>>,
    /// File entries by ID
    files: Arc<RwLock<HashMap<usize, FileEntry>>>,
    /// File scanner
    scanner: FileScanner,
    /// Ranking algorithm
    ranker: Arc<RwLock<Ranker>>,
}

impl Indexer {
    pub fn new(config: ScanConfig) -> Self {
        Self {
            trie: Arc::new(RwLock::new(Trie::new())),
            trigram: Arc::new(RwLock::new(TrigramIndex::new())),
            files: Arc::new(RwLock::new(HashMap::new())),
            scanner: FileScanner::new(config),
            ranker: Arc::new(RwLock::new(Ranker::new())),
        }
    }

    /// Index a directory
    pub async fn index_directory(&self, path: &Path) -> Result<usize, String> {
        let entries = self.scanner.scan_directory(path).await;
        let count = entries.len();

        let mut trie = self.trie.write().await;
        let mut trigram = self.trigram.write().await;
        let mut files = self.files.write().await;

        for entry in entries {
            let file_id = entry.id;
            let file_name = entry.name.clone();

            // Add to trie (word-by-word)
            for word in file_name.split(|c: char| !c.is_alphanumeric()) {
                if !word.is_empty() {
                    trie.insert(word, file_id);
                }
            }

            // Add to trigram index
            trigram.add_file(&file_name, file_id);

            // Store file entry
            files.insert(file_id, entry);
        }

        Ok(count)
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
