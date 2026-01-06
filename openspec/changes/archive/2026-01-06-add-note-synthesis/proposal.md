# Change: Add Obsidian-Based Note Synthesis

## Why

After reading and annotating documents (Phases 1-3), researchers need to:

1. Synthesize insights across multiple documents
2. Create literature review summaries
3. Generate structured notes from annotations

The user's existing workflow uses **Zotero Integration** plugin in Obsidian to import annotations. This creates markdown files with:
- Metadata (citekey, tags, creators, title, etc.)
- PDF links
- Annotations with semantic colors and page references
- Structure based on color (blue=h2, purple=h3, magenta=h4, orange=code blocks)

**Phase 4 should work WITH this existing workflow**, not replace it:
- Read annotations from Obsidian markdown files (not Zotero directly)
- Create synthesis notes in Obsidian
- Maintain links to source annotation files

## What Changes

1. **Slash Commands** - AI-guided workflows using native file tools
   - `/summarize` - Generate summary from single document's Obsidian annotations
   - `/synthesize` - Synthesize annotations across multiple citekeys from Obsidian

2. **Obsidian Note Templates** - Output formats matching Obsidian conventions
   - Wikilinks to source annotation files
   - Dataview-compatible frontmatter
   - Callout blocks for structured content

## Approach: Slash Commands Only (No MCP Tools)

Since Obsidian vaults are just filesystem directories, we use **slash commands** that instruct the AI to:
- Use the `Read` tool to read annotation files directly
- Parse annotations using documented regex patterns
- Write synthesis notes using the `Write` tool

This is simpler than adding MCP tools because:
- No new Rust code required
- Leverages AI's native file capabilities  
- Easier to customize (edit markdown, not code)
- Faster to implement

## Workflow

```
Zotero (annotations) 
    │
    ▼ (Zotero Integration plugin - existing)
Obsidian (annotation markdown files)
    │
    ▼ (Slash commands - NEW)
Obsidian (synthesis notes)
```

The AI:
1. Reads annotation files from Obsidian vault using `Read` tool
2. Parses annotations with their colors, text, and page references
3. Synthesizes across documents
4. Writes new notes to Obsidian vault using `Write` tool

## Impact

- **New specs**: `note-synthesis` (slash command requirements only)
- **Removed**: `obsidian-integration` spec (no MCP tools needed)
- **Removed**: `mcp-server` spec changes (no new tools)
- **New files**:
  - `.opencode/command/synthesize.md`
  - `.opencode/command/summarize.md`
- **Configuration**: Obsidian vault path via environment variable or user-provided path

## Dependencies

- **Requires**: User has Zotero Integration plugin configured in Obsidian
- **Enhanced by**: Phase 3 (`add-reading-strategies`) for strategy-specific synthesis

## Open Questions

1. How to locate the Obsidian vault?
   - **Answer**: User provides path in command or environment variable `OBSIDIAN_VAULT_PATH`

2. Where should synthesis notes be created?
   - **Answer**: User specifies output path, or default to `Synthesis/` folder in vault

3. How to handle updates when annotations change?
   - **Answer**: Synthesis notes are regenerated, not incrementally updated
