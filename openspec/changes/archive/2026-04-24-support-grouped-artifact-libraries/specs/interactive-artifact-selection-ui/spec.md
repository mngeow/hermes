## MODIFIED Requirements

### Requirement: Provide a terminal UI for interactive artifact selection
The system SHALL provide a terminal UI for interactive install selection that lets users browse installable skills, agents, and commands without passing artifact names in command arguments, SHALL present grouped source folders as browseable nodes when a library contains nested folders, and SHALL present that UI with a Hermes-branded header and vertically stacked artifact panes.

#### Scenario: Open the install selection UI
- **WHEN** the user runs `hermes install` without explicit artifact names in an interactive terminal
- **THEN** the CLI SHALL open the install selection UI instead of prompting with simple sequential multi-select dialogs

#### Scenario: Show grouped artifact trees and metadata
- **WHEN** the install selection UI is displayed and one or more source roots contain nested folders
- **THEN** each corresponding pane SHALL show folder nodes and descendant installable artifact nodes
- **AND** leaf skill nodes SHALL show skill names and descriptions
- **AND** leaf agent nodes SHALL show agent names, descriptions, and modes when available
- **AND** leaf command nodes SHALL show command names and descriptions when available

#### Scenario: Render vertically stacked panes beneath Hermes branding
- **WHEN** the install selection UI is displayed
- **THEN** it SHALL render a Hermes-branded ASCII art or text-art header region
- **AND** it SHALL render the skills, agents, and commands panes in a vertically stacked layout beneath that header

### Requirement: Support keyboard-driven selection and confirmation
The system SHALL allow users to navigate, select, confirm, and cancel install choices entirely from the keyboard, SHALL let users select either individual artifact leaves or a folder node that resolves to all valid descendant artifacts, and SHALL use clear visual styling so the active pane and selected items remain easy to distinguish.

#### Scenario: Select an individual grouped artifact and confirm installation
- **WHEN** the user navigates to a leaf artifact within a grouped folder and confirms their choices
- **THEN** the CLI SHALL return that selected artifact to the install workflow
- **AND** proceed with installation using the existing install behavior for that artifact kind

#### Scenario: Select a folder subtree and confirm installation
- **WHEN** the user selects a folder node in the install selection UI and confirms their choices
- **THEN** the CLI SHALL treat every valid descendant artifact in that folder subtree as selected
- **AND** proceed with installation for those descendant artifacts only

#### Scenario: Cancel interactive selection
- **WHEN** the user cancels from the install selection UI
- **THEN** the CLI SHALL exit the interactive install flow without writing changes to `.opencode/`

#### Scenario: Highlight the active pane in a dark themed screen
- **WHEN** the install selection UI is displayed
- **THEN** it SHALL use a darker overall visual theme than the default plain list presentation
- **AND** it SHALL use stronger title, border, highlight, or accent styling to indicate which pane currently has focus
- **AND** it SHALL preserve non-color cues for both artifact and folder selection state so selected content remains identifiable even with limited terminal color support
