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

## Approach: Slash Commands Only

Since Obsidian vaults are just filesystem directories, we use **slash commands** that instruct the AI to use native file tools (`Read`, `Write`) rather than implementing MCP server tools.

Benefits:
- No new Rust code required
- Leverages AI's native file capabilities
- Easier to customize (edit markdown instructions)
- Faster to implement and iterate

## Color Semantics

Colors are divided into two categories based on `update-section-color-semantics`:

### Hierarchy Colors (Document Structure)
These generate **Obsidian headings** and represent document structure, NOT semantic content:

| Color | Hex | Template Behavior | Use In Synthesis |
|-------|-----|-------------------|------------------|
| Blue (section1) | `#2ea8e5` | Creates `##` heading | Organizational structure |
| Purple (section2) | `#a28ae5` | Creates `###` heading | Sub-section structure |
| Magenta (section3) | `#e56eee` | Creates `####` heading | Minor heading structure |

### Semantic Colors (Content Meaning)
These mark **content types** and should be used for synthesis grouping:

| Color | Hex | Template Behavior | Use In Synthesis |
|-------|-----|-------------------|------------------|
| Green (positive) | `#5fb236` | Bold comment | Key findings, evidence, claims |
| Grey (detail) | `#aaaaaa` | Bold comment | Definitions, methodology, context |
| Red (negative) | `#ff6666` | Bold comment | Criticisms, limitations, concerns |
| Orange (code) | `#f19837` | Code block | Technical content, statistics |
| Yellow (question) | `#ffd400` | Bold comment | Questions, gaps, uncertainties |

### Comment Prefixes for Finer Categorization
Semantic colors can be further categorized by comment prefixes:
- `positive`: `THESIS:`, `PREMISE:`, `EVIDENCE:`, `CLAIM:`, `A:`, `FINDING:`, `CORE:`
- `negative`: `WEAKNESS:`, `LIMITATION:`, `UNCLEAR:`, `CONCERN:`
- `question`: `Q:`, `GAP:`, `UNCLEAR:`, `RELEVANT:`
- `detail`: `ASSUMPTION:`, `TERM:`, `CONNECTION:`, `METHOD:`, `THEME [x]:`, `DETAIL:`
- `code`: `STAT:`, `CODE:`, `DATA:`

## Annotation Parsing Patterns

### Color Detection
```regex
<mark style="background-color: (#[0-9a-fA-F]{6})">
```

### Page Reference
```regex
\[@[\w-]+ p\. (\d+)\]
```

### Comment Extraction
- For section colors: heading text after `## ` / `### ` / `#### `
- For semantic colors: bold text `**comment text**`

### Comment Prefix Extraction
```regex
^(THESIS|PREMISE|EVIDENCE|CLAIM|A|FINDING|CORE|WEAKNESS|LIMITATION|UNCLEAR|CONCERN|Q|GAP|RELEVANT|ASSUMPTION|TERM|CONNECTION|METHOD|THEME|DETAIL|STAT|CODE|DATA):?\s*
```

## Output Formats

### Single Document Summary

#### When Section Colors Present (Structure-Aware)
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

## {{section1_heading}}

### Key Points
- {{text}} ([[smithML2023#p. {{page}}|p. {{page}}]])

### Critical Notes
- {{text}} ([[smithML2023#p. {{page}}|p. {{page}}]])

## {{next_section_heading}}
...
```

#### When No Section Colors (Semantic Grouping)
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
- {{text}} ([[smithML2023#p. {{page}}|p. {{page}}]])

## Critical Notes  
- {{text}} ([[smithML2023#p. {{page}}|p. {{page}}]])

## Questions & Gaps
- {{text}} ([[smithML2023#p. {{page}}|p. {{page}}]])

## Methodology & Context
- {{text}} ([[smithML2023#p. {{page}}|p. {{page}}]])
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

## Common Themes

### {{theme_from_section_headings}}

**From [[@smithML2023]]:**
- {{text}} (p. {{page}})

**From [[@jonesAI2024]]:**
- {{text}} (p. {{page}})

## Key Findings (Positive)
- {{text}} - [[@{{source}}]] (p. {{page}})

## Critical Points (Negative)
- {{text}} - [[@{{source}}]] (p. {{page}})

## Open Questions
- {{text}} - [[@{{source}}]] (p. {{page}})

## Contradictions & Gaps
{{contradictions}}

## Next Steps
- [ ] Follow up on...
```

## Configuration

The AI needs to know where the Obsidian vault is located. Options:

1. **User provides path in command**: `/summarize smithML2023 vault:/path/to/vault`
2. **Environment variable**: `OBSIDIAN_VAULT_PATH`
3. **Ask user**: If not provided, prompt for the path

Default folders:
- Annotations folder: `References/` or root of vault
- Synthesis output: `Synthesis/` or user-specified path
