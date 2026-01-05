# Change: Update Section Color Semantics for Obsidian Compatibility

## Why

The Obsidian Zotero import template interprets Blue/Purple/Magenta (section1/2/3) colors as **document hierarchy** markers, generating H2/H3/H4 headings respectively. However, the current `/read` command files use these colors for **semantic content types** (thesis, supporting arguments, methodology).

This mismatch means exported annotations don't produce the expected hierarchical structure in Obsidian notes.

## What Changes

1. **Redefine section1/2/3 colors** as strictly hierarchical:
   - `section1` (Blue) = H2-level section headings / Major topics
   - `section2` (Purple) = H3-level subsection headings
   - `section3` (Magenta) = H4-level sub-subsections / Minor headings

2. **Update `/read.md`** color scheme documentation to reflect hierarchical usage

3. **Update `/read-sq3r.md`** - Adjust color mappings for SQ3R phases

4. **Update `/read-review.md`** - Adjust color mappings for literature review

5. **Update `/read-analyze.md`** - Adjust color mappings for analytical reading
   - Move "premises" and "conclusions" from section1/2 to other colors

6. **Update `/read-skim.md`** - Verify minimal color set is compatible

## Impact

- **Modified specs**: `color-scheme`
- **Modified files**:
  - `.opencode/command/read.md`
  - `.opencode/command/read-sq3r.md`
  - `.opencode/command/read-review.md`
  - `.opencode/command/read-analyze.md`
  - `.opencode/command/read-skim.md`

## Migration

Existing annotations in Zotero will retain their colors. Users re-importing to Obsidian will see improved heading structure for new annotations.
