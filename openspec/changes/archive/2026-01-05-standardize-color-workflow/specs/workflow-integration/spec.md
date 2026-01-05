# workflow-integration Specification

## Purpose
Clarify the integration between Zotero annotations, Obsidian note extraction, and note synthesis, documenting the two-phase workflow and the role of the Zotero Integration plugin.

## ADDED Requirements

### Requirement: Two-Phase Workflow
The system SHALL maintain separate reading and synthesis phases with distinct responsibilities.

#### Scenario: Reading phase creates Zotero annotations
- **WHEN** user invokes `/read` with a document
- **THEN** the AI creates annotations in Zotero only
- **AND** does NOT write to Obsidian vault

#### Scenario: Synthesis phase reads from Obsidian
- **WHEN** user invokes `/synthesize` 
- **THEN** the AI reads annotation files from Obsidian vault
- **AND** creates synthesis notes in Obsidian
- **AND** does NOT modify Zotero annotations

#### Scenario: User controls export timing
- **WHEN** user has annotated documents in Zotero
- **THEN** the user decides when to export annotations to Obsidian
- **AND** the user uses Zotero Integration plugin to trigger export
- **AND** synthesis can happen after export is complete

### Requirement: Zotero as Source of Truth
Zotero SHALL remain the authoritative source for annotations during the reading phase.

#### Scenario: Annotations created in Zotero
- **WHEN** `/read` command creates highlights
- **THEN** they are stored in Zotero's database
- **AND** are visible in Zotero's PDF reader
- **AND** can be edited manually by the user in Zotero

#### Scenario: No direct Obsidian writes during reading
- **WHEN** `/read` command is invoked
- **THEN** no Obsidian vault path is required
- **AND** no markdown files are created in Obsidian
- **AND** reading can be done without Obsidian installed

### Requirement: Zotero Integration Plugin Bridge
The system SHALL rely on the user's Zotero Integration plugin to export annotations from Zotero to Obsidian.

#### Scenario: User exports with plugin
- **WHEN** user has annotations in Zotero
- **THEN** user triggers Zotero Integration plugin (manually or auto-sync)
- **AND** plugin creates/updates markdown files in Obsidian vault
- **AND** markdown files use the user's template format

#### Scenario: Template compatibility
- **WHEN** Zotero Integration plugin exports annotations
- **THEN** it uses the user's configured template
- **AND** template includes `<mark style="background-color: #xxx">` for colors
- **AND** template includes page references and annotation metadata

#### Scenario: Color preservation through export
- **WHEN** annotations have semantic colors in Zotero
- **THEN** Zotero Integration plugin exports them with correct hex codes
- **AND** Obsidian markdown files preserve color information
- **AND** synthesis can read colors from markdown

### Requirement: Obsidian Vault Requirement for Synthesis Only
Obsidian vault access SHALL only be required for the synthesis phase.

#### Scenario: Reading without vault
- **WHEN** user invokes `/read` command
- **THEN** no Obsidian vault path configuration is needed
- **AND** reading succeeds without Obsidian

#### Scenario: Synthesis requires vault
- **WHEN** user invokes `/synthesize` command
- **THEN** Obsidian vault path MUST be configured
- **AND** synthesis fails with clear error if vault path is missing
- **AND** error message guides user to configure vault path

#### Scenario: Vault path configuration
- **WHEN** synthesis needs to access Obsidian vault
- **THEN** vault path is read from environment variable `OBSIDIAN_VAULT_PATH` or config file
- **AND** path points to the root of the Obsidian vault

### Requirement: Workflow Separation Rationale
Documentation SHALL explain why reading and synthesis are separate.

#### Scenario: Documented benefits of separation
- **WHEN** user asks why phases are separate
- **THEN** documentation explains:
  - Reading focuses on ONE document comprehension
  - Synthesis combines insights from MULTIPLE documents
  - User can review/edit Zotero annotations before synthesis
  - Obsidian export can be re-run if annotations change
  - Reading can be done without vault access (portable)

#### Scenario: Alternative workflow acknowledged
- **WHEN** documentation describes the workflow
- **THEN** it acknowledges that direct Obsidian writing during `/read` is possible
- **AND** explains why it's not the default approach
- **AND** notes it could be added as future enhancement (e.g., `--write-obsidian` flag)

## MODIFIED Requirements
None.

## REMOVED Requirements
None.
