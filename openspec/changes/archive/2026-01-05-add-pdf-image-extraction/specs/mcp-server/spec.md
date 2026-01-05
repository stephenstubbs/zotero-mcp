## ADDED Requirements

### Requirement: Page Image Extraction Tool
The MCP server SHALL expose a tool `zotero_extract_page_image` for rendering pages.

#### Scenario: Extract full page as image
- **WHEN** `zotero_extract_page_image` is called with attachment_key and page
- **THEN** it returns the full page as a base64-encoded image

#### Scenario: Extract region as image
- **WHEN** `zotero_extract_page_image` is called with a rect parameter
- **THEN** it returns only the specified region as an image

#### Scenario: Custom DPI rendering
- **WHEN** `zotero_extract_page_image` is called with `dpi: 300`
- **THEN** the rendered image has higher resolution

#### Scenario: JPEG format output
- **WHEN** `zotero_extract_page_image` is called with `format: "jpeg"`
- **THEN** the response contains a JPEG image with smaller size

### Requirement: Figure Listing Tool
The MCP server SHALL expose a tool `zotero_list_figures` for figure detection.

#### Scenario: List detected figures
- **WHEN** `zotero_list_figures` is called for a page
- **THEN** it returns a list of figure regions with indices and bounding boxes

#### Scenario: Figure metadata
- **WHEN** figures are listed
- **THEN** each figure includes: index, bounding box, estimated type (image/chart/diagram)

#### Scenario: No figures found
- **WHEN** `zotero_list_figures` is called for a page without figures
- **THEN** it returns an empty list with a message

### Requirement: Figure Extraction Tool
The MCP server SHALL expose a tool `zotero_get_figure` for extracting specific figures.

#### Scenario: Extract figure by index
- **WHEN** `zotero_get_figure` is called with a figure_index from `zotero_list_figures`
- **THEN** it returns that figure as a base64-encoded image

#### Scenario: Invalid figure index
- **WHEN** `zotero_get_figure` is called with an index that doesn't exist
- **THEN** it returns an error indicating the figure was not found

#### Scenario: Figure with context
- **WHEN** `zotero_get_figure` is called with `include_context: true`
- **THEN** the extracted region includes some padding around the figure
