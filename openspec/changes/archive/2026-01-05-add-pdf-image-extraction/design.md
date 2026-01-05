# Design: PDF Image Extraction

## Context

Building on Phase 1's text extraction and annotation capabilities, this phase adds visual content extraction. Modern AI assistants with vision capabilities (Claude, GPT-4V) can analyze images, but need the images provided in their input.

### Constraints
- MuPDF (already used for text) supports image extraction and page rendering
- MCP tool responses can include base64-encoded images
- Large images may hit context limits - need size management

## Goals / Non-Goals

### Goals
- Extract embedded images from PDFs
- Render arbitrary page regions as images
- Provide figure detection for common academic paper layouts
- Return images in formats suitable for vision AI

### Non-Goals
- OCR (text extraction from images) - use existing text extraction
- Image editing or manipulation
- Figure captioning (AI responsibility)
- Cross-page figure assembly

## Architecture

```
┌─────────────────┐
│  MCP Server     │
│  (new tools)    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐     ┌─────────────────┐
│  zotero-client  │────▶│  PDF File       │
│  image module   │     │  (MuPDF)        │
└─────────────────┘     └─────────────────┘
         │
         ▼
┌─────────────────┐
│  Base64 Image   │
│  Response       │
└─────────────────┘
```

## Decisions

### Decision 1: Use MuPDF for Rendering
**What**: Use MuPDF's page rendering for region extraction.

**Why**:
- Already a dependency via `mupdf` crate
- High-quality PDF rendering
- Consistent with text extraction

### Decision 2: Base64 Encoding in Tool Responses
**What**: Return images as base64-encoded strings with MIME type.

**Why**:
- Standard format for MCP image content
- No need for temporary files or URLs
- Works with all vision AI providers

**Format**:
```json
{
  "image": {
    "type": "base64",
    "media_type": "image/png",
    "data": "iVBORw0KGgo..."
  }
}
```

### Decision 3: Heuristic Figure Detection
**What**: Detect figures using whitespace and content analysis.

**Why**:
- No ML model dependencies
- Fast execution
- Good enough for standard layouts

**Algorithm**:
1. Find large whitespace regions
2. Identify bounded content areas
3. Filter by aspect ratio and size
4. Return as candidate figure regions

## MCP Tool Definitions

### Tool: `zotero_extract_page_image`
```json
{
  "name": "zotero_extract_page_image",
  "description": "Render a PDF page or region as an image for vision analysis",
  "inputSchema": {
    "type": "object",
    "properties": {
      "attachment_key": { "type": "string" },
      "page": { "type": "integer", "description": "1-based page number" },
      "rect": {
        "type": "array",
        "items": { "type": "number" },
        "description": "Optional region [x1, y1, x2, y2]. If omitted, renders full page."
      },
      "format": {
        "type": "string",
        "enum": ["png", "jpeg"],
        "default": "png"
      },
      "dpi": {
        "type": "integer",
        "default": 150,
        "description": "Resolution for rendering"
      }
    },
    "required": ["attachment_key", "page"]
  }
}
```

### Tool: `zotero_list_figures`
```json
{
  "name": "zotero_list_figures",
  "description": "Detect and list figure regions on a PDF page",
  "inputSchema": {
    "type": "object",
    "properties": {
      "attachment_key": { "type": "string" },
      "page": { "type": "integer" }
    },
    "required": ["attachment_key", "page"]
  }
}
```

### Tool: `zotero_get_figure`
```json
{
  "name": "zotero_get_figure",
  "description": "Extract a detected figure as an image",
  "inputSchema": {
    "type": "object",
    "properties": {
      "attachment_key": { "type": "string" },
      "page": { "type": "integer" },
      "figure_index": { "type": "integer", "description": "Index from zotero_list_figures" },
      "format": { "type": "string", "enum": ["png", "jpeg"], "default": "png" }
    },
    "required": ["attachment_key", "page", "figure_index"]
  }
}
```

## Risks / Trade-offs

### Risk: Large Images Exceed Context Limits
**Impact**: Vision AI may reject very large images.
**Mitigation**: 
- Default to 150 DPI (reasonable size)
- Allow DPI configuration
- Add max dimension option

### Risk: Figure Detection Misses Complex Layouts
**Impact**: Some figures not detected in non-standard papers.
**Mitigation**:
- Allow manual rect specification
- Provide full-page fallback
- Document limitations

### Trade-off: Image Quality vs Size
**Choice**: PNG default for quality.
**Trade-off**: Larger response sizes.
**Mitigation**: JPEG option for size-sensitive use cases.

## Resolved Questions

1. Should we cache rendered images?
   - **Decision**: No caching for now
   - Avoids storage management complexity
   - Can revisit if performance becomes an issue
