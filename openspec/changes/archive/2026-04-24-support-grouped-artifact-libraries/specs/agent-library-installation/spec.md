## MODIFIED Requirements

### Requirement: Discover installable agents from a configured source library
The system SHALL recursively scan the configured agents source root for descendant markdown files and only expose a file as installable when it contains YAML frontmatter with a required `description` field and its filename stem is unique across the full source tree.

#### Scenario: List valid agent files from grouped folders
- **WHEN** the configured agents source root contains valid nested agent files such as `review/review.md` and `quality/security-auditor.md`
- **THEN** the CLI SHALL list them as available agents using each file stem as the agent name
- **AND** surface the `description` and `mode`, if present

#### Scenario: Skip grouping-only folders
- **WHEN** the configured agents source root contains intermediate folders used only to group descendant agent files
- **THEN** the CLI SHALL treat those folders as grouping containers instead of installable agents
- **AND** SHALL NOT report those grouping folders as invalid by themselves

#### Scenario: Reject invalid nested agent files
- **WHEN** a descendant agent markdown file is missing frontmatter, missing `description`, or has an invalid `mode`
- **THEN** the CLI SHALL exclude it from installation choices
- **AND** the `doctor` command SHALL report the validation problem

#### Scenario: Reject duplicate agent names across groups
- **WHEN** two valid nested agent files share the same filename stem
- **THEN** the CLI SHALL exclude that duplicate agent name from installation choices
- **AND** the `doctor` command SHALL report the ambiguity

### Requirement: Install standalone project-local OpenCode agents
The system SHALL install selected agents as markdown files in `<project>/.opencode/agents/<agent-name>.md` without rewriting their contents even when the source files were discovered in grouped folders.

#### Scenario: Install named grouped agents non-interactively
- **WHEN** the user runs `hermes install` with an explicit agent name that resolves to a uniquely named nested agent file
- **THEN** the CLI SHALL copy that agent into `.opencode/agents/<agent-name>.md`
- **AND** preserve the full frontmatter and prompt body verbatim

#### Scenario: Select grouped agents interactively
- **WHEN** the user runs `hermes install` without explicit agent names and a grouped agents source root is configured
- **THEN** the CLI SHALL let the user browse grouped agent folders through the interactive install selection UI
- **AND** install only the selected descendant agents
