# Change: Add Advanced Reading Strategies

## Why

Phase 1 provides basic critical reading with semantic highlighting, but researchers use various structured reading methodologies:

- **SQ3R** (Survey, Question, Read, Recite, Review) - textbook comprehension
- **PQRST** (Preview, Question, Read, Summarize, Test) - exam preparation
- **Analytical Reading** (Adler's levels) - deep comprehension
- **Literature Review** - systematic evidence gathering

Each strategy requires different annotation patterns, reading sequences, and output formats. By encoding these strategies in slash commands, users can invoke purpose-specific workflows.

## What Changes

1. **Strategy-Specific Slash Commands** - New commands for each methodology
   - `/read-sq3r` - SQ3R methodology with structured phases
   - `/read-review` - Literature review with evidence extraction
   - `/read-analyze` - Deep analytical reading with argument mapping
   - `/read-skim` - Quick overview with key points only

2. **Strategy Configuration** - Structured prompts for each strategy
   - Phase-specific instructions for AI
   - Appropriate color usage per strategy
   - Output format guidance (notes, summaries, questions)

3. **Enhanced `/read` Command** - Strategy selection parameter
   - `strategy:sq3r` parameter on base `/read` command
   - Default to "critical" strategy (Phase 1 behavior)

## Impact

- **New specs**: `reading-strategies`
- **Modified specs**: `slash-command` (strategy parameter)
- **New files**:
  - `.opencode/commands/read-sq3r.md`
  - `.opencode/commands/read-review.md`
  - `.opencode/commands/read-analyze.md`
  - `.opencode/commands/read-skim.md`

## Dependencies

- **Requires**: Phase 1 (`add-critical-reading-workflow`) must be complete
- **Optional**: Phase 2 (`add-pdf-image-extraction`) enhances figure analysis

## Open Questions

1. Should strategies be separate commands or parameters to `/read`?
   - **Recommendation**: Both - separate commands for discoverability, parameter for flexibility

2. How detailed should strategy instructions be?
   - **Recommendation**: Detailed enough for AI to execute phases autonomously

3. Should we support custom user-defined strategies?
   - **Recommendation**: Not in Phase 3, consider for future enhancement
