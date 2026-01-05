---
description: Quick skim strategy for rapid relevance assessment and key point extraction.
---
Execute the /read-skim command with the following arguments:
<UserRequest>
  $ARGUMENTS
</UserRequest>

# /read-skim - Skim Reading Strategy

## Overview

The Skim strategy is designed for rapid assessment of a paper's relevance and key findings. It focuses on extracting maximum value in minimum time by reading only strategic portions of the document. Ideal for literature surveys, triage of search results, or deciding whether to read a paper in depth.

## Usage

```
/read-skim <citekey> [time:<minutes>] [focus:<topic>]
```

## Arguments

- `citekey` (required): BetterBibTeX citation key (e.g., "smithMachineLearning2023")
- `time:<minutes>` (optional): Time budget in minutes - `5m` or `10m` (default: 10m)
- `focus:<topic>` (optional): Specific topic to look for

## Examples

```
/read-skim smithML2023
/read-skim jonesNLP2024 time:5m
/read-skim brownStats2023 time:10m focus:"neural networks"
```

## Time Budgets

| Time | Scope | Best For |
|------|-------|----------|
| `5m` | Title, abstract, headings, conclusion only | Quick relevance check |
| `10m` | Above + key figures, introduction highlights | Deciding whether to read fully |

## Color Scheme for Skimming

Skim uses a **minimal color set** (3-4 colors) to avoid overhead:

| Color | Semantic Name | Skim Usage | Example |
|-------|---------------|------------|---------|
| `positive` (Green) | Key Findings | Main results, important conclusions | "Accuracy improved by 15%" |
| `section1` (Blue) | Main Topic | Core subject/contribution | "We propose a novel architecture" |
| `question` (Yellow) | Relevance Flag | Points relevant to your focus | "Applicable to NLP tasks" |
| `negative` (Red) | Concern/Limitation | Major limitations or red flags | "Only tested on synthetic data" |

### Colors NOT Used in Skim Strategy

To keep skimming fast, these colors are **not used**:
- `section2` (Purple) - Too detailed for skimming
- `section3` (Magenta) - Too detailed for skimming
- `detail` (Grey) - Background info not needed for skim
- `code` (Orange) - Technical details saved for full reading

### Annotation Philosophy

- **Fewer is better**: Aim for 3-8 total annotations
- **Only essential points**: If you wouldn't remember it, don't annotate
- **Quick comments**: One-line comments only
- **No figure analysis**: Just note presence, don't extract images

## Instructions for AI

When this command is invoked, execute a rapid skim reading focused on speed and relevance assessment.

### Time Management

**5-minute budget**:
- 1 min: Metadata + abstract
- 1 min: Headings scan
- 1 min: Conclusion
- 2 min: Create annotations + summary

**10-minute budget**:
- 2 min: Metadata + abstract
- 2 min: Introduction highlights
- 2 min: Headings + figure captions
- 2 min: Conclusion
- 2 min: Create annotations + summary

### Step 1: Quick Lookup (30 seconds)

1. **Get the item**:
   ```
   zotero_lookup(citekey: "<citekey>")
   ```

2. **Note metadata** (don't annotate, just remember):
   - Authors, year, venue
   - Paper type (empirical, theoretical, survey)

### Step 2: Abstract (1-2 minutes)

1. **Read first page**:
   ```
   zotero_read_pdf_pages(attachment_key: "<key>", pages: "1")
   ```

2. **Extract from abstract**:
   - Research question/problem
   - Approach/method (one line)
   - Key finding/contribution

3. **One annotation only** (Blue - main topic):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<core contribution sentence>",
     page: 1,
     color: "section1",
     comment: "CORE: <one-line summary>"
   )
   ```

4. **If `focus:` provided**, check relevance:
   - Does abstract mention the focus topic?
   - If yes, add Yellow highlight on relevant phrase

### Step 3: Structure Scan (1-2 minutes)

1. **Get outline**:
   ```
   zotero_get_pdf_outline(attachment_key: "<key>")
   ```

2. **Note structure mentally**:
   - Number of sections
   - Methodology presence
   - Results/experiments section
   - Standard vs. unusual organization

3. **No annotations in this step** - just orientation

### Step 4: Figures Overview (10-minute budget only)

1. **Quick figure detection on key pages**:
   ```
   zotero_list_figures(attachment_key: "<key>", page: <results_page>)
   ```

2. **Do NOT extract figures** - just note:
   - Number of figures
   - Types (charts, diagrams, tables)
   - What they appear to show (from captions)

3. **If a figure looks crucial** (Green annotation on caption):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<figure caption>",
     page: <page>,
     color: "positive",
     comment: "KEY FIGURE: <what it shows>"
   )
   ```

### Step 5: Conclusion (1-2 minutes)

1. **Read conclusion section**:
   ```
   zotero_read_pdf_pages(attachment_key: "<key>", section: "Conclusion")
   ```
   Or last 1-2 pages if no outline.

2. **Highlight key findings** (Green):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<main finding>",
     page: <page>,
     color: "positive",
     comment: "FINDING: <brief summary>"
   )
   ```
   
   Maximum 2 findings highlights.

3. **Note any major limitations** (Red - only if prominent):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<limitation>",
     page: <page>,
     color: "negative",
     comment: "LIMITATION: <brief note>"
   )
   ```

### Step 6: Introduction Scan (10-minute budget only)

1. **Read introduction**:
   ```
   zotero_read_pdf_pages(attachment_key: "<key>", section: "Introduction")
   ```

2. **Look for**:
   - Problem statement
   - Gap in existing work
   - Paper's contribution claims

3. **One annotation if valuable** (Blue or Yellow):
   - Blue if it clarifies the contribution
   - Yellow if it connects to your focus topic

## Output: Skim Summary

Provide a rapid assessment:

```
## Skim Summary: [Title]
**Time spent**: [X] minutes

### Relevance Assessment
**Relevance to [focus/general research]**: [High/Medium/Low/Not Relevant]
**Recommendation**: [Read fully / Read specific sections / Skip / Archive for later]

### One-Sentence Summary
[Single sentence capturing the paper's core contribution]

### Key Points
1. [Main finding/contribution]
2. [Secondary point if time allowed]
3. [Third point if very relevant]

### Paper Characteristics
- **Type**: [Empirical/Theoretical/Survey/Position]
- **Methodology**: [Brief - e.g., "Large-scale experiment" or "Theoretical analysis"]
- **Domain**: [Application area]

### Red Flags (if any)
- [Any major concerns spotted]

### Relevance to [Focus Topic] (if focus provided)
- [Direct/Indirect/Tangential/None]
- [Specific connection if found]

### Sections to Read if Continuing
1. [Most important section]
2. [Second priority]

### Annotations Created
- [Number] annotations total
- Colors used: [list]

### Quick Stats
- Pages: [total]
- Figures: [count]
- Year: [publication year]
```

## Skim vs. Full Read Decision Tree

After skimming, recommend next steps:

```
Is topic directly relevant?
├─ No → "Skip - not relevant to [focus]"
├─ Unsure → "Archive for later review"
└─ Yes → Check quality
    ├─ Major red flags? → "Skip - [reason]"
    └─ Looks solid → Check depth needed
        ├─ Need details? → "Full read recommended"
        ├─ Need methods? → "Read Methods section only"
        ├─ Need results? → "Read Results + Figures"
        └─ Just citations? → "Use for references only"
```

## Best Practices for Skimming

1. **Don't get pulled in**: Resist the urge to read interesting tangents
2. **Trust the structure**: Conclusions usually contain key findings
3. **Figures tell stories**: Captions often summarize results
4. **Abstract is reliable**: Authors put their best summary there
5. **Watch the clock**: Stop when time budget expires
6. **Minimal annotations**: More than 8 means you're not skimming
7. **Make a decision**: The goal is to decide read/skip, not understand fully

## Error Handling

- **No abstract**: Read first 2 paragraphs of introduction instead
- **No conclusion**: Read last section, whatever it's called
- **No outline**: Use page numbers (first page, last 2 pages)
- **Very long paper**: Stick to abstract + conclusion, note length as consideration
- **Non-standard structure**: Focus on identifying methodology and results sections

## What Skimming is NOT

- **Not thorough**: You will miss details - that's okay
- **Not for retention**: This is for triage, not learning
- **Not for citation**: Don't cite based on skim alone
- **Not for complex texts**: Philosophy/theory papers need full reading
