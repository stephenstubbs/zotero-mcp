# Design: Outline Section Support

## Context

The critical reading workflow for open questions requires reading specific document sections. PDFs may or may not have outline bookmarks (table of contents). We need a flexible approach that:

1. Uses outline bookmarks when available
2. Falls back to user-provided page numbers when not
3. Maintains consistency with viewpoint-mcp patterns

## Goals

- Provide AI with document structure information (outline)
- Allow section-based reading without manual page lookup
- Support graceful degradation when outline is unavailable
- Keep existing page-range functionality intact

## Non-Goals

- Automatic section detection for PDFs without outlines
- OCR or text analysis to infer document structure
- Editing or creating PDF outlines

## Decisions

### Decision: Two-Tool Approach

**What**: Separate `zotero_get_pdf_outline` tool from `zotero_read_pdf_pages`

**Why**: 
- Follows viewpoint-mcp pattern of single-responsibility tools
- AI can inspect outline before deciding what to read
- Cleaner separation of concerns
- Outline retrieval is cheap, reading is expensive

**Alternatives considered**:
- Combined tool that returns outline + first page text: Too coupled, harder to control
- Only page-based reading with section lookup in slash command: Less flexible for AI

### Decision: Section Parameter on Read Tool

**What**: Add optional `section` parameter to `zotero_read_pdf_pages`

**Why**:
- Keeps backwards compatibility (pages parameter still works)
- Single tool call to read a section by name
- AI doesn't need to calculate page ranges

**Behavior**:
- If `section` provided, resolve to page range using outline
- If section not found in outline, return error with available sections
- If PDF has no outline, return error suggesting page numbers

### Decision: Outline-First Workflow in Slash Command

**What**: Slash command instructs AI to check outline before reading

**Workflow**:
```
1. Call zotero_get_pdf_outline
2. If outline exists and has entries:
   - Present available sections to context
   - Use section names for reading
3. If outline is empty:
   - Inform that no outline exists
   - Ask user for page numbers OR
   - Use from_page parameter if provided
```

**Why**:
- Matches user expectation (use bookmarks if available)
- Explicit fallback path (no silent failures)
- User can override with from_page:N

## Data Structures

### OutlineItem

```rust
pub struct OutlineItem {
    pub title: String,
    pub page: Option<u32>,  // 0-based page number
    pub children: Vec<OutlineItem>,
}
```

### Tool Response: zotero_get_pdf_outline

```json
{
  "has_outline": true,
  "items": [
    {
      "title": "Introduction",
      "page": 0,
      "children": []
    },
    {
      "title": "Methods",
      "page": 5,
      "children": [
        {"title": "Data Collection", "page": 6, "children": []},
        {"title": "Analysis", "page": 10, "children": []}
      ]
    }
  ]
}
```

## Risks / Trade-offs

### Risk: Inconsistent Outline Quality

**Issue**: PDF outlines vary in quality - some are detailed, some only have chapters, some are malformed.

**Mitigation**: 
- Return raw outline structure, let AI interpret
- Include page numbers so AI can fall back to ranges
- Document that outline quality depends on PDF

### Risk: Section Name Ambiguity

**Issue**: Multiple sections might have similar names (e.g., "Introduction" vs "1. Introduction")

**Mitigation**:
- Case-insensitive matching
- Partial match with preference for exact
- Return error with suggestions if ambiguous

## Resolved Questions

1. Should we support reading multiple sections in one call?
   - **Decision**: Yes, support multiple sections in one call
   - Accept comma-separated section names: `section: "Introduction,Methods"`
   - Returns combined text from all specified sections, clearly delimited

2. Should we include page count in outline response?
   - **Decision**: Yes, include page count
   - Response includes `total_pages` field
   - Useful for AI to estimate section lengths and reading scope
