# CLAUDE.md

Zed extension for [knap](https://github.com/sleb/knap). Its role is narrow:
locate the knap binary and register it as the Markdown language server.

**All policies, architecture decisions, and feature documentation live in the
[knap](https://github.com/sleb/knap) repository.** Only extension-specific
content belongs here.

## Commands

```bash
cargo build        # build the extension (native)
zed ext build      # compile to WASM for Zed
zed ext publish    # publish to the Zed extension registry
```

## Scope

**Belongs here:** binary resolution logic (`binary_path` in `src/lib.rs`),
`extension.toml` metadata, Zed-specific language server settings.

**Belongs in knap:** feature descriptions, server configuration options
(`initializationOptions`), architecture, roadmap, release policy.
