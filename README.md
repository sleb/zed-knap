# zed-knap

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A [Zed](https://zed.dev) extension that connects the
[knap](https://github.com/sleb/knap) language server to Zed for Markdown files.

The extension's role is narrow: it downloads the `knap` binary from GitHub
Releases and registers it as the language server for Markdown. All features —
wiki-link completions, Go to Definition, Find References, rename refactoring —
are implemented in the knap server and work through Zed's standard LSP support.

> **Backlinks code lens:** knap v0.7 adds a `↑ N backlinks` annotation at the
> top of every note. Zed support is pending an upcoming Zed release that adds
> code lens rendering. The annotation will appear automatically once that update
> ships — no extension change required.

## Getting started

1. Open the Extensions panel in Zed (`cmd+shift+x` on macOS)
2. Search for **Knap** and click **Install**

The extension downloads the `knap` binary automatically on first use and keeps it up to date — no manual installation required.

## Configuration

All settings live under `"lsp" > "knap"` in your Zed `settings.json`
(`cmd+,` → *Open Settings File*).

### Custom binary path

If you prefer to manage the `knap` binary yourself (e.g. built from source or
installed via `cargo install knap`), point the extension at it:

```json
{
  "lsp": {
    "knap": {
      "binary": {
        "path": "/path/to/knap"
      }
    }
  }
}
```

When a custom path is set the extension checks GitHub on startup and warns in
the language server log if your binary is behind the latest release.

### Suppressing the outdated binary warning

If you are intentionally running an older version, silence the warning by adding
`"ignore_update_warnings"` to the `"settings"` block:

```json
{
  "lsp": {
    "knap": {
      "binary": {
        "path": "/path/to/knap"
      },
      "settings": {
        "ignore_update_warnings": true
      }
    }
  }
}
```
