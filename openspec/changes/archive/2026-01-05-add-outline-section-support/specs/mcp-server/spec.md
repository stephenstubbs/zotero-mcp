## ADDED Requirements

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

## MODIFIED Requirements

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
