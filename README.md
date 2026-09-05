# ARB for Zed

Language support for Flutter Application Resource Bundle (`.arb`) localization files in [Zed](https://zed.dev).

The extension recognizes `.arb` files as **ARB**, uses the Tree-sitter JSON grammar for syntax highlighting, and starts the separate `arb-language-server` npm package for diagnostics and completions.

## Features

- JSON syntax highlighting, indentation, and bracket matching and closing.
- JSON syntax diagnostics and ARB schema validation, including the required `@@locale` field.
- Schema-based completions for ARB metadata.
- Diagnostics for invalid message keys and metadata referencing missing messages.
- Template checks for message metadata and placeholders, plus warnings for translations missing template messages, when configured through `l10n.yaml`.
- JSON formatting through Zed's Prettier integration with the settings below.

The language server does not provide formatting or generate Flutter localization code. Its placeholder checks are not a complete ICU message validator.

## Installation

### Install from source

1. Install Zed and [Rust through rustup](https://rustup.rs/).
2. Clone this repository:

```sh
git clone https://github.com/AlexanderRavenscroft/arb.git
```

3. In Zed, run `zed: install dev extension` from the command palette, or choose **Install Dev Extension** on the Extensions page.
4. Select the cloned `arb` directory containing `extension.toml`.
5. Open a `.arb` file and confirm that its language is **ARB**.

Zed builds the Rust extension and JSON grammar. With rustup, it installs the required WebAssembly target automatically; it also downloads the grammar build tooling. See [Zed's development instructions](https://zed.dev/docs/extensions/developing-extensions) for alternative toolchain setups.

On first language-server startup, the extension installs `arb-language-server@0.0.1` from npm into its Zed-managed work directory. The package must be available on npm and network access is required for installation. Node.js is obtained through Zed's extension API; you do not need to add the server to your Flutter project's dependencies.

## Setup

### Required Prettier configuration

For Prettier formatting of ARB files, add this configuration to Zed's `settings.json`. Merge the `ARB` entry into your existing `languages` object if one is already present:

```jsonc
{
	"languages": {
		"ARB": {
			// "format_on_save": "on",
			"formatter": "prettier",
			"prettier": {
				"allowed": true,
				"parser": "json",
			},
		},
	},
}
```

`formatter` selects Prettier, `allowed` enables its use for ARB, and `parser` explicitly selects JSON for the `.arb` extension. Keep the language key as `ARB`, even though the file contents use JSON.

Uncomment `format_on_save` to explicitly enable formatting on save. Leaving it commented preserves your existing format-on-save behavior; manual document formatting remains available.

These comments and trailing commas belong to Zed's settings syntax. ARB files themselves must contain valid JSON. See [Zed's JSON formatting documentation](https://zed.dev/docs/languages/json).

### Flutter localization configuration

Open the Flutter project root in Zed and place `l10n.yaml` there. For example:

```yaml
arb-dir: lib/l10n
template-arb-file: app_en.arb
```

This identifies `lib/l10n/app_en.arb` as the template. Basic JSON/schema validation and message-key checks work without this configuration; template and translation checks require a valid configuration.

Example template:

```json
{
	"@@locale": "en",
	"greeting": "Hello, {name}!",
	"@greeting": {
		"description": "Greeting shown to the user",
		"placeholders": {
			"name": {
				"type": "String",
				"example": "Alex"
			}
		}
	}
}
```

Save template changes before expecting missing-message warnings in translations to update: those checks read the template from disk. The server currently uses the first workspace folder for its initial configuration and checks translations directly inside `arb-dir`.

## How it works

| Component | Responsibility |
| --- | --- |
| `extension.toml` | Extension metadata, JSON grammar revision, and ARB language-server registration |
| `languages/arb/` | File association, editing defaults, and Tree-sitter queries |
| `src/lib.rs` | Server installation checks and Node.js launch command |
| `Cargo.toml` | Rust library configuration and the `zed_extension_api` dependency |

The server version is pinned in `src/lib.rs`. The extension reuses the installed version when it matches and both `out/server.js` and `schemas/arb.json` exist. Otherwise it requests installation, then checks both files before returning the startup command.

The server runs locally through Node.js with `--stdio`. Zed identifies documents as `ARB` in the editor and sends `json` as their LSP language ID. The schema is distributed with the npm package; it is not bundled separately in this repository.

## Troubleshooting

| Problem | Check |
| --- | --- |
| A `.arb` file is recognized as JSON or plain text | Confirm the extension is installed and select **ARB** as the file's language. Check for conflicting file associations. |
| Formatting fails or does not run | Apply the complete `languages.ARB` Prettier configuration above. Enable format on save explicitly if needed. |
| The server cannot be installed | Run `zed: open log` and check the reported npm/package-version error and network availability. |
| An error mentions missing `out/` or `schemas/` files | The installed npm package is incomplete. Report the error; the server package must ship both directories. |
| Template or translation diagnostics are absent | Check the project-root `l10n.yaml`, its paths, and whether the template is saved and valid JSON. |

Report extension problems in [this repository's issue tracker](https://github.com/AlexanderRavenscroft/arb/issues). Include your Zed version, operating system, relevant log output, and a minimal ARB/configuration example.

## Development and publishing

Use the development installation above to test changes in Zed. Before submitting a release, manually verify file recognition, highlighting, diagnostics, completions, and Prettier formatting at the exact commit being submitted.

Follow Zed's [publishing prerequisites](https://zed.dev/docs/extensions/publishing/prerequisites) and [publishing guide](https://zed.dev/docs/extensions/publishing/publishing-guide). Keep the extension scoped to ARB support and download the language server through Zed's API rather than bundling it. Do not commit generated `target/`, `grammars/`, or `extension.wasm` build artifacts.

When changing the server version, publish and verify the npm package first, then update `PACKAGE_VERSION` in `src/lib.rs` and the version documented here. Update `extension.toml` for an extension release. `publish = false` in `Cargo.toml` prevents Cargo registry publication; the Zed extension is distributed through Zed's extension registry.

## License

[MIT](LICENSE).
