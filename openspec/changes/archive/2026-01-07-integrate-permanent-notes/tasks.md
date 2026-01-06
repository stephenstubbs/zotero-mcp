## 1. Rename Command

- [x] 1.1 Rename `readassist-zettel.md` to `readassist-permanent-note.md`
- [x] 1.2 Update all `/readassist-zettel` references to `/readassist-permanent-note` in the renamed file
- [x] 1.3 Update cross-references in `readassist-read.md` from `/readassist-zettel` to `/readassist-permanent-note`

## 2. Update Summarize Command

- [x] 2.1 Add permanent note extraction step before summary generation in `readassist-summarize.md`
- [x] 2.2 Add "Permanent Notes" section template to summary output format
- [x] 2.3 Add instructions to link to extracted permanent notes using wikilinks
- [x] 2.4 Add handling for case when no `IDEA:` annotations exist (skip section)

## 3. Update Synthesize Command

- [x] 3.1 Add permanent note extraction step for each citekey before synthesis in `readassist-synthesize.md`
- [x] 3.2 Add "Permanent Notes" section template to synthesis output format
- [x] 3.3 Group permanent notes by source citekey in output
- [x] 3.4 Add instructions to link to extracted permanent notes using wikilinks
- [x] 3.5 Add handling for case when no `IDEA:` annotations exist (skip section)

## 4. Validation

- [x] 4.1 Verify all command files reference correct new command name
- [x] 4.2 Verify no broken cross-references remain
- [x] 4.3 Run openspec validate --strict
