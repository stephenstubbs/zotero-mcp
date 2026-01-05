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

