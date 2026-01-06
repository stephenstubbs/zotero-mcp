---
description: Synthesize annotations across multiple documents from Obsidian into a unified note.
---
Execute the /readassist-synthesize command with the following arguments:
<UserRequest>
  $ARGUMENTS
</UserRequest>

# /readassist-synthesize - Multi-Document Synthesis

## Overview

The `/readassist-synthesize` command reads annotations from multiple Obsidian annotation files and creates a synthesis note that identifies common themes, groups findings by source, and highlights agreements, contradictions, and gaps across the literature. It first extracts permanent notes from `IDEA:`-prefixed annotations in all sources, then generates the synthesis with links to the extracted permanent notes grouped by source.

## Usage

```
/readassist-synthesize <citekey1> <citekey2> [citekey3...] [vault:<path>] [theme:<topic>] [output:<path>]
```

## Arguments

- `citekeys` (required): Two or more BetterBibTeX citation keys
- `vault:<path>` (optional): Path to Obsidian vault root (or set `OBSIDIAN_VAULT_PATH` env var)
- `theme:<topic>` (optional): Focus synthesis on a specific theme/topic
- `output:<path>` (optional): Output path for synthesis note (default: `1. Literature/1.4. Synthesis/<citekey>Synthesis.md`)

## Examples

```
/readassist-synthesize smithML2023 jonesAI2024 brownDeep2023
/readassist-synthesize smithML2023 jonesAI2024 theme:"methodology"
/readassist-synthesize smithML2023 jonesAI2024 brownDeep2023 output:Literature/ml-methods-synthesis.md
/readassist-synthesize smithML2023 jonesAI2024 vault:/home/user/ObsidianVault theme:"transformer models"
```

## Instructions for AI

When this command is invoked, follow these steps:

### Step 1: Locate the Vault

1. Check if `vault:` argument was provided
2. Otherwise, check environment variable `OBSIDIAN_VAULT_PATH`
3. If neither, ask the user for the vault path

### Step 2: Find All Annotation Files

For each citekey:
1. Search for files with `citekey: <citekey>` in frontmatter
2. Common locations:
   - `1. Literature/1.1. Annotations/@<citekey>Annotations.md`
   - Root of vault
3. Use `Glob` tool: `**/*<citekey>*.md`

Track which citekeys were found and which were not.

### Step 3: Read and Parse Each Annotation File

For each found file, use the `Read` tool. The files follow the Zotero Integration export template format.

#### Annotation File Structure

Each annotation file has this structure:

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

Store each annotation with its source citekey.

### Step 4: Extract Permanent Notes from All Sources

**Before generating the synthesis**, extract permanent notes from annotations marked with `IDEA:` prefix in each source.

1. **For each citekey**, filter for IDEA: annotations and create permanent notes:
   - Extract the idea text (comment after `IDEA:` prefix)
   - Extract the source text (highlighted text)
   - Extract the source reference (`[@citekey p. X]`)
   - Generate a slugified filename from the idea title

2. **Write permanent notes** to `{vault}/Permanent/` folder:
   - Use the permanent note format from `/readassist-permanent-note`
   - Track the list of created files grouped by source citekey

3. **Track created notes by source**: Store a map of `citekey → [(filename, source_reference), ...]`

**Example**: For citekey `smithML2023` with annotation `IDEA: Transfer learning trades data for compute`:
- File: `Permanent/transfer-learning-trades-data-for-compute.md`
- Track: `smithML2023 → [("transfer-learning-trades-data-for-compute", "[@smithML2023 p. 18]")]`

**If no IDEA: annotations exist in any source**: Skip this step and omit the Permanent Notes section from output.

### Step 5: Identify Common Themes

1. **Extract section headings** from all documents (section1/2/3 annotations)
2. **Group similar headings** (e.g., "Methods", "Methodology", "Approach" are related)
3. **If theme: argument provided**, filter/prioritize annotations related to that theme
4. **Identify emergent themes** from comment prefixes and content

### Step 6: Analyze Cross-Document Patterns

Look for:
- **Agreements**: Similar claims/findings across sources
- **Contradictions**: Conflicting claims between sources
- **Gaps**: Topics mentioned in one source but not others
- **Trends**: Patterns that emerge across the literature

### Step 7: Generate Synthesis Note

```markdown
---
type: synthesis
sources:
  - "[@{citekey1}]"
  - "[@{citekey2}]"
  - "[@{citekey3}]"
theme: {theme if provided, else null}
created: {today's date}
status: draft
tags:
  - synthesis
  - literature-review
---

# Synthesis: {theme or "Literature Review"}

## Overview

{AI-generated 2-3 sentence overview of what these sources cover and their main contributions}

## Sources

| Source | Focus | Key Contribution |
|--------|-------|------------------|
| [@{citekey1}] | {brief focus} | {main finding} |
| [@{citekey2}] | {brief focus} | {main finding} |
| [@{citekey3}] | {brief focus} | {main finding} |

## Common Themes

### {Theme 1 - from section headings or emergent}

**From [@{citekey1}]:**
- {annotation text} [@{citekey1} p. {page}]

**From [@{citekey2}]:**
- {annotation text} [@{citekey2} p. {page}]

### {Theme 2}

**From [@{citekey1}]:**
- {annotation text} [@{citekey1} p. {page}]

**From [@{citekey3}]:**
- {annotation text} [@{citekey3} p. {page}]

## Key Findings

{All positive/green annotations grouped together with source attribution}

- {finding} — [@{source} p. {page}]
- {finding} — [@{source} p. {page}]
- {finding} — [@{source} p. {page}]

## Critical Points

{All negative/red annotations grouped together}

- {limitation/criticism} — [@{source} p. {page}]
- {limitation/criticism} — [@{source} p. {page}]

## Open Questions

{All question/yellow annotations grouped together}

- {question/gap} — [@{source} p. {page}]
- {question/gap} — [@{source} p. {page}]

## Methodology Comparison

{Detail/grey annotations related to methods}

| Aspect | {citekey1} | {citekey2} | {citekey3} |
|--------|------------|------------|------------|
| Design | {method} | {method} | {method} |
| Sample | {sample} | {sample} | {sample} |
| Analysis | {analysis} | {analysis} | {analysis} |

## Agreements & Contradictions

### Points of Agreement
- {shared finding across sources}

### Points of Contradiction
- {citekey1} claims X, but {citekey2} argues Y

### Unique Contributions
- [@{citekey1}]: {unique insight}
- [@{citekey2}]: {unique insight}

## Research Gaps

{Synthesized gaps from all sources}

1. {Gap identified across literature}
2. {Gap identified across literature}

## Next Steps

- [ ] {follow-up action}
- [ ] {follow-up action}

## Permanent Notes

{if permanent notes were extracted in Step 4:}
The following atomic ideas were extracted as permanent notes:

### From [@{citekey1}]
- [[{filename1}]] — [@{citekey1} p. {page}]
- [[{filename2}]] — [@{citekey1} p. {page}]

### From [@{citekey2}]
- [[{filename3}]] — [@{citekey2} p. {page}]

{if no IDEA: annotations existed in any source, omit this entire section}
```

### Step 8: Write the Synthesis Note

1. Determine output path:
   - Use `output:` argument if provided
   - Otherwise: `{vault}/Synthesis/synthesis-{YYYY-MM-DD}.md`
2. Use `Write` tool to create the file
3. Report:
   - File path created
   - Number of sources included
   - Number of annotations synthesized
   - Any citekeys that could not be found

## Theme-Focused Synthesis

When `theme:<topic>` is provided:

1. **Filter annotations** to those related to the theme:
   - Check comment text for theme keywords
   - Check `THEME:` prefixes
   - Include section headings that match

2. **Prioritize theme-related content** in output:
   - Theme section comes first after overview
   - Other sections show theme-relevant annotations prominently

3. **Note gaps** where sources don't address the theme

## Citation Format

Use **pandoc citation format** for all references:

- Source document: `[@{citekey}]`
- Single page reference: `[@{citekey} p. {page}]`
- Multi-page reference: `[@{citekey} pp. {start}-{end}]`
- Inline attribution: `— [@{citekey} p. {page}]`
- Frontmatter sources: `"[@{citekey}]"` (quoted for YAML array)

## Error Handling

- **Vault not found**: Ask user to provide vault path
- **Some citekeys not found**: Report which were found, which were not, proceed with found ones
- **No annotations in file**: Note the file was found but had no annotations
- **Only one citekey found**: Suggest using `/readassist-summarize` instead, or proceed with limited synthesis
- **Parse errors**: Report and continue with valid annotations

## Example Output

```markdown
---
type: synthesis
sources:
  - "[@smithML2023]"
  - "[@jonesAI2024]"
  - "[@brownDeep2023]"
theme: transformer models
created: 2024-01-15
status: draft
tags:
  - synthesis
  - literature-review
  - transformers
---

# Synthesis: Transformer Models in NLP

## Overview

These three papers examine transformer architectures for natural language processing tasks. Smith (2023) focuses on efficiency improvements, Jones (2024) explores multilingual applications, and Brown (2023) addresses scaling challenges.

## Sources

| Source | Focus | Key Contribution |
|--------|-------|------------------|
| [@smithML2023] | Efficiency | Sparse attention reduces compute by 40% |
| [@jonesAI2024] | Multilingual | Cross-lingual transfer without parallel data |
| [@brownDeep2023] | Scaling | Linear attention scales to 100K tokens |

## Common Themes

### Attention Mechanisms

**From [@smithML2023]:**
- Sparse attention patterns maintain 95% accuracy [@smithML2023 p. 8]
- Local+global attention hybrid outperforms full attention [@smithML2023 p. 12]

**From [@brownDeep2023]:**
- Linear attention achieves O(n) complexity [@brownDeep2023 p. 5]
- Kernel approximations trade 2% accuracy for 10x speed [@brownDeep2023 p. 9]

### Training Efficiency

**From [@smithML2023]:**
- Gradient checkpointing enables larger batch sizes [@smithML2023 p. 15]

**From [@jonesAI2024]:**
- Curriculum learning improves convergence 30% [@jonesAI2024 p. 22]

## Key Findings

- Sparse attention maintains accuracy while reducing compute 40% — [@smithML2023 p. 8]
- Zero-shot cross-lingual transfer achieves 85% of supervised performance — [@jonesAI2024 p. 18]
- Linear attention scales to 100K token sequences — [@brownDeep2023 p. 5]

## Critical Points

- Sparse patterns may miss long-range dependencies — [@smithML2023 p. 20]
- Cross-lingual transfer fails for low-resource language pairs — [@jonesAI2024 p. 25]
- Linear attention underperforms on tasks requiring precise position — [@brownDeep2023 p. 15]

## Open Questions

- How do sparse patterns interact with different positional encodings? — [@smithML2023 p. 22]
- Can cross-lingual transfer work for morphologically rich languages? — [@jonesAI2024 p. 28]

## Agreements & Contradictions

### Points of Agreement
- All three papers find attention mechanism is the primary compute bottleneck
- Efficiency gains possible without major accuracy loss

### Points of Contradiction
- smithML2023 advocates sparse patterns; brownDeep2023 prefers linear approximations

### Unique Contributions
- [@smithML2023]: Systematic comparison of sparsity patterns
- [@jonesAI2024]: First zero-shot multilingual framework
- [@brownDeep2023]: Theoretical analysis of attention complexity

## Research Gaps

1. No paper addresses multimodal transformers
2. Limited study of inference-time efficiency (all focus on training)
3. No comparison across different model sizes

## Next Steps

- [ ] Review multimodal transformer literature
- [ ] Compare findings with vision transformer papers
- [ ] Investigate inference optimization techniques

## Permanent Notes

The following atomic ideas were extracted as permanent notes:

### From [@smithML2023]
- [[sparse-attention-reduces-compute-40-percent]] — [@smithML2023 p. 8]
- [[local-global-attention-hybrid]] — [@smithML2023 p. 12]

### From [@jonesAI2024]
- [[zero-shot-cross-lingual-transfer]] — [@jonesAI2024 p. 18]

### From [@brownDeep2023]
- [[linear-attention-scales-to-100k-tokens]] — [@brownDeep2023 p. 5]
```
