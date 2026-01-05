# color-scheme Specification

## Purpose
Define the semantic color scheme used across Zotero annotations, reading strategies, and Obsidian integration, ensuring consistency between all system components.

## Requirements

### Requirement: Eight Semantic Colors
The system SHALL provide eight semantic highlight colors with standardized hex codes and meanings.

#### Scenario: Complete color enumeration
- **WHEN** colors are defined in the system
- **THEN** exactly 8 colors are available: `section1`, `section2`, `section3`, `positive`, `detail`, `negative`, `code`, `question`

#### Scenario: Yellow/Question color available
- **WHEN** a user needs to mark questions or uncertainties
- **THEN** the `question` color with hex `#ffd400` is available
- **AND** it has semantic meaning "Question / Uncertainty / Needs clarification"

#### Scenario: Hex code standardization
- **WHEN** colors are converted to hex codes
- **THEN** the following mappings apply:
  - `section1` → `#2ea8e5` (Blue)
  - `section2` → `#a28ae5` (Purple)
  - `section3` → `#e56eee` (Magenta)
  - `positive` → `#5fb236` (Green)
  - `detail` → `#aaaaaa` (Grey)
  - `negative` → `#ff6666` (Red)
  - `code` → `#f19837` (Orange)
  - `question` → `#ffd400` (Yellow)

### Requirement: Semantic Color Descriptions
Each color SHALL have a clear semantic description that guides usage.

#### Scenario: Primary use descriptions
- **WHEN** documentation describes color semantics
- **THEN** the following primary uses are documented:
  - `section1`: "Section 1 / Primary organization"
  - `section2`: "Section 2 / Secondary organization"
  - `section3`: "Section 3 / Tertiary organization"
  - `positive`: "Positive point / Agreement"
  - `detail`: "Point detail / Context"
  - `negative`: "Negative point / Criticism"
  - `code`: "Code / Technical content"
  - `question`: "Question / Uncertainty / Needs clarification"

#### Scenario: Strategy-specific usage documented
- **WHEN** a reading strategy uses colors differently
- **THEN** the strategy's command file documents its specific color usage
- **AND** explains how colors map to strategy phases or concepts

### Requirement: Reverse Color Mapping
The system SHALL support converting hex codes to semantic color names.

#### Scenario: Parse hex from Obsidian annotations
- **WHEN** parsing `<mark style="background-color: #ffd400">` from Obsidian
- **THEN** it maps to semantic color `question`

#### Scenario: Unknown hex code handling
- **WHEN** parsing an annotation with hex `#123456` (not in the standard set)
- **THEN** the system returns `None` or an error indicating unsupported color

### Requirement: Color Compatibility Across Components
All system components SHALL use the same color definitions.

#### Scenario: Zotero client color enum
- **WHEN** `HighlightColor` enum is defined in `zotero-client`
- **THEN** it includes all 8 semantic colors with correct hex mappings

#### Scenario: MCP server color parameter
- **WHEN** `HighlightColorParam` enum is defined in `zotero-mcp`
- **THEN** it includes all 8 semantic colors
- **AND** converts correctly to `HighlightColor`

#### Scenario: Obsidian parser color mapping
- **WHEN** Obsidian integration parses annotations
- **THEN** it recognizes all 8 hex codes and maps to semantic names

#### Scenario: Reading strategy color references
- **WHEN** slash command instructions reference colors
- **THEN** they use semantic names (`question`) not hex codes (`#ffd400`)
- **AND** the AI receives semantic color names for annotations

## ADDED Requirements

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
Reading strategies SHALL be permitted to reuse colors for different semantic purposes when usage is documented in the strategy's command file.

#### Scenario: Blue means different things in different strategies
- **WHEN** `/read-sq3r` uses `section1` for chapter structure
- **AND** `/read-analyze` uses `section1` for premises
- **THEN** both are valid uses
- **AND** each strategy's command file documents its specific meaning

#### Scenario: Not all colors required
- **WHEN** `/read-skim` strategy only needs 3 colors
- **THEN** it documents which colors it uses
- **AND** other colors remain available but unused

## MODIFIED Requirements
None. This is a new spec extracted from color-related requirements in other specs.

## REMOVED Requirements
None.
