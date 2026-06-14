# Releasing zed-knap

## Versioning

zed-knap follows [Semantic Versioning 2.0.0](https://semver.org/). The version
in `extension.toml` and `Cargo.toml` must always match.

| Increment | When                                                                 |
| --------- | -------------------------------------------------------------------- |
| `PATCH`   | Bug fixes to binary resolution or other extension-only logic         |
| `MINOR`   | New Zed-specific behaviour (e.g. new settings, new platform support) |
| `MAJOR`   | Breaking change to the extension's configuration shape               |

---

## Release checklist

### 1. Quality gates

```bash
cargo test    # all tests pass
cargo build   # build passes, no type errors
```

Also install locally as a dev extension (**Extensions panel → Install Dev Extension**) and confirm the language server starts.

### 2. Update version

Bump the version string in both files (they must stay in sync):

- [ ] `extension.toml` — `version = "x.y.z"`
- [ ] `Cargo.toml` — `version = "x.y.z"`

### 3. Update CHANGELOG

- [ ] Add an entry for the new version at the top of `CHANGELOG.md`

```markdown
## [x.y.z] — YYYY-MM-DD

### Added

- ...

### Fixed

- ...

### Changed

- ...
```

Use only the sections that apply. Write from the user's perspective — what
changed in their editor, not what changed in the code.

### 4. Commit and tag

```bash
git add extension.toml Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "Release vx.y.z"
git tag -a vx.y.z -m "vx.y.z"
git push && git push --tags
```

### 5. Publish to the Zed extension registry

Open a PR to [zed-industries/extensions](https://github.com/zed-industries/extensions).

1. Advance the submodule to the new tag:
   ```bash
   git submodule update --remote extensions/knap
   ```
2. Update the `version` field in `extensions.toml` to match the version in `extension.toml`

Once the PR is merged, the Zed team compiles the WASM and publishes the extension automatically.

---

## After the release

- [ ] Verify the extension appears correctly in the Zed Extensions panel
- [ ] Verify the published version installs and the language server starts
