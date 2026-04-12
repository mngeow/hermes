## MODIFIED Requirements

### Requirement: Initialize a project-local OpenCode artifact workspace
The system SHALL create a project-local `.opencode` workspace with `skills/`, `agents/`, and `catalog.toml` when initializing the installer in a project and when an install operation needs to bootstrap a new project.

#### Scenario: Initialize the local workspace
- **WHEN** the user runs `hermes init` with at least one resolvable source root
- **THEN** the CLI SHALL create `.opencode/`, `.opencode/skills/`, `.opencode/agents/`, and `.opencode/catalog.toml`
- **AND** record the install mode

#### Scenario: Require at least one resolvable source root
- **WHEN** `hermes init` is invoked without a skills source root and without an agents source root after considering CLI flags, user config, and environment variables
- **THEN** the CLI SHALL fail with a clear error

#### Scenario: Bootstrap the local workspace during install
- **WHEN** the user runs `hermes install` in a project without `.opencode/catalog.toml`
- **AND** at least one requested kind has a resolvable source root
- **THEN** the CLI SHALL create `.opencode/`, `.opencode/skills/`, `.opencode/agents/`, and `.opencode/catalog.toml` before installing artifacts

### Requirement: Resolve source roots predictably
The system SHALL resolve skill and agent source roots independently using a stable precedence order of CLI flags, then the user-level Hermes config file, then environment variables.

#### Scenario: CLI flags override user config and environment configuration
- **WHEN** a source root is provided by CLI flag, user config, and environment variable
- **THEN** the CLI SHALL use the CLI flag value

#### Scenario: Fall back to user config or environment configuration
- **WHEN** a source root is not provided by CLI flag
- **THEN** the CLI SHALL use the user config value if present
- **AND** otherwise use the corresponding environment variable if present

#### Scenario: Do not resolve source roots from the project catalog
- **WHEN** `.opencode/catalog.toml` contains `skills_source_root` or `agents_source_root`
- **AND** the corresponding CLI flag and user config value are absent
- **THEN** the CLI SHALL ignore the catalog value for source-root resolution
- **AND** continue to the corresponding environment variable

#### Scenario: Handle a missing source root by requested kind
- **WHEN** a command explicitly requests a kind with no resolvable source root
- **THEN** the CLI SHALL fail with a clear error for that kind

### Requirement: Record installed artifact state in a unified catalog
The system SHALL persist install mode and installed skills and agents in `.opencode/catalog.toml` with per-artifact source paths, install paths, and hashes needed to manage them later, and SHALL not use the catalog as the persisted default source-root configuration store.

#### Scenario: Record installed skill metadata
- **WHEN** a skill is installed
- **THEN** the catalog SHALL store its `name`, `description`, `source_rel_path`, `installed_rel_path`, `source_hash`, and `installed_hash`

#### Scenario: Record installed agent metadata
- **WHEN** an agent is installed
- **THEN** the catalog SHALL store its `name`, `description`, `mode`, `source_rel_path`, `installed_rel_path`, `source_hash`, and `installed_hash`

#### Scenario: Keep default source roots out of the project catalog
- **WHEN** Hermes writes `.opencode/catalog.toml`
- **THEN** it SHALL omit top-level persisted default `skills_source_root` and `agents_source_root` fields
