---
description: SQ3R reading methodology for deep textbook comprehension.
---
Execute the /read-sq3r command with the following arguments:
<UserRequest>
  $ARGUMENTS
</UserRequest>

# /read-sq3r - SQ3R Reading Strategy

## Overview

SQ3R (Survey, Question, Read, Recite, Review) is a structured reading methodology designed for deep comprehension of educational material. This strategy is ideal for textbooks, study materials, and learning-focused reading.

## Usage

```
/read-sq3r <citekey> [pages:<range>] [chapters:<names>] [focus:<topic>]
```

## Arguments

- `citekey` (required): BetterBibTeX citation key (e.g., "smithMachineLearning2023")
- `pages:<range>` (optional): Page range to read (e.g., "1-10", "1,3,5")
- `chapters:<names>` (optional): Chapter/section names (e.g., "Introduction,Methods")
- `focus:<topic>` (optional): Specific topic to focus on during reading

## Examples

```
/read-sq3r smithTextbook2023 chapters:"Chapter 3"
/read-sq3r jonesStatistics2024 pages:45-60 focus:"hypothesis testing"
/read-sq3r brownML2023 chapters:"Neural Networks,Backpropagation"
```

## Color Scheme for SQ3R

SQ3R uses colors to distinguish different types of information across phases:

| Color | Semantic Name | SQ3R Usage | Example |
|-------|---------------|------------|---------|
| `section1` (Blue) | Structure-Primary | Main chapter/section headings during Survey | "Chapter 5: Data Structures" |
| `section2` (Purple) | Structure-Secondary | Subsection headings during Survey | "5.2 Binary Trees" |
| `section3` (Magenta) | Structure-Tertiary | Sub-subsections or key topics | "5.2.1 Balanced Trees" |
| `question` (Yellow) | Questions | Questions generated during Question phase | "What is the purpose of balancing?" |
| `positive` (Green) | Answers/Key Points | Answers found during Read phase | "Balancing ensures O(log n) time" |
| `negative` (Red) | Confusion/Review | Areas needing review or unclear concepts | "How does rotation work exactly?" |
| `detail` (Grey) | Supporting Details | Examples, definitions, supporting info | "Example: AVL tree rotation" |
| `code` (Orange) | Technical Content | Code, formulas, algorithms | "rotateLeft(node)" |

### Color Usage by Phase

- **Survey Phase**: Blue, Purple, Magenta (structure)
- **Question Phase**: Yellow (questions)
- **Read Phase**: Green (answers), Red (confusion), Grey (details), Orange (technical)
- **Recite Phase**: Comments on existing highlights
- **Review Phase**: Final note summarizing all colors

## Instructions for AI

When this command is invoked, execute the SQ3R methodology through five distinct phases.

### Phase 1: Survey (2-3 minutes)

**Goal**: Get the big picture before diving into details.

1. **Look up the item**:
   ```
   zotero_lookup(citekey: "<citekey>")
   ```

2. **Get the document outline**:
   ```
   zotero_get_pdf_outline(attachment_key: "<key>")
   ```

3. **Survey the structure**:
   - Read the title, chapter headings, and subheadings
   - Scan figures and their captions
   - Read the introduction paragraph
   - Read the summary/conclusion paragraph
   
4. **Create structure annotations**:
   - Highlight main section titles with `section1` (Blue)
   - Highlight subsections with `section2` (Purple)
   - Highlight key topic areas with `section3` (Magenta)
   - Add comments describing what each section covers

5. **Detect figures**:
   ```
   zotero_list_figures(attachment_key: "<key>", page: <page>)
   ```
   Note figure locations for later detailed analysis.

**Survey Output**: Report to user:
- Document structure overview
- Main topics covered
- Number of figures/diagrams
- Estimated reading time

### Phase 2: Question (2-3 minutes)

**Goal**: Convert headings into questions to guide active reading.

1. **Generate questions from each heading**:
   - Turn each section heading into a question
   - Example: "Binary Trees" → "What are binary trees and why are they useful?"
   - Example: "Implementation" → "How is this implemented?"

2. **Create question annotations**:
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<heading text>",
     page: <page>,
     color: "question",
     comment: "Q: <your generated question>"
   )
   ```

3. **Add focus questions** (if `focus:` provided):
   - Generate 2-3 specific questions related to the focus topic
   - Add these as annotations on relevant sections

**Question Output**: List of questions to be answered during reading.

### Phase 3: Read (Variable time)

**Goal**: Read actively, seeking answers to your questions.

1. **Read the content**:
   ```
   zotero_read_pdf_pages(attachment_key: "<key>", section: "<section>")
   ```
   Or by pages if no outline exists.

2. **Highlight answers** - When you find answers to questions:
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<answer text>",
     page: <page>,
     color: "positive",
     comment: "A: <summary of answer> [answers Q about <topic>]"
   )
   ```

3. **Mark confusion** - When something is unclear:
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<confusing text>",
     page: <page>,
     color: "negative",
     comment: "UNCLEAR: <what's confusing about this>"
   )
   ```

4. **Note supporting details**:
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<detail text>",
     page: <page>,
     color: "detail",
     comment: "<why this is relevant>"
   )
   ```

5. **Highlight technical content**:
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<code or formula>",
     page: <page>,
     color: "code",
     comment: "<explanation of what this does>"
   )
   ```

6. **Analyze figures**:
   - Extract important figures:
     ```
     zotero_get_figure(attachment_key: "<key>", page: <page>, figure_index: <idx>)
     ```
   - Analyze with vision AI
   - Create area annotation:
     ```
     zotero_create_area_annotation(
       attachment_key: "<key>",
       page: <page>,
       rect: [x1, y1, x2, y2],
       color: "positive",
       comment: "Figure shows: <description>. Answers Q about <topic>."
     )
     ```

### Phase 4: Recite (Per section)

**Goal**: Test yourself by summarizing in your own words.

After each major section:

1. **Summarize without looking**:
   - Create a mental summary of what you just read
   - Focus on answers to your questions

2. **Add recitation comments** to existing highlights:
   - Link answers back to original questions
   - Note which questions are now answered
   - Identify remaining gaps

3. **Check understanding**:
   - If you can't summarize, re-read the section
   - Mark unclear parts with `negative` (Red)

### Phase 5: Review (End of session)

**Goal**: Create a comprehensive summary and identify follow-up needs.

1. **Create summary note**:
   - Summarize key points learned
   - List questions answered
   - List questions still unanswered
   - Note areas needing further study

2. **Report to user**:

**Review Output Format**:
```
## SQ3R Reading Summary: [Title]

### Questions Answered
1. Q: [Question] → A: [Answer summary]
2. Q: [Question] → A: [Answer summary]
...

### Unanswered Questions
- [Question] - needs more research on [topic]

### Key Takeaways
1. [Main point 1]
2. [Main point 2]
3. [Main point 3]

### Areas Needing Review
- [Confusing topic] - page X
- [Complex concept] - page Y

### Annotations Created
- X structure highlights (blue/purple/magenta)
- Y question annotations (yellow)
- Z answer highlights (green)
- N confusion markers (red)
- M detail highlights (grey)
- K technical highlights (orange)

### Recommended Next Steps
1. [Suggestion based on unanswered questions]
2. [Related topic to explore]
```

## Best Practices for SQ3R

1. **Don't skip Survey**: The overview is crucial for context
2. **Write genuine questions**: Curiosity drives engagement
3. **Link answers to questions**: Always reference which question is being answered
4. **Mark confusion immediately**: Don't hope to understand later
5. **Recite honestly**: If you can't summarize, you haven't learned it
6. **Use figures**: Visual learning reinforces concepts
7. **Review is essential**: The final summary consolidates learning

## Error Handling

- **No outline**: Use page-by-page approach, create your own structure during Survey
- **Long document**: Focus on specific chapters; don't try to SQ3R entire books at once
- **No questions generated**: Start with basic what/why/how for each heading
- **All confusion**: Re-read more slowly, or suggest simpler prerequisite material
