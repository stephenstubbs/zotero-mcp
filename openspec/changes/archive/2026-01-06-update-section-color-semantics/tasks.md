# Tasks: Update Section Color Semantics

## 1. Update Base `/read.md` Color Documentation

- [x] 1.1 Update Semantic Color Scheme table
  - [x] 1.1.1 Change `section1` description to "Document section headings (H2)"
  - [x] 1.1.2 Change `section2` description to "Subsection headings (H3)"
  - [x] 1.1.3 Change `section3` description to "Sub-subsection headings (H4)"
  - [x] 1.1.4 Update other colors with comment prefix examples
- [x] 1.2 Add "Color Philosophy" section explaining:
  - [x] 1.2.1 Section colors = document hierarchy (Obsidian H2/H3/H4)
  - [x] 1.2.2 Semantic colors = content meaning
  - [x] 1.2.3 Comment prefixes for sub-categorization
- [x] 1.3 Add complete color reference table with all comment prefixes

## 2. Update `/read-sq3r.md` Color Documentation

- [x] 2.1 Verify color scheme table is correct (section colors for Survey structure)
- [x] 2.2 Add complete mapping table showing:
  - [x] 2.2.1 Color → Semantic meaning → Comment prefix → Example
- [x] 2.3 Document all comment prefixes used:
  - `Q:` for questions
  - `A:` for answers
  - `UNCLEAR:` for confusion
  - `DETAIL:` for supporting info

## 3. Update `/read-review.md` Color Documentation

- [x] 3.1 Update color scheme table
  - [x] 3.1.1 Remove section1/2/3 from "Theme Category" usage
  - [x] 3.1.2 Move themes to `detail` (Grey) with `THEME [x]:` prefix
  - [x] 3.1.3 Reserve section colors for document structure only
- [x] 3.2 Add complete mapping table showing:
  - [x] 3.2.1 Color → Semantic meaning → Comment prefix → Example
- [x] 3.3 Document all comment prefixes used:
  - `CLAIM:` for supported claims
  - `LIMITATION:` for weaknesses
  - `GAP:` for research gaps
  - `THEME [x]:` for thematic tagging
  - `METHOD:` for methodology
  - `STAT:` for statistics
- [x] 3.4 Update "Color Usage by Extraction Phase" section

## 4. Update `/read-analyze.md` Color Documentation

- [x] 4.1 Update color scheme table (major changes)
  - [x] 4.1.1 Move "Premises" from section1 → `positive` (Green) with `PREMISE:` prefix
  - [x] 4.1.2 Move "Conclusions" from section2 → `positive` (Green) with `THESIS:`/`CONCLUSION:` prefix
  - [x] 4.1.3 Move "Connections" from section3 → `detail` (Grey) with `CONNECTION:` prefix
  - [x] 4.1.4 Document section1/2/3 for document structure only
- [x] 4.2 Add complete mapping table showing:
  - [x] 4.2.1 Color → Semantic meaning → Comment prefix → Example
- [x] 4.3 Document all comment prefixes used:
  - `PREMISE [Pn]:` for premises (numbered)
  - `THESIS:` for main thesis
  - `CONCLUSION [Cn]:` for conclusions (numbered)
  - `EVIDENCE:` for supporting evidence
  - `WEAKNESS:` for logical gaps
  - `ASSUMPTION:` for unstated assumptions
  - `TERM:` for definitions
  - `CONNECTION:` for links to other works
  - `UNCLEAR:` for ambiguous arguments
- [x] 4.4 Update "Color Usage by Phase" section
- [x] 4.5 Update Phase 1-4 instructions to use new colors/prefixes

## 5. Update `/read-skim.md` Color Documentation

- [x] 5.1 Update color scheme table
  - [x] 5.1.1 Change section1 from "Main Topic" → use Green/Grey for core contribution
  - [x] 5.1.2 Focus on 4 content colors: green, red, yellow, grey
- [x] 5.2 Add complete mapping table showing:
  - [x] 5.2.1 Color → Semantic meaning → Comment prefix → Example
- [x] 5.3 Document all comment prefixes used:
  - `CORE:` for main contribution
  - `FINDING:` for key results
  - `CONCERN:` for limitations
  - `RELEVANT:` for focus-related points
- [x] 5.4 Update "Colors NOT Used" section

## 6. Add Master Color Reference

- [x] 6.1 Create or update a central color reference section in `/read.md` with:
  - [x] 6.1.1 Complete 8-color table with hex codes
  - [x] 6.1.2 Hierarchy colors vs semantic colors distinction
  - [x] 6.1.3 All standard comment prefixes across strategies
  - [x] 6.1.4 Obsidian template behavior explanation

## 7. Validation

- [x] 7.1 Verify all command files have consistent color documentation
- [x] 7.2 Verify all comment prefixes are documented
- [x] 7.3 Ensure no strategy uses section1/2/3 for non-hierarchical purposes
- [x] 7.4 Cross-check coverage table: every concept has a color + prefix

## Dependencies

- Tasks 2-5 can run in parallel after Task 1
- Task 6 depends on Tasks 2-5 (needs all prefixes finalized)
- Task 7 depends on all previous tasks

## Key Principle

**Section colors (Blue/Purple/Magenta) = Document hierarchy (H2/H3/H4 in Obsidian)**
**Semantic colors (Green/Red/Yellow/Grey/Orange) = Content meaning**
**Comment prefixes = Sub-categorization within colors**

## Complete Color → Concept Mapping

| Color | Name | Concepts | Comment Prefixes |
|-------|------|----------|------------------|
| Blue | `section1` | H2 headings | (none - text is heading) |
| Purple | `section2` | H3 headings | (none - text is heading) |
| Magenta | `section3` | H4 headings | (none - text is heading) |
| Green | `positive` | Premises, Thesis, Conclusions, Evidence, Claims, Answers, Findings | `PREMISE:`, `THESIS:`, `CONCLUSION:`, `EVIDENCE:`, `CLAIM:`, `A:`, `FINDING:`, `CORE:` |
| Red | `negative` | Weaknesses, Limitations, Confusion, Concerns | `WEAKNESS:`, `LIMITATION:`, `UNCLEAR:`, `CONCERN:` |
| Yellow | `question` | Questions, Gaps, Unclear, Relevance | `Q:`, `GAP:`, `UNCLEAR:`, `RELEVANT:` |
| Grey | `detail` | Assumptions, Definitions, Connections, Methodology, Themes, Details | `ASSUMPTION:`, `TERM:`, `CONNECTION:`, `METHOD:`, `THEME [x]:`, `DETAIL:` |
| Orange | `code` | Data, Statistics, Formulas, Code | `STAT:`, `CODE:`, `DATA:` |
