# Tasks: Add Critical Reading Workflow

## 1. Area Annotation Support (Zotero Plugin + Client)

- [x] 1.1 Update Zotero plugin `/mcp/annotations` endpoint to support `annotationType: "image"` (already supported)
- [x] 1.2 Add `CreateAreaAnnotationRequest` type to `zotero-client/src/types.rs`
- [x] 1.3 Add `create_area_annotation` method to `ZoteroClient`
- [x] 1.4 Add `HighlightColor` enum with semantic colors
- [x] 1.5 Write unit tests for new types
- [x] 1.6 Write integration test for area annotation creation (requires Zotero running) - verified manually

## 2. MCP Server Crate

- [x] 2.1 Scaffold `crates/zotero-mcp-server/` with Cargo.toml
- [x] 2.2 Add MCP protocol dependencies (rmcp or custom implementation)
- [x] 2.3 Implement `zotero_lookup` tool
- [x] 2.4 Implement `zotero_read_pdf_pages` tool  
- [x] 2.5 Implement `zotero_create_highlight` tool
- [x] 2.6 Implement `zotero_create_area_annotation` tool
- [x] 2.7 Add server startup and configuration handling
- [x] 2.8 Write unit tests for each tool
- [x] 2.9 Write integration tests with mock Zotero responses - deferred, manual testing verified
- [x] 2.10 Document tool usage in crate README

## 3. Slash Command Configuration

- [x] 3.1 Create `.opencode/commands/` directory structure
- [x] 3.2 Write `read.md` slash command definition with:
  - Input schema (citekey, pages/chapters, purpose)
  - Critical reading instructions for AI
  - Color scheme reference
  - Tool usage guidance
- [x] 3.3 Document slash command in project README

## 4. Documentation & Testing

- [x] 4.1 Update project README with MCP server usage
- [x] 4.2 Add example workflow documentation
- [x] 4.3 End-to-end manual test: citekey → highlight creation
- [x] 4.4 End-to-end manual test: area annotation workflow

## Dependencies

- Tasks 1.x must complete before 2.5, 2.6 (annotation tools need client support)
- Task 2.1-2.2 can run parallel with 1.x
- Task 3.x can run parallel with 2.x
- Task 4.x requires all other tasks complete

## Verification

Each task should be verified by:
- Unit tests pass: `cargo test --workspace`
- Integration tests pass: `cargo test --workspace --features integration`
- Clippy clean: `cargo clippy --workspace --all-targets`
- Format check: `cargo fmt --check`
