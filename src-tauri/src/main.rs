#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clipboard;
mod screenshot;
mod search;
mod workflow;
mod plugin;

use anyhow::{anyhow, Result};
use clipboard::{list_history, ClipboardItem, ClipboardPayload};
use screenshot::ScreenshotItem;
use search::{query_documents, SearchResult};
use workflow::{Workflow, WorkflowRunResult};
use plugin::{PluginManifest, PluginInstallRequest};
use serde::Deserialize;
use tauri::{
  menu::{MenuBuilder, MenuItemBuilder},
  tray::TrayIconBuilder,
  AppHandle, Manager, Runtime,
};
use tauri::Emitter;
use tauri::window::WindowBuilder;
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Deserialize)]
struct TranslationPayload {
  text: String,
  target: String,
}

#[tauri::command]
fn clipboard_get_history() -> Vec<ClipboardItem> {
  list_history()
}

#[tauri::command]
fn clipboard_save_entry(payload: ClipboardPayload) -> Result<ClipboardItem, String> {
  clipboard::add_entry(payload).map_err(|err| err.to_string())
}

#[tauri::command]
fn clipboard_set_pin(id: String, pinned: bool) -> Result<(), String> {
  clipboard::set_pin(&id, pinned).map_err(|err| err.to_string())
}

#[tauri::command]
fn clipboard_remove(id: String) -> Result<(), String> {
  clipboard::remove(&id).map_err(|err| err.to_string())
}

#[tauri::command]
fn clipboard_clear_unpinned() {
  clipboard::clear_non_pinned();
}

#[tauri::command]
fn screenshot_capture(note: Option<String>) -> Result<ScreenshotItem, String> {
  screenshot::capture_stub(note).map_err(|err| err.to_string())
}

#[tauri::command]
fn screenshot_list() -> Vec<ScreenshotItem> {
  screenshot::list()
}

#[tauri::command]
fn screenshot_set_pin(id: String, pinned: bool) -> Result<(), String> {
  screenshot::toggle_pin(&id, pinned).map_err(|err| err.to_string())
}

#[tauri::command]
fn screenshot_remove(id: String) -> Result<(), String> {
  screenshot::remove(&id).map_err(|err| err.to_string())
}

#[tauri::command]
fn search_files(query: String) -> Vec<SearchResult> {
  query_documents(&query)
}

#[tauri::command]
fn search_refresh() -> Result<(), String> {
  let cwd = std::env::current_dir().map_err(|err| err.to_string())?;
  search::refresh_index(&cwd).map_err(|err| err.to_string())
}

#[tauri::command]
fn workflow_list() -> Vec<Workflow> {
  workflow::list()
}

#[tauri::command]
fn workflow_save(workflow: Workflow) -> Result<Workflow, String> {
  workflow::save(workflow).map_err(|err| err.to_string())
}

#[tauri::command]
fn workflow_run<R: Runtime>(app: AppHandle<R>, id: String) -> Result<WorkflowRunResult, String> {
  workflow::run(&app, &id).map_err(|err| err.to_string())
}

#[tauri::command]
fn plugin_marketplace() -> Vec<PluginManifest> {
  plugin::marketplace()
}

#[tauri::command]
fn plugin_installed() -> Vec<PluginManifest> {
  plugin::installed()
}

#[tauri::command]
fn plugin_install(request: PluginInstallRequest) -> Result<PluginManifest, String> {
  plugin::install(request).map_err(|err| err.to_string())
}

#[tauri::command]
fn plugin_uninstall(id: String) -> Result<(), String> {
  plugin::uninstall(&id).map_err(|err| err.to_string())
}

#[tauri::command]
fn show_launcher<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
  ensure_launcher_window(&app).map_err(|err| err.to_string())?;
  if let Some(window) = app.get_window("launcher") {
    let visible = window.is_visible().unwrap_or(false);
    if visible {
      window.hide().map_err(|err| err.to_string())?;
    } else {
      window.show().map_err(|err| err.to_string())?;
      window.set_focus().ok();
    }
  }
  Ok(())
}

#[tauri::command]
fn show_settings<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
  if let Some(launcher) = app.get_window("launcher") {
    launcher.hide().ok();
  }
  if let Some(window) = app.get_window("main") {
    window.show().map_err(|err| err.to_string())?;
    window.set_focus().ok();
  }
  Ok(())
}

#[tauri::command]
fn execute_entry<R: Runtime>(
  app: AppHandle<R>,
  entry_id: String,
  payload: serde_json::Value,
) -> Result<(), String> {
  match entry_id.as_str() {
    "text-editor" => {
      #[cfg(target_os = "windows")]
      {
        app
          .shell()
          .command("notepad.exe")
          .spawn()
          .map_err(|err| err.to_string())?;
      }
      #[cfg(target_os = "macos")]
      {
        app
          .shell()
          .command("open")
          .args(["-a", "TextEdit"])
          .spawn()
          .map_err(|err| err.to_string())?;
      }
      #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
      {
        app
          .shell()
          .command("nano")
          .spawn()
          .map_err(|err| err.to_string())?;
      }
    }
    "terminal" => {
      #[cfg(target_os = "windows")]
      {
        app
          .shell()
          .command("powershell.exe")
          .spawn()
          .map_err(|err| err.to_string())?;
      }
      #[cfg(target_os = "macos")]
      {
        app
          .shell()
          .command("open")
          .args(["-a", "Terminal"])
          .spawn()
          .map_err(|err| err.to_string())?;
      }
      #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
      {
        app
          .shell()
          .command("gnome-terminal")
          .spawn()
          .map_err(|err| err.to_string())?;
      }
    }
    "screenshot-tool" => {
      #[cfg(target_os = "windows")]
      {
        app
          .shell()
          .command("snippingtool.exe")
          .spawn()
          .map_err(|err| err.to_string())?;
      }
      #[cfg(target_os = "macos")]
      {
        app
          .shell()
          .command("open")
          .args(["-a", "Screenshot"])
          .spawn()
          .map_err(|err| err.to_string())?;
      }
    }
    "system-settings" => {
      #[cfg(target_os = "windows")]
      {
        app
          .shell()
          .open("ms-settings:", None)
          .map_err(|err| err.to_string())?;
      }
      #[cfg(target_os = "macos")]
      {
        app
          .shell()
          .command("open")
          .args(["x-apple.systempreferences:"])
          .spawn()
          .map_err(|err| err.to_string())?;
      }
    }
    "unitools-docs" => {
      #[cfg(target_os = "macos")]
      {
        app
          .shell()
          .command("open")
          .args(["https://github.com/yourusername/unitools"])
          .spawn()
          .map_err(|err| err.to_string())?;
      }
      #[cfg(target_os = "windows")]
      {
        app
          .shell()
          .command("explorer.exe")
          .args(["https://github.com/yourusername/unitools"])
          .spawn()
          .map_err(|err| err.to_string())?;
      }
      #[cfg(target_os = "linux")]
      {
        app
          .shell()
          .command("xdg-open")
          .args(["https://github.com/yourusername/unitools"])
          .spawn()
          .map_err(|err| err.to_string())?;
      }
    }
    "unitools-workflow" => {
      if let Some(main) = app.get_window("main") {
        main
          .emit("workflow:open", serde_json::json!({ "source": "launcher" }))
          .map_err(|err| err.to_string())?;
      }
    }
    "calculator" => {
      let result = payload
        .get("result")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
      let message = format!("计算结果：{}", result);
      app
        .notification()
        .builder()
        .title("UniTools")
        .body(message)
        .show()
        .map_err(|err| err.to_string())?;
    }
    "translator" => {
      let translated = payload
        .get("translated")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
      app
        .notification()
        .builder()
        .title("UniTools")
        .body(translated)
        .show()
        .map_err(|err| err.to_string())?;
    }
    entry_id if entry_id.starts_with("web-search") => {
      if let Some(url) = payload
        .get("url")
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
      {
        app
          .shell()
          .open(url, None)
          .map_err(|err| err.to_string())?;
      } else {
        return Err("缺少有效的搜索地址".into());
      }
    }
    _ => {
      if let Some(path) = payload.get("path").and_then(|value| value.as_str()) {
        #[cfg(target_os = "macos")]
        {
          app
            .shell()
            .command("open")
            .args([path])
            .spawn()
            .map_err(|err| err.to_string())?;
        }
        #[cfg(target_os = "windows")]
        {
          app
            .shell()
            .command("explorer.exe")
            .args([path])
            .spawn()
            .map_err(|err| err.to_string())?;
        }
        #[cfg(target_os = "linux")]
        {
          app
            .shell()
            .command("xdg-open")
            .args([path])
            .spawn()
            .map_err(|err| err.to_string())?;
        }
      }
    }
  }
  Ok(())
}

#[tauri::command]
fn translate_text(payload: TranslationPayload) -> Result<String, String> {
  let dictionary = [
    ("你好", "hello"),
    ("谢谢", "thank you"),
    ("剪贴板", "clipboard"),
    ("搜索", "search"),
  ];

  fn normalize(input: &str) -> String {
    input.trim().to_lowercase()
  }

  let text = normalize(&payload.text);
  let target = normalize(&payload.target);

  let output = match target.as_str() {
    "en" => dictionary
      .iter()
      .find(|(key, _)| normalize(key) == text)
      .map(|(_, value)| value.to_string())
      .unwrap_or_else(|| payload.text.clone()),
    "zh" => dictionary
      .iter()
      .find(|(_, value)| normalize(value) == text)
      .map(|(key, _)| key.to_string())
      .unwrap_or_else(|| payload.text.clone()),
    _ => payload.text.clone(),
  };

  Ok(output)
}

fn ensure_launcher_window<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
  if app.get_window("launcher").is_some() {
    return Ok(());
  }

  let config = app
    .config()
    .app
    .windows
    .iter()
    .find(|window| window.label == "launcher")
    .ok_or_else(|| anyhow!("launcher window configuration missing"))?
    .clone();

  WindowBuilder::from_config(app, &config)?.build()?;

  Ok(())
}

fn attach_tray<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
  #[cfg(desktop)]
  {
    let show_preferences = MenuItemBuilder::with_id("show_settings", "偏好设置").build(app)?;
    let toggle_launcher = MenuItemBuilder::with_id("toggle_launcher", "切换启动器").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    let menu = MenuBuilder::new(app)
      .item(&show_preferences)
      .item(&toggle_launcher)
      .separator()
      .item(&quit)
      .build()?;

    let show_preferences_id = show_preferences.id().clone();
    let toggle_launcher_id = toggle_launcher.id().clone();
    let quit_id = quit.id().clone();

    let mut builder = TrayIconBuilder::new()
      .menu(&menu)
      .on_menu_event(move |app, event| {
        let identifier = event.id();
        if identifier == &show_preferences_id {
          let _ = show_settings(app.clone());
        } else if identifier == &toggle_launcher_id {
          let _ = show_launcher(app.clone());
        } else if identifier == &quit_id {
          app.exit(0);
        }
      });

    if let Some(icon) = app.default_window_icon().cloned() {
      builder = builder.icon(icon);
    }

    builder.build(app)?;
  }

  Ok(())
}

fn register_shortcuts<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
  attach_tray(app)?;
  ensure_launcher_window(app)?;

  let shortcuts = app.global_shortcut();

  #[cfg(target_os = "macos")]
  {
    shortcuts.on_shortcut("Option+Space", move |handle, _, _| {
      let _ = show_launcher(handle.clone());
    })?;

    shortcuts.on_shortcut("Super+Comma", move |handle, _, _| {
      let _ = show_settings(handle.clone());
    })?;

    shortcuts.on_shortcut("Super+Shift+V", move |handle, _, _| {
      if let Some(main) = handle.get_window("main") {
        main.emit("clipboard:capture", serde_json::Value::Null).ok();
        main.emit("clipboard:toggle", serde_json::Value::Null).ok();
        main.show().ok();
        main.set_focus().ok();
      }
    })?;
  }

  #[cfg(not(target_os = "macos"))]
  {
    shortcuts.on_shortcut("Alt+Space", move |handle, _, _| {
      let _ = show_launcher(handle.clone());
    })?;

    shortcuts.on_shortcut("Control+Comma", move |handle, _, _| {
      let _ = show_settings(handle.clone());
    })?;

    shortcuts.on_shortcut("Ctrl+Shift+V", move |handle, _, _| {
      if let Some(main) = handle.get_window("main") {
        main.emit("clipboard:capture", serde_json::Value::Null).ok();
        main.emit("clipboard:toggle", serde_json::Value::Null).ok();
        main.show().ok();
        main.set_focus().ok();
      }
    })?;
  }

  Ok(())
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_global_shortcut::Builder::new().build())
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_sql::Builder::default().build())
    .setup(|app| {
      if let Ok(cwd) = std::env::current_dir() {
        search::initialize_index(&cwd);
      }
      let handle = app.handle();
      register_shortcuts(&handle)?;
      if let Some(main) = app.get_window("main") {
        main.hide().ok();
      }
      let _ = show_launcher(handle.clone());
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      show_launcher,
      show_settings,
      execute_entry,
      translate_text,
      clipboard_get_history,
      clipboard_save_entry,
      clipboard_set_pin,
      clipboard_remove,
      clipboard_clear_unpinned,
      screenshot_capture,
      screenshot_list,
      screenshot_set_pin,
      screenshot_remove,
      search_files,
      search_refresh,
      workflow_list,
      workflow_save,
      workflow_run,
      plugin_marketplace,
      plugin_installed,
      plugin_install,
      plugin_uninstall
    ])
    .run(tauri::generate_context!())
    .expect("error while running UniTools");
}
