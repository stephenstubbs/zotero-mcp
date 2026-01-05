# Tasks: Add Advanced Reading Strategies

## Prerequisites
- [x] 0.1 Phase 1 (`add-critical-reading-workflow`) is complete and merged

## 1. SQ3R Strategy Command

- [x] 1.1 Create `.opencode/command/read-sq3r.md`
- [x] 1.2 Write Survey phase instructions with structure highlighting
- [x] 1.3 Write Question phase instructions with question generation
- [x] 1.4 Write Read phase instructions with answer highlighting
- [x] 1.5 Write Recite phase instructions with summarization
- [x] 1.6 Write Review phase instructions with note creation
- [x] 1.7 Add "Color Scheme for SQ3R" section to command file
  - [x] 1.7.1 Document: yellow for questions, green for answers, red for confusion
  - [x] 1.7.2 Document: blue/purple for structure during Survey phase
  - [x] 1.7.3 Provide example annotations for each color
- [ ] 1.8 Test end-to-end with sample textbook chapter

## 2. Literature Review Strategy Command

- [x] 2.1 Create `.opencode/command/read-review.md`
- [x] 2.2 Write metadata extraction instructions
- [x] 2.3 Write evidence extraction instructions with claim/limitation highlighting
- [x] 2.4 Write quality assessment instructions
- [x] 2.5 Write categorization instructions with thematic tagging
- [x] 2.6 Add "Color Scheme for Literature Review" section
  - [x] 2.6.1 Document: green for supported claims, red for limitations
  - [x] 2.6.2 Document: yellow for gaps in literature
  - [x] 2.6.3 Provide example annotations for each color
- [x] 2.7 Document output format for synthesis matrix
- [ ] 2.8 Test with sample research paper

## 3. Analytical Reading Strategy Command

- [x] 3.1 Create `.opencode/command/read-analyze.md`
- [x] 3.2 Write structural analysis instructions (thesis, arguments)
- [x] 3.3 Write interpretation instructions (terms, assumptions)
- [x] 3.4 Write critique instructions (validity, gaps)
- [x] 3.5 Write integration instructions (connections, implications)
- [x] 3.6 Add "Color Scheme for Analytical Reading" section
  - [x] 3.6.1 Document: blue for premises, purple for conclusions
  - [x] 3.6.2 Document: yellow for unclear arguments
  - [x] 3.6.3 Provide example annotations for each color
- [x] 3.7 Document depth parameter options
- [ ] 3.8 Test with argumentative paper

## 4. Skim Strategy Command

- [x] 4.1 Create `.opencode/command/read-skim.md`
- [x] 4.2 Write quick overview instructions (title, abstract, headings)
- [x] 4.3 Write figure review instructions
- [x] 4.4 Write conclusion extraction instructions
- [x] 4.5 Add "Color Scheme for Skimming" section
  - [x] 4.5.1 Document minimal color set (3-4 colors only)
  - [x] 4.5.2 Note which colors are not used in skim strategy
- [x] 4.6 Document time constraints
- [ ] 4.7 Test with sample paper (verify <10 min completion)

## 5. Base Command Enhancement

- [x] 5.1 Update `.opencode/command/read.md` with `strategy:` parameter
- [x] 5.2 Document available strategies
- [x] 5.3 Set default strategy to "critical"
- [x] 5.4 Add strategy descriptions to help text

## 6. Documentation

- [x] 6.1 Create strategy comparison guide (in each command file)
- [x] 6.2 Document when to use each strategy (in /read.md)
- [x] 6.3 Add examples for each strategy (in each command file)
- [ ] 6.4 Update project README with strategy overview

## Dependencies

- Task 0.1 blocks all other tasks
- Tasks 1-4 can run in parallel
- Task 5.x depends on at least one strategy (1-4) being complete
- Task 6.x can start after any strategy is complete

## Verification

- [x] Each strategy command file created
- [x] AI follows phase structure as documented
- [x] Annotations use correct colors per strategy (documented)
- [x] Output matches documented format
- [ ] Manual review of annotation quality (requires testing)

## Notes

- The directory path is `.opencode/command/` (singular), not `.opencode/commands/`
- Testing tasks (1.8, 2.8, 3.8, 4.7) require manual testing with real documents
- README update (6.4) deferred as optional enhancement
