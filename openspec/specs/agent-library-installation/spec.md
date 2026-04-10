## Purpose

Define how `hermes` discovers, validates, and installs reusable OpenCode markdown agents into a project-local `.opencode/agents/` directory.

## Requirements

### Requirement: Discover installable agents from a configured source library
The system SHALL treat direct child markdown files of the configured agents source root as candidate agents and only expose a file as installable when it contains YAML frontmatter with a required `description` field.

#### Scenario: List valid agent files
- **WHEN** the configured agents source root contains files such as `review.md` and `security-auditor.md` with valid frontmatter
- **THEN** the CLI SHALL list them as available agents using the file stem as the agent name
- **AND** surface the `description` and `mode`, if present

#### Scenario: Reject invalid agent files
- **WHEN** a candidate agent file is missing frontmatter, missing `description`, or has an invalid `mode`
- **THEN** the CLI SHALL exclude it from installation choices
- **AND** the `doctor` command SHALL report the validation problem

### Requirement: Install standalone project-local OpenCode agents
The system SHALL install selected agents as markdown files in `<project>/.opencode/agents/<agent-name>.md` without rewriting their contents.

#### Scenario: Install named agents non-interactively
- **WHEN** the user runs `hermes install` with explicit agent names
- **THEN** the CLI SHALL copy each selected agent markdown file into `.opencode/agents/<agent-name>.md`
- **AND** preserve the full frontmatter and prompt body verbatim

#### Scenario: Select agents interactively
- **WHEN** the user runs `hermes install` without explicit agent names and an agents source root is configured
- **THEN** the CLI SHALL present an interactive multi-select list of available agents
- **AND** install only the selected agents

### Requirement: Support standalone markdown agents in v1
The system SHALL support single-file OpenCode agents whose prompt body is contained in the same markdown file and SHALL reject unsupported multi-file patterns in v1.

#### Scenario: Accept a standalone markdown agent
- **WHEN** an agent file contains its prompt body directly after YAML frontmatter
- **THEN** the CLI SHALL treat it as installable

#### Scenario: Reject an external prompt reference in v1
- **WHEN** an agent frontmatter contains `prompt: {file:...}`
- **THEN** the CLI SHALL reject that agent for installation
- **AND** the `doctor` command SHALL report that multi-file agents are unsupported in v1
