# note-synthesis Specification

## Purpose
TBD - created by archiving change add-note-synthesis. Update Purpose after archive.
## Requirements
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

#### Scenario: Parse annotation file structure
- **WHEN** reading an annotation file
- **THEN** the YAML frontmatter is extracted (citekey, tags, status)
- **AND** the `[!Cite]` callout provides the primary citation
- **AND** the `[!Synth]` callout provides related citekeys
- **AND** the `[!md]` callout provides metadata (author, title, year, etc.)
- **AND** the `[!Abstract]` callout provides the abstract
- **AND** the `# Annotations` section contains all highlights

#### Scenario: Parse highlight annotation block
- **WHEN** parsing a highlight annotation
- **THEN** the color is extracted from `<mark style="background-color: #HEX">Highlight</mark>`
- **AND** section colors (blue/purple/magenta) have heading markers (`##`/`###`/`####`)
- **AND** the comment is extracted from bold text `**comment**`
- **AND** the highlighted text is extracted from quoted content `"text"`
- **AND** the page reference is extracted from `[@citekey p. X]` or `[@citekey pp. X-Y]`

#### Scenario: Parse code annotation block
- **WHEN** parsing an annotation with code color (`#f19837`)
- **THEN** the comment is used as the code fence language
- **AND** the highlighted text is inside the code fence
- **Example**: Comment `python` with text `print("hello")` becomes ` ```python\nprint("hello")\n``` `

#### Scenario: Parse image annotation
- **WHEN** parsing an image annotation
- **THEN** the image reference is extracted from `![[imageBaseName]]`
- **AND** the associated comment and page reference are captured

#### Scenario: Parse note annotation
- **WHEN** the annotation type is `Note` (not `Highlight`)
- **THEN** only the comment text is captured (no highlighted text)

#### Scenario: Parse plural page references
- **WHEN** an annotation spans multiple pages
- **THEN** the page reference may be `[@citekey pp. X-Y]` format
- **AND** both formats (`p.` and `pp.`) are recognized

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

### Requirement: Zettel Command
The system SHALL provide a `/readassist-permanent-note` slash command for extracting permanent notes from annotations.

#### Scenario: Extract IDEA-marked annotations only
- **WHEN** `/readassist-permanent-note <citekey>` is invoked
- **AND** the AI reads the annotation file from the Obsidian vault
- **THEN** it extracts only annotations with `IDEA:` comment prefix
- **AND** ignores annotations without the `IDEA:` prefix

#### Scenario: Create atomic permanent notes
- **WHEN** an `IDEA:` annotation is processed
- **THEN** a single permanent note file is created
- **AND** the note contains exactly one idea rewritten in the user's own words
- **AND** the note links back to the source using pandoc citation format

#### Scenario: Permanent note output location
- **WHEN** permanent notes are created
- **THEN** they are written to `Permanent/` folder in the vault
- **AND** filenames are based on the idea title (slugified)

#### Scenario: Preserve pandoc citations
- **WHEN** a permanent note references its source
- **THEN** it uses the original pandoc citation format `[@citekey p. X]`
- **AND** supports plural page references `[@citekey pp. X-Y]`

#### Scenario: Permanent note frontmatter
- **WHEN** a permanent note is created
- **THEN** frontmatter includes `type: permanent`, `source: "[@citekey p. X]"`, `created: date`

### Requirement: IDEA Comment Prefix
The annotation workflow SHALL support an `IDEA:` comment prefix for marking zettel-worthy annotations.

#### Scenario: IDEA prefix marks atomic ideas
- **WHEN** annotating with any semantic color
- **AND** the comment starts with `IDEA:`
- **THEN** that annotation is flagged for `/zettel` extraction

#### Scenario: IDEA prefix compatible with summarize
- **WHEN** `/summarize` processes an annotation with `IDEA:` prefix
- **THEN** it treats the annotation normally (prefix is part of comment text)
- **AND** the annotation appears in the appropriate color-based section

#### Scenario: IDEA prefix compatible with synthesize
- **WHEN** `/synthesize` processes an annotation with `IDEA:` prefix
- **THEN** it treats the annotation normally (prefix is part of comment text)
- **AND** the annotation contributes to theme identification and cross-document analysis

### Requirement: Pandoc Citation Format
All note synthesis commands SHALL use pandoc citation format for source references.

#### Scenario: Summarize uses pandoc citations
- **WHEN** `/summarize` generates a summary note
- **THEN** source references use pandoc format `[@citekey p. X]`
- **AND** multi-page references use `[@citekey pp. X-Y]`
- **AND** frontmatter source field uses `[@citekey]`

#### Scenario: Synthesize uses pandoc citations
- **WHEN** `/synthesize` generates a synthesis note
- **THEN** source references use pandoc format `[@citekey p. X]`
- **AND** multi-page references use `[@citekey pp. X-Y]`
- **AND** frontmatter sources array uses `[@citekey]` entries
- **AND** inline attributions use `[@citekey p. X]` format

#### Scenario: Zettel uses pandoc citations
- **WHEN** `/zettel` generates permanent notes
- **THEN** source references use pandoc format `[@citekey p. X]`
- **AND** multi-page references use `[@citekey pp. X-Y]`
- **AND** frontmatter source field uses `[@citekey p. X]`

### Requirement: Summarize Permanent Note Integration
The `/readassist-summarize` command SHALL extract permanent notes before generating summaries.

#### Scenario: Automatic permanent note extraction
- **WHEN** `/readassist-summarize <citekey>` is invoked
- **THEN** the AI first extracts permanent notes from `IDEA:` annotations
- **AND** then generates the summary note

#### Scenario: Permanent notes section in summary
- **WHEN** permanent notes are extracted during summarize
- **THEN** the summary includes a "Permanent Notes" section
- **AND** each note is listed with a wikilink `[[note-filename]]`
- **AND** each note shows its source reference

#### Scenario: Skip section when no IDEA annotations
- **WHEN** no `IDEA:` annotations are found
- **THEN** the "Permanent Notes" section is omitted from the summary

### Requirement: Synthesize Permanent Note Integration
The `/readassist-synthesize` command SHALL extract permanent notes from all sources before generating synthesis.

#### Scenario: Multi-source permanent note extraction
- **WHEN** `/readassist-synthesize <citekey1> <citekey2> ...` is invoked
- **THEN** the AI extracts permanent notes from each source's `IDEA:` annotations
- **AND** then generates the synthesis note

#### Scenario: Permanent notes section in synthesis
- **WHEN** permanent notes are extracted during synthesize
- **THEN** the synthesis includes a "Permanent Notes" section
- **AND** notes are grouped by source citekey
- **AND** each note is listed with a wikilink `[[note-filename]]`

#### Scenario: Skip section when no IDEA annotations across all sources
- **WHEN** no `IDEA:` annotations are found in any source
- **THEN** the "Permanent Notes" section is omitted from the synthesis

