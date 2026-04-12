use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

struct KnapExtension;

impl zed::Extension for KnapExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Command> {
        // The binary path is overridden per-project via `.zed/settings.json`
        // `lsp.knap.binary.path`. This fallback finds `knap` on PATH.
        Ok(Command::new("knap"))
    }
}

zed::register_extension!(KnapExtension);
