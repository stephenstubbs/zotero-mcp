# Change: Add PDF Image Extraction for AI Vision Analysis

## Why

Phase 1 enables text highlighting and area annotations, but the AI cannot actually "see" figures, diagrams, or images in PDFs. To enable true critical reading of visual content, the AI needs:

1. Ability to extract images/figures from PDF pages
2. Render page regions as images for vision model analysis
3. Detect figure boundaries automatically

This enables workflows like:
- "Analyze the methodology diagram on page 5"
- "Summarize all figures in this paper"
- "Annotate the key relationships shown in Figure 3"

## What Changes

1. **Image Extraction** - New capability in `zotero-client` crate
   - Extract embedded images from PDF pages
   - Render arbitrary page regions as PNG/JPEG
   - Detect figure bounding boxes using heuristics

2. **MCP Server Tools** - New tools for vision workflows
   - Tool: `zotero_extract_page_image` - Render a page or region as image
   - Tool: `zotero_list_figures` - Detect and list figures on a page
   - Tool: `zotero_get_figure` - Extract a specific figure as base64 image

3. **Vision Model Integration** - Return images in MCP-compatible format
   - Base64-encoded images in tool responses
   - Support for PNG and JPEG output formats

## Impact

- **New specs**: `image-extraction`
- **Modified specs**: `mcp-server` (new tools)
- **New code**:
  - `crates/zotero-client/src/image.rs` - Image extraction module
  - MCP server tool implementations
- **Dependencies**: May require additional image processing crates

## Dependencies

- **Requires**: Phase 1 (`add-critical-reading-workflow`) must be complete
- **Blocked by**: MCP server must exist before adding new tools

## Open Questions

1. What image format should be default? PNG (lossless) vs JPEG (smaller)?
   - **Recommendation**: PNG default, JPEG option for large images

2. How to handle multi-page figures or figures spanning columns?
   - **Recommendation**: Start with single-region extraction, enhance later

3. Should figure detection use ML models or heuristics?
   - **Recommendation**: Start with heuristics (whitespace analysis), ML later
