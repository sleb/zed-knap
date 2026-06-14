---
name: release
description: Release a new version of zed-knap. Use this skill when the user says "release", "cut a release", "publish a new version", "bump the version", or anything about shipping a new version of the extension. Walks through every step in docs/RELEASING.md: quality gates, version bump, CHANGELOG, git commit/tag/push, and PR reminder.
---

# Release zed-knap

Walk through the release checklist in `docs/RELEASING.md` in order. Do not skip steps. Confirm with the user before any destructive or outward-facing action (pushing, tagging).

## Step 1 — Determine the version

If the user didn't specify a version, ask: "What version are we releasing? (current is in `extension.toml`)" Read both `extension.toml` and `Cargo.toml` to show the current version. Decide together whether this is a PATCH, MINOR, or MAJOR increment based on what's changed:

| Increment | When |
|-----------|------|
| PATCH | Bug fixes to binary resolution or other extension-only logic |
| MINOR | New Zed-specific behaviour (new settings, new platform support) |
| MAJOR | Breaking change to the extension's configuration shape |

## Step 2 — Quality gates

Run both checks. If either fails, stop and help the user fix it before continuing.

```bash
cargo test
cargo build
```

Also remind the user to install the extension locally as a dev extension (Extensions panel → Install Dev Extension) and confirm the language server starts. Ask them to confirm before moving on.

## Step 3 — Bump version

Update the version string in **both** files — they must stay in sync:

- `extension.toml` → `version = "x.y.z"`
- `Cargo.toml` → `version = "x.y.z"`

After editing, read both back and confirm they match.

## Step 4 — Update CHANGELOG

Run `git log --oneline $(git describe --tags --abbrev=0)..HEAD` to see commits since the last tag. If no prior tag exists, use `git log --oneline`.

Draft a CHANGELOG entry at the top of `CHANGELOG.md` (create the file if it doesn't exist):

```markdown
## [x.y.z] — YYYY-MM-DD

### Added

- ...

### Fixed

- ...

### Changed

- ...
```

Use only sections that apply. Write from the user's perspective — what changed in their editor, not what changed in the code. Show the draft to the user and ask them to approve or edit it before proceeding.

## Step 5 — Commit and tag

Show the user the exact commands you're about to run and ask for confirmation before executing:

```bash
git add extension.toml Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "Release vx.y.z"
git tag -a vx.y.z -m "vx.y.z"
```

After committing and tagging locally, confirm again before pushing:

```bash
git push && git push --tags
```

## Step 6 — Publish reminder

Remind the user that publishing requires a PR to [zed-industries/extensions](https://github.com/zed-industries/extensions).

Check whether this is the first release or an update by looking for a prior git tag:

```bash
git tag --list 'v*' | sort -V | head -5
```

**First release only:**
1. Add this repo as a Git submodule under `extensions/knap`
2. Add an entry to the root `extensions.toml`
3. Run `pnpm sort-extensions`

**Subsequent releases:**
1. In the `zed-industries/extensions` repo, advance the submodule:
   ```bash
   git submodule update --remote extensions/knap
   ```
2. Update the `version` field in `extensions.toml` to match `extension.toml`

Once the PR is merged, the Zed team compiles the WASM and publishes the extension automatically.

## Step 7 — Post-release checklist

After the PR is merged, remind the user to:

- Verify the extension appears correctly in the Zed Extensions panel
- Verify the published version installs and the language server starts
