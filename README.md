# zed-knap

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A [Zed](https://zed.dev) extension that connects the
[knap](https://github.com/sleb/knap) language server to Zed for Markdown files.

The extension's role is narrow: it downloads the `knap` binary from GitHub
Releases and registers it as the language server for Markdown. All features are
implemented in the server — see the [knap README](https://github.com/sleb/knap)
for the full list.

## What you get

- **Completions** — file path and anchor (`#heading`) completions inside Markdown links; frontmatter key/value completions from your schema
- **Navigation** — Go to Definition on links and anchors; Find References on files, headings, and tags; Document and Workspace Symbols
- **Diagnostics & fixes** — broken link and anchor warnings; Quick Fix to create a missing file or replace a broken anchor
- **Refactoring** — rename a file, heading, or tag and all references update atomically
- **Inlay hints & code lens** — title hints next to link paths; backlink counts above notes with incoming links

See the [knap README](https://github.com/sleb/knap) for the full feature list.

## Getting started

1. Open the Extensions panel in Zed (`cmd+shift+x` on macOS)
2. Search for **Knap** and click **Install**

The extension downloads the `knap` binary automatically on first use and keeps it up to date — no manual installation required.

## Configuration

All settings live under `"lsp" > "knap"` in your Zed `settings.json`
(`cmd+,` → _Open Settings File_).

### Server configuration

Pass knap options via `initialization_options`. knap works with zero
configuration for a standard single-folder Markdown workspace; the most
common options are:

| Option              | Type       | Default  | Description                                              |
| ------------------- | ---------- | -------- | -------------------------------------------------------- |
| `extensions`        | `string[]` | `["md"]` | File extensions treated as notes                         |
| `newNoteDir`        | `string`   | —        | Folder where Quick Fix "Create note" places new files    |
| `frontmatterSchema` | `object`   | —        | Allowed frontmatter keys and values; enables completions and diagnostics |

```json
{
  "lsp": {
    "knap": {
      "initialization_options": {
        "extensions": ["md", "mdx"],
        "newNoteDir": "0-Inbox"
      }
    }
  }
}
```

See [Getting Started](https://github.com/sleb/knap/blob/main/docs/GETTING_STARTED.md)
in the knap repo for the full configuration reference, including `frontmatterSchema`.

### Code lens

Code lens is disabled by default in Zed. To see backlink counts above your
notes, enable it in `settings.json`:

```json
{
  "code_lens": true
}
```

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
