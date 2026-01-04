# /read - Critical Reading Workflow

Initiate an AI-assisted critical reading session for a Zotero item.

## Usage

```
/read <citekey> [pages:<range>] [chapters:<names>] [purpose:<text>]
```

## Arguments

- `citekey` (required): BetterBibTeX citation key (e.g., "smithMachineLearning2023")
- `pages:<range>` (optional): Page range to read (e.g., "1-10", "1,3,5", "all")
- `chapters:<names>` (optional): Chapter names to read (e.g., "Introduction,Methods")
- `purpose:<text>` (optional): Reading purpose to focus analysis

## Examples

```
/read smithML2023 pages:1-10 purpose:"understand methodology"
/read jonesDeepLearning2024 chapters:"Introduction,Results"
/read brownNLP2023
```

## Instructions for AI

When this command is invoked, follow this critical reading workflow:

### Step 1: Look Up the Item

Use the `zotero_lookup` tool to find the item by citation key:
```
zotero_lookup(citekey: "<citekey>")
```

This returns the item metadata and PDF attachment keys. Note the `attachment_key` for subsequent operations.

### Step 2: Read PDF Content

Use the `zotero_read_pdf_pages` tool to extract text:
```
zotero_read_pdf_pages(attachment_key: "<key>", pages: "<range>")
```

If chapters are specified instead of pages, you may need to:
1. First read "all" pages to identify chapter locations
2. Then read specific page ranges for each chapter

### Step 3: Analyze Content

Apply critical reading techniques based on the purpose:

- **Identify key claims and arguments**
- **Note supporting evidence**
- **Mark areas of agreement/disagreement**
- **Highlight technical terms and definitions**
- **Note figures and diagrams for reference**

### Step 4: Create Annotations

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

## Best Practices

1. **Be precise with text**: The highlight text must match the PDF exactly (including spacing)
2. **Add meaningful comments**: Explain why each highlight is significant
3. **Use colors consistently**: Follow the semantic color scheme above
4. **Summarize findings**: After annotating, provide a summary of key insights
5. **Ask for clarification**: If page numbers or chapters are unclear, ask the user

## Error Handling

- **Text not found**: Try adjusting the text (check for hyphenation, line breaks)
- **Page out of range**: Verify the page count with the user
- **Item not found**: Confirm the citation key is correct

## Output

After completing the reading session, provide:

1. **Summary of key findings**
2. **List of annotations created** (with colors and comments)
3. **Questions or areas needing clarification**
4. **Suggested follow-up readings** (if applicable)
