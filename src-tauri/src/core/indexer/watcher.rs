// File system watcher for incremental indexing
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

pub struct FileWatcher {
    paths: Vec<PathBuf>,
}

impl FileWatcher {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    /// Add a path to watch
    pub fn add_path(&mut self, path: PathBuf) {
        if !self.paths.contains(&path) {
            self.paths.push(path);
        }
    }

    /// Start watching for file changes
    pub async fn start_watching<F>(
        &self,
        on_change: F,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(PathBuf) + Send + 'static,
    {
        let (tx, mut rx) = mpsc::channel::<PathBuf>(100);

        // Create watcher
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                            for path in event.paths {
                                let _ = tx.blocking_send(path);
                            }
                        }
                        _ => {}
                    }
                }
            },
            notify::Config::default()
                .with_poll_interval(Duration::from_secs(2))
                .with_compare_contents(false),
        )?;

        // Watch all paths
        for path in &self.paths {
            watcher.watch(path, RecursiveMode::Recursive)?;
        }

        // Keep watcher alive and process events
        tokio::spawn(async move {
            let mut debounce_map: std::collections::HashMap<PathBuf, tokio::time::Instant> =
                std::collections::HashMap::new();
            let debounce_duration = Duration::from_secs(2);

            while let Some(path) = rx.recv().await {
                // Debounce: only process if enough time has passed since last event
                let now = tokio::time::Instant::now();
                if let Some(&last_time) = debounce_map.get(&path) {
                    if now.duration_since(last_time) < debounce_duration {
                        continue;
                    }
                }
                debounce_map.insert(path.clone(), now);

                // Process the change
                on_change(path);
            }

            // Keep watcher alive
            drop(watcher);
        });

        Ok(())
    }
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_path() {
        let mut watcher = FileWatcher::new();
        let path = PathBuf::from("/test/path");
        watcher.add_path(path.clone());
        assert_eq!(watcher.paths.len(), 1);
        assert_eq!(watcher.paths[0], path);
    }
}
