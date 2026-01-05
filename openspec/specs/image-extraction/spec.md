# image-extraction Specification

## Purpose
TBD - created by archiving change add-pdf-image-extraction. Update Purpose after archive.
## Requirements
### Requirement: Page Rendering
The system SHALL render PDF pages as images.

#### Scenario: Render full page
- **WHEN** `render_page` is called with a page number
- **THEN** it returns the full page as a PNG image

#### Scenario: Render with custom DPI
- **WHEN** `render_page` is called with `dpi: 300`
- **THEN** the output image has higher resolution than the default 150 DPI

#### Scenario: Render as JPEG
- **WHEN** `render_page` is called with `format: "jpeg"`
- **THEN** it returns a JPEG-encoded image

#### Scenario: Invalid page number
- **WHEN** `render_page` is called with a page number exceeding the PDF
- **THEN** it returns an error indicating the page is out of range

### Requirement: Region Rendering
The system SHALL render specific regions of PDF pages.

#### Scenario: Render rectangular region
- **WHEN** `render_region` is called with page and rect coordinates
- **THEN** it returns only the specified rectangular area as an image

#### Scenario: Region extends beyond page
- **WHEN** the specified rect extends beyond page boundaries
- **THEN** the region is clipped to page bounds

### Requirement: Embedded Image Extraction
The system SHALL extract images embedded in PDFs.

#### Scenario: Extract all images from page
- **WHEN** `extract_embedded_images` is called for a page
- **THEN** it returns a list of images with their positions and data

#### Scenario: Page has no images
- **WHEN** `extract_embedded_images` is called for a text-only page
- **THEN** it returns an empty list

### Requirement: Figure Detection
The system SHALL detect figure regions in PDF pages.

#### Scenario: Detect figures on page
- **WHEN** `detect_figures` is called for a page with figures
- **THEN** it returns bounding boxes for detected figure regions

#### Scenario: No figures detected
- **WHEN** `detect_figures` is called for a text-only page
- **THEN** it returns an empty list

#### Scenario: Figure detection includes charts and diagrams
- **WHEN** a page contains charts, diagrams, or graphs
- **THEN** they are detected as figure regions

### Requirement: Base64 Output
The system SHALL provide base64-encoded image output.

#### Scenario: Base64 encoding
- **WHEN** an image is rendered
- **THEN** it can be returned as a base64-encoded string

#### Scenario: Include MIME type
- **WHEN** base64 output is generated
- **THEN** the MIME type (image/png or image/jpeg) is included

