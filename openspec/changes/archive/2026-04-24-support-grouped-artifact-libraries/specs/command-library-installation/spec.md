## MODIFIED Requirements

### Requirement: Discover installable commands from a configured source library
The system SHALL recursively scan the configured commands source root for descendant markdown files and only expose a file as installable when it contains a non-empty markdown template body, any YAML frontmatter present parses successfully, and its filename stem is unique across the full source tree.

#### Scenario: List valid commands from grouped folders
- **WHEN** the configured commands source root contains valid nested command files such as `git/review-changes.md` and `project/test.md`
- **THEN** the CLI SHALL list them as available commands using each file stem as the command name
- **AND** surface the `description` when present

#### Scenario: Skip grouping-only folders
- **WHEN** the configured commands source root contains intermediate folders used only to group descendant command files
- **THEN** the CLI SHALL treat those folders as grouping containers instead of installable commands
- **AND** SHALL NOT report those grouping folders as invalid by themselves

#### Scenario: Reject invalid nested command files
- **WHEN** a descendant command file has invalid YAML frontmatter or an empty template body
- **THEN** the CLI SHALL exclude it from installation choices
- **AND** the `doctor` command SHALL report the validation problem

#### Scenario: Reject duplicate command names across groups
- **WHEN** two valid nested command files share the same filename stem
- **THEN** the CLI SHALL exclude that duplicate command name from installation choices
- **AND** the `doctor` command SHALL report the ambiguity

### Requirement: Install standalone project-local OpenCode commands
The system SHALL install selected commands as markdown files in `<project>/.opencode/commands/<command-name>.md` without rewriting their contents even when the source files were discovered in grouped folders.

#### Scenario: Install named grouped commands non-interactively
- **WHEN** the user runs `hermes install` with an explicit command name that resolves to a uniquely named nested command file
- **THEN** the CLI SHALL copy that command into `.opencode/commands/<command-name>.md`
- **AND** the CLI SHALL report which commands were installed

#### Scenario: Select grouped commands interactively
- **WHEN** the user runs `hermes install` without explicit command names and a grouped commands source root is configured
- **THEN** the CLI SHALL let the user select available commands through the interactive install selection UI
- **AND** install only the selected descendant commands
