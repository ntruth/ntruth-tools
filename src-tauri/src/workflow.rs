use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::{DateTime, Utc};
use tauri::{AppHandle, Runtime};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_notification::NotificationExt;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
  pub id: String,
  pub kind: String,
  pub label: String,
  #[serde(default)]
  pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTrigger {
  pub r#type: String,
  pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
  pub id: String,
  pub name: String,
  pub description: String,
  #[serde(default)]
  pub trigger: Option<WorkflowTrigger>,
  pub nodes: Vec<WorkflowNode>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRunResult {
  pub workflow_id: String,
  pub executed_at: DateTime<Utc>,
  pub logs: Vec<String>,
  pub status: String,
}

static WORKFLOWS: Lazy<Mutex<Vec<Workflow>>> = Lazy::new(|| {
  let now = Utc::now();
  Mutex::new(vec![Workflow {
    id: "wf-clipboard-to-notes".into(),
    name: "保存剪贴板到笔记".into(),
    description: "复制文本后自动保存至项目笔记文件".into(),
    trigger: Some(WorkflowTrigger {
      r#type: "clipboard".into(),
      value: Some("text".into()),
    }),
    nodes: vec![
      WorkflowNode {
        id: "node-read".into(),
        kind: "read_clipboard".into(),
        label: "读取剪贴板".into(),
        config: json!({ "format": "text" }),
      },
      WorkflowNode {
        id: "node-append".into(),
        kind: "append_file".into(),
        label: "追加到笔记".into(),
        config: json!({
          "path": "notes/inbox.md",
          "prefix": "- "
        }),
      },
      WorkflowNode {
        id: "node-notify".into(),
        kind: "notify".into(),
        label: "发送系统通知".into(),
        config: json!({ "title": "UniTools", "message": "已保存至笔记" }),
      },
    ],
    updated_at: now,
  }, Workflow {
    id: "wf-screenshot-to-board".into(),
    name: "截图归档".into(),
    description: "捕获截图并同步到贴图面板".into(),
    trigger: Some(WorkflowTrigger {
      r#type: "shortcut".into(),
      value: Some("Ctrl+Alt+S".into()),
    }),
    nodes: vec![
      WorkflowNode {
        id: "node-capture".into(),
        kind: "capture_screen".into(),
        label: "捕获截图".into(),
        config: json!({ "mode": "window" }),
      },
      WorkflowNode {
        id: "node-pin".into(),
        kind: "pin_canvas".into(),
        label: "固定到画布".into(),
        config: json!({ "board": "design" }),
      },
    ],
    updated_at: now,
  }])
});

fn generate_id() -> String {
  format!("wf-{}", Utc::now().timestamp_millis())
}

pub fn list() -> Vec<Workflow> {
  WORKFLOWS
    .lock()
    .iter()
    .cloned()
    .collect::<Vec<_>>()
}

pub fn save(mut workflow: Workflow) -> Result<Workflow> {
  let mut workflows = WORKFLOWS.lock();
  workflow.updated_at = Utc::now();
  if workflow.id.is_empty() {
    workflow.id = generate_id();
  }

  if let Some(current) = workflows.iter_mut().find(|item| item.id == workflow.id) {
    *current = workflow.clone();
    return Ok(workflow);
  }

  workflows.push(workflow.clone());
  Ok(workflow)
}

pub fn run<R: Runtime>(app: &AppHandle<R>, id: &str) -> Result<WorkflowRunResult> {
  let workflows = WORKFLOWS.lock();
  let workflow = workflows
    .iter()
    .find(|item| item.id == id)
    .cloned()
    .ok_or_else(|| anyhow!("未找到工作流"))?;

  let mut logs = Vec::new();
  let mut clipboard_buffer: Option<String> = None;

  for node in &workflow.nodes {
    match node.kind.as_str() {
      "read_clipboard" => {
        let text = app.clipboard().read_text().unwrap_or_default();
        clipboard_buffer = Some(text.clone());
        logs.push(format!("读取剪贴板: {}", text));
      }
      "append_file" => {
        let target = node
          .config
          .get("path")
          .and_then(|value| value.as_str())
          .unwrap_or("~/Library/Logs/unitools-workflow.txt");
        let expanded = expand_path(target);
        if let Some(parent) = expanded.parent() {
          let _ = std::fs::create_dir_all(parent);
        }
        let mut file = OpenOptions::new()
          .create(true)
          .append(true)
          .open(&expanded)?;
        let use_clipboard = node
          .config
          .get("useClipboard")
          .and_then(|value| value.as_bool())
          .unwrap_or(true);
        let content = node
          .config
          .get("content")
          .and_then(|value| value.as_str())
          .map(|value| value.to_string())
          .filter(|value| !value.is_empty())
          .or_else(|| if use_clipboard { clipboard_buffer.clone() } else { None })
          .unwrap_or_default();
        if !content.is_empty() {
          writeln!(file, "{}", content)?;
        }
        logs.push(format!(
          "写入文件 {}",
          expanded.to_string_lossy()
        ));
      }
      "notify" => {
        let title = node
          .config
          .get("title")
          .and_then(|value| value.as_str())
          .unwrap_or("UniTools 工作流");
        let body = node
          .config
          .get("message")
          .and_then(|value| value.as_str())
          .unwrap_or("操作完成");
        app
          .notification()
          .builder()
          .title(title)
          .body(body)
          .show()?;
        logs.push(format!("通知: {}", body));
      }
      "capture_screen" => {
        crate::screenshot::capture_stub(None)?;
        logs.push("截图已保存到贴图列表".into());
      }
      "pin_canvas" => {
        logs.push("预留：固定截图到贴图面板".into());
      }
      other => {
        logs.push(format!("未识别的节点 {}", other));
      }
    }
  }

  Ok(WorkflowRunResult {
    workflow_id: workflow.id,
    executed_at: Utc::now(),
    logs,
    status: "success".into(),
  })
}

fn expand_path(path: &str) -> PathBuf {
  if let Some(stripped) = path.strip_prefix("~/") {
    if let Ok(home) = env::var("HOME") {
      return PathBuf::from(home).join(stripped);
    }
  }
  PathBuf::from(path)
}
