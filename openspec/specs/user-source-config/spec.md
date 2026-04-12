## Purpose

Define how Hermes persists and resolves user-level default skills and agents source roots from `~/.config/hermes_cli/config.toml`.

## Requirements

### Requirement: Persist default source roots in a user-level Hermes config file
The system SHALL persist default `skills_source_root` and `agents_source_root` values in `~/.config/hermes_cli/config.toml`.

#### Scenario: Create the user config directory and file
- **WHEN** the user runs `hermes configure --skills-source <skills-dir> --agents-source <agents-dir>`
- **THEN** the CLI SHALL create `~/.config/hermes_cli/` if it does not already exist
- **AND** write `~/.config/hermes_cli/config.toml` with absolute paths for the provided source roots

#### Scenario: Update one configured source root without clearing the other
- **WHEN** the user runs `hermes configure` with only one source root flag
- **AND** `~/.config/hermes_cli/config.toml` already contains a value for the other source root
- **THEN** the CLI SHALL update only the provided source root
- **AND** preserve the existing value for the omitted source root

#### Scenario: Fail when no configurable values are supplied
- **WHEN** the user runs `hermes configure` without a skills source root flag and without an agents source root flag
- **THEN** the CLI SHALL fail with a clear error that explains at least one source root must be provided

### Requirement: Use the user config file during source-root resolution
The system SHALL use `~/.config/hermes_cli/config.toml` as a persisted source of default roots for commands that need skills or agents discovery.

#### Scenario: Resolve install roots from user config
- **WHEN** the user runs `hermes install` without source-root CLI flags
- **AND** `~/.config/hermes_cli/config.toml` contains source roots for the requested kinds
- **THEN** the CLI SHALL resolve those roots from the user config file
- **AND** continue without requiring a prior project-local `hermes init`

#### Scenario: Fall back when the user config file is absent
- **WHEN** `~/.config/hermes_cli/config.toml` does not exist
- **THEN** Hermes SHALL continue source-root resolution using the remaining supported sources
- **AND** fail only if a requested kind still has no resolvable source root
