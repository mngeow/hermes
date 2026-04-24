## Purpose

Define how `hermes` initializes the local `.opencode` workspace, records installed artifacts in a unified catalog, resolves source roots, and safely manages sync, removal, and validation.

## Requirements

### Requirement: Initialize a project-local OpenCode artifact workspace
The system SHALL create a project-local `.opencode` workspace with `skills/`, `agents/`, `commands/`, and `catalog.toml` when initializing the installer in a project and when an install operation needs to bootstrap a new project.

#### Scenario: Initialize the local workspace
- **WHEN** the user runs `hermes init` with at least one resolvable source root
- **THEN** the CLI SHALL create `.opencode/`, `.opencode/skills/`, `.opencode/agents/`, `.opencode/commands/`, and `.opencode/catalog.toml`
- **AND** record the install mode

#### Scenario: Require at least one resolvable source root
- **WHEN** `hermes init` is invoked without a skills source root, without an agents source root, and without a commands source root after considering CLI flags, user config, and environment variables
- **THEN** the CLI SHALL fail with a clear error

#### Scenario: Bootstrap the local workspace during install
- **WHEN** the user runs `hermes install` in a project without `.opencode/catalog.toml`
- **AND** at least one requested kind has a resolvable source root
- **THEN** the CLI SHALL create `.opencode/`, `.opencode/skills/`, `.opencode/agents/`, `.opencode/commands/`, and `.opencode/catalog.toml` before installing artifacts

### Requirement: Resolve source roots predictably
The system SHALL resolve skill, agent, and command source roots independently using a stable precedence order of CLI flags, then the user-level Hermes config file, then environment variables.

#### Scenario: CLI flags override user config and environment configuration
- **WHEN** a source root is provided by CLI flag, user config, and environment variable
- **THEN** the CLI SHALL use the CLI flag value

#### Scenario: Fall back to user config or environment configuration
- **WHEN** a source root is not provided by CLI flag
- **THEN** the CLI SHALL use the user config value if present
- **AND** otherwise use the corresponding environment variable if present

#### Scenario: Do not resolve source roots from the project catalog
- **WHEN** `.opencode/catalog.toml` contains `skills_source_root`, `agents_source_root`, or `commands_source_root`
- **AND** the corresponding CLI flag and user config value are absent
- **THEN** the CLI SHALL ignore the catalog value for source-root resolution
- **AND** continue to the corresponding environment variable

#### Scenario: Handle a missing source root by requested kind
- **WHEN** a command explicitly requests a kind with no resolvable source root
- **THEN** the CLI SHALL fail with a clear error for that kind

### Requirement: Record installed artifact state in a unified catalog
The system SHALL persist install mode and installed skills, agents, and commands in `.opencode/catalog.toml` with per-artifact source-relative paths, flat install paths, and hashes needed to manage them later, including grouped source paths for recursively discovered libraries, and SHALL not use the catalog as the persisted default source-root configuration store.

#### Scenario: Record installed skill metadata from a grouped source path
- **WHEN** a skill discovered from a grouped source path is installed
- **THEN** the catalog SHALL store its `name`, `description`, grouped `source_rel_path`, flat `installed_rel_path`, `source_hash`, and `installed_hash`

#### Scenario: Record installed agent metadata from a grouped source path
- **WHEN** an agent discovered from a grouped source path is installed
- **THEN** the catalog SHALL store its `name`, `description`, `mode`, grouped `source_rel_path`, flat `installed_rel_path`, `source_hash`, and `installed_hash`

#### Scenario: Record installed command metadata from a grouped source path
- **WHEN** a command discovered from a grouped source path is installed
- **THEN** the catalog SHALL store its `name`, optional `description`, grouped `source_rel_path`, flat `installed_rel_path`, `source_hash`, and `installed_hash`

#### Scenario: Keep default source roots out of the project catalog
- **WHEN** Hermes writes `.opencode/catalog.toml`
- **THEN** it SHALL omit top-level persisted default `skills_source_root`, `agents_source_root`, and `commands_source_root` fields

### Requirement: Protect locally modified installed artifacts
The system SHALL detect when an installed skill, agent, or command differs from the last recorded installed hash and SHALL not overwrite it silently.

#### Scenario: Skip overwrite of a modified local artifact
- **WHEN** an installed artifact's current hash differs from the catalog's recorded `installed_hash`
- **THEN** install and sync operations SHALL skip overwriting that artifact
- **AND** the CLI SHALL explain that the local copy was modified

#### Scenario: Force overwrite of a modified local artifact
- **WHEN** the user reruns `hermes install` or `hermes sync` with `--force`
- **THEN** the CLI MAY overwrite the modified local copy with the source version
- **AND** MUST update the catalog hashes accordingly

### Requirement: Synchronize managed artifacts from source
The system SHALL support a sync operation that updates installed artifacts when the source changes and the local installed copy has not been edited.

#### Scenario: Sync an unchanged local copy to a new source version
- **WHEN** the source hash changed and the installed artifact hash still matches the catalog's recorded `installed_hash`
- **THEN** the CLI SHALL reinstall the artifact from source
- **AND** update both source and installed hashes in the catalog

#### Scenario: Leave an up-to-date artifact unchanged
- **WHEN** the source and installed hashes still match the catalog
- **THEN** sync MUST leave the artifact unchanged

### Requirement: Remove and inspect managed artifacts
The system SHALL support removal and inspection of managed skills, agents, and commands.

#### Scenario: Remove a managed artifact
- **WHEN** the user removes a managed skill, agent, or command
- **THEN** the CLI SHALL delete its installed path
- **AND** remove its entry from the catalog

#### Scenario: List available and installed artifacts
- **WHEN** the user requests available or installed items
- **THEN** the CLI SHALL list skills, agents, or commands for the requested kind using source or catalog data respectively

#### Scenario: Validate the current workspace with doctor
- **WHEN** the user runs `hermes doctor`
- **THEN** the CLI SHALL verify source root existence, installed artifact validity, and catalog or file consistency for managed skills, agents, and commands
- **AND** report any detected problems
