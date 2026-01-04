# Zotero MCP

A toolchain for AI-assisted critical reading of academic papers in Zotero. This project provides an MCP (Model Context Protocol) server that enables AI assistants to interact with Zotero for reading PDFs and creating annotations.

## Components

```
zotero-mcp/
├── crates/
│   ├── zotero-client/      # Rust client library for Zotero API
│   └── zotero-mcp-server/  # MCP server exposing Zotero tools
├── zotero-mcp-plugin/      # Zotero 7 plugin for HTTP API
└── .opencode/commands/     # Slash commands for AI workflows
```

## Quick Start

### 1. Install the Zotero Plugin

```bash
cd zotero-mcp-plugin
./build.sh
```

Then in Zotero: **Tools → Add-ons → Install Add-on From File** → select `mcp-zotero-api.xpi`

### 2. Install BetterBibTeX

Install [BetterBibTeX](https://retorque.re/zotero-better-bibtex/) for citation key support.

### 3. Build the MCP Server

```bash
cargo build --release --package zotero-mcp-server
```

### 4. Configure Your MCP Client

Add to your MCP configuration (e.g., `~/.config/opencode/mcp.json`):

```json
{
  "mcpServers": {
    "zotero": {
      "command": "/path/to/zotero-mcp-server",
      "env": {
        "ZOTERO_URL": "http://localhost:23119/mcp"
      }
    }
  }
}
```

## Available MCP Tools

| Tool | Description |
|------|-------------|
| `zotero_lookup` | Find items by BetterBibTeX citation key |
| `zotero_read_pdf_pages` | Extract text from PDF pages |
| `zotero_create_highlight` | Create text highlights with semantic colors |
| `zotero_create_area_annotation` | Create area annotations for figures |

## Semantic Color Scheme

Annotations use a consistent color scheme for meaning:

| Color | Hex | Purpose |
|-------|-----|---------|
| `section1` | #2ea8e5 (Blue) | Primary organization |
| `section2` | #a28ae5 (Purple) | Secondary organization |
| `section3` | #e56eee (Magenta) | Tertiary organization |
| `positive` | #5fb236 (Green) | Agreement/Support |
| `detail` | #aaaaaa (Grey) | Context/Detail |
| `negative` | #ff6666 (Red) | Criticism/Disagreement |
| `code` | #f19837 (Orange) | Technical content |

## Critical Reading Workflow

Use the `/read` slash command (if your AI client supports it):

```
/read smithML2023 1-20 "Understand the core methodology and evaluate the experimental design"
```

This will:
1. Look up the paper by citation key
2. Read the specified pages
3. Create semantic annotations for key points
4. Summarize findings and critical notes

### Manual Workflow

1. **Look up the paper:**
   ```
   zotero_lookup(citekey: "smithML2023")
   ```

2. **Read pages:**
   ```
   zotero_read_pdf_pages(attachment_key: "ABC123", pages: "1-10")
   ```

3. **Highlight key findings:**
   ```
   zotero_create_highlight(
     attachment_key: "ABC123",
     text: "significant improvement over baseline",
     page: 5,
     color: "positive",
     comment: "Main result"
   )
   ```

## Project Structure

### zotero-client (Library)

Rust client for the Zotero MCP plugin HTTP API:

- Item search and lookup
- PDF attachment discovery
- Text extraction with MuPDF
- Annotation creation (highlight, area)

### zotero-mcp-server (Binary)

MCP server implementing the critical reading tools. Uses stdio transport for MCP protocol communication.

### zotero-mcp-plugin (Zotero Extension)

Zotero 7 plugin that exposes HTTP endpoints at `http://localhost:23119/mcp/`:

- `GET /ping` - Health check
- `GET /search` - Search items
- `GET /items` - Get item by key
- `GET /children` - Get child items
- `POST /annotations` - Create annotations

## Development

### Prerequisites

- Rust 1.75+
- MuPDF (for PDF text extraction)
- Zotero 7 with the MCP plugin installed

### Build

```bash
# Build everything
cargo build --workspace

# Run tests
cargo test --workspace

# Check formatting and lints
cargo fmt --check
cargo clippy --workspace --all-targets
```

### Running the MCP Server Locally

```bash
RUST_LOG=info cargo run --package zotero-mcp-server
```

## License

MIT
