// Ranking algorithm for search results
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileScore {
    pub file_id: usize,
    pub score: f64,
}

pub struct Ranker {
    /// Track file access frequency
    access_count: HashMap<usize, u32>,
    /// Track last access time
    last_access: HashMap<usize, SystemTime>,
}

impl Ranker {
    pub fn new() -> Self {
        Self {
            access_count: HashMap::new(),
            last_access: HashMap::new(),
        }
    }

    /// Record a file access
    pub fn record_access(&mut self, file_id: usize) {
        *self.access_count.entry(file_id).or_insert(0) += 1;
        self.last_access.insert(file_id, SystemTime::now());
    }

    /// Rank search results based on various factors
    pub fn rank_results(
        &self,
        file_ids: Vec<usize>,
        query: &str,
        file_names: &HashMap<usize, String>,
    ) -> Vec<FileScore> {
        let mut scores: Vec<FileScore> = file_ids
            .into_iter()
            .map(|file_id| {
                let score = self.calculate_score(file_id, query, file_names);
                FileScore { file_id, score }
            })
            .collect();

        // Sort by score descending
        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        scores
    }

    fn calculate_score(
        &self,
        file_id: usize,
        query: &str,
        file_names: &HashMap<usize, String>,
    ) -> f64 {
        let file_name = match file_names.get(&file_id) {
            Some(name) => name,
            None => return 0.0,
        };

        let query_lower = query.to_lowercase();
        let name_lower = file_name.to_lowercase();

        // Base score: match quality
        let mut score = 0.0;

        // Exact match bonus
        if name_lower == query_lower {
            score += 100.0;
        }
        // Starts with query bonus
        else if name_lower.starts_with(&query_lower) {
            score += 50.0;
        }
        // Contains query bonus
        else if name_lower.contains(&query_lower) {
            score += 25.0;
        }

        // Word boundary bonus (query matches complete word)
        if is_word_boundary_match(&name_lower, &query_lower) {
            score += 20.0;
        }

        // Frequency bonus (logarithmic scale)
        if let Some(&count) = self.access_count.get(&file_id) {
            score += (count as f64).ln() * 5.0;
        }

        // Recency bonus
        if let Some(&last_access) = self.last_access.get(&file_id) {
            if let Ok(duration) = SystemTime::now().duration_since(last_access) {
                let days_ago = duration.as_secs() as f64 / 86400.0;
                // Decay factor: recent files get higher scores
                let recency_score = (-days_ago / 30.0).exp() * 10.0;
                score += recency_score;
            }
        }

        // Shorter file names get slight bonus (prefer concise matches)
        let length_penalty = (file_name.len() as f64 / 100.0).min(5.0);
        score -= length_penalty;

        score.max(0.0)
    }
}

impl Default for Ranker {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if query matches at word boundaries
fn is_word_boundary_match(text: &str, query: &str) -> bool {
    let words: Vec<&str> = text.split(|c: char| !c.is_alphanumeric()).collect();
    words.iter().any(|word| word.starts_with(query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_boundary_match() {
        assert!(is_word_boundary_match("hello-world", "hel"));
        assert!(is_word_boundary_match("hello-world", "wor"));
        assert!(!is_word_boundary_match("hello-world", "ello"));
    }

    #[test]
    fn test_calculate_score() {
        let ranker = Ranker::new();
        let mut file_names = HashMap::new();
        file_names.insert(1, "hello.txt".to_string());
        file_names.insert(2, "hello-world.txt".to_string());
        file_names.insert(3, "something-hello.txt".to_string());

        let score1 = ranker.calculate_score(1, "hello", &file_names);
        let score2 = ranker.calculate_score(2, "hello", &file_names);
        let score3 = ranker.calculate_score(3, "hello", &file_names);

        // Exact match should score highest
        assert!(score1 > score2);
        assert!(score1 > score3);
    }
}
