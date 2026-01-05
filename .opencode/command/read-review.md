---
description: Literature review strategy for systematic evidence extraction and synthesis.
---
Execute the /read-review command with the following arguments:
<UserRequest>
  $ARGUMENTS
</UserRequest>

# /read-review - Literature Review Strategy

## Overview

The Literature Review strategy is designed for systematic evidence extraction from research papers. It focuses on identifying claims, methods, findings, and limitations to build a synthesis matrix for comprehensive literature analysis.

## Usage

```
/read-review <citekey> [pages:<range>] [extraction:<types>] [theme:<topic>]
```

## Arguments

- `citekey` (required): BetterBibTeX citation key (e.g., "smithMachineLearning2023")
- `pages:<range>` (optional): Page range to read (e.g., "1-10", "all")
- `extraction:<types>` (optional): Focus on specific elements (e.g., "claims,methods,findings")
- `theme:<topic>` (optional): Thematic tag for categorization

## Examples

```
/read-review smithML2023
/read-review jonesNLP2024 extraction:"methods,findings" theme:"transformer models"
/read-review brownStats2023 pages:1-15 theme:"regression analysis"
```

## Color Scheme for Literature Review

Literature Review uses colors to categorize different types of evidence.

### Hierarchy Colors (Generate Obsidian Headings)

| Color | Hex | Review Usage | Obsidian Result |
|-------|-----|--------------|-----------------|
| `section1` (Blue) | #2ea8e5 | Document section headings (e.g., "Methods", "Results") | `## Heading` (H2) |
| `section2` (Purple) | #a28ae5 | Subsection headings | `### Heading` (H3) |
| `section3` (Magenta) | #e56eee | Sub-subsection headings | `#### Heading` (H4) |

### Semantic Colors (Content Meaning)

| Color | Hex | Review Usage | Comment Prefix | Example |
|-------|-----|--------------|----------------|---------|
| `positive` (Green) | #5fb236 | Key findings, supported claims | `CLAIM:` | "CLAIM: Deep learning outperforms ML" |
| `negative` (Red) | #ff6666 | Limitations, biases, weaknesses | `LIMITATION:` | "LIMITATION: Small sample size" |
| `question` (Yellow) | #ffd400 | Research gaps, future directions | `GAP:` | "GAP: No long-term studies exist" |
| `detail` (Grey) | #aaaaaa | Methodology, themes, context | `METHOD:`, `THEME [x]:` | "METHOD: Randomized controlled trial" |
| `code` (Orange) | #f19837 | Statistics, quantitative results | `STAT:` | "STAT: r=0.85, p<0.05" |

### Comment Prefixes for Literature Review

| Prefix | Color | Usage |
|--------|-------|-------|
| `CLAIM:` | Green | Supported claims with evidence strength |
| `LIMITATION:` | Red | Study limitations and validity threats |
| `GAP:` | Yellow | Research gaps and future directions |
| `METHOD:` | Grey | Study design, procedures, sample details |
| `THEME [x]:` | Grey | Thematic categorization (e.g., `THEME [performance]:`) |
| `STAT:` | Orange | Statistics and quantitative results |
| `FIGURE:` | Green | Figure analysis and key visual findings |

### Color Usage by Extraction Phase

- **Metadata Phase**: Not annotated (captured in notes)
- **Evidence Phase**: Green (`CLAIM:`), Red (`LIMITATION:`), Orange (`STAT:`)
- **Quality Phase**: Red (`LIMITATION:`), Yellow (`GAP:`)
- **Categorization Phase**: Grey (`THEME [x]:`), Grey (`METHOD:`)
- **Structure Phase**: Blue/Purple/Magenta (document headings only)

## Instructions for AI

When this command is invoked, execute the Literature Review methodology through four phases.

### Phase 1: Metadata Extraction

**Goal**: Capture bibliographic and study characteristics.

1. **Look up the item**:
   ```
   zotero_lookup(citekey: "<citekey>")
   ```

2. **Extract metadata** (store mentally for synthesis matrix):
   - Authors and year
   - Journal/venue
   - Study type (empirical, theoretical, review, meta-analysis)
   - Research design (experimental, observational, qualitative, mixed)
   - Sample characteristics (size, population, setting)
   - Domain/field

3. **Get document structure**:
   ```
   zotero_get_pdf_outline(attachment_key: "<key>")
   ```

4. **Read abstract**:
   ```
   zotero_read_pdf_pages(attachment_key: "<key>", pages: "1")
   ```
   Extract: research question, methodology, key findings.

**Metadata Output**: Brief summary of study characteristics.

### Phase 2: Evidence Extraction

**Goal**: Systematically identify and highlight claims, methods, and findings.

1. **Read the full paper or specified sections**:
   ```
   zotero_read_pdf_pages(attachment_key: "<key>", section: "<section>")
   ```

2. **Highlight supported claims** (Green):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<claim text>",
     page: <page>,
     color: "positive",
     comment: "CLAIM: <restated claim> | Evidence: <type> | Strength: <high/medium/low>"
   )
   ```

   Example comment:
   ```
   "CLAIM: Deep learning outperforms traditional ML | Evidence: Empirical (controlled experiment) | Strength: High (large sample, rigorous design)"
   ```

3. **Highlight limitations** (Red):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<limitation text>",
     page: <page>,
     color: "negative",
     comment: "LIMITATION: <type> - <impact on validity>"
   )
   ```

   Limitation types:
   - Sample: size, selection bias, generalizability
   - Design: confounds, measurement issues
   - Analysis: statistical limitations
   - Scope: narrow focus, missing variables

4. **Highlight statistics and data** (Orange):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<statistic>",
     page: <page>,
     color: "code",
     comment: "STAT: <metric> = <value> | Context: <what this means>"
   )
   ```

5. **Highlight methodology details** (Grey):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<method detail>",
     page: <page>,
     color: "detail",
     comment: "METHOD: <brief summary of procedure>"
   )
   ```

6. **Extract and analyze figures**:
   ```
   zotero_list_figures(attachment_key: "<key>", page: <page>)
   zotero_get_figure(attachment_key: "<key>", page: <page>, figure_index: <idx>)
   ```
   
   Annotate results figures:
   ```
   zotero_create_area_annotation(
     attachment_key: "<key>",
     page: <page>,
     rect: [x1, y1, x2, y2],
     color: "positive",
     comment: "FIGURE: <description> | Key finding: <what it shows>"
   )
   ```

### Phase 3: Quality Assessment

**Goal**: Evaluate study rigor and identify potential biases.

1. **Assess internal validity**:
   - Control groups present?
   - Randomization used?
   - Blinding implemented?
   - Confounds addressed?

2. **Assess external validity**:
   - Sample representativeness
   - Setting generalizability
   - Time period relevance

3. **Identify biases** (Red highlights if found in text):
   - Selection bias
   - Publication bias
   - Confirmation bias
   - Funding conflicts

4. **Note research gaps** (Yellow):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<gap statement>",
     page: <page>,
     color: "question",
     comment: "GAP: <what's missing> | Implication: <why it matters>"
   )
   ```

**Quality Output**: Rate overall study quality (High/Medium/Low) with justification.

### Phase 4: Categorization

**Goal**: Tag findings by theme for synthesis matrix.

1. **Apply thematic tags** using Grey with `THEME [x]:` prefix:

   If `theme:` argument provided, use it as primary theme:
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<relevant claim>",
     page: <page>,
     color: "detail",
     comment: "THEME [<theme name>]: <how this relates>"
   )
   ```

2. **Identify emergent themes** (all use Grey with different prefixes):
   - Performance/Effectiveness → `THEME [performance]:`
   - Methodology/Approach → `THEME [methodology]:`
   - Applications/Use Cases → `THEME [applications]:`

3. **Cross-reference with existing literature**:
   - Note agreements with other papers
   - Note disagreements or contradictions
   - Identify unique contributions

**Note**: Section colors (Blue/Purple/Magenta) are reserved for document structure headings only. Use comment prefixes on Grey highlights for thematic categorization.

## Output: Synthesis Matrix Entry

After completing the review, provide a structured extraction:

```
## Literature Review: [Title] ([Author], [Year])

### Study Characteristics
- **Type**: [Empirical/Theoretical/Review/Meta-analysis]
- **Design**: [RCT/Quasi-experimental/Observational/Qualitative]
- **Sample**: [N=X, population description]
- **Setting**: [Context/domain]

### Key Findings
1. [Finding 1] - Evidence strength: [High/Medium/Low]
2. [Finding 2] - Evidence strength: [High/Medium/Low]
3. [Finding 3] - Evidence strength: [High/Medium/Low]

### Methodology Summary
- [Key methodological approach]
- [Data collection methods]
- [Analysis techniques]

### Limitations
1. [Limitation 1] - Impact: [Major/Minor]
2. [Limitation 2] - Impact: [Major/Minor]

### Quality Assessment
- **Overall**: [High/Medium/Low]
- **Internal Validity**: [Rating + justification]
- **External Validity**: [Rating + justification]
- **Potential Biases**: [List any identified]

### Thematic Categorization
- Primary Theme: [Theme] - [Relevant findings]
- Secondary Themes: [Theme list]

### Research Gaps Identified
- [Gap 1]
- [Gap 2]

### Annotations Created
- X claims highlighted (green)
- Y limitations marked (red)
- Z statistics noted (orange)
- N methodology/theme details (grey)
- M gaps identified (yellow)
- K structure headings (blue/purple/magenta)

### Synthesis Notes
- **Agrees with**: [Other papers on topic]
- **Contradicts**: [Other papers, if any]
- **Unique contribution**: [What's new here]
```

## Best Practices for Literature Review

1. **Be systematic**: Extract the same elements from every paper
2. **Use consistent language**: Standardize how you describe claims and limitations
3. **Quote precisely**: Exact text helps with later citation
4. **Rate evidence strength**: Not all findings are equally robust
5. **Note page numbers**: Makes verification easy
6. **Track themes**: Build your synthesis matrix as you go
7. **Be critical but fair**: Note limitations without dismissing the work

## Error Handling

- **No methods section**: Check for "Methodology" or "Approach" headings
- **No explicit limitations**: Note this as a potential weakness
- **Statistical details missing**: Flag as limitation for quantitative claims
- **Multiple studies in one paper**: Extract separately for each study
