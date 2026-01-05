# Change: Add Critical Reading Workflow via Slash Command

## Why

Users need an AI-assisted critical reading workflow that can:
1. Accept a BetterBibTeX citation key with chapter/page ranges
2. Read and analyze PDF content using critical reading techniques
3. Create semantic highlights and area annotations in Zotero using a defined color scheme

Currently, the project has a working `zotero-client` crate that can create text highlights, but lacks:
- An MCP server exposing tools to AI assistants
- Area/image annotation support (only text highlights exist)
- A slash command to initiate the critical reading workflow
- Integration with AI IDEs like opencode

## What Changes

### Phase 1: Foundation (This Proposal)

1. **MCP Server** - New crate `zotero-mcp-server` exposing Zotero operations as MCP tools
   - Tool: `zotero_lookup` - Find item by BetterBibTeX citekey
   - Tool: `zotero_read_pdf_pages` - Extract text from specified pages/chapters
   - Tool: `zotero_create_highlight` - Create text highlight with semantic color
   - Tool: `zotero_create_area_annotation` - Create image/area annotation with semantic color

2. **Area Annotation Support** - Extend Zotero plugin and client
   - **BREAKING**: Plugin must support `annotationType: "image"` with area position
   - Rust client must support area annotation creation

3. **Slash Command Configuration** - Define the `/read` command interface
   - Input: citekey, page/chapter range, reading purpose
   - Output: Triggers AI critical reading workflow

### Color Scheme (Semantic Highlighting)

| Color | Hex | Purpose |
|-------|-----|---------|
| Blue | `#2ea8e5` | Section 1 |
| Purple | `#a28ae5` | Section 2 |
| Magenta | `#e56eee` | Section 3 |
| Green | `#5fb236` | Positive point |
| Grey | `#aaaaaa` | Point detail |
| Red | `#ff6666` | Negative point |
| Orange | `#f19837` | Code |

### Future Phases (Separate Proposals)

- Phase 2: PDF image extraction for AI vision analysis
- Phase 3: Advanced reading strategies (SQ3R, etc.)
- Phase 4: Note generation and synthesis

## Impact

- **New specs**: `mcp-server`, `slash-command`, `area-annotation`
- **Modified specs**: `zotero-client` (area annotation support)
- **New code**:
  - `crates/zotero-mcp-server/` - MCP server crate
  - Plugin updates for area annotations
- **Configuration**: `.opencode/commands/read.md` slash command definition

## Open Questions

1. Should the MCP server be a separate binary or part of the client library?
   - **Recommendation**: Separate crate for clean separation of concerns

2. How should chapter ranges be specified? (e.g., "1-3", "Introduction-Methods")
   - **Recommendation**: Support both page numbers and PDF outline/bookmark names

3. Should the AI receive the full page text or use retrieval augmentation?
   - **Recommendation**: Start with full text, optimize later if needed
