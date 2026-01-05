# Change: Standardize Color Workflow Between Zotero and Obsidian

## Why

The user has an existing Obsidian workflow using the **Zotero Integration plugin** that extracts annotations from Zotero PDFs into markdown files. The workflow uses 8 colors with specific semantic meanings:

| Color | Hex | Current Use | System Support |
|-------|-----|-------------|----------------|
| Blue | `#2ea8e5` | Section 1 | ✅ `section1` |
| Purple | `#a28ae5` | Section 2 | ✅ `section2` |
| Magenta | `#e56eee` | Section 3 | ✅ `section3` |
| Green | `#5fb236` | Positive point | ✅ `positive` |
| Grey | `#aaaaaa` | Point detail | ✅ `detail` |
| Red | `#ff6666` | Negative point | ✅ `negative` |
| Orange | `#f19837` | Code | ✅ `code` |
| **Yellow** | `#ffd400` | **Not used yet** | ❌ **MISSING** |

**Problem 1: Missing Yellow Color**
- The system defines 7 colors; the user's template has 8 (yellow unused)
- Future reading strategies (add-reading-strategies) will assign specific colors to different methodologies
- We need to decide: add yellow to the system OR document that only 7 colors are available

**Problem 2: Workflow Integration Confusion**
- Current plan (add-note-synthesis): AI reads Obsidian annotation files → synthesizes → writes back to Obsidian
- Alternative approach: AI creates synthesis notes **during** the reading phase → writes directly to Obsidian
- The manual "annotation import to Obsidian" step would be eliminated if synthesis happens during reading

**User's Questions:**
1. "Is there a way to standardise the colours across all strategies?"
2. "Should synthesis happen at the same time as reading and write to Obsidian?"
3. "Should I skip the manual annotation import step?"

## What Changes

This proposal addresses **color standardization** and provides **workflow clarification** without changing implementation yet.

### Option A: Add Yellow Color (Recommended)

1. **Add Yellow to HighlightColor enum**
   - New variant: `HighlightColor::Warning` or `HighlightColor::Question`
   - Hex: `#ffd400`
   - Semantic meaning: TBD based on user preference

2. **Document color usage per reading strategy**
   - Each strategy in `add-reading-strategies` specifies which of the 8 colors it uses
   - Update `.opencode/command/read-*.md` files with color mappings
   - Example: SQ3R uses blue/purple/magenta for structure, grey for questions, green for answers

3. **Update Obsidian integration specs**
   - `add-note-synthesis` must map all 8 colors (including yellow)
   - Parser handles yellow highlights from Obsidian template

### Option B: Document 7-Color Limitation (Alternative)

1. **Clarify that only 7 colors are available**
   - Update user's Obsidian template to remove yellow
   - Document that reading strategies share the 7 semantic colors
   - Each strategy may use colors differently but from the same palette

2. **No code changes needed**
   - System already has 7 colors implemented
   - Obsidian integration already handles all 7

### Workflow Integration Decision

**Keep existing workflow (No change to add-note-synthesis)**
- User continues using Zotero Integration plugin to export annotations to Obsidian
- AI reads Obsidian annotation files for synthesis (as planned in add-note-synthesis)
- Rationale: Separation of concerns
  - Reading phase: Create annotations in Zotero
  - Synthesis phase: Read from Obsidian, synthesize across documents

**Why not merge reading + synthesis:**
- Zotero remains the source of truth for annotations
- User can manually review/edit annotations in Zotero before synthesis
- Obsidian extraction can be re-run if annotations change
- Synthesis can happen later, across multiple documents
- Reading can be done without Obsidian vault access

## Impact

### Option A (Add Yellow)
- **Modified specs**: `zotero-client` (HighlightColor enum)
- **Modified specs**: `mcp-server` (HighlightColorParam enum)
- **Modified specs**: `reading-strategies` (document yellow usage)
- **Modified specs**: `obsidian-integration` (add yellow to color mapping)
- **Modified files**: 
  - `crates/zotero-client/src/types.rs`
  - `crates/zotero-mcp/src/tools.rs`
  - `.opencode/command/read-*.md` (strategy-specific files)

### Option B (Document 7 Colors)
- **Modified specs**: `reading-strategies` (clarify 7-color constraint)
- **Modified specs**: `obsidian-integration` (exclude yellow from mapping)
- **User action required**: Update Obsidian template to remove yellow

### Workflow Clarification
- **No spec changes**: Confirms add-note-synthesis approach is correct
- **Documentation**: Update proposal to emphasize workflow separation

## Dependencies

- **Blocks**: `add-reading-strategies` (needs color scheme finalized)
- **Blocks**: `add-note-synthesis` (needs color mapping finalized)

## Open Questions

### 1. Should we add yellow as the 8th color?
**Recommendation**: **YES** - Add `HighlightColor::Question` with hex `#ffd400`

**Rationale**:
- User already has yellow in their template (even if unused)
- 8 colors provides more flexibility for future reading strategies
- SQ3R methodology explicitly uses questions (perfect for yellow)
- Better to have it available than force users to modify existing templates

**If yes, what should yellow represent?**
- **Proposed**: `Question` - Questions, uncertainties, areas needing clarification
- **Use cases**: SQ3R question phase, marking confusing passages, open questions for follow-up

### 2. How should colors be used across different reading strategies?

**Recommendation**: **Each strategy documents its specific color mappings**

**Approach**:
- Define a "color usage table" in each `/read-*` command file
- Strategies can reuse colors for different purposes
- Example:
  - **SQ3R**: Blue=structure, Yellow=questions, Green=answers, Red=confusion
  - **Literature Review**: Green=supported claims, Red=limitations, Grey=methodology
  - **Analytical Reading**: Blue=premises, Purple=conclusions, Green=agreement, Red=critique

**Documentation standard**:
```markdown
## Color Scheme for [Strategy Name]

| Color | Use In This Strategy |
|-------|---------------------|
| section1 (Blue) | Main thesis / Chapter structure |
| ... | ... |
| question (Yellow) | Questions generated during Survey phase |
```

### 3. Should synthesis happen during reading or as a separate phase?

**Recommendation**: **Keep as separate phase** (no change to current plans)

**Rationale**:
- **Reading phase**: Focus on understanding and annotating ONE document
- **Synthesis phase**: Combine insights from MULTIPLE documents
- Allows user to review/edit Zotero annotations before synthesis
- Maintains flexibility (can synthesize later, across different document sets)
- Respects existing Zotero Integration workflow

**Workflow remains**:
```
/read (Phase 1-3) → Annotations in Zotero
    ↓ (User's Zotero Integration plugin)
Obsidian annotation files
    ↓ (/synthesize Phase 4)
Obsidian synthesis notes
```

### 4. What if the user wants to skip manual Obsidian import?

**Future enhancement** (not part of this change):
- Could add `/read --synthesize` flag to create Obsidian notes immediately
- Would require Obsidian vault path configuration
- Would bypass Zotero Integration plugin entirely
- Not recommended for initial implementation (adds complexity)

## Recommendation Summary

**Recommended approach**:
1. ✅ Add yellow as 8th color: `HighlightColor::Question` (`#ffd400`)
2. ✅ Document color mappings per strategy in command files
3. ✅ Keep reading and synthesis as separate phases
4. ✅ Update `add-note-synthesis` to handle 8 colors including yellow
5. ✅ Update `add-reading-strategies` to specify color usage per strategy

**User actions**:
- No template changes needed (yellow already present)
- Continue using Zotero Integration plugin for Obsidian export
- Use `/read` for annotating, `/synthesize` for multi-document synthesis

**Next steps**:
1. Get user confirmation on yellow color addition and semantic meaning
2. Update this proposal based on feedback
3. Proceed with implementation in dependent changes
