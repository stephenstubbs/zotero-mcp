# PDF Image Extraction Implementation

## Summary

Successfully implemented PDF image extraction for the Zotero MCP server with **file-based output** instead of base64 encoding to avoid massive MCP responses.

## Problem: Base64 Response Size

Initial implementation returned base64-encoded images in MCP responses, which was **completely impractical**:

| DPI | Format | File Size | Base64 Size | Too Large? |
|-----|--------|-----------|-------------|------------|
| 50  | PNG    | 421 KB    | 562 KB      | ✗ YES      |
| 50  | JPEG   | 76 KB     | 101 KB      | ✗ YES      |
| 72  | PNG    | 831 KB    | 1107 KB     | ✗ YES      |
| 72  | JPEG   | 146 KB    | 194 KB      | ✗ YES      |
| 150 | PNG    | 3407 KB   | 4544 KB     | ✗ YES      |

Even at 50 DPI JPEG (lowest quality), responses were **101 KB** - far too large for MCP tool responses which should be under 10-20 KB.

## Solution: File-Based Output

Instead of embedding images in responses, we now:

1. **Render image to temporary file** in `/tmp/`
2. **Return file path only** in MCP response
3. **Client reads file directly** from filesystem

### Response Size Comparison

| Approach | Response Size | Reduction |
|----------|---------------|-----------|
| Base64 (150 DPI PNG) | 4,544,000 bytes | - |
| File path | 66 bytes | **99.998%** ✓ |

## Implementation

### New Functions

Added to `crates/zotero-client/src/image/render.rs`:

```rust
/// Render a PDF page and save to file (returns file path)
pub fn render_page_to_file<P, O>(
    pdf_path: P,
    page_num: usize,
    dpi: u32,
    format: ImageFormat,
    output_path: O,
) -> Result<String>

/// Render a PDF region and save to file (returns file path)
pub fn render_region_to_file<P, O>(
    pdf_path: P,
    page_num: usize,
    rect: [f64; 4],
    dpi: u32,
    format: ImageFormat,
    output_path: O,
) -> Result<String>
```

### MCP Tool Changes

#### `zotero_extract_page_image`
- **Before:** Returned `{ data: "<4MB base64>", mime_type: "...", width: ..., height: ... }`
- **After:** Returns `{ file_path: "/tmp/zotero-page-KEY-PAGE-TIMESTAMP.ext", mime_type: "..." }`

#### `zotero_get_figure`
- **Before:** Returned base64-encoded figure image
- **After:** Returns file path to saved figure image

### Temp File Naming

Files are saved to system temp directory with unique names:

- **Pages:** `/tmp/zotero-page-{key}-{page}-{timestamp}.{ext}`
- **Figures:** `/tmp/zotero-figure-{key}-p{page}-f{index}-{timestamp}.{ext}`

### DPI Defaults

Since we're no longer constrained by response size, we can use higher quality:

- **Full pages:** 150 DPI (was 72 DPI in base64 version)
- **Figures:** 150 DPI (was 100 DPI in base64 version)

Users can still customize DPI via the `dpi` parameter.

## Usage Example

### MCP Tool Call

```json
{
  "name": "zotero_extract_page_image",
  "arguments": {
    "attachment_key": "B8YU42RN",
    "page": 1,
    "dpi": 150,
    "format": "jpeg"
  }
}
```

### Response

```json
{
  "file_path": "/tmp/zotero-page-B8YU42RN-1-1704459600.jpg",
  "mime_type": "image/jpeg"
}
```

### Claude Desktop Can Now

1. **Read the file directly** using filesystem access
2. **Analyze images at high resolution** (150+ DPI) without MCP size limits
3. **Process multiple pages** without overwhelming the response buffer

## Benefits

1. ✅ **Tiny responses** - 66 bytes vs 4+ MB
2. ✅ **Higher quality** - Can use 150+ DPI without size penalty
3. ✅ **No prompt length issues** - File paths don't consume context window
4. ✅ **Familiar pattern** - MCP clients already handle file references
5. ✅ **Simple cleanup** - Temp files auto-deleted by OS temp cleanup

## Files Modified

- `crates/zotero-client/src/image/render.rs` - Added `render_page_to_file()` and `render_region_to_file()`
- `crates/zotero-client/src/image/mod.rs` - Exported new functions
- `crates/zotero-mcp/src/image_tools.rs` - Updated to use file-based output
- `crates/zotero-mcp/src/server.rs` - Updated tool descriptions

## Testing

```bash
# Test file-based rendering
cargo run --package zotero-client --features image --example test_file_output

# Run all tests
cargo test --workspace
```

All 50 tests pass ✓

## Alternative Approaches Considered

1. **HTTP Server** - More complex, requires port management
2. **Lower DPI caps** - Still too large even at 50 DPI
3. **Aggressive compression** - Minimal benefit, complexity cost
4. **Chunked streaming** - Not standard MCP protocol

File-based approach is the simplest and most effective solution.
