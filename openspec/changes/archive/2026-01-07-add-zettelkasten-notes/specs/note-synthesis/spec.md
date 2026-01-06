# note-synthesis Specification Delta

## ADDED Requirements

### Requirement: Zettel Command
The system SHALL provide a `/zettel` slash command for extracting permanent notes from annotations.

#### Scenario: Extract IDEA-marked annotations only
- **WHEN** `/zettel <citekey>` is invoked
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

## MODIFIED Requirements

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
