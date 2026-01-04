# Tasks: Add PDF Image Extraction

## Prerequisites
- [ ] 0.1 Phase 1 (`add-critical-reading-workflow`) is complete and merged

## 1. Image Extraction Module

- [ ] 1.1 Add `image` feature flag to `zotero-client/Cargo.toml`
- [ ] 1.2 Create `crates/zotero-client/src/image/mod.rs` module structure
- [ ] 1.3 Implement `render_page` function using MuPDF pixmap rendering
- [ ] 1.4 Implement `render_region` function for partial page rendering
- [ ] 1.5 Implement `extract_embedded_images` for direct image extraction
- [ ] 1.6 Add PNG encoding support (using `image` crate or MuPDF)
- [ ] 1.7 Add JPEG encoding support with quality parameter
- [ ] 1.8 Write unit tests for image rendering
- [ ] 1.9 Write integration tests with sample PDFs

## 2. Figure Detection

- [ ] 2.1 Create `crates/zotero-client/src/image/detection.rs`
- [ ] 2.2 Implement whitespace analysis for figure boundary detection
- [ ] 2.3 Implement content density analysis
- [ ] 2.4 Add figure candidate filtering (size, aspect ratio)
- [ ] 2.5 Return figure regions as bounding boxes
- [ ] 2.6 Write unit tests for detection algorithm
- [ ] 2.7 Test with various academic paper layouts

## 3. MCP Server Tools

- [ ] 3.1 Implement `zotero_extract_page_image` tool
- [ ] 3.2 Implement `zotero_list_figures` tool
- [ ] 3.3 Implement `zotero_get_figure` tool
- [ ] 3.4 Add base64 encoding for image responses
- [ ] 3.5 Add DPI and format configuration
- [ ] 3.6 Write unit tests for each tool
- [ ] 3.7 Write integration tests with vision response format

## 4. Documentation

- [ ] 4.1 Document image extraction API in crate docs
- [ ] 4.2 Add usage examples for vision workflows
- [ ] 4.3 Update slash command with figure analysis guidance
- [ ] 4.4 Document figure detection limitations

## Dependencies

- Task 0.1 blocks all other tasks
- Tasks 1.x must complete before 3.x
- Tasks 2.x must complete before 3.2, 3.3
- Task 4.x can run parallel with testing

## Verification

- Unit tests pass: `cargo test --workspace --features image`
- Integration tests pass: `cargo test --workspace --features "integration image"`
- Image output verified manually with sample PDFs
- Base64 output tested with vision AI
