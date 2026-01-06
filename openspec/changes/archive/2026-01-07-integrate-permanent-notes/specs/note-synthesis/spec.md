## MODIFIED Requirements

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

## ADDED Requirements

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
