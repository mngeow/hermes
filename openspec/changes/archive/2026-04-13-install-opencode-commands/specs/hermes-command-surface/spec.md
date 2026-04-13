## MODIFIED Requirements

### Requirement: Provide the first Hermes command surface
The system SHALL provide a `hermes` CLI with `init`, `configure`, `install`, `list`, `sync`, `remove`, and `doctor` commands for managing local OpenCode skills, agents, and commands.

#### Scenario: Initialize a project workspace
- **WHEN** the user runs `hermes init`
- **THEN** the CLI SHALL create the local `.opencode` workspace described by the artifact catalog management capability

#### Scenario: Configure default source roots
- **WHEN** the user runs `hermes configure`
- **THEN** the CLI SHALL update the user-level source-root configuration described by the user source config capability

#### Scenario: Install selected artifacts
- **WHEN** the user runs `hermes install`
- **THEN** the CLI SHALL install the requested skills, agents, and commands using the skill, agent, and command installation capabilities

#### Scenario: Install selected artifacts in a fresh project
- **WHEN** the user runs `hermes install` in a project without an existing Hermes workspace
- **AND** source roots are resolvable for the requested artifact kinds
- **THEN** the CLI SHALL create the local workspace described by the artifact catalog management capability
- **AND** continue installing the requested artifacts

#### Scenario: List artifacts
- **WHEN** the user runs `hermes list`
- **THEN** the CLI SHALL display available or installed skills, agents, or commands for the requested kind

#### Scenario: Synchronize artifacts
- **WHEN** the user runs `hermes sync`
- **THEN** the CLI SHALL reconcile installed skills, agents, and commands against their configured source roots

#### Scenario: Remove an installed artifact
- **WHEN** the user runs `hermes remove`
- **THEN** the CLI SHALL remove the requested installed skill, agent, or command from the local workspace

#### Scenario: Validate the local workspace
- **WHEN** the user runs `hermes doctor`
- **THEN** the CLI SHALL validate the local workspace and report any detected problems across managed skills, agents, and commands

### Requirement: Support explicit and interactive install selection
The system SHALL support both explicit artifact selection through command arguments and interactive artifact selection through a terminal UI when names are omitted, including commands as a first-class install kind.

#### Scenario: Install explicit skills, agents, and commands with flags
- **WHEN** the user runs `hermes install --skills code-review --agents review --commands test`
- **THEN** the CLI SHALL install the named skill, agent, and command without opening the interactive selection UI

#### Scenario: Install skills through the kind-specific subcommand form
- **WHEN** the user runs `hermes install skills code-review python-testing`
- **THEN** the CLI SHALL install only the named skills without opening the interactive selection UI for agents or commands

#### Scenario: Install agents through the kind-specific subcommand form
- **WHEN** the user runs `hermes install agents review`
- **THEN** the CLI SHALL install only the named agents without opening the interactive selection UI for skills or commands

#### Scenario: Install commands through the kind-specific subcommand form
- **WHEN** the user runs `hermes install commands test review-changes`
- **THEN** the CLI SHALL install only the named commands without opening the interactive selection UI for skills or agents

#### Scenario: Launch terminal UI when no names are provided
- **WHEN** the user runs `hermes install` without explicit artifact names in an interactive terminal session
- **THEN** the CLI SHALL launch the interactive install selection UI for available skills, agents, and commands

#### Scenario: Fail clearly without a terminal session
- **WHEN** the user runs `hermes install` without explicit artifact names in a non-interactive terminal session
- **THEN** the CLI SHALL fail with a clear error that explains interactive selection requires a terminal or explicit artifact names
