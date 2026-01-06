# Change: Integrate Permanent Notes into Summarize and Synthesize Workflows

## Why

The current workflow treats permanent note extraction (`/readassist-zettel`) as a separate step from summarization and synthesis. This means:
1. Users must manually run zettel extraction before summary/synthesis
2. Summary and synthesis notes don't reference the atomic permanent notes that were extracted
3. Connections between permanent notes and their source summaries are lost

By integrating permanent note extraction into summarize/synthesize, users get:
- Automatic extraction of `IDEA:`-marked annotations as permanent notes
- Links from summaries/syntheses to the permanent notes (showing connections)
- A single command that produces both the summary AND the underlying permanent notes

## What Changes

### 1. Rename Command
- `/readassist-zettel` → `/readassist-permanent-note`
- More descriptive name that aligns with Zettelkasten terminology

### 2. Integrate into Summarize
- `/readassist-summarize` first runs permanent note extraction (same logic as `/readassist-permanent-note`)
- After extraction, generates the summary as before
- Summary includes a "Permanent Notes" section listing extracted notes with wikilinks

### 3. Integrate into Synthesize  
- `/readassist-synthesize` first runs permanent note extraction for each citekey
- After extraction, generates the synthesis as before
- Synthesis includes a "Permanent Notes" section grouping notes by source with wikilinks

### 4. New Output Section
Both summarize and synthesize will include:
```markdown
## Permanent Notes

The following atomic ideas were extracted from annotations marked with `IDEA:`:

- [[transfer-learning-trades-data-for-compute]] - From [@smithML2023 p. 18]
- [[attention-is-all-you-need]] - From [@smithML2023 p. 25]
```

## Approach

1. Extract permanent note logic into a reusable workflow step
2. Both summarize and synthesize call this step first
3. Track which permanent notes were created
4. Add a "Permanent Notes" section to the output template
5. Use wikilinks (`[[filename]]`) to connect to permanent notes

## Impact

- **Modified specs**: `slash-command`, `note-synthesis`
- **Renamed file**: `readassist-zettel.md` → `readassist-permanent-note.md`
- **Modified files**: `readassist-summarize.md`, `readassist-synthesize.md`

## Decisions

- Permanent note extraction runs silently as part of summarize/synthesize (no separate confirmation)
- If no `IDEA:` annotations exist, the "Permanent Notes" section is omitted
- Existing permanent notes are not overwritten (append number if exists)
