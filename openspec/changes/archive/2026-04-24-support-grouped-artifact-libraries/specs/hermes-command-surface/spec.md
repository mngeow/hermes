## MODIFIED Requirements

### Requirement: Support explicit and interactive install selection
The system SHALL support both explicit artifact selection through command arguments and interactive artifact selection through a terminal UI when names are omitted, including grouped source libraries whose nested artifacts remain selectable by their unique install names.

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

#### Scenario: Reject ambiguous explicit artifact names from grouped libraries
- **WHEN** the requested source library contains multiple grouped artifacts of the same kind that would install under the same artifact name and the user requests that name explicitly
- **THEN** the CLI SHALL fail with a clear ambiguity error for that kind
- **AND** tell the user to rename or regroup the colliding source artifacts
