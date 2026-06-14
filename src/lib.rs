use zed_extension_api::{
    self as zed,
    process::Command as ProcessCommand,
    serde_json,
    settings::LspSettings,
    Architecture, DownloadedFileType, GithubReleaseOptions, LanguageServerId,
    LanguageServerInstallationStatus, Os, Result, Worktree,
};

struct KnapExtension;

impl zed::Extension for KnapExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.resolve_binary(language_server_id, worktree)?,
            args: vec![],
            env: vec![],
        })
    }
}

impl KnapExtension {
    fn resolve_binary(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<String> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        if let Some(path) = lsp_settings.binary.and_then(|b| b.path) {
            self.check_for_update(language_server_id, &path, &lsp_settings.settings);
            return Ok(path);
        }

        self.download_latest(language_server_id)
    }

    fn download_latest(&self, language_server_id: &LanguageServerId) -> Result<String> {
        let (os, arch) = zed::current_platform();
        let platform = match (os, arch) {
            (Os::Mac, Architecture::Aarch64) => "aarch64-apple-darwin",
            (Os::Mac, Architecture::X8664) => "x86_64-apple-darwin",
            (Os::Linux, Architecture::Aarch64) => "aarch64-unknown-linux-gnu",
            (Os::Linux, Architecture::X8664) => "x86_64-unknown-linux-gnu",
            (Os::Windows, Architecture::X8664) => "x86_64-pc-windows-msvc",
            _ => return Err("Unsupported platform".into()),
        };
        let binary_name = match os {
            Os::Windows => "knap.exe",
            _ => "knap",
        };

        let release = zed::latest_github_release(
            "sleb/knap",
            GithubReleaseOptions { require_assets: true, pre_release: false },
        )
        .map_err(|e| {
            let msg = format!(
                "knap is not installed and GitHub is unreachable ({e}). \
                Install knap (e.g. `cargo install knap`) then set its path in Zed settings: \
                \"lsp\": {{ \"knap\": {{ \"binary\": {{ \"path\": \"/path/to/knap\" }} }} }}"
            );
            zed::set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Failed(msg.clone()),
            );
            msg
        })?;

        let dir = format!("knap-{}", release.version);
        let path = format!("{dir}/knap-{platform}/{binary_name}");

        if std::fs::metadata(&path).is_ok_and(|m| m.is_file()) {
            return Ok(path);
        }

        let (asset_name, file_type) = match os {
            Os::Windows => (
                format!("knap-{platform}.zip"),
                DownloadedFileType::Zip,
            ),
            _ => (
                format!("knap-{platform}.tar.gz"),
                DownloadedFileType::GzipTar,
            ),
        };
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("No release asset found for {platform}"))?;

        eprintln!("[knap] downloading version {} from GitHub", release.version);
        zed::download_file(&asset.download_url, &dir, file_type)?;
        zed::make_file_executable(&path)?;

        Ok(path)
    }

    // Best-effort: logs a warning if the configured binary is behind the latest GitHub release.
    // Skips silently if GitHub is unreachable; logs to stderr if the binary can't be queried.
    fn check_for_update(
        &self,
        language_server_id: &LanguageServerId,
        path: &str,
        settings: &Option<serde_json::Value>,
    ) {
        if let Err(e) = self.try_check_for_update(language_server_id, path, settings) {
            eprintln!("[knap] update check failed: {e}");
        }
    }

    fn try_check_for_update(
        &self,
        language_server_id: &LanguageServerId,
        path: &str,
        settings: &Option<serde_json::Value>,
    ) -> Result<()> {
        let ignore = settings
            .as_ref()
            .and_then(|s: &serde_json::Value| s.get("ignore_update_warnings"))
            .and_then(|v: &serde_json::Value| v.as_bool())
            .unwrap_or(false);
        if ignore {
            return Ok(());
        }

        // Offline or GitHub unreachable: skip silently.
        let Ok(release) = zed::latest_github_release(
            "sleb/knap",
            GithubReleaseOptions { require_assets: false, pre_release: false },
        ) else {
            return Ok(());
        };
        let latest = release.version.trim_start_matches('v');

        let output = ProcessCommand::new(path)
            .arg("--version")
            .output()
            .map_err(|e| format!("failed to run `{path} --version`: {e}"))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let installed = parse_installed_version(&stdout)
            .ok_or_else(|| format!("could not parse version from `{path} --version` output"))?;

        if installed != latest {
            let msg = format!(
                "knap {installed} is outdated (latest: {latest}). \
                Update your binary or add \"ignore_update_warnings\": true under \
                \"lsp\" > \"knap\" > \"settings\" in your Zed settings to suppress this warning."
            );
            eprintln!("[knap] WARNING: {msg}");
            zed::set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Failed(msg),
            );
        }

        Ok(())
    }
}

fn parse_installed_version(output: &str) -> Option<&str> {
    output.split_whitespace().find_map(|w| {
        let v = w.trim_start_matches('v');
        v.starts_with(|c: char| c.is_ascii_digit()).then_some(v)
    })
}

zed::register_extension!(KnapExtension);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bare_version() {
        assert_eq!(parse_installed_version("knap 0.1.2"), Some("0.1.2"));
    }

    #[test]
    fn parses_v_prefixed_version() {
        assert_eq!(parse_installed_version("knap v0.1.2"), Some("0.1.2"));
    }

    #[test]
    fn parses_version_only() {
        assert_eq!(parse_installed_version("0.1.2"), Some("0.1.2"));
    }

    #[test]
    fn returns_none_for_empty_output() {
        assert_eq!(parse_installed_version(""), None);
    }

    #[test]
    fn returns_none_for_non_version_output() {
        assert_eq!(parse_installed_version("error: something went wrong"), None);
    }
}
