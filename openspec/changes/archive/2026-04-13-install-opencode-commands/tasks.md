## 1. Configuration and workspace plumbing

- [x] 1.1 Extend CLI/config/environment source-root resolution to support `--commands-source`, `commands_source_root`, and the command source root precedence rules.
- [x] 1.2 Update workspace bootstrap and catalog models so Hermes creates `.opencode/commands/` and can persist installed command metadata and hashes.

## 2. Command artifact lifecycle

- [x] 2.1 Implement command discovery and validation for direct child markdown files in the configured commands source root.
- [x] 2.2 Implement command installation flows for `hermes install --commands ...` and `hermes install commands ...`, preserving command files verbatim in `.opencode/commands/`.
- [x] 2.3 Extend `list`, `sync`, `remove`, and `doctor` to manage installed commands with the same hash-based overwrite protection used for other artifacts.

## 3. Interactive UX and documentation

- [x] 3.1 Update the interactive install TUI to display, select, and return command artifacts alongside skills and agents.
- [x] 3.2 Update README and supporting docs/examples to describe command source configuration, command installation, and the markdown-only scope for v1.

## 4. Verification

- [x] 4.1 Add or update automated tests covering command discovery, install/catalog behavior, and command-aware management flows.
- [x] 4.2 Run `cargo build` and confirm the build passes.
