## ADDED Requirements

### Requirement: Vault Configuration
The system SHALL support configuring the Obsidian vault location.

#### Scenario: Environment variable configuration
- **WHEN** `OBSIDIAN_VAULT_PATH` environment variable is set
- **THEN** the system uses that path as the vault root

#### Scenario: Config file configuration
- **WHEN** `.zotero-mcp.toml` contains `[obsidian] vault_path`
- **THEN** the system uses that path as the vault root

#### Scenario: Missing configuration
- **WHEN** no vault path is configured
- **THEN** Obsidian tools return an error indicating configuration is required

### Requirement: Annotation File Discovery
The system SHALL locate annotation files in the Obsidian vault.

#### Scenario: Find by citekey in frontmatter
- **WHEN** searching for annotations for a citekey
- **THEN** the system finds files with `citekey: <key>` in frontmatter

#### Scenario: Search within folder
- **WHEN** `OBSIDIAN_ANNOTATIONS_FOLDER` is configured
- **THEN** the system searches only within that folder

#### Scenario: List all annotation files
- **WHEN** `obsidian_list_annotation_files` is called
- **THEN** it returns all files with `category: Annotations` in frontmatter

### Requirement: Annotation Parsing
The system SHALL parse annotations from Zotero Integration template format.

#### Scenario: Parse highlight annotation
- **WHEN** parsing a highlight block with `<mark style="background-color: #xxx">`
- **THEN** it extracts color, text, comment, and page reference

#### Scenario: Parse section headings
- **WHEN** a blue/purple/magenta highlight creates a heading (##, ###, ####)
- **THEN** the heading level and comment are captured

#### Scenario: Parse code blocks
- **WHEN** an orange highlight creates a code block
- **THEN** the comment is captured as the language, text as code content

#### Scenario: Parse image annotations
- **WHEN** parsing an image annotation with `![[image.png]]`
- **THEN** the image path is captured

#### Scenario: Parse page references
- **WHEN** annotation contains `[@citekey p. X]`
- **THEN** the page number is extracted

### Requirement: Semantic Color Mapping
The system SHALL map hex colors to semantic names and categorize them by purpose.

#### Scenario: Hierarchy colors for document structure
- **WHEN** parsing annotations with section colors
- **THEN** colors are mapped to hierarchy category:
  - `#2ea8e5` → section1 (Blue) → H2 heading in Obsidian
  - `#a28ae5` → section2 (Purple) → H3 heading in Obsidian
  - `#e56eee` → section3 (Magenta) → H4 heading in Obsidian
- **AND** these annotations represent document structure, not semantic content types

#### Scenario: Semantic colors for content meaning
- **WHEN** parsing annotations with non-section colors
- **THEN** colors are mapped to semantic category:
  - `#5fb236` → positive (Green) → Thesis, evidence, claims, findings
  - `#aaaaaa` → detail (Grey) → Definitions, methodology, context
  - `#ff6666` → negative (Red) → Criticisms, weaknesses, concerns
  - `#f19837` → code (Orange) → Technical content, statistics
  - `#ffd400` → question (Yellow) → Questions, gaps, uncertainties
- **AND** these annotations represent content meaning

#### Scenario: Comment prefix extraction
- **WHEN** parsing annotations with comment prefixes (e.g., `THESIS:`, `Q:`, `WEAKNESS:`)
- **THEN** the system extracts prefix and content separately
- **AND** prefixes inform finer categorization within semantic colors

### Requirement: Structure-Aware Synthesis
The system SHALL use section color annotations to organize synthesis output structure.

#### Scenario: Section colors provide outline
- **WHEN** synthesizing annotations from a document
- **AND** the document has section1/2/3 annotations
- **THEN** the synthesis note uses those as organizational headings
- **AND** semantic color annotations are grouped under their nearest section heading

#### Scenario: Synthesis without section colors
- **WHEN** synthesizing annotations from a document
- **AND** the document has no section color annotations
- **THEN** the synthesis note groups annotations by semantic color
- **AND** each semantic color gets its own section (Key Points, Critical Notes, Questions, etc.)

#### Scenario: Multi-document synthesis uses common themes
- **WHEN** synthesizing annotations across multiple documents
- **THEN** the system identifies common themes from section headings
- **AND** groups related annotations from different sources under shared themes

### Requirement: Note Writing
The system SHALL create markdown notes in the Obsidian vault.

#### Scenario: Write note with frontmatter
- **WHEN** `obsidian_write_note` is called with frontmatter
- **THEN** YAML frontmatter is prepended to the content

#### Scenario: Write to synthesis folder
- **WHEN** `OBSIDIAN_SYNTHESIS_FOLDER` is configured
- **THEN** notes are created in that folder by default

#### Scenario: Overwrite existing note
- **WHEN** writing to an existing file path
- **THEN** the file is overwritten with new content

### Requirement: Wikilink Generation
The system SHALL generate Obsidian-compatible wikilinks.

#### Scenario: Link to annotation file
- **WHEN** referencing a source document
- **THEN** a wikilink `[[@citekey]]` is generated

#### Scenario: Link to specific page
- **WHEN** referencing a specific page in a source
- **THEN** a link `[[citekey#p. X|p. X]]` is generated

#### Scenario: Link in frontmatter sources
- **WHEN** listing sources in frontmatter
- **THEN** they are formatted as `"[[citekey]]"` strings
