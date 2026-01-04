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

1. **Obsidian Integration** - Tools to read/write Obsidian vault files
   - Tool: `obsidian_read_annotations` - Parse annotation file for a citekey
   - Tool: `obsidian_write_note` - Create/update markdown notes
   - Tool: `obsidian_list_annotation_files` - Find annotation files in vault

2. **Synthesis Commands** - Slash commands for multi-document workflows
   - `/synthesize` - Synthesize annotations across multiple citekeys from Obsidian
   - `/summarize` - Generate summary from single document's Obsidian annotations

3. **Obsidian Note Templates** - Output formats matching Obsidian conventions
   - Wikilinks to source annotation files
   - Dataview-compatible frontmatter
   - Callout blocks for structured content

## Workflow

```
Zotero (annotations) 
    │
    ▼ (Zotero Integration plugin - existing)
Obsidian (annotation markdown files)
    │
    ▼ (Phase 4 - NEW)
Obsidian (synthesis notes)
```

The AI:
1. Reads annotation files from Obsidian vault
2. Parses annotations with their colors, text, and page references
3. Synthesizes across documents
4. Writes new notes to Obsidian vault

## Impact

- **New specs**: `obsidian-integration`, `note-synthesis`
- **Modified specs**: `mcp-server` (new tools)
- **New files**:
  - `.opencode/commands/synthesize.md`
  - `.opencode/commands/summarize.md`
- **Configuration**: Obsidian vault path setting

## Dependencies

- **Requires**: Phase 1 (`add-critical-reading-workflow`) must be complete
- **Requires**: User has Zotero Integration plugin configured in Obsidian
- **Enhanced by**: Phase 3 (`add-reading-strategies`) for strategy-specific synthesis

## Open Questions

1. How to locate the Obsidian vault?
   - **Recommendation**: Environment variable `OBSIDIAN_VAULT_PATH` or config file

2. Where should synthesis notes be created?
   - **Recommendation**: Configurable folder, default to `Synthesis/` in vault

3. Should we parse the Zotero Integration template or use a standard format?
   - **Recommendation**: Parse the user's actual template (provided above) for maximum compatibility

4. How to handle updates when annotations change?
   - **Recommendation**: Synthesis notes are regenerated, not incrementally updated
