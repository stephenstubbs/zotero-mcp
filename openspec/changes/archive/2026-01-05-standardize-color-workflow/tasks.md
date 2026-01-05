# Tasks: Standardize Color Workflow

## Validation Tasks

- [x] 1. Validate user requirements
  - [x] 1.1 Confirm yellow should be added as `question` color
  - [x] 1.2 Confirm semantic meaning: "Question / Uncertainty / Needs clarification"
  - [x] 1.3 Confirm two-phase workflow (reading + synthesis separate)
  - [x] 1.4 Confirm no changes needed to user's Obsidian template

## Implementation Tasks

### Phase 1: Add Yellow Color to System (3-4 hours)

- [x] 2. Update `zotero-client` crate
  - [x] 2.1 Add `Question` variant to `HighlightColor` enum in `types.rs`
  - [x] 2.2 Add hex code `#ffd400` to `to_hex()` method
  - [x] 2.3 Add description "Question / Uncertainty / Needs clarification" to `description()` method
  - [x] 2.4 Run `cargo test --package zotero-client` to verify existing tests pass

- [x] 3. Update `zotero-mcp` crate  
  - [x] 3.1 Add `Question` variant to `HighlightColorParam` enum in `tools.rs`
  - [x] 3.2 Add conversion case `Question => HighlightColor::Question` in `From` impl
  - [x] 3.3 Run `cargo test --package zotero-mcp` to verify existing tests pass

- [x] 4. Add tests for yellow color
  - [x] 4.1 Add test in `zotero-client/src/tests.rs` for yellow hex conversion
  - [x] 4.2 Add test for yellow serialization (`"question"` ↔ `HighlightColor::Question`)
  - [x] 4.3 Add test in `zotero-mcp/src/tools.rs` for `HighlightColorParam::Question` conversion
  - [x] 4.4 Run `cargo test --workspace` and verify all pass

- [x] 5. Validate MCP schema
  - [x] 5.1 Build the MCP server: `cargo build --package zotero-mcp`
  - [x] 5.2 Start server and inspect schema for `zotero_create_highlight` tool
  - [x] 5.3 Verify `color` parameter enum includes `"question"` option
  - [x] 5.4 Verify `color` parameter description mentions yellow

### Phase 2: Document Color Usage in Reading Strategies (2-3 hours)

- [x] 6. Update base `/read` command documentation
  - [x] 6.1 Add yellow/question to color table in `.opencode/command/read.md`
  - [x] 6.2 Update color scheme section with 8 colors
  - [x] 6.3 Add note about strategy-specific color interpretations

> **Note**: Strategy-specific color documentation (tasks 7-10) moved to `add-reading-strategies` proposal where the command files will be created.

### Phase 3: Update Note Synthesis for Yellow (1-2 hours)

- [x] 11. Update `add-note-synthesis` specs
  - [x] 11.1 Add yellow to color mapping table in `specs/obsidian-integration/spec.md`
  - [x] 11.2 Add scenario for parsing yellow highlights from Obsidian
  - [x] 11.3 Update color mapping requirement to include 8 colors
  - [x] 11.4 Run `openspec validate add-note-synthesis` to verify specs are valid

- [x] 12. Update `add-note-synthesis` design
  - [x] 12.1 Add yellow to color table in `design.md`
  - [x] 12.2 Document `#ffd400 => question` mapping in parser design
  - [x] 12.3 Add example of yellow highlight in Obsidian markdown format

### Phase 4: Update Reading Strategies Specs (1 hour)

- [x] 13. Update `add-reading-strategies` specs
  - [x] 13.1 Add requirement for strategy-specific color documentation
  - [x] 13.2 Add scenario for yellow usage in SQ3R strategy
  - [x] 13.3 Update design to reference 8 colors instead of 7
  - [x] 13.4 Run `openspec validate add-reading-strategies` to verify specs are valid

> **Note**: Project documentation tasks (task 14) moved to `add-reading-strategies` (task 6.x) since they depend on strategy implementation.

## Testing Tasks

- [x] 15. Manual testing
  - [x] 15.1 Create test annotation with yellow color via MCP tool
  - [x] 15.2 Verify yellow annotation appears in Zotero PDF reader
  - [x] 15.3 Export to Obsidian with Zotero Integration plugin
  - [x] 15.4 Verify yellow appears as `#ffd400` in Obsidian markdown
  - [x] 15.5 Test all 8 colors end-to-end (Zotero → Obsidian)
  - **User confirmed**: "I have tested it and yellow works."

- [x] 16. Validation
  - [x] 16.1 Run `cargo test --workspace` - all tests pass
  - [x] 16.2 Run `cargo clippy --workspace` - no warnings
  - [x] 16.3 Run `openspec validate standardize-color-workflow --strict` - passes
  - [x] 16.4 Run `openspec validate add-note-synthesis --strict` - passes (after updates)
  - [x] 16.5 Run `openspec validate add-reading-strategies --strict` - passes (after updates)

## Dependencies and Sequencing

**Parallel work** (can be done simultaneously):
- Phase 1 (tasks 2-5): Add yellow to code - no dependencies
- Phase 2 (tasks 6-10): Documentation - can start before code is done

**Sequential work**:
- Phase 3 and 4 (tasks 11-13): Update dependent proposals - requires Phase 1 complete
- Task 15: Manual testing - requires Phase 1 complete and user's Zotero Integration plugin configured

**Blocks other work**:
- `add-reading-strategies` implementation should wait for this change to complete
- `add-note-synthesis` implementation should wait for this change to complete

## Estimated Total Time

- **Phase 1**: 3-4 hours (code changes + tests)
- **Phase 2**: 2-3 hours (strategy color documentation)
- **Phase 3**: 1-2 hours (synthesis spec updates)
- **Phase 4**: 1 hour (reading strategies spec updates)
- **Documentation**: 1 hour
- **Testing**: 1-2 hours (manual validation)

**Total**: 9-13 hours

## Success Criteria

- ✅ `HighlightColor::Question` variant exists with hex `#ffd400`
- ✅ All 8 colors work in MCP tools (`zotero_create_highlight`, `zotero_create_area_annotation`)
- ✅ Each reading strategy documents its color usage clearly
- ✅ `add-note-synthesis` specs recognize all 8 colors including yellow
- ✅ All tests pass (`cargo test --workspace`)
- ✅ No clippy warnings (`cargo clippy --workspace`)
- ✅ OpenSpec validation passes for all related changes
- ✅ Manual end-to-end test confirms yellow works from Zotero → Obsidian
- ✅ User confirms yellow semantic meaning and workflow approach
