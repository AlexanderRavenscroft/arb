use std::{env, path::PathBuf};

use zed_extension_api::{self as zed, Result};

const PACKAGE_NAME: &str = "arb-language-server";
const PACKAGE_VERSION: &str = "1.0.0";
const SERVER_ENTRY_POINT: &str = "out/server.js";

struct ArbExtension;

impl ArbExtension {
    fn server_script_path(language_server_id: &zed::LanguageServerId) -> Result<PathBuf> {
        let extension_dir = env::current_dir()
            .map_err(|error| format!("failed to locate the extension work directory: {error}"))?;
        let package_dir = extension_dir.join("node_modules").join(PACKAGE_NAME);
        let server_path = package_dir.join(SERVER_ENTRY_POINT);
        let schema_path = package_dir.join("schemas/arb.json");
        let installed_version =
            zed::npm_package_installed_version(PACKAGE_NAME).map_err(|error| {
                format!("failed to read installed '{PACKAGE_NAME}' version: {error}")
            })?;

        // Reuse the tested version without contacting npm on every startup.
        if installed_version.as_deref() != Some(PACKAGE_VERSION)
            || !server_path.is_file()
            || !schema_path.is_file()
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION).map_err(|error| {
                format!("failed to install '{PACKAGE_NAME}@{PACKAGE_VERSION}' from npm: {error}")
            })?;
        }

        for path in [&server_path, &schema_path] {
            if !path.is_file() {
                return Err(format!(
					"installed '{PACKAGE_NAME}@{PACKAGE_VERSION}' is missing '{}'; the npm package must include out/ and schemas/",
					path.display(),
				));
            }
        }

        Ok(server_path)
    }
}

impl zed::Extension for ArbExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = Self::server_script_path(language_server_id)?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                server_path.to_string_lossy().into_owned(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(ArbExtension);
