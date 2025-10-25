use anyhow::{bail, Result};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
  pub id: String,
  pub name: String,
  pub version: String,
  pub author: String,
  pub summary: String,
  pub category: String,
  #[serde(default)]
  pub repository: Option<String>,
  #[serde(default)]
  pub homepage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInstallRequest {
  pub id: String,
}

static MARKETPLACE: Lazy<Vec<PluginManifest>> = Lazy::new(|| {
  vec![
    PluginManifest {
      id: "plugin-ocr".into(),
      name: "视觉 OCR".into(),
      version: "0.2.1".into(),
      author: "Vision Lab".into(),
      summary: "对截图执行 OCR 识别并复制文本".into(),
      category: "效率".into(),
      repository: Some("https://github.com/yourusername/unitools-ocr".into()),
      homepage: None,
    },
    PluginManifest {
      id: "plugin-translator".into(),
      name: "AI 翻译增强".into(),
      version: "1.1.0".into(),
      author: "UniTools Team".into(),
      summary: "连接云端翻译服务，增强工作流节点".into(),
      category: "语言".into(),
      repository: None,
      homepage: Some("https://unitools.dev/plugins/translator".into()),
    },
    PluginManifest {
      id: "plugin-todo".into(),
      name: "任务同步".into(),
      version: "0.9.5".into(),
      author: "Productivity Inc.".into(),
      summary: "将剪贴板中的待办事项同步到任务平台".into(),
      category: "任务".into(),
      repository: Some("https://git.example.com/plugins/todo".into()),
      homepage: None,
    },
  ]
});

static INSTALLED: Lazy<Mutex<Vec<PluginManifest>>> = Lazy::new(|| {
  Mutex::new(vec![PluginManifest {
    id: "plugin-translator".into(),
    name: "AI 翻译增强".into(),
    version: "1.1.0".into(),
    author: "UniTools Team".into(),
    summary: "连接云端翻译服务，增强工作流节点".into(),
    category: "语言".into(),
    repository: None,
    homepage: Some("https://unitools.dev/plugins/translator".into()),
  }])
});

pub fn marketplace() -> Vec<PluginManifest> {
  MARKETPLACE.clone()
}

pub fn installed() -> Vec<PluginManifest> {
  INSTALLED.lock().clone()
}

pub fn install(request: PluginInstallRequest) -> Result<PluginManifest> {
  let plugin = MARKETPLACE
    .iter()
    .find(|item| item.id == request.id)
    .cloned()
    .ok_or_else(|| anyhow::anyhow!("未找到插件"))?;

  let mut installed = INSTALLED.lock();
  if installed.iter().any(|item| item.id == plugin.id) {
    bail!("插件已安装");
  }
  installed.push(plugin.clone());
  Ok(plugin)
}

pub fn uninstall(id: &str) -> Result<()> {
  let mut installed = INSTALLED.lock();
  let original_len = installed.len();
  installed.retain(|item| item.id != id);
  if installed.len() == original_len {
    bail!("插件未安装");
  }
  Ok(())
}
