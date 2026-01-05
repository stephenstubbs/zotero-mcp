# Change: Add Outline Section Support for Open Questions Workflow

## Why

The critical reading workflow needs a way to specify document sections for reading. PDFs often have outline bookmarks (table of contents) that define logical sections. Users need:

1. A way to read sections by outline bookmark names (e.g., "Introduction", "Methods")
2. A fallback when no outline exists - user provides page numbers directly
3. Consistency with viewpoint-mcp patterns for optional capabilities

Currently, the `zotero_read_pdf_pages` tool only accepts page ranges. For open questions/critical reading, users often want to read logical sections rather than arbitrary page ranges.

## What Changes

### 1. PDF Outline Extraction Tool

Add `zotero_get_pdf_outline` tool to retrieve the document's table of contents:
- Returns outline items with `title`, `page`, and nested children
- Returns empty array if PDF has no outline
- Allows AI to discover available sections before reading

### 2. Enhanced Page Reading Tool

Modify `zotero_read_pdf_pages` to support:
- Existing: `pages: "1-5"` (page ranges)
- New: `section: "Introduction"` (outline bookmark name)
- Behavior: Section name is resolved to page range using the outline

### 3. Slash Command Enhancement

Update `/read` command to guide AI through the workflow:
1. First call `zotero_get_pdf_outline` to check for bookmarks
2. If outline exists, present available sections to AI
3. If no outline, instruct AI to ask user for page numbers
4. Support `from_page:N` parameter for page-number-only workflows

## Impact

- **Modified specs**: `mcp-server` (new tool, modified tool)
- **Modified specs**: `slash-command` (enhanced instructions)
- **New code**:
  - `get_pdf_outline()` function in `zotero-client/src/pdf.rs`
  - `zotero_get_pdf_outline` tool in MCP server
  - Enhanced `zotero_read_pdf_pages` with section support

## Consistency with viewpoint-mcp

Following viewpoint-mcp patterns:
- Tool returns structured data that AI interprets
- Fallback behavior is explicit (empty outline = ask user)
- Optional parameters don't break existing usage
