use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, GithubReleaseOptions, LanguageServerId, Os,
    Result, Worktree,
};

struct KnapExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for KnapExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self.binary_path()?;
        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: vec![],
        })
    }

}

impl KnapExtension {
    fn binary_path(&mut self) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if std::fs::metadata(path).is_ok_and(|m| m.is_file()) {
                return Ok(path.clone());
            }
        }

        // Prefer a locally installed binary (e.g. built from source via
        // `cargo install --path .`) over the downloaded release.
        let mut local_paths: Vec<String> = vec![
            "/usr/local/bin/knap".to_string(),
            "/opt/homebrew/bin/knap".to_string(),
        ];
        if let Ok(home) = std::env::var("HOME") {
            local_paths.push(format!("{home}/.cargo/bin/knap"));
        }
        for path in &local_paths {
            if std::fs::metadata(path).is_ok_and(|m| m.is_file()) {
                eprintln!("[knap] using local binary: {path}");
                self.cached_binary_path = Some(path.clone());
                return Ok(path.clone());
            }
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

        let release = zed::latest_github_release(
            "sleb/knap",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset_name = format!("knap-{platform}.tar.gz");
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("No release asset found for {platform}"))?;

        eprintln!("[knap] downloading binary version {} from GitHub", release.version);
        let binary_path = format!("knap-{}", release.version);
        zed::download_file(&asset.download_url, &binary_path, DownloadedFileType::GzipTar)?;

        // The tarball contains a top-level directory named knap-{platform}/,
        // so the binary lives one level deeper after extraction.
        let binary_name = match os {
            Os::Windows => "knap.exe",
            _ => "knap",
        };
        let path = format!("{binary_path}/knap-{platform}/{binary_name}");
        zed::make_file_executable(&path)?;

        self.cached_binary_path = Some(path.clone());
        Ok(path)
    }
}

zed::register_extension!(KnapExtension);
