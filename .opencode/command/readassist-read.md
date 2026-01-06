---
description: Initiate an AI-assisted critical reading session for a Zotero item.
---
Execute the /readassist-read command with the following arguments:
<UserRequest>
  $ARGUMENTS
</UserRequest>

# /readassist-read - Critical Reading Workflow

## Usage

```
/readassist-read <citekey> [pages:<range>] [chapters:<names>] [from_page:<N>] [purpose:<text>] [strategy:<name>]
```

## Arguments

- `citekey` (required): BetterBibTeX citation key (e.g., "smithMachineLearning2023")
- `pages:<range>` (optional): Page range to read (e.g., "1-10", "1,3,5", "all")
- `chapters:<names>` (optional): Chapter/section names to read (e.g., "Introduction,Methods")
- `from_page:<N>` (optional): Start reading from page N (skips outline lookup)
- `purpose:<text>` (optional): Reading purpose to focus analysis
- `strategy:<name>` (optional): Reading strategy to use (default: "critical")

## Available Strategies

| Strategy | Command | Best For |
|----------|---------|----------|
| `critical` | `/readassist-read` (default) | General critical reading and analysis |
| `sq3r` | `/readassist-read-sq3r` | Textbook learning, deep comprehension |
| `review` | `/readassist-read-review` | Literature review, evidence extraction |
| `analyze` | `/readassist-read-analyze` | Argument analysis, philosophical texts |
| `skim` | `/readassist-read-skim` | Quick relevance assessment |

Each strategy has a dedicated command file with detailed instructions. Use `/readassist-read strategy:<name>` for quick access or the dedicated commands for full documentation.

## Examples

```
/readassist-read smithML2023 pages:1-10 purpose:"understand methodology"
/readassist-read jonesDeepLearning2024 chapters:"Introduction,Results"
/readassist-read brownNLP2023
/readassist-read brownNLP2023 from_page:1
/readassist-read smithTextbook2023 strategy:sq3r
/readassist-read jonesResearch2024 strategy:review
/readassist-read brownPhilosophy2023 strategy:analyze
/readassist-read smithPaper2024 strategy:skim
```

## Instructions for AI

When this command is invoked, parse the `<UserRequest>` block above to extract:
1. `citekey` - the first argument (required)
2. `pages:` - page range if provided
3. `chapters:` - section names if provided  
4. `from_page:` - starting page if provided
5. `purpose:` - reading purpose if provided
6. `strategy:` - reading strategy if provided (default: "critical")

### Strategy Routing

If a `strategy:` parameter is provided, use the corresponding methodology:

| Strategy Value | Behavior |
|----------------|----------|
| `critical` or not specified | Use the default critical reading workflow below |
| `sq3r` | Follow the `/readassist-read-sq3r` methodology (Survey, Question, Read, Recite, Review) |
| `review` | Follow the `/readassist-read-review` methodology (Evidence extraction for literature review) |
| `analyze` | Follow the `/readassist-read-analyze` methodology (Deep argument analysis) |
| `skim` | Follow the `/readassist-read-skim` methodology (Quick relevance assessment) |

**Invalid strategy**: If an unknown strategy is specified, inform the user:
```
Unknown strategy "[name]". Available strategies:
- critical (default): General critical reading
- sq3r: Textbook learning methodology
- review: Literature review extraction
- analyze: Deep argument analysis
- skim: Quick relevance assessment

Use /readassist-read --help or see strategy-specific commands (/readassist-read-sq3r, /readassist-read-review, etc.) for details.
```

### Default Critical Reading Workflow

When using the default "critical" strategy, follow this workflow:

### Step 1: Look Up the Item

Use the `zotero_lookup` tool to find the item by citation key:
```
zotero_lookup(citekey: "<citekey>")
```

This returns the item metadata and PDF attachment keys. Note the `attachment_key` for subsequent operations.

### Step 2: Check for Document Outline (Outline-First Workflow)

**IMPORTANT**: Unless `pages:` or `from_page:` is explicitly provided, always check for an outline first.

Use `zotero_get_pdf_outline` to discover the document structure:
```
zotero_get_pdf_outline(attachment_key: "<key>")
```

This returns:
```json
{
  "has_outline": true,
  "total_pages": 42,
  "items": [
    {"title": "Introduction", "page": 0, "children": []},
    {"title": "Methods", "page": 5, "children": [
      {"title": "Data Collection", "page": 6, "children": []},
      {"title": "Analysis", "page": 10, "children": []}
    ]},
    {"title": "Results", "page": 15, "children": []}
  ]
}
```

#### Decision Tree:

```
1. If `pages:` argument provided → Skip to Step 3 (use page range)
2. If `from_page:` argument provided → Skip to Step 3 (read from that page)
3. If `chapters:` argument provided → Use section names in Step 3
4. Otherwise → Call zotero_get_pdf_outline:
   a. If has_outline is true:
      - Present available sections to guide reading
      - Use section parameter for reading
   b. If has_outline is false:
      - Inform user: "This PDF has no bookmarks/outline"
      - Ask user for specific page numbers, OR
      - Offer to read from the beginning
```

### Step 3: Read PDF Content

**Option A: Read by page range** (when `pages:` or `from_page:` provided)
```
zotero_read_pdf_pages(attachment_key: "<key>", pages: "<range>")
```

**Option B: Read by section name** (when outline exists)
```
zotero_read_pdf_pages(attachment_key: "<key>", section: "Introduction")
```

Multiple sections can be comma-separated:
```
zotero_read_pdf_pages(attachment_key: "<key>", section: "Introduction,Methods")
```

### Step 4: Detect and Extract Figures

**IMPORTANT**: After reading text content, automatically detect figures, charts, and diagrams on each page.

For each page that was read, call `zotero_list_figures` to find visual content:
```
zotero_list_figures(attachment_key: "<key>", page: <page_number>)
```

This returns a list of detected figures with their locations and types:
```json
[
  {
    "index": 0,
    "rect": [72.0, 200.5, 523.0, 680.3],
    "figure_type": "chart",
    "confidence": 0.85,
    "width": 451.0,
    "height": 479.8
  }
]
```

#### When to Extract Figures

Extract and analyze figures in these situations:

1. **Text references a figure** (e.g., "as shown in Figure 3", "see diagram above")
2. **High-confidence detections** (confidence ≥ 0.7)
3. **Large figures** that likely contain important information (width/height > 200 points)
4. **When reading purpose** suggests visual analysis is important (e.g., "understand the architecture", "analyze results")

#### Extracting Figure Images

When you identify an important figure, extract it as an image:
```
zotero_get_figure(
  attachment_key: "<key>",
  page: <page_number>,
  figure_index: <index>,
  format: "jpeg",
  include_context: false
)
```

This returns a file path to the extracted figure image:
```json
{
  "file_path": "/tmp/zotero-figure-KEY-p5-f0-1234567890.jpg",
  "mime_type": "image/jpeg"
}
```

**Then analyze the image** using your vision capabilities and incorporate insights into your reading notes.

### Step 5: Analyze Content (Text + Images)

Apply critical reading techniques based on the purpose, combining both text and visual analysis:

- **Identify key claims and arguments** from text
- **Analyze figures, charts, and diagrams** for supporting evidence
- **Note data from visualizations** (trends, comparisons, results)
- **Mark areas of agreement/disagreement**
- **Highlight technical terms and definitions**
- **Cross-reference text and figures** (e.g., "Figure 3 confirms the claim in paragraph 2")
- **Identify visual patterns** not explicitly stated in text

### Step 6: Create Annotations

Use semantic highlighting to annotate the PDF:

#### Text Highlights (`zotero_create_highlight`)

```
zotero_create_highlight(
  attachment_key: "<key>",
  text: "<exact text from PDF>",
  page: <1-based page number>,
  color: "<semantic color>",
  comment: "<your note>"
)
```

#### Area Annotations for Figures (`zotero_create_area_annotation`)

**CRITICAL**: After analyzing a figure image, create an area annotation with insights from the visual analysis:

```
zotero_create_area_annotation(
  attachment_key: "<key>",
  page: <1-based page number>,
  rect: [x1, y1, x2, y2],  # Use the rect from zotero_list_figures
  color: "<semantic color>",
  comment: "<description of what you saw in the image>"
)
```

**Example comment for a figure annotation:**
```
"Bar chart showing 3 algorithms. Algorithm A: 94% accuracy, Algorithm B: 87%, Algorithm C: 91%. 
Algorithm A is clearly best but has higher latency (shown in Figure 4)."
```

This creates a visual annotation box around the figure with your detailed analysis embedded as a comment.

## Color Philosophy

Colors are divided into two categories:

### Hierarchy Colors (Section Colors)
These colors generate **headings in Obsidian** when imported. Use them ONLY for document structure:
- `section1` (Blue) → H2 heading in Obsidian
- `section2` (Purple) → H3 heading in Obsidian
- `section3` (Magenta) → H4 heading in Obsidian

### Semantic Colors (Content Colors)
These colors mark content meaning. Use **comment prefixes** to sub-categorize:
- `positive` (Green) → Positive points, evidence, answers
- `negative` (Red) → Criticisms, weaknesses, concerns
- `question` (Yellow) → Questions, gaps, uncertainties
- `detail` (Grey) → Context, definitions, methodology
- `code` (Orange) → Technical content, statistics, formulas

## Complete Color Reference

| Color | Hex | Category | Use For | Comment Prefixes |
|-------|-----|----------|---------|------------------|
| `section1` | #2ea8e5 (Blue) | Hierarchy | Document section headings (H2) | *(text becomes heading)* |
| `section2` | #a28ae5 (Purple) | Hierarchy | Subsection headings (H3) | *(text becomes heading)* |
| `section3` | #e56eee (Magenta) | Hierarchy | Sub-subsection headings (H4) | *(text becomes heading)* |
| `positive` | #5fb236 (Green) | Semantic | Thesis, premises, evidence, claims, answers, findings | `THESIS:`, `PREMISE:`, `EVIDENCE:`, `CLAIM:`, `A:`, `FINDING:`, `CORE:` |
| `negative` | #ff6666 (Red) | Semantic | Weaknesses, limitations, confusion, concerns | `WEAKNESS:`, `LIMITATION:`, `UNCLEAR:`, `CONCERN:` |
| `question` | #ffd400 (Yellow) | Semantic | Questions, gaps, unclear points, relevance flags | `Q:`, `GAP:`, `UNCLEAR:`, `RELEVANT:` |
| `detail` | #aaaaaa (Grey) | Semantic | Assumptions, definitions, connections, methodology, themes | `ASSUMPTION:`, `TERM:`, `CONNECTION:`, `METHOD:`, `THEME [x]:`, `DETAIL:` |
| `code` | #f19837 (Orange) | Semantic | Code, statistics, formulas, data | `STAT:`, `CODE:`, `DATA:` |

### Cross-Cutting Prefix: IDEA:

The `IDEA:` prefix is special - it can be **added before any other prefix** to mark annotations that contain atomic ideas suitable for Zettelkasten permanent notes.

**Usage:** Add `IDEA:` at the start of any comment (before other prefixes) when you encounter a standalone insight worth extracting as a permanent note.

| Format | Example Comment |
|--------|-----------------|
| `IDEA:` alone | `IDEA: Transfer learning trades data for compute` |
| `IDEA:` + `FINDING:` | `IDEA: FINDING: Transfer learning reduces data needs by 60%` |
| `IDEA:` + `CLAIM:` | `IDEA: CLAIM: Attention is all you need for sequence modeling` |
| `IDEA:` + `LIMITATION:` | `IDEA: LIMITATION: Sample size invalidates cross-domain claims` |
| `IDEA:` + `GAP:` | `IDEA: GAP: No studies examine long-term effects` |
| `IDEA:` + `METHOD:` | `IDEA: METHOD: Curriculum learning improves convergence` |
| `IDEA:` + `STAT:` | `IDEA: STAT: O(n log n) complexity enables real-time processing` |

**How commands parse IDEA: prefix:**
- `/readassist-permanent-note`: Looks for `IDEA:` at start, extracts the annotation as a permanent note
- `/readassist-summarize`: First extracts permanent notes from `IDEA:` annotations, then generates summary with links to them
- `/readassist-synthesize`: First extracts permanent notes from `IDEA:` annotations in all sources, then generates synthesis with links to them

**Zettelkasten Workflow:**
1. During `/readassist-read`, mark atomic ideas with `IDEA:` prefix (before any other prefix)
2. Run `/readassist-summarize <citekey>` or `/readassist-synthesize <citekeys...>`
3. Permanent notes are automatically extracted from `IDEA:`-prefixed annotations
4. Each idea becomes a standalone permanent note in `Permanent/` folder
5. The summary/synthesis includes links to the permanent notes

**What makes a good IDEA: annotation:**
- **Atomic**: Contains exactly one idea
- **Rewritten**: In your own words, not just a quote
- **Self-contained**: Understandable without the source context
- **Linkable**: Could connect to other ideas in your knowledge base

### Obsidian Integration

The Obsidian Zotero import template generates markdown headings from section colors:
```
Blue highlight    → ## Highlighted Text
Purple highlight  → ### Highlighted Text  
Magenta highlight → #### Highlighted Text
```

This means section colors should mark **actual document structure** (chapter titles, section names) rather than semantic content types.

**Note**: Different reading strategies may interpret semantic colors differently. See strategy-specific command files (e.g., `/readassist-read-sq3r`, `/readassist-read-review`) for their specific color mappings and comment prefixes.

## Fallback Behavior: No Outline Available

When `zotero_get_pdf_outline` returns `has_outline: false`:

1. **Inform the user**: "This PDF doesn't have bookmarks. I'll need page numbers to read specific sections."

2. **Offer options**:
   - "Would you like me to read from the beginning (page 1)?"
   - "Please provide specific page numbers (e.g., 'pages 1-10')"
   - "Would you like me to read the entire document?"

3. **If user provides page numbers**: Use `pages:` parameter
4. **If user wants to start from beginning**: Use `pages: "1-5"` or similar reasonable chunk

## Best Practices

1. **Check outline first**: Always call `zotero_get_pdf_outline` before reading unless pages are specified
2. **Detect figures automatically**: After reading each page, call `zotero_list_figures` to find visual content
3. **Extract important figures**: Use `zotero_get_figure` for figures with confidence ≥ 0.7 or that are referenced in text
4. **Analyze images with vision AI**: Read the extracted figure files and describe what you see
5. **Annotate figures with insights**: Create area annotations with detailed descriptions from image analysis
6. **Be precise with text**: The highlight text must match the PDF exactly (including spacing)
7. **Add meaningful comments**: Explain why each highlight is significant, incorporating visual evidence
8. **Use colors consistently**: Follow the semantic color scheme above
9. **Summarize findings**: After annotating, provide a summary combining text and visual insights
10. **Ask for clarification**: If page numbers or chapters are unclear, ask the user

## Error Handling

- **Section not found**: Check the exact section names from the outline
- **No outline available**: Ask user for page numbers instead
- **Text not found**: Try adjusting the text (check for hyphenation, line breaks)
- **Page out of range**: Verify the page count with the user
- **Item not found**: Confirm the citation key is correct
- **No figures detected**: This is normal for text-heavy pages; continue with text-only analysis
- **Figure extraction fails**: Note the issue but continue with text analysis

## Output

After completing the reading session, provide:

1. **Summary of key findings** (combining text and visual analysis)
2. **List of annotations created** (with colors and comments)
   - Text highlights with their significance
   - Figure annotations with visual analysis insights
3. **Key insights from figures** (what the charts/diagrams reveal)
4. **Questions or areas needing clarification**
5. **Suggested follow-up readings** (if applicable)

## Example Workflow: Reading with Images

```
User: /readassist-read smithML2023 chapters:"Results"

AI Process:
1. zotero_lookup("smithML2023") → Get attachment_key
2. zotero_get_pdf_outline(key) → Find "Results" section on pages 15-20
3. zotero_read_pdf_pages(key, section: "Results") → Extract text
4. For each page (15-20):
   a. zotero_list_figures(key, page: N) → Detect figures
   b. If important figures found:
      - zotero_get_figure(key, page: N, index: 0) → Extract image
      - Analyze image with vision AI
      - zotero_create_area_annotation with detailed description
5. Create text highlights for key claims
6. Provide summary combining text and visual analysis

Response to User:
"The Results section presents three key findings:

1. Algorithm performance (Figure 5): Bar chart shows Algorithm X achieves 
   94.5% accuracy vs 87.3% baseline, a 7.2 percentage point improvement.

2. Latency comparison (Figure 6): Line graph indicates Algorithm X reduces 
   average response time from 120ms to 45ms across all dataset sizes.

3. Scalability (Figure 7): Scatter plot demonstrates consistent performance 
   gains even with 10M+ records.

I've created 8 annotations: 5 text highlights on key claims and 3 figure 
annotations with detailed visual analysis from the charts."
```
