## ADDED Requirements

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
  2. Use `zotero_read_pdf_pages` to extract content
  3. Use `zotero_create_highlight` for text annotations
  4. Use `zotero_create_area_annotation` for figures/images
