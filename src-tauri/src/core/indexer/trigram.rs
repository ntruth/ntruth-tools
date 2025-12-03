// Trigram indexing for fuzzy search
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct TrigramIndex {
    /// Map from trigram to set of file IDs
    index: HashMap<String, HashSet<usize>>,
}

impl TrigramIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    /// Add a file to the trigram index
    pub fn add_file(&mut self, text: &str, file_id: usize) {
        let trigrams = extract_trigrams(&text.to_lowercase());

        for trigram in trigrams {
            self.index
                .entry(trigram)
                .or_insert_with(HashSet::new)
                .insert(file_id);
        }
    }

    /// Search for files matching the query using trigrams
    pub fn search(&self, query: &str) -> Vec<(usize, f64)> {
        let query_trigrams = extract_trigrams(&query.to_lowercase());

        if query_trigrams.is_empty() {
            return Vec::new();
        }

        // Count how many query trigrams each file matches
        let mut file_scores: HashMap<usize, usize> = HashMap::new();

        for trigram in &query_trigrams {
            if let Some(file_ids) = self.index.get(trigram) {
                for &file_id in file_ids {
                    *file_scores.entry(file_id).or_insert(0) += 1;
                }
            }
        }

        // Calculate similarity score (Jaccard similarity)
        let mut results: Vec<(usize, f64)> = file_scores
            .into_iter()
            .map(|(file_id, matches)| {
                let score = matches as f64 / query_trigrams.len() as f64;
                (file_id, score)
            })
            .filter(|(_, score)| *score > 0.3) // Minimum similarity threshold
            .collect();

        // Sort by score descending
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        results
    }

    /// Remove a file from the index
    pub fn remove_file(&mut self, file_id: usize) {
        for file_ids in self.index.values_mut() {
            file_ids.remove(&file_id);
        }
    }
}

impl Default for TrigramIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract trigrams from text
fn extract_trigrams(text: &str) -> Vec<String> {
    let text = format!("  {}  ", text); // Add padding
    let chars: Vec<char> = text.chars().collect();

    if chars.len() < 3 {
        return Vec::new();
    }

    let mut trigrams = Vec::new();
    for i in 0..chars.len() - 2 {
        let trigram: String = chars[i..i + 3].iter().collect();
        trigrams.push(trigram);
    }

    trigrams
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_trigrams() {
        let trigrams = extract_trigrams("hello");
        assert!(trigrams.contains(&"  h".to_string()));
        assert!(trigrams.contains(&" he".to_string()));
        assert!(trigrams.contains(&"hel".to_string()));
        assert!(trigrams.contains(&"ell".to_string()));
        assert!(trigrams.contains(&"llo".to_string()));
        assert!(trigrams.contains(&"lo ".to_string()));
        assert!(trigrams.contains(&"o  ".to_string()));
    }

    #[test]
    fn test_trigram_search() {
        let mut index = TrigramIndex::new();
        index.add_file("hello world", 1);
        index.add_file("hello there", 2);
        index.add_file("goodbye", 3);

        let results = index.search("hello");
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|(id, _)| *id == 1));
        assert!(results.iter().any(|(id, _)| *id == 2));
    }
}
