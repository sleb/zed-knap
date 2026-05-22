use zed_extension_api::{
    self as zed,
    settings::LspSettings,
    Architecture, DownloadedFileType, GithubReleaseOptions, LanguageServerId, Os, Result, Worktree,
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
            command: self.binary_path(language_server_id, worktree)?,
            args: vec![],
            env: vec![],
        })
    }
}

impl KnapExtension {
    fn binary_path(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<String> {
        // User-configured path takes priority — no download, no version check.
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        if let Some(path) = settings.binary.and_then(|b| b.path) {
            return Ok(path);
        }

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

        // Always fetch latest release metadata so new versions are picked up on startup.
        let release = zed::latest_github_release(
            "sleb/knap",
            GithubReleaseOptions { require_assets: true, pre_release: false },
        )?;

        // The versioned directory is the cache key: if it exists, the binary is current.
        let dir = format!("knap-{}", release.version);
        let path = format!("{dir}/knap-{platform}/{binary_name}");

        if std::fs::metadata(&path).is_ok_and(|m| m.is_file()) {
            return Ok(path);
        }

        let asset_name = format!("knap-{platform}.tar.gz");
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("No release asset found for {platform}"))?;

        eprintln!("[knap] downloading version {} from GitHub", release.version);
        zed::download_file(&asset.download_url, &dir, DownloadedFileType::GzipTar)?;
        zed::make_file_executable(&path)?;

        Ok(path)
    }
}

zed::register_extension!(KnapExtension);
