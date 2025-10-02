use std::fs;

use zed_extension_api::{
    self as zed, CodeLabel, CodeLabelSpan, GithubReleaseOptions, Range,
    lsp::{Symbol, SymbolKind},
};

struct TechniqueExtension {
    cached_binary_version: Option<String>,
    cached_binary_path: Option<String>,
}

impl zed::Extension for TechniqueExtension {
    fn new() -> Self {
        Self {
            cached_binary_version: None,
            cached_binary_path: None,
        }
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &zed::LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
        match symbol.kind {
            SymbolKind::Constructor => {
                let name = &symbol.name;

                eprintln!("{:?}", name);

                if let Some(pos) = name.find(" :") {
                    let procedure_name = &name[..pos];
                    let signature = &name[pos + 2..]; // Skip " :"

                    let mut spans = vec![
                        // Procedure name with constructor highlight
                        CodeLabelSpan::literal(procedure_name, Some("constructor".to_string())),
                        CodeLabelSpan::literal(" ", None),
                        CodeLabelSpan::literal(":", Some("punctuation.delimiter".to_string())),
                    ];

                    if signature.len() > 0 {
                        let signature = &signature[1..]; // Skip " "

                        spans.push(CodeLabelSpan::literal(" ", None));
                        spans.push(CodeLabelSpan::literal(signature, Some("type".to_string())));
                    }

                    Some(CodeLabel {
                        code: name.clone(),
                        spans,
                        filter_range: Range {
                            start: 10,
                            end: name.len() as u32,
                        },
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        let args = vec!["language".to_string()];
        let env = worktree.shell_env();

        let path = match &self.cached_binary_path {
            Some(path) => path.clone(),
            None => {
                zed::set_language_server_installation_status(
                    language_server_id,
                    &zed_extension_api::LanguageServerInstallationStatus::CheckingForUpdate,
                );

                let release = zed::latest_github_release(
                    "technique-lang/technique",
                    GithubReleaseOptions {
                        require_assets: true,
                        pre_release: false,
                    },
                )?;

                let (platform, arch) = zed::current_platform();
                let required = format!(
                    "technique-{version}-{os}-{arch}.{extension}",
                    version = release.version,
                    os = match platform {
                        zed::Os::Linux => "linux",
                        zed::Os::Mac => "darwin",
                        zed::Os::Windows => "windows",
                    },
                    arch = match arch {
                        zed::Architecture::X8664 => "x86_64",
                        zed::Architecture::Aarch64 => "aarch64",
                        zed::Architecture::X86 => "i686",
                    },
                    extension = match platform {
                        zed::Os::Linux => "gz",
                        zed::Os::Mac => "gz",
                        zed::Os::Windows => "zip",
                    },
                );

                let asset = release
                    .assets
                    .iter()
                    .find(|asset| asset.name == required)
                    .ok_or(format!("required {:?} not found in release", required))?;

                let dir = format!("technique-{}", release.version);
                std::fs::create_dir_all(&dir)
                    .map_err(|_| format!("failed to create directory: {}", dir))?;

                let path = format!("{}/technique", dir);
                if !fs::metadata(&path).map_or(false, |stat| stat.is_file()) {
                    zed::set_language_server_installation_status(
                        language_server_id,
                        &zed_extension_api::LanguageServerInstallationStatus::Downloading,
                    );

                    let url = &asset.download_url;

                    let compression = match platform {
                        zed::Os::Linux => zed_extension_api::DownloadedFileType::Gzip,
                        zed::Os::Mac => zed_extension_api::DownloadedFileType::Gzip,
                        zed::Os::Windows => zed_extension_api::DownloadedFileType::Zip,
                    };

                    zed::download_file(url, &path, compression)?;

                    zed::make_file_executable(&path)?;
                }

                self.cached_binary_path = Some(path.clone());
                self.cached_binary_version = Some(release.version);
                path
            }
        };

        Ok(zed::Command {
            command: path,
            args,
            env,
        })
    }
}

zed::register_extension!(TechniqueExtension);
