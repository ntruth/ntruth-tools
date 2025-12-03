// Trie data structure for prefix matching
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_end: bool,
    pub file_ids: Vec<usize>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
            file_ids: Vec::new(),
        }
    }
}

impl Default for TrieNode {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    /// Insert a word into the trie with associated file ID
    pub fn insert(&mut self, word: &str, file_id: usize) {
        let word_lower = word.to_lowercase();
        let mut node = &mut self.root;

        for ch in word_lower.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }

        node.is_end = true;
        if !node.file_ids.contains(&file_id) {
            node.file_ids.push(file_id);
        }
    }

    /// Remove a word's file ID from the trie
    pub fn remove(&mut self, word: &str, file_id: usize) {
        let word_lower = word.to_lowercase();
        let chars: Vec<char> = word_lower.chars().collect();
        
        // Navigate to the node containing this word
        let mut node = &mut self.root;
        for ch in &chars {
            match node.children.get_mut(ch) {
                Some(child) => node = child,
                None => return, // Word not found
            }
        }
        
        // Remove file_id from this node
        node.file_ids.retain(|&id| id != file_id);
        
        // If no more file_ids, mark as not end
        if node.file_ids.is_empty() {
            node.is_end = false;
        }
        
        // Note: We don't clean up empty nodes for simplicity
        // A full implementation would traverse back and remove empty nodes
    }

    /// Search for words with given prefix
    pub fn search_prefix(&self, prefix: &str) -> Vec<usize> {
        let prefix_lower = prefix.to_lowercase();
        let mut node = &self.root;

        // Navigate to the prefix node
        for ch in prefix_lower.chars() {
            match node.children.get(&ch) {
                Some(next_node) => node = next_node,
                None => return Vec::new(),
            }
        }

        // Collect all file IDs from this node and its children
        self.collect_file_ids(node)
    }

    /// Fuzzy search - find words similar to the query
    pub fn fuzzy_search(&self, query: &str, max_distance: usize) -> Vec<usize> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        self.fuzzy_search_helper(&self.root, &query_lower, "", max_distance, &mut results);
        results
    }

    fn fuzzy_search_helper(
        &self,
        node: &TrieNode,
        query: &str,
        current: &str,
        max_distance: usize,
        results: &mut Vec<usize>,
    ) {
        // Calculate edit distance
        let distance = levenshtein_distance(current, query);

        if distance <= max_distance && node.is_end {
            results.extend_from_slice(&node.file_ids);
        }

        // Continue searching if we haven't exceeded max distance
        if current.len() < query.len() + max_distance {
            for (ch, child_node) in &node.children {
                let mut new_current = current.to_string();
                new_current.push(*ch);
                self.fuzzy_search_helper(child_node, query, &new_current, max_distance, results);
            }
        }
    }

    fn collect_file_ids(&self, node: &TrieNode) -> Vec<usize> {
        let mut file_ids = Vec::new();

        if node.is_end {
            file_ids.extend_from_slice(&node.file_ids);
        }

        for child in node.children.values() {
            file_ids.extend(self.collect_file_ids(child));
        }

        file_ids
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix: Vec<Vec<usize>> = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost,
            );
        }
    }

    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_insert_and_search() {
        let mut trie = Trie::new();
        trie.insert("hello", 1);
        trie.insert("world", 2);
        trie.insert("help", 3);

        let results = trie.search_prefix("hel");
        assert!(results.contains(&1));
        assert!(results.contains(&3));
        assert!(!results.contains(&2));
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
        assert_eq!(levenshtein_distance("hello", "hallo"), 1);
        assert_eq!(levenshtein_distance("hello", "world"), 4);
    }
}
