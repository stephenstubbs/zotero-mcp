## MODIFIED Requirements

### Requirement: Read Command Definition
The system SHALL provide a `/read` slash command for initiating critical reading workflows.

#### Scenario: Invoke with citekey and pages
- **WHEN** user invokes `/read smithML2023 pages:1-10 purpose:"understand methodology"`
- **THEN** the AI receives the command parameters and critical reading instructions

#### Scenario: Invoke with citekey and chapters
- **WHEN** user invokes `/read smithML2023 chapters:"Introduction,Methods"`
- **THEN** the AI receives the chapter names to look up in the PDF

#### Scenario: Invoke with citekey only
- **WHEN** user invokes `/read smithML2023`
- **THEN** the AI is instructed to read the full document

#### Scenario: Invoke with strategy parameter
- **WHEN** user invokes `/read smithML2023 strategy:sq3r`
- **THEN** the AI uses the SQ3R reading methodology instead of default critical reading

#### Scenario: List available strategies
- **WHEN** user invokes `/read --help` or requests strategy list
- **THEN** the available strategies are displayed: critical, sq3r, review, analyze, skim

## ADDED Requirements

### Requirement: Strategy-Specific Commands
The system SHALL provide dedicated commands for each reading strategy.

#### Scenario: SQ3R command
- **WHEN** user invokes `/read-sq3r citekey`
- **THEN** it is equivalent to `/read citekey strategy:sq3r`

#### Scenario: Review command
- **WHEN** user invokes `/read-review citekey`
- **THEN** it is equivalent to `/read citekey strategy:review`

#### Scenario: Analyze command
- **WHEN** user invokes `/read-analyze citekey`
- **THEN** it is equivalent to `/read citekey strategy:analyze`

#### Scenario: Skim command
- **WHEN** user invokes `/read-skim citekey`
- **THEN** it is equivalent to `/read citekey strategy:skim`
