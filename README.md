# ARB for Zed

Language support for Flutter Application Resource Bundle (`.arb`) localization files in [Zed](https://zed.dev).

The extension recognizes `.arb` files as **ARB** and provides tools for editing messages, metadata, and placeholders.

## Features

- JSON syntax highlighting, indentation, bracket matching, and automatic bracket closing.
- JSON syntax and ARB schema diagnostics, plus metadata completions.
- Checks for invalid message keys and metadata referencing missing messages.
- Template checks for missing message metadata, invalid placeholder names, and missing or unused placeholder metadata.
- Warnings for translations missing messages from the saved template.
- Quick fixes to add missing message and placeholder metadata.
- Snippets for messages, placeholders, plurals, selects, and new ARB files.

Template and translation checks require `l10n.yaml`; basic JSON/schema validation and message-key checks work without it. Formatting is available through Zed's Prettier integration as configured below.

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

Zed builds the Rust extension and Tree-sitter JSON grammar. With rustup, it installs the required WebAssembly target automatically and downloads the grammar build tooling. See [Zed's development instructions](https://zed.dev/docs/extensions/developing-extensions) for alternative setups.

On first use, the extension installs [arb-language-server@1.0.0](https://www.npmjs.com/package/arb-language-server/v/1.0.0) into its Zed-managed work directory. Zed manages the Node.js runtime; no npm dependency needs to be added to your Flutter project. Initial installation requires network access.

## Setup

### Flutter localization configuration

Open the Flutter project root in Zed and place `l10n.yaml` there. For example:

```yaml
arb-dir: lib/l10n
template-arb-file: app_en.arb
```

This identifies `lib/l10n/app_en.arb` as the template. For supported options such as `use-escaping`, `relax-syntax`, and `required-resource-attributes`, see the [server configuration reference](https://github.com/AlexanderRavenscroft/arb-language-server#project-configuration).

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

Save template changes to update missing-message warnings in open translations. For quick fixes, open Zed's code actions on the affected message or placeholder. If both message and placeholder metadata are missing, add the message metadata first. Quick fixes require valid JSON.

### Formatting (optional)

Merge this configuration into Zed's `settings.json` to format ARB files with Prettier:

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

Uncomment `format_on_save` to enable formatting on save. Keep the language key as `ARB` and the parser as `json`. ARB files must contain valid JSON, without comments or trailing commas. See [Zed's language settings](https://zed.dev/docs/configuring-languages).

## Snippets

Type a prefix in an ARB file and select its snippet from the completion menu:

| Prefix           | Inserts                                       |
| ---------------- | --------------------------------------------- |
| `arb`            | An ARB file with locale and context fields    |
| `string`         | A message with description metadata           |
| `stringWithArgs` | A message with a placeholder and its metadata |
| `plural`         | A plural message with an integer placeholder  |
| `select`         | A select message with a string placeholder    |

Insert message snippets inside the root object and add commas between neighboring entries as needed.

## Current limitations

- The server warns when `@@locale` is missing, even if Flutter can infer the locale from the filename.
- Open one Flutter project at its root. Configuration initially uses the first workspace folder; multiple or nested project configurations are unsupported.
- Missing-message checks cover translations directly inside `arb-dir` and read the template from disk.
- An empty, comment-only, or `null` `l10n.yaml` can prevent server startup. Use a YAML mapping such as the example above, then restart the language server.
- Placeholder checks are not a complete ICU message validator. The server does not provide formatting or generate Flutter localization code.

## Troubleshooting

| Problem                                           | Check                                                                                                                  |
| ------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| A `.arb` file is recognized as JSON or plain text | Confirm the extension is installed and select **ARB** as the file's language. Check for conflicting file associations. |
| Formatting fails                                  | Apply the `languages.ARB` Prettier configuration above.                                                                |
| The server fails to install or start              | Run `zed: open log`; check network access, package errors, and `l10n.yaml`.                                            |
| Template or translation diagnostics are absent    | Check the project-root `l10n.yaml`, its paths, and whether the template is saved and valid JSON.                       |

Report extension problems in [this repository's issue tracker](https://github.com/AlexanderRavenscroft/arb/issues). Include your Zed version, operating system, relevant log output, and a minimal ARB/configuration example.

## Development

The Rust adapter uses `zed_extension_api`; see [Cargo.toml](Cargo.toml) for its version. The separate [language server](https://github.com/AlexanderRavenscroft/arb-language-server) supplies diagnostics and completions. Publish and test a server release before changing its pin in `src/lib.rs` and this README.

Before a Zed release, test the exact submission commit as a dev extension and follow Zed's [publishing prerequisites](https://zed.dev/docs/extensions/publishing/prerequisites) and [publishing guide](https://zed.dev/docs/extensions/publishing/publishing-guide). Keep generated `target/`, `grammars/`, and `extension.wasm` artifacts out of Git.

## License

[MIT](LICENSE).
