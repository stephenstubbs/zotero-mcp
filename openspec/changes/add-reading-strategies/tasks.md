# Tasks: Add Advanced Reading Strategies

## Prerequisites
- [ ] 0.1 Phase 1 (`add-critical-reading-workflow`) is complete and merged

## 1. SQ3R Strategy Command

- [ ] 1.1 Create `.opencode/commands/read-sq3r.md`
- [ ] 1.2 Write Survey phase instructions with structure highlighting
- [ ] 1.3 Write Question phase instructions with question generation
- [ ] 1.4 Write Read phase instructions with answer highlighting
- [ ] 1.5 Write Recite phase instructions with summarization
- [ ] 1.6 Write Review phase instructions with note creation
- [ ] 1.7 Add "Color Scheme for SQ3R" section to command file
  - [ ] 1.7.1 Document: yellow for questions, green for answers, red for confusion
  - [ ] 1.7.2 Document: blue/purple for structure during Survey phase
  - [ ] 1.7.3 Provide example annotations for each color
- [ ] 1.8 Test end-to-end with sample textbook chapter

## 2. Literature Review Strategy Command

- [ ] 2.1 Create `.opencode/commands/read-review.md`
- [ ] 2.2 Write metadata extraction instructions
- [ ] 2.3 Write evidence extraction instructions with claim/limitation highlighting
- [ ] 2.4 Write quality assessment instructions
- [ ] 2.5 Write categorization instructions with thematic tagging
- [ ] 2.6 Add "Color Scheme for Literature Review" section
  - [ ] 2.6.1 Document: green for supported claims, red for limitations
  - [ ] 2.6.2 Document: yellow for gaps in literature
  - [ ] 2.6.3 Provide example annotations for each color
- [ ] 2.7 Document output format for synthesis matrix
- [ ] 2.8 Test with sample research paper

## 3. Analytical Reading Strategy Command

- [ ] 3.1 Create `.opencode/commands/read-analyze.md`
- [ ] 3.2 Write structural analysis instructions (thesis, arguments)
- [ ] 3.3 Write interpretation instructions (terms, assumptions)
- [ ] 3.4 Write critique instructions (validity, gaps)
- [ ] 3.5 Write integration instructions (connections, implications)
- [ ] 3.6 Add "Color Scheme for Analytical Reading" section
  - [ ] 3.6.1 Document: blue for premises, purple for conclusions
  - [ ] 3.6.2 Document: yellow for unclear arguments
  - [ ] 3.6.3 Provide example annotations for each color
- [ ] 3.7 Document depth parameter options
- [ ] 3.8 Test with argumentative paper

## 4. Skim Strategy Command

- [ ] 4.1 Create `.opencode/commands/read-skim.md`
- [ ] 4.2 Write quick overview instructions (title, abstract, headings)
- [ ] 4.3 Write figure review instructions
- [ ] 4.4 Write conclusion extraction instructions
- [ ] 4.5 Add "Color Scheme for Skimming" section
  - [ ] 4.5.1 Document minimal color set (3-4 colors only)
  - [ ] 4.5.2 Note which colors are not used in skim strategy
- [ ] 4.6 Document time constraints
- [ ] 4.7 Test with sample paper (verify <10 min completion)

## 5. Base Command Enhancement

- [ ] 5.1 Update `.opencode/commands/read.md` with `strategy:` parameter
- [ ] 5.2 Document available strategies
- [ ] 5.3 Set default strategy to "critical"
- [ ] 5.4 Add strategy descriptions to help text

## 6. Documentation

- [ ] 6.1 Create strategy comparison guide
- [ ] 6.2 Document when to use each strategy
- [ ] 6.3 Add examples for each strategy
- [ ] 6.4 Update project README with strategy overview

## Dependencies

- Task 0.1 blocks all other tasks
- Tasks 1-4 can run in parallel
- Task 5.x depends on at least one strategy (1-4) being complete
- Task 6.x can start after any strategy is complete

## Verification

- Each strategy command executes without errors
- AI follows phase structure as documented
- Annotations use correct colors per strategy
- Output matches documented format
- Manual review of annotation quality
