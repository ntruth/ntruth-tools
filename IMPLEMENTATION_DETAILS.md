# Main Window UI and File Search Implementation

## Overview

This implementation provides a comprehensive main window UI and file search functionality for OmniBox, including:

- Modern, borderless transparent window with glassmorphism effects
- Responsive search input with real-time suggestions
- Keyboard-driven navigation
- Multiple search types (files, apps, calculator, web search, etc.)
- Fast file indexing with Trie and Trigram algorithms

## Features Implemented

### Frontend Components (SolidJS + TypeScript)

#### 1. Search Input Component (`src/components/SearchBox/SearchInput.tsx`)
- Auto-focus on window open
- Real-time input with 150ms debounce
- Clear button
- Input type indicator (e.g., "Google Search", "Calculator")
- Icons from lucide-solid

#### 2. Result List Component (`src/components/ResultList/`)
- Virtual scrolling support for large result sets
- Auto-scroll to selected item
- Loading state
- Empty state with custom messages

#### 3. Result Item Component (`src/components/ResultList/ResultItem.tsx`)
- Icon display (file, app, calculator, web, AI, clipboard, command)
- Title and subtitle
- Highlight matching text (basic implementation)
- Keyboard shortcut hints (⌘/Ctrl + 1-9)
- Hover effects
- Selection state styling

#### 4. Action Bar Component (`src/components/ActionBar/ActionBar.tsx`)
- Displays keyboard shortcuts
- Toggle visibility based on results

#### 5. Custom Hooks
- `useKeyboard.ts`: Handles keyboard navigation (arrows, Enter, Escape, Cmd+numbers)
- `useDebounce.ts`: Debounces values with configurable delay

#### 6. Main Page (`src/pages/Main/index.tsx`)
- Integrates all components
- Manages search state
- Handles keyboard navigation
- Executes search results based on action type

### Backend Components (Rust + Tauri)

#### 1. Input Parser (`src-tauri/src/core/parser/mod.rs`)

Parses user input and determines action type:

- **File/App Search**: Direct input → searches files and apps
- **Calculator**: Starts with `=` or contains math expressions
  - Supports: +, -, *, /, ^, %, sin, cos, tan, sqrt, log, ln, abs, pi, e
  - Uses `meval` crate for evaluation
- **Web Search**: Keyword + query
  - `gg` → Google
  - `bd` → Baidu
  - `bi` → Bing
  - `ddg` → DuckDuckGo
  - `gh` → GitHub
  - `so` → Stack Overflow
  - `yt` → YouTube
  - `tw` → Twitter
  - `npm` → NPM
  - `crate` → Crates.io
- **URL**: Auto-detects URLs and adds protocol if missing
- **AI Query**: `ai <query>` → Opens AI chat
- **Clipboard Search**: `cb <query>` → Searches clipboard history
- **Bookmark Search**: `bm <query>` → Searches bookmarks
- **System Command**: `> <command>` → Executes system command

#### 2. File Indexer (`src-tauri/src/core/indexer/`)

##### Trie Index (`trie.rs`)
- Prefix matching for fast lookups
- Fuzzy search with Levenshtein distance
- Case-insensitive

##### Trigram Index (`trigram.rs`)
- Fuzzy matching using trigrams
- Similarity scoring (Jaccard)
- Minimum threshold of 0.3

##### File Scanner (`scanner.rs`)
- Recursive directory scanning
- Configurable exclusion patterns:
  - `node_modules/`, `.git/`, `target/`, `dist/`, `build/`, `.cache/`
  - `Library/` (macOS), `AppData/` (Windows)
- Max depth limit (default: 10)
- Hidden file filtering

##### Ranking Algorithm (`ranker.rs`)
Scores results based on:
- **Match Quality**: Exact match (100) > Starts with (50) > Contains (25)
- **Word Boundary**: +20 for matching complete words
- **Frequency**: Logarithmic scale based on access count
- **Recency**: Exponential decay (30-day half-life)
- **Length Penalty**: Shorter names preferred

##### File Watcher (`watcher.rs`)
- Real-time file system monitoring using `notify` crate
- Debounced events (2 seconds)
- Watches for create, modify, and delete events

#### 3. Platform-Specific App Scanners

##### macOS (`src-tauri/src/platform/macos/apps.rs`)
- Scans `/Applications` and `~/Applications`
- Parses `.app` bundles
- Reads `Info.plist` for:
  - Bundle ID
  - Version
  - Icon path
- Extracts `.icns` icon files

##### Windows (`src-tauri/src/platform/windows/apps.rs`)
- Scans Start Menu (Common and User)
- Scans Program Files directories
- Parses `.lnk` shortcut files
- Finds `.exe` executables
- Filters out uninstallers and system utilities

#### 4. Search Commands (`src-tauri/src/commands/search.rs`)

- `search(query)`: Main search command
  - Parses input
  - Routes to appropriate handler
  - Returns structured results
- `calculate(expression)`: Calculator command
  - Evaluates math expressions
  - Returns formatted result

#### 5. App State (`src-tauri/src/app/state.rs`)

- Manages global app state
- Holds indexer instance
- Initializes indexing on startup for:
  - `~/Documents`
  - `~/Desktop`
  - `~/Downloads`

## Architecture Decisions

### Why SolidJS?
- Fine-grained reactivity (no virtual DOM)
- Better performance for real-time search
- Smaller bundle size

### Why Trie + Trigram?
- **Trie**: Fast prefix matching for exact/partial matches
- **Trigram**: Handles typos and fuzzy matching
- Combined: Best of both worlds

### Why Two-Phase Indexing?
1. Initial scan on startup (common directories)
2. File watcher for incremental updates
- Balances startup time vs. real-time accuracy

## Usage

### Search Syntax

```
hello              → Search files/apps for "hello"
= 2 + 2            → Calculator: 4
gg rust            → Google search for "rust"
ai explain quantum → Ask AI about quantum
cb password        → Search clipboard for "password"
> ls -la           → Execute shell command
https://rust-lang.org → Open URL
```

### Keyboard Shortcuts

- `↑/↓`: Navigate results
- `Enter`: Execute selected result
- `Esc`: Close window
- `⌘/Ctrl + 1-9`: Quick select (first 9 results)

## Performance Characteristics

- **Search Response**: < 50ms (target)
- **Index Capacity**: 100,000+ files
- **Memory Usage**: ~50MB for 100k files
- **Initial Index Time**: ~2-5 seconds for typical user directories

## Future Improvements

1. **Search Filters**
   - File extension: `.pdf`, `.md`
   - Path: `path:Documents`
   - Size: `size:>10mb`
   - Modified: `modified:7d`, `modified:2024-01`

2. **App Icon Caching**
   - Extract and cache app icons
   - Display in result list

3. **Smart Ranking**
   - Learn from user behavior
   - Boost frequently used results
   - Time-of-day patterns

4. **Advanced Fuzzy Matching**
   - CamelCase matching (e.g., "FBR" → "FooBarBaz")
   - Acronym matching
   - Phonetic matching

5. **Performance Optimization**
   - Index compression
   - Lazy loading
   - Result caching

## Testing

### Frontend
```bash
pnpm typecheck  # Type checking
pnpm lint       # Linting
```

### Backend
```bash
cargo test      # Run Rust tests
cargo clippy    # Linting
```

## Dependencies

### Frontend
- `solid-js`: UI framework
- `lucide-solid`: Icons
- `@tauri-apps/api`: Tauri bindings

### Backend
- `meval`: Math expression evaluation
- `notify`: File system watching
- `urlencoding`: URL encoding for web search
- `tokio`: Async runtime
- `serde`: Serialization

## File Structure

```
src/
├── components/
│   ├── SearchBox/
│   │   ├── SearchInput.tsx
│   │   └── index.ts
│   ├── ResultList/
│   │   ├── ResultList.tsx
│   │   ├── ResultItem.tsx
│   │   └── index.ts
│   └── ActionBar/
│       ├── ActionBar.tsx
│       └── index.ts
├── hooks/
│   ├── useKeyboard.ts
│   ├── useDebounce.ts
│   └── index.ts
├── pages/
│   └── Main/
│       └── index.tsx
├── services/
│   └── tauri.ts
└── types/
    └── search.ts

src-tauri/src/
├── core/
│   ├── parser/
│   │   └── mod.rs
│   └── indexer/
│       ├── mod.rs
│       ├── trie.rs
│       ├── trigram.rs
│       ├── scanner.rs
│       ├── ranker.rs
│       └── watcher.rs
├── platform/
│   ├── macos/
│   │   ├── mod.rs
│   │   └── apps.rs
│   └── windows/
│       ├── mod.rs
│       └── apps.rs
├── commands/
│   └── search.rs
└── app/
    └── state.rs
```
