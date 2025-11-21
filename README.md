# coding-agent-search

Unified TUI for local coding-agent history search (Codex, Claude Code, Gemini CLI, Cline, OpenCode, Amp).

## Toolchain & dependency policy
- Toolchain: pinned to latest Rust nightly via `rust-toolchain.toml` (rustfmt, clippy included).
- Crates: track latest releases with wildcard constraints (`*`). Run `cargo update` regularly to pick up fixes.
- Edition: 2024.

## Env loading
Load `.env` at startup using dotenvy (see `src/main.rs`); do not use `std::env::var` without calling `dotenvy::dotenv().ok()` first.

## Dev commands (nightly)
- `cargo check --all-targets`
- `cargo clippy --all-targets -- -D warnings`
- `cargo fmt --check`

## Structure (scaffold)
- `src/main.rs` – entrypoint wiring tracing + dotenvy
- `src/lib.rs` – library entry
- `src/config/` – configuration layer
- `src/storage/` – SQLite backend
- `src/search/` – Tantivy/FTS
- `src/connectors/` – agent log parsers
- `src/indexer/` – indexing orchestration
- `src/ui/` – Ratatui interface
- `src/model/` – domain types
