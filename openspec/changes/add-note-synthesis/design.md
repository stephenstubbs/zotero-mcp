# Design: Obsidian-Based Note Synthesis

## Context

The user's workflow already exports Zotero annotations to Obsidian using the Zotero Integration plugin. This creates markdown files with a specific structure:

```markdown
---
category: Annotations
status: Pending
citekey: smithML2023
tags: machine-learning, review
---

> [!Cite]
> [@smithML2023]

# Annotations
## Imported: 2024-01-15 2:30 pm

<mark style="background-color: #2ea8e5">Highlight</mark>
## **Section heading from comment**
Highlighted text content here
[@smithML2023 p. 5]

<mark style="background-color: #5fb236">Highlight</mark>
**Positive point comment**
The key finding was significant
[@smithML2023 p. 12]

<mark style="background-color: #ffd400">Highlight</mark>
**Question: What about edge cases?**
The authors do not address boundary conditions
[@smithML2023 p. 18]
```

Phase 4 builds ON TOP of this workflow rather than replacing it.

### Color Semantics in Template

| Color | Hex | Template Behavior |
|-------|-----|-------------------|
| Blue | `#2ea8e5` | Creates `##` heading from comment |
| Purple | `#a28ae5` | Creates `###` heading from comment |
| Magenta | `#e56eee` | Creates `####` heading from comment |
| Green | `#5fb236` | Positive point (bold comment) |
| Grey | `#aaaaaa` | Detail (bold comment) |
| Red | `#ff6666` | Negative point (bold comment) |
| Orange | `#f19837` | Code block with comment as language |
| Yellow | `#ffd400` | Question / Uncertainty (bold comment) |

## Goals / Non-Goals

### Goals
- Read annotation files from Obsidian vault
- Parse annotations with colors, text, page references
- Synthesize annotations across multiple documents
- Create new notes in Obsidian with proper linking
- Maintain compatibility with existing Zotero Integration workflow

### Non-Goals
- Replace Zotero Integration plugin
- Modify existing annotation files
- Sync changes back to Zotero
- Real-time annotation monitoring

## Architecture

```
┌─────────────────┐
│  /synthesize    │
│  /summarize     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐     ┌─────────────────┐
│  MCP Server     │────▶│  Obsidian Vault │
│  (new tools)    │     │  (filesystem)   │
└─────────────────┘     └─────────────────┘
         │
         ▼
┌─────────────────┐
│  Synthesis Note │
│  (markdown)     │
└─────────────────┘
```

### File Discovery

Annotation files are located by:
1. Searching for files with `citekey: <key>` in frontmatter
2. Or by naming convention if configured (e.g., `@citekey.md`)

## Decisions

### Decision 1: Filesystem Access for Obsidian
**What**: Read/write Obsidian files directly via filesystem.

**Why**:
- Obsidian vaults are just folders of markdown files
- No Obsidian API needed (works even when Obsidian is closed)
- Simple, reliable, fast

**Alternative**: Obsidian Local REST API plugin.
**Rejected**: Requires plugin installation, Obsidian must be running.

### Decision 2: Parse User's Actual Template
**What**: Parse the specific Zotero Integration template structure the user has.

**Why**:
- User already has established workflow
- Template contains rich semantic information
- Changing templates would break existing notes

**Parsing Strategy**:
```
1. Extract frontmatter (citekey, tags, status)
2. Find "# Annotations" section
3. Parse each annotation block:
   - Color from <mark style="background-color: #xxx">
   - Type from "Highlight", "Note", "Image"
   - Heading level from ##/###/####
   - Comment from **bold text**
   - Text content
   - Page reference from [@citekey p. X]
```

### Decision 3: Wikilinks for Obsidian Integration
**What**: Use `[[wikilinks]]` in synthesis notes.

**Why**:
- Native Obsidian linking
- Enables graph view connections
- Backlinks work automatically

**Format**:
```markdown
## Key Findings

From [[@smithML2023]]:
- Finding 1 ([[smithML2023#p. 5|p. 5]])
- Finding 2 ([[smithML2023#p. 12|p. 12]])
```

### Decision 4: Dataview-Compatible Frontmatter
**What**: Include frontmatter that works with Dataview plugin.

**Why**:
- Dataview is commonly used for note organization
- Enables queries across synthesis notes
- Structured metadata aids discovery

**Format**:
```yaml
---
type: synthesis
sources:
  - "[[smithML2023]]"
  - "[[jonesAI2024]]"
themes:
  - methodology
  - results
created: 2024-01-15
---
```

## MCP Tool Definitions

### Tool: `obsidian_read_annotations`
```json
{
  "name": "obsidian_read_annotations",
  "description": "Read and parse annotations from an Obsidian annotation file",
  "inputSchema": {
    "type": "object",
    "properties": {
      "citekey": {
        "type": "string",
        "description": "BetterBibTeX citekey to find annotations for"
      },
      "colors": {
        "type": "array",
        "items": { "type": "string" },
        "description": "Filter to specific colors (e.g., ['positive', 'negative'])"
      }
    },
    "required": ["citekey"]
  }
}
```

**Response**:
```json
{
  "citekey": "smithML2023",
  "title": "Machine Learning Survey",
  "file_path": "/vault/References/@smithML2023.md",
  "annotations": [
    {
      "type": "highlight",
      "color": "positive",
      "color_hex": "#5fb236",
      "text": "The key finding was significant",
      "comment": "Important result",
      "page": "12",
      "heading_level": null
    },
    {
      "type": "highlight", 
      "color": "section1",
      "color_hex": "#2ea8e5",
      "text": "Introduction content",
      "comment": "Background",
      "page": "1",
      "heading_level": 2
    }
  ]
}
```

### Tool: `obsidian_write_note`
```json
{
  "name": "obsidian_write_note",
  "description": "Create or update a markdown note in Obsidian vault",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": {
        "type": "string",
        "description": "Path relative to vault root (e.g., 'Synthesis/methodology-review.md')"
      },
      "content": {
        "type": "string",
        "description": "Markdown content for the note"
      },
      "frontmatter": {
        "type": "object",
        "description": "YAML frontmatter as key-value pairs"
      }
    },
    "required": ["path", "content"]
  }
}
```

### Tool: `obsidian_list_annotation_files`
```json
{
  "name": "obsidian_list_annotation_files",
  "description": "List annotation files in the Obsidian vault",
  "inputSchema": {
    "type": "object",
    "properties": {
      "folder": {
        "type": "string",
        "description": "Folder to search (default: entire vault)"
      },
      "tags": {
        "type": "array",
        "items": { "type": "string" },
        "description": "Filter by tags in frontmatter"
      }
    }
  }
}
```

### Tool: `obsidian_search`
```json
{
  "name": "obsidian_search",
  "description": "Search for content across Obsidian vault files",
  "inputSchema": {
    "type": "object",
    "properties": {
      "query": {
        "type": "string",
        "description": "Text to search for"
      },
      "folder": {
        "type": "string",
        "description": "Limit search to folder"
      }
    },
    "required": ["query"]
  }
}
```

## Annotation Parsing

### Parser Implementation

```rust
struct ParsedAnnotation {
    annotation_type: AnnotationType,  // highlight, note, image
    color: SemanticColor,
    color_hex: String,
    text: Option<String>,
    comment: Option<String>,
    page: Option<String>,
    heading_level: Option<u8>,  // 2, 3, or 4 for ##, ###, ####
    image_path: Option<String>,
}

// Parse from template format:
// <mark style="background-color: #2ea8e5">Highlight</mark>
// ## **Comment text**
// Annotation text content
// [@citekey p. 5]
```

### Color Detection Regex
```regex
<mark style="background-color: (#[0-9a-fA-F]{6})">
```

### Page Reference Regex
```regex
\[@\w+ p\. (\d+)\]
```

## Synthesis Output Format

### Single Document Summary
```markdown
---
type: summary
source: "[[smithML2023]]"
created: 2024-01-15
status: draft
---

# Summary: {{title}}

> [!info] Source
> [[@smithML2023]] - {{authors}}, {{year}}

## Key Points
{{#each positive_annotations}}
- {{text}} ([[smithML2023#p. {{page}}|p. {{page}}]])
{{/each}}

## Critical Notes  
{{#each negative_annotations}}
- {{text}} ([[smithML2023#p. {{page}}|p. {{page}}]])
{{/each}}

## Structure
{{#each section_annotations}}
{{heading}} {{comment}}
- {{text}}
{{/each}}
```

### Multi-Document Synthesis
```markdown
---
type: synthesis
sources:
  - "[[smithML2023]]"
  - "[[jonesAI2024]]"
  - "[[brownDeep2023]]"
theme: methodology
created: 2024-01-15
---

# Synthesis: {{theme}}

## Overview
{{ai_generated_overview}}

## Findings by Source

### From [[@smithML2023]]
{{#each smith_annotations}}
- {{text}} (p. {{page}})
{{/each}}

### From [[@jonesAI2024]]
{{#each jones_annotations}}
- {{text}} (p. {{page}})
{{/each}}

## Themes

### {{theme_1}}
{{theme_1_synthesis}}

### {{theme_2}}
{{theme_2_synthesis}}

## Contradictions & Gaps
{{contradictions}}

## Next Steps
- [ ] Follow up on...
```

## Configuration

### Environment Variables
```bash
OBSIDIAN_VAULT_PATH=/path/to/vault
OBSIDIAN_ANNOTATIONS_FOLDER=References  # where annotation files live
OBSIDIAN_SYNTHESIS_FOLDER=Synthesis     # where to write synthesis notes
```

### Config File (alternative)
```toml
# .zotero-mcp.toml
[obsidian]
vault_path = "/path/to/vault"
annotations_folder = "References"
synthesis_folder = "Synthesis"
```

## Risks / Trade-offs

### Risk: Template Format Changes
**Impact**: Parser breaks if user modifies Zotero Integration template.
**Mitigation**: Document supported template format; provide template validation.

### Risk: Large Vaults Slow to Search
**Impact**: Finding annotation files may be slow.
**Mitigation**: Cache file index; allow folder restriction.

### Trade-off: Filesystem vs API
**Choice**: Direct filesystem access.
**Trade-off**: Can't use Obsidian-specific features (e.g., templates plugin).
**Mitigation**: Generate standard markdown that works with any plugin.

## Open Questions

1. Should we support multiple vault configurations?
   - **Recommendation**: Single vault for now, expand if needed

2. How to handle annotation file naming variations?
   - **Recommendation**: Search by frontmatter `citekey:` field, not filename
