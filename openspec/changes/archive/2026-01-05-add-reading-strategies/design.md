# Design: Advanced Reading Strategies

## Context

Different reading purposes require different approaches. A student studying for an exam needs different support than a researcher conducting a literature review. By encoding proven reading methodologies as slash commands, we provide structured workflows that guide AI behavior.

### Reading Methodologies Supported

| Strategy | Purpose | Key Phases |
|----------|---------|------------|
| SQ3R | Textbook learning | Survey → Question → Read → Recite → Review |
| Literature Review | Evidence synthesis | Extract → Categorize → Assess → Synthesize |
| Analytical | Deep comprehension | Structure → Interpret → Critique → Integrate |
| Skim | Quick overview | Title → Abstract → Headings → Conclusions |

## Goals / Non-Goals

### Goals
- Provide structured reading workflows via slash commands
- Guide AI through methodology-specific phases
- Produce appropriate outputs for each strategy
- Use semantic colors consistently within strategies

### Non-Goals
- Enforce rigid methodology compliance
- Replace user judgment on reading approach
- Create new annotation types (use existing)
- Implement spaced repetition or learning tracking

## Strategy Definitions

### SQ3R Strategy (`/read-sq3r`)

**Purpose**: Deep comprehension of textbook/educational material

**Phases**:
1. **Survey** (2 min)
   - Read title, headings, subheadings
   - Note figures and their captions
   - Read summary/conclusion
   - Highlight structure with Section colors (Blue/Purple/Magenta)

2. **Question** (2 min)
   - Convert headings to questions
   - Add questions as annotation comments
   - Use Yellow (`question`) for question annotations

3. **Read** (variable)
   - Read actively, seeking answers to questions
   - Highlight key answers with Green (positive)
   - Mark confusing parts with Red (negative)

4. **Recite** (per section)
   - Summarize section in own words (annotation comment)
   - Link back to questions

5. **Review** (end)
   - Create summary note in Zotero
   - List unanswered questions

**Color Mapping**:
- Blue/Purple/Magenta: Structure (Survey phase)
- Yellow (`question`): Questions generated during Question phase
- Green (`positive`): Answers/Key points found during Read phase
- Red (`negative`): Confusion/Need review
- Grey (`detail`): Supporting details and examples
- Orange (`code`): Technical content/Code

### Literature Review Strategy (`/read-review`)

**Purpose**: Systematic evidence extraction for research synthesis

**Phases**:
1. **Metadata Extraction**
   - Record: authors, year, journal, methodology
   - Identify study type (empirical, theoretical, review)

2. **Evidence Extraction**
   - Highlight claims with Green (supported)
   - Highlight limitations with Red (critique)
   - Highlight methodology details with Grey
   - Mark sample sizes, statistics with Orange

3. **Quality Assessment**
   - Note study quality indicators
   - Flag potential biases

4. **Categorization**
   - Tag by theme using annotation comments
   - Use Section colors for thematic grouping

**Output**: Structured extraction suitable for synthesis matrix

### Analytical Reading Strategy (`/read-analyze`)

**Purpose**: Deep critical analysis of arguments

**Phases**:
1. **Structural Analysis**
   - Identify thesis/main argument
   - Map argument structure
   - Highlight premises (Blue)
   - Highlight conclusions (Purple)

2. **Interpretation**
   - Identify key terms and definitions
   - Note assumptions (Grey)
   - Mark evidence (Green)

3. **Critique**
   - Evaluate argument validity
   - Note logical gaps (Red)
   - Identify counter-arguments

4. **Integration**
   - Connect to other works
   - Note implications

**Color Mapping**:
- Blue (`section1`): Premises/Claims
- Purple (`section2`): Conclusions/Thesis
- Green (`positive`): Supporting evidence
- Red (`negative`): Weaknesses/Gaps
- Grey (`detail`): Assumptions/Definitions
- Orange (`code`): Data/Statistics
- Yellow (`question`): Unclear arguments/Questions

### Skim Strategy (`/read-skim`)

**Purpose**: Quick overview for relevance assessment

**Phases**:
1. **Title & Abstract** - Read and summarize
2. **Headings** - Note structure
3. **Figures** - Review key visuals
4. **Conclusion** - Extract main findings

**Time**: 5-10 minutes maximum
**Output**: Brief relevance assessment + key takeaways

## Slash Command Structure

### Base `/read` with Strategy Parameter

```
/read <citekey> [pages:X-Y] [strategy:sq3r|review|analyze|skim|critical]
```

Default strategy: `critical` (Phase 1 behavior)

### Strategy-Specific Commands

```
/read-sq3r <citekey> [pages:X-Y] [focus:"topic"]
/read-review <citekey> [extraction:"claims,methods,findings"]
/read-analyze <citekey> [depth:surface|moderate|deep]
/read-skim <citekey> [time:5m|10m]
```

## Decisions

### Decision 1: Strategy as Parameter and Separate Commands
**What**: Support both `/read strategy:sq3r` and `/read-sq3r`.

**Why**:
- Separate commands are more discoverable
- Parameter allows quick switching
- Consistent with other CLI patterns

### Decision 2: Phase-Based Instructions
**What**: Structure AI instructions by methodology phase.

**Why**:
- Matches how humans learn these methods
- Provides clear checkpoints
- Enables partial completion

### Decision 3: Consistent Color Semantics Within Strategy
**What**: Each strategy documents its specific color meanings.

**Why**:
- Colors may mean different things in different contexts
- Clear documentation prevents confusion
- Maintains flexibility

## Risks / Trade-offs

### Risk: AI May Not Follow Phases Strictly
**Mitigation**: Provide explicit phase transitions in prompts.

### Risk: Overwhelming Users with Options
**Mitigation**: Default to simple critical reading; strategies are opt-in.

### Trade-off: Detailed vs Flexible Instructions
**Choice**: Detailed instructions with noted flexibility points.
**Trade-off**: May feel prescriptive.
**Mitigation**: Document that strategies are guidelines, not rules.
