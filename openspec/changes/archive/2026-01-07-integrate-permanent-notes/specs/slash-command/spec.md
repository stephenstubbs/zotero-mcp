## MODIFIED Requirements

### Requirement: Zettel Command Definition
The system SHALL provide a `/readassist-permanent-note` slash command for extracting permanent notes from IDEA-marked annotations.

#### Scenario: Invoke with citekey
- **WHEN** user invokes `/readassist-permanent-note smithML2023`
- **THEN** the AI extracts annotations with `IDEA:` prefix and creates permanent notes

#### Scenario: Invoke with output folder
- **WHEN** user invokes `/readassist-permanent-note smithML2023 output:Permanent/`
- **THEN** the AI creates permanent notes in the specified folder

### Requirement: Summarize Command Definition
The system SHALL provide a `/readassist-summarize` slash command for generating summary notes from annotations.

#### Scenario: Invoke with citekey
- **WHEN** user invokes `/readassist-summarize smithML2023`
- **THEN** the AI first extracts permanent notes from `IDEA:` annotations
- **AND** then generates a structured summary note
- **AND** the summary includes links to extracted permanent notes

#### Scenario: Invoke with vault path
- **WHEN** user invokes `/readassist-summarize smithML2023 vault:/path/to/vault`
- **THEN** the AI uses the specified vault path to locate the annotation file

#### Scenario: No IDEA annotations present
- **WHEN** user invokes `/readassist-summarize` on annotations without `IDEA:` prefix
- **THEN** permanent note extraction is skipped
- **AND** the "Permanent Notes" section is omitted from output

### Requirement: Synthesize Command Definition
The system SHALL provide a `/readassist-synthesize` slash command for multi-document synthesis.

#### Scenario: Invoke with multiple citekeys
- **WHEN** user invokes `/readassist-synthesize smithML2023 jonesAI2024 brownDeep2023`
- **THEN** the AI first extracts permanent notes from `IDEA:` annotations in each source
- **AND** then creates a synthesis note
- **AND** the synthesis includes links to extracted permanent notes grouped by source

#### Scenario: Invoke with theme
- **WHEN** user invokes `/readassist-synthesize smithML2023 jonesAI2024 theme:"methodology"`
- **THEN** the AI focuses the synthesis on the specified theme

#### Scenario: No IDEA annotations in any source
- **WHEN** user invokes `/readassist-synthesize` on sources without `IDEA:` annotations
- **THEN** permanent note extraction is skipped for those sources
- **AND** the "Permanent Notes" section is omitted if no notes were extracted
