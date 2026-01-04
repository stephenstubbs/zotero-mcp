# Zotero MCP Server

An MCP (Model Context Protocol) server that exposes Zotero operations as tools for AI assistants. Enables critical reading workflows with PDF text extraction and annotation creation.

## Features

- **zotero_lookup** - Find Zotero items by BetterBibTeX citation key
- **zotero_read_pdf_pages** - Extract text from PDF pages
- **zotero_create_highlight** - Create text highlight annotations with semantic colors
- **zotero_create_area_annotation** - Create area annotations for figures/diagrams

## Prerequisites

1. [Zotero](https://www.zotero.org/) installed and running
2. The `zotero-mcp-plugin` installed in Zotero (see project root)
3. [BetterBibTeX](https://retorque.re/zotero-better-bibtex/) extension for citation keys

## Installation

```bash
cargo install --path crates/zotero-mcp-server
```

Or build from source:

```bash
cargo build --release --package zotero-mcp-server
```

## Usage

### Basic Usage

```bash
# Run with default Zotero URL (http://localhost:23119/mcp)
zotero-mcp-server

# Run with custom Zotero URL
ZOTERO_URL=http://192.168.1.100:23119/mcp zotero-mcp-server
```

### MCP Configuration

Add to your MCP client configuration (e.g., `~/.config/opencode/mcp.json`):

```json
{
  "mcpServers": {
    "zotero": {
      "command": "zotero-mcp-server",
      "env": {
        "ZOTERO_URL": "http://localhost:23119/mcp"
      }
    }
  }
}
```

## Available Tools

### zotero_lookup

Find a Zotero item by its BetterBibTeX citation key.

**Parameters:**
- `citekey` (string, required): BetterBibTeX citation key (e.g., "smithMachineLearning2023")

**Returns:** Item metadata including key, title, type, date, and PDF attachment keys.

### zotero_read_pdf_pages

Extract text from specific pages of a PDF attachment.

**Parameters:**
- `attachment_key` (string, required): Zotero attachment key for the PDF
- `pages` (string, required): Page range - e.g., "1-5", "1,3,5", or "all"

**Returns:** Extracted text content with page delimiters.

### zotero_create_highlight

Create a text highlight annotation on a PDF.

**Parameters:**
- `attachment_key` (string, required): Zotero attachment key for the PDF
- `text` (string, required): Text to highlight (must match PDF content exactly)
- `page` (integer, required): 1-based page number
- `color` (enum, required): Semantic color - one of:
  - `section1` - Blue (#2ea8e5) - Primary organization
  - `section2` - Purple (#a28ae5) - Secondary organization
  - `section3` - Magenta (#e56eee) - Tertiary organization
  - `positive` - Green (#5fb236) - Agreement/Support
  - `detail` - Grey (#aaaaaa) - Context/Detail
  - `negative` - Red (#ff6666) - Criticism/Disagreement
  - `code` - Orange (#f19837) - Technical content
- `comment` (string, optional): Comment to attach to the highlight

**Returns:** Confirmation with annotation key.

### zotero_create_area_annotation

Create an area/image annotation for figures, diagrams, etc.

**Parameters:**
- `attachment_key` (string, required): Zotero attachment key for the PDF
- `page` (integer, required): 1-based page number
- `rect` (array of 4 floats, required): Bounding box [x1, y1, x2, y2] in PDF coordinates
- `color` (enum, required): Semantic color (same as zotero_create_highlight)
- `comment` (string, optional): Comment to attach to the annotation

**Returns:** Confirmation with annotation key.

## Semantic Color Scheme

The server uses a predefined color scheme for consistent annotation meanings:

| Color | Hex | Purpose |
|-------|-----|---------|
| section1 | #2ea8e5 (Blue) | Section 1 / Primary organization |
| section2 | #a28ae5 (Purple) | Section 2 / Secondary organization |
| section3 | #e56eee (Magenta) | Section 3 / Tertiary organization |
| positive | #5fb236 (Green) | Positive point / Agreement |
| detail | #aaaaaa (Grey) | Point detail / Context |
| negative | #ff6666 (Red) | Negative point / Criticism |
| code | #f19837 (Orange) | Code / Technical content |

## Environment Variables

- `ZOTERO_URL` - URL of the Zotero MCP plugin (default: `http://localhost:23119/mcp`)
- `RUST_LOG` - Log level for tracing (e.g., `info`, `debug`, `trace`)

## Example Workflow

1. Look up an item by citation key:
   ```
   zotero_lookup(citekey: "smithML2023")
   ```

2. Read the introduction (pages 1-5):
   ```
   zotero_read_pdf_pages(attachment_key: "ABC123", pages: "1-5")
   ```

3. Highlight a key finding in green:
   ```
   zotero_create_highlight(
     attachment_key: "ABC123",
     text: "Our results show significant improvement",
     page: 3,
     color: "positive",
     comment: "Main finding"
   )
   ```

4. Mark a figure for reference:
   ```
   zotero_create_area_annotation(
     attachment_key: "ABC123",
     page: 4,
     rect: [72.0, 200.0, 540.0, 400.0],
     color: "section1",
     comment: "Figure 1: Architecture diagram"
   )
   ```

## License

MIT
