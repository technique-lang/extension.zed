use zed_extension_api as zed;

struct TechniqueExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for TechniqueExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {        
        let path = "/home/andrew/src/technique-lang/technique/target/debug/technique";
        self.cached_binary_path = Some(path.to_string());

        let command = path.to_string();
        let args = vec!["language".to_string()];
        let env = vec![];

        Ok(zed::Command { command, args, env })
    }
}

zed::register_extension!(TechniqueExtension);
