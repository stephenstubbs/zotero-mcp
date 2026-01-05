# Image Extraction Tools Usage Guide

## Quick Start

The Zotero MCP server provides three tools for PDF image extraction. All tools return **file paths** instead of base64 data.

## Tools

### 1. `zotero_extract_page_image`

Render a full page or specific region as an image file.

**Parameters:**
- `attachment_key` (required): Zotero attachment key (e.g., "B8YU42RN")
- `page` (required): 1-based page number
- `rect` (optional): Region `[x1, y1, x2, y2]` in PDF coordinates
- `dpi` (optional): Resolution (default: 150)
- `format` (optional): "png" or "jpeg" (default: "png")

**Example: Full page**
```json
{
  "name": "zotero_extract_page_image",
  "arguments": {
    "attachment_key": "B8YU42RN",
    "page": 1,
    "format": "jpeg"
  }
}
```

**Example: Specific region**
```json
{
  "name": "zotero_extract_page_image",
  "arguments": {
    "attachment_key": "B8YU42RN",
    "page": 3,
    "rect": [100, 200, 400, 500],
    "dpi": 200,
    "format": "png"
  }
}
```

**Response:**
```json
{
  "file_path": "/tmp/zotero-page-B8YU42RN-1-1704459600.jpg",
  "mime_type": "image/jpeg"
}
```

### 2. `zotero_list_figures`

Detect figures, charts, diagrams, and images on a page.

**Parameters:**
- `attachment_key` (required): Zotero attachment key
- `page` (required): 1-based page number

**Example:**
```json
{
  "name": "zotero_list_figures",
  "arguments": {
    "attachment_key": "B8YU42RN",
    "page": 5
  }
}
```

**Response:**
```json
[
  {
    "index": 0,
    "rect": [72.0, 200.5, 523.0, 680.3],
    "figure_type": "chart",
    "confidence": 0.85,
    "width": 451.0,
    "height": 479.8
  },
  {
    "index": 1,
    "rect": [72.0, 720.0, 300.0, 900.0],
    "figure_type": "image",
    "confidence": 0.92,
    "width": 228.0,
    "height": 180.0
  }
]
```

### 3. `zotero_get_figure`

Extract a specific detected figure as an image file.

**Parameters:**
- `attachment_key` (required): Zotero attachment key
- `page` (required): 1-based page number
- `figure_index` (required): Index from `zotero_list_figures` (0-based)
- `format` (optional): "png" or "jpeg" (default: "png")
- `include_context` (optional): Add padding around figure (default: false)

**Example:**
```json
{
  "name": "zotero_get_figure",
  "arguments": {
    "attachment_key": "B8YU42RN",
    "page": 5,
    "figure_index": 0,
    "format": "jpeg",
    "include_context": true
  }
}
```

**Response:**
```json
{
  "file_path": "/tmp/zotero-figure-B8YU42RN-p5-f0-1704459700.jpg",
  "mime_type": "image/jpeg"
}
```

## Typical Workflow

### Analyzing a specific page

1. **List figures** on the page:
   ```json
   {"name": "zotero_list_figures", "arguments": {"attachment_key": "ABC123", "page": 10}}
   ```

2. **Extract specific figure** by index:
   ```json
   {"name": "zotero_get_figure", "arguments": {"attachment_key": "ABC123", "page": 10, "figure_index": 0}}
   ```

3. **Read the image file** and analyze with vision AI

### Extracting a known region

If you already know the coordinates (e.g., from previous analysis):

```json
{
  "name": "zotero_extract_page_image",
  "arguments": {
    "attachment_key": "ABC123",
    "page": 7,
    "rect": [150, 300, 450, 600],
    "format": "jpeg",
    "dpi": 200
  }
}
```

## Tips

### DPI Selection

- **72-100 DPI**: Fast, small files, good for quick previews
- **150 DPI** (default): Balanced quality and file size
- **200-300 DPI**: High quality for detailed analysis or OCR

### Format Selection

- **PNG**: Lossless, larger files (~5x bigger than JPEG), best for diagrams with text
- **JPEG**: Lossy compression, smaller files, good for photos and complex images

### Coordinate System

Rectangles use PDF coordinates:
- Origin is **bottom-left** corner of page
- Format: `[x1, y1, x2, y2]` where:
  - `x1, y1` = bottom-left corner of region
  - `x2, y2` = top-right corner of region
- Typical US Letter page: 612 points wide × 792 points tall (72 points = 1 inch)

### File Cleanup

Temporary files are created in the system temp directory (`/tmp/` on Unix) and will be automatically cleaned up by the OS. Files are uniquely named with timestamps to avoid conflicts.

## Finding Attachment Keys

Use `zotero_lookup` with BetterBibTeX citation keys:

```json
{
  "name": "zotero_lookup",
  "arguments": {
    "citekey": "smithMachineLearning2023"
  }
}
```

Response includes `pdf_attachments` array with keys.

## Error Handling

Common errors:

- **"PDF not found"**: Invalid `attachment_key`
- **"Failed to load page"**: Page number out of range
- **"Figure X not found"**: Invalid `figure_index` - check `zotero_list_figures` first
- **"Region has zero width or height"**: Invalid `rect` coordinates
