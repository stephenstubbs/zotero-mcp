# Change: Add Zotero Client Library

## Why
The project needs a Rust client library to interact with the Zotero MCP plugin API. This enables the MCP server to search items, read PDF content, and create annotations in Zotero while it's running. A working Python prototype exists and needs to be ported to Rust.

## What Changes
- Add `zotero-client` crate with HTTP client for Zotero MCP plugin API
- Implement ping endpoint to check plugin availability
- Implement item search and listing (search by query, list all items)
- Implement item retrieval by key
- Implement children listing (attachments, notes, annotations)
- Implement annotation creation with position data for PDF highlights
- Add PDF text extraction with position information (for highlight coordinate calculation)

## Impact
- Affected specs: `zotero-client` (new capability)
- Affected code: New crate `crates/zotero-client/`
- Dependencies: `reqwest` (HTTP), `serde` (JSON), `thiserror` (errors), `tokio` (async runtime)
- Optional dependency: `pdf_oxide` (pure Rust PDF library) for text position extraction via `pdf` feature flag

## Implementation Notes
- Used `pdf_oxide` instead of `mupdf` for PDF extraction due to build issues with native C dependencies
- `pdf_oxide` is a pure Rust library that provides text extraction with bounding box positions
- The PDF feature is optional and enabled with `--features pdf`
