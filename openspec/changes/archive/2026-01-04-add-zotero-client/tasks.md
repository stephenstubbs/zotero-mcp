## 1. Project Setup
- [x] 1.1 Create `crates/zotero-client/` directory structure with folder modules
- [x] 1.2 Add `zotero-client` to workspace members in root `Cargo.toml`
- [x] 1.3 Create `crates/zotero-client/Cargo.toml` with dependencies (reqwest, serde, thiserror, tokio)
- [x] 1.4 Create module structure: `src/lib.rs`, `src/error.rs`, `src/client.rs`, `src/types.rs`

## 2. Core Types
- [x] 2.1 Define `ZoteroItem` struct (id, key, itemType, title, creators, date, extra)
- [x] 2.2 Define `ZoteroAttachment` struct (id, key, title, contentType, path)
- [x] 2.3 Define `ZoteroAnnotation` struct (id, key, annotationType, text, comment, color, pageLabel, sortIndex, position)
- [x] 2.4 Define `AnnotationPosition` struct (pageIndex, rects)
- [x] 2.5 Define `CreateAnnotationRequest` struct for annotation creation
- [x] 2.6 Define `ZoteroClientError` enum with thiserror

## 3. HTTP Client Implementation
- [x] 3.1 Create `ZoteroClient` struct with base URL and reqwest client
- [x] 3.2 Implement `ZoteroClient::new()` with default localhost:23119 URL
- [x] 3.3 Implement `ping()` method to check plugin availability
- [x] 3.4 Implement `search_items(query, limit)` method
- [x] 3.5 Implement `list_items(limit)` method
- [x] 3.6 Implement `get_item(key)` method
- [x] 3.7 Implement `get_children(key)` method
- [x] 3.8 Implement `create_annotation(request)` method

## 4. PDF Text Extraction (Optional)
- [x] 4.1 Add optional PDF extraction feature flag (`pdf = ["pdf_oxide"]`)
- [x] 4.2 Implement `extract_text_with_positions(path, page)` function using pdf_oxide
- [x] 4.3 Implement coordinate conversion (pdf_oxide bbox → Zotero rects format)
- [x] 4.4 Implement `find_text_positions(path, page, search_text)` helper
- [x] 4.5 Implement `get_page_count(path)` utility
- [x] 4.6 Implement `extract_text(path, page)` for simple text extraction

## 5. Tests
- [x] 5.1 Create unit tests in `src/tests.rs`
- [x] 5.2 Add unit tests for type serialization/deserialization (12 tests)
- [x] 5.3 Add integration tests in `tests/integration.rs` with `integration` feature flag
- [x] 5.4 Add integration test for ping endpoint
- [x] 5.5 Add integration test for search and item retrieval
- [x] 5.6 Add integration test for annotation creation
- [x] 5.7 Add integration test for citation key lookup

## 6. Documentation
- [x] 6.1 Add rustdoc comments to all public types and methods
- [x] 6.2 Add usage examples in doc comments
- [x] 6.3 Add crate-level documentation with Quick Start guide
