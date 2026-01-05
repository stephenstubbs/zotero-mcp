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

Colors are divided into two categories based on `update-section-color-semantics`:

#### Hierarchy Colors (Document Structure)
These generate **Obsidian headings** and represent document structure, NOT semantic content:

| Color | Hex | Template Behavior | Use In Synthesis |
|-------|-----|-------------------|------------------|
| Blue (section1) | `#2ea8e5` | Creates `##` heading | Organizational structure |
| Purple (section2) | `#a28ae5` | Creates `###` heading | Sub-section structure |
| Magenta (section3) | `#e56eee` | Creates `####` heading | Minor heading structure |

#### Semantic Colors (Content Meaning)
These mark **content types** and should be used for synthesis grouping:

| Color | Hex | Template Behavior | Use In Synthesis |
|-------|-----|-------------------|------------------|
| Green (positive) | `#5fb236` | Bold comment | Key findings, evidence, claims |
| Grey (detail) | `#aaaaaa` | Bold comment | Definitions, methodology, context |
| Red (negative) | `#ff6666` | Bold comment | Criticisms, limitations, concerns |
| Orange (code) | `#f19837` | Code block | Technical content, statistics |
| Yellow (question) | `#ffd400` | Bold comment | Questions, gaps, uncertainties |

#### Comment Prefixes for Finer Categorization
Semantic colors can be further categorized by comment prefixes:
- `positive`: `THESIS:`, `PREMISE:`, `EVIDENCE:`, `CLAIM:`, `A:`, `FINDING:`, `CORE:`
- `negative`: `WEAKNESS:`, `LIMITATION:`, `UNCLEAR:`, `CONCERN:`
- `question`: `Q:`, `GAP:`, `UNCLEAR:`, `RELEVANT:`
- `detail`: `ASSUMPTION:`, `TERM:`, `CONNECTION:`, `METHOD:`, `THEME [x]:`, `DETAIL:`
- `code`: `STAT:`, `CODE:`, `DATA:`

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
enum ColorCategory {
    Hierarchy,  // section1, section2, section3 - for document structure
    Semantic,   // positive, negative, question, detail, code - for content meaning
}

struct ParsedAnnotation {
    annotation_type: AnnotationType,  // highlight, note, image
    color: SemanticColor,
    color_hex: String,
    color_category: ColorCategory,    // NEW: hierarchy vs semantic
    text: Option<String>,
    comment: Option<String>,
    comment_prefix: Option<String>,   // NEW: extracted prefix like "THESIS:", "Q:"
    page: Option<String>,
    heading_level: Option<u8>,        // 2, 3, or 4 for ##, ###, #### (hierarchy colors only)
    image_path: Option<String>,
}

// Parse from template format:
// <mark style="background-color: #2ea8e5">Highlight</mark>
// ## **Comment text**
// Annotation text content
// [@citekey p. 5]
```

### Comment Prefix Extraction

```rust
// Extract prefix from comment like "THESIS: Main argument here"
fn extract_comment_prefix(comment: &str) -> (Option<String>, String) {
    let prefixes = ["THESIS:", "PREMISE:", "EVIDENCE:", "CLAIM:", "A:", "FINDING:", 
                    "CORE:", "WEAKNESS:", "LIMITATION:", "UNCLEAR:", "CONCERN:",
                    "Q:", "GAP:", "RELEVANT:", "ASSUMPTION:", "TERM:", 
                    "CONNECTION:", "METHOD:", "THEME", "DETAIL:", "STAT:", 
                    "CODE:", "DATA:"];
    
    for prefix in prefixes {
        if comment.starts_with(prefix) || comment.starts_with(&format!("**{}**", prefix)) {
            return (Some(prefix.to_string()), comment[prefix.len()..].trim().to_string());
        }
    }
    (None, comment.to_string())
}
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

The synthesis output uses **section colors for structure** and **semantic colors for content grouping**.

#### When Section Colors Present (Structure-Aware)
If the source document has section1/2/3 annotations, use them as the organizational backbone:

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

## {{section1_heading}}  <!-- From blue annotation comment -->

### Key Points
{{#each section1_positive_annotations}}
- {{text}} ([[smithML2023#p. {{page}}|p. {{page}}]])
{{/each}}

### Critical Notes
{{#each section1_negative_annotations}}
- {{text}} ([[smithML2023#p. {{page}}|p. {{page}}]])
{{/each}}

## {{section1_heading_2}}  <!-- Next blue annotation -->
...
```

#### When No Section Colors (Semantic Grouping)
Fall back to grouping by semantic color when no structural annotations exist:

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

## Questions & Gaps
{{#each question_annotations}}
- {{text}} ([[smithML2023#p. {{page}}|p. {{page}}]])
{{/each}}

## Methodology & Context
{{#each detail_annotations}}
- {{text}} ([[smithML2023#p. {{page}}|p. {{page}}]])
{{/each}}
```

### Multi-Document Synthesis

For multi-document synthesis, identify **common themes from section headings** across documents and group related semantic annotations under shared themes.

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

## Common Themes

### {{theme_from_section_headings}}
<!-- Group annotations from all sources that fall under this theme -->

**From [[@smithML2023]]:**
{{#each smith_theme_annotations}}
- {{text}} (p. {{page}})
{{/each}}

**From [[@jonesAI2024]]:**
{{#each jones_theme_annotations}}
- {{text}} (p. {{page}})
{{/each}}

### {{another_theme}}
...

## Key Findings (Positive)
{{#each all_positive_annotations}}
- {{text}} - [[@{{source}}]] (p. {{page}})
{{/each}}

## Critical Points (Negative)
{{#each all_negative_annotations}}
- {{text}} - [[@{{source}}]] (p. {{page}})
{{/each}}

## Open Questions
{{#each all_question_annotations}}
- {{text}} - [[@{{source}}]] (p. {{page}})
{{/each}}

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
