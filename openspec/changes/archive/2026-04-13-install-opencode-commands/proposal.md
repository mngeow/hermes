## Why

Hermes currently manages shared skills and agents, but OpenCode projects can also rely on reusable custom command files under `.opencode/commands/`. Adding command installation now keeps Hermes aligned with the broader OpenCode workspace model and removes a manual copy step when bootstrapping new projects.

## What Changes

- Add support for discovering reusable OpenCode command markdown files from a configured commands source root and installing them into `.opencode/commands/`.
- Extend Hermes management flows so commands can be installed, listed, synced, removed, and validated alongside skills and agents.
- Persist a default `commands_source_root`, track installed command metadata in the local catalog, and include commands in the interactive install UI.
- Scope v1 command support to markdown command files copied verbatim; importing JSON-defined commands from OpenCode config files remains out of scope.

## Capabilities

### New Capabilities
- `command-library-installation`: Discover, validate, and install reusable OpenCode markdown commands into the project-local `.opencode/commands/` directory.

### Modified Capabilities
- `hermes-command-surface`: Extend the CLI surface so install, list, sync, remove, doctor, and configure flows can address commands as a first-class artifact kind.
- `artifact-catalog-management`: Add command workspace directories, source-root resolution, catalog metadata, sync safety, removal, and doctor coverage for installed commands.
- `user-source-config`: Persist and resolve a default `commands_source_root` alongside the existing skills and agents roots.
- `interactive-artifact-selection-ui`: Let the install TUI browse and select reusable commands in addition to skills and agents.

## Impact

- Affected code: CLI argument parsing, source-root resolution, manifest/catalog models, install/sync/remove/doctor flows, and the interactive install TUI.
- Affected files: likely `src/cli.rs`, `src/app.rs`, `src/install.rs`, `src/manifest.rs`, `src/models.rs`, TUI-related modules, README/docs, and new OpenSpec deltas for command installation behavior.
- External dependencies: no new external service dependencies expected; existing filesystem and TUI infrastructure will need to cover the new artifact kind.
