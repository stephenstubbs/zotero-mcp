# Change: Rename slash commands to readassist prefix

## Why

The reading-assistance slash commands currently use various prefixes (`/read`, `/summarize`, `/synthesize`, `/zettel`). To establish a consistent branding and clearly distinguish these commands from the openspec commands, all non-openspec commands should use a unified `readassist` prefix.

## What Changes

- **Rename command files** in `.opencode/command/`:
  - `read.md` → `readassist-read.md`
  - `read-analyze.md` → `readassist-read-analyze.md`
  - `read-review.md` → `readassist-read-review.md`
  - `read-skim.md` → `readassist-read-skim.md`
  - `read-sq3r.md` → `readassist-read-sq3r.md`
  - `summarize.md` → `readassist-summarize.md`
  - `synthesize.md` → `readassist-synthesize.md`
  - `zettel.md` → `readassist-zettel.md`

- **Update command invocations** within each file:
  - `/read` → `/readassist-read`
  - `/read-analyze` → `/readassist-read-analyze`
  - `/read-review` → `/readassist-read-review`
  - `/read-skim` → `/readassist-read-skim`
  - `/read-sq3r` → `/readassist-read-sq3r`
  - `/summarize` → `/readassist-summarize`
  - `/synthesize` → `/readassist-synthesize`
  - `/zettel` → `/readassist-zettel`

- **Update cross-references** between command files where they reference each other

- **Files NOT changed** (keep as-is):
  - `openspec-apply.md`
  - `openspec-archive.md`
  - `openspec-proposal.md`

## Impact

- Affected specs: `slash-command`
- Affected code: `.opencode/command/*.md` files (8 files renamed and updated)
- No code changes required (only documentation/command definition files)
- No breaking API changes (this is a naming convention update for slash commands)
