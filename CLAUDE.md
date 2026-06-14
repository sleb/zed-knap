# CLAUDE.md

Zed extension for [knap](https://github.com/sleb/knap). Its role is narrow:
locate the knap binary and register it as the Markdown language server.

**All policies, architecture decisions, and feature documentation live in the
[knap](https://github.com/sleb/knap) repository.** Only extension-specific
content belongs here.

## Commands

```bash
cargo build   # build and type-check the extension
```

To test locally, use **Install Dev Extension** in Zed's Extensions panel and point it at this directory.

Publishing requires a PR to [zed-industries/extensions](https://github.com/zed-industries/extensions) — see `docs/RELEASING.md`.

## Scope

**Belongs here:** binary resolution logic (`binary_path` in `src/lib.rs`),
`extension.toml` metadata, Zed-specific language server settings.

**Belongs in knap:** feature descriptions, server configuration options
(`initializationOptions`), architecture, roadmap, release policy.
