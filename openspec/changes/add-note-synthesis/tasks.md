# Tasks: Add Obsidian-Based Note Synthesis

## Prerequisites
- [ ] 0.1 Phase 1 (`add-critical-reading-workflow`) is complete and merged
- [ ] 0.2 User has Zotero Integration plugin configured in Obsidian

## 1. Obsidian Integration Crate

- [ ] 1.1 Create `crates/obsidian-client/` crate structure
- [ ] 1.2 Add vault path configuration (env var + config file)
- [ ] 1.3 Implement vault file discovery (list markdown files)
- [ ] 1.4 Implement frontmatter parsing (YAML extraction)
- [ ] 1.5 Implement file reading with path resolution
- [ ] 1.6 Implement file writing with frontmatter serialization
- [ ] 1.7 Write unit tests for file operations

## 2. Annotation Parser

- [ ] 2.1 Create annotation parsing module
- [ ] 2.2 Implement color extraction from `<mark>` tags
- [ ] 2.3 Implement annotation type detection (highlight, note, image)
- [ ] 2.4 Implement page reference extraction (`[@citekey p. X]`)
- [ ] 2.5 Implement comment extraction (bold text, heading text)
- [ ] 2.6 Implement heading level detection (##, ###, ####)
- [ ] 2.7 Implement image path extraction for image annotations
- [ ] 2.8 Map color hex codes to semantic names
- [ ] 2.9 Write unit tests with sample annotation files
- [ ] 2.10 Test with user's actual template format

## 3. MCP Server Tools

- [ ] 3.1 Implement `obsidian_read_annotations` tool
- [ ] 3.2 Implement `obsidian_write_note` tool
- [ ] 3.3 Implement `obsidian_list_annotation_files` tool
- [ ] 3.4 Implement `obsidian_search` tool
- [ ] 3.5 Add color filtering to annotation reading
- [ ] 3.6 Add tag filtering to file listing
- [ ] 3.7 Write unit tests for each tool
- [ ] 3.8 Write integration tests with sample vault

## 4. Summarize Command

- [ ] 4.1 Create `.opencode/commands/summarize.md`
- [ ] 4.2 Write instructions for reading annotations from Obsidian
- [ ] 4.3 Write instructions for grouping by semantic color
- [ ] 4.4 Write instructions for generating summary
- [ ] 4.5 Write instructions for creating note with wikilinks
- [ ] 4.6 Document output location configuration
- [ ] 4.7 Test end-to-end with annotated document

## 5. Synthesize Command

- [ ] 5.1 Create `.opencode/commands/synthesize.md`
- [ ] 5.2 Write instructions for gathering annotations from multiple files
- [ ] 5.3 Write instructions for theme identification
- [ ] 5.4 Write instructions for cross-document synthesis
- [ ] 5.5 Write instructions for creating synthesis note
- [ ] 5.6 Document frontmatter format for Dataview compatibility
- [ ] 5.7 Test with multiple annotated documents

## 6. Output Templates

- [ ] 6.1 Create summary note template with wikilinks
- [ ] 6.2 Create synthesis note template with source attribution
- [ ] 6.3 Create literature review template
- [ ] 6.4 Ensure Dataview-compatible frontmatter in all templates
- [ ] 6.5 Document template customization options

## 7. Configuration

- [ ] 7.1 Add `OBSIDIAN_VAULT_PATH` environment variable support
- [ ] 7.2 Add `OBSIDIAN_ANNOTATIONS_FOLDER` configuration
- [ ] 7.3 Add `OBSIDIAN_SYNTHESIS_FOLDER` configuration
- [ ] 7.4 Support `.zotero-mcp.toml` config file
- [ ] 7.5 Document configuration options

## 8. Documentation

- [ ] 8.1 Document Obsidian integration setup
- [ ] 8.2 Document required Zotero Integration template format
- [ ] 8.3 Add workflow examples (single doc summary, multi-doc synthesis)
- [ ] 8.4 Document Dataview queries for synthesis notes
- [ ] 8.5 Update project README

## Dependencies

- Task 0.x blocks all other tasks
- Tasks 1.x and 2.x can run in parallel
- Task 3.x depends on 1.x and 2.x
- Tasks 4-5 depend on 3.x
- Tasks 6-8 can run parallel with 4-5

## Verification

- Unit tests pass: `cargo test --workspace`
- Integration tests pass: `cargo test --workspace --features integration`
- Annotation parsing matches user's template format
- Created notes appear correctly in Obsidian
- Wikilinks resolve to source files
- Frontmatter works with Dataview queries
