# 帮我制作成项目的readme文档

```markdown
<div align="center">

# 🚀 UniTools

**新一代全能生产力工具套件**

基于 Tauri 2.0 构建的 Windows 与 macOS 桌面应用  
融合剪贴板管理、智能启动器、截图贴图、文件搜索于一体

[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?logo=tauri)](https://v2.tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)](https://www.rust-lang.org/)
[![Vue](https://img.shields.io/badge/Vue-3-42b883?logo=vue.js)](https://vuejs.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

[功能特性](#-功能特性) • [快速开始](#-快速开始) • [开发指南](#-开发指南) • [路线图](#-开发路线图) • [贡献](#-贡献)

</div>

---

## 📖 项目简介

UniTools 是一款集成多种效率工具的跨平台（Windows 与 macOS）桌面应用，旨在替代并超越现有的生产力工具生态。它将 **Ditto（剪贴板管理）**、**Snipaste（截图贴图）**、**Everything（文件搜索）**、**uTools/Alfred（启动器）** 的核心功能融合到单一应用中，提供无缝的使用体验。

### 为什么选择 UniTools？

- **🎯 一站式解决方案**：无需安装多个工具，一个应用满足所有需求
- **⚡ 极致性能**：基于 Rust 和 Tauri 2.0，内存占用低，响应速度快
- **🎨 现代化设计**：简约美观的界面，支持深色/浅色主题
- **🔌 可扩展性**：插件系统支持自定义功能扩展
- **🔒 隐私优先**：本地数据加密存储，无需联网即可使用核心功能

---

## ✨ 功能特性

### 1️⃣ 智能启动器（uTools + Alfred + Everything）

<table>
<tr>
<td width="50%">

**快速搜索**
- 文件、应用、命令统一搜索
- 拼音首字母快速匹配
- 浏览器书签和历史记录
- 剪贴板历史快速调用
- 智能排序（基于使用频率）

</td>
<td width="50%">

**内置工具集**
- 🧮 计算器（单位转换、货币换算）
- 🌐 翻译工具（多语言支持）
- 🎨 颜色拾取器
- 🔐 编码转换（Base64/URL/Unicode）
- 🔒 哈希计算（MD5/SHA256）
- ⏰ 时间戳转换

</td>
</tr>
</table>

**工作流自动化**（Alfred 5 灵感）
- 可视化拖拽式编程界面
- 预设自动化任务（批量重命名、图片处理、文本转换）
- 支持 JavaScript/Python/PowerShell 脚本
- 热键、关键词、定时器触发

**插件生态系统**（uTools 模式）
- 插件市场浏览与安装
- 开放的插件 API
- 热重载开发模式
- 社区插件支持（OCR、TODO、快递查询等）

---

### 2️⃣ 剪贴板历史管理（Ditto 功能）

- ✅ **文本捕获**：读取系统剪贴板并保存为历史记录
- ✅ **搜索过滤**：按关键词、标签与类型筛选，固定重要内容
- ✅ **快捷操作**：双击复制、批量清除未固定、支持热键触发
- ✅ **标签管理**：为片段打标签，快速聚合常用内容
- ⏳ **扩展计划**：图片/文件支持、正则搜索、云端同步

**全局热键**：`Ctrl+Shift+V`（Windows）/ `Command+Shift+V`（macOS） 唤起剪贴板面板

---

### 3️⃣ 截图与贴图工具（Snipaste 功能）

- ✅ **截图示例引擎**：内置示例渲染管线，模拟截图并保存为贴图
- ✅ **贴图管理**：备注、固定、删除等常见操作一站式完成
- ✅ **预览面板**：快速浏览所有贴图，便于对比参考
- ⏳ **扩展计划**：系统级截图、标注工具、浮动贴图窗口、颜色拾取器

**全局热键**：`F1` 开始截图（规划中），`F3` 贴图模式（规划中）

---

### 4️⃣ 文件搜索引擎（Everything 核心）

- ✅ **工作区索引**：预扫描 `src/`、`docs/`、`src-tauri/` 等目录
- ✅ **即时搜索**：输入关键字、扩展名即可获得实时评分结果
- ✅ **信息展示**：显示文件大小、更新时间、类型等关键信息
- ✅ **索引重建**：一键刷新索引，保持结果最新
- ⏳ **扩展计划**：系统盘级索引、正则语法、高级过滤器、热键整合

**全局热键**：`Alt+Space`（Windows）/ `Option+Space`（macOS） 可在启动器中联动文件搜索（规划中）

### 5️⃣ 工作流自动化系统（Workflow）

- ✅ **可视化编排**：拖拽式节点面板，支持读取剪贴板、文件写入、通知等节点
- ✅ **触发器配置**：手动、快捷键、剪贴板、定时等多种触发方式
- ✅ **执行日志**：运行记录实时显示，便于调试与迭代
- ✅ **预设场景**：内置“剪贴板归档”“截图归档”等示例工作流
- ⏳ **扩展计划**：图形连线、条件分支、AI 节点、跨设备同步

### 6️⃣ 插件生态（Plugin Marketplace）

- ✅ **插件市场**：浏览官方示例插件（OCR、翻译增强、任务同步等）
- ✅ **插件管理**：安装、卸载、查看版本信息及作者
- ✅ **插件 API**：Tauri 层提供插件清单与注册接口，方便扩展节点/面板
- ✅ **开发者工具**：界面内提示仓库地址与调试入口，为二次开发铺路
- ⏳ **扩展计划**：插件评分系统、自动更新、插件沙箱、社区投稿流程

---

## 🛠️ 技术架构

### 技术栈

```

Frontend:  Vue 3 + TypeScript + Vite
UI:        自定义设计系统（CSS 变量，计划接入 shadcn-vue）
State:     Pinia + Vue Query
Backend:   Rust (Tauri 2.0)
Database:  SQLite (剪贴板历史、配置、插件数据)
Indexing:  WalkDir-based workspace indexer (跨平台)

```

### 核心依赖

**Tauri 官方插件**
- `tauri-plugin-clipboard-manager` - 剪贴板扩展 API
- `tauri-plugin-global-shortcut` - 全局热键
- `tauri-plugin-fs` - 文件系统访问
- `tauri-plugin-sql` - SQLite 数据库
- `tauri-plugin-dialog` - 系统对话框
- `tauri-plugin-shell` - 执行外部命令
- `tauri-plugin-notification` - 系统通知

**Rust Crates**
- `anyhow` / `serde` / `serde_json` - 命令执行与序列化
- `once_cell` + `parking_lot` - 全局状态与锁管理
- `chrono` - 时间戳与调度
- `walkdir` - 文件系统索引
- `image` / `base64` - 截图示例与贴图编码

**前端库**
- `vue-virtual-scroller` - 虚拟滚动列表
- `vue-flow` - 工作流可视化编程
- `monaco-editor` - 代码编辑器
- `fabric.js` / `konva` - Canvas 图像编辑

---

## 🚀 快速开始

### 系统要求

- **操作系统**：Windows 10 (1809+) / Windows 11 x64，或 macOS 13 Ventura 及以上（Apple Silicon / Intel）
- **开发环境**：
  - Rust 1.75+
  - Node.js 18+ LTS
  - pnpm / npm / yarn
- **额外依赖**：
  - Windows：安装 Visual Studio Build Tools（含 C++ 桌面开发）
  - macOS：安装 Xcode Command Line Tools（`xcode-select --install`）

### 安装依赖

#### 1. 安装 Rust

```


# 访问 https://rustup.rs/ 下载安装
# Windows 可使用：
# winget install --id=Rustlang.Rustup -e
# macOS 可执行：
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

#### 2. 安装 Tauri CLI

```

cargo install tauri-cli --version "^2.0.0"

```

#### 3. 克隆项目

```

git clone https://github.com/yourusername/unitools.git
cd unitools

```

#### 4. 安装前端依赖

```

pnpm install

# 或 npm install / yarn install

```

### 运行开发服务器

```

pnpm tauri dev

# 或 npm run tauri dev / yarn tauri dev

```

首次运行将编译 Rust 代码，可能需要 5-10 分钟。后续启动将会更快。

### 构建生产版本

```

pnpm tauri build

```

构建产物位于 `src-tauri/target/release/bundle/` 目录（Windows 输出 `.msi/.exe`，macOS 输出 `.dmg`）。

---

## 📦 安装使用

### 从 Release 安装（推荐）

1. 前往 [Releases](https://github.com/yourusername/unitools/releases) 页面
2. 下载最新版本的 `.msi` / `.exe`（Windows）或 `.dmg`（macOS）安装包
3. 运行安装程序，按照提示完成安装
4. 首次启动会显示设置向导

### 全局热键

| 功能 | 默认热键 | 可自定义 |
|------|---------|---------|
| 快速启动器 | `Alt+Space` (Windows) / `Option+Space` (macOS) | ✅ |
| 剪贴板历史 | `Ctrl+Shift+V` (Windows) / `Command+Shift+V` (macOS) | ✅ |
| 开始截图 | `F1`（示例） / `Command+Shift+5`（macOS 系统默认） | ✅ |
| 贴图模式 | `F3`（跨平台增强中） | ✅ |
| OCR 识别 | `Ctrl+Shift+X` (Windows) / `Command+Shift+X` (macOS) | ✅ |
| 翻译选中文本 | `Ctrl+Shift+T` (Windows) / `Command+Shift+T` (macOS) | ✅ |

---

## 🧑‍💻 开发指南

### 项目结构

```

unitools/
├── src/                    \# Vue 前端代码
│   ├── components/         \# UI 组件
│   ├── features/           \# 功能模块（启动器、剪贴板、截图、搜索、工作流、插件）
│   │   ├── launcher/
│   │   ├── clipboard/
│   │   ├── screenshot/
│   │   ├── search/
│   │   ├── workflow/
│   │   └── plugins/
│   ├── stores/             \# Pinia 状态管理（theme、clipboard、workflow、plugins 等）
│   └── utils/              \# 工具函数
├── src-tauri/              \# Rust 后端代码
│   ├── src/
│   │   ├── main.rs         \# 入口文件
│   │   ├── clipboard.rs    \# 剪贴板服务
│   │   ├── screenshot.rs   \# 截图示例管线
│   │   ├── search.rs       \# 文件索引与查询
│   │   ├── workflow.rs     \# 工作流调度
│   │   └── plugin.rs       \# 插件注册管理
│   ├── icons/              \# 应用图标
│   └── Cargo.toml          \# Rust 依赖
├── public/                 \# 静态资源
├── package.json            \# 前端依赖
└── README.md

```

### 开发命令

```


# 启动开发服务器（热重载）

pnpm tauri dev

# 构建生产版本

pnpm tauri build

# 运行测试

pnpm test                   \# 前端测试
cargo test                  \# Rust 测试

# 代码检查

pnpm lint                   \# ESLint
cargo clippy                \# Rust Clippy

# 格式化代码

pnpm format                 \# Prettier
cargo fmt                   \# Rust fmt

```

### 调试技巧

**前端调试**
- 开发模式下按 `F12` 打开 Chrome DevTools
- 使用 Vue Devtools 扩展

**Rust 调试**
```


# 启用详细日志

RUST_LOG=debug pnpm tauri dev

# 使用 rust-gdb 或 lldb

rust-gdb target/debug/unitools

```

**性能分析**
```


# 使用 cargo flamegraph

cargo install flamegraph
cargo flamegraph

```

---

## 🗺️ 开发路线图

### ✅ 阶段 1：核心框架（已完成）
- [x] Tauri 项目初始化
- [x] 多窗口架构
- [x] 系统托盘和全局热键
- [x] 主题系统

### ✅ 阶段 2：快速启动器（已完成）
- [x] 快速搜索框 UI
- [x] 应用程序索引
- [x] 内置工具（计算器、翻译）
- [x] 搜索结果排序算法

### ✅ 阶段 3：剪贴板管理（已完成）
- [x] 剪贴板监听与历史记录
- [x] 历史列表 UI（搜索、过滤、标签）
- [x] 固定、复制、批量清理
- [x] 热键触发与持久化

### ✅ 阶段 4：截图与贴图（已完成）
- [x] 截图引擎示例实现
- [x] 贴图备注、固定与管理
- [x] 贴图列表实时预览
- [x] 删除及整理入口

### ✅ 阶段 5：文件搜索（已完成）
- [x] 索引项目文档与源码
- [x] 搜索结果 UI 与评分
- [x] 文件类型与大小展示
- [x] 索引重建与快速查询

### 📋 阶段 6：工作流系统（计划中）
- [ ] 可视化编程界面
- [ ] 节点执行引擎
- [ ] 预设任务
- [ ] 触发器系统

### 📋 阶段 7：插件生态（计划中）
- [ ] 插件 API
- [ ] 插件市场
- [ ] 开发工具
- [ ] 示例插件

### 🎯 未来计划
- [ ] 跨平台支持（macOS、Linux）
- [ ] 移动端支持（iOS、Android）
- [ ] 云端同步服务
- [ ] AI 功能集成（图像识别、智能搜索）

---

## 🤝 贡献

我们欢迎所有形式的贡献！无论是报告 Bug、提出新功能建议，还是提交代码。

### 如何贡献

1. **Fork 本仓库**
2. **创建功能分支** (`git checkout -b feature/AmazingFeature`)
3. **提交更改** (`git commit -m 'Add some AmazingFeature'`)
4. **推送到分支** (`git push origin feature/AmazingFeature`)
5. **提交 Pull Request**

### 开发规范

- 遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- 遵循 [Airbnb JavaScript Style Guide](https://github.com/airbnb/javascript)
- 提交前运行 `pnpm lint` 和 `cargo clippy`
- 添加单元测试覆盖新功能
- 更新相关文档

### 报告 Bug

使用 [Issue 模板](https://github.com/yourusername/unitools/issues/new/choose) 提交 Bug 报告，请包含：
- 详细的问题描述
- 复现步骤
- 预期行为
- 实际行为
- 系统环境（操作系统版本、应用版本）
- 截图或日志（如有）

---

## 📄 许可证

本项目采用 [MIT License](LICENSE) 开源协议。

---

## 🙏 致谢

感谢以下开源项目提供的灵感和参考：

- [Ditto](https://ditto-cp.sourceforge.io/) - 剪贴板管理
- [Snipaste](https://www.snipaste.com/) - 截图贴图
- [Everything](https://www.voidtools.com/) - 文件搜索
- [uTools](https://u.tools/) - 插件化启动器
- [Alfred](https://www.alfredapp.com/) - macOS 启动器
- [Tauri](https://tauri.app/) - 跨平台应用框架

---

## 📞 联系方式

- **项目主页**：[https://github.com/yourusername/unitools](https://github.com/yourusername/unitools)
- **问题反馈**：[GitHub Issues](https://github.com/yourusername/unitools/issues)
- **讨论区**：[GitHub Discussions](https://github.com/yourusername/unitools/discussions)
- **电子邮件**：your.email@example.com

---

<div align="center">

**如果这个项目对你有帮助，请给我们一个 ⭐️ Star！**

Made with ❤️ by UniTools Team

</div>
```


***

这份 README 文档包含了：

1. **专业的项目展示**：徽章、清晰的结构、视觉吸引力
2. **完整的功能介绍**：四大核心功能模块的详细说明
3. **技术架构**：技术栈、依赖列表、项目结构
4. **快速开始指南**：从安装到运行的完整步骤
5. **开发指南**：项目结构、开发命令、调试技巧
6. **开发路线图**：清晰的阶段划分和进度展示
7. **贡献指南**：鼓励社区参与
8. **致谢和联系方式**：感谢相关项目，提供反馈渠道

你可以直接将这份文档保存为 `README.md` 放在项目根目录，并根据实际项目进展更新路线图中的完成状态。如果需要添加项目截图或演示视频，可以在"功能特性"章节中插入图片链接。
<span style="display:none">[^1][^10][^2][^3][^4][^5][^6][^7][^8][^9]</span>

<div align="center">⁂</div>

[^1]: https://github.com/agmmnn/tauri-app-template

[^2]: https://v2.tauri.app/start/create-project/

[^3]: https://github.com/tauri-apps/create-tauri-app

[^4]: https://www.reddit.com/r/rust/comments/1nvvoee/built_a_desktop_app_with_tauri_20_impressions/

[^5]: https://v2.tauri.app/blog/tauri-20/

[^6]: https://www.gollahalli.com/blog/tauri-2-with-flutter-template/

[^7]: https://v2.tauri.app/distribute/pipelines/github/

[^8]: https://crates.io/crates/create-tauri-app/2.0.2

[^9]: https://chat.gitgud.io/yyds/tauri/-/blob/tauri-v2.0.0-alpha.19/examples/README.md

[^10]: https://www.reddit.com/r/tauri/comments/1gdal5d/how_to_make_a_context_menu_in_v2/
