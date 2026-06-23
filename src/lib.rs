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

                if let Some(pos) = name.find(" :") {
                    let procedure_name = &name[..pos];

                    Some(CodeLabel {
                        code: name.clone(),
                        spans: vec![CodeLabelSpan::CodeRange(Range {
                            start: 0,
                            end: name.len() as u32,
                        })],
                        filter_range: Range {
                            start: 0,
                            end: procedure_name.len() as u32,
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
                // Prefer the latest published release, but if that check is
                // unavailable (e.g. we're offline) fall back to the newest
                // binary already downloaded.
                let path = match self.fetch_latest_binary(language_server_id) {
                    Ok(path) => path,
                    Err(error) => self.find_latest_local_binary().ok_or(error)?,
                };
                self.cached_binary_path = Some(path.clone());
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

impl TechniqueExtension {
    fn fetch_latest_binary(
        &mut self,
        language_server_id: &zed::LanguageServerId,
    ) -> Result<String, String> {
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

        self.cached_binary_version = Some(release.version);
        self.remove_stale_binaries(&dir);
        Ok(path)
    }

    // Remove previously-downloaded binaries other than the current one so
    // they don't accumulate across releases. Best-effort.
    fn remove_stale_binaries(&self, keep: &str) {
        let Ok(entries) = fs::read_dir(".") else {
            return;
        };
        for entry in entries {
            let Ok(entry) = entry else {
                continue;
            };
            let Ok(name) = entry.file_name().into_string() else {
                continue;
            };
            if name == keep {
                continue;
            }
            let Some(version) = name.strip_prefix("technique-") else {
                continue;
            };
            let Some(_) = parse_version(version) else {
                continue;
            };
            let _ = fs::remove_dir_all(&name);
        }
    }

    // Locate the most recent previously-downloaded binary, used when unable
    // to check for a newer release upstream.
    fn find_latest_local_binary(&mut self) -> Option<String> {
        let mut best: Option<((u32, u32, u32), String, String)> = None;

        for entry in fs::read_dir(".").ok()? {
            let Ok(entry) = entry else {
                continue;
            };
            let Ok(name) = entry.file_name().into_string() else {
                continue;
            };
            let Some(version) = name.strip_prefix("technique-") else {
                continue;
            };
            let path = format!("{}/technique", name);
            if !fs::metadata(&path).map_or(false, |stat| stat.is_file()) {
                continue;
            }
            let Some(parsed) = parse_version(version) else {
                continue;
            };
            if best.as_ref().map_or(true, |(v, _, _)| parsed > *v) {
                best = Some((parsed, path, version.to_string()));
            }
        }

        let (_, path, version) = best?;
        self.cached_binary_version = Some(version);
        Some(path)
    }
}

// Parse a release tag like "v0.6.2" into comparable components.
fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
    let mut parts = version.trim_start_matches('v').split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next()?.parse().ok()?;
    Some((major, minor, patch))
}

zed::register_extension!(TechniqueExtension);
