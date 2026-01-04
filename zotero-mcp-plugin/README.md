# MCP Zotero API Plugin

A Zotero 7 plugin that exposes HTTP endpoints for external tools (like MCP servers) to create annotations and modify Zotero items while Zotero is running.

## Installation

1. Build the XPI: `./build.sh`
2. In Zotero, go to **Tools → Add-ons**
3. Click the gear icon and select **Install Add-on From File...**
4. Select the `mcp-zotero-api.xpi` file
5. Restart Zotero

## API Endpoints

All endpoints are available at `http://localhost:23119/mcp/...`

### GET /mcp/ping

Check if the plugin is active.

**Response:**
```json
{
  "status": "ok",
  "plugin": "mcp-zotero-api",
  "version": "1.0.0",
  "zoteroVersion": "7.0.x"
}
```

### POST /mcp/annotations

Create a new annotation on a PDF attachment.

**Request Body:**
```json
{
  "parentItemKey": "ABCD1234",
  "annotationType": "highlight",
  "text": "The highlighted text",
  "comment": "My note about this highlight",
  "color": "#ffd400",
  "pageLabel": "1",
  "sortIndex": "00000|000000|000000",
  "position": {
    "pageIndex": 0,
    "rects": [[100, 200, 300, 220]]
  }
}
```

**Required fields:**
- `parentItemKey`: The key of the PDF attachment item

**Optional fields:**
- `annotationType`: "highlight" (default), "note", "image", "ink", "underline"
- `text`: The text content of the annotation
- `comment`: A comment/note attached to the annotation
- `color`: Hex color code (default: "#ffd400" yellow)
- `pageLabel`: The page label/number
- `sortIndex`: Sort index for ordering
- `position`: Position data (JSON object)

**Response:**
```json
{
  "success": true,
  "annotation": {
    "id": 12345,
    "key": "WXYZ5678",
    "parentItemKey": "ABCD1234",
    "type": "highlight",
    "text": "The highlighted text",
    "color": "#ffd400",
    "pageLabel": "1"
  }
}
```

### GET /mcp/items?key=ABCD1234

Get item details by key.

**Response:**
```json
{
  "id": 123,
  "key": "ABCD1234",
  "itemType": "book",
  "title": "Example Book",
  "creators": [...],
  "attachments": [...]
}
```

### GET /mcp/search?q=query&limit=25

Search for items.

**Response:**
```json
{
  "results": [...],
  "total": 10
}
```

### GET /mcp/children?key=ABCD1234

Get child items (attachments, notes, annotations) for an item.

**Response:**
```json
{
  "parentKey": "ABCD1234",
  "children": [...]
}
```

## Usage with Python

```python
import requests
import json

BASE_URL = "http://localhost:23119"

# Check if plugin is active
response = requests.get(f"{BASE_URL}/mcp/ping")
print(response.json())

# Create a highlight annotation
annotation_data = {
    "parentItemKey": "PDF_ATTACHMENT_KEY",
    "annotationType": "highlight",
    "text": "Important text to highlight",
    "comment": "This is my note",
    "color": "#ffd400",
    "pageLabel": "1",
    "position": {
        "pageIndex": 0,
        "rects": [[100, 200, 300, 220]]
    }
}

response = requests.post(
    f"{BASE_URL}/mcp/annotations",
    headers={"Content-Type": "application/json"},
    data=json.dumps(annotation_data)
)
print(response.json())
```

## Development

The plugin is a bootstrapped Zotero 7 plugin using the standard WebExtension-style manifest.

### Files

- `manifest.json` - Plugin metadata
- `bootstrap.js` - Main plugin code with HTTP endpoint registration
- `icon.svg` - Plugin icon

### Building

```bash
./build.sh
```

This creates `mcp-zotero-api.xpi` which can be installed in Zotero.
