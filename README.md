# OmniBox

<p align="center">
  <strong>跨平台高效启动器与生产力工具</strong>
</p>

<p align="center">
  <a href="#功能特性">功能特性</a> •
  <a href="#快速开始">快速开始</a> •
  <a href="#技术栈">技术栈</a> •
  <a href="#开发指南">开发指南</a> •
  <a href="#路线图">路线图</a>
</p>

---

## 简介

OmniBox 是一款跨平台（macOS/Windows）的高效启动器与生产力工具，集成快速搜索、剪贴板管理、截图标注、AI 助手、工作流自动化及插件扩展能力，旨在成为用户桌面效率的统一入口。

### 核心价值

| 价值点 | 描述 |
|--------|------|
| **快速访问** | 全局快捷键一键唤起，毫秒级响应 |
| **统一入口** | 搜索、AI、剪贴板、截图、工作流统一管理 |
| **本地优先** | 数据本地存储，隐私安全 |
| **高度可定制** | 插件系统 + 工作流 + 自定义配置 |

## 功能特性

### 🔍 核心搜索
- 文件搜索 - 本地文件名模糊匹配
- 应用搜索 - 已安装应用快速启动
- 计算器 - 支持基础运算、函数、单位换算
- Web 搜索 - 内置 Google、Baidu、GitHub 等多种搜索引擎
- AI 问答 - 调用配置的 AI 服务

### 📋 剪贴板管理
- 支持文本、富文本、图片、文件、颜色值
- 智能预览与快速粘贴
- 收藏功能与来源追踪
- 敏感内容自动过滤

### 📸 截图与贴图
- 区域截图、窗口截图、全屏截图
- 丰富的标注工具（矩形、箭头、文字、马赛克等）
- 贴图功能 - 悬浮显示、透明度调节
- 取色器

### 🤖 AI 助手
- 支持 OpenAI、Anthropic、Google、Ollama 等服务
- 多模态支持
- 预设 Prompt 模板
- 流式响应

### ⚡ 工作流自动化
- 多种触发方式（关键词、快捷键、定时等）
- 可视化节点编辑器
- 丰富的节点类型（输入、逻辑、转换、动作、AI、脚本）

### 🔌 插件系统
- 多种插件类型（SearchProvider、ActionHandler、WorkflowNode 等）
- 插件市场
- 权限控制

## 快速开始

### 系统要求

| 平台 | 最低版本 |
|------|----------|
| macOS | 11.0 (Big Sur) |
| Windows | 10 (1903+) |

### 安装

```bash
# 克隆仓库
git clone https://github.com/ntruth/ntruth-tools.git
cd ntruth-tools

# 安装依赖
pnpm install

# 开发模式运行
pnpm tauri dev

# 构建生产版本
pnpm tauri build
```

### 全局快捷键

| 功能 | macOS | Windows |
|------|-------|---------|
| 主搜索窗口 | `Cmd+Space` | `Alt+Space` |
| 剪贴板历史 | `Cmd+Shift+V` | `Ctrl+Shift+V` |
| 区域截图 | `Cmd+Shift+4` | `Ctrl+Shift+S` |
| AI 对话 | `Cmd+Shift+A` | `Ctrl+Shift+A` |

## 技术栈

| 层级 | 技术选型 | 说明 |
|------|----------|------|
| **前端框架** | SolidJS + TypeScript | 高性能响应式 UI |
| **样式方案** | TailwindCSS | 原子化 CSS |
| **构建工具** | Vite | 快速 HMR |
| **后端框架** | Tauri 2.0 | Rust 跨平台桌面框架 |
| **后端语言** | Rust | 系统级性能 |
| **数据库** | SQLite + SQLCipher | 本地加密存储 |
| **异步运行时** | Tokio | 异步 I/O |

## 项目结构

```
omnibox/
├── src/                          # 前端源码 (SolidJS)
│   ├── components/               # 通用组件
│   ├── pages/                    # 页面组件
│   ├── stores/                   # 状态管理
│   ├── services/                 # 前端服务层
│   └── types/                    # TypeScript 类型
│
├── src-tauri/                    # Rust 后端
│   └── src/
│       ├── app/                  # 应用核心
│       ├── commands/             # Tauri Commands
│       ├── core/                 # 核心业务模块
│       ├── platform/             # 平台特定实现
│       └── storage/              # 数据存储
│
└── tests/                        # 测试
```

## 开发指南

### 前置要求

- Node.js 18+
- pnpm 8+
- Rust 1.75+
- 平台特定依赖（参考 [Tauri 文档](https://tauri.app/v1/guides/getting-started/prerequisites)）

### 开发命令

```bash
# 启动开发服务器
pnpm tauri dev

# 类型检查
pnpm typecheck

# 代码格式化
pnpm format

# 运行测试
pnpm test
```

## 性能指标

| 指标 | 目标值 |
|------|--------|
| 冷启动时间 | < 500ms |
| 热启动时间 | < 100ms |
| 搜索响应时间 | < 50ms |
| 内存占用（空闲） | < 50MB |
| 安装包大小 | < 20MB |

## 路线图

查看 [TODOLIST.md](./TODOLIST.md) 了解详细的开发计划。

## 参考项目

- [Alfred 5](https://www.alfredapp.com/) (macOS)
- [Raycast](https://www.raycast.com/) (macOS)
- [uTools](https://u.tools/) (跨平台)
- [Snipaste](https://www.snipaste.com/) (截图贴图)

## 许可证

[MIT License](./LICENSE)

## 贡献

欢迎提交 Issue 和 Pull Request!