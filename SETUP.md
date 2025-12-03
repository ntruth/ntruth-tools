# OmniBox Development Setup

This guide helps you set up the OmniBox development environment.

## Prerequisites

### All Platforms
- **Node.js** 18+ ([Download](https://nodejs.org/))
- **pnpm** 8+ (Install: `npm install -g pnpm`)
- **Rust** 1.75+ ([Install](https://www.rust-lang.org/tools/install))

### Platform-Specific Dependencies

#### macOS
```bash
# Xcode Command Line Tools
xcode-select --install

# Install dependencies via Homebrew
brew install pkg-config
```

#### Linux (Debian/Ubuntu)
```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  pkg-config
```

#### Windows
- Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section)

## Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/ntruth/ntruth-tools.git
   cd ntruth-tools
   ```

2. **Install frontend dependencies**
   ```bash
   pnpm install
   ```

3. **Build frontend**
   ```bash
   pnpm build
   ```

4. **Check Rust backend**
   ```bash
   cd src-tauri
   cargo check
   ```

## Development

### Run in development mode
```bash
# From project root
pnpm tauri dev
```

This will:
- Start the Vite dev server (http://localhost:1420)
- Build and run the Tauri application
- Enable hot module replacement (HMR)

### Build for production
```bash
pnpm tauri build
```

The built application will be in `src-tauri/target/release/`.

## Project Structure

```
omnibox/
├── src/                    # Frontend (SolidJS)
│   ├── App.tsx            # Root component with multi-window routing
│   ├── main.tsx           # Application entry
│   ├── types/             # TypeScript type definitions
│   ├── services/          # Tauri API wrappers
│   └── stores/            # State management
│
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── main.rs        # Application entry
│   │   ├── app/           # App core (state, config, errors)
│   │   ├── commands/      # Tauri commands (IPC handlers)
│   │   ├── core/          # Core business modules (placeholders)
│   │   ├── platform/      # Platform-specific implementations
│   │   ├── storage/       # Database and cache
│   │   └── utils/         # Utility modules
│   └── tauri.conf.json    # Tauri configuration
│
└── dist/                  # Build output (generated)
```

## Available Scripts

```bash
# Frontend
pnpm dev              # Start Vite dev server
pnpm build            # Build frontend for production
pnpm typecheck        # Run TypeScript type checking
pnpm lint             # Run ESLint
pnpm lint:fix         # Fix ESLint issues
pnpm format           # Format code with Prettier

# Tauri
pnpm tauri dev        # Run app in development mode
pnpm tauri build      # Build app for production

# Rust
cd src-tauri
cargo check           # Check Rust code
cargo build           # Build Rust backend
cargo test            # Run Rust tests
cargo fmt             # Format Rust code
cargo clippy          # Run Rust linter
```

## Multi-Window Architecture

OmniBox uses multiple windows:
- **main**: Main search window (borderless, transparent)
- **clipboard**: Clipboard history window
- **settings**: Settings/preferences window
- **ai-chat**: AI conversation window
- **pin-{id}**: Pinned screenshot windows (multiple)

The frontend App.tsx automatically routes to the correct component based on the window label.

## Configuration

Application configuration will be stored at:
- **macOS**: `~/Library/Application Support/OmniBox/config.yaml`
- **Linux**: `~/.config/OmniBox/config.yaml`
- **Windows**: `%APPDATA%/OmniBox/config.yaml`

## Troubleshooting

### "tauri: command not found"
Make sure you've installed dependencies: `pnpm install`

### Build fails on Linux with "glib-2.0 not found"
Install required system dependencies (see Linux setup above).

### Build fails on Windows
Ensure you have Visual Studio C++ Build Tools and WebView2 installed.

### Frontend hot reload not working
Try stopping and restarting `pnpm tauri dev`.

## Next Steps

This is the initial project structure. The following features are placeholders and need implementation:
- File indexing engine
- Clipboard management
- Screenshot engine
- AI integration
- Workflow system
- Plugin system

See [TODOLIST.md](./TODOLIST.md) for the full development roadmap.

## Contributing

Please read our contributing guidelines before submitting pull requests.

## License

MIT License - see [LICENSE](./LICENSE) for details.
