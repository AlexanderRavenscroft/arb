use std::{env, fs};

use zed_extension_api::serde_json::{self, json};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, Result};

const LANGUAGE_SERVER_ID: &str = "arb-language-server";
const PACKAGE_NAME: &str = "vscode-langservers-extracted";
const SERVER_PATH: &str =
    "node_modules/vscode-langservers-extracted/bin/vscode-json-language-server";

const ARB_SCHEMA: &str = include_str!("../schemas/arb.json");

struct ArbExtension {
    did_find_server: bool,
}

impl ArbExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).is_ok_and(|metadata| metadata.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &zed::LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;

        if !server_exists || installed_version.as_ref() != Some(&latest_version) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let install_result = zed::npm_install_package(PACKAGE_NAME, &latest_version);
            if let Err(error) = install_result {
                if !self.server_exists() {
                    return Err(error);
                }
            }

            if !self.server_exists() {
                return Err(format!(
					"installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
				));
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for ArbExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_script_path = self.server_script_path(language_server_id)?;
        let extension_dir = env::current_dir()
            .map_err(|error| format!("failed to locate the extension directory: {error}"))?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                extension_dir
                    .join(server_script_path)
                    .to_string_lossy()
                    .into_owned(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let initialization_options = LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|settings| settings.initialization_options)
            .unwrap_or_else(|| {
                json!({
                    "provideFormatter": false
                })
            });

        Ok(Some(initialization_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let arb_schema: serde_json::Value = serde_json::from_str(ARB_SCHEMA)
            .map_err(|error| format!("failed to parse bundled ARB schema: {error}"))?;

        let workspace_configuration = LspSettings::for_worktree(LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|settings| settings.settings)
            .unwrap_or_else(|| {
                json!({
                    "json": {
                        "format": {
                            "enable": false
                        },
                        "validate": {
                            "enable": true
                        },
                        "schemas": [
                            {
                                "fileMatch": ["*.arb"],
                                "url": "arb://schemas/arb.json",
                                "schema": arb_schema
                            }
                        ]
                    }
                })
            });

        Ok(Some(workspace_configuration))
    }
}

zed::register_extension!(ArbExtension);
