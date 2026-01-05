# slash-command Specification

## Purpose
TBD - created by archiving change add-critical-reading-workflow. Update Purpose after archive.
## Requirements
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

### Requirement: Critical Reading Instructions
The slash command SHALL provide AI with critical reading methodology.

#### Scenario: Color scheme provided
- **WHEN** the slash command is invoked
- **THEN** the AI receives the semantic color scheme for annotations

#### Scenario: Reading strategy provided
- **WHEN** the slash command is invoked with a purpose
- **THEN** the AI is guided to focus reading on that purpose

### Requirement: Command Arguments
The slash command SHALL accept structured arguments.

#### Scenario: Parse citekey argument
- **WHEN** the first argument is a BetterBibTeX citekey
- **THEN** it is passed to the AI as the target document identifier

#### Scenario: Parse pages argument
- **WHEN** `pages:X-Y` is provided
- **THEN** the page range is extracted for the AI

#### Scenario: Parse chapters argument
- **WHEN** `chapters:"Name1,Name2"` is provided
- **THEN** the chapter names are extracted for the AI

#### Scenario: Parse purpose argument
- **WHEN** `purpose:"text"` is provided
- **THEN** the reading purpose is extracted for the AI

### Requirement: Tool Guidance
The slash command SHALL instruct the AI on which MCP tools to use.

#### Scenario: Tool sequence guidance
- **WHEN** the slash command is invoked
- **THEN** the AI is instructed to:
  1. Use `zotero_lookup` to find the item
  2. Use `zotero_get_pdf_outline` to check for bookmarks
  3. If outline exists, use section names with `zotero_read_pdf_pages`
  4. If no outline, ask user for page numbers or use `from_page` parameter
  5. Use `zotero_create_highlight` for text annotations
  6. Use `zotero_create_area_annotation` for figures/images

#### Scenario: No outline fallback guidance
- **WHEN** the slash command is invoked
- **AND** the PDF has no outline bookmarks
- **THEN** the AI is instructed to inform the user
- **AND** ask for specific page numbers to read
- **AND** offer to read from the beginning if user prefers

### Requirement: Open Questions Workflow
The slash command SHALL support an outline-first workflow for open questions reading.

#### Scenario: Outline discovery
- **WHEN** user invokes `/read` without specific pages
- **THEN** the AI calls `zotero_get_pdf_outline` first
- **AND** presents the document structure to guide reading

#### Scenario: Section-based reading
- **WHEN** outline exists with named sections
- **THEN** the AI can read sections by name
- **AND** uses outline to navigate logical document structure

#### Scenario: Page-based fallback
- **WHEN** no outline exists
- **THEN** the AI explains that no bookmarks are available
- **AND** asks user to provide page numbers
- **AND** offers option to start from page 1

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

