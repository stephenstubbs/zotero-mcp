# /read Command - Image Analysis Integration

## Summary

Enhanced the `/read` slash command to automatically detect and analyze figures, charts, and diagrams using vision AI during critical reading sessions.

## What Changed

### New Step 4: Detect and Extract Figures

After reading text content, the `/read` command now:

1. **Automatically detects figures** on each page using `zotero_list_figures`
2. **Extracts important figures** as image files using `zotero_get_figure`
3. **Analyzes images with vision AI** to understand charts, diagrams, and visual content

### Enhanced Step 5: Analyze Content (Text + Images)

Reading analysis now combines:
- Text analysis (key claims, arguments, evidence)
- Visual analysis (data from charts, diagram insights, visual patterns)
- Cross-referencing between text and figures

### Enhanced Step 6: Create Annotations

Area annotations now include:
- **Detailed visual analysis** from AI image analysis
- **Data extracted from charts** (values, trends, comparisons)
- **Descriptions of diagrams** (components, relationships, flows)

## When Figures Are Extracted

The command extracts figures when:
1. Text explicitly references them (e.g., "see Figure 3")
2. High confidence detections (≥ 0.7)
3. Large figures likely containing important info (>200 points)
4. Reading purpose suggests visual analysis (e.g., "analyze results")

## Example Workflow

```
User: /read smithML2023 chapters:"Results"

Old behavior (text-only):
- Read text from Results section
- Highlight key sentences
- Create area boxes around figures (blind)
- Summarize text content only

New behavior (text + images):
- Read text from Results section
- Detect figures on each page
- Extract Figure 5 (bar chart) as image
- Analyze chart: "Shows Algorithm X: 94.5%, Baseline: 87.3%"
- Extract Figure 6 (line graph) as image  
- Analyze graph: "Latency reduced from 120ms to 45ms"
- Create area annotations WITH visual insights
- Summarize combining text + visual data
```

## New Best Practices

1. **Detect figures automatically** after reading each page
2. **Extract important figures** (confidence ≥ 0.7 or referenced in text)
3. **Analyze images with vision AI** before annotating
4. **Annotate figures with insights** from actual image analysis
5. **Summarize combining text and visual evidence**

## Tools Used

- `zotero_list_figures` - Detect figure regions on a page
- `zotero_get_figure` - Extract specific figure as image file
- Vision AI - Analyze extracted images
- `zotero_create_area_annotation` - Annotate with visual insights

## Benefits

✅ **Richer analysis** - Understanding charts, not just mentioning them  
✅ **Data extraction** - Get actual values from graphs and tables  
✅ **Visual insights** - Discover patterns not stated in text  
✅ **Better annotations** - Area comments describe what's actually in the figure  
✅ **Complete understanding** - Text + visual content together  

## Impact on Reading Sessions

**Before:** "The paper shows improved performance (Figure 3)."  
**After:** "The bar chart in Figure 3 shows Algorithm A achieves 94.5% accuracy compared to 87.3% for the baseline, a 7.2 percentage point improvement. The error bars indicate this difference is statistically significant (p < 0.01)."

The `/read` command now provides comprehensive analysis of both textual and visual content in academic papers.
