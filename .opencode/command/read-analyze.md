---
description: Analytical reading strategy for deep argument analysis and critical evaluation.
---
Execute the /read-analyze command with the following arguments:
<UserRequest>
  $ARGUMENTS
</UserRequest>

# /read-analyze - Analytical Reading Strategy

## Overview

Analytical Reading (based on Mortimer Adler's levels of reading) is designed for deep critical analysis of argumentative texts. It focuses on understanding the logical structure of arguments, evaluating validity, and synthesizing insights. Ideal for philosophy papers, theoretical articles, position papers, and any text making substantive claims.

## Usage

```
/read-analyze <citekey> [pages:<range>] [chapters:<names>] [depth:<level>]
```

## Arguments

- `citekey` (required): BetterBibTeX citation key (e.g., "smithMachineLearning2023")
- `pages:<range>` (optional): Page range to read (e.g., "1-10")
- `chapters:<names>` (optional): Chapter/section names (e.g., "Introduction,Discussion")
- `depth:<level>` (optional): Analysis depth - `surface`, `moderate`, or `deep` (default: moderate)

## Depth Levels

| Level | Time | Passes | Focus |
|-------|------|--------|-------|
| `surface` | 15-20 min | 1 | Main thesis and key arguments only |
| `moderate` | 30-45 min | 2 | Full argument structure + basic critique |
| `deep` | 60+ min | 3+ | Comprehensive analysis, all assumptions, counterarguments |

## Examples

```
/read-analyze smithPhilosophy2023 depth:deep
/read-analyze jonesTheory2024 chapters:"Argument,Discussion" depth:moderate
/read-analyze brownPosition2023 pages:1-20 depth:surface
```

## Color Scheme for Analytical Reading

Analytical Reading uses colors to map the logical structure of arguments.

### Hierarchy Colors (Generate Obsidian Headings)

| Color | Hex | Analysis Usage | Obsidian Result |
|-------|-----|----------------|-----------------|
| `section1` (Blue) | #2ea8e5 | Document section headings only | `## Heading` (H2) |
| `section2` (Purple) | #a28ae5 | Subsection headings only | `### Heading` (H3) |
| `section3` (Magenta) | #e56eee | Sub-subsection headings only | `#### Heading` (H4) |

### Semantic Colors (Content Meaning)

| Color | Hex | Analysis Usage | Comment Prefix | Example |
|-------|-----|----------------|----------------|---------|
| `positive` (Green) | #5fb236 | Premises, thesis, conclusions, evidence | `PREMISE:`, `THESIS:`, `CONCLUSION:`, `EVIDENCE:` | "PREMISE [P1]: Studies show X leads to Y" |
| `negative` (Red) | #ff6666 | Weaknesses, logical gaps, fallacies | `WEAKNESS:` | "WEAKNESS: Unsupported claim" |
| `question` (Yellow) | #ffd400 | Unclear arguments, ambiguity | `UNCLEAR:` | "UNCLEAR: What does 'significant' mean?" |
| `detail` (Grey) | #aaaaaa | Assumptions, definitions, connections | `ASSUMPTION:`, `TERM:`, `CONNECTION:` | "ASSUMPTION: Author assumes utilitarianism" |
| `code` (Orange) | #f19837 | Quantitative support, formal logic | `DATA:`, `FORMULA:` | "DATA: P(A|B) > P(A)" |

### Comment Prefixes for Analytical Reading

| Prefix | Color | Usage |
|--------|-------|-------|
| `THESIS:` | Green | Main thesis statement |
| `PREMISE [Pn]:` | Green | Numbered premises (P1, P2, P3...) |
| `CONCLUSION [Cn]:` | Green | Numbered conclusions (C1, C2...) |
| `EVIDENCE:` | Green | Supporting evidence for premises/conclusions |
| `WEAKNESS:` | Red | Logical gaps, unsupported claims, fallacies |
| `UNCLEAR:` | Yellow | Ambiguous arguments needing clarification |
| `ASSUMPTION:` | Grey | Unstated assumptions author relies on |
| `TERM:` | Grey | Key term definitions |
| `CONNECTION:` | Grey | Links to other works, implications |
| `DATA:` | Orange | Quantitative support |
| `FORMULA:` | Orange | Formal logic expressions |

### Color Usage by Phase

- **Structural Analysis**: Green (`THESIS:`, `PREMISE [Pn]:`, `CONCLUSION [Cn]:`)
- **Interpretation**: Grey (`ASSUMPTION:`, `TERM:`), Green (`EVIDENCE:`)
- **Critique**: Red (`WEAKNESS:`), Yellow (`UNCLEAR:`)
- **Integration**: Grey (`CONNECTION:`), Orange (`DATA:`, `FORMULA:`)
- **Document Structure**: Blue/Purple/Magenta (headings only)

## Instructions for AI

When this command is invoked, execute Analytical Reading through four phases. The depth parameter determines how thorough each phase should be.

### Phase 1: Structural Analysis

**Goal**: Map the logical skeleton of the argument.

1. **Look up the item**:
   ```
   zotero_lookup(citekey: "<citekey>")
   ```

2. **Get document structure**:
   ```
   zotero_get_pdf_outline(attachment_key: "<key>")
   ```

3. **First pass - Identify the thesis**:
   ```
   zotero_read_pdf_pages(attachment_key: "<key>", section: "Abstract")
   zotero_read_pdf_pages(attachment_key: "<key>", section: "Introduction")
   zotero_read_pdf_pages(attachment_key: "<key>", section: "Conclusion")
   ```

4. **Highlight the main thesis** (Green with `THESIS:` prefix):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<thesis statement>",
     page: <page>,
     color: "positive",
     comment: "THESIS: <restated in your words>"
   )
   ```

5. **Map argument structure**:
   - Read through the full text
   - Identify each major claim/premise
   
6. **Highlight premises** (Green with `PREMISE [Pn]:` prefix):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<premise text>",
     page: <page>,
     color: "positive",
     comment: "PREMISE [P1]: <restated> | Supports: <which conclusion>"
   )
   ```
   
   Number premises (P1, P2, P3...) to track the argument chain.

7. **Highlight sub-conclusions** (Green with `CONCLUSION [Cn]:` prefix):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<conclusion text>",
     page: <page>,
     color: "positive",
     comment: "CONCLUSION [C1]: <restated> | Derived from: P1, P3"
   )
   ```

**Structural Output**:
```
Argument Map:
- Thesis: [Main claim]
- P1 → C1 (intermediate)
- P2 + P3 → C2 (intermediate)
- C1 + C2 → Thesis (main)
```

### Phase 2: Interpretation

**Goal**: Understand the meaning and context of the argument.

1. **Identify key terms**:
   - Find technical or specialized terms
   - Look for explicit definitions
   - Note how terms are used in context

2. **Highlight definitions** (Grey):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<definition or key term usage>",
     page: <page>,
     color: "detail",
     comment: "TERM: <term> = <definition/meaning in context>"
   )
   ```

3. **Identify assumptions** (Grey):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<text revealing assumption>",
     page: <page>,
     color: "detail",
     comment: "ASSUMPTION: <unstated belief author relies on>"
   )
   ```

4. **Highlight evidence** (Green):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<evidence text>",
     page: <page>,
     color: "positive",
     comment: "EVIDENCE for [P#/C#]: <type> - <strength assessment>"
   )
   ```

   Evidence types:
   - Empirical: data, studies, observations
   - Logical: deductive reasoning, proofs
   - Authoritative: expert citations
   - Analogical: comparisons, examples

5. **Analyze figures** (if depth is moderate or deep):
   ```
   zotero_list_figures(attachment_key: "<key>", page: <page>)
   zotero_get_figure(attachment_key: "<key>", page: <page>, figure_index: <idx>)
   ```
   
   ```
   zotero_create_area_annotation(
     attachment_key: "<key>",
     page: <page>,
     rect: [x1, y1, x2, y2],
     color: "positive",
     comment: "DIAGRAM: <what it shows> | Supports: [P#/C#]"
   )
   ```

### Phase 3: Critique

**Goal**: Evaluate the validity and soundness of the argument.

1. **Check logical validity**:
   - Do conclusions follow from premises?
   - Are there logical fallacies?
   - Are inferences warranted?

2. **Highlight weaknesses** (Red):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<problematic text>",
     page: <page>,
     color: "negative",
     comment: "WEAKNESS: <type> - <explanation>"
   )
   ```

   Weakness types:
   - `LOGICAL GAP`: Conclusion doesn't follow from premises
   - `UNSUPPORTED`: Claim lacks evidence
   - `FALLACY`: Specific logical fallacy (name it)
   - `CONTRADICTION`: Inconsistent with other claims
   - `OVERGENERALIZATION`: Extends beyond evidence
   - `FALSE DICHOTOMY`: Presents false either/or

3. **Mark unclear arguments** (Yellow):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<ambiguous text>",
     page: <page>,
     color: "question",
     comment: "UNCLEAR: <what's ambiguous> | Possible interpretations: <list>"
   )
   ```

4. **Identify counterarguments** (for deep analysis):
   - What would opponents say?
   - Does the author address counterarguments?
   - Are rebuttals effective?

5. **Assess evidence quality**:
   - Is evidence relevant to the claim?
   - Is evidence sufficient?
   - Is evidence current/reliable?

**Critique Output** (for moderate/deep):
```
Argument Evaluation:
- Logical Structure: [Valid/Invalid + explanation]
- Evidence Quality: [Strong/Moderate/Weak]
- Key Weaknesses: [List with P#/C# references]
- Fallacies Found: [List if any]
```

### Phase 4: Integration

**Goal**: Connect to broader context and draw implications.

1. **Note connections to other works** (Grey with `CONNECTION:` prefix):
   ```
   zotero_create_highlight(
     attachment_key: "<key>",
     text: "<text referencing or relating to other work>",
     page: <page>,
     color: "detail",
     comment: "CONNECTION: <relationship to other work/idea>"
   )
   ```

   Connection types:
   - `AGREES WITH`: Supports another author's position
   - `CONTRADICTS`: Opposes another view
   - `EXTENDS`: Builds on previous work
   - `APPLIES`: Uses theory from another domain

2. **Identify implications**:
   - What follows if the argument is correct?
   - What are practical consequences?
   - What questions remain?

3. **Formulate your position** (for deep analysis):
   - Do you accept the thesis? Why/why not?
   - Which premises are most/least convincing?
   - How would you strengthen the argument?

## Output: Analytical Summary

After completing the analysis, provide a structured evaluation:

```
## Analytical Reading: [Title] ([Author], [Year])

### Argument Structure

**Main Thesis**: [Restated thesis]

**Argument Map**:
```
P1: [Premise 1]
P2: [Premise 2]
    ↓
C1: [Intermediate conclusion 1]

P3: [Premise 3]
P4: [Premise 4]
    ↓
C2: [Intermediate conclusion 2]

C1 + C2
    ↓
THESIS: [Main conclusion]
```

### Key Terms & Definitions
- **[Term 1]**: [Definition/usage]
- **[Term 2]**: [Definition/usage]

### Core Assumptions
1. [Assumption 1] - Stated/Unstated
2. [Assumption 2] - Stated/Unstated

### Evidence Assessment
| Claim | Evidence Type | Strength | Notes |
|-------|---------------|----------|-------|
| P1 | Empirical | Strong | Large study cited |
| P2 | Authoritative | Moderate | Expert opinion |
| C1 | Logical | Strong | Valid inference |

### Critical Evaluation

**Logical Validity**: [Valid/Invalid]
- [Explanation of logical structure]

**Soundness**: [Sound/Unsound]
- [Assessment of premise truth]

**Identified Weaknesses**:
1. [Weakness 1] (affects [P#/C#])
2. [Weakness 2] (affects [P#/C#])

**Possible Fallacies**:
- [Fallacy name]: [Location and explanation]

**Counterarguments Not Addressed**:
- [Counterargument 1]
- [Counterargument 2]

### Connections & Implications

**Relates to**:
- [Other work 1]: [Relationship]
- [Other work 2]: [Relationship]

**Implications if Correct**:
- [Implication 1]
- [Implication 2]

### Overall Assessment

**Argument Strength**: [Strong/Moderate/Weak]
**Recommendation**: [Accept/Reject/Accept with reservations]
**Key Insight**: [Most valuable takeaway]

### Annotations Created
- X thesis/premises/conclusions highlighted (green)
- Y evidence marked (green)
- Z weaknesses flagged (red)
- N assumptions/definitions/connections noted (grey)
- M unclear points (yellow)
- K data/formulas (orange)
- J structure headings (blue/purple/magenta)

### Questions for Further Investigation
1. [Question 1]
2. [Question 2]
```

## Best Practices for Analytical Reading

1. **Read charitably first**: Understand before critiquing
2. **Number your premises**: Makes tracking argument flow easier
3. **Distinguish claims from evidence**: Don't confuse what's asserted with what supports it
4. **Name fallacies precisely**: Use standard fallacy terminology
5. **Check your own biases**: Note when you agree/disagree before evaluating
6. **Consider the strongest version**: Critique the best interpretation of the argument
7. **Separate validity from soundness**: A valid argument can have false premises

## Error Handling

- **No clear thesis**: Note this as a structural weakness; look for implicit thesis
- **Multiple arguments**: Analyze each separately, then synthesize
- **Highly technical content**: Focus on logical structure even if domain expertise is limited
- **Emotional appeals**: Note when pathos substitutes for logos
