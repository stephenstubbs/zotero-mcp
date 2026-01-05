# Design: Critical Reading Workflow

## Context

This design enables AI-assisted critical reading of academic papers stored in Zotero. The user invokes a slash command from their IDE (e.g., opencode), the MCP server provides tools for the AI to read PDF content and create annotations, and the Zotero plugin receives annotation requests.

### Stakeholders
- Researchers using Zotero for reference management
- AI coding assistants (Claude, etc.) that consume MCP tools
- IDE integrations (opencode) that provide slash commands

### Constraints
- Zotero plugin runs inside Zotero's JavaScript environment
- MCP protocol requires specific tool/resource definitions
- PDFs may have varying structures (chapters, sections, page numbering)

## Goals / Non-Goals

### Goals
- Enable AI to read specific pages from a Zotero PDF by citekey
- Support semantic highlighting with predefined color meanings
- Support area annotations for figures/images
- Provide a simple slash command interface for users
- Maintain separation between MCP server (tools) and Zotero client (API)

### Non-Goals
- OCR or image understanding (future phase)
- Real-time collaboration features
- Syncing annotations to cloud services
- Supporting non-PDF attachment types

## Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  IDE/opencode   │────▶│  MCP Server      │────▶│  Zotero Plugin  │
│  /read command  │     │  (Rust binary)   │     │  (HTTP API)     │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                               │
                               ▼
                        ┌──────────────────┐
                        │  PDF Files       │
                        │  (local storage) │
                        └──────────────────┘
```

### Component Responsibilities

1. **Slash Command** (`.opencode/commands/read.md`)
   - Parses user input (citekey, pages, purpose)
   - Provides AI with context and instructions
   - References MCP tools for execution

2. **MCP Server** (`crates/zotero-mcp-server/`)
   - Exposes tools via MCP protocol
   - Coordinates between AI requests and Zotero
   - Handles PDF reading directly (uses `zotero-client` pdf feature)

3. **Zotero Client** (`crates/zotero-client/`)
   - HTTP client for Zotero plugin API
   - PDF text extraction (existing)
   - Area annotation support (new)

4. **Zotero Plugin** (`zotero-mcp-plugin/`)
   - Creates annotations in Zotero database
   - Area annotation type support (new)

## Decisions

### Decision 1: MCP Server as Separate Crate
**What**: Create `zotero-mcp-server` as a standalone binary crate.

**Why**: 
- Clean separation of MCP protocol concerns from client library
- Can be installed/run independently
- Follows workspace pattern already established

**Alternatives considered**:
- Embed MCP server in client library: Rejected, mixes concerns
- Single binary with client: Rejected, forces users to run server even for library use

### Decision 2: Direct PDF Access in MCP Server
**What**: MCP server reads PDFs directly from filesystem paths provided by Zotero.

**Why**:
- Avoids round-tripping large PDF content through HTTP
- Zotero already provides file paths via API
- `zotero-client` already has PDF reading capability

**Alternatives considered**:
- Stream PDF through Zotero plugin: Rejected, inefficient and complex
- Require user to provide PDF path: Rejected, poor UX

### Decision 3: Predefined Color Constants
**What**: Define color constants in both Rust and slash command documentation.

**Why**:
- Consistent semantic meaning across all usage
- AI can reference colors by purpose rather than hex codes
- Users understand annotation meanings

**Implementation**:
```rust
pub enum HighlightColor {
    Section1,    // #2ea8e5 Blue
    Section2,    // #a28ae5 Purple
    Section3,    // #e56eee Magenta
    Positive,    // #5fb236 Green
    Detail,      // #aaaaaa Grey
    Negative,    // #ff6666 Red
    Code,        // #f19837 Orange
}
```

### Decision 4: Area Annotation Position Format
**What**: Use Zotero's native area annotation format with `type: "image"`.

**Why**:
- Compatible with Zotero's annotation system
- Allows consistent rendering in Zotero PDF viewer

**Format**:
```json
{
  "annotationType": "image",
  "position": {
    "pageIndex": 0,
    "rects": [[x1, y1, x2, y2]]
  }
}
```

## MCP Tool Definitions

### Tool: `zotero_lookup`
```json
{
  "name": "zotero_lookup",
  "description": "Find a Zotero item by its BetterBibTeX citation key",
  "inputSchema": {
    "type": "object",
    "properties": {
      "citekey": {
        "type": "string",
        "description": "BetterBibTeX citation key (e.g., 'smithMachineLearning2023')"
      }
    },
    "required": ["citekey"]
  }
}
```

### Tool: `zotero_read_pdf_pages`
```json
{
  "name": "zotero_read_pdf_pages",
  "description": "Extract text from specific pages of a PDF attachment",
  "inputSchema": {
    "type": "object",
    "properties": {
      "attachment_key": {
        "type": "string",
        "description": "Zotero attachment key for the PDF"
      },
      "pages": {
        "type": "string",
        "description": "Page range (e.g., '1-5', '1,3,5', 'all')"
      }
    },
    "required": ["attachment_key", "pages"]
  }
}
```

### Tool: `zotero_create_highlight`
```json
{
  "name": "zotero_create_highlight",
  "description": "Create a text highlight annotation on a PDF",
  "inputSchema": {
    "type": "object",
    "properties": {
      "attachment_key": { "type": "string" },
      "text": { "type": "string", "description": "Text to highlight (must match PDF content)" },
      "page": { "type": "integer", "description": "1-based page number" },
      "color": { 
        "type": "string",
        "enum": ["section1", "section2", "section3", "positive", "detail", "negative", "code"],
        "description": "Semantic color for the highlight"
      },
      "comment": { "type": "string", "description": "Optional comment" }
    },
    "required": ["attachment_key", "text", "page", "color"]
  }
}
```

### Tool: `zotero_create_area_annotation`
```json
{
  "name": "zotero_create_area_annotation",
  "description": "Create an area/image annotation on a PDF (for figures, diagrams, etc.)",
  "inputSchema": {
    "type": "object",
    "properties": {
      "attachment_key": { "type": "string" },
      "page": { "type": "integer", "description": "1-based page number" },
      "rect": {
        "type": "array",
        "items": { "type": "number" },
        "description": "Bounding box [x1, y1, x2, y2] in PDF coordinates"
      },
      "color": {
        "type": "string", 
        "enum": ["section1", "section2", "section3", "positive", "detail", "negative", "code"]
      },
      "comment": { "type": "string" }
    },
    "required": ["attachment_key", "page", "rect", "color"]
  }
}
```

## Risks / Trade-offs

### Risk: PDF Text Search May Not Find Exact Matches
**Impact**: Highlights may fail to be created if text differs slightly.
**Mitigation**: Use fuzzy matching and provide feedback to AI for retry with adjusted text.

### Risk: Area Coordinates May Not Align with User Expectations
**Impact**: AI may select wrong regions for figures.
**Mitigation**: Start with user-specified rectangles, add image detection in future phase.

### Trade-off: Sync vs Async Annotation Creation
**Choice**: Synchronous creation - simpler, immediate feedback.
**Trade-off**: Large batches of annotations may be slow.
**Mitigation**: Accept for Phase 1, batch API in future if needed.

## Migration Plan

1. **Phase 1a**: Add area annotation support to Zotero plugin
2. **Phase 1b**: Update `zotero-client` with area annotation types
3. **Phase 1c**: Create `zotero-mcp-server` crate with basic tools
4. **Phase 1d**: Create slash command configuration
5. **Integration testing**: End-to-end test with real Zotero instance

No breaking changes to existing client API - only additions.

## Resolved Questions

1. **Chapter detection**: How to map chapter names to page ranges?
   - **Decision**: Use PDF outline/bookmarks when available, otherwise ask user for page numbers
   - If outline bookmarks exist, use those to resolve section names to page ranges
   - If no outline exists, the AI asks the user for page numbers
   - User also has the option to provide page numbers directly (from the beginning) via `from_page:N` parameter
   - See `add-outline-section-support` change for implementation details

2. **MCP server configuration**: How does user specify Zotero URL?
   - **Decision**: Environment variable `ZOTERO_URL` with localhost default
   - Follows standard 12-factor app configuration pattern
   - Default: `http://localhost:23119/mcp`
