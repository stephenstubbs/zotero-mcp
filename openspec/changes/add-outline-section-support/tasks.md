# Tasks: Add Outline Section Support

## 1. PDF Outline Extraction

- [ ] 1.1 Add `OutlineItem` struct to `zotero-client/src/types.rs`
- [ ] 1.2 Implement `get_pdf_outline()` in `zotero-client/src/pdf.rs`
- [ ] 1.3 Add `resolve_section_to_pages()` helper function
- [ ] 1.4 Write unit tests for outline extraction
- [ ] 1.5 Write integration test with real PDF containing outline

## 2. MCP Server Tools

- [ ] 2.1 Implement `zotero_get_pdf_outline` tool
- [ ] 2.2 Modify `zotero_read_pdf_pages` to accept `section` parameter
- [ ] 2.3 Add section-to-page-range resolution in read tool
- [ ] 2.4 Write unit tests for new tool
- [ ] 2.5 Write integration tests for section reading

## 3. Slash Command Update

- [ ] 3.1 Update `/read` command with outline-first workflow
- [ ] 3.2 Add `from_page:N` parameter documentation
- [ ] 3.3 Add AI instructions for no-outline fallback
- [ ] 3.4 Document the workflow decision tree

## 4. Documentation

- [ ] 4.1 Update tool documentation with outline examples
- [ ] 4.2 Add example workflow for open questions reading
- [ ] 4.3 Document fallback behavior

## Dependencies

- Task 1.x must complete before 2.x (tools depend on client functions)
- Task 3.x can run parallel with 2.x
- Task 4.x requires all other tasks complete

## Verification

Each task should be verified by:
- Unit tests pass: `cargo test --workspace`
- Integration tests pass: `cargo test --workspace --features integration`
- Clippy clean: `cargo clippy --workspace --all-targets`
- Format check: `cargo fmt --check`
