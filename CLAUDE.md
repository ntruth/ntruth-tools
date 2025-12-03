# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

OmniBox is a cross-platform (macOS/Windows) productivity launcher application built with Tauri 2.0 and Rust. It integrates quick search, clipboard management, screenshot annotation, AI assistant, workflow automation, and plugin extension capabilities.

## Tech Stack

- **Frontend**: SolidJS + TypeScript + Vite + TailwindCSS
- **Backend**: Tauri 2.0 + Rust
- **Database**: SQLite + SQLCipher (encrypted local storage)
- **Async Runtime**: Tokio
- **Serialization**: Serde

## Project Structure

```
omnibox/
├── src/                          # Frontend (SolidJS)
│   ├── components/               # Reusable UI components
│   ├── pages/                    # Page components (Main, Clipboard, Screenshot, AI, Settings)
│   ├── stores/                   # Solid stores for state management
│   ├── services/                 # Frontend service layer (Tauri API wrappers)
│   ├── hooks/                    # Custom hooks
│   ├── types/                    # TypeScript type definitions
│   └── utils/                    # Utility functions
│
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── main.rs              # Application entry point
│   │   ├── app/                 # App core (state, config, shortcuts, tray)
│   │   ├── commands/            # Tauri commands (IPC handlers)
│   │   ├── core/                # Core business modules
│   │   │   ├── parser/          # Input parsing engine
│   │   │   ├── indexer/         # File indexing engine
│   │   │   ├── clipboard/       # Clipboard management
│   │   │   ├── screenshot/      # Screenshot engine
│   │   │   ├── ai/              # AI client implementations
│   │   │   ├── workflow/        # Workflow engine
│   │   │   └── plugin/          # Plugin system
│   │   ├── platform/            # Platform-specific implementations (macOS/Windows)
│   │   ├── storage/             # Database, keychain, cache
│   │   └── utils/               # Utility modules
│   └── tauri.conf.json          # Tauri configuration
```

## Common Commands

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build

# Type check
pnpm typecheck

# Format code
pnpm format

# Run tests
pnpm test

# Rust format
cargo fmt --manifest-path src-tauri/Cargo.toml

# Rust clippy
cargo clippy --manifest-path src-tauri/Cargo.toml
```

## Architecture Notes

### Multi-Window Architecture

The app uses multiple windows managed by Tauri:
- **main**: Main search window (borderless, transparent)
- **clipboard**: Clipboard history window (borderless, follows cursor)
- **screenshot-overlay**: Screenshot selection overlay (fullscreen, transparent)
- **screenshot-editor**: Annotation editor
- **pin-{id}**: Pinned screenshot windows (can have multiple)
- **ai-chat**: AI conversation window
- **settings**: Settings/preferences window

### IPC Communication

Frontend communicates with Rust backend via Tauri's IPC:
- Use `invoke()` for command calls
- Use `listen()` / `emit()` for events
- All commands are defined in `src-tauri/src/commands/`

### State Management

- **Frontend**: Solid stores (`src/stores/`)
- **Backend**: AppState struct managed by Tauri (`src-tauri/src/app/state.rs`)

### Configuration

Configuration is stored in `config.yaml` at:
- macOS: `~/Library/Application Support/OmniBox/`
- Windows: `%APPDATA%/OmniBox/`

Configuration is managed by `ConfigManager` and includes settings for:
- General, Features, Appearance
- Clipboard, Screenshot, AI
- Web Search, Shortcuts, Indexer

## Key Modules

### Input Parser (`core/parser/`)
Parses user input and determines the action type:
- File/app search
- Calculator expressions
- Web search triggers (e.g., `gg query`)
- AI triggers (`ai query`)
- Clipboard search (`cb query`)

### File Indexer (`core/indexer/`)
- Uses Trie + Trigram indexing for fast fuzzy search
- Watches file system for real-time updates
- Configurable index paths and exclusions

### Clipboard Manager (`core/clipboard/`)
- Monitors system clipboard
- Stores history in SQLite
- Filters sensitive content
- Simulates paste action

### Screenshot Engine (`core/screenshot/`)
- Captures screen regions/windows
- Provides annotation tools
- Manages pinned screenshot windows

### AI Client (`core/ai/`)
- Supports multiple providers (OpenAI, Anthropic, Ollama, etc.)
- Handles streaming responses
- Manages conversation history

### Workflow Engine (`core/workflow/`)
- Executes node-based workflows
- Supports multiple triggers (keyword, hotkey, cron, etc.)
- Extensible node types

## Coding Guidelines

### Rust
- Use `Result<T, AppError>` for error handling
- Use `Arc<T>` for shared state, `RwLock` for mutable shared state
- Keep platform-specific code in `platform/` module
- Use `#[tauri::command]` for IPC handlers

### TypeScript/SolidJS
- Use TypeScript strict mode
- Prefer Solid's fine-grained reactivity
- Keep components small and focused
- Use services layer for Tauri API calls

### General
- Follow existing code style
- Write tests for new features
- Update documentation when adding features
- Use meaningful commit messages
