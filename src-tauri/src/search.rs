use std::cmp::Reverse;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocument {
  pub id: String,
  pub name: String,
  pub path: String,
  pub ext: Option<String>,
  pub size: u64,
  pub modified: DateTime<Utc>,
}

static INDEX: Lazy<Mutex<Vec<SearchDocument>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn normalize(text: &str) -> String {
  text
    .to_lowercase()
    .chars()
    .filter(|ch| !ch.is_whitespace())
    .collect::<String>()
}

fn canonicalize_path(path: &Path) -> PathBuf {
  path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn resolve_root(path: &Path) -> Option<PathBuf> {
  if path.exists() {
    Some(canonicalize_path(path))
  } else {
    None
  }
}

fn file_entry(path: &Path) -> Option<SearchDocument> {
  let metadata = fs::metadata(path).ok()?;
  let is_app_bundle = metadata.is_dir() && path.extension().map(|ext| ext == "app").unwrap_or(false);
  if !metadata.is_file() && !is_app_bundle {
    return None;
  }
  let canonical_path = canonicalize_path(path);
  let name = path.file_name()?.to_string_lossy().to_string();
  let modified: DateTime<Utc> = metadata
    .modified()
    .ok()
    .map(DateTime::<Utc>::from)
    .unwrap_or_else(Utc::now);

  Some(SearchDocument {
    id: format!(
      "doc-{}",
      canonical_path
        .to_string_lossy()
        .replace(['/', '\\', ' ', '.'], "_")
    ),
    name,
    path: canonical_path.to_string_lossy().to_string(),
    ext: path.extension().map(|ext| ext.to_string_lossy().to_string()),
    size: if metadata.is_file() { metadata.len() } else { 0 },
    modified,
  })
}

fn collect_documents(root: &Path) -> Result<Vec<SearchDocument>> {
  let mut documents = Vec::new();
  for entry in WalkDir::new(root)
    .max_depth(4)
    .follow_links(true)
    .into_iter()
    .filter_map(|entry| entry.ok())
  {
    if entry.file_type().is_file() || entry.path().extension().map(|ext| ext == "app").unwrap_or(false) {
      if let Some(doc) = file_entry(entry.path()) {
        documents.push(doc);
      }
    }
  }

  documents.sort_by_key(|doc| Reverse(doc.modified));

  Ok(documents)
}

pub fn refresh_index(root: &Path) -> Result<()> {
  if let Some(resolved) = resolve_root(root) {
    let mut documents = collect_documents(&resolved)?;
    let mut index = INDEX.lock();
    index.retain(|doc| {
      let doc_path = Path::new(&doc.path);
      !doc_path.starts_with(&resolved)
    });
    index.append(&mut documents);
    index.sort_by_key(|doc| Reverse(doc.modified));
  }
  Ok(())
}

pub fn initialize_index(app_dir: &Path) {
  let mut roots = vec![app_dir.join("docs"), app_dir.join("src"), app_dir.join("src-tauri")];

  #[cfg(target_os = "macos")]
  {
    if let Ok(home) = env::var("HOME") {
      roots.push(Path::new(&home).join("Applications"));
      roots.push(Path::new(&home).join("Documents"));
      roots.push(Path::new(&home).join("Downloads"));
    }
    roots.push(Path::new("/Applications").to_path_buf());
    roots.push(Path::new("/System/Applications").to_path_buf());
  }

  #[cfg(not(target_os = "macos"))]
  {
    if let Ok(home) = env::var("HOME") {
      roots.push(Path::new(&home).join("Documents"));
      roots.push(Path::new(&home).join("Downloads"));
    }
  }

  let mut aggregated = Vec::new();
  for root in roots {
    if let Some(resolved) = resolve_root(&root) {
      if let Ok(mut docs) = collect_documents(&resolved) {
        aggregated.append(&mut docs);
      }
    }
  }

  aggregated.sort_by_key(|doc| Reverse(doc.modified));

  let mut index = INDEX.lock();
  *index = aggregated;
}

#[derive(Debug, Serialize, Clone)]
pub struct SearchResult {
  pub document: SearchDocument,
  pub score: f64,
  pub highlight: Option<String>,
}

pub fn query_documents(query: &str) -> Vec<SearchResult> {
  if query.trim().is_empty() {
    return INDEX
      .lock()
      .iter()
      .take(20)
      .map(|doc| SearchResult {
        document: doc.clone(),
        score: 1.0,
        highlight: None,
      })
      .collect();
  }

  let normalized_query = normalize(query);
  let tokens: Vec<&str> = query
    .split_whitespace()
    .filter(|token| !token.is_empty())
    .collect();

  let mut results = Vec::new();
  for document in INDEX.lock().iter() {
    let mut score = 0.0;
    let normalized_name = normalize(&document.name);
    if normalized_name.contains(&normalized_query) {
      score += 5.0;
    }

    for token in &tokens {
      if document.name.to_lowercase().contains(&token.to_lowercase()) {
        score += 2.0;
      }
      if let Some(ext) = &document.ext {
        if ext.to_lowercase() == token.to_lowercase() {
          score += 1.5;
        }
      }
    }

    if score > 0.0 {
      let highlight = tokens
        .iter()
        .find(|token| document.name.to_lowercase().contains(&token.to_lowercase()))
        .map(|token| token.to_string());

      results.push(SearchResult {
        document: document.clone(),
        score,
        highlight,
      });
    }
  }

  results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
  results.truncate(30);
  results
}
