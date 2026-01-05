# color-scheme Specification

## Purpose
TBD - created by archiving change standardize-color-workflow. Update Purpose after archive.
## Requirements
### Requirement: Question Color Semantics
The `question` color SHALL be used for marking uncertainties, questions, and areas needing clarification.

#### Scenario: Mark generated questions in SQ3R
- **WHEN** using `/read-sq3r` strategy
- **AND** AI generates questions during the Question phase
- **THEN** questions are highlighted with `question` (yellow) color

#### Scenario: Mark confusing passages
- **WHEN** reading any document
- **AND** AI encounters a confusing or ambiguous passage
- **THEN** it can use `question` color to mark it for follow-up

#### Scenario: Mark gaps in literature review
- **WHEN** using `/read-review` strategy
- **AND** AI identifies gaps or areas for future research
- **THEN** those are highlighted with `question` color

### Requirement: Color Usage Flexibility
Reading strategies SHALL be permitted to reuse non-section colors for different semantic purposes when usage is documented in the strategy's command file. Section colors (section1, section2, section3) SHALL be reserved for document hierarchy.

#### Scenario: Non-section colors can vary by strategy
- **WHEN** `/read-sq3r` uses `positive` for answers
- **AND** `/read-review` uses `positive` for supported claims
- **THEN** both are valid uses
- **AND** each strategy's command file documents its specific meaning

#### Scenario: Section colors are hierarchical only
- **WHEN** any reading strategy uses section1/2/3 colors
- **THEN** they represent document structure (headings/sections)
- **AND** they are NOT used for semantic content types like "premises" or "themes"

#### Scenario: Not all colors required
- **WHEN** `/read-skim` strategy only needs 3 colors
- **THEN** it documents which colors it uses
- **AND** other colors remain available but unused

### Requirement: Section Colors for Document Hierarchy
The section colors (section1, section2, section3) SHALL be reserved for marking document structure hierarchy to ensure compatibility with Obsidian import templates.

#### Scenario: Section1 generates H2 headings
- **WHEN** an annotation uses `section1` (Blue, #2ea8e5)
- **THEN** the Obsidian import template generates an H2 (`##`) heading
- **AND** the annotation should mark a major section or topic

#### Scenario: Section2 generates H3 headings
- **WHEN** an annotation uses `section2` (Purple, #a28ae5)
- **THEN** the Obsidian import template generates an H3 (`###`) heading
- **AND** the annotation should mark a subsection

#### Scenario: Section3 generates H4 headings
- **WHEN** an annotation uses `section3` (Magenta, #e56eee)
- **THEN** the Obsidian import template generates an H4 (`####`) heading
- **AND** the annotation should mark a sub-subsection or minor heading

#### Scenario: Semantic meaning via comments
- **WHEN** AI needs to express semantic meaning (e.g., "this is a premise", "this is a conclusion")
- **THEN** it uses the comment field on any appropriate color
- **AND** does not rely on section colors for semantic categorization

