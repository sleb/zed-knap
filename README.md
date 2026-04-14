# zed-knap

A [Zed](https://zed.dev) extension for [knap](https://github.com/sleb/knap) — a Markdown LSP server that brings Obsidian-style wiki-link navigation to your editor.

## Features

- `[[wiki-link]]` completions, Go to Definition, and Find References
- Broken and ambiguous link diagnostics
- Rename a file — all `[[links]]` pointing to it are updated automatically
- Aliased links `[[Note|display text]]` — rename preserves the alias
- Attachment links `[[image.png]]` resolve against non-note files
- Configurable file extensions and vault subdirectory

## Installation

1. Open the Extensions panel in Zed (`cmd+shift+x` on macOS)
2. Search for **Knap**
3. Click **Install**

The extension automatically downloads the `knap` binary from [GitHub Releases](https://github.com/sleb/knap/releases) for your platform (macOS and Linux, x86_64 and aarch64).

## Usage

The extension activates automatically for Markdown files. Open any `.md` file in a workspace and knap will index it along with all other Markdown files in the workspace.

- **Completions** — type `[[` to get a list of all notes in the workspace
- **Go to Definition** — `cmd+click` or `F12` on a `[[wiki-link]]` to open the target file
- **Find References** — right-click a file and choose Find All References to see every note that links to it
- **Rename** — rename a file via the sidebar or `F2`; all backlinks are rewritten automatically

## Configuration

Configure knap via Zed's LSP settings in your `settings.json` (`cmd+,`):

```json
{
  "lsp": {
    "knap": {
      "initialization_options": {
        "noteRoot": "notes",
        "extensions": [".md", ".mdx"],
        "linkResolution": "shortest"
      }
    }
  }
}
```

| Option | Default | Description |
|--------|---------|-------------|
| `noteRoot` | _(workspace root)_ | Restrict indexing to a subdirectory (e.g. `"docs"` in a monorepo) |
| `extensions` | `[".md"]` | File extensions treated as notes |
| `linkResolution` | `"shortest"` | Link resolution strategy: `"shortest"` (stem only) or `"relative"` (full relative path) |
