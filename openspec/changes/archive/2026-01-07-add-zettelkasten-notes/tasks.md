# Tasks

## Phase 1: Update Existing Commands

- [x] Update `/summarize` command with complete annotation template documentation
- [x] Update `/summarize` to use pandoc citation format `[@citekey p. X]` instead of wikilinks
- [x] Update `/synthesize` command with complete annotation template documentation
- [x] Update `/synthesize` to use pandoc citation format `[@citekey p. X]` instead of wikilinks
- [x] Verify color scheme documentation is consistent across both commands
- [x] Update `/read` command to document `IDEA:` prefix for Zettelkasten workflow

## Phase 2: Implement Zettel Command

- [x] Create `.opencode/command/zettel.md` slash command
- [x] Document `IDEA:` prefix convention in command file
- [x] Document permanent note output format with pandoc citations
- [x] Document `Permanent/` folder output location

## Phase 3: Validation

- [ ] Test `/summarize` with real annotation file (verify pandoc citations)
- [ ] Test `/synthesize` with multiple annotation files (verify pandoc citations)
- [ ] Test `/zettel` with `IDEA:` prefixed annotations
- [ ] Verify all three commands parse the same annotation file correctly
