# Changelog

## [0.1.0] — 2026-06-13

### Added

- Automatically downloads the `knap` binary from GitHub Releases on first use and keeps it up to date — no manual installation required
- Registers `knap` as the language server for Markdown files in Zed
- Support for macOS (Apple Silicon and Intel), Linux (aarch64 and x86_64), and Windows (x86_64)
- Custom binary path via `"lsp" > "knap" > "binary" > "path"` in Zed settings, with an automatic warning if the configured binary is behind the latest release
- Warning can be suppressed with `"ignore_update_warnings": true` under `"lsp" > "knap" > "settings"`
