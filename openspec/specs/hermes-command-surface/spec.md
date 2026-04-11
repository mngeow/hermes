## Purpose

Define the first end-user `hermes` CLI command surface for initializing a workspace and managing installed OpenCode artifacts.

## Requirements

### Requirement: Provide the first Hermes command surface
The system SHALL provide a `hermes` CLI with `init`, `install`, `list`, `sync`, `remove`, and `doctor` commands for managing local OpenCode artifacts.

#### Scenario: Initialize a project workspace
- **WHEN** the user runs `hermes init`
- **THEN** the CLI SHALL create the local `.opencode` workspace described by the artifact catalog management capability

#### Scenario: Install selected artifacts
- **WHEN** the user runs `hermes install`
- **THEN** the CLI SHALL install the requested skills and agents using the skill and agent installation capabilities

#### Scenario: List artifacts
- **WHEN** the user runs `hermes list`
- **THEN** the CLI SHALL display available or installed artifacts for the requested kind

#### Scenario: Synchronize artifacts
- **WHEN** the user runs `hermes sync`
- **THEN** the CLI SHALL reconcile installed artifacts against their configured source roots

#### Scenario: Remove an installed artifact
- **WHEN** the user runs `hermes remove`
- **THEN** the CLI SHALL remove the requested installed skill or agent from the local workspace

#### Scenario: Validate the local workspace
- **WHEN** the user runs `hermes doctor`
- **THEN** the CLI SHALL validate the local workspace and report any detected problems

### Requirement: Support explicit and interactive install selection
The system SHALL support both explicit artifact selection through command arguments and interactive artifact selection when names are omitted.

#### Scenario: Install explicit skills and agents with flags
- **WHEN** the user runs `hermes install --skills code-review --agents review`
- **THEN** the CLI SHALL install the named skill and agent without prompting

#### Scenario: Install skills through the kind-specific subcommand form
- **WHEN** the user runs `hermes install skills code-review python-testing`
- **THEN** the CLI SHALL install only the named skills without prompting for agents

#### Scenario: Install agents through the kind-specific subcommand form
- **WHEN** the user runs `hermes install agents review`
- **THEN** the CLI SHALL install only the named agents without prompting for skills

#### Scenario: Prompt when no names are provided
- **WHEN** the user runs `hermes install` without explicit artifact names
- **THEN** the CLI SHALL prompt for installable skills and agents when their source roots are available
