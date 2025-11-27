# 🔎 coding-agent-search (cass)

![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-blue.svg)
![Rust](https://img.shields.io/badge/Rust-nightly-orange.svg)
![Status](https://img.shields.io/badge/status-alpha-purple.svg)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green.svg)

**Unified, high-performance TUI to index and search your local coding agent history.**  
Aggregates sessions from Codex, Claude Code, Gemini CLI, Cline, OpenCode, and Amp into a single, searchable timeline.

<div align="center">

```bash
# Fast path: checksum-verified install + self-test
curl -fsSL https://raw.githubusercontent.com/Dicklesworthstone/coding_agent_session_search/main/install.sh \
  | bash -s -- --easy-mode --verify
```

```powershell
# Windows (PowerShell)
irm https://raw.githubusercontent.com/Dicklesworthstone/coding_agent_session_search/main/install.ps1 | iex
install.ps1 -EasyMode -Verify
```

</div>

---

## ✨ Key Features

### ⚡ Instant Search (Sub-60ms Latency)
- **"Search-as-you-type"**: Results update instantly with every keystroke.
- **Edge N-Gram Indexing**: We frontload the work by pre-computing prefix matches (e.g., "cal" -> "calculate") during indexing, trading disk space for O(1) lookup speed at query time.
- **Smart Tokenization**: Handles `snake_case` ("my_var" matches "my" and "var"), hyphenated terms, and code symbols (`c++`, `foo.bar`) correctly.
- **Zero-Stall Updates**: The background indexer commits changes atomically; `reader.reload()` ensures new messages appear in the search bar immediately without restarting.

### 🖥️ Rich Terminal UI (TUI)
- **Three-Pane Layout**: Filter bar (top), scrollable results (left), and syntax-highlighted details (right).
- **Live Status**: Footer shows real-time indexing progress (e.g., `Indexing 150/2000 (7%)`) and active filters.
- **Mouse Support**: Click to select results, scroll panes, or clear filters.
- **Theming**: Adaptive Dark/Light modes with role-colored messages (User/Assistant/System).

### 🔗 Universal Connectors
Ingests history from all major local agents, normalizing them into a unified `Conversation -> Message -> Snippet` model:
- **Codex**: `~/.codex/sessions` (Rollout JSONL)
- **Cline**: VS Code global storage (Task directories)
- **Gemini CLI**: `~/.gemini/tmp` (Chat JSON)
- **Claude Code**: `~/.claude/projects` (Session JSONL)
- **OpenCode**: `.opencode` directories (SQLite)
- **Amp**: `~/.local/share/amp` & VS Code storage

---

## 🧠 Architecture & Engineering

`cass` employs a dual-storage strategy to balance data integrity with search performance.

### The Pipeline
1.  **Ingestion**: Connectors scan proprietary agent files and normalize them into standard structs.
2.  **Storage (SQLite)**: The **Source of Truth**. Data is persisted to a normalized SQLite schema (`messages`, `conversations`, `agents`). This ensures ACID compliance, reliable storage, and supports complex relational queries (stats, grouping).
3.  **Search Index (Tantivy)**: The **Speed Layer**. New messages are incrementally pushed to a Tantivy full-text index. This index is optimized for speed:
    *   **Fields**: `title`, `content`, `agent`, `workspace`, `created_at`.
    *   **Prefix Fields**: `title_prefix` and `content_prefix` use **Index-Time Edge N-Grams** (not stored on disk to save space) for instant prefix matching.
    *   **Deduping**: Search results are deduplicated by content hash to remove noise from repeated tool outputs.

```mermaid
flowchart LR
    classDef pastel fill:#f4f2ff,stroke:#c2b5ff,color:#2e2963;
    classDef pastel2 fill:#e6f7ff,stroke:#9bd5f5,color:#0f3a4d;
    classDef pastel3 fill:#e8fff3,stroke:#9fe3c5,color:#0f3d28;
    classDef pastel4 fill:#fff7e6,stroke:#f2c27f,color:#4d350f;
    classDef pastel5 fill:#ffeef2,stroke:#f5b0c2,color:#4d1f2c;

    subgraph Sources
      A[Codex
      Cline
      Gemini
      Claude
      OpenCode
      Amp]:::pastel
    end

    subgraph "Ingestion Layer"
      C1[**Connectors**<br/>Detect & Scan<br/>Normalize<br/>Dedupe]:::pastel2
    end

    subgraph "Dual Storage"
      S1[**SQLite (WAL)**<br/>Source of Truth<br/>Relational Data<br/>Migrations]:::pastel3
      T1[**Tantivy Index**<br/>Search Optimized<br/>Edge N-Grams<br/>Prefix Cache]:::pastel4
    end

    subgraph "Presentation"
      U1[**TUI (Ratatui)**<br/>Async Search<br/>Filter Pills<br/>Details]:::pastel5
      U2[**CLI / Robot**<br/>JSON Output<br/>Automation]:::pastel5
    end

    A --> C1
    C1 -->|Persist| S1
    C1 -->|Index New| T1
    S1 -.->|Rebuild| T1
    T1 -->|Query| U1
    T1 -->|Query| U2
```

### Background Indexing & Watch Mode
- **Non-Blocking**: The indexer runs in a background thread. You can search while it works.
- **Watch Mode**: Uses file system watchers (`notify`) to detect changes in agent logs. When you save a file or an agent replies, `cass` re-indexes just that conversation and refreshes the search view automatically.
- **Progress Tracking**: Atomic counters track scanning/indexing phases, displayed unobtrusively in the TUI footer.

---

## 🚀 Quickstart

### 1. Install
```bash
curl -fsSL https://raw.githubusercontent.com/Dicklesworthstone/coding_agent_session_search/main/install.sh \
  | bash -s -- --easy-mode --verify
```

### 2. Launch
```bash
cass
```
*On first run, `cass` performs a full index. You'll see progress in the footer. Search works immediately (falling back to SQLite or partial results until complete).*

### 3. Usage
- **Type to search**: "python error", "refactor auth", "c++".
- **Navigation**: `Up`/`Down` to select, `Right` to focus detail pane.
- **Filters**:
    - `F3`: Filter by Agent (e.g., "codex").
    - `F4`: Filter by Workspace/Project.
    - `F5`/`F6`: Time filters (Today, Week, etc.).
- **Actions**:
    - `Enter`: Open original log file in `$EDITOR`.
    - `y`: Copy file path or snippet to clipboard.

---

## 🛠️ CLI Reference

The `cass` binary supports both interactive use and automation.

```bash
cass [tui] [--data-dir DIR] [--once]
cass index [--full] [--watch] [--data-dir DIR]
cass search "query" --robot --limit 5
cass stats --json
```

- **cass (default)**: Starts TUI + background watcher.
- **index --full**: Forces a complete rebuild of the DB and Index.
- **index --watch**: Runs essentially as a daemon, watching for file changes.
- **search --robot**: Outputs JSON for other tools to consume.

## 🤖 AI / Automation Mode

`cass` is designed to be used by *other* AI agents.

- **Self-Documenting**: Run `cass --robot-help` for a machine-optimized guide (Contract v1).
- **Structured Data**: Use `--robot` or `--json` for strictly typed JSON output on stdout.
- **Exit Codes**:
    - `0`: Success
    - `2`: Usage error
    - `3`: Missing index (run `cass index --full`)
    - `9`: Unknown error
- **Traceability**: Use `--trace-file <path>` to log execution spans for debugging.

### Ready-to-paste blurb for AGENTS.md / CLAUDE.md
> **cass (Coding Agent Session Search)** — High-performance local search for agent history.
> - **Discovery**: `cass --robot-help` (prints automation contract).
> - **Search**: `cass search "query" --robot [--limit N --agent codex]`.
> - **Inspect**: `cass view <source_path> -n <line> --json`.
> - **Manage**: `cass index --full` (rebuilds index).
> - **Design**: stdout is data-only JSON; stderr is diagnostics.

---

## 🔒 Integrity & Safety
- **Verified Install**: The installer enforces SHA256 checksums.
- **Sandboxed Data**: All indexes/DBs live in standard platform data directories (`~/.local/share/coding-agent-search` on Linux).
- **Read-Only Source**: `cass` *never* modifies your agent log files. It only reads them.

## 🧪 Developer Workflow
We target **Rust Nightly** to leverage the latest optimizations.

```bash
# Build & Test
cargo build --release
cargo test

# Run End-to-End Tests
cargo test --test e2e_index_tui
cargo test --test install_scripts
```

## 📜 License
MIT or Apache-2.0. See [LICENSE](LICENSE) for details.