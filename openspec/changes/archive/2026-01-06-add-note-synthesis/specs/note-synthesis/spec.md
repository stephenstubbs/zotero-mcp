## ADDED Requirements

### Requirement: Summarize Command
The system SHALL provide a `/summarize` slash command for single-document summaries.

#### Scenario: Generate summary from Obsidian annotations
- **WHEN** `/summarize <citekey>` is invoked
- **AND** the AI reads the annotation file from the Obsidian vault
- **THEN** it parses annotations and generates a structured summary note

#### Scenario: Summary groups by semantic color
- **WHEN** a summary is generated
- **THEN** it groups annotations by semantic color (positive, negative, question, detail, code)
- **AND** section colors (section1/2/3) provide structural organization

#### Scenario: Wikilinks to source
- **WHEN** a summary references the source document
- **THEN** it includes `[[@citekey]]` wikilinks

#### Scenario: Page references as links
- **WHEN** annotations include page numbers
- **THEN** they are formatted as `[[citekey#p. X|p. X]]` links

#### Scenario: Dataview-compatible frontmatter
- **WHEN** a summary note is created
- **THEN** frontmatter includes `type: summary`, `source: "[[citekey]]"`, `created: date`

### Requirement: Synthesize Command
The system SHALL provide a `/synthesize` slash command for multi-document synthesis.

#### Scenario: Synthesize multiple documents
- **WHEN** `/synthesize <citekey1> <citekey2> ...` is invoked
- **THEN** annotations from all documents are gathered and synthesized

#### Scenario: Theme-focused synthesis
- **WHEN** `/synthesize ... theme:"<topic>"` is specified
- **THEN** synthesis focuses on annotations related to that theme

#### Scenario: Source attribution
- **WHEN** synthesis includes findings from multiple sources
- **THEN** each finding links to its source document with wikilinks

#### Scenario: Synthesis frontmatter
- **WHEN** a synthesis note is created
- **THEN** frontmatter includes `type: synthesis`, `sources: [...]`, `created: date`

### Requirement: Annotation Parsing
The slash commands SHALL parse Zotero Integration template format annotations.

#### Scenario: Parse highlight color
- **WHEN** parsing `<mark style="background-color: #xxx">Highlight</mark>`
- **THEN** the hex color is extracted and mapped to semantic name

#### Scenario: Parse section headings
- **WHEN** a blue/purple/magenta annotation creates a heading (##/###/####)
- **THEN** the heading level and comment text are captured

#### Scenario: Parse page references
- **WHEN** annotation contains `[@citekey p. X]`
- **THEN** the page number is extracted

#### Scenario: Parse comment prefixes
- **WHEN** a comment contains a prefix like `THESIS:`, `Q:`, `WEAKNESS:`
- **THEN** the prefix is extracted for finer categorization

### Requirement: Color Semantics
The slash commands SHALL interpret colors according to the established color scheme.

#### Scenario: Hierarchy colors for structure
- **WHEN** parsing section1/2/3 colors (blue/purple/magenta)
- **THEN** they are used for organizational structure in output
- **AND** they represent document headings, not content types

#### Scenario: Semantic colors for content
- **WHEN** parsing positive/negative/question/detail/code colors
- **THEN** they are used to group content by meaning
- **AND** they inform the synthesis output sections
