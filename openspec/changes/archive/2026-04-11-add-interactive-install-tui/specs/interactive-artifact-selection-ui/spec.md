## ADDED Requirements

### Requirement: Provide a terminal UI for interactive artifact selection
The system SHALL provide a terminal UI for interactive install selection that lets users browse installable skills and agents without passing artifact names in command arguments.

#### Scenario: Open the install selection UI
- **WHEN** the user runs `hermes install` without explicit artifact names in an interactive terminal
- **THEN** the CLI SHALL open the install selection UI instead of prompting with simple sequential multi-select dialogs

#### Scenario: Show installable artifact metadata
- **WHEN** the install selection UI is displayed
- **THEN** it SHALL show installable skill names and descriptions
- **AND** it SHALL show installable agent names, descriptions, and modes when available

### Requirement: Support keyboard-driven selection and confirmation
The system SHALL allow users to navigate, select, confirm, and cancel install choices entirely from the keyboard.

#### Scenario: Select artifacts and confirm installation
- **WHEN** the user navigates the install selection UI and confirms their choices
- **THEN** the CLI SHALL return the selected skills and agents to the install workflow
- **AND** proceed with installation using the existing install behavior for those artifacts

#### Scenario: Cancel interactive selection
- **WHEN** the user cancels from the install selection UI
- **THEN** the CLI SHALL exit the interactive install flow without writing changes to `.opencode/`

### Requirement: Handle unavailable interactive terminal sessions clearly
The system SHALL fail clearly when interactive selection is requested implicitly but no usable terminal UI session can be opened.

#### Scenario: No TTY is available for interactive install
- **WHEN** the user runs `hermes install` without explicit artifact names in a non-interactive terminal session
- **THEN** the CLI SHALL exit with a clear error explaining that interactive selection requires a terminal
- **AND** tell the user to rerun in a TTY or provide explicit artifact names
