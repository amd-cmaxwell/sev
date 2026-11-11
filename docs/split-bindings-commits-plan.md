TODO
---
# Split bindings into per-language commits

## Context

The `publish-bindings` job in `release.yml` currently makes one large commit containing Go, Python, and JS bindings together. This makes the bindings branch history hard to navigate. We want to split this into 3 separate commits (one per language), with tags pointing to the appropriate language-specific commit.

Also includes the pending changes: removing `prepare` script from the JS template and committing `dist/` for JS packages.

## Changes — all in `.github/workflows/release.yml`

### 1. Update `publish-bindings` outputs

Change `bindings-commit` (singular SHA) to `bindings-commit-count` (number of commits made: 0–3), so rollback knows how many commits to revert.

### 2. Restructure the commit step into 3 sequential commits

After merging main into bindings, make up to 3 commits in order:

**Commit 1 — Go bindings:**
- `git add -f bindings/go/`
- Commit message: `chore: generate Go bindings for <summary>`
- Tag immediately: `bindings/go/{pkg}/v{version}` for each FFI module

**Commit 2 — Python bindings:**
- `git add -f bindings/python/`
- Commit message: `chore: generate Python bindings for <summary>`
- Tag immediately: `ffi/{crate_dir}/v{version}` for each FFI module

**Commit 3 — JS bindings:**
- `git add -f bindings/js/...` (package.json, pnpm-workspace.yaml, src/, dist/)
- Commit message: `chore: generate JS bindings for <summary>`
- Tag immediately: `bindings/js/{npm_scope}/{pkg}/v{version}` for each FFI module

Each commit is only created if `git diff --cached` is non-empty (skip if no changes for that language). Error if zero commits were made total.

### 3. Remove the separate "Tag bindings commit" step

Tags are now created inline with each commit so they point to the correct language-specific commit.

### 4. Push once at the end

Single `git push origin bindings --tags` after all commits and tags are created.

### 5. Update rollback

- **"Reset bindings branch"**: Instead of reverting one commit (`~1`), revert N commits (`~N`) using the `bindings-commit-count` output. The safety check becomes: verify that `origin/bindings` matches the expected HEAD (last commit SHA, output alongside the count).
- **Add "Delete JS bindings tags"** step: mirrors the existing "Delete FFI bindings tags" and "Delete Go module tags" steps, deleting `bindings/js/{scope}/{pkg}/v{version}` tags.

### 6. Include the JS `dist/` and template changes

- The `git add -f "${pkg_dir}dist/"` line is already in the working tree from the earlier edit
- The `templates/js/package.json` change (removing `prepare` script) is already in the working tree

## Tag assignment

| Tag pattern | Points to | Consumer |
|---|---|---|
| `bindings/go/{pkg}/v{ver}` | Go commit | Go modules |
| `ffi/{crate}/v{ver}` | Python commit | pip install |
| `bindings/js/{scope}/{pkg}/v{ver}` | JS commit | pnpm git install |

## Verification

- Review the diff to confirm tag creation happens on the right commit
- Run the release workflow via `workflow_dispatch` and verify:
  - 3 separate commits appear on the bindings branch
  - Go tags point to the Go commit
  - FFI tags point to the Python commit
  - Rollback (if triggered) correctly resets all 3 commits
