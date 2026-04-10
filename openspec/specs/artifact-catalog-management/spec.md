## Purpose

Define how `hermes` initializes the local `.opencode` workspace, records installed artifacts in a unified catalog, resolves source roots, and safely manages sync, removal, and validation.

## Requirements

### Requirement: Initialize a project-local OpenCode artifact workspace
The system SHALL create a project-local `.opencode` workspace with `skills/`, `agents/`, and `catalog.toml` when initializing the installer in a project.

#### Scenario: Initialize the local workspace
- **WHEN** the user runs `hermes init` with at least one source root
- **THEN** the CLI SHALL create `.opencode/`, `.opencode/skills/`, `.opencode/agents/`, and `.opencode/catalog.toml`
- **AND** record the install mode and any provided source roots

#### Scenario: Require at least one source root
- **WHEN** `hermes init` is invoked without a skills source root and without an agents source root
- **THEN** the CLI SHALL fail with a clear error

### Requirement: Resolve source roots predictably
The system SHALL resolve skill and agent source roots independently using a stable precedence order of CLI flags, then catalog configuration, then environment variables.

#### Scenario: CLI flags override persisted and environment configuration
- **WHEN** a source root is provided by CLI flag, catalog configuration, and environment variable
- **THEN** the CLI SHALL use the CLI flag value

#### Scenario: Fall back to persisted or environment configuration
- **WHEN** a source root is not provided by CLI flag
- **THEN** the CLI SHALL use the catalog value if present
- **AND** otherwise use the corresponding environment variable if present

#### Scenario: Handle a missing source root by requested kind
- **WHEN** a command explicitly requests a kind with no resolvable source root
- **THEN** the CLI SHALL fail with a clear error for that kind

### Requirement: Record installed artifact state in a unified catalog
The system SHALL persist installed skills and agents in `.opencode/catalog.toml` with source paths, install paths, and hashes needed to manage them later.

#### Scenario: Record installed skill metadata
- **WHEN** a skill is installed
- **THEN** the catalog SHALL store its `name`, `description`, `source_rel_path`, `installed_rel_path`, `source_hash`, and `installed_hash`

#### Scenario: Record installed agent metadata
- **WHEN** an agent is installed
- **THEN** the catalog SHALL store its `name`, `description`, `mode`, `source_rel_path`, `installed_rel_path`, `source_hash`, and `installed_hash`

### Requirement: Protect locally modified installed artifacts
The system SHALL detect when an installed skill or agent differs from the last recorded installed hash and SHALL not overwrite it silently.

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
The system SHALL support removal and inspection of managed artifacts.

#### Scenario: Remove a managed artifact
- **WHEN** the user removes a managed skill or agent
- **THEN** the CLI SHALL delete its installed path
- **AND** remove its entry from the catalog

#### Scenario: List available and installed artifacts
- **WHEN** the user requests available or installed items
- **THEN** the CLI SHALL list artifacts for the requested kind using source or catalog data respectively

#### Scenario: Validate the current workspace with doctor
- **WHEN** the user runs `hermes doctor`
- **THEN** the CLI SHALL verify source root existence, installed artifact validity, and catalog or file consistency
- **AND** report any detected problems
