---
description: Generate a summary note from Obsidian annotations for a single document.
---
Execute the /readassist-summarize command with the following arguments:
<UserRequest>
  $ARGUMENTS
</UserRequest>

# /readassist-summarize - Single Document Summary

## Overview

The `/readassist-summarize` command reads annotations from an Obsidian annotation file (created by the Zotero Integration plugin) and generates a structured summary note. It first extracts permanent notes from `IDEA:`-prefixed annotations, then groups remaining annotations by semantic color and uses section colors for organizational structure. The summary includes links to extracted permanent notes.

## Usage

```
/readassist-summarize <citekey> [vault:<path>] [output:<path>]
```

## Arguments

- `citekey` (required): BetterBibTeX citation key (e.g., "smithML2023")
- `vault:<path>` (optional): Path to Obsidian vault root (or set `OBSIDIAN_VAULT_PATH` env var)
- `output:<path>` (optional): Output path for summary note (default: `1. Literature/1.3. Summary/<citekey>Summary.md`)

## Examples

```
/readassist-summarize smithML2023
/readassist-summarize smithML2023 vault:/home/user/ObsidianVault
/readassist-summarize smithML2023 output:Summaries/ml-survey-summary.md
```

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
> **itemType**:: journalArticle
> **Journal**:: *Journal Name*
> **Volume**:: 1
> **Issue**:: 2
> **DOI**:: 10.xxxx/xxxxx
> ...other metadata...

> [!Abstract]
> Abstract text here.

# Notes
> Optional user notes

# Annotations
## Imported: YYYY-MM-DD h:mm A

{annotation blocks follow}
```

#### Frontmatter Extraction

Extract YAML frontmatter between `---` markers:
- `citekey`: citation key
- `tags`: list of tags
- `status`: annotation status

#### Metadata Extraction

From the `[!md]` callout, extract:
- `Title`: document title
- `Year`: publication year
- `FirstAuthor`: primary author

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
- `#2ea8e5` (section1) → `##` heading
- `#a28ae5` (section2) → `###` heading
- `#e56eee` (section3) → `####` heading

**Code annotation (orange #f19837):**
```markdown
<mark style="background-color: #f19837">Highlight</mark>
```python
print("code content")
```
[@citekey p. X]
```
The comment becomes the code fence language.

**Image annotation:**
```markdown
<mark style="background-color: #HEX">Image</mark>
**Comment**
![[imageBaseName]]
[@citekey p. X]
```

**Note annotation (no highlight):**
```markdown
<mark style="background-color: #HEX">Note</mark>
**Comment text only**
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

#### Comment Prefix Extraction

First, check if comment starts with `IDEA:` prefix. If so, strip it before processing:
- `IDEA: FINDING: Deep learning improves accuracy` → process as `FINDING: Deep learning improves accuracy`
- `IDEA: Transfer learning trades data for compute` → process as `Transfer learning trades data for compute`

Then check for semantic prefixes:
```
THESIS:, PREMISE:, EVIDENCE:, CLAIM:, A:, FINDING:, CORE:
WEAKNESS:, LIMITATION:, UNCLEAR:, CONCERN:
Q:, GAP:, RELEVANT:
ASSUMPTION:, TERM:, CONNECTION:, METHOD:, THEME:, DETAIL:
STAT:, CODE:, DATA:
```

### Step 4: Extract Permanent Notes

**Before generating the summary**, extract permanent notes from annotations marked with `IDEA:` prefix.

1. **Filter for IDEA: annotations**: From all parsed annotations, identify those where the comment starts with `IDEA:`

2. **For each IDEA annotation**, create a permanent note:
   - Extract the idea text (comment after `IDEA:` prefix)
   - Extract the source text (highlighted text)
   - Extract the source reference (`[@citekey p. X]`)
   - Generate a slugified filename from the idea title

3. **Write permanent notes** to `{vault}/Permanent/` folder:
   - Use the permanent note format from `/readassist-permanent-note`
   - Track the list of created files (filenames without extension) for linking

4. **Track created notes**: Store a list of `(filename, source_reference)` pairs for the Permanent Notes section

**Example**: If annotation has comment `IDEA: Transfer learning trades data for compute`, create:
- File: `Permanent/transfer-learning-trades-data-for-compute.md`
- Track: `("transfer-learning-trades-data-for-compute", "[@smithML2023 p. 18]")`

**If no IDEA: annotations exist**: Skip this step and omit the Permanent Notes section from output.

### Step 5: Organize Annotations

Group parsed annotations by category:

**Hierarchy annotations** (section1/2/3): These provide document structure
**Semantic annotations**: Group by color:
- `positive`: Key findings, evidence, claims
- `negative`: Criticisms, limitations, concerns
- `question`: Questions, gaps, uncertainties
- `detail`: Definitions, methodology, context
- `code`: Technical content, statistics, data

### Step 6: Generate Summary Note

Choose output format based on whether section colors are present:

#### Format A: Structure-Aware (when section colors exist)

Use section annotations as organizational headings, with semantic annotations grouped under their nearest section:

```markdown
---
type: summary
source: "[@{citekey}]"
created: {today's date}
status: draft
tags:
  - summary
  - {tags from original}
---

# Summary: {title or citekey}

> [!info] Source
> [@{citekey}]

## {section1 comment}

### Key Points
- {positive annotation text} [@{citekey} p. {page}]

### Critical Notes
- {negative annotation text} [@{citekey} p. {page}]

### Questions
- {question annotation text} [@{citekey} p. {page}]

## {next section1 comment}
...

## Permanent Notes

{if permanent notes were extracted in Step 4:}
The following atomic ideas were extracted as permanent notes:

- [[{filename1}]] — [@{citekey} p. {page}]
- [[{filename2}]] — [@{citekey} p. {page}]

{if no IDEA: annotations existed, omit this entire section}
```

#### Format B: Semantic Grouping (when no section colors)

Group all annotations by semantic color:

```markdown
---
type: summary
source: "[@{citekey}]"
created: {today's date}
status: draft
tags:
  - summary
  - {tags from original}
---

# Summary: {title or citekey}

> [!info] Source
> [@{citekey}]

## Key Points
{for each positive annotation:}
- {text} [@{citekey} p. {page}]
  {if comment and comment != text:} *{comment}*

## Critical Notes
{for each negative annotation:}
- {text} [@{citekey} p. {page}]
  {if comment:} *{comment}*

## Questions & Gaps
{for each question annotation:}
- {text} [@{citekey} p. {page}]
  {if comment:} *{comment}*

## Methodology & Context
{for each detail annotation:}
- {text} [@{citekey} p. {page}]
  {if comment:} *{comment}*

## Technical Details
{for each code annotation:}
- `{text}` [@{citekey} p. {page}]
  {if comment:} *{comment}*

## Permanent Notes

{if permanent notes were extracted in Step 4:}
The following atomic ideas were extracted as permanent notes:

- [[{filename1}]] — [@{citekey} p. {page}]
- [[{filename2}]] — [@{citekey} p. {page}]

{if no IDEA: annotations existed, omit this entire section}
```

### Step 7: Write the Summary Note

1. Determine output path:
   - Use `output:` argument if provided
   - Otherwise: `{vault}/Synthesis/{citekey}-summary.md`
2. Use `Write` tool to create the file
3. Report the file path to the user

## Citation Format

Use **pandoc citation format** for all references:

- Source document: `[@{citekey}]`
- Single page reference: `[@{citekey} p. {page}]`
- Multi-page reference: `[@{citekey} pp. {start}-{end}]`
- Frontmatter source: `"[@{citekey}]"` (quoted for YAML)

## Error Handling

- **Vault not found**: Ask user to provide vault path
- **Annotation file not found**: Report citekey not found, suggest checking vault location
- **No annotations**: Report that the file has no parsed annotations
- **Parse errors**: Report which annotations could not be parsed and continue with valid ones

## Example Output

```markdown
---
type: summary
source: "[@smithML2023]"
created: 2024-01-15
status: draft
tags:
  - summary
  - machine-learning
---

# Summary: Machine Learning in Healthcare

> [!info] Source
> [@smithML2023]

## Key Points
- Deep learning models achieved 94% accuracy on diagnostic tasks [@smithML2023 p. 12]
  *FINDING: Significant improvement over baseline*
- Transfer learning reduced training data requirements by 60% [@smithML2023 p. 18]

## Critical Notes
- Small sample size limits generalizability [@smithML2023 p. 25]
  *LIMITATION: N=150 patients*
- No external validation performed [@smithML2023 p. 26]

## Questions & Gaps
- How does performance vary across demographic groups? [@smithML2023 p. 30]
  *GAP: No subgroup analysis*

## Methodology & Context
- Retrospective cohort study design [@smithML2023 p. 8]
  *METHOD: Chart review 2018-2022*

## Technical Details
- `AUC = 0.92, 95% CI [0.89, 0.95]` [@smithML2023 p. 15]
  *STAT: Primary outcome metric*

## Permanent Notes

The following atomic ideas were extracted as permanent notes:

- [[transfer-learning-trades-data-for-compute]] — [@smithML2023 p. 18]
- [[deep-learning-outperforms-traditional-ml]] — [@smithML2023 p. 12]
```
