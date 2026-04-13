## ADDED Requirements

### Requirement: Discover installable commands from a configured source library
The system SHALL treat direct child markdown files of the configured commands source root as candidate commands and only expose a file as installable when it contains a non-empty markdown template body and any YAML frontmatter present parses successfully.

#### Scenario: List valid commands from the source library
- **WHEN** the configured commands source root contains valid command files such as `test.md` and `review-changes.md`
- **THEN** the CLI SHALL list them as available commands using the file stem as the command name
- **AND** surface the `description` when present

#### Scenario: Reject invalid command files
- **WHEN** a candidate command file has invalid YAML frontmatter or an empty template body
- **THEN** the CLI SHALL exclude it from installation choices
- **AND** the `doctor` command SHALL report the validation problem

### Requirement: Install standalone project-local OpenCode commands
The system SHALL install selected commands as markdown files in `<project>/.opencode/commands/<command-name>.md` without rewriting their contents.

#### Scenario: Install named commands non-interactively
- **WHEN** the user runs `hermes install` with explicit command names
- **THEN** the CLI SHALL copy each selected command markdown file into `.opencode/commands/<command-name>.md`
- **AND** the CLI SHALL report which commands were installed

#### Scenario: Select commands interactively
- **WHEN** the user runs `hermes install` without explicit command names and a commands source root is configured
- **THEN** the CLI SHALL let the user select available commands through the interactive install selection UI
- **AND** install only the selected commands

### Requirement: Preserve command prompt configuration during installation
The system SHALL preserve the full command markdown file verbatim so OpenCode can evaluate the installed template, placeholders, and frontmatter exactly as authored.

#### Scenario: Preserve frontmatter and template body
- **WHEN** a selected command file contains optional frontmatter such as `description`, `agent`, `subtask`, or `model` followed by a markdown template body
- **THEN** the CLI SHALL copy the full file into the installed path without rewriting its contents

#### Scenario: Preserve filename-derived command names
- **WHEN** the source command file is named `review-changes.md`
- **THEN** the installed copy SHALL keep the filename `review-changes.md`
- **AND** Hermes SHALL preserve the command name derived from that filename
