## MODIFIED Requirements

### Requirement: Support explicit and interactive install selection
The system SHALL support both explicit artifact selection through command arguments and interactive artifact selection through a terminal UI when names are omitted.

#### Scenario: Install explicit skills and agents with flags
- **WHEN** the user runs `hermes install --skills code-review --agents review`
- **THEN** the CLI SHALL install the named skill and agent without opening the interactive selection UI

#### Scenario: Install skills through the kind-specific subcommand form
- **WHEN** the user runs `hermes install skills code-review python-testing`
- **THEN** the CLI SHALL install only the named skills without opening the interactive selection UI for agents

#### Scenario: Install agents through the kind-specific subcommand form
- **WHEN** the user runs `hermes install agents review`
- **THEN** the CLI SHALL install only the named agents without opening the interactive selection UI for skills

#### Scenario: Launch terminal UI when no names are provided
- **WHEN** the user runs `hermes install` without explicit artifact names in an interactive terminal session
- **THEN** the CLI SHALL launch the interactive install selection UI for available skills and agents

#### Scenario: Fail clearly without a terminal session
- **WHEN** the user runs `hermes install` without explicit artifact names in a non-interactive terminal session
- **THEN** the CLI SHALL fail with a clear error that explains interactive selection requires a terminal or explicit artifact names
