# 🚀 UniTools

<div align="center">

**多合一 Tauri 2.0 桌面效率工具箱**

基于 Rust 与 Vue 3 构建，集启动器、剪贴板历史、截图贴图、文件搜索、工作流与插件生态于一体。

[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?logo=tauri)](https://v2.tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)](https://www.rust-lang.org/)
[![Vue](https://img.shields.io/badge/Vue-3-42b883?logo=vue.js)](https://vuejs.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

[功能亮点](#-功能亮点) • [技术架构](#-技术架构) • [快速开始](#-快速开始) • [开发工作流](#-开发工作流) • [路线图](#-路线图) • [贡献指南](#-贡献指南)

</div>

---

## 📖 项目简介

UniTools 是一款面向 Windows 与 macOS 的桌面效率工具套件。项目旨在为重度知识工作者提供统一入口，以轻量级的 Tauri 2.0 容器串联高性能的 Rust 后端与现代 Vue 3 前端，让常用效率工作流在单一应用中无缝衔接。

---

## ✨ 功能亮点

- **智能启动器**：关键字匹配、拼音首字母、内置计算器与翻译器、快速打开应用与命令。
- **剪贴板历史**：自动捕获文本、图片、文件条目，支持搜索、标签、固定与快捷复制。
- **截图与贴图**：内置示例截图管线，支持贴图预览、备注、固定与管理。
- **文件搜索**：针对工作区的索引器提供毫秒级搜索结果、文件元信息与索引重建。
- **工作流自动化**：拖拽式节点编排、丰富触发器、执行日志与预设工作流示例。
- **插件生态**：插件中心浏览、安装与管理，开放 API 支持社区扩展。
- **全局热键与多窗口**：系统托盘常驻，核心模块可通过热键与独立窗口快速唤起。

---

## 🛠️ 技术架构

### 技术栈

```
Frontend:  Vue 3 + TypeScript + Vite
UI:        CSS 变量 + 自定义组件（规划引入 shadcn-vue）
State:     Pinia + @tanstack/vue-query
Backend:   Rust + Tauri 2.0
Database:  SQLite（剪贴板、配置、插件数据）
Indexing:  WalkDir-based 索引器
```

### 模块划分

- **前端窗口**：`src/` 下的 Vue 组件、features 切片、Pinia stores 与多入口窗口。
- **系统桥接层**：Tauri 插件提供全局热键、文件访问、剪贴板、通知等能力。
- **Rust 后端**：`src-tauri/src/` 中的 `clipboard.rs`、`screenshot.rs`、`search.rs`、`workflow.rs`、`plugin.rs` 等模块处理高权限逻辑。
- **配置与文档**：`docs/ai-context/` 提供多智能体协作指南，`claude.md` 约定编码规范。

---

## 🚀 快速开始

### 系统要求

- Windows 10 1809+ / Windows 11 (x64) 或 macOS 13 Ventura+
- Rust 1.75+、Node.js 18+、pnpm（或 npm / yarn）
- 对应平台的构建依赖：Windows 需安装 Visual Studio Build Tools，macOS 需安装 Xcode Command Line Tools

### 安装步骤

```bash
# 1. 安装 Rust（https://rustup.rs/）
# 2. 安装 Tauri CLI
cargo install tauri-cli --version "^2.0.0"

# 3. 克隆并进入仓库
git clone https://github.com/notruth/ntruth-tools.git
cd ntruth-tools

# 4. 安装前端依赖
pnpm install

# 5. 启动开发环境
pnpm tauri dev
```

首次启动会编译 Rust 侧代码，请预留 5-10 分钟。

### 构建发行包

```bash
pnpm tauri build
```

构建产物会输出在 `src-tauri/target/release/bundle/`。

---

## 💻 开发工作流

- `pnpm tauri dev`：启动带有热重载的 Tauri 开发环境。
- `pnpm tauri build`：构建生产安装包。
- `pnpm test` / `cargo test`：运行前端与 Rust 测试。
- `pnpm lint` / `cargo clippy`：静态检查，提交前需确保无警告。
- `pnpm format` / `cargo fmt`：格式化代码保持统一规范。

启用详细日志可执行 `RUST_LOG=debug pnpm tauri dev`，调试 Rust 时可配合 `rust-gdb` / `lldb`。

---

## 🧱 项目结构

```
ntruth-tools/
├── src/
│   ├── main.ts                 # 前端入口
│   ├── launcher/               # 启动器窗口入口
│   ├── components/             # 共享 UI 组件
│   ├── features/               # 功能切片（launcher、clipboard、screenshot、search、workflow、plugins）
│   ├── stores/                 # Pinia stores 与对应测试
│   └── composables/            # 共享逻辑
├── src-tauri/
│   ├── src/main.rs             # Tauri 主入口与窗口管理
│   ├── src/clipboard.rs        # 剪贴板服务
│   ├── src/screenshot.rs       # 截图示例与贴图
│   ├── src/search.rs           # 文件索引与查询
│   ├── src/workflow.rs         # 工作流调度
│   ├── src/plugin.rs           # 插件注册与生命周期
│   └── Cargo.toml
├── docs/ai-context/            # AI 协作上下文
├── claude.md                   # 编码规范与协作原则
└── readme.md
```

---

## 🗺️ 路线图

- [x] 阶段 1：核心框架（多窗口、托盘、主题、热键）
- [x] 阶段 2：快速启动器（搜索、排序、内置工具）
- [x] 阶段 3：剪贴板管理（历史、标签、固定、批量操作）
- [x] 阶段 4：截图与贴图（示例引擎、贴图管理、预览）
- [x] 阶段 5：文件搜索（索引器、即时搜索、元信息展示）
- [x] 阶段 6：工作流系统（节点编排、触发器、执行日志、预设流程）
- [x] 阶段 7：插件生态（市场、安装/卸载、示例插件、开放 API）
- [ ] 云端同步、跨设备共享
- [ ] AI 增强功能（智能补全、OCR、语义搜索）

---

## 🤝 贡献指南

- 参阅 `claude.md` 与 `docs/ai-context/` 了解项目规范。
- Fork 仓库并基于 `main` 创建特性分支：`git checkout -b feat/my-feature`。
- 提交前运行 `pnpm lint`, `pnpm test`, `cargo clippy`, `cargo test`。
- 使用规范化的 Commit 信息（例如 `feat: add workflow automation nodes`）。
- 在 Pull Request 中描述问题背景、关键改动与测试结果，可附带 UI 截图或 GIF。

Bug 反馈与功能建议请通过 [Issues](../../issues) 提交，讨论与问答使用 [Discussions](../../discussions)。

---

## 📄 许可证

本项目基于 [MIT License](LICENSE) 开源，可自由使用与二次开发。

---

## 🙏 致谢

项目灵感与实现细节参考并感谢以下社区与工具：

- [Tauri](https://tauri.app/) 提供跨平台应用框架
- [Ditto](https://ditto-cp.sourceforge.io/)、[Snipaste](https://www.snipaste.com/)、[Everything](https://www.voidtools.com/)、[uTools](https://u.tools/)、[Alfred](https://www.alfredapp.com/)
- 社区贡献的 Tauri 插件与 Rust/Vue 生态

---

<div align="center">

如果 UniTools 对你有帮助，欢迎 Star ⭐️ 支持与分享。

</div>
