# mcp-server Specification

## Purpose
TBD - created by archiving change add-critical-reading-workflow. Update Purpose after archive.
## Requirements
### Requirement: MCP Server Initialization
The system SHALL provide an MCP server binary that connects to a Zotero instance.

#### Scenario: Default connection
- **WHEN** the MCP server starts without configuration
- **THEN** it connects to Zotero at `http://localhost:23119/mcp`

#### Scenario: Custom Zotero URL
- **WHEN** the environment variable `ZOTERO_URL` is set
- **THEN** the server connects to that URL instead of the default

#### Scenario: Zotero not available
- **WHEN** the server starts and Zotero is not running
- **THEN** tools return appropriate errors when invoked

### Requirement: Zotero Lookup Tool
The system SHALL expose an MCP tool `zotero_lookup` to find items by BetterBibTeX citation key.

#### Scenario: Item found by citekey
- **WHEN** `zotero_lookup` is called with a valid citekey
- **THEN** it returns the item metadata including key, title, and PDF attachment keys

#### Scenario: Item not found
- **WHEN** `zotero_lookup` is called with a non-existent citekey
- **THEN** it returns an error indicating the item was not found

#### Scenario: Item has no PDF attachment
- **WHEN** `zotero_lookup` finds an item without PDF attachments
- **THEN** the response indicates no PDFs are available

### Requirement: PDF Page Reading Tool
The system SHALL expose an MCP tool `zotero_read_pdf_pages` to extract text from PDF pages.

#### Scenario: Read single page
- **WHEN** `zotero_read_pdf_pages` is called with `pages: "1"`
- **THEN** it returns the text content of page 1

#### Scenario: Read page range
- **WHEN** `zotero_read_pdf_pages` is called with `pages: "1-5"`
- **THEN** it returns text content of pages 1 through 5, clearly delimited

#### Scenario: Read all pages
- **WHEN** `zotero_read_pdf_pages` is called with `pages: "all"`
- **THEN** it returns text content of all pages in the PDF

#### Scenario: Invalid page number
- **WHEN** `zotero_read_pdf_pages` is called with a page number exceeding the PDF page count
- **THEN** it returns an error indicating the page is out of range

#### Scenario: PDF file not found
- **WHEN** the PDF file path from Zotero does not exist locally
- **THEN** it returns an error indicating the file is missing

#### Scenario: Read by section name
- **WHEN** `zotero_read_pdf_pages` is called with `section: "Introduction"`
- **THEN** it resolves the section name to a page range using the PDF outline
- **AND** returns text content from that section's pages

#### Scenario: Read multiple sections
- **WHEN** `zotero_read_pdf_pages` is called with `section: "Introduction,Methods"`
- **THEN** it resolves each section name to page ranges using the PDF outline
- **AND** returns combined text content from all specified sections
- **AND** each section's content is clearly delimited with section headers

#### Scenario: Section not found
- **WHEN** `zotero_read_pdf_pages` is called with `section: "NonExistent"`
- **THEN** it returns an error indicating the section was not found
- **AND** includes a list of available section names from the outline

#### Scenario: Section requested but no outline
- **WHEN** `zotero_read_pdf_pages` is called with `section` parameter
- **AND** the PDF has no outline
- **THEN** it returns an error indicating no outline exists
- **AND** suggests using `pages` parameter instead

### Requirement: Highlight Creation Tool
The system SHALL expose an MCP tool `zotero_create_highlight` for text highlights.

#### Scenario: Create highlight with semantic color
- **WHEN** `zotero_create_highlight` is called with text, page, and color "positive"
- **THEN** it creates a green (#5fb236) highlight on the matching text

#### Scenario: Create highlight with comment
- **WHEN** `zotero_create_highlight` is called with a comment
- **THEN** the highlight includes the comment

#### Scenario: Text not found on page
- **WHEN** the specified text is not found on the given page
- **THEN** it returns an error indicating the text was not found

#### Scenario: All semantic colors supported
- **WHEN** `zotero_create_highlight` is called with any of: section1, section2, section3, positive, detail, negative, code
- **THEN** it creates a highlight with the corresponding hex color

### Requirement: Area Annotation Tool
The system SHALL expose an MCP tool `zotero_create_area_annotation` for image/figure selection.

#### Scenario: Create area annotation
- **WHEN** `zotero_create_area_annotation` is called with page and rect coordinates
- **THEN** it creates an image annotation at the specified location

#### Scenario: Area annotation with semantic color
- **WHEN** `zotero_create_area_annotation` is called with color "code"
- **THEN** the area annotation uses orange (#f19837) color

#### Scenario: Area annotation with comment
- **WHEN** `zotero_create_area_annotation` is called with a comment
- **THEN** the annotation includes the comment

### Requirement: Semantic Color Mapping
The system SHALL map semantic color names to hex codes.

#### Scenario: Color mapping
- **WHEN** a tool receives a semantic color name
- **THEN** it maps to the correct hex code:
  - section1 → #2ea8e5 (Blue)
  - section2 → #a28ae5 (Purple)
  - section3 → #e56eee (Magenta)
  - positive → #5fb236 (Green)
  - detail → #aaaaaa (Grey)
  - negative → #ff6666 (Red)
  - code → #f19837 (Orange)

### Requirement: PDF Outline Retrieval Tool
The system SHALL expose an MCP tool `zotero_get_pdf_outline` to retrieve the document's table of contents.

#### Scenario: PDF has outline
- **WHEN** `zotero_get_pdf_outline` is called for a PDF with bookmarks
- **THEN** it returns the outline structure with titles and page numbers
- **AND** nested sections are represented as children

#### Scenario: PDF has no outline
- **WHEN** `zotero_get_pdf_outline` is called for a PDF without bookmarks
- **THEN** it returns `has_outline: false` with an empty items array
- **AND** the response indicates the user should provide page numbers

#### Scenario: Include page count
- **WHEN** `zotero_get_pdf_outline` is called
- **THEN** the response includes `total_pages` field
- **AND** AI can use this to estimate section lengths

