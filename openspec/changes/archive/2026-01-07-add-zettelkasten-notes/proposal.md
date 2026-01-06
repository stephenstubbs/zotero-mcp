# Change: Add Zettelkasten Permanent Notes Command

## Why

Researchers using the Zettelkasten method need to extract **atomic, permanent notes** from their reading annotations. The current workflow supports:
- `/summarize` - Single document summaries (grouped by color/section)
- `/synthesize` - Multi-document synthesis (themes, agreements, contradictions)

Neither produces the **atomic, single-idea notes** that Zettelkasten requires. Permanent notes should:
- Contain exactly one idea, rewritten in the researcher's own words
- Link back to the source annotation
- Be discoverable and linkable to other permanent notes

## What Changes

1. **New `/zettel` slash command** - Extracts permanent notes from Obsidian annotations
   - Only processes annotations marked with `IDEA:` comment prefix
   - Rewrites each idea atomically in the user's own words
   - Creates individual note files in `Permanent/` folder
   - Uses pandoc citation format for source linking

2. **New `IDEA:` comment prefix convention** - Marks annotations for zettel extraction
   - Added to `/read` command as a cross-cutting prefix (works with any semantic color)
   - Does not interfere with `/summarize` or `/synthesize` (they ignore or include it like other prefixes)
   - No conflict with existing prefixes (THESIS:, CLAIM:, FINDING:, etc.)

3. **Update `/read` command** - Document `IDEA:` prefix for Zettelkasten workflow
   - Add `IDEA:` to the color reference table as a cross-cutting prefix
   - Explain that `IDEA:` can be combined with any semantic color
   - Document that `/zettel` will extract these annotations

4. **Standardize pandoc citation format** - All three commands use consistent citation format
   - `/summarize` - Uses `[@citekey p. X]` for page references (currently uses wikilinks)
   - `/synthesize` - Uses `[@citekey p. X]` for page references (currently uses wikilinks)
   - `/zettel` - Uses `[@citekey p. X]` for source attribution

5. **Documentation update for annotation template** - All three commands (`/summarize`, `/synthesize`, `/zettel`) will document the exact annotation template format for accurate parsing

## Approach

Like `/summarize` and `/synthesize`, the `/zettel` command will:
- Read annotation files from Obsidian vault (no MCP tools needed)
- Parse annotations using the documented template format
- Write permanent notes using the `Write` tool

The key difference is output granularity:
- `/summarize` → 1 summary file per source
- `/synthesize` → 1 synthesis file per session
- `/zettel` → N permanent note files (one per `IDEA:` annotation)

## Annotation Template Understanding

All commands must parse the Zotero Integration export template correctly:

### File Structure
```
---
category: Annotations
status: Pending
citekey: {{citekey}}
tags: {{allTags}}
---

> [!Cite]
> [@{{citekey}}]

>[!Synth]
>**Related**:: [[@relatedCitekey1]], [[@relatedCitekey2]]

>[!md]
> **FirstAuthor**:: LastName, FirstName
> **Title**:: Document Title
> **Year**:: 2024
> **Citekey**:: citekey
> ...metadata fields...

> [!Abstract]
> Abstract text here.

# Annotations
## Imported: YYYY-MM-DD h:mm A

<mark style="background-color: #HEX">Highlight</mark>
## **Comment text**
"Highlighted text content"
[@citekey p. 22]
```

### Annotation Block Format
```
<mark style="background-color: {{color}}>Highlight|Note|Image</mark>
[##|###|####] [**{{comment}}**]
["{{annotatedText}}"]
[![[imageBaseName]]]
[@{{citekey}} p. {{pageLabel}}]
```

Key patterns:
- Section colors (blue/purple/magenta) add heading markers (`##`/`###`/`####`)
- Code color (`#f19837`) wraps text in code fences with comment as language
- Other colors show comment in bold before quoted text
- Page references use pandoc format: `[@citekey p. X]` or `[@citekey pp. X-Y]`

### Color Scheme (8 colors)
| Hex | Name | Purpose |
|-----|------|---------|
| `#2ea8e5` | section1 | H2 heading / major section |
| `#a28ae5` | section2 | H3 heading / subsection |
| `#e56eee` | section3 | H4 heading / sub-subsection |
| `#5fb236` | positive | Key findings, evidence, agreements |
| `#aaaaaa` | detail | Definitions, methodology, context |
| `#ff6666` | negative | Criticisms, limitations, concerns |
| `#f19837` | code | Technical content, code blocks |
| `#ffd400` | question | Questions, gaps, uncertainties |

## Impact

- **Modified spec**: `note-synthesis` (add `/zettel` command, pandoc citation requirement)
- **New files**:
  - `.opencode/command/zettel.md` - Slash command implementation
- **Modified files**:
  - `.opencode/command/read.md` - Add `IDEA:` prefix documentation
  - `.opencode/command/summarize.md` - Add template documentation, switch to pandoc citations
  - `.opencode/command/synthesize.md` - Add template documentation, switch to pandoc citations

## Dependencies

- Requires: User marks annotations with `IDEA:` prefix during reading
- Requires: Zotero Integration plugin exports annotations to Obsidian

## Open Questions

None - all clarified in discussion.
