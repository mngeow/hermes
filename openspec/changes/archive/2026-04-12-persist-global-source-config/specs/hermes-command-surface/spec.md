## MODIFIED Requirements

### Requirement: Provide the first Hermes command surface
The system SHALL provide a `hermes` CLI with `init`, `configure`, `install`, `list`, `sync`, `remove`, and `doctor` commands for managing local OpenCode artifacts.

#### Scenario: Initialize a project workspace
- **WHEN** the user runs `hermes init`
- **THEN** the CLI SHALL create the local `.opencode` workspace described by the artifact catalog management capability

#### Scenario: Configure default source roots
- **WHEN** the user runs `hermes configure`
- **THEN** the CLI SHALL update the user-level source-root configuration described by the user source config capability

#### Scenario: Install selected artifacts
- **WHEN** the user runs `hermes install`
- **THEN** the CLI SHALL install the requested skills and agents using the skill and agent installation capabilities

#### Scenario: Install selected artifacts in a fresh project
- **WHEN** the user runs `hermes install` in a project without an existing Hermes workspace
- **AND** source roots are resolvable for the requested artifact kinds
- **THEN** the CLI SHALL create the local workspace described by the artifact catalog management capability
- **AND** continue installing the requested artifacts

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
