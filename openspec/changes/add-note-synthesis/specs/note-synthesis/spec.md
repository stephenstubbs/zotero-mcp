## ADDED Requirements

### Requirement: Annotation Reading Tool
The MCP server SHALL expose a tool `obsidian_read_annotations` for parsing annotation files.

#### Scenario: Read annotations by citekey
- **WHEN** `obsidian_read_annotations` is called with a citekey
- **THEN** it returns all parsed annotations from that document's file

#### Scenario: Filter by color
- **WHEN** `obsidian_read_annotations` is called with `colors: ["positive", "negative"]`
- **THEN** only annotations of those semantic colors are returned

#### Scenario: Include metadata
- **WHEN** annotations are read
- **THEN** the response includes: citekey, title, file_path, and annotation list

#### Scenario: Annotation structure
- **WHEN** annotations are returned
- **THEN** each includes: type, color, color_hex, text, comment, page, heading_level

#### Scenario: File not found
- **WHEN** no annotation file exists for the citekey
- **THEN** an error indicates the citekey was not found in the vault

### Requirement: Note Writing Tool
The MCP server SHALL expose a tool `obsidian_write_note` for creating notes.

#### Scenario: Create note with content
- **WHEN** `obsidian_write_note` is called with path and content
- **THEN** a markdown file is created at that path

#### Scenario: Include frontmatter
- **WHEN** `obsidian_write_note` is called with frontmatter object
- **THEN** YAML frontmatter is prepended to the content

#### Scenario: Dataview-compatible output
- **WHEN** frontmatter includes `type`, `sources`, `created`
- **THEN** the frontmatter is formatted for Dataview queries

### Requirement: File Listing Tool
The MCP server SHALL expose a tool `obsidian_list_annotation_files`.

#### Scenario: List all annotation files
- **WHEN** `obsidian_list_annotation_files` is called
- **THEN** it returns all annotation files with their citekeys and titles

#### Scenario: Filter by tags
- **WHEN** `tags: ["machine-learning"]` is specified
- **THEN** only files with matching tags are returned

#### Scenario: Filter by folder
- **WHEN** `folder: "References"` is specified
- **THEN** only files in that folder are searched

### Requirement: Summarize Command
The system SHALL provide a `/summarize` command for single-document summaries.

#### Scenario: Generate summary from Obsidian
- **WHEN** `/summarize citekey` is invoked
- **THEN** annotations are read from Obsidian and synthesized into a summary note

#### Scenario: Summary structure
- **WHEN** a summary is generated
- **THEN** it groups annotations by semantic color with section headers

#### Scenario: Wikilinks to source
- **WHEN** a summary references the source
- **THEN** it includes `[[@citekey]]` wikilinks

#### Scenario: Page references as links
- **WHEN** annotations include page numbers
- **THEN** they link to `[[citekey#p. X|p. X]]`

### Requirement: Synthesize Command
The system SHALL provide a `/synthesize` command for multi-document synthesis.

#### Scenario: Synthesize multiple documents
- **WHEN** `/synthesize citekey1 citekey2 citekey3` is invoked
- **THEN** annotations from all documents are gathered and synthesized

#### Scenario: Theme-focused synthesis
- **WHEN** `/synthesize ... theme:"methodology"` is specified
- **THEN** synthesis focuses on annotations related to that theme

#### Scenario: Source attribution
- **WHEN** synthesis includes findings from multiple sources
- **THEN** each finding links to its source document

#### Scenario: Synthesis frontmatter
- **WHEN** a synthesis note is created
- **THEN** frontmatter includes `type: synthesis`, `sources: [...]`, `created: date`

#### Scenario: Output location
- **WHEN** synthesis note is created
- **THEN** it is placed in the configured synthesis folder

### Requirement: Search Tool
The MCP server SHALL expose a tool `obsidian_search` for vault-wide search.

#### Scenario: Search content
- **WHEN** `obsidian_search` is called with a query
- **THEN** it returns files containing that text

#### Scenario: Search within folder
- **WHEN** `folder` parameter is specified
- **THEN** search is limited to that folder
