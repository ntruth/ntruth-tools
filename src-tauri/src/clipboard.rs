use std::cmp::Reverse;

use anyhow::Result;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
  pub id: String,
  pub r#type: String,
  pub content: String,
  pub created_at: DateTime<Utc>,
  pub pinned: bool,
  pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClipboardPayload {
  pub content: String,
  #[serde(default)]
  pub r#type: Option<String>,
  #[serde(default)]
  pub tags: Option<Vec<String>>,
}

static HISTORY: Lazy<Mutex<Vec<ClipboardItem>>> = Lazy::new(|| {
  let now = Utc::now();
  Mutex::new(vec![
    ClipboardItem {
      id: "clp-001".into(),
      r#type: "text".into(),
      content: "快速启动器：ALT + SPACE".into(),
      created_at: now,
      pinned: true,
      tags: vec!["hotkey".into(), "launcher".into()],
    },
    ClipboardItem {
      id: "clp-002".into(),
      r#type: "text".into(),
      content: "https://github.com/yourusername/unitools".into(),
      created_at: now - chrono::Duration::minutes(5),
      pinned: false,
      tags: vec!["link".into()],
    },
  ])
});

fn generate_id() -> String {
  format!("clp-{}", Utc::now().timestamp_millis())
}

pub fn list_history() -> Vec<ClipboardItem> {
  let mut items = HISTORY.lock().clone();
  items.sort_by_key(|item| (Reverse(item.pinned), Reverse(item.created_at)));
  items
}

pub fn add_entry(payload: ClipboardPayload) -> Result<ClipboardItem> {
  let mut history = HISTORY.lock();

  if payload.content.trim().is_empty() {
    anyhow::bail!("无法保存空的剪贴板内容");
  }

  let item = ClipboardItem {
    id: generate_id(),
    r#type: payload.r#type.unwrap_or_else(|| "text".into()),
    content: payload.content,
    created_at: Utc::now(),
    pinned: false,
    tags: payload.tags.unwrap_or_default(),
  };

  history.insert(0, item.clone());
  if history.len() > 200 {
    history.truncate(200);
  }

  Ok(item)
}

pub fn set_pin(id: &str, pinned: bool) -> Result<()> {
  let mut history = HISTORY.lock();
  if let Some(item) = history.iter_mut().find(|entry| entry.id == id) {
    item.pinned = pinned;
    return Ok(());
  }
  anyhow::bail!("未找到剪贴板记录");
}

pub fn remove(id: &str) -> Result<()> {
  let mut history = HISTORY.lock();
  let original_len = history.len();
  history.retain(|item| item.id != id);
  if history.len() == original_len {
    anyhow::bail!("未找到剪贴板记录");
  }
  Ok(())
}

pub fn clear_non_pinned() {
  let mut history = HISTORY.lock();
  history.retain(|item| item.pinned);
}
