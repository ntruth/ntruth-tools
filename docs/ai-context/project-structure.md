# UniTools 项目结构文档

## 项目概览
- **定位**：基于 Tauri 2.0 的 Windows 与 macOS 桌面效率工具，整合启动器、剪贴板历史、截图贴图和文件搜索能力。
- **目标用户**：需要一站式效率工具套件的 Windows / macOS 高效工作人群。
- **运行形态**：前端采用 Vue 3 + TypeScript 构建的多窗口界面，后端依托 Rust 与 Tauri 插件层处理系统能力。

## 当前阶段与路线
- **阶段 1（核心框架）**：已完成，包括 Tauri 初始化、多窗口架构、系统托盘与全局热键、主题系统。
- **阶段 2（快速启动器）**：已完成，提供快速搜索 UI、应用索引、计算器/翻译内置工具与自定义排序算法。
- **阶段 3（剪贴板管理）**：已完成，具备历史记录、搜索过滤、固定和批量清理功能。
- **阶段 4（截图与贴图）**：已完成，支持截图示例管线、贴图预览与固定管理。
- **阶段 5（文件搜索）**：已完成，提供项目级索引、评分排序与索引刷新入口。
- **阶段 6（工作流系统）**：已完成，可视化节点编排、触发器配置与执行日志。
- **阶段 7（插件生态）**：已完成，含插件市场、安装卸载流程与示例插件。

## 技术栈概览
```
Frontend:  Vue 3 + TypeScript + Vite
UI:        自定义 CSS 变量（计划引入 shadcn-vue 组件）
State:     Pinia + Vue Query
Backend:   Rust (Tauri 2.0)
Database:  SQLite（剪贴板历史、配置、插件数据）
Indexing:  WalkDir-based workspace indexer
```

## 架构组成
- **桌面前端**：基于 Vue 3 组件体系构建，负责 UI、状态管理、插件生态前端面板以及工作流可视化。
- **系统桥接层**：Tauri 2.0 提供窗口管理、全局热键、文件系统访问等桌面能力，通过官方插件扩展。
- **Rust 后端服务**：处理剪贴板监听、截图 API、文件索引构建、插件管理与工作流执行等高权限逻辑。
- **持久化层**：SQLite 负责记录剪贴板历史、配置项与插件数据；自研索引器维护文件搜索用的元数据。

## 目录结构
```
unitools/
├── src/
│   ├── App.vue                  # 主窗口界面
│   ├── main.ts                  # 前端入口，注册 Pinia/Vue Query
│   ├── styles.css               # 主题样式变量
│   ├── components/
│   │   └── ThemeToggle.vue      # 主题切换组件
│   ├── features/
│   │   ├── clipboard/           # 阶段 3 剪贴板面板
│   │   │   ├── ClipboardPanel.vue
│   │   │   └── types.ts
│   │   ├── launcher/            # 阶段 2 启动器模块
│   │   │   ├── LauncherApp.vue
│   │   │   ├── LauncherPreview.vue
│   │   │   ├── search-data.ts
│   │   │   ├── types.ts
│   │   │   ├── useSearch.ts
│   │   │   └── useSearch.spec.ts
│   │   ├── screenshot/          # 阶段 4 截图贴图
│   │   │   └── ScreenshotDock.vue
│   │   ├── search/              # 阶段 5 文件搜索
│   │   │   └── FileSearchPanel.vue
│   │   ├── workflow/            # 阶段 6 工作流系统
│   │   │   ├── WorkflowBuilder.vue
│   │   │   └── types.ts
│   │   └── plugins/             # 阶段 7 插件生态
│   │       ├── PluginCenter.vue
│   │       └── types.ts
│   ├── launcher/
│   │   └── main.ts              # 启动器窗口入口
│   ├── stores/
│   │   ├── clipboard.ts         # 剪贴板状态管理
│   │   ├── screenshot.ts        # 截图状态管理
│   │   ├── search.ts            # 文件搜索状态
│   │   ├── workflow.ts          # 工作流状态管理
│   │   ├── plugins.ts           # 插件状态管理
│   │   ├── clipboard.spec.ts    # 剪贴板 store 测试
│   │   ├── workflow.spec.ts     # 工作流 store 测试
│   │   ├── plugins.spec.ts      # 插件 store 测试
│   │   └── theme.ts             # 主题状态管理
│   └── env.d.ts
├── src-tauri/
│   ├── src/main.rs              # Rust 侧窗口、托盘、指令、热键
│   ├── src/clipboard.rs         # 剪贴板服务
│   ├── src/screenshot.rs        # 截图示例管线
│   ├── src/search.rs            # 文件索引与查询
│   ├── src/workflow.rs          # 工作流调度
│   ├── src/plugin.rs            # 插件注册与安装
│   ├── Cargo.toml               # Rust 依赖定义
│   ├── tauri.conf.json          # Tauri 配置（窗口、多页面构建）
│   └── build.rs
├── index.html                   # 主窗口 HTML 入口
├── launcher.html                # 启动器窗口 HTML 入口
├── package.json                 # 前端依赖与脚本
├── vite.config.ts               # Vite/多入口配置
├── vitest.setup.ts              # 测试环境配置
└── docs/ai-context/…            # AI 协作上下文文档
```

## 核心依赖
- **前端**：Vue 3、Pinia、@tanstack/vue-query、vite、vitest。
- **Tauri 插件**：`tauri-plugin-clipboard-manager`、`tauri-plugin-global-shortcut`、`tauri-plugin-shell`、`tauri-plugin-notification`、`tauri-plugin-dialog`、`tauri-plugin-sql`、`tauri-plugin-fs`。
- **Rust Crates**：`tauri`、`serde`、`serde_json`、`anyhow`、`once_cell`、`parking_lot`、`image`、`chrono`、`walkdir`、`base64`。

## 常用开发命令
```
pnpm tauri dev      # 启动开发环境
pnpm tauri build    # 构建生产包
pnpm test           # 前端测试
cargo test          # Rust 测试
pnpm lint           # 前端 Lint
cargo clippy        # Rust Lint
pnpm format         # 前端格式化
cargo fmt           # Rust 格式化
```

## AI 协作提醒
- 在开展任何任务前，请优先阅读本文件与 `claude.md`，同步掌握技术栈、目录规划与协作规范。
- 遵循 `claude.md` 中列出的编码原则（KISS、YAGNI、DRY、SOLID 等）、类型提示要求与安全策略。
- 提交改动前务必运行相关测试与 Lint，确保文档与代码保持同步。

本文件与 `claude.md` 一起为多智能体提供统一的上下文，请保持同步更新。
