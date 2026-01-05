# reading-strategies Specification

## Purpose
TBD - created by archiving change add-reading-strategies. Update Purpose after archive.
## Requirements
### Requirement: SQ3R Reading Strategy
The system SHALL provide a `/read-sq3r` command implementing the SQ3R methodology.

#### Scenario: Survey phase
- **WHEN** `/read-sq3r` is invoked
- **THEN** the AI first surveys headings, figures, and structure
- **AND** highlights structural elements with Section colors

#### Scenario: Question phase
- **WHEN** the Survey phase completes
- **THEN** the AI generates questions from headings
- **AND** adds questions as Yellow-colored (`question`) annotations

#### Scenario: Read phase
- **WHEN** the Question phase completes
- **THEN** the AI reads content seeking answers
- **AND** highlights answers with Green and confusion with Red

#### Scenario: Recite and Review phases
- **WHEN** reading completes
- **THEN** the AI summarizes each section
- **AND** creates a final review note in Zotero

### Requirement: Literature Review Strategy
The system SHALL provide a `/read-review` command for systematic evidence extraction.

#### Scenario: Evidence extraction
- **WHEN** `/read-review` is invoked
- **THEN** the AI extracts claims, methods, and findings
- **AND** highlights supported claims with Green and limitations with Red

#### Scenario: Quality assessment
- **WHEN** evidence is extracted
- **THEN** the AI notes quality indicators and potential biases

#### Scenario: Thematic categorization
- **WHEN** extraction completes
- **THEN** the AI categorizes findings by theme using annotation comments

### Requirement: Analytical Reading Strategy
The system SHALL provide a `/read-analyze` command for deep argument analysis.

#### Scenario: Structural analysis
- **WHEN** `/read-analyze` is invoked
- **THEN** the AI identifies thesis, premises, and conclusions
- **AND** highlights premises with Blue and conclusions with Purple

#### Scenario: Critique
- **WHEN** structure is analyzed
- **THEN** the AI evaluates argument validity
- **AND** marks logical gaps with Red

#### Scenario: Depth parameter
- **WHEN** `/read-analyze depth:deep` is specified
- **THEN** the AI performs more thorough analysis with additional passes

### Requirement: Skim Reading Strategy
The system SHALL provide a `/read-skim` command for quick overview.

#### Scenario: Quick overview
- **WHEN** `/read-skim` is invoked
- **THEN** the AI reads only title, abstract, headings, figures, and conclusion

#### Scenario: Time constraint
- **WHEN** `/read-skim time:5m` is specified
- **THEN** the AI limits analysis to fit within the time budget

#### Scenario: Relevance assessment output
- **WHEN** skimming completes
- **THEN** the AI outputs a brief relevance assessment with key takeaways

### Requirement: Strategy Selection Parameter
The base `/read` command SHALL accept a strategy parameter.

#### Scenario: Explicit strategy selection
- **WHEN** `/read citekey strategy:sq3r` is invoked
- **THEN** the SQ3R methodology is used

#### Scenario: Default strategy
- **WHEN** `/read citekey` is invoked without a strategy parameter
- **THEN** the "critical" strategy (Phase 1 behavior) is used

#### Scenario: Invalid strategy
- **WHEN** an unknown strategy is specified
- **THEN** an error lists available strategies

### Requirement: Strategy-Specific Color Documentation
Each reading strategy command file SHALL document its specific color usage.

#### Scenario: Color scheme section in command file
- **WHEN** a reading strategy command file is created (e.g., `/read-sq3r.md`)
- **THEN** it includes a "Color Scheme" section
- **AND** the section documents which of the 8 colors the strategy uses
- **AND** the section explains what each color means in that strategy's context

#### Scenario: Color reuse flexibility
- **WHEN** different strategies use the same color differently
- **THEN** both usages are valid
- **AND** each strategy's command file documents its specific interpretation
- **Example**: `/read-sq3r` uses `section1` for chapter structure, while `/read-analyze` uses it for premises

