# Design: Standardize Color Workflow Between Zotero and Obsidian

## Context

The user has an existing annotation workflow:
1. Uses Zotero to store PDFs and create annotations
2. Uses Zotero Integration plugin in Obsidian to extract annotations into markdown
3. Has a custom template that uses 8 semantic colors (including yellow)
4. Wants to use different reading strategies that may need different color interpretations

Two active proposals depend on color standardization:
- **add-reading-strategies**: Needs to know which colors to use for each methodology
- **add-note-synthesis**: Needs to parse colors from Obsidian markdown files

## Problem Statement

### Color Inventory

**Current system implementation** (7 colors):
```rust
pub enum HighlightColor {
    Section1,  // #2ea8e5 - Blue
    Section2,  // #a28ae5 - Purple  
    Section3,  // #e56eee - Magenta
    Positive,  // #5fb236 - Green
    Detail,    // #aaaaaa - Grey
    Negative,  // #ff6666 - Red
    Code,      // #f19837 - Orange
}
```

**User's Obsidian template** (8 colors):
```
Blue     #2ea8e5 - section 1 (creates ## heading)
Purple   #a28ae5 - section 2 (creates ### heading)
Magenta  #e56eee - section 3 (creates #### heading)
Green    #5fb236 - positive point
Grey     #aaaaaa - point detail
Red      #ff6666 - negative point
Orange   #f19837 - code (creates ``` block)
Yellow   #ffd400 - not used yet ← MISSING FROM SYSTEM
```

### Workflow Questions

**Current plan** (add-note-synthesis):
```
Zotero (create annotations with /read)
    ↓ Zotero Integration plugin (manual/automatic)
Obsidian annotation.md files
    ↓ AI reads annotations (/synthesize)
Obsidian synthesis notes
```

**Alternative approach** (user's question):
```
Zotero (create annotations with /read)
    ↓ AI writes directly to Obsidian during /read
Obsidian annotation.md files (created by AI)
    ↓ Skip manual import step
```

**Should we change the workflow?**

## Design Decisions

### Decision 1: Add Yellow as 8th Color ✅

**Choice**: Add `HighlightColor::Question` with hex `#ffd400`

**Alternatives considered**:
1. ❌ Keep 7 colors, ask user to remove yellow from template
2. ❌ Add yellow but leave semantic meaning undefined
3. ✅ Add yellow with `Question` semantic meaning

**Rationale**:
- User already has yellow in template (no breaking changes)
- SQ3R reading strategy explicitly needs question marking
- "Question" is a natural 8th semantic category (complements Detail, Positive, Negative)
- Provides flexibility for future strategies
- Low implementation cost (add one enum variant)

**Implementation**:
```rust
pub enum HighlightColor {
    Section1,  // #2ea8e5 - Blue - Section 1 / Primary organization
    Section2,  // #a28ae5 - Purple - Section 2 / Secondary organization
    Section3,  // #e56eee - Magenta - Section 3 / Tertiary organization
    Positive,  // #5fb236 - Green - Positive point / Agreement
    Detail,    // #aaaaaa - Grey - Point detail / Context
    Negative,  // #ff6666 - Red - Negative point / Criticism
    Code,      // #f19837 - Orange - Code / Technical content
    Question,  // #ffd400 - Yellow - Question / Uncertainty / Needs clarification
}
```

**Color mapping table** (for documentation):
| Color | Hex | Enum | Primary Use | Alternative Uses |
|-------|-----|------|-------------|------------------|
| Blue | #2ea8e5 | `section1` | Main thesis, primary structure | Premises (analytical), themes (skim) |
| Purple | #a28ae5 | `section2` | Supporting arguments | Conclusions (analytical), sub-themes |
| Magenta | #e56eee | `section3` | Methodology, framework | Evidence types, categories |
| Green | #5fb236 | `positive` | Agreement, strong evidence | Answers (SQ3R), supported claims (review) |
| Grey | #aaaaaa | `detail` | Background, definitions | Methodology details, context |
| Red | #ff6666 | `negative` | Critique, weak points | Confusion (SQ3R), limitations (review) |
| Orange | #f19837 | `code` | Code, formulas | Technical specs, algorithms |
| Yellow | #ffd400 | `question` | Questions, uncertainties | Areas needing follow-up, open questions |

### Decision 2: Strategy-Specific Color Documentation

**Choice**: Each reading strategy documents its own color usage

**Approach**: Add "Color Scheme" section to each `/read-*` command file

**Example** (SQ3R strategy):
```markdown
## Color Scheme for SQ3R

This strategy uses colors to represent different phases:

| Color | SQ3R Use |
|-------|----------|
| `section1` (Blue) | Chapter/section headings during Survey |
| `section2` (Purple) | Subsection structure during Survey |
| `question` (Yellow) | Questions generated during Question phase |
| `positive` (Green) | Answers found during Read phase |
| `detail` (Grey) | Supporting details and examples |
| `negative` (Red) | Confusing passages or contradictions |
| `code` (Orange) | Formulas or technical content |
| `section3` (Magenta) | Not used in SQ3R |
```

**Example** (Literature Review strategy):
```markdown
## Color Scheme for Literature Review

This strategy focuses on evidence evaluation:

| Color | Review Use |
|-------|-----------|
| `positive` (Green) | Supported claims, strong evidence |
| `negative` (Red) | Limitations, critiques, weak evidence |
| `detail` (Grey) | Methodology, sample size, context |
| `section1` (Blue) | Research questions |
| `section2` (Purple) | Key findings |
| `code` (Orange) | Statistical results, equations |
| `question` (Yellow) | Gaps in literature, future work |
| `section3` (Magenta) | Theoretical framework |
```

**Benefits**:
- Each strategy optimizes color usage for its purpose
- Colors are reused flexibly across strategies
- Users can choose strategy based on their goals
- Documentation is clear and strategy-specific

### Decision 3: Keep Reading and Synthesis Separate ✅

**Choice**: Maintain two-phase workflow (no changes to add-note-synthesis plan)

**Reading Phase** (`/read`):
- Focuses on understanding ONE document deeply
- Creates annotations in Zotero
- No Obsidian interaction required
- Can be done anywhere (no vault dependency)

**Synthesis Phase** (`/synthesize`):
- Focuses on combining insights from MULTIPLE documents
- Reads annotations from Obsidian (after user has exported from Zotero)
- Creates synthesis notes in Obsidian
- Requires vault access

**Why separate**:
1. **Single responsibility**: Reading focuses on comprehension; synthesis on integration
2. **User control**: User can review/edit Zotero annotations before synthesis
3. **Flexibility**: Can synthesize later, across different document sets
4. **Compatibility**: Works with existing Zotero Integration plugin
5. **No vault dependency during reading**: Reading works anywhere, synthesis requires vault

**Workflow**:
```
┌─────────────┐
│   /read     │ ← AI-assisted critical reading
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Zotero    │ ← Annotations stored here (source of truth)
└──────┬──────┘
       │
       ▼ (User triggers Zotero Integration plugin)
┌─────────────┐
│  Obsidian   │ ← Annotation markdown files
│ annotations │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ /synthesize │ ← AI reads annotations, creates synthesis
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Obsidian   │ ← Synthesis notes (cross-document insights)
│  synthesis  │
└─────────────┘
```

**Future enhancement** (not in this change):
- Could add `--write-obsidian` flag to `/read` for direct Obsidian export
- Would require additional configuration (vault path, template)
- Would be opt-in, not default behavior

## Implementation Plan

### Phase 1: Add Yellow Color

**Files to modify**:
1. `crates/zotero-client/src/types.rs`:
   - Add `HighlightColor::Question` variant
   - Add hex mapping `#ffd400`
   - Add description `"Question / Uncertainty / Needs clarification"`

2. `crates/zotero-mcp/src/tools.rs`:
   - Add `HighlightColorParam::Question` variant
   - Add conversion `Question => HighlightColor::Question`

3. `crates/zotero-client/src/tests.rs`:
   - Add test for yellow color conversion
   - Add test for yellow serialization/deserialization

**Validation**:
- `cargo test --workspace` passes
- `cargo clippy` passes
- MCP tool schema includes `question` as valid color value

### Phase 2: Update Reading Strategies

**Files to create/modify**:
1. `.opencode/command/read-sq3r.md`:
   - Add "Color Scheme for SQ3R" section
   - Document yellow for questions, green for answers

2. `.opencode/command/read-review.md`:
   - Add "Color Scheme for Literature Review" section
   - Document yellow for gaps/future work

3. `.opencode/command/read-analyze.md`:
   - Add "Color Scheme for Analytical Reading" section
   - Document color usage for argument analysis

4. `.opencode/command/read-skim.md`:
   - Add "Color Scheme for Skimming" section
   - Document minimal color usage (key points only)

**Each command file includes**:
```markdown
## Color Scheme for [Strategy]

| Color | Use in [Strategy] |
|-------|-------------------|
| ... | ... |
```

### Phase 3: Update Note Synthesis

**Files to modify** (in add-note-synthesis change):
1. `openspec/changes/add-note-synthesis/specs/obsidian-integration/spec.md`:
   - Add yellow to color mapping requirement
   - Update color table to include 8 colors

2. `openspec/changes/add-note-synthesis/design.md`:
   - Update color table to include yellow
   - Document `#ffd400 => question` mapping

**Parser implementation** (future):
```rust
fn hex_to_semantic_color(hex: &str) -> Option<SemanticColor> {
    match hex {
        "#2ea8e5" => Some(SemanticColor::Section1),
        "#a28ae5" => Some(SemanticColor::Section2),
        "#e56eee" => Some(SemanticColor::Section3),
        "#5fb236" => Some(SemanticColor::Positive),
        "#aaaaaa" => Some(SemanticColor::Detail),
        "#ff6666" => Some(SemanticColor::Negative),
        "#f19837" => Some(SemanticColor::Code),
        "#ffd400" => Some(SemanticColor::Question),  // ← NEW
        _ => None,
    }
}
```

## Trade-offs

### Adding Yellow

**Pros**:
- ✅ Matches user's existing template (no breaking changes)
- ✅ Provides semantic slot for questions/uncertainties
- ✅ Enables richer reading strategies (especially SQ3R)
- ✅ Low implementation cost

**Cons**:
- ⚠️ Increases cognitive load slightly (8 colors vs 7)
- ⚠️ Some strategies may not use all 8 colors

**Mitigation**: Each strategy documents which colors it uses, so users aren't overwhelmed

### Keeping Separate Phases

**Pros**:
- ✅ Clear separation of concerns (read vs synthesize)
- ✅ User controls when synthesis happens
- ✅ No Obsidian vault dependency during reading
- ✅ Works with existing Zotero Integration workflow
- ✅ Can synthesize across multiple documents later

**Cons**:
- ⚠️ Requires manual Zotero Integration plugin trigger
- ⚠️ Extra step in workflow

**Mitigation**: User can configure Zotero Integration plugin to auto-sync, making it nearly automatic

## Open Questions for User

Before finalizing:

1. **Yellow semantic meaning**: Is "Question" the right name, or prefer "Warning", "Attention", "Unclear"?

2. **Template changes needed**: User's template already has yellow defined but unused. Should we:
   - Document yellow usage in template comments?
   - Leave it for user to assign meaning?
   - Provide example usage?

3. **Auto-sync preference**: Does user have Zotero Integration plugin set to auto-sync, or manual trigger?
   - If auto-sync: workflow is nearly seamless
   - If manual: consider documenting the sync step clearly

4. **Future direct-to-Obsidian**: Interest in `/read --write-obsidian` flag in the future?
   - Would bypass Zotero Integration plugin
   - Would create annotation files directly in vault
   - More complex, but eliminates manual step
