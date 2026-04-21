# zed-knap

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A [Zed](https://zed.dev) extension that connects the
[knap](https://github.com/sleb/knap) language server to Zed for Markdown files.

The extension's role is narrow: it downloads the `knap` binary from GitHub
Releases and registers it as the language server for Markdown. All features —
wiki-link completions, Go to Definition, Find References, rename refactoring —
are implemented in the knap server and work through Zed's standard LSP support.

## Installation

1. Open the Extensions panel in Zed (`cmd+shift+x` on macOS)
2. Search for **Knap**
3. Click **Install**

The extension downloads the `knap` binary automatically for your platform.
