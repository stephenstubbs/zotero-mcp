# Project Context

## Purpose
Zotero MCP is an MCP (Model Context Protocol) server providing Zotero automation

### Goals
- Rust-native
- LLM-friendly reading and annotation of documents

## Tech Stack
- **Language**: Rust (stable, via rust-overlay, edition 2024)
- **Async Runtime**: Tokio
- **Error Handling**: `thiserror` (libraries), `anyhow` (binaries)
- **Build Environment**: Nix flakes + direnv
- **TLS / SSL**: Rusttls
- **Zotero Automation**: Connects to API provided by provided Zotero plugin

## Conventions

### Code Style
- `rustfmt` default configuration
- Pedantic clippy lints enabled
- Prefer explicit over implicit

### Module Structure
- **Folder modules only** (directories, not single `.rs` files)
- **No inline tests** (`#[cfg(test)] mod tests` blocks)
- **Maximum 500 lines per file** — refactor into smaller modules if exceeded

```
module_name/
├── mod.rs        # Public exports
├── error.rs      # Module-specific errors (thiserror)
├── tests/        # Unit tests (folder module)
│   ├── mod.rs
│   └── *.rs
└── ...
```

```rust
// In mod.rs
#[cfg(test)]
mod tests;
```

### Naming
- Error types: `{Module}Error` (e.g., `ToolError`, `ServerError`)
- Result aliases: `type Result<T> = std::result::Result<T, {Module}Error>`
- Async: prefer `async fn` over `impl Future`

### Architecture
- **Workspace required**: Always use a Cargo workspace with multiple crates, never a single-crate project
- **Separate crates**: MCP server, tools, browser adapter, CLI
- **Hexagonal Architecture (Ports & Adapters)**: Core logic independent of external concerns

### Testing

| Type | Location | Browser? | Command |
|------|----------|----------|---------|
| Unit | `src/**/tests/` | No (mocked) | `cargo test` |
| Integration | `tests/` (crate root) | Yes | `cargo test --features integration` |

**Integration tests** require the `integration` feature flag:
```toml
[features]
integration = []
```
```rust
#![cfg(feature = "integration")]
```

**IMPORTANT: Always run BOTH test commands when implementing changes:**
```bash
# Unit tests (fast, no browser)
cargo test --workspace

# Integration tests (requires Chromium)
cargo test --workspace --features integration
```

Integration tests are NOT run by default. Failing to run integration tests will miss real browser interaction bugs.

**Requirements**:
- New features must include integration tests with real Chromium
- Test both success and failure paths
- Use `tracing` + `tracing-subscriber` with `env-filter` for test output

### Version Control
- **VCS**: jj (Jujutsu), not git
- **Commits**: Conventional commits (feat:, fix:, refactor:, docs:, test:, chore:)
