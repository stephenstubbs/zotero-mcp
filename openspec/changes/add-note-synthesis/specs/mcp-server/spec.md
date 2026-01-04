## ADDED Requirements

### Requirement: Obsidian Annotation Reading Tool
The MCP server SHALL expose a tool `obsidian_read_annotations` for reading parsed annotations.

#### Scenario: Read by citekey
- **WHEN** `obsidian_read_annotations` is called with `citekey: "smithML2023"`
- **THEN** it locates the annotation file and parses all annotations

#### Scenario: Response format
- **WHEN** annotations are read
- **THEN** the response includes:
  - `citekey`: the requested citekey
  - `title`: document title from frontmatter/metadata
  - `file_path`: path to the annotation file
  - `annotations`: array of parsed annotations

#### Scenario: Annotation fields
- **WHEN** an annotation is parsed
- **THEN** it includes: `type`, `color`, `color_hex`, `text`, `comment`, `page`, `heading_level`, `image_path`

#### Scenario: Color filtering
- **WHEN** `colors: ["positive"]` is specified
- **THEN** only green (#5fb236) annotations are returned

### Requirement: Obsidian Note Writing Tool
The MCP server SHALL expose a tool `obsidian_write_note` for creating notes.

#### Scenario: Write markdown file
- **WHEN** `obsidian_write_note` is called with `path` and `content`
- **THEN** a file is created at `{vault_path}/{path}`

#### Scenario: Frontmatter handling
- **WHEN** `frontmatter: { type: "synthesis", sources: [...] }` is provided
- **THEN** the file starts with YAML frontmatter block

#### Scenario: Response
- **WHEN** a note is written
- **THEN** the response includes the full file path

### Requirement: Obsidian File Listing Tool
The MCP server SHALL expose a tool `obsidian_list_annotation_files`.

#### Scenario: List all
- **WHEN** `obsidian_list_annotation_files` is called without filters
- **THEN** all files with `category: Annotations` frontmatter are returned

#### Scenario: List response
- **WHEN** files are listed
- **THEN** each includes: `citekey`, `title`, `path`, `tags`

#### Scenario: Tag filter
- **WHEN** `tags: ["review"]` is specified
- **THEN** only files with that tag are returned

### Requirement: Obsidian Search Tool
The MCP server SHALL expose a tool `obsidian_search` for content search.

#### Scenario: Text search
- **WHEN** `obsidian_search` is called with `query: "neural network"`
- **THEN** files containing that text are returned with match context

#### Scenario: Folder scope
- **WHEN** `folder: "References"` is specified
- **THEN** only that folder is searched

#### Scenario: Search response
- **WHEN** search results are returned
- **THEN** each includes: `path`, `matches` (with line numbers and context)
