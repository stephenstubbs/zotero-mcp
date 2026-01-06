# Tasks: Add Obsidian-Based Note Synthesis

## Prerequisites
- [x] 0.1 User has Zotero Integration plugin configured in Obsidian

## 1. Summarize Command

- [x] 1.1 Create `.opencode/command/summarize.md`
- [x] 1.2 Document annotation file location (vault path, citekey lookup)
- [x] 1.3 Document annotation parsing patterns (color, text, page, comment)
- [x] 1.4 Document color-to-semantic mapping with hierarchy vs content distinction
- [x] 1.5 Document comment prefix extraction (THESIS:, Q:, etc.)
- [x] 1.6 Document summary output template with wikilinks
- [x] 1.7 Document Dataview-compatible frontmatter format

## 2. Synthesize Command

- [x] 2.1 Create `.opencode/command/synthesize.md`
- [x] 2.2 Document multi-file reading workflow
- [x] 2.3 Document theme identification from section headings
- [x] 2.4 Document cross-document grouping strategy
- [x] 2.5 Document synthesis output template with source attribution
- [x] 2.6 Document theme parameter usage

## 3. Validation

- [x] 3.1 Run `openspec validate add-note-synthesis --strict`
- [ ] 3.2 Test `/summarize` with sample annotation file
- [ ] 3.3 Test `/synthesize` with multiple annotation files

## Dependencies

- Tasks 1.x and 2.x can run in parallel
- Task 3.x depends on 1.x and 2.x

## Verification

- Slash commands parse annotation files correctly
- Created notes appear correctly in Obsidian
- Wikilinks resolve to source files
- Frontmatter works with Dataview queries
