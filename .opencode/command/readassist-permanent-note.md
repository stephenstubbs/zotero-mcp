---
description: Extract permanent notes from IDEA-marked annotations following Zettelkasten principles.
---
Execute the /readassist-permanent-note command with the following arguments:
<UserRequest>
  $ARGUMENTS
</UserRequest>

# /readassist-permanent-note - Zettelkasten Permanent Notes

## Overview

The `/readassist-permanent-note` command extracts **atomic permanent notes** from Obsidian annotations that have been marked with the `IDEA:` comment prefix. Each permanent note contains exactly one idea, rewritten in your own words, with a link back to the source.

## Usage

```
/readassist-permanent-note <citekey> [vault:<path>] [output:<folder>]
```

## Arguments

- `citekey` (required): BetterBibTeX citation key (e.g., "smithML2023")
- `vault:<path>` (optional): Path to Obsidian vault root (or set `OBSIDIAN_VAULT_PATH` env var)
- `output:<folder>` (optional): Output folder for permanent notes (default: `1. Literature/1.2. Permanent/`)

## Examples

```
/readassist-permanent-note smithML2023
/readassist-permanent-note smithML2023 vault:/home/user/ObsidianVault
/readassist-permanent-note smithML2023 output:Zettelkasten/
```

## The IDEA: Prefix Convention

During reading (with `/readassist-read` or manual annotation), mark annotations that contain atomic ideas worth extracting as permanent notes by starting the comment with `IDEA:`.

**Example annotation in Zotero:**
- Highlight: "Transfer learning reduces data requirements by leveraging pre-trained models"
- Comment: `IDEA: Transfer learning trades data for compute - use existing model knowledge instead of raw data`
- Color: positive (green)

This annotation will be extracted by `/readassist-permanent-note` because it has the `IDEA:` prefix.

**Annotations without `IDEA:` prefix are ignored** by this command. Use `/readassist-summarize` or `/readassist-synthesize` for those.

## Instructions for AI

When this command is invoked, follow these steps:

### Step 1: Locate the Vault

1. Check if `vault:` argument was provided
2. Otherwise, check environment variable `OBSIDIAN_VAULT_PATH`
3. If neither, ask the user for the vault path

### Step 2: Find the Annotation File

1. Search for files with `citekey: <citekey>` in frontmatter
2. Common locations:
   - `1. Literature/1.1. Annotations/@<citekey>Annotations.md`
   - Root of vault
3. Use `Glob` tool to find: `**/*<citekey>*.md`

### Step 3: Read and Parse the Annotation File

Use the `Read` tool to read the annotation file. The file follows the Zotero Integration export template format.

#### Annotation File Structure

The annotation file has this structure:

```markdown
---
category: Annotations
status: Pending
citekey: {citekey}
tags: {tags}
---

> [!Cite]
> [@{citekey}]

>[!Synth]
>**Related**:: [[@relatedCitekey1]], [[@relatedCitekey2]]

>[!md]
> **FirstAuthor**:: LastName, FirstName
> **Title**:: Document Title
> **Year**:: 2024
> **Citekey**:: {citekey}
> ...other metadata...

> [!Abstract]
> Abstract text here.

# Annotations
## Imported: YYYY-MM-DD h:mm A

{annotation blocks follow}
```

#### Color Scheme (8 colors)

| Hex | Name | Category | Purpose |
|-----|------|----------|---------|
| `#2ea8e5` | section1 | hierarchy | H2 heading / major section |
| `#a28ae5` | section2 | hierarchy | H3 heading / subsection |
| `#e56eee` | section3 | hierarchy | H4 heading / sub-subsection |
| `#5fb236` | positive | semantic | Key findings, evidence, agreements |
| `#aaaaaa` | detail | semantic | Definitions, methodology, context |
| `#ff6666` | negative | semantic | Criticisms, limitations, concerns |
| `#f19837` | code | semantic | Technical content, code blocks |
| `#ffd400` | question | semantic | Questions, gaps, uncertainties |

#### Annotation Block Patterns

**Standard highlight annotation:**
```markdown
<mark style="background-color: #HEX">Highlight</mark>
**Comment text**
"Highlighted text content"
[@citekey p. X]
```

**Section color annotation (blue/purple/magenta):**
```markdown
<mark style="background-color: #2ea8e5">Highlight</mark>
## **Section Title**
"Highlighted text content"
[@citekey p. X]
```

**Code annotation (orange #f19837):**
```markdown
<mark style="background-color: #f19837">Highlight</mark>
```python
print("code content")
```
[@citekey p. X]
```

#### Page Reference Patterns

Single page:
```regex
\[@[\w-]+ p\. (\d+)\]
```

Multiple pages:
```regex
\[@[\w-]+ pp\. (\d+)-(\d+)\]
```

### Step 4: Filter for IDEA: Annotations

From all parsed annotations, **keep only those where the comment starts with `IDEA:`**.

For each IDEA annotation, extract:
1. **Idea text**: The comment text after `IDEA:` prefix (this is the core idea in the user's words)
   - If there's a secondary prefix, include it: `IDEA: FINDING: X` → idea text is `FINDING: X`
   - If no secondary prefix: `IDEA: X` → idea text is `X`
2. **Source text**: The highlighted text (original author's words)
3. **Source reference**: The pandoc citation `[@citekey p. X]`
4. **Color**: The semantic color (provides context about the type of idea)
5. **Secondary prefix** (if present): `FINDING:`, `CLAIM:`, `GAP:`, etc. (useful for categorization)

### Step 5: Generate Permanent Notes

For each `IDEA:` annotation, create a separate permanent note file.

#### Permanent Note Format

```markdown
---
type: permanent
source: "[@{citekey} p. {page}]"
created: {today's date}
tags:
  - permanent
  - {color-based-tag}
---

# {Idea title - derived from IDEA: comment}

{The idea rewritten as a complete, standalone statement. This should be:
- Atomic: one idea only
- In your own words: not a direct quote
- Self-contained: understandable without the source
- Linked: references the source for provenance}

---

**Source:** [@{citekey} p. {page}]

> "{Original highlighted text}"
```

#### Filename Convention

Create a slugified filename from the idea title:
- Lowercase
- Replace spaces with hyphens
- Remove special characters
- Limit to ~50 characters

Example: `IDEA: Transfer learning trades data for compute` → `transfer-learning-trades-data-for-compute.md`

#### Color-Based Tags

Add a tag based on the annotation color:
| Color | Tag |
|-------|-----|
| positive | `#finding` |
| negative | `#critique` |
| question | `#question` |
| detail | `#concept` |
| code | `#technical` |
| section1/2/3 | `#structure` |

### Step 6: Write the Permanent Notes

1. Determine output folder:
   - Use `output:` argument if provided
   - Otherwise: `{vault}/Permanent/`
2. Create the folder if it doesn't exist
3. For each IDEA annotation:
   - Generate the filename
   - Use `Write` tool to create the file
4. Report:
   - Number of permanent notes created
   - List of files created
   - Any parsing issues encountered

## Citation Format

Use **pandoc citation format** for all references:

- Source reference: `[@{citekey} p. {page}]`
- Multi-page reference: `[@{citekey} pp. {start}-{end}]`
- Frontmatter source: `"[@{citekey} p. {page}]"` (quoted for YAML)

## Error Handling

- **Vault not found**: Ask user to provide vault path
- **Annotation file not found**: Report citekey not found, suggest checking vault location
- **No IDEA: annotations**: Report that no annotations have the `IDEA:` prefix, suggest marking ideas during reading
- **Parse errors**: Report which annotations could not be parsed and continue with valid ones
- **File exists**: If a permanent note file already exists, append a number (e.g., `idea-title-2.md`)

## Example

### Input Annotation (in Obsidian file)

```markdown
<mark style="background-color: #5fb236">Highlight</mark>
**IDEA: Transfer learning trades data requirements for compute - leverage existing knowledge**
"Transfer learning reduced training data requirements by 60% while maintaining accuracy"
[@smithML2023 p. 18]
```

### Output Permanent Note

File: `Permanent/transfer-learning-trades-data-requirements-for-compute.md`

```markdown
---
type: permanent
source: "[@smithML2023 p. 18]"
created: 2026-01-06
tags:
  - permanent
  - finding
---

# Transfer learning trades data requirements for compute

Transfer learning allows models to leverage knowledge from pre-trained models, dramatically reducing the amount of training data needed. Instead of learning everything from scratch, the model starts with existing knowledge and adapts it to the new task. This represents a fundamental trade-off: compute (for pre-training) substitutes for data (for task-specific training).

---

**Source:** [@smithML2023 p. 18]

> "Transfer learning reduced training data requirements by 60% while maintaining accuracy"
```

## Multiple Citekeys

To extract permanent notes from multiple sources at once, run the command multiple times or use a shell loop:

```
/readassist-permanent-note smithML2023
/readassist-permanent-note jonesAI2024
/readassist-permanent-note brownDeep2023
```

Each run creates permanent notes in the same `Permanent/` folder, building your Zettelkasten over time.
