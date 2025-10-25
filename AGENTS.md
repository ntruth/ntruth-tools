# Repository Guidelines

## Project Structure & Module Organization
The app targets Windows and macOS via Tauri 2.0. Vue 3 sources live under `src/`, with shared UI atoms in `components/`, feature slices in `features/` (`launcher`, `clipboard`, `screenshot`, `search`, `workflow`, `plugins`), and Pinia stores in `stores/`. Window bootstraps live in `launcher/` (quick launcher) and `main.ts`. Native integrations live in `src-tauri/` with per-capability modules (`clipboard.rs`, `screenshot.rs`, `search.rs`, `workflow.rs`, `plugin.rs`) and root wiring in `src-tauri/src/main.rs`. Documentation for agents sits in `docs/ai-context/`. Keep new modules small and purpose-driven; co-locate tests beside the module (see `useSearch.spec.ts`, `clipboard.spec.ts`, `workflow.spec.ts`).

## Build, Test, and Development Commands
- `pnpm tauri dev` — launches the Vue + Tauri development shell with hot reload.
- `pnpm tauri build` — produces the production installer in `src-tauri/target/release/bundle/`.
- `pnpm test` — runs front-end unit and component suites.
- `cargo test` — executes Rust-side unit tests.
- `pnpm lint` / `cargo clippy` — TypeScript and Rust linting; resolve all warnings before review.
- `pnpm format` / `cargo fmt` — apply Prettier and Rustfmt to keep diffs clean.

## Coding Style & Naming Conventions
Use TypeScript with strict typing. Prefer the Vue Composition API and Pinia stores; place shared composables in `composables/` prefixed with `use`. Vue single-file components adopt PascalCase filenames (`ClipboardPanel.vue`, `ScreenshotDock.vue`). Rust modules follow snake_case filenames and PascalCase types. Stick to the existing CSS-variable design language and keep styles scoped; introduce shadcn-vue or Tailwind only after aligning with maintainers. Keep functions short, document non-trivial modules, and apply SOLID + DRY principles highlighted in `claude.md`.

## Testing Guidelines
Front-end tests should accompany new Vue components or stores (`ComponentName.spec.ts` in the same feature folder). Mock Tauri bridges where possible to keep tests deterministic. Rust tests belong beside their modules using `#[cfg(test)]` blocks; cover error paths that touch OS APIs. Run both `pnpm test` and `cargo test` before opening a PR, and add regression tests whenever fixing bugs.

## Commit & Pull Request Guidelines
Write atomic commits with imperative present-tense subjects (`feat: add workflow automation nodes`). Reference issue IDs when available. Pull requests need: problem statement, summary of changes, test evidence (`pnpm test`, `cargo test` outputs), and UI screenshots/gifs for visual updates. Highlight risks or follow-up work, and ensure documentation (README, docs/ai-context) stays in sync with behavior changes.

## Security & Configuration Tips
Never commit secrets; use `.env` locally and document required variables. Validate all user-supplied input on the Rust side, and keep logging free of sensitive payloads. When adding new Tauri permissions, document the rationale in `docs/` so agents understand the expanded surface area.
