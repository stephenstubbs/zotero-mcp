---
description: Initiate an AI-assisted critical reading session for a Zotero item.
---
Execute the /read command with the following arguments:
<UserRequest>
  $ARGUMENTS
</UserRequest>

# /read - Critical Reading Workflow

## Usage

```
/read <citekey> [pages:<range>] [chapters:<names>] [from_page:<N>] [purpose:<text>]
```

## Arguments

- `citekey` (required): BetterBibTeX citation key (e.g., "smithMachineLearning2023")
- `pages:<range>` (optional): Page range to read (e.g., "1-10", "1,3,5", "all")
- `chapters:<names>` (optional): Chapter/section names to read (e.g., "Introduction,Methods")
- `from_page:<N>` (optional): Start reading from page N (skips outline lookup)
- `purpose:<text>` (optional): Reading purpose to focus analysis

## Examples

```
/read smithML2023 pages:1-10 purpose:"understand methodology"
/read jonesDeepLearning2024 chapters:"Introduction,Results"
/read brownNLP2023
/read brownNLP2023 from_page:1
```

## Instructions for AI

When this command is invoked, parse the `<UserRequest>` block above to extract:
1. `citekey` - the first argument (required)
2. `pages:` - page range if provided
3. `chapters:` - section names if provided  
4. `from_page:` - starting page if provided
5. `purpose:` - reading purpose if provided

Then follow this critical reading workflow:

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

### Step 4: Analyze Content

Apply critical reading techniques based on the purpose:

- **Identify key claims and arguments**
- **Note supporting evidence**
- **Mark areas of agreement/disagreement**
- **Highlight technical terms and definitions**
- **Note figures and diagrams for reference**

### Step 5: Create Annotations

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

#### Area Annotations (`zotero_create_area_annotation`)

For figures, diagrams, or regions:
```
zotero_create_area_annotation(
  attachment_key: "<key>",
  page: <1-based page number>,
  rect: [x1, y1, x2, y2],
  color: "<semantic color>",
  comment: "<description>"
)
```

## Semantic Color Scheme

Use these colors consistently for meaning:

| Color | Hex | Use For |
|-------|-----|---------|
| `section1` | #2ea8e5 (Blue) | Main thesis / Primary arguments |
| `section2` | #a28ae5 (Purple) | Supporting arguments |
| `section3` | #e56eee (Magenta) | Methodology / Framework |
| `positive` | #5fb236 (Green) | Points of agreement / Strong evidence |
| `detail` | #aaaaaa (Grey) | Background info / Context / Definitions |
| `negative` | #ff6666 (Red) | Critique / Weak points / Disagreement |
| `code` | #f19837 (Orange) | Code snippets / Technical formulas |

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
2. **Be precise with text**: The highlight text must match the PDF exactly (including spacing)
3. **Add meaningful comments**: Explain why each highlight is significant
4. **Use colors consistently**: Follow the semantic color scheme above
5. **Summarize findings**: After annotating, provide a summary of key insights
6. **Ask for clarification**: If page numbers or chapters are unclear, ask the user

## Error Handling

- **Section not found**: Check the exact section names from the outline
- **No outline available**: Ask user for page numbers instead
- **Text not found**: Try adjusting the text (check for hyphenation, line breaks)
- **Page out of range**: Verify the page count with the user
- **Item not found**: Confirm the citation key is correct

## Output

After completing the reading session, provide:

1. **Summary of key findings**
2. **List of annotations created** (with colors and comments)
3. **Questions or areas needing clarification**
4. **Suggested follow-up readings** (if applicable)
