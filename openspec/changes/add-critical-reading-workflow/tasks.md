# Tasks: Add Critical Reading Workflow

## 1. Area Annotation Support (Zotero Plugin + Client)

- [ ] 1.1 Update Zotero plugin `/mcp/annotations` endpoint to support `annotationType: "image"`
- [ ] 1.2 Add `CreateAreaAnnotationRequest` type to `zotero-client/src/types.rs`
- [ ] 1.3 Add `create_area_annotation` method to `ZoteroClient`
- [ ] 1.4 Add `HighlightColor` enum with semantic colors
- [ ] 1.5 Write unit tests for new types
- [ ] 1.6 Write integration test for area annotation creation (requires Zotero running)

## 2. MCP Server Crate

- [ ] 2.1 Scaffold `crates/zotero-mcp-server/` with Cargo.toml
- [ ] 2.2 Add MCP protocol dependencies (rmcp or custom implementation)
- [ ] 2.3 Implement `zotero_lookup` tool
- [ ] 2.4 Implement `zotero_read_pdf_pages` tool  
- [ ] 2.5 Implement `zotero_create_highlight` tool
- [ ] 2.6 Implement `zotero_create_area_annotation` tool
- [ ] 2.7 Add server startup and configuration handling
- [ ] 2.8 Write unit tests for each tool
- [ ] 2.9 Write integration tests with mock Zotero responses
- [ ] 2.10 Document tool usage in crate README

## 3. Slash Command Configuration

- [ ] 3.1 Create `.opencode/commands/` directory structure
- [ ] 3.2 Write `read.md` slash command definition with:
  - Input schema (citekey, pages/chapters, purpose)
  - Critical reading instructions for AI
  - Color scheme reference
  - Tool usage guidance
- [ ] 3.3 Document slash command in project README

## 4. Documentation & Testing

- [ ] 4.1 Update project README with MCP server usage
- [ ] 4.2 Add example workflow documentation
- [ ] 4.3 End-to-end manual test: citekey → highlight creation
- [ ] 4.4 End-to-end manual test: area annotation workflow

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
