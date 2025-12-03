

**基于 Tauri 2.0 + Rust**

**版本**：v2.0

**日期**：2025年12月3日

---

## 一、技术栈总览

### 1.1 核心技术选型

| 层级 | 技术选型 | 版本 | 说明 |
| --- | --- | --- | --- |
| **前端框架** | SolidJS | 1.8+ | 高性能响应式 UI，细粒度更新 |
| **前端语言** | TypeScript | 5.0+ | 类型安全 |
| **前端构建** | Vite | 5.0+ | 快速 HMR，ESBuild |
| **样式方案** | TailwindCSS | 3.4+ | 原子化 CSS |
| **后端框架** | Tauri | 2.0+ | Rust 跨平台桌面框架 |
| **后端语言** | Rust | 1.75+ | 系统级性能 |
| **异步运行时** | Tokio | 1.35+ | 异步 I/O |
| **数据库** | SQLite + SQLCipher | 3.45+ | 本地加密存储 |
| **ORM** | SQLx | 0.7+ | 编译时 SQL 检查 |
| **序列化** | Serde | 1.0+ | 高性能序列化 |
| **日志** | tracing | 0.1+ | 结构化日志 |
| **HTTP 客户端** | reqwest | 0.11+ | 异步 HTTP |

### 1.2 项目结构

```
omnibox/
├── src/                          # 前端源码 (SolidJS)
│   ├── App.tsx                   # 应用入口
│   ├── main.tsx                  # 渲染入口
│   ├── index.html                # HTML 模板
│   │
│   ├── components/               # 通用组件
│   │   ├── ui/                   # 基础 UI 组件
│   │   │   ├── Button.tsx
│   │   │   ├── Input.tsx
│   │   │   ├── Modal.tsx
│   │   │   ├── Dropdown.tsx
│   │   │   └── Toggle.tsx
│   │   ├── SearchBox/            # 搜索框组件
│   │   ├── ResultList/           # 结果列表组件
│   │   └── ActionBar/            # 操作栏组件
│   │
│   ├── pages/                    # 页面组件
│   │   ├── Main/                 # 主搜索窗口
│   │   │   ├── index.tsx
│   │   │   ├── SearchInput.tsx
│   │   │   └── ResultItem.tsx
│   │   ├── Clipboard/            # 剪贴板历史窗口
│   │   │   ├── index.tsx
│   │   │   ├── ClipboardItem.tsx
│   │   │   └── ClipboardPreview.tsx
│   │   ├── Screenshot/           # 截图相关
│   │   │   ├── Overlay.tsx       # 选区覆盖层
│   │   │   ├── Editor.tsx        # 标注编辑器
│   │   │   └── Pin.tsx           # 贴图窗口
│   │   ├── AI/                   # AI 对话窗口
│   │   │   ├── index.tsx
│   │   │   ├── ChatMessage.tsx
│   │   │   └── PromptInput.tsx
│   │   └── Settings/             # 配置中心
│   │       ├── index.tsx         # 设置主页面
│   │       ├── Layout.tsx        # 左右布局
│   │       ├── General.tsx
│   │       ├── Features.tsx
│   │       ├── Appearance.tsx
│   │       ├── Clipboard.tsx
│   │       ├── Screenshot.tsx
│   │       ├── AISettings.tsx
│   │       ├── Workflows/
│   │       │   ├── index.tsx
│   │       │   ├── WorkflowList.tsx
│   │       │   └── WorkflowEditor.tsx
│   │       ├── Plugins/
│   │       │   ├── index.tsx
│   │       │   ├── Installed.tsx
│   │       │   └── Store.tsx
│   │       ├── WebSearch.tsx
│   │       ├── Advanced.tsx
│   │       └── About.tsx
│   │
│   ├── stores/                   # 状态管理 (Solid Stores)
│   │   ├── search.ts             # 搜索状态
│   │   ├── clipboard.ts          # 剪贴板状态
│   │   ├── screenshot.ts         # 截图状态
│   │   ├── ai.ts                 # AI 对话状态
│   │   ├── workflow.ts           # 工作流状态
│   │   ├── settings.ts           # 设置状态
│   │   └── ui.ts                 # UI 状态
│   │
│   ├── hooks/                    # 自定义 Hooks
│   │   ├── useKeyboard.ts        # 键盘事件
│   │   ├── useTauriEvent.ts      # Tauri 事件监听
│   │   ├── useDebounce.ts        # 防抖
│   │   └── useClickOutside.ts    # 点击外部
│   │
│   ├── services/                 # 前端服务层
│   │   ├── tauri.ts              # Tauri API 封装
│   │   ├── search.ts             # 搜索服务
│   │   ├── clipboard.ts          # 剪贴板服务
│   │   └── ai.ts                 # AI 服务
│   │
│   ├── utils/                    # 工具函数
│   │   ├── format.ts             # 格式化
│   │   ├── platform.ts           # 平台判断
│   │   └── shortcut.ts           # 快捷键处理
│   │
│   ├── types/                    # TypeScript 类型
│   │   ├── search.ts
│   │   ├── clipboard.ts
│   │   ├── screenshot.ts
│   │   ├── ai.ts
│   │   ├── workflow.ts
│   │   ├── plugin.ts
│   │   └── settings.ts
│   │
│   └── styles/                   # 全局样式
│       ├── globals.css
│       └── themes.css
│
├── src-tauri/                    # Rust 后端
│   ├── Cargo.toml                # Rust 依赖
│   ├── tauri.conf.json           # Tauri 配置
│   ├── build.rs                  # 构建脚本
│   ├── icons/                    # 应用图标
│   │
│   └── src/
│       ├── main.rs               # 应用入口
│       ├── lib.rs                # 库导出
│       │
│       ├── app/                  # 应用核心
│       │   ├── mod.rs
│       │   ├── state.rs          # 全局状态管理
│       │   ├── config.rs         # 配置定义与管理
│       │   ├── error.rs          # 错误定义
│       │   ├── shortcuts.rs      # 全局快捷键
│       │   └── tray.rs           # 系统托盘
│       │
│       ├── commands/             # Tauri Commands (IPC)
│       │   ├── mod.rs
│       │   ├── search.rs         # 搜索相关命令
│       │   ├── clipboard.rs      # 剪贴板命令
│       │   ├── screenshot.rs     # 截图命令
│       │   ├── ai.rs             # AI 命令
│       │   ├── workflow.rs       # 工作流命令
│       │   ├── plugin.rs         # 插件命令
│       │   ├── settings.rs       # 设置命令
│       │   └── system.rs         # 系统命令
│       │
│       ├── core/                 # 核心业务模块
│       │   ├── mod.rs
│       │   │
│       │   ├── parser/           # 输入解析引擎
│       │   │   ├── mod.rs
│       │   │   ├── lexer.rs      # 词法分析
│       │   │   ├── triggers.rs   # 触发词处理
│       │   │   ├── calculator.rs # 计算器
│       │   │   ├── web_search.rs # Web 搜索解析
│       │   │   └── filters.rs    # 过滤器解析
│       │   │
│       │   ├── indexer/          # 文件索引引擎
│       │   │   ├── mod.rs
│       │   │   ├── scanner.rs    # 文件扫描器
│       │   │   ├── watcher.rs    # 文件监听器
│       │   │   ├── trie.rs       # 前缀树
│       │   │   ├── trigram.rs    # Trigram 索引
│       │   │   └── ranker.rs     # 排序算法
│       │   │
│       │   ├── clipboard/        # 剪贴板管理
│       │   │   ├── mod.rs
│       │   │   ├── monitor.rs    # 剪贴板监听
│       │   │   ├── storage.rs    # 历史存储
│       │   │   ├── window.rs     # 独立窗口管理
│       │   │   └── paste.rs      # 模拟粘贴
│       │   │
│       │   ├── screenshot/       # 截图引擎
│       │   │   ├── mod.rs
│       │   │   ├── capture.rs    # 屏幕捕获
│       │   │   ├── selector.rs   # 区域选择
│       │   │   ├── editor.rs     # 标注编辑
│       │   │   ├── pin.rs        # 贴图管理
│       │   │   └── picker.rs     # 取色器
│       │   │
│       │   ├── ai/               # AI 客户端
│       │   │   ├── mod.rs
│       │   │   ├── client.rs     # 通用客户端接口
│       │   │   ├── openai.rs     # OpenAI 实现
│       │   │   ├── anthropic.rs  # Anthropic 实现
│       │   │   ├── ollama.rs     # Ollama 实现
│       │   │   ├── streaming.rs  # 流式响应处理
│       │   │   └── conversation.rs # 对话管理
│       │   │
│       │   ├── workflow/         # 工作流引擎
│       │   │   ├── mod.rs
│       │   │   ├── engine.rs     # 执行引擎
│       │   │   ├── nodes/        # 节点实现
│       │   │   │   ├── mod.rs
│       │   │   │   ├── input.rs
│       │   │   │   ├── transform.rs
│       │   │   │   ├── action.rs
│       │   │   │   ├── logic.rs
│       │   │   │   ├── ai.rs
│       │   │   │   └── script.rs
│       │   │   ├── triggers.rs   # 触发器
│       │   │   └── context.rs    # 执行上下文
│       │   │
│       │   └── plugin/           # 插件系统
│       │       ├── mod.rs
│       │       ├── loader.rs     # 插件加载器
│       │       ├── registry.rs   # 插件注册表
│       │       ├── sandbox.rs    # 沙箱环境
│       │       ├── api.rs        # 插件 API
│       │       └── store.rs      # 插件市场
│       │
│       ├── platform/             # 平台特定实现
│       │   ├── mod.rs
│       │   ├── macos/
│       │   │   ├── mod.rs
│       │   │   ├── accessibility.rs  # 辅助功能权限
│       │   │   ├── screenshot.rs     # 截图实现
│       │   │   ├── clipboard.rs      # 剪贴板实现
│       │   │   ├── apps.rs           # 应用列表
│       │   │   └── window.rs         # 窗口管理
│       │   └── windows/
│       │       ├── mod.rs
│       │       ├── screenshot.rs
│       │       ├── clipboard.rs
│       │       ├── apps.rs
│       │       └── window.rs
│       │
│       ├── storage/              # 数据存储
│       │   ├── mod.rs
│       │   ├── database.rs       # SQLite 连接池
│       │   ├── keychain.rs       # 密钥存储
│       │   ├── cache.rs          # 缓存管理
│       │   └── migrations/       # 数据库迁移
│       │       ├── mod.rs
│       │       └── v1_initial.sql
│       │
│       └── utils/                # 工具模块
│           ├── mod.rs
│           ├── crypto.rs         # 加密工具
│           ├── image.rs          # 图像处理
│           ├── logger.rs         # 日志配置
│           └── template.rs       # 模板渲染
│
├── packages/                     # 共享包（可选）
│   └── shared-types/             # 共享类型定义
│
├── tests/                        # 集成测试
│   ├── e2e/                      # 端到端测试
│   └── fixtures/                 # 测试数据
│
├── scripts/                      # 构建脚本
│   ├── build.sh
│   └── release.sh
│
├── .github/                      # GitHub Actions
│   └── workflows/
│       ├── ci.yml
│       └── release.yml
│
├── package.json
├── pnpm-lock.yaml
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.js
├── postcss.config.js
└── README.md

```

---

## 二、系统架构

### 2.1 整体架构图

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                 OmniBox                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────── Windows ────────────────────────────────────┐ │
│  │                                                                         │ │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌──────────────┐   │ │
│  │  │    Main     │  │  Clipboard  │  │  Screenshot │  │   Settings   │   │ │
│  │  │   Window    │  │   Window    │  │   Windows   │  │    Window    │   │ │
│  │  │             │  │             │  │  (Overlay/  │  │              │   │ │
│  │  │  搜索主窗口  │  │ 剪贴板历史  │  │ Editor/Pin) │  │   配置中心   │   │ │
│  │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬───────┘   │ │
│  │         │                │                │                │           │ │
│  └─────────┼────────────────┼────────────────┼────────────────┼───────────┘ │
│            │                │                │                │             │
│  ┌─────────┴────────────────┴────────────────┴────────────────┴───────────┐ │
│  │                        Frontend (SolidJS + TypeScript)                  │ │
│  │                                                                         │ │
│  │  ┌─────────────────────────────────────────────────────────────────┐   │ │
│  │  │                         Solid Stores                             │   │ │
│  │  │   searchStore │ clipboardStore │ aiStore │ settingsStore │ ...  │   │ │
│  │  └────────────────────────────────┬────────────────────────────────┘   │ │
│  │                                   │                                     │ │
│  │  ┌────────────────────────────────┴────────────────────────────────┐   │ │
│  │  │                       Service Layer                              │   │ │
│  │  │            invoke() / listen() / emit() wrappers                 │   │ │
│  │  └────────────────────────────────┬────────────────────────────────┘   │ │
│  └───────────────────────────────────┼─────────────────────────────────────┘ │
│                                      │                                       │
│                              ════════╪════════  IPC (Tauri Commands/Events)  │
│                                      │                                       │
│  ┌───────────────────────────────────┼─────────────────────────────────────┐ │
│  │                        Backend (Rust + Tauri 2.0)                        │ │
│  │                                   │                                       │ │
│  │  ┌────────────────────────────────┴────────────────────────────────┐    │ │
│  │  │                      Command Router                              │    │ │
│  │  │                  #[tauri::command] handlers                      │    │ │
│  │  └───┬─────────┬─────────┬─────────┬─────────┬─────────┬───────────┘    │ │
│  │      │         │         │         │         │         │                 │ │
│  │  ┌───┴───┐ ┌───┴───┐ ┌───┴───┐ ┌───┴───┐ ┌───┴───┐ ┌───┴───┐           │ │
│  │  │Parser │ │Indexer│ │Clipbd │ │Screen │ │  AI   │ │Workflw│           │ │
│  │  │       │ │       │ │       │ │ shot  │ │Client │ │Engine │           │ │
│  │  └───┬───┘ └───┬───┘ └───┬───┘ └───┬───┘ └───┬───┘ └───┬───┘           │ │
│  │      │         │         │         │         │         │                 │ │
│  │  ┌───┴─────────┴─────────┴─────────┴─────────┴─────────┴───────────┐    │ │
│  │  │                        Core Services                             │    │ │
│  │  │  AppState │ ConfigManager │ Database │ CredentialStore │ Cache  │    │ │
│  │  └───┬─────────────────────────────────────────────────────────────┘    │ │
│  │      │                                                                   │ │
│  │  ┌───┴─────────────────────────────────────────────────────────────┐    │ │
│  │  │                     Platform Abstraction                         │    │ │
│  │  │              macOS (Cocoa/AppKit)  │  Windows (Win32/WinRT)      │    │ │
│  │  └─────────────────────────────────────────────────────────────────┘    │ │
│  └──────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
                     │                    │                    │
           ┌─────────┴────────┐ ┌─────────┴────────┐ ┌─────────┴────────┐
           │   File System    │ │   AI Providers   │ │   System APIs    │
           │  SQLite Database │ │  OpenAI/Claude/  │ │ Keychain/CredMgr │
           │   Config Files   │ │  Ollama/Custom   │ │  Notifications   │
           └──────────────────┘ └──────────────────┘ └──────────────────┘

```

### 2.2 多窗口架构

```
┌─────────────────────────────────────────────────────────────────┐
│                      Window Manager                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────┐   Window Label: "main"                     │
│  │   Main Window   │   路由: /                                  │
│  │                 │   特性: 无边框, 透明背景, 全局快捷键唤起     │
│  │   搜索 + 结果   │   尺寸: 680x500 (可配置)                    │
│  └─────────────────┘                                            │
│                                                                  │
│  ┌─────────────────┐   Window Label: "clipboard"                │
│  │Clipboard Window │   路由: /clipboard                         │
│  │                 │   特性: 无边框, 跟随光标, 全局快捷键唤起     │
│  │  剪贴板历史列表  │   尺寸: 400x500                            │
│  └─────────────────┘                                            │
│                                                                  │
│  ┌─────────────────┐   Window Label: "screenshot-overlay"       │
│  │Screenshot Overlay│  路由: /screenshot/overlay                 │
│  │                 │   特性: 全屏, 透明, 置顶, 临时窗口          │
│  │    区域选择     │   尺寸: 全屏                                │
│  └─────────────────┘                                            │
│                                                                  │
│  ┌─────────────────┐   Window Label: "screenshot-editor"        │
│  │Screenshot Editor│   路由: /screenshot/editor                  │
│  │                 │   特性: 标准窗口, 可缩放                    │
│  │    标注编辑器   │   尺寸: 动态 (根据截图尺寸)                 │
│  └─────────────────┘                                            │
│                                                                  │
│  ┌─────────────────┐   Window Label: "pin-{id}"                 │
│  │   Pin Window    │   路由: /screenshot/pin?id={id}            │
│  │                 │   特性: 无边框, 置顶, 可多个                │
│  │     贴图窗口    │   尺寸: 动态 (根据图片尺寸)                 │
│  └─────────────────┘                                            │
│                                                                  │
│  ┌─────────────────┐   Window Label: "ai-chat"                  │
│  │  AI Chat Window │   路由: /ai                                 │
│  │                 │   特性: 标准窗口, 可缩放                    │
│  │   AI 对话窗口   │   尺寸: 600x700                            │
│  └─────────────────┘                                            │
│                                                                  │
│  ┌─────────────────┐   Window Label: "settings"                 │
│  │ Settings Window │   路由: /settings                          │
│  │                 │   特性: 标准窗口, 单例                      │
│  │     配置中心    │   尺寸: 900x600                            │
│  └─────────────────┘                                            │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘

```

### 2.3 数据流架构

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                             用户输入处理流程                                  │
└─────────────────────────────────────────────────────────────────────────────┘

用户输入: "gg rust programming"
    │
    ▼
┌─────────────────┐
│   SearchInput   │  (前端组件)
│    Component    │
└────────┬────────┘
         │ onInput (debounced 150ms)
         ▼
┌─────────────────┐
│  searchStore    │  (Solid Store)
│   setQuery()    │
└────────┬────────┘
         │ createEffect
         ▼
┌─────────────────┐
│  searchService  │  (前端服务层)
│    .search()    │
└────────┬────────┘
         │ invoke("parse_and_search", { query })
         ▼
════════════════════════════════════════  IPC Boundary
         │
         ▼
┌─────────────────┐
│  search.rs      │  (Tauri Command)
│  parse_and_     │
│  search()       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  InputParser    │  (Core Module)
│   .parse()      │
└────────┬────────┘
         │
         ▼
┌─────────────────────────────────────────────────┐
│              ParseResult::WebSearch             │
│  {                                              │
│    engine: { id: "google", keyword: "gg", ... } │
│    query: "rust programming",                   │
│    url: "<https://google.com/search?q=>..."       │
│  }                                              │
└────────┬────────────────────────────────────────┘
         │
         ▼
┌─────────────────┐
│  SearchResult   │  构建搜索结果
│   Builder       │
└────────┬────────┘
         │
         ▼
════════════════════════════════════════  IPC Boundary
         │
         ▼
┌─────────────────┐
│  searchStore    │  (更新状态)
│  setResults()   │
└────────┬────────┘
         │ reactive update
         ▼
┌─────────────────┐
│   ResultList    │  (前端组件)
│    Component    │  显示: "Search Google: rust programming"
└─────────────────┘

```

---

## 三、核心模块设计

### 3.1 应用状态管理 (AppState)

```rust
// src-tauri/src/app/state.rs

use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::AppHandle;

use crate::app::config::ConfigManager;
use crate::core::parser::InputParser;
use crate::core::indexer::FileIndexer;
use crate::core::clipboard::ClipboardManager;
use crate::core::screenshot::ScreenshotManager;
use crate::core::ai::AIClient;
use crate::core::workflow::WorkflowEngine;
use crate::core::plugin::PluginManager;
use crate::storage::{Database, CredentialStore, Cache};

/// 全局应用状态
/// 通过 Tauri 的 manage() 注入，在 Command 中通过 State<AppState> 访问
pub struct AppState {
    /// 应用句柄
    pub app_handle: AppHandle,

    /// 配置管理器
    pub config: Arc<RwLock<ConfigManager>>,

    /// 输入解析器
    pub parser: Arc<InputParser>,

    /// 文件索引器
    pub indexer: Arc<FileIndexer>,

    /// 剪贴板管理器
    pub clipboard: Arc<ClipboardManager>,

    /// 截图管理器
    pub screenshot: Arc<ScreenshotManager>,

    /// AI 客户端
    pub ai: Arc<RwLock<AIClient>>,

    /// 工作流引擎
    pub workflow: Arc<WorkflowEngine>,

    /// 插件管理器
    pub plugins: Arc<PluginManager>,

    /// 数据库
    pub db: Arc<Database>,

    /// 凭证存储
    pub credentials: Arc<CredentialStore>,

    /// 缓存
    pub cache: Arc<Cache>,
}

impl AppState {
    /// 初始化应用状态
    pub async fn new(app_handle: AppHandle) -> Result<Self, AppError> {
        // 1. 加载配置
        let config = Arc::new(RwLock::new(ConfigManager::load()?));
        let config_read = config.read().await;

        // 2. 初始化存储
        let db = Arc::new(Database::new(&config_read.data_dir).await?);
        let credentials = Arc::new(CredentialStore::new()?);
        let cache = Arc::new(Cache::new(&config_read.data_dir)?);

        // 3. 初始化核心模块
        let parser = Arc::new(InputParser::new(
            config_read.web_search.clone(),
            config_read.triggers.clone(),
        ));

        let indexer = Arc::new(FileIndexer::new(
            db.clone(),
            cache.clone(),
            config_read.indexer.clone(),
        ).await?);

        let clipboard = Arc::new(ClipboardManager::new(
            app_handle.clone(),
            db.clone(),
            config_read.clipboard.clone(),
        )?);

        let screenshot = Arc::new(ScreenshotManager::new(
            app_handle.clone(),
            config_read.screenshot.clone(),
        )?);

        let ai = Arc::new(RwLock::new(AIClient::new(
            credentials.clone(),
            config_read.ai.clone(),
        )?));

        let workflow = Arc::new(WorkflowEngine::new(
            db.clone(),
            config_read.workflow.clone(),
        ).await?);

        let plugins = Arc::new(PluginManager::new(
            config_read.data_dir.join("plugins"),
        ).await?);

        drop(config_read);

        Ok(Self {
            app_handle,
            config,
            parser,
            indexer,
            clipboard,
            screenshot,
            ai,
            workflow,
            plugins,
            db,
            credentials,
            cache,
        })
    }

    /// 重新加载配置
    pub async fn reload_config(&self) -> Result<(), AppError> {
        let new_config = ConfigManager::load()?;
        let mut config = self.config.write().await;
        *config = new_config;

        // 通知各模块配置已更新
        self.parser.update_config(config.web_search.clone(), config.triggers.clone());
        self.indexer.update_config(config.indexer.clone()).await?;
        self.clipboard.update_config(config.clipboard.clone())?;
        self.screenshot.update_config(config.screenshot.clone())?;
        self.ai.write().await.update_config(config.ai.clone())?;

        Ok(())
    }
}

```

### 3.2 配置管理 (ConfigManager)

```rust
// src-tauri/src/app/config.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

/// 应用完整配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    /// 通用设置
    pub general: GeneralConfig,

    /// 功能开关
    pub features: FeaturesConfig,

    /// 外观设置
    pub appearance: AppearanceConfig,

    /// 剪贴板设置
    pub clipboard: ClipboardConfig,

    /// 截图设置
    pub screenshot: ScreenshotConfig,

    /// AI 设置
    pub ai: AIConfig,

    /// 工作流设置
    pub workflow: WorkflowConfig,

    /// Web 搜索设置
    pub web_search: WebSearchConfig,

    /// 高级设置
    pub advanced: AdvancedConfig,

    /// 快捷键设置
    pub shortcuts: ShortcutsConfig,

    /// 触发词设置
    pub triggers: TriggersConfig,

    /// 文件索引设置
    pub indexer: IndexerConfig,

    // 运行时字段（不序列化）
    #[serde(skip)]
    pub data_dir: PathBuf,
    #[serde(skip)]
    pub config_path: PathBuf,
}

/// 通用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub launch_at_startup: bool,
    pub language: String,
    pub check_updates: bool,
    pub show_tray_icon: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            launch_at_startup: true,
            language: "system".to_string(),
            check_updates: true,
            show_tray_icon: true,
        }
    }
}

/// 功能开关
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FeaturesConfig {
    pub file_search: bool,
    pub app_search: bool,
    pub calculator: bool,
    pub web_search: bool,
    pub clipboard_manager: bool,
    pub screenshot: bool,
    pub ai_assistant: bool,
}

impl Default for FeaturesConfig {
    fn default() -> Self {
        Self {
            file_search: true,
            app_search: true,
            calculator: true,
            web_search: true,
            clipboard_manager: true,
            screenshot: true,
            ai_assistant: true,
        }
    }
}

/// 外观设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppearanceConfig {
    pub theme: Theme,
    pub accent_color: String,
    pub window_width: u32,
    pub result_count: u32,
    pub show_icons: bool,
    pub blur_background: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Light,
    Dark,
    #[default]
    System,
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            accent_color: "#007AFF".to_string(),
            window_width: 680,
            result_count: 8,
            show_icons: true,
            blur_background: true,
        }
    }
}

/// 剪贴板设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ClipboardConfig {
    pub enabled: bool,
    pub max_history: usize,
    pub max_item_size_mb: usize,
    pub retention_days: Option<u32>,
    pub store_images: bool,
    pub ignore_apps: Vec<String>,
    pub sensitive_patterns: Vec<String>,
}

impl Default for ClipboardConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_history: 1000,
            max_item_size_mb: 10,
            retention_days: Some(30),
            store_images: true,
            ignore_apps: vec![],
            sensitive_patterns: vec![
                r"(?i)password".to_string(),
                r"(?i)secret".to_string(),
                r"(?i)api[_-]?key".to_string(),
            ],
        }
    }
}

/// 截图设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ScreenshotConfig {
    pub default_save_path: PathBuf,
    pub default_format: ImageFormat,
    pub jpeg_quality: u8,
    pub play_sound: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    #[default]
    Png,
    Jpeg,
    WebP,
}

impl Default for ScreenshotConfig {
    fn default() -> Self {
        Self {
            default_save_path: dirs::picture_dir()
                .unwrap_or_default()
                .join("Screenshots"),
            default_format: ImageFormat::Png,
            jpeg_quality: 90,
            play_sound: true,
        }
    }
}

/// AI 设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AIConfig {
    pub enabled: bool,
    pub default_provider: Option<String>,
    pub providers: Vec<AIProviderConfig>,
    pub save_conversations: bool,
    pub stream_response: bool,
    pub custom_prompts: Vec<CustomPrompt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderConfig {
    pub id: String,
    pub name: String,
    pub provider_type: AIProviderType,
    pub model: String,
    pub endpoint: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AIProviderType {
    OpenAI,
    Anthropic,
    Google,
    Ollama,
    AzureOpenAI,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPrompt {
    pub id: String,
    pub name: String,
    pub prompt: String,
    pub shortcut: Option<String>,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_provider: None,
            providers: vec![
                AIProviderConfig {
                    id: "openai".to_string(),
                    name: "OpenAI".to_string(),
                    provider_type: AIProviderType::OpenAI,
                    model: "gpt-4o".to_string(),
                    endpoint: None,
                    enabled: false,
                },
                AIProviderConfig {
                    id: "anthropic".to_string(),
                    name: "Anthropic".to_string(),
                    provider_type: AIProviderType::Anthropic,
                    model: "claude-3-5-sonnet-20241022".to_string(),
                    endpoint: None,
                    enabled: false,
                },
                AIProviderConfig {
                    id: "ollama".to_string(),
                    name: "Ollama (Local)".to_string(),
                    provider_type: AIProviderType::Ollama,
                    model: "llama3.2".to_string(),
                    endpoint: Some("<http://localhost:11434>".to_string()),
                    enabled: false,
                },
            ],
            save_conversations: true,
            stream_response: true,
            custom_prompts: vec![
                CustomPrompt {
                    id: "translate".to_string(),
                    name: "Translate".to_string(),
                    prompt: "Translate the following text to {{target_language}}:\\n\\n{{input}}".to_string(),
                    shortcut: None,
                },
                CustomPrompt {
                    id: "explain_code".to_string(),
                    name: "Explain Code".to_string(),
                    prompt: "Explain the following code in detail:\\n\\n```\\n{{input}}\\n```".to_string(),
                    shortcut: None,
                },
            ],
        }
    }
}

/// 快捷键设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShortcutsConfig {
    pub main_window: String,
    pub clipboard_history: String,
    pub screenshot_region: String,
    pub screenshot_window: String,
    pub screenshot_pin: String,
    pub color_picker: String,
    pub ai_chat: String,
}

impl Default for ShortcutsConfig {
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        {
            Self {
                main_window: "Cmd+Space".to_string(),
                clipboard_history: "Cmd+Shift+V".to_string(),
                screenshot_region: "Cmd+Shift+4".to_string(),
                screenshot_window: "Cmd+Shift+5".to_string(),
                screenshot_pin: "Cmd+Shift+P".to_string(),
                color_picker: "Cmd+Shift+C".to_string(),
                ai_chat: "Cmd+Shift+A".to_string(),
            }
        }
        #[cfg(target_os = "windows")]
        {
            Self {
                main_window: "Alt+Space".to_string(),
                clipboard_history: "Ctrl+Shift+V".to_string(),
                screenshot_region: "Ctrl+Shift+S".to_string(),
                screenshot_window: "Ctrl+Shift+W".to_string(),
                screenshot_pin: "Ctrl+Shift+P".to_string(),
                color_picker: "Ctrl+Shift+C".to_string(),
                ai_chat: "Ctrl+Shift+A".to_string(),
            }
        }
    }
}

/// Web 搜索设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WebSearchConfig {
    pub default_engine: String,
    pub engines: Vec<SearchEngine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEngine {
    pub id: String,
    pub name: String,
    pub keyword: String,
    pub url_template: String,
    pub icon: Option<String>,
    pub is_builtin: bool,
}

impl Default for WebSearchConfig {
    fn default() -> Self {
        Self {
            default_engine: "google".to_string(),
            engines: vec![
                SearchEngine {
                    id: "google".to_string(),
                    name: "Google".to_string(),
                    keyword: "gg".to_string(),
                    url_template: "<https://www.google.com/search?q={query}>".to_string(),
                    icon: None,
                    is_builtin: true,
                },
                SearchEngine {
                    id: "baidu".to_string(),
                    name: "Baidu".to_string(),
                    keyword: "bd".to_string(),
                    url_template: "<https://www.baidu.com/s?wd={query}>".to_string(),
                    icon: None,
                    is_builtin: true,
                },
                SearchEngine {
                    id: "bing".to_string(),
                    name: "Bing".to_string(),
                    keyword: "bi".to_string(),
                    url_template: "<https://www.bing.com/search?q={query}>".to_string(),
                    icon: None,
                    is_builtin: true,
                },
                SearchEngine {
                    id: "duckduckgo".to_string(),
                    name: "DuckDuckGo".to_string(),
                    keyword: "ddg".to_string(),
                    url_template: "<https://duckduckgo.com/?q={query}>".to_string(),
                    icon: None,
                    is_builtin: true,
                },
                SearchEngine {
                    id: "github".to_string(),
                    name: "GitHub".to_string(),
                    keyword: "gh".to_string(),
                    url_template: "<https://github.com/search?q={query}>".to_string(),
                    icon: None,
                    is_builtin: true,
                },
                SearchEngine {
                    id: "stackoverflow".to_string(),
                    name: "Stack Overflow".to_string(),
                    keyword: "so".to_string(),
                    url_template: "<https://stackoverflow.com/search?q={query}>".to_string(),
                    icon: None,
                    is_builtin: true,
                },
                SearchEngine {
                    id: "youtube".to_string(),
                    name: "YouTube".to_string(),
                    keyword: "yt".to_string(),
                    url_template: "<https://www.youtube.com/results?search_query={query}>".to_string(),
                    icon: None,
                    is_builtin: true,
                },
                SearchEngine {
                    id: "npm".to_string(),
                    name: "NPM".to_string(),
                    keyword: "npm".to_string(),
                    url_template: "<https://www.npmjs.com/search?q={query}>".to_string(),
                    icon: None,
                    is_builtin: true,
                },
                SearchEngine {
                    id: "crates".to_string(),
                    name: "Crates.io".to_string(),
                    keyword: "crate".to_string(),
                    url_template: "<https://crates.io/search?q={query}>".to_string(),
                    icon: None,
                    is_builtin: true,
                },
            ],
        }
    }
}

/// 文件索引设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IndexerConfig {
    pub enabled: bool,
    pub include_paths: Vec<PathBuf>,
    pub exclude_paths: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
    pub max_depth: Option<u32>,
    pub index_hidden: bool,
    pub watch_changes: bool,
}

impl Default for IndexerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            include_paths: vec![dirs::home_dir().unwrap_or_default()],
            exclude_paths: vec![],
            exclude_patterns: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "target".to_string(),
                "*.log".to_string(),
                "*.tmp".to_string(),
            ],
            max_depth: None,
            index_hidden: false,
            watch_changes: true,
        }
    }
}

/// 配置管理器
pub struct ConfigManager {
    config: AppConfig,
}

impl ConfigManager {
    /// 加载配置
    pub fn load() -> Result<Self, ConfigError> {
        let data_dir = Self::get_data_dir()?;
        let config_path = data_dir.join("config.yaml");

        let mut config = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            serde_yaml::from_str(&content)?
        } else {
            AppConfig::default()
        };

        config.data_dir = data_dir;
        config.config_path = config_path;

        Ok(Self { config })
    }

    /// 保存配置
    pub fn save(&self) -> Result<(), ConfigError> {
        let content = serde_yaml::to_string(&self.config)?;
        std::fs::write(&self.config.config_path, content)?;
        Ok(())
    }

    /// 获取配置
    pub fn get(&self) -> &AppConfig {
        &self.config
    }

    /// 更新配置
    pub fn update<F>(&mut self, f: F) -> Result<(), ConfigError>
    where
        F: FnOnce(&mut AppConfig),
    {
        f(&mut self.config);
        self.save()
    }

    /// 获取数据目录
    fn get_data_dir() -> Result<PathBuf, ConfigError> {
        #[cfg(target_os = "macos")]
        let base = dirs::home_dir()
            .ok_or(ConfigError::HomeDirNotFound)?
            .join("Library/Application Support/OmniBox");

        #[cfg(target_os = "windows")]
        let base = dirs::config_dir()
            .ok_or(ConfigError::HomeDirNotFound)?
            .join("OmniBox");

        std::fs::create_dir_all(&base)?;
        Ok(base)
    }
}

impl std::ops::Deref for ConfigManager {
    type Target = AppConfig;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl std::ops::DerefMut for ConfigManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}

```

### 3.3 输入解析引擎 (InputParser)

```rust
// src-tauri/src/core/parser/mod.rs

pub mod lexer;
pub mod triggers;
pub mod calculator;
pub mod web_search;
pub mod filters;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::app::config::{WebSearchConfig, SearchEngine, TriggersConfig};

/// 解析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ParseResult {
    /// 空输入
    Empty,

    /// 通用搜索
    Search {
        query: String,
        filters: SearchFilters,
    },

    /// Web 搜索
    WebSearch {
        engine: SearchEngine,
        query: String,
        url: String,
    },

    /// 直接打开 URL
    OpenUrl {
        url: String,
    },

    /// 路由到特定模块
    Route {
        module: ModuleType,
        query: String,
    },

    /// 计算结果
    Calculate {
        expression: String,
        result: CalculateResult,
    },

    /// 执行系统命令
    Command {
        command: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleType {
    Ai,
    Clipboard,
    Bookmark,
    File,
    App,
    Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculateResult {
    pub value: f64,
    pub formatted: String,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchFilters {
    pub extensions: Option<Vec<String>>,
    pub path_contains: Option<String>,
    pub size_min: Option<u64>,
    pub size_max: Option<u64>,
    pub modified_after: Option<i64>,
}

/// 输入解析器
pub struct InputParser {
    /// Web 搜索配置
    web_search: WebSearchConfig,
    /// 触发词配置
    triggers: TriggersConfig,
    /// 计算器
    calculator: Calculator,
    /// 搜索引擎关键词映射
    engine_map: HashMap<String, SearchEngine>,
    /// 模块触发词映射
    module_map: HashMap<String, ModuleType>,
}

impl InputParser {
    pub fn new(web_search: WebSearchConfig, triggers: TriggersConfig) -> Self {
        let mut engine_map = HashMap::new();
        for engine in &web_search.engines {
            engine_map.insert(engine.keyword.to_lowercase(), engine.clone());
        }

        let module_map = Self::build_module_map(&triggers);

        Self {
            web_search,
            triggers,
            calculator: Calculator::new(),
            engine_map,
            module_map,
        }
    }

    /// 更新配置
    pub fn update_config(&mut self, web_search: WebSearchConfig, triggers: TriggersConfig) {
        self.engine_map.clear();
        for engine in &web_search.engines {
            self.engine_map.insert(engine.keyword.to_lowercase(), engine.clone());
        }

        self.module_map = Self::build_module_map(&triggers);
        self.web_search = web_search;
        self.triggers = triggers;
    }

    /// 解析用户输入
    pub fn parse(&self, input: &str) -> ParseResult {
        let trimmed = input.trim();

        // 1. 空输入
        if trimmed.is_empty() {
            return ParseResult::Empty;
        }

        // 2. 检测系统命令 (> prefix)
        if let Some(cmd) = trimmed.strip_prefix('>') {
            return ParseResult::Command {
                command: cmd.trim().to_string(),
            };
        }

        // 3. 检测模块触发词
        if let Some(result) = self.check_module_trigger(trimmed) {
            return result;
        }

        // 4. 检测 Web 搜索关键词
        if let Some(result) = self.check_web_search(trimmed) {
            return result;
        }

        // 5. 检测是否为 URL
        if let Some(result) = self.check_url(trimmed) {
            return result;
        }

        // 6. 检测数学表达式
        if let Some(result) = self.check_calculation(trimmed) {
            return result;
        }

        // 7. 默认：通用搜索
        let filters = self.extract_filters(trimmed);
        let query = self.extract_query(trimmed, &filters);

        ParseResult::Search { query, filters }
    }

    /// 检测模块触发词
    fn check_module_trigger(&self, input: &str) -> Option<ParseResult> {
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let keyword = parts[0].to_lowercase();

        if let Some(module) = self.module_map.get(&keyword) {
            let query = parts.get(1).unwrap_or(&"").trim().to_string();
            return Some(ParseResult::Route {
                module: module.clone(),
                query,
            });
        }

        None
    }

    /// 检测 Web 搜索关键词
    fn check_web_search(&self, input: &str) -> Option<ParseResult> {
        let parts: Vec<&str> = input.splitn(2, ' ').collect();

        if parts.len() < 2 {
            return None;
        }

        let keyword = parts[0].to_lowercase();
        let query = parts[1].trim();

        if let Some(engine) = self.engine_map.get(&keyword) {
            let url = engine.url_template.replace("{query}", &urlencoding::encode(query));
            return Some(ParseResult::WebSearch {
                engine: engine.clone(),
                query: query.to_string(),
                url,
            });
        }

        None
    }

    /// 检测是否为 URL
    fn check_url(&self, input: &str) -> Option<ParseResult> {
        // 以协议开头
        if input.starts_with("http://") || input.starts_with("https://") {
            return Some(ParseResult::OpenUrl {
                url: input.to_string(),
            });
        }

        // 类似 URL 的模式: xxx.xxx 或 xxx.xxx/path
        let url_pattern = regex::Regex::new(
            r"^[a-zA-Z0-9][-a-zA-Z0-9]*(\\.[a-zA-Z0-9][-a-zA-Z0-9]*)+(/.*)?$"
        ).unwrap();

        if url_pattern.is_match(input) {
            return Some(ParseResult::OpenUrl {
                url: format!("https://{}", input),
            });
        }

        None
    }

    /// 检测数学表达式
    fn check_calculation(&self, input: &str) -> Option<ParseResult> {
        // 以 = 开头强制计算
        let expr = if let Some(stripped) = input.strip_prefix('=') {
            stripped.trim()
        } else if self.calculator.is_expression(input) {
            input
        } else {
            return None;
        };

        match self.calculator.evaluate(expr) {
            Ok(result) => Some(ParseResult::Calculate {
                expression: expr.to_string(),
                result,
            }),
            Err(_) => None,
        }
    }

    /// 提取搜索过滤器
    fn extract_filters(&self, input: &str) -> SearchFilters {
        let mut filters = SearchFilters::default();

        // 扩展名过滤: .pdf, .md
        let ext_regex = regex::Regex::new(r"\\.(\\w+)(?:\\s|$)").unwrap();
        let extensions: Vec<String> = ext_regex
            .captures_iter(input)
            .map(|c| c[1].to_string())
            .collect();
        if !extensions.is_empty() {
            filters.extensions = Some(extensions);
        }

        // 路径过滤: path:Documents
        let path_regex = regex::Regex::new(r"path:(\\S+)").unwrap();
        if let Some(caps) = path_regex.captures(input) {
            filters.path_contains = Some(caps[1].to_string());
        }

        // 大小过滤: size:>10mb
        let size_regex = regex::Regex::new(r"size:([<>])(\\d+)(kb|mb|gb)?").unwrap();
        if let Some(caps) = size_regex.captures(input) {
            let multiplier: u64 = match caps.get(3).map(|m| m.as_str()) {
                Some("kb") => 1024,
                Some("mb") => 1024 * 1024,
                Some("gb") => 1024 * 1024 * 1024,
                _ => 1,
            };
            let size: u64 = caps[2].parse().unwrap_or(0) * multiplier;
            match &caps[1] {
                ">" => filters.size_min = Some(size),
                "<" => filters.size_max = Some(size),
                _ => {}
            }
        }

        filters
    }

    /// 提取核心查询词（移除过滤器语法）
    fn extract_query(&self, input: &str, _filters: &SearchFilters) -> String {
        let patterns = [
            r"\\.\\w+(?:\\s|$)",
            r"path:\\S+",
            r"size:[<>]\\d+(?:kb|mb|gb)?",
            r"modified:\\S+",
        ];

        let mut query = input.to_string();
        for pattern in patterns {
            let re = regex::Regex::new(pattern).unwrap();
            query = re.replace_all(&query, " ").to_string();
        }

        query.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn build_module_map(triggers: &TriggersConfig) -> HashMap<String, ModuleType> {
        let mut map = HashMap::new();

        // 默认触发词
        map.insert("ai".to_string(), ModuleType::Ai);
        map.insert("cb".to_string(), ModuleType::Clipboard);
        map.insert("clip".to_string(), ModuleType::Clipboard);
        map.insert("bm".to_string(), ModuleType::Bookmark);
        map.insert("file".to_string(), ModuleType::File);
        map.insert("app".to_string(), ModuleType::App);
        map.insert("settings".to_string(), ModuleType::Settings);
        map.insert("prefs".to_string(), ModuleType::Settings);

        // 用户自定义触发词会覆盖默认
        for (keyword, module) in &triggers.custom {
            map.insert(keyword.to_lowercase(), module.clone());
        }

        map
    }
}

/// 计算器
pub struct Calculator {
    // 可以使用 meval 或自实现
}

impl Calculator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_expression(&self, input: &str) -> bool {
        // 简单判断：包含运算符且以数字或括号开头
        let has_operators = input.contains('+')
            || input.contains('-')
            || input.contains('*')
            || input.contains('/');

        let starts_valid = input.trim().chars().next()
            .map(|c| c.is_ascii_digit() || c == '(' || c == '-')
            .unwrap_or(false);

        has_operators && starts_valid
    }

    pub fn evaluate(&self, expr: &str) -> Result<CalculateResult, CalculatorError> {
        // 使用 meval crate 进行计算
        let value = meval::eval_str(expr)
            .map_err(|e| CalculatorError::EvalError(e.to_string()))?;

        Ok(CalculateResult {
            value,
            formatted: self.format_number(value),
            unit: None,
        })
    }

    fn format_number(&self, value: f64) -> String {
        if value.fract() == 0.0 && value.abs() < 1e15 {
            format!("{}", value as i64)
        } else {
            format!("{:.6}", value).trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CalculatorError {
    #[error("Evaluation error: {0}")]
    EvalError(String),
}

```

### 3.4 文件索引引擎 (FileIndexer)

```rust
// src-tauri/src/core/indexer/mod.rs

pub mod scanner;
pub mod watcher;
pub mod trie;
pub mod trigram;
pub mod ranker;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::app::config::IndexerConfig;
use crate::storage::{Database, Cache};

/// 文件条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub id: i64,
    pub name: String,
    pub path: PathBuf,
    pub extension: Option<String>,
    pub size: u64,
    pub modified_at: i64,
    pub created_at: i64,
    pub is_dir: bool,
    pub is_app: bool,

    // 排序权重因素
    pub use_count: u32,
    pub last_accessed: Option<i64>,
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSearchResult {
    pub entry: FileEntry,
    pub score: f64,
    pub match_positions: Vec<(usize, usize)>,
}

/// 索引统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_files: usize,
    pub total_dirs: usize,
    pub total_apps: usize,
    pub index_size_bytes: u64,
    pub last_full_scan: Option<i64>,
    pub last_update: i64,
}

/// 文件索引引擎
pub struct FileIndexer {
    /// 内存索引
    index: Arc<RwLock<MemoryIndex>>,
    /// 文件监听器
    watcher: Arc<RwLock<Option<FileWatcher>>>,
    /// 数据库
    db: Arc<Database>,
    /// 缓存
    cache: Arc<Cache>,
    /// 配置
    config: Arc<RwLock<IndexerConfig>>,
}

/// 内存索引结构
struct MemoryIndex {
    /// 前缀树索引
    trie: trie::RadixTrie,
    /// Trigram 索引（用于模糊搜索）
    trigram: trigram::TrigramIndex,
    /// ID -> 条目映射
    entries: HashMap<i64, FileEntry>,
    /// 路径 -> ID 映射
    path_to_id: HashMap<PathBuf, i64>,
    /// 下一个 ID
    next_id: i64,
    /// 统计信息
    stats: IndexStats,
}

impl FileIndexer {
    /// 创建索引器
    pub async fn new(
        db: Arc<Database>,
        cache: Arc<Cache>,
        config: IndexerConfig,
    ) -> Result<Self, IndexerError> {
        let index = Arc::new(RwLock::new(MemoryIndex::new()));

        let indexer = Self {
            index,
            watcher: Arc::new(RwLock::new(None)),
            db,
            cache,
            config: Arc::new(RwLock::new(config)),
        };

        // 从数据库加载索引
        indexer.load_from_db().await?;

        // 启动文件监听
        let config = indexer.config.read().await;
        if config.watch_changes {
            drop(config);
            indexer.start_watcher().await?;
        }

        Ok(indexer)
    }

    /// 搜索文件
    pub async fn search(
        &self,
        query: &str,
        filters: &super::parser::SearchFilters,
        limit: usize,
    ) -> Vec<FileSearchResult> {
        let index = self.index.read().await;
        let query_lower = query.to_lowercase();

        // 1. 前缀匹配
        let prefix_ids = index.trie.search_prefix(&query_lower);

        // 2. Trigram 模糊匹配
        let fuzzy_ids = index.trigram.search(&query_lower, 0.3);

        // 3. 合并去重
        let mut seen = std::collections::HashSet::new();
        let mut results: Vec<FileSearchResult> = Vec::new();

        for id in prefix_ids.into_iter().chain(fuzzy_ids.into_iter()) {
            if seen.contains(&id) {
                continue;
            }
            seen.insert(id);

            if let Some(entry) = index.entries.get(&id) {
                // 应用过滤器
                if !self.matches_filters(entry, filters) {
                    continue;
                }

                let score = ranker::calculate_score(entry, &query_lower);
                let match_positions = ranker::find_match_positions(&entry.name, &query_lower);

                results.push(FileSearchResult {
                    entry: entry.clone(),
                    score,
                    match_positions,
                });
            }
        }

        // 4. 排序并截取
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.into_iter().take(limit).collect()
    }

    /// 搜索应用
    pub async fn search_apps(&self, query: &str, limit: usize) -> Vec<FileSearchResult> {
        let index = self.index.read().await;
        let query_lower = query.to_lowercase();

        let mut results: Vec<FileSearchResult> = index.entries
            .values()
            .filter(|e| e.is_app && e.name.to_lowercase().contains(&query_lower))
            .map(|entry| {
                let score = ranker::calculate_score(entry, &query_lower);
                let match_positions = ranker::find_match_positions(&entry.name, &query_lower);
                FileSearchResult {
                    entry: entry.clone(),
                    score,
                    match_positions,
                }
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.into_iter().take(limit).collect()
    }

    /// 重建索引
    pub async fn rebuild(&self) -> Result<IndexStats, IndexerError> {
        let start = std::time::Instant::now();
        let config = self.config.read().await.clone();

        // 扫描文件系统
        let scanner = scanner::FileScanner::new(&config);
        let entries = scanner.scan_all().await?;

        let mut index = self.index.write().await;
        index.clear();

        let mut stats = IndexStats {
            total_files: 0,
            total_dirs: 0,
            total_apps: 0,
            index_size_bytes: 0,
            last_full_scan: Some(chrono::Utc::now().timestamp()),
            last_update: chrono::Utc::now().timestamp(),
        };

        for entry in entries {
            if entry.is_app {
                stats.total_apps += 1;
            } else if entry.is_dir {
                stats.total_dirs += 1;
            } else {
                stats.total_files += 1;
            }
            index.add_entry(entry);
        }

        stats.index_size_bytes = index.estimate_size();
        index.stats = stats.clone();

        drop(index);

        // 持久化到数据库
        self.save_to_db().await?;

        tracing::info!(
            "Index rebuilt in {:?}: {} files, {} dirs, {} apps",
            start.elapsed(),
            stats.total_files,
            stats.total_dirs,
            stats.total_apps
        );

        Ok(stats)
    }

    /// 获取索引统计
    pub async fn get_stats(&self) -> IndexStats {
        self.index.read().await.stats.clone()
    }

    /// 记录文件访问（更新权重）
    pub async fn record_access(&self, path: &PathBuf) -> Result<(), IndexerError> {
        let mut index = self.index.write().await;

        if let Some(id) = index.path_to_id.get(path).copied() {
            if let Some(entry) = index.entries.get_mut(&id) {
                entry.use_count += 1;
                entry.last_accessed = Some(chrono::Utc::now().timestamp());
            }
        }

        Ok(())
    }

    /// 更新配置
    pub async fn update_config(&self, config: IndexerConfig) -> Result<(), IndexerError> {
        let mut current_config = self.config.write().await;

        let paths_changed = current_config.include_paths != config.include_paths
            || current_config.exclude_paths != config.exclude_paths;

        *current_config = config;
        drop(current_config);

        // 如果索引路径变化，重建索引
        if paths_changed {
            self.rebuild().await?;
        }

        Ok(())
    }

    /// 处理文件系统事件
    pub async fn handle_fs_event(&self, event: watcher::FsEvent) -> Result<(), IndexerError> {
        let mut index = self.index.write().await;

        match event {
            watcher::FsEvent::Create(path) => {
                if let Ok(entry) = self.path_to_entry(&path).await {
                    index.add_entry(entry);
                }
            }
            watcher::FsEvent::Delete(path) => {
                index.remove_by_path(&path);
            }
            watcher::FsEvent::Modify(path) => {
                if let Ok(entry) = self.path_to_entry(&path).await {
                    index.update_entry(entry);
                }
            }
            watcher::FsEvent::Rename { from, to } => {
                index.remove_by_path(&from);
                if let Ok(entry) = self.path_to_entry(&to).await {
                    index.add_entry(entry);
                }
            }
        }

        index.stats.last_update = chrono::Utc::now().timestamp();

        Ok(())
    }

    fn matches_filters(&self, entry: &FileEntry, filters: &super::parser::SearchFilters) -> bool {
        // 扩展名过滤
        if let Some(ref exts) = filters.extensions {
            match &entry.extension {
                Some(ext) if exts.iter().any(|e| e.eq_ignore_ascii_case(ext)) => {}
                _ => return false,
            }
        }

        // 路径过滤
        if let Some(ref path_contains) = filters.path_contains {
            if !entry.path.to_string_lossy().to_lowercase().contains(&path_contains.to_lowercase()) {
                return false;
            }
        }

        // 大小过滤
        if let Some(min) = filters.size_min {
            if entry.size < min {
                return false;
            }
        }
        if let Some(max) = filters.size_max {
            if entry.size > max {
                return false;
            }
        }

        true
    }

    async fn load_from_db(&self) -> Result<(), IndexerError> {
        let entries = self.db.get_all_file_entries().await?;
        let mut index = self.index.write().await;

        for entry in entries {
            index.add_entry(entry);
        }

        Ok(())
    }

    async fn save_to_db(&self) -> Result<(), IndexerError> {
        let index = self.index.read().await;
        let entries: Vec<_> = index.entries.values().cloned().collect();
        drop(index);

        self.db.save_file_entries(&entries).await?;

        Ok(())
    }

    async fn start_watcher(&self) -> Result<(), IndexerError> {
        let config = self.config.read().await;
        let watcher = watcher::FileWatcher::new(
            config.include_paths.clone(),
            config.exclude_paths.clone(),
            config.exclude_patterns.clone(),
        )?;

        let index_ref = self.index.clone();
        let db_ref = self.db.clone();

        watcher.start(move |event| {
            let index = index_ref.clone();
            let db = db_ref.clone();
            tokio::spawn(async move {
                // 处理文件系统事件...
            });
        })?;

        *self.watcher.write().await = Some(watcher);

        Ok(())
    }

    async fn path_to_entry(&self, path: &PathBuf) -> Result<FileEntry, IndexerError> {
        let metadata = tokio::fs::metadata(path).await?;
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let is_app = self.is_application(path);

        Ok(FileEntry {
            id: 0, // 由 MemoryIndex 分配
            name,
            path: path.clone(),
            extension: path.extension().map(|e| e.to_string_lossy().to_string()),
            size: metadata.len(),
            modified_at: metadata.modified()
                .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64)
                .unwrap_or(0),
            created_at: metadata.created()
                .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64)
                .unwrap_or(0),
            is_dir: metadata.is_dir(),
            is_app,
            use_count: 0,
            last_accessed: None,
        })
    }

    #[cfg(target_os = "macos")]
    fn is_application(&self, path: &PathBuf) -> bool {
        path.extension().map(|e| e == "app").unwrap_or(false)
    }

    #[cfg(target_os = "windows")]
    fn is_application(&self, path: &PathBuf) -> bool {
        path.extension().map(|e| e == "exe" || e == "lnk").unwrap_or(false)
    }
}

impl MemoryIndex {
    fn new() -> Self {
        Self {
            trie: trie::RadixTrie::new(),
            trigram: trigram::TrigramIndex::new(),
            entries: HashMap::new(),
            path_to_id: HashMap::new(),
            next_id: 1,
            stats: IndexStats {
                total_files: 0,
                total_dirs: 0,
                total_apps: 0,
                index_size_bytes: 0,
                last_full_scan: None,
                last_update: 0,
            },
        }
    }

    fn add_entry(&mut self, mut entry: FileEntry) {
        let id = self.next_id;
        self.next_id += 1;
        entry.id = id;

        let name_lower = entry.name.to_lowercase();

        self.trie.insert(&name_lower, id);
        self.trigram.insert(&name_lower, id);
        self.path_to_id.insert(entry.path.clone(), id);
        self.entries.insert(id, entry);
    }

    fn remove_by_path(&mut self, path: &PathBuf) {
        if let Some(id) = self.path_to_id.remove(path) {
            if let Some(entry) = self.entries.remove(&id) {
                let name_lower = entry.name.to_lowercase();
                self.trie.remove(&name_lower, id);
                self.trigram.remove(&name_lower, id);
            }
        }
    }

    fn update_entry(&mut self, entry: FileEntry) {
        self.remove_by_path(&entry.path);
        self.add_entry(entry);
    }

    fn clear(&mut self) {
        self.trie = trie::RadixTrie::new();
        self.trigram = trigram::TrigramIndex::new();
        self.entries.clear();
        self.path_to_id.clear();
        self.next_id = 1;
    }

    fn estimate_size(&self) -> u64 {
        // 粗略估算内存使用
        (self.entries.len() * std::mem::size_of::<FileEntry>()
            + self.trie.estimate_size()
            + self.trigram.estimate_size()) as u64
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IndexerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Database error: {0}")]
    Database(#[from] crate::storage::DatabaseError),
    #[error("Watcher error: {0}")]
    Watcher(String),
}

```

### 3.5 剪贴板管理器 (ClipboardManager)

```rust
// src-tauri/src/core/clipboard/mod.rs

pub mod monitor;
pub mod storage;
pub mod window;
pub mod paste;

use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::app::config::ClipboardConfig;
use crate::storage::Database;

/// 剪贴板内容类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClipboardContent {
    Text {
        text: String,
        html: Option<String>,
    },
    Image {
        /// Base64 编码的图片数据
        data: String,
        width: u32,
        height: u32,
        format: String,
    },
    Files {
        paths: Vec<String>,
    },
    Color {
        hex: String,
        rgb: (u8, u8, u8),
    },
}

/// 剪贴板历史条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    pub id: i64,
    pub content: ClipboardContent,
    pub preview: String,
    pub app_name: Option<String>,
    pub app_icon: Option<String>,
    pub created_at: i64,
    pub is_favorite: bool,
    pub is_sensitive: bool,
}

/// 剪贴板管理器
pub struct ClipboardManager {
    /// 应用句柄
    app_handle: AppHandle,
    /// 数据库
    db: Arc<Database>,
    /// 配置
    config: Arc<RwLock<ClipboardConfig>>,
    /// 剪贴板监听器
    monitor: Arc<monitor::ClipboardMonitor>,
    /// 窗口管理器
    window_manager: Arc<window::ClipboardWindowManager>,
    /// 内存缓存（最近条目）
    recent_cache: Arc<RwLock<Vec<ClipboardEntry>>>,
    /// 事件广播
    event_tx: broadcast::Sender<ClipboardEvent>,
}

#[derive(Debug, Clone)]
pub enum ClipboardEvent {
    NewEntry(ClipboardEntry),
    EntryDeleted(i64),
    FavoriteToggled(i64, bool),
}

impl ClipboardManager {
    pub fn new(
        app_handle: AppHandle,
        db: Arc<Database>,
        config: ClipboardConfig,
    ) -> Result<Self, ClipboardError> {
        let (event_tx, _) = broadcast::channel(100);

        let config = Arc::new(RwLock::new(config));
        let recent_cache = Arc::new(RwLock::new(Vec::new()));

        let monitor = Arc::new(monitor::ClipboardMonitor::new(
            app_handle.clone(),
            config.clone(),
        )?);

        let window_manager = Arc::new(window::ClipboardWindowManager::new());

        Ok(Self {
            app_handle,
            db,
            config,
            monitor,
            window_manager,
            recent_cache,
            event_tx,
        })
    }

    /// 启动剪贴板监听
    pub async fn start_monitoring(&self) -> Result<(), ClipboardError> {
        let db = self.db.clone();
        let config = self.config.clone();
        let cache = self.recent_cache.clone();
        let event_tx = self.event_tx.clone();
        let app_handle = self.app_handle.clone();

        self.monitor.start(move |content, app_name| {
            let db = db.clone();
            let config = config.clone();
            let cache = cache.clone();
            let event_tx = event_tx.clone();
            let app_handle = app_handle.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_clipboard_change(
                    db, config, cache, event_tx, app_handle, content, app_name
                ).await {
                    tracing::error!("Failed to handle clipboard change: {}", e);
                }
            });
        })?;

        Ok(())
    }

    /// 处理剪贴板变化
    async fn handle_clipboard_change(
        db: Arc<Database>,
        config: Arc<RwLock<ClipboardConfig>>,
        cache: Arc<RwLock<Vec<ClipboardEntry>>>,
        event_tx: broadcast::Sender<ClipboardEvent>,
        app_handle: AppHandle,
        content: ClipboardContent,
        app_name: Option<String>,
    ) -> Result<(), ClipboardError> {
        let config = config.read().await;

        // 检查是否忽略该应用
        if let Some(ref name) = app_name {
            if config.ignore_apps.iter().any(|a| a.eq_ignore_ascii_case(name)) {
                return Ok(());
            }
        }

        // 检查是否为敏感内容
        let is_sensitive = Self::check_sensitive(&content, &config.sensitive_patterns);

        // 生成预览
        let preview = Self::generate_preview(&content);

        // 创建条目
        let entry = ClipboardEntry {
            id: 0,
            content,
            preview,
            app_name,
            app_icon: None,
            created_at: chrono::Utc::now().timestamp(),
            is_favorite: false,
            is_sensitive,
        };

        drop(config);

        // 保存到数据库
        let saved_entry = db.save_clipboard_entry(&entry).await?;

        // 更新缓存
        let mut cache = cache.write().await;
        cache.insert(0, saved_entry.clone());
        if cache.len() > 100 {
            cache.truncate(100);
        }
        drop(cache);

        // 发送事件
        let _ = event_tx.send(ClipboardEvent::NewEntry(saved_entry.clone()));

        // 通知前端
        app_handle.emit_all("clipboard:new-entry", &saved_entry).ok();

        Ok(())
    }

    /// 切换剪贴板窗口
    pub fn toggle_window(&self) -> Result<(), ClipboardError> {
        self.window_manager.toggle(&self.app_handle)
    }

    /// 获取历史记录
    pub async fn get_history(
        &self,
        query: Option<&str>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<ClipboardEntry>, ClipboardError> {
        // 优先从缓存获取
        if query.is_none() && offset == 0 {
            let cache = self.recent_cache.read().await;
            if cache.len() >= limit {
                return Ok(cache.iter().take(limit).cloned().collect());
            }
        }

        self.db.get_clipboard_history(query, limit, offset).await
            .map_err(ClipboardError::from)
    }

    /// 从历史复制到剪贴板
    pub async fn copy_from_history(&self, entry_id: i64) -> Result<(), ClipboardError> {
        let entry = self.db.get_clipboard_entry(entry_id).await?
            .ok_or(ClipboardError::EntryNotFound(entry_id))?;

        // 暂停监听以避免循环
        self.monitor.pause();

        // 写入系统剪贴板
        paste::write_to_clipboard(&entry.content)?;

        // 恢复监听
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        self.monitor.resume();

        Ok(())
    }

    /// 模拟粘贴
    pub async fn simulate_paste(&self) -> Result<(), ClipboardError> {
        paste::simulate_paste().await
    }

    /// 删除历史条目
    pub async fn delete_entry(&self, entry_id: i64) -> Result<(), ClipboardError> {
        self.db.delete_clipboard_entry(entry_id).await?;

        let mut cache = self.recent_cache.write().await;
        cache.retain(|e| e.id != entry_id);
        drop(cache);

        let _ = self.event_tx.send(ClipboardEvent::EntryDeleted(entry_id));
        self.app_handle.emit_all("clipboard:entry-deleted", entry_id).ok();

        Ok(())
    }

    /// 切换收藏状态
    pub async fn toggle_favorite(&self, entry_id: i64) -> Result<bool, ClipboardError> {
        let new_state = self.db.toggle_clipboard_favorite(entry_id).await?;

        let mut cache = self.recent_cache.write().await;
        if let Some(entry) = cache.iter_mut().find(|e| e.id == entry_id) {
            entry.is_favorite = new_state;
        }
        drop(cache);

        let _ = self.event_tx.send(ClipboardEvent::FavoriteToggled(entry_id, new_state));
        self.app_handle.emit_all("clipboard:favorite-toggled", (entry_id, new_state)).ok();

        Ok(new_state)
    }

    /// 清理过期历史
    pub async fn cleanup_expired(&self) -> Result<usize, ClipboardError> {
        let config = self.config.read().await;

        if let Some(days) = config.retention_days {
            let cutoff = chrono::Utc::now().timestamp() - (days as i64 * 86400);
            let deleted = self.db.delete_clipboard_before(cutoff).await?;

            tracing::info!("Cleaned up {} expired clipboard entries", deleted);
            return Ok(deleted);
        }

        // 检查数量限制
        let count = self.db.get_clipboard_count().await?;
        if count > config.max_history {
            let to_delete = count - config.max_history;
            let deleted = self.db.delete_oldest_clipboard_entries(to_delete).await?;
            tracing::info!("Cleaned up {} clipboard entries (exceeded limit)", deleted);
            return Ok(deleted);
        }

        Ok(0)
    }

    /// 更新配置
    pub fn update_config(&self, config: ClipboardConfig) -> Result<(), ClipboardError> {
        let mut current = self.config.blocking_write();
        *current = config;
        Ok(())
    }

    fn check_sensitive(content: &ClipboardContent, patterns: &[String]) -> bool {
        let text = match content {
            ClipboardContent::Text { text, .. } => text,
            _ => return false,
        };

        for pattern in patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if re.is_match(text) {
                    return true;
                }
            }
        }

        false
    }

    fn generate_preview(content: &ClipboardContent) -> String {
        match content {
            ClipboardContent::Text { text, .. } => {
                let preview = text.chars().take(200).collect::<String>();
                if text.len() > 200 {
                    format!("{}...", preview)
                } else {
                    preview
                }
            }
            ClipboardContent::Image { width, height, .. } => {
                format!("Image ({}x{})", width, height)
            }
            ClipboardContent::Files { paths } => {
                if paths.len() == 1 {
                    paths[0].clone()
                } else {
                    format!("{} files", paths.len())
                }
            }
            ClipboardContent::Color { hex, .. } => {
                format!("Color: {}", hex)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClipboardError {
    #[error("Entry not found: {0}")]
    EntryNotFound(i64),
    #[error("Database error: {0}")]
    Database(#[from] crate::storage::DatabaseError),
    #[error("Platform error: {0}")]
    Platform(String),
    #[error("Window error: {0}")]
    Window(#[from] tauri::Error),
}

```

### 3.6 剪贴板独立窗口 (ClipboardWindowManager)

```rust
// src-tauri/src/core/clipboard/window.rs

use tauri::{AppHandle, Manager, Window, WindowBuilder, WindowUrl, PhysicalPosition};

pub struct ClipboardWindowManager {
    window_label: String,
}

impl ClipboardWindowManager {
    pub const WINDOW_LABEL: &'static str = "clipboard";

    pub fn new() -> Self {
        Self {
            window_label: Self::WINDOW_LABEL.to_string(),
        }
    }

    /// 切换窗口显示状态
    pub fn toggle(&self, app_handle: &AppHandle) -> Result<(), super::ClipboardError> {
        if let Some(window) = app_handle.get_window(&self.window_label) {
            if window.is_visible().unwrap_or(false) {
                window.hide()?;
            } else {
                self.position_and_show(&window)?;
            }
        } else {
            self.create_and_show(app_handle)?;
        }

        Ok(())
    }

    /// 创建并显示窗口
    fn create_and_show(&self, app_handle: &AppHandle) -> Result<Window, super::ClipboardError> {
        let window = WindowBuilder::new(
            app_handle,
            &self.window_label,
            WindowUrl::App("index.html#/clipboard".into()),
        )
        .title("Clipboard History")
        .inner_size(400.0, 500.0)
        .min_inner_size(300.0, 300.0)
        .resizable(true)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .visible(false)
        .focused(true)
        .build()?;

        // 注册失焦自动隐藏
        let window_clone = window.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(false) = event {
                window_clone.hide().ok();
            }
        });

        self.position_and_show(&window)?;

        Ok(window)
    }

    /// 定位并显示窗口
    fn position_and_show(&self, window: &Window) -> Result<(), super::ClipboardError> {
        // 获取光标位置
        let cursor_pos = self.get_cursor_position()?;

        // 获取窗口大小
        let window_size = window.outer_size()?;

        // 获取当前显示器
        if let Some(monitor) = window.current_monitor()? {
            let monitor_pos = monitor.position();
            let monitor_size = monitor.size();

            // 计算窗口位置，确保不超出屏幕
            let mut x = cursor_pos.0;
            let mut y = cursor_pos.1;

            // 右边界检查
            if x + window_size.width as i32 > monitor_pos.x + monitor_size.width as i32 {
                x = monitor_pos.x + monitor_size.width as i32 - window_size.width as i32;
            }

            // 下边界检查
            if y + window_size.height as i32 > monitor_pos.y + monitor_size.height as i32 {
                y = cursor_pos.1 - window_size.height as i32;
            }

            // 左边界检查
            if x < monitor_pos.x {
                x = monitor_pos.x;
            }

            // 上边界检查
            if y < monitor_pos.y {
                y = monitor_pos.y;
            }

            window.set_position(PhysicalPosition::new(x, y))?;
        }

        window.show()?;
        window.set_focus()?;

        Ok(())
    }

    /// 获取光标位置
    #[cfg(target_os = "macos")]
    fn get_cursor_position(&self) -> Result<(i32, i32), super::ClipboardError> {
        use core_graphics::event::CGEvent;
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
            .map_err(|_| super::ClipboardError::Platform("Failed to create event source".into()))?;

        let event = CGEvent::new(source)
            .map_err(|_| super::ClipboardError::Platform("Failed to create event".into()))?;

        let point = event.location();
        Ok((point.x as i32, point.y as i32))
    }

    #[cfg(target_os = "windows")]
    fn get_cursor_position(&self) -> Result<(i32, i32), super::ClipboardError> {
        use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
        use windows::Win32::Foundation::POINT;

        let mut point = POINT::default();
        unsafe {
            GetCursorPos(&mut point)
                .map_err(|e| super::ClipboardError::Platform(format!("GetCursorPos failed: {}", e)))?;
        }

        Ok((point.x, point.y))
    }
}

```

---

## 四、截图系统设计

### 4.1 截图管理器

```rust
// src-tauri/src/core/screenshot/mod.rs

pub mod capture;
pub mod selector;
pub mod editor;
pub mod pin;
pub mod picker;

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Window};

use crate::app::config::ScreenshotConfig;

/// 截图结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureResult {
    pub id: String,
    pub image_data: String,  // Base64 PNG
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub monitor_id: u32,
    pub captured_at: i64,
}

/// 截图区域
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub monitor_id: Option<u32>,
}

/// 颜色值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorValue {
    pub hex: String,
    pub rgb: (u8, u8, u8),
    pub hsl: (f32, f32, f32),
    pub position: (i32, i32),
}

/// 贴图信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinInfo {
    pub id: String,
    pub image_data: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub opacity: f32,
    pub scale: f32,
    pub click_through: bool,
}

/// 截图管理器
pub struct ScreenshotManager {
    app_handle: AppHandle,
    config: Arc<RwLock<ScreenshotConfig>>,
    /// 活跃的贴图窗口
    pins: Arc<RwLock<HashMap<String, PinInfo>>>,
    /// 当前截图
    current_capture: Arc<RwLock<Option<CaptureResult>>>,
}

impl ScreenshotManager {
    pub fn new(app_handle: AppHandle, config: ScreenshotConfig) -> Result<Self, ScreenshotError> {
        Ok(Self {
            app_handle,
            config: Arc::new(RwLock::new(config)),
            pins: Arc::new(RwLock::new(HashMap::new())),
            current_capture: Arc::new(RwLock::new(None)),
        })
    }

    /// 启动区域截图
    pub async fn start_region_capture(&self) -> Result<(), ScreenshotError> {
        // 1. 隐藏 OmniBox 窗口
        self.hide_omnibox_windows()?;

        // 2. 短暂延迟
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        // 3. 创建全屏选区覆盖层
        self.create_selection_overlay().await?;

        Ok(())
    }

    /// 完成区域截图
    pub async fn complete_region_capture(&self, region: CaptureRegion) -> Result<CaptureResult, ScreenshotError> {
        // 1. 关闭选区覆盖层
        if let Some(overlay) = self.app_handle.get_window("screenshot-overlay") {
            overlay.close()?;
        }

        // 2. 执行截图
        let result = capture::capture_region(&region).await?;

        // 3. 存储当前截图
        *self.current_capture.write().await = Some(result.clone());

        // 4. 播放音效
        let config = self.config.read().await;
        if config.play_sound {
            self.play_capture_sound();
        }

        // 5. 打开编辑器
        self.open_editor(&result).await?;

        Ok(result)
    }

    /// 快速截图并贴图
    pub async fn quick_capture_and_pin(&self, region: CaptureRegion) -> Result<PinInfo, ScreenshotError> {
        // 1. 关闭选区覆盖层
        if let Some(overlay) = self.app_handle.get_window("screenshot-overlay") {
            overlay.close()?;
        }

        // 2. 执行截图
        let result = capture::capture_region(&region).await?;

        // 3. 创建贴图
        let pin = self.create_pin(
            result.image_data,
            result.width,
            result.height,
            region.x,
            region.y,
        ).await?;

        Ok(pin)
    }

    /// 窗口截图
    pub async fn capture_window(&self, window_id: Option<u32>) -> Result<CaptureResult, ScreenshotError> {
        self.hide_omnibox_windows()?;
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        let result = if let Some(id) = window_id {
            capture::capture_window(id).await?
        } else {
            // 显示窗口选择器
            self.show_window_selector().await?
        };

        let config = self.config.read().await;
        if config.play_sound {
            self.play_capture_sound();
        }

        self.open_editor(&result).await?;

        Ok(result)
    }

    /// 全屏截图
    pub async fn capture_fullscreen(&self, monitor_id: Option<u32>) -> Result<CaptureResult, ScreenshotError> {
        self.hide_omnibox_windows()?;
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        let result = capture::capture_fullscreen(monitor_id).await?;

        let config = self.config.read().await;
        if config.play_sound {
            self.play_capture_sound();
        }

        self.open_editor(&result).await?;

        Ok(result)
    }

    /// 取色器
    pub async fn start_color_picker(&self) -> Result<(), ScreenshotError> {
        self.create_color_picker_overlay().await
    }

    /// 完成取色
    pub async fn pick_color(&self, x: i32, y: i32) -> Result<ColorValue, ScreenshotError> {
        // 关闭取色器覆盖层
        if let Some(overlay) = self.app_handle.get_window("color-picker-overlay") {
            overlay.close()?;
        }

        picker::pick_color_at(x, y).await
    }

    /// 创建贴图
    pub async fn create_pin(
        &self,
        image_data: String,
        width: u32,
        height: u32,
        x: i32,
        y: i32,
    ) -> Result<PinInfo, ScreenshotError> {
        let id = uuid::Uuid::new_v4().to_string();

        let pin = PinInfo {
            id: id.clone(),
            image_data,
            width,
            height,
            x,
            y,
            opacity: 1.0,
            scale: 1.0,
            click_through: false,
        };

        // 创建贴图窗口
        self.create_pin_window(&pin).await?;

        // 存储贴图信息
        self.pins.write().await.insert(id.clone(), pin.clone());

        Ok(pin)
    }

    /// 更新贴图
    pub async fn update_pin(&self, id: &str, updates: PinUpdate) -> Result<(), ScreenshotError> {
        let mut pins = self.pins.write().await;

        if let Some(pin) = pins.get_mut(id) {
            if let Some(opacity) = updates.opacity {
                pin.opacity = opacity;
            }
            if let Some(scale) = updates.scale {
                pin.scale = scale;
            }
            if let Some(click_through) = updates.click_through {
                pin.click_through = click_through;
            }
            if let Some((x, y)) = updates.position {
                pin.x = x;
                pin.y = y;
            }

            // 通知窗口更新
            if let Some(window) = self.app_handle.get_window(&format!("pin-{}", id)) {
                window.emit("pin:update", pin)?;
            }
        }

        Ok(())
    }

    /// 关闭贴图
    pub async fn close_pin(&self, id: &str) -> Result<(), ScreenshotError> {
        self.pins.write().await.remove(id);

        if let Some(window) = self.app_handle.get_window(&format!("pin-{}", id)) {
            window.close()?;
        }

        Ok(())
    }

    /// 关闭所有贴图
    pub async fn close_all_pins(&self) -> Result<(), ScreenshotError> {
        let pins = self.pins.read().await;
        let ids: Vec<_> = pins.keys().cloned().collect();
        drop(pins);

        for id in ids {
            self.close_pin(&id).await?;
        }

        Ok(())
    }

    /// 保存截图
    pub async fn save_to_file(&self, image_data: &str, path: Option<&str>) -> Result<String, ScreenshotError> {
        let config = self.config.read().await;

        let save_path = if let Some(p) = path {
            std::path::PathBuf::from(p)
        } else {
            // 生成默认文件名
            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
            let extension = match config.default_format {
                super::super::app::config::ImageFormat::Png => "png",
                super::super::app::config::ImageFormat::Jpeg => "jpg",
                super::super::app::config::ImageFormat::WebP => "webp",
            };
            config.default_save_path.join(format!("screenshot_{}.{}", timestamp, extension))
        };

        // 确保目录存在
        if let Some(parent) = save_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 解码 Base64 并保存
        let image_bytes = base64::decode(image_data)
            .map_err(|e| ScreenshotError::InvalidImageData(e.to_string()))?;

        std::fs::write(&save_path, image_bytes)?;

        Ok(save_path.to_string_lossy().to_string())
    }

    /// 复制到剪贴板
    pub async fn copy_to_clipboard(&self, image_data: &str) -> Result<(), ScreenshotError> {
        capture::copy_image_to_clipboard(image_data).await
    }

    /// 更新配置
    pub fn update_config(&self, config: ScreenshotConfig) -> Result<(), ScreenshotError> {
        let mut current = self.config.blocking_write();
        *current = config;
        Ok(())
    }

    // 私有方法

    fn hide_omnibox_windows(&self) -> Result<(), ScreenshotError> {
        for (label, window) in self.app_handle.windows() {
            if !label.starts_with("screenshot") && !label.starts_with("pin") {
                window.hide()?;
            }
        }
        Ok(())
    }

    async fn create_selection_overlay(&self) -> Result<Window, ScreenshotError> {
        use tauri::{WindowBuilder, WindowUrl};

        let window = WindowBuilder::new(
            &self.app_handle,
            "screenshot-overlay",
            WindowUrl::App("index.html#/screenshot/overlay".into()),
        )
        .title("Screenshot")
        .fullscreen(true)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .focused(true)
        .build()?;

        Ok(window)
    }

    async fn open_editor(&self, capture: &CaptureResult) -> Result<Window, ScreenshotError> {
        use tauri::{WindowBuilder, WindowUrl};

        let window = WindowBuilder::new(
            &self.app_handle,
            "screenshot-editor",
            WindowUrl::App("index.html#/screenshot/editor".into()),
        )
        .title("Screenshot Editor")
        .inner_size(
            (capture.width as f64 + 200.0).min(1200.0),
            (capture.height as f64 + 150.0).min(900.0),
        )
        .min_inner_size(400.0, 300.0)
        .decorations(true)
        .resizable(true)
        .center()
        .focused(true)
        .build()?;

        // 发送截图数据
        window.emit("screenshot:loaded", capture)?;

        Ok(window)
    }

    async fn create_pin_window(&self, pin: &PinInfo) -> Result<Window, ScreenshotError> {
        use tauri::{WindowBuilder, WindowUrl, PhysicalPosition, PhysicalSize};

        let window = WindowBuilder::new(
            &self.app_handle,
            &format!("pin-{}", pin.id),
            WindowUrl::App(format!("index.html#/screenshot/pin?id={}", pin.id).into()),
        )
        .title("Pin")
        .inner_size(pin.width as f64, pin.height as f64)
        .position(pin.x as f64, pin.y as f64)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .build()?;

        // 发送贴图数据
        window.emit("pin:loaded", pin)?;

        Ok(window)
    }

    async fn create_color_picker_overlay(&self) -> Result<(), ScreenshotError> {
        use tauri::{WindowBuilder, WindowUrl};

        WindowBuilder::new(
            &self.app_handle,
            "color-picker-overlay",
            WindowUrl::App("index.html#/screenshot/color-picker".into()),
        )
        .title("Color Picker")
        .fullscreen(true)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .focused(true)
        .build()?;

        Ok(())
    }

    async fn show_window_selector(&self) -> Result<CaptureResult, ScreenshotError> {
        // 实现窗口选择器
        todo!()
    }

    fn play_capture_sound(&self) {
        // 播放截图音效
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("afplay")
                .arg("/System/Library/Components/CoreAudio.component/Contents/SharedSupport/SystemSounds/system/Grab.aif")
                .spawn()
                .ok();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinUpdate {
    pub opacity: Option<f32>,
    pub scale: Option<f32>,
    pub click_through: Option<bool>,
    pub position: Option<(i32, i32)>,
}

#[derive(Debug, thiserror::Error)]
pub enum ScreenshotError {
    #[error("Capture failed: {0}")]
    CaptureFailed(String),
    #[error("Invalid image data: {0}")]
    InvalidImageData(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Window error: {0}")]
    Window(#[from] tauri::Error),
    #[error("Platform error: {0}")]
    Platform(String),
}

```

---

## 五、工作流引擎设计

### 5.1 工作流数据结构

```rust
// src-tauri/src/core/workflow/mod.rs

pub mod engine;
pub mod nodes;
pub mod triggers;
pub mod context;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::app::config::WorkflowConfig;
use crate::storage::Database;

/// 工作流定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub triggers: Vec<WorkflowTrigger>,
    pub nodes: Vec<WorkflowNode>,
    pub connections: Vec<Connection>,
    pub variables: HashMap<String, VariableDefinition>,
    pub enabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 工作流触发器
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkflowTrigger {
    Keyword {
        keyword: String,
        show_in_search: bool,
    },
    Hotkey {
        shortcut: String,
    },
    FileAction {
        extensions: Vec<String>,
        action: FileActionType,
    },
    ClipboardContent {
        pattern: String,
    },
    Schedule {
        cron: String,
    },
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileActionType {
    Open,
    Copy,
    Move,
    Delete,
}

/// 工作流节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub config: serde_json::Value,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// 节点类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeType {
    // 输入节点
    Input { input_type: InputType },
    UserInput { prompt: String, default_value: Option<String> },

    // 逻辑节点
    Condition { conditions: Vec<ConditionRule> },
    Loop { max_iterations: Option<u32> },
    Delay { milliseconds: u64 },

    // 转换节点
    Transform { operation: TransformOperation },
    Template { template: String },
    Script { language: ScriptLanguage, code: String },

    // 动作节点
    OpenUrl { url_template: String },
    OpenFile { path_template: String },
    LaunchApp { app_id: String, arguments: Vec<String> },
    RunShell { command: String, shell: Option<String> },
    HttpRequest {
        method: String,
        url: String,
        headers: HashMap<String, String>,
        body: Option<String>,
    },
    CopyToClipboard { content_template: String },
    Notification { title: String, body: String },
    WriteFile { path: String, content: String, append: bool },
    ShowDialog { dialog_type: DialogType, message: String },

    // AI 节点
    AI { prompt_template: String, provider: Option<String> },

    // 输出节点
    Output { output_type: OutputType },

    // 子工作流
    SubWorkflow { workflow_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputType {
    Text,
    File,
    Clipboard,
    Selection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformOperation {
    JsonParse,
    JsonStringify,
    RegexExtract { pattern: String, group: usize },
    Replace { find: String, replace: String },
    Split { separator: String },
    Join { separator: String },
    Uppercase,
    Lowercase,
    Trim,
    Base64Encode,
    Base64Decode,
    UrlEncode,
    UrlDecode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptLanguage {
    JavaScript,
    Python,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DialogType {
    Info,
    Warning,
    Error,
    Input,
    Confirm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputType {
    Notification,
    Clipboard,
    ShowResult,
    Silent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionRule {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
    pub target_node: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    Matches,
    GreaterThan,
    LessThan,
    IsEmpty,
    IsNotEmpty,
}

/// 节点连接
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
}

/// 变量定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDefinition {
    pub name: String,
    pub var_type: VariableType,
    pub default_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

```

### 5.2 工作流执行引擎

```rust
// src-tauri/src/core/workflow/engine.rs

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::*;
use super::context::ExecutionContext;
use super::nodes::NodeExecutor;
use crate::storage::Database;

/// 工作流执行引擎
pub struct WorkflowEngine {
    /// 工作流存储
    workflows: Arc<RwLock<HashMap<String, Workflow>>>,
    /// 数据库
    db: Arc<Database>,
    /// 节点执行器
    node_executor: Arc<NodeExecutor>,
    /// 配置
    config: WorkflowConfig,
}

impl WorkflowEngine {
    pub async fn new(db: Arc<Database>, config: WorkflowConfig) -> Result<Self, WorkflowError> {
        let engine = Self {
            workflows: Arc::new(RwLock::new(HashMap::new())),
            db,
            node_executor: Arc::new(NodeExecutor::new()),
            config,
        };

        engine.load_all().await?;

        Ok(engine)
    }

    /// 加载所有工作流
    pub async fn load_all(&self) -> Result<(), WorkflowError> {
        let workflows = self.db.get_all_workflows().await?;
        let mut map = self.workflows.write().await;

        for workflow in workflows {
            map.insert(workflow.id.clone(), workflow);
        }

        Ok(())
    }

    /// 创建工作流
    pub async fn create(&self, workflow: Workflow) -> Result<String, WorkflowError> {
        let id = if workflow.id.is_empty() {
            uuid::Uuid::new_v4().to_string()
        } else {
            workflow.id.clone()
        };

        let mut workflow = workflow;
        workflow.id = id.clone();
        workflow.created_at = chrono::Utc::now().timestamp();
        workflow.updated_at = workflow.created_at;

        self.db.save_workflow(&workflow).await?;
        self.workflows.write().await.insert(id.clone(), workflow);

        Ok(id)
    }

    /// 更新工作流
    pub async fn update(&self, workflow: Workflow) -> Result<(), WorkflowError> {
        let mut workflow = workflow;
        workflow.updated_at = chrono::Utc::now().timestamp();

        self.db.save_workflow(&workflow).await?;
        self.workflows.write().await.insert(workflow.id.clone(), workflow);

        Ok(())
    }

    /// 删除工作流
    pub async fn delete(&self, id: &str) -> Result<(), WorkflowError> {
        self.db.delete_workflow(id).await?;
        self.workflows.write().await.remove(id);
        Ok(())
    }

    /// 获取工作流
    pub async fn get(&self, id: &str) -> Option<Workflow> {
        self.workflows.read().await.get(id).cloned()
    }

    /// 获取所有工作流
    pub async fn list(&self) -> Vec<Workflow> {
        self.workflows.read().await.values().cloned().collect()
    }

    /// 执行工作流
    pub async fn execute(
        &self,
        workflow_id: &str,
        input: serde_json::Value,
    ) -> Result<ExecutionResult, WorkflowError> {
        let workflow = self.get(workflow_id).await
            .ok_or(WorkflowError::NotFound(workflow_id.to_string()))?;

        if !workflow.enabled {
            return Err(WorkflowError::Disabled(workflow_id.to_string()));
        }

        // 创建执行上下文
        let mut context = ExecutionContext::new(&workflow, input);

        // 拓扑排序获取执行顺序
        let execution_order = self.topological_sort(&workflow)?;

        // 执行节点
        for node_id in execution_order {
            let node = workflow.nodes.iter()
                .find(|n| n.id == node_id)
                .ok_or(WorkflowError::NodeNotFound(node_id.clone()))?;

            let result = self.node_executor.execute(node, &mut context).await?;
            context.set_node_output(&node_id, result);

            // 检查是否需要跳转（条件节点）
            if let Some(next_node) = context.get_next_node() {
                // 处理条件跳转
            }
        }

        Ok(ExecutionResult {
            workflow_id: workflow_id.to_string(),
            success: true,
            output: context.get_final_output(),
            duration_ms: context.get_duration_ms(),
            node_outputs: context.get_all_outputs(),
        })
    }

    /// 通过触发器查找工作流
    pub async fn find_by_trigger(&self, trigger: &WorkflowTrigger) -> Vec<Workflow> {
        let workflows = self.workflows.read().await;

        workflows.values()
            .filter(|w| w.enabled && w.triggers.iter().any(|t| self.trigger_matches(t, trigger)))
            .cloned()
            .collect()
    }

    fn trigger_matches(&self, workflow_trigger: &WorkflowTrigger, query_trigger: &WorkflowTrigger) -> bool {
        match (workflow_trigger, query_trigger) {
            (WorkflowTrigger::Keyword { keyword: wk, .. }, WorkflowTrigger::Keyword { keyword: qk, .. }) => {
                wk.eq_ignore_ascii_case(qk)
            }
            (WorkflowTrigger::Hotkey { shortcut: ws }, WorkflowTrigger::Hotkey { shortcut: qs }) => {
                ws == qs
            }
            _ => false,
        }
    }

    fn topological_sort(&self, workflow: &Workflow) -> Result<Vec<String>, WorkflowError> {
        // 构建邻接表
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();

        for node in &workflow.nodes {
            in_degree.entry(node.id.clone()).or_insert(0);
            adjacency.entry(node.id.clone()).or_insert_with(Vec::new);
        }

        for conn in &workflow.connections {
            *in_degree.entry(conn.to_node.clone()).or_insert(0) += 1;
            adjacency.entry(conn.from_node.clone())
                .or_insert_with(Vec::new)
                .push(conn.to_node.clone());
        }

        // Kahn 算法
        let mut queue: Vec<String> = in_degree.iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut result = Vec::new();

        while let Some(node_id) = queue.pop() {
            result.push(node_id.clone());

            if let Some(neighbors) = adjacency.get(&node_id) {
                for neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(neighbor.clone());
                        }
                    }
                }
            }
        }

        if result.len() != workflow.nodes.len() {
            return Err(WorkflowError::CyclicDependency);
        }

        Ok(result)
    }
}

/// 执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub workflow_id: String,
    pub success: bool,
    pub output: serde_json::Value,
    pub duration_ms: u64,
    pub node_outputs: HashMap<String, serde_json::Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkflowError {
    #[error("Workflow not found: {0}")]
    NotFound(String),
    #[error("Workflow disabled: {0}")]
    Disabled(String),
    #[error("Node not found: {0}")]
    NodeNotFound(String),
    #[error("Cyclic dependency detected")]
    CyclicDependency,
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[error("Database error: {0}")]
    Database(#[from] crate::storage::DatabaseError),
}

```

---

## 六、插件系统设计

### 6.1 插件管理器

```rust
// src-tauri/src/core/plugin/mod.rs

pub mod loader;
pub mod registry;
pub mod sandbox;
pub mod api;
pub mod store;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// 插件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub icon: Option<String>,
    pub keywords: Vec<String>,
    pub plugin_type: PluginType,
    pub triggers: Vec<PluginTrigger>,
    pub permissions: Vec<Permission>,
    pub main: String,
    pub config_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginType {
    SearchProvider,
    ActionHandler,
    WorkflowNode,
    SystemEnhancement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PluginTrigger {
    Keyword { keyword: String },
    Regex { pattern: String },
    FileType { extensions: Vec<String> },
    Hotkey { shortcut: String },
    Event { event: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    Clipboard,
    FileSystem,
    Network,
    Notifications,
    Shell,
    SystemInfo,
}

/// 插件实例
pub struct PluginInstance {
    pub manifest: PluginManifest,
    pub path: PathBuf,
    pub enabled: bool,
    pub config: serde_json::Value,
    runtime: Option<sandbox::PluginRuntime>,
}

/// 插件管理器
pub struct PluginManager {
    plugins: Arc<RwLock<HashMap<String, PluginInstance>>>,
    plugins_dir: PathBuf,
    registry_url: String,
}

impl PluginManager {
    pub async fn new(plugins_dir: PathBuf) -> Result<Self, PluginError> {
        std::fs::create_dir_all(&plugins_dir)?;

        let manager = Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            plugins_dir,
            registry_url: "<https://plugins.omnibox.app/api/v1>".to_string(),
        };

        manager.load_all().await?;

        Ok(manager)
    }

    /// 加载所有已安装插件
    pub async fn load_all(&self) -> Result<(), PluginError> {
        let entries = std::fs::read_dir(&self.plugins_dir)?;

        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                if let Err(e) = self.load_plugin(&path).await {
                    tracing::warn!("Failed to load plugin {:?}: {}", path, e);
                }
            }
        }

        Ok(())
    }

    /// 加载单个插件
    pub async fn load_plugin(&self, path: &PathBuf) -> Result<String, PluginError> {
        let manifest_path = path.join("manifest.json");
        let manifest: PluginManifest = serde_json::from_str(
            &std::fs::read_to_string(&manifest_path)?
        )?;

        // 验证权限
        self.validate_permissions(&manifest)?;

        let plugin = PluginInstance {
            manifest: manifest.clone(),
            path: path.clone(),
            enabled: true,
            config: serde_json::Value::Object(Default::default()),
            runtime: None,
        };

        let id = manifest.id.clone();
        self.plugins.write().await.insert(id.clone(), plugin);

        tracing::info!("Loaded plugin: {} v{}", manifest.name, manifest.version);

        Ok(id)
    }

    /// 从市场安装插件
    pub async fn install(&self, plugin_id: &str) -> Result<(), PluginError> {
        // 获取插件信息
        let info = self.fetch_plugin_info(plugin_id).await?;

        // 下载插件包
        let response = reqwest::get(&info.download_url).await?;
        let bytes = response.bytes().await?;

        // 解压到插件目录
        let plugin_dir = self.plugins_dir.join(plugin_id);
        std::fs::create_dir_all(&plugin_dir)?;

        let cursor = std::io::Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(cursor)?;
        archive.extract(&plugin_dir)?;

        // 加载插件
        self.load_plugin(&plugin_dir).await?;

        Ok(())
    }

    /// 卸载插件
    pub async fn uninstall(&self, plugin_id: &str) -> Result<(), PluginError> {
        let mut plugins = self.plugins.write().await;

        if let Some(plugin) = plugins.remove(plugin_id) {
            std::fs::remove_dir_all(&plugin.path)?;
            tracing::info!("Uninstalled plugin: {}", plugin_id);
        }

        Ok(())
    }

    /// 启用/禁用插件
    pub async fn set_enabled(&self, plugin_id: &str, enabled: bool) -> Result<(), PluginError> {
        let mut plugins = self.plugins.write().await;

        if let Some(plugin) = plugins.get_mut(plugin_id) {
            plugin.enabled = enabled;
        }

        Ok(())
    }

    /// 获取已安装插件列表
    pub async fn list_installed(&self) -> Vec<PluginManifest> {
        self.plugins.read().await
            .values()
            .map(|p| p.manifest.clone())
            .collect()
    }

    /// 搜索插件市场
    pub async fn search_store(&self, query: &str) -> Result<Vec<store::PluginInfo>, PluginError> {
        store::search(&self.registry_url, query).await
    }

    /// 执行插件搜索
    pub async fn search(
        &self,
        query: &str,
        trigger: &PluginTrigger,
    ) -> Vec<SearchResult> {
        let plugins = self.plugins.read().await;
        let mut results = Vec::new();

        for plugin in plugins.values() {
            if !plugin.enabled {
                continue;
            }

            // 检查触发器匹配
            let matches = plugin.manifest.triggers.iter().any(|t| {
                match (t, trigger) {
                    (PluginTrigger::Keyword { keyword: pk }, PluginTrigger::Keyword { keyword: qk }) => {
                        pk.eq_ignore_ascii_case(qk)
                    }
                    _ => false
                }
            });

            if matches {
                // 调用插件搜索方法
                if let Some(ref runtime) = plugin.runtime {
                    // 在沙箱中执行
                }
            }
        }

        results
    }

    fn validate_permissions(&self, manifest: &PluginManifest) -> Result<(), PluginError> {
        // 验证插件声明的权限是否合理
        for permission in &manifest.permissions {
            match permission {
                Permission::Shell => {
                    tracing::warn!(
                        "Plugin {} requests Shell permission - use with caution",
                        manifest.id
                    );
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn fetch_plugin_info(&self, plugin_id: &str) -> Result<store::PluginInfo, PluginError> {
        let url = format!("{}/plugins/{}", self.registry_url, plugin_id);
        let response = reqwest::get(&url).await?;
        let info: store::PluginInfo = response.json().await?;
        Ok(info)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub icon: Option<String>,
    pub score: f64,
    pub action: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("Plugin not found: {0}")]
    NotFound(String),
    #[error("Invalid manifest: {0}")]
    InvalidManifest(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

```

---

## 七、Tauri Commands 设计

### 7.1 搜索相关命令

```rust
// src-tauri/src/commands/search.rs

use tauri::State;
use crate::app::state::AppState;
use crate::core::parser::ParseResult;
use crate::core::indexer::FileSearchResult;

/// 解析并搜索
#[tauri::command]
pub async fn parse_and_search(
    state: State<'_, AppState>,
    query: String,
) -> Result<SearchResponse, String> {
    // 解析输入
    let parse_result = state.parser.parse(&query);

    // 根据解析结果获取搜索结果
    let results = match &parse_result {
        ParseResult::Empty => vec![],

        ParseResult::Search { query, filters } => {
            let file_results = state.indexer.search(query, filters, 10).await;
            let app_results = state.indexer.search_apps(query, 5).await;

            merge_and_convert_results(file_results, app_results)
        }

        ParseResult::WebSearch { engine, query, url } => {
            vec![SearchResultItem {
                id: format!("web-{}", engine.id),
                result_type: "web".to_string(),
                title: format!("Search {}: \\"{}\\"", engine.name, query),
                subtitle: Some(url.clone()),
                icon: engine.icon.clone(),
                score: 1.0,
                action: SearchAction::OpenUrl { url: url.clone() },
            }]
        }

        ParseResult::OpenUrl { url } => {
            vec![SearchResultItem {
                id: "url-open".to_string(),
                result_type: "web".to_string(),
                title: format!("Open {}", url),
                subtitle: Some("Press Enter to open in browser".to_string()),
                icon: None,
                score: 1.0,
                action: SearchAction::OpenUrl { url: url.clone() },
            }]
        }

        ParseResult::Calculate { expression, result } => {
            vec![SearchResultItem {
                id: "calc".to_string(),
                result_type: "calculator".to_string(),
                title: result.formatted.clone(),
                subtitle: Some(format!("= {}", expression)),
                icon: None,
                score: 1.0,
                action: SearchAction::CopyText { text: result.formatted.clone() },
            }]
        }

        ParseResult::Route { module, query } => {
            handle_module_route(&state, module, query).await
        }

        ParseResult::Command { command } => {
            vec![SearchResultItem {
                id: "command".to_string(),
                result_type: "command".to_string(),
                title: format!("Run: {}", command),
                subtitle: Some("Press Enter to execute".to_string()),
                icon: None,
                score: 1.0,
                action: SearchAction::RunCommand { command: command.clone() },
            }]
        }
    };

    Ok(SearchResponse {
        parse_result,
        results,
    })
}

/// 执行搜索结果动作
#[tauri::command]
pub async fn execute_action(
    state: State<'_, AppState>,
    action: SearchAction,
) -> Result<(), String> {
    match action {
        SearchAction::OpenFile { path } => {
            opener::open(&path).map_err(|e| e.to_string())?;
            state.indexer.record_access(&path.into()).await.ok();
        }
        SearchAction::OpenUrl { url } => {
            opener::open(&url).map_err(|e| e.to_string())?;
        }
        SearchAction::LaunchApp { path } => {
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open")
                    .arg("-a")
                    .arg(&path)
                    .spawn()
                    .map_err(|e| e.to_string())?;
            }
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("cmd")
                    .args(["/C", "start", "", &path])
                    .spawn()
                    .map_err(|e| e.to_string())?;
            }
            state.indexer.record_access(&path.into()).await.ok();
        }
        SearchAction::CopyText { text } => {
            // 复制到剪贴板
            arboard::Clipboard::new()
                .and_then(|mut cb| cb.set_text(&text))
                .map_err(|e| e.to_string())?;
        }
        SearchAction::RunCommand { command } => {
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&command)
                    .spawn()
                    .map_err(|e| e.to_string())?;
            }
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("cmd")
                    .args(["/C", &command])
                    .spawn()
                    .map_err(|e| e.to_string())?;
            }
        }
        SearchAction::RunWorkflow { workflow_id, input } => {
            state.workflow.execute(&workflow_id, input).await
                .map_err(|e| e.to_string())?;
        }
        SearchAction::ShowInFinder { path } => {
            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open")
                    .arg("-R")
                    .arg(&path)
                    .spawn()
                    .map_err(|e| e.to_string())?;
            }
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("explorer")
                    .arg("/select,")
                    .arg(&path)
                    .spawn()
                    .map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

/// 重建文件索引
#[tauri::command]
pub async fn rebuild_index(
    state: State<'_, AppState>,
) -> Result<crate::core::indexer::IndexStats, String> {
    state.indexer.rebuild().await.map_err(|e| e.to_string())
}

/// 获取索引统计
#[tauri::command]
pub async fn get_index_stats(
    state: State<'_, AppState>,
) -> Result<crate::core::indexer::IndexStats, String> {
    Ok(state.indexer.get_stats().await)
}

// 类型定义

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResponse {
    pub parse_result: ParseResult,
    pub results: Vec<SearchResultItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResultItem {
    pub id: String,
    pub result_type: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub icon: Option<String>,
    pub score: f64,
    pub action: SearchAction,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SearchAction {
    OpenFile { path: String },
    OpenUrl { url: String },
    LaunchApp { path: String },
    CopyText { text: String },
    RunCommand { command: String },
    RunWorkflow { workflow_id: String, input: serde_json::Value },
    ShowInFinder { path: String },
}

// Helper functions

fn merge_and_convert_results(
    files: Vec<FileSearchResult>,
    apps: Vec<FileSearchResult>,
) -> Vec<SearchResultItem> {
    let mut results = Vec::new();

    // 应用优先
    for app in apps {
        results.push(SearchResultItem {
            id: format!("app-{}", app.entry.id),
            result_type: "app".to_string(),
            title: app.entry.name.clone(),
            subtitle: Some(app.entry.path.to_string_lossy().to_string()),
            icon: None, // TODO: 获取应用图标
            score: app.score,
            action: SearchAction::LaunchApp {
                path: app.entry.path.to_string_lossy().to_string(),
            },
        });
    }

    // 文件
    for file in files {
        results.push(SearchResultItem {
            id: format!("file-{}", file.entry.id),
            result_type: if file.entry.is_dir { "folder" } else { "file" }.to_string(),
            title: file.entry.name.clone(),
            subtitle: Some(file.entry.path.to_string_lossy().to_string()),
            icon: None,
            score: file.score,
            action: SearchAction::OpenFile {
                path: file.entry.path.to_string_lossy().to_string(),
            },
        });
    }

    // 按分数排序
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    results
}

async fn handle_module_route(
    state: &State<'_, AppState>,
    module: &crate::core::parser::ModuleType,
    query: &str,
) -> Vec<SearchResultItem> {
    use crate::core::parser::ModuleType;

    match module {
        ModuleType::Ai => {
            vec![SearchResultItem {
                id: "ai-query".to_string(),
                result_type: "ai".to_string(),
                title: format!("Ask AI: \\"{}\\"", query),
                subtitle: Some("Press Enter to start chat".to_string()),
                icon: None,
                score: 1.0,
                action: SearchAction::RunWorkflow {
                    workflow_id: "__ai_chat__".to_string(),
                    input: serde_json::json!({ "query": query }),
                },
            }]
        }
        ModuleType::Clipboard => {
            // 搜索剪贴板历史
            match state.clipboard.get_history(Some(query), 10, 0).await {
                Ok(entries) => {
                    entries.into_iter().map(|e| SearchResultItem {
                        id: format!("clip-{}", e.id),
                        result_type: "clipboard".to_string(),
                        title: e.preview.clone(),
                        subtitle: Some(format_timestamp(e.created_at)),
                        icon: None,
                        score: 1.0,
                        action: SearchAction::CopyText { text: e.preview },
                    }).collect()
                }
                Err(_) => vec![],
            }
        }
        ModuleType::Settings => {
            vec![SearchResultItem {
                id: "settings".to_string(),
                result_type: "settings".to_string(),
                title: "Open Settings".to_string(),
                subtitle: None,
                icon: None,
                score: 1.0,
                action: SearchAction::RunCommand {
                    command: "__open_settings__".to_string(),
                },
            }]
        }
        _ => vec![],
    }
}

fn format_timestamp(timestamp: i64) -> String {
    use chrono::{TimeZone, Utc, Local};

    let dt = Utc.timestamp_opt(timestamp, 0).unwrap();
    let local = dt.with_timezone(&Local);
    local.format("%Y-%m-%d %H:%M").to_string()
}

```

### 7.2 配置相关命令

```rust
// src-tauri/src/commands/settings.rs

use tauri::State;
use crate::app::state::AppState;
use crate::app::config::*;

/// 获取完整配置
#[tauri::command]
pub async fn get_config(
    state: State<'_, AppState>,
) -> Result<AppConfig, String> {
    let config = state.config.read().await;
    Ok(config.get().clone())
}

/// 更新通用配置
#[tauri::command]
pub async fn update_general_config(
    state: State<'_, AppState>,
    config: GeneralConfig,
) -> Result<(), String> {
    let mut cfg = state.config.write().await;
    cfg.general = config;
    cfg.save().map_err(|e| e.to_string())?;

    // 应用配置变更
    if cfg.general.launch_at_startup {
        // 设置开机启动
    }

    Ok(())
}

/// 更新外观配置
#[tauri::command]
pub async fn update_appearance_config(
    state: State<'_, AppState>,
    config: AppearanceConfig,
) -> Result<(), String> {
    let mut cfg = state.config.write().await;
    cfg.appearance = config;
    cfg.save().map_err(|e| e.to_string())?;

    // 通知前端更新主题
    state.app_handle.emit_all("config:appearance-changed", &cfg.appearance).ok();

    Ok(())
}

/// 更新快捷键配置
#[tauri::command]
pub async fn update_shortcuts_config(
    state: State<'_, AppState>,
    config: ShortcutsConfig,
) -> Result<(), String> {
    let mut cfg = state.config.write().await;
    let old_shortcuts = cfg.shortcuts.clone();
    cfg.shortcuts = config.clone();
    cfg.save().map_err(|e| e.to_string())?;

    // 重新注册全局快捷键
    crate::app::shortcuts::update_shortcuts(&state.app_handle, &old_shortcuts, &config)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 更新剪贴板配置
#[tauri::command]
pub async fn update_clipboard_config(
    state: State<'_, AppState>,
    config: ClipboardConfig,
) -> Result<(), String> {
    let mut cfg = state.config.write().await;
    cfg.clipboard = config.clone();
    cfg.save().map_err(|e| e.to_string())?;

    state.clipboard.update_config(config).map_err(|e| e.to_string())?;

    Ok(())
}

/// 更新截图配置
#[tauri::command]
pub async fn update_screenshot_config(
    state: State<'_, AppState>,
    config: ScreenshotConfig,
) -> Result<(), String> {
    let mut cfg = state.config.write().await;
    cfg.screenshot = config.clone();
    cfg.save().map_err(|e| e.to_string())?;

    state.screenshot.update_config(config).map_err(|e| e.to_string())?;

    Ok(())
}

/// 更新 AI 配置
#[tauri::command]
pub async fn update_ai_config(
    state: State<'_, AppState>,
    config: AIConfig,
) -> Result<(), String> {
    let mut cfg = state.config.write().await;
    cfg.ai = config.clone();
    cfg.save().map_err(|e| e.to_string())?;

    state.ai.write().await.update_config(config).map_err(|e| e.to_string())?;

    Ok(())
}

/// 保存 AI API Key
#[tauri::command]
pub async fn save_ai_api_key(
    state: State<'_, AppState>,
    provider_id: String,
    api_key: String,
) -> Result<(), String> {
    state.credentials
        .set(&format!("ai_{}_api_key", provider_id), &api_key)
        .map_err(|e| e.to_string())?;

    // 更新 AI 客户端
    state.ai.write().await.reload_credentials().await.map_err(|e| e.to_string())?;

    Ok(())
}

/// 更新索引配置
#[tauri::command]
pub async fn update_indexer_config(
    state: State<'_, AppState>,
    config: IndexerConfig,
) -> Result<(), String> {
    let mut cfg = state.config.write().await;
    cfg.indexer = config.clone();
    cfg.save().map_err(|e| e.to_string())?;

    state.indexer.update_config(config).await.map_err(|e| e.to_string())?;

    Ok(())
}

/// 更新 Web 搜索配置
#[tauri::command]
pub async fn update_web_search_config(
    state: State<'_, AppState>,
    config: WebSearchConfig,
) -> Result<(), String> {
    let mut cfg = state.config.write().await;
    cfg.web_search = config.clone();
    cfg.save().map_err(|e| e.to_string())?;

    state.parser.update_config(config, cfg.triggers.clone());

    Ok(())
}

/// 添加自定义搜索引擎
#[tauri::command]
pub async fn add_search_engine(
    state: State<'_, AppState>,
    engine: SearchEngine,
) -> Result<(), String> {
    let mut cfg = state.config.write().await;

    // 检查关键词是否已存在
    if cfg.web_search.engines.iter().any(|e| e.keyword == engine.keyword) {
        return Err("Keyword already exists".to_string());
    }

    cfg.web_search.engines.push(engine);
    cfg.save().map_err(|e| e.to_string())?;

    state.parser.update_config(cfg.web_search.clone(), cfg.triggers.clone());

    Ok(())
}

/// 删除搜索引擎
#[tauri::command]
pub async fn delete_search_engine(
    state: State<'_, AppState>,
    engine_id: String,
) -> Result<(), String> {
    let mut cfg = state.config.write().await;

    // 不能删除内置引擎
    if cfg.web_search.engines.iter().any(|e| e.id == engine_id && e.is_builtin) {
        return Err("Cannot delete built-in engine".to_string());
    }

    cfg.web_search.engines.retain(|e| e.id != engine_id);
    cfg.save().map_err(|e| e.to_string())?;

    state.parser.update_config(cfg.web_search.clone(), cfg.triggers.clone());

    Ok(())
}

/// 导出配置
#[tauri::command]
pub async fn export_config(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let config = state.config.read().await;
    serde_json::to_string_pretty(config.get()).map_err(|e| e.to_string())
}

/// 导入配置
#[tauri::command]
pub async fn import_config(
    state: State<'_, AppState>,
    config_json: String,
) -> Result<(), String> {
    let imported: AppConfig = serde_json::from_str(&config_json)
        .map_err(|e| format!("Invalid config: {}", e))?;

    let mut cfg = state.config.write().await;

    // 保留运行时字段
    let data_dir = cfg.data_dir.clone();
    let config_path = cfg.config_path.clone();

    **cfg = imported;
    cfg.data_dir = data_dir;
    cfg.config_path = config_path;

    cfg.save().map_err(|e| e.to_string())?;

    // 重新加载所有模块配置
    state.reload_config().await.map_err(|e| e.to_string())?;

    Ok(())
}

/// 重置配置
#[tauri::command]
pub async fn reset_config(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut cfg = state.config.write().await;

    let data_dir = cfg.data_dir.clone();
    let config_path = cfg.config_path.clone();

    **cfg = AppConfig::default();
    cfg.data_dir = data_dir;
    cfg.config_path = config_path;

    cfg.save().map_err(|e| e.to_string())?;

    state.reload_config().await.map_err(|e| e.to_string())?;

    Ok(())
}

```

---

## 八、前端架构设计

### 8.1 配置中心布局 (参考 Alfred 5)

```tsx
// src/pages/Settings/index.tsx

import { Component, createSignal, For, Show } from "solid-js";
import { Dynamic } from "solid-js/web";
import styles from "./Settings.module.css";

// 配置页面组件
import General from "./General";
import Features from "./Features";
import Appearance from "./Appearance";
import Clipboard from "./Clipboard";
import Screenshot from "./Screenshot";
import AISettings from "./AISettings";
import Workflows from "./Workflows";
import Plugins from "./Plugins";
import WebSearch from "./WebSearch";
import Advanced from "./Advanced";
import About from "./About";

interface SettingsSection {
  id: string;
  name: string;
  icon: string;
  component: Component;
}

const sections: SettingsSection[] = [
  { id: "general", name: "General", icon: "⚙️", component: General },
  { id: "features", name: "Features", icon: "🎛️", component: Features },
  { id: "appearance", name: "Appearance", icon: "🎨", component: Appearance },
  { id: "clipboard", name: "Clipboard", icon: "📋", component: Clipboard },
  { id: "screenshot", name: "Screenshot", icon: "📸", component: Screenshot },
  { id: "ai", name: "AI", icon: "🤖", component: AISettings },
  { id: "workflows", name: "Workflows", icon: "⚡", component: Workflows },
  { id: "plugins", name: "Plugins", icon: "🧩", component: Plugins },
  { id: "web-search", name: "Web Search", icon: "🔍", component: WebSearch },
  { id: "advanced", name: "Advanced", icon: "🔧", component: Advanced },
  { id: "about", name: "About", icon: "ℹ️", component: About },
];

const Settings: Component = () => {
  const [activeSection, setActiveSection] = createSignal("general");

  const currentSection = () => sections.find(s => s.id === activeSection());

  return (
    <div class={styles.settings}>
      {/* 左侧导航栏 */}
      <nav class={styles.sidebar}>
        <div class={styles.sidebarHeader}>
          <h1>Preferences</h1>
        </div>

        <ul class={styles.navList}>
          <For each={sections}>
            {(section) => (
              <li
                class={styles.navItem}
                classList={{ [styles.active]: activeSection() === section.id }}
                onClick={() => setActiveSection(section.id)}
              >
                <span class={styles.navIcon}>{section.icon}</span>
                <span class={styles.navLabel}>{section.name}</span>
              </li>
            )}
          </For>
        </ul>

        <div class={styles.sidebarFooter}>
          <span class={styles.version}>OmniBox v1.0.0</span>
        </div>
      </nav>

      {/* 右侧内容区 */}
      <main class={styles.content}>
        <header class={styles.contentHeader}>
          <h2>
            <span class={styles.headerIcon}>{currentSection()?.icon}</span>
            {currentSection()?.name}
          </h2>
        </header>

        <div class={styles.contentBody}>
          <Show when={currentSection()}>
            <Dynamic component={currentSection()!.component} />
          </Show>
        </div>
      </main>
    </div>
  );
};

export default Settings;

```

### 8.2 配置中心样式

```css
/* src/pages/Settings/Settings.module.css */

.settings {
  display: flex;
  height: 100vh;
  background: var(--bg-primary);
  color: var(--text-primary);
}

/* 左侧导航栏 */
.sidebar {
  width: 220px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
}

.sidebarHeader {
  padding: 20px;
  border-bottom: 1px solid var(--border-color);
}

.sidebarHeader h1 {
  font-size: 18px;
  font-weight: 600;
  margin: 0;
}

.navList {
  list-style: none;
  padding: 8px;
  margin: 0;
  flex: 1;
  overflow-y: auto;
}

.navItem {
  display: flex;
  align-items: center;
  padding: 10px 12px;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
  margin-bottom: 2px;
}

.navItem:hover {
  background: var(--bg-hover);
}

.navItem.active {
  background: var(--accent-color);
  color: white;
}

.navIcon {
  font-size: 16px;
  margin-right: 10px;
  width: 20px;
  text-align: center;
}

.navLabel {
  font-size: 14px;
}

.sidebarFooter {
  padding: 12px 20px;
  border-top: 1px solid var(--border-color);
}

.version {
  font-size: 12px;
  color: var(--text-secondary);
}

/* 右侧内容区 */
.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.contentHeader {
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-primary);
}

.contentHeader h2 {
  font-size: 20px;
  font-weight: 600;
  margin: 0;
  display: flex;
  align-items: center;
  gap: 10px;
}

.headerIcon {
  font-size: 24px;
}

.contentBody {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

/* 通用配置项样式 */
:global(.settingSection) {
  margin-bottom: 32px;
}

:global(.settingSection h3) {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 16px;
}

:global(.settingRow) {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  border-bottom: 1px solid var(--border-light);
}

:global(.settingRow:last-child) {
  border-bottom: none;
}

:global(.settingLabel) {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

:global(.settingLabel span) {
  font-size: 14px;
  font-weight: 500;
}

:global(.settingLabel small) {
  font-size: 12px;
  color: var(--text-secondary);
}

:global(.settingControl) {
  display: flex;
  align-items: center;
  gap: 8px;
}

```

### 8.3 Web 搜索配置页面

```tsx
// src/pages/Settings/WebSearch.tsx

import { Component, For, createSignal, createResource, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import styles from "./WebSearch.module.css";

interface SearchEngine {
  id: string;
  name: string;
  keyword: string;
  url_template: string;
  icon?: string;
  is_builtin: boolean;
}

const WebSearch: Component = () => {
  const [engines, { refetch }] = createResource(async () => {
    const config = await invoke<{ web_search: { engines: SearchEngine[] } }>("get_config");
    return config.web_search.engines;
  });

  const [editingEngine, setEditingEngine] = createSignal<SearchEngine | null>(null);
  const [isAdding, setIsAdding] = createSignal(false);

  const handleSave = async (engine: SearchEngine) => {
    if (isAdding()) {
      await invoke("add_search_engine", { engine });
    } else {
      // 更新引擎 (需要删除旧的再添加)
      await invoke("delete_search_engine", { engineId: engine.id });
      await invoke("add_search_engine", { engine });
    }
    setEditingEngine(null);
    setIsAdding(false);
    refetch();
  };

  const handleDelete = async (engineId: string) => {
    if (confirm("Are you sure you want to delete this search engine?")) {
      await invoke("delete_search_engine", { engineId });
      refetch();
    }
  };

  const startAdding = () => {
    setIsAdding(true);
    setEditingEngine({
      id: crypto.randomUUID(),
      name: "",
      keyword: "",
      url_template: "",
      is_builtin: false,
    });
  };

  return (
    <div class={styles.webSearch}>
      <div class="settingSection">
        <div class={styles.header}>
          <h3>Search Engines</h3>
          <button class={styles.addButton} onClick={startAdding}>
            + Add Engine
          </button>
        </div>

        <p class={styles.hint}>
          Type the keyword followed by your search query. For example: <code>gg hello world</code> will search Google for "hello world".
        </p>

        <div class={styles.engineList}>
          <div class={styles.engineHeader}>
            <span class={styles.colName}>Name</span>
            <span class={styles.colKeyword}>Keyword</span>
            <span class={styles.colUrl}>URL Template</span>
            <span class={styles.colActions}>Actions</span>
          </div>

          <For each={engines()}>
            {(engine) => (
              <div class={styles.engineRow}>
                <div class={styles.colName}>
                  <span class={styles.engineIcon}>{engine.icon || "🔍"}</span>
                  <span>{engine.name}</span>
                  <Show when={engine.is_builtin}>
                    <span class={styles.builtinBadge}>Built-in</span>
                  </Show>
                </div>
                <div class={styles.colKeyword}>
                  <code>{engine.keyword}</code>
                </div>
                <div class={styles.colUrl} title={engine.url_template}>
                  {engine.url_template}
                </div>
                <div class={styles.colActions}>
                  <button
                    class={styles.editBtn}
                    onClick={() => {
                      setIsAdding(false);
                      setEditingEngine(engine);
                    }}
                  >
                    Edit
                  </button>
                  <Show when={!engine.is_builtin}>
                    <button
                      class={styles.deleteBtn}
                      onClick={() => handleDelete(engine.id)}
                    >
                      Delete
                    </button>
                  </Show>
                </div>
              </div>
            )}
          </For>
        </div>
      </div>

      {/* 编辑弹窗 */}
      <Show when={editingEngine()}>
        <EngineEditModal
          engine={editingEngine()!}
          isNew={isAdding()}
          onSave={handleSave}
          onCancel={() => {
            setEditingEngine(null);
            setIsAdding(false);
          }}
        />
      </Show>
    </div>
  );
};

interface EngineEditModalProps {
  engine: SearchEngine;
  isNew: boolean;
  onSave: (engine: SearchEngine) => void;
  onCancel: () => void;
}

const EngineEditModal: Component<EngineEditModalProps> = (props) => {
  const [engine, setEngine] = createSignal({ ...props.engine });
  const [errors, setErrors] = createSignal<Record<string, string>>({});

  const validate = (): boolean => {
    const e = engine();
    const newErrors: Record<string, string> = {};

    if (!e.name.trim()) {
      newErrors.name = "Name is required";
    }
    if (!e.keyword.trim()) {
      newErrors.keyword = "Keyword is required";
    } else if (!/^[a-zA-Z0-9]+$/.test(e.keyword)) {
      newErrors.keyword = "Keyword must be alphanumeric";
    }
    if (!e.url_template.trim()) {
      newErrors.url_template = "URL template is required";
    } else if (!e.url_template.includes("{query}")) {
      newErrors.url_template = "URL template must contain {query}";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: Event) => {
    e.preventDefault();
    if (validate()) {
      props.onSave(engine());
    }
  };

  return (
    <div class={styles.modalOverlay} onClick={props.onCancel}>
      <div class={styles.modal} onClick={(e) => e.stopPropagation()}>
        <h3>{props.isNew ? "Add Search Engine" : "Edit Search Engine"}</h3>

        <form onSubmit={handleSubmit}>
          <div class={styles.formField}>
            <label>Name</label>
            <input
              type="text"
              value={engine().name}
              onInput={(e) => setEngine({ ...engine(), name: e.currentTarget.value })}
              placeholder="e.g., Google"
              disabled={props.engine.is_builtin}
            />
            <Show when={errors().name}>
              <span class={styles.error}>{errors().name}</span>
            </Show>
          </div>

          <div class={styles.formField}>
            <label>Keyword</label>
            <input
              type="text"
              value={engine().keyword}
              onInput={(e) => setEngine({ ...engine(), keyword: e.currentTarget.value })}
              placeholder="e.g., gg"
              disabled={props.engine.is_builtin}
            />
            <small>The prefix you type to trigger this search engine</small>
            <Show when={errors().keyword}>
              <span class={styles.error}>{errors().keyword}</span>
            </Show>
          </div>

          <div class={styles.formField}>
            <label>URL Template</label>
            <input
              type="text"
              value={engine().url_template}
              onInput={(e) => setEngine({ ...engine(), url_template: e.currentTarget.value })}
              placeholder="e.g., <https://www.google.com/search?q={query}>"
            />
            <small>Use <code>{"{query}"}</code> as a placeholder for the search terms</small>
            <Show when={errors().url_template}>
              <span class={styles.error}>{errors().url_template}</span>
            </Show>
          </div>

          <div class={styles.formActions}>
            <button type="button" class={styles.cancelBtn} onClick={props.onCancel}>
              Cancel
            </button>
            <button type="submit" class={styles.saveBtn}>
              {props.isNew ? "Add" : "Save"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default WebSearch;

```

---

## 九、数据库设计

### 9.1 数据库 Schema

```sql
-- src-tauri/src/storage/migrations/v1_initial.sql

-- 文件索引表
CREATE TABLE IF NOT EXISTS file_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    path TEXT NOT NULL UNIQUE,
    extension TEXT,
    size INTEGER NOT NULL DEFAULT 0,
    modified_at INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    is_dir INTEGER NOT NULL DEFAULT 0,
    is_app INTEGER NOT NULL DEFAULT 0,
    use_count INTEGER NOT NULL DEFAULT 0,
    last_accessed INTEGER
);

CREATE INDEX IF NOT EXISTS idx_file_entries_name ON file_entries(name);
CREATE INDEX IF NOT EXISTS idx_file_entries_extension ON file_entries(extension);
CREATE INDEX IF NOT EXISTS idx_file_entries_is_app ON file_entries(is_app);

-- 剪贴板历史表
CREATE TABLE IF NOT EXISTS clipboard_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content_type TEXT NOT NULL,
    content_text TEXT,
    content_html TEXT,
    content_image BLOB,
    content_files TEXT, -- JSON array
    preview TEXT NOT NULL,
    app_name TEXT,
    app_icon TEXT,
    created_at INTEGER NOT NULL,
    is_favorite INTEGER NOT NULL DEFAULT 0,
    is_sensitive INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_clipboard_created_at ON clipboard_history(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_clipboard_is_favorite ON clipboard_history(is_favorite);

-- AI 对话历史表
CREATE TABLE IF NOT EXISTS ai_conversations (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    provider_id TEXT NOT NULL,
    model TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS ai_messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    conversation_id TEXT NOT NULL,
    role TEXT NOT NULL, -- 'user', 'assistant', 'system'
    content TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    tokens_used INTEGER,
    FOREIGN KEY (conversation_id) REFERENCES ai_conversations(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_ai_messages_conversation ON ai_messages(conversation_id);

-- 工作流表
CREATE TABLE IF NOT EXISTS workflows (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    icon TEXT,
    triggers TEXT NOT NULL, -- JSON
    nodes TEXT NOT NULL, -- JSON
    connections TEXT NOT NULL, -- JSON
    variables TEXT, -- JSON
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 插件数据表
CREATE TABLE IF NOT EXISTS plugin_data (
    plugin_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    PRIMARY KEY (plugin_id, key)
);

-- 搜索历史表
CREATE TABLE IF NOT EXISTS search_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    query TEXT NOT NULL,
    result_type TEXT,
    result_id TEXT,
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_search_history_query ON search_history(query);
CREATE INDEX IF NOT EXISTS idx_search_history_created_at ON search_history(created_at DESC);

-- 应用使用统计表
CREATE TABLE IF NOT EXISTS app_usage (
    app_path TEXT PRIMARY KEY,
    launch_count INTEGER NOT NULL DEFAULT 0,
    last_launched INTEGER
);

```

---

## 十、性能优化策略

### 10.1 前端优化

| 策略 | 实现方式 |
| --- | --- |
| **懒加载** | 路由级别代码分割，设置页面按需加载 |
| **虚拟滚动** | 搜索结果、剪贴板历史使用虚拟列表 |
| **防抖节流** | 搜索输入防抖 150ms |
| **缓存** | 搜索结果本地缓存，图标缓存 |
| **最小化重渲染** | SolidJS 细粒度更新 |

### 10.2 后端优化

| 策略 | 实现方式 |
| --- | --- |
| **内存索引** | 文件索引全部加载到内存 |
| **增量更新** | 文件系统监听，增量更新索引 |
| **连接池** | SQLite 连接池复用 |
| **异步 IO** | Tokio 异步运行时 |
| **批量操作** | 数据库批量插入/更新 |
| **延迟加载** | 配置、插件按需加载 |

### 10.3 性能指标目标

| 指标 | 目标值 | 测量方式 |
| --- | --- | --- |
| 冷启动时间 | < 500ms | 从点击图标到窗口显示 |
| 热启动时间 | < 100ms | 快捷键唤起到窗口显示 |
| 搜索响应时间 | < 50ms | 输入到结果显示 |
| 内存占用（空闲） | < 50MB | 后台运行时 |
| 内存占用（活跃） | < 150MB | 正常使用时 |
| CPU 占用（空闲） | < 1% | 后台运行时 |
| 安装包大小 | < 20MB | 压缩后安装包 |

---

## 十一、安全设计

### 11.1 数据安全

| 措施 | 说明 |
| --- | --- |
| **数据库加密** | SQLCipher 加密本地数据库 |
| **凭证存储** | macOS Keychain / Windows Credential Manager |
| **敏感数据过滤** | 剪贴板敏感内容自动识别并标记 |
| **内存安全** | Rust 内存安全保证 |

### 11.2 插件安全

| 措施 | 说明 |
| --- | --- |
| **沙箱隔离** | 插件运行在隔离环境 |
| **权限声明** | 插件必须声明所需权限 |
| **用户确认** | 敏感权限需用户确认 |
| **代码签名** | 官方市场插件需签名验证 |

---

## 十二、构建与发布

### 12.1 构建配置

```json
// src-tauri/tauri.conf.json
{
  "$schema": "../node_modules/@tauri-apps/cli/schema.json",
  "build": {
    "beforeBuildCommand": "pnpm build",
    "beforeDevCommand": "pnpm dev",
    "devPath": "<http://localhost:5173>",
    "distDir": "../dist"
  },
  "package": {
    "productName": "OmniBox",
    "version": "1.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": true
      },
      "dialog": {
        "all": true
      },
      "fs": {
        "all": true,
        "scope": ["$HOME/**", "$APPDATA/**", "$RESOURCE/**"]
      },
      "globalShortcut": {
        "all": true
      },
      "notification": {
        "all": true
      },
      "clipboard": {
        "all": true
      },
      "window": {
        "all": true
      },
      "path": {
        "all": true
      }
    },
    "bundle": {
      "active": true,
      "category": "Productivity",
      "copyright": "© 2025 OmniBox",
      "identifier": "app.omnibox",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "macOS": {
        "entitlements": "entitlements.plist",
        "minimumSystemVersion": "11.0"
      },
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": ""
      }
    },
    "security": {
      "csp": "default-src 'self'; img-src 'self' data: https:; style-src 'self' 'unsafe-inline'"
    },
    "systemTray": {
      "iconPath": "icons/tray.png",
      "iconAsTemplate": true
    },
    "windows": [
      {
        "title": "OmniBox",
        "label": "main",
        "width": 680,
        "height": 500,
        "resizable": false,
        "decorations": false,
        "transparent": true,
        "alwaysOnTop": true,
        "skipTaskbar": true,
        "visible": false,
        "center": true
      }
    ]
  }
}

```

### 12.2 GitHub Actions CI/CD

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        include:
          - platform: macos-latest
            target: aarch64-apple-darwin
          - platform: macos-latest
            target: x86_64-apple-darwin
          - platform: windows-latest
            target: x86_64-pc-windows-msvc

    runs-on: ${{ matrix.platform }}

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Setup pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install dependencies
        run: pnpm install

      - name: Build
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
          APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
          APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}
          APPLE_ID: ${{ secrets.APPLE_ID }}
          APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
          APPLE_TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}
        with:
          tagName: ${{ github.ref_name }}
          releaseName: 'OmniBox ${{ github.ref_name }}'
          releaseBody: 'See the assets to download and install this version.'
          releaseDraft: true
          prerelease: false
          args: --target ${{ matrix.target }}

```

---

本技术架构文档覆盖了 OmniBox v2.0 需求文档中定义的所有功能模块，包括：

1. **核心搜索** - 输入解析、文件索引、Web 搜索
2. **剪贴板管理** - 独立窗口、历史记录、快捷键唤起
3. **截图系统** - 区域/窗口截图、标注编辑、贴图、取色器
4. **AI 助手** - 多服务商支持、流式响应、对话管理
5. **工作流引擎** - 可视化编辑、节点执行、触发器
6. **插件系统** - 加载、沙箱、市场
7. **配置中心** - Alfred 5 风格布局、完整配置项