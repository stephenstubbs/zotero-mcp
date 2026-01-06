## MODIFIED Requirements

### Requirement: Read Command Definition
The system SHALL provide a `/readassist-read` slash command for initiating critical reading workflows.

#### Scenario: Invoke with citekey and pages
- **WHEN** user invokes `/readassist-read smithML2023 pages:1-10 purpose:"understand methodology"`
- **THEN** the AI receives the command parameters and critical reading instructions

#### Scenario: Invoke with citekey and chapters
- **WHEN** user invokes `/readassist-read smithML2023 chapters:"Introduction,Methods"`
- **THEN** the AI receives the chapter names to look up in the PDF

#### Scenario: Invoke with citekey only
- **WHEN** user invokes `/readassist-read smithML2023`
- **THEN** the AI is instructed to read the full document

#### Scenario: Invoke with strategy parameter
- **WHEN** user invokes `/readassist-read smithML2023 strategy:sq3r`
- **THEN** the AI uses the SQ3R reading methodology instead of default critical reading

#### Scenario: List available strategies
- **WHEN** user invokes `/readassist-read --help` or requests strategy list
- **THEN** the available strategies are displayed: critical, sq3r, review, analyze, skim

### Requirement: Strategy-Specific Commands
The system SHALL provide dedicated commands for each reading strategy.

#### Scenario: SQ3R command
- **WHEN** user invokes `/readassist-read-sq3r citekey`
- **THEN** it is equivalent to `/readassist-read citekey strategy:sq3r`

#### Scenario: Review command
- **WHEN** user invokes `/readassist-read-review citekey`
- **THEN** it is equivalent to `/readassist-read citekey strategy:review`

#### Scenario: Analyze command
- **WHEN** user invokes `/readassist-read-analyze citekey`
- **THEN** it is equivalent to `/readassist-read citekey strategy:analyze`

#### Scenario: Skim command
- **WHEN** user invokes `/readassist-read-skim citekey`
- **THEN** it is equivalent to `/readassist-read citekey strategy:skim`

## ADDED Requirements

### Requirement: Summarize Command Definition
The system SHALL provide a `/readassist-summarize` slash command for generating summary notes from annotations.

#### Scenario: Invoke with citekey
- **WHEN** user invokes `/readassist-summarize smithML2023`
- **THEN** the AI reads the Obsidian annotation file and generates a structured summary note

#### Scenario: Invoke with vault path
- **WHEN** user invokes `/readassist-summarize smithML2023 vault:/path/to/vault`
- **THEN** the AI uses the specified vault path to locate the annotation file

### Requirement: Synthesize Command Definition
The system SHALL provide a `/readassist-synthesize` slash command for multi-document synthesis.

#### Scenario: Invoke with multiple citekeys
- **WHEN** user invokes `/readassist-synthesize smithML2023 jonesAI2024 brownDeep2023`
- **THEN** the AI reads annotations from all sources and creates a synthesis note

#### Scenario: Invoke with theme
- **WHEN** user invokes `/readassist-synthesize smithML2023 jonesAI2024 theme:"methodology"`
- **THEN** the AI focuses the synthesis on the specified theme

### Requirement: Zettel Command Definition
The system SHALL provide a `/readassist-zettel` slash command for extracting permanent notes from IDEA-marked annotations.

#### Scenario: Invoke with citekey
- **WHEN** user invokes `/readassist-zettel smithML2023`
- **THEN** the AI extracts annotations with `IDEA:` prefix and creates permanent notes

#### Scenario: Invoke with output folder
- **WHEN** user invokes `/readassist-zettel smithML2023 output:Permanent/`
- **THEN** the AI creates permanent notes in the specified folder
