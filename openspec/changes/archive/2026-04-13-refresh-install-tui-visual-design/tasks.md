## 1. Layout and branding

- [x] 1.1 Update the interactive install TUI layout to place the Hermes banner at the top and stack the skills, agents, and commands panes vertically beneath it.
- [x] 1.2 Add a Hermes-branded ASCII art or text-art header that fits within the install screen without obscuring artifact lists on typical terminal sizes.

## 2. Visual styling

- [x] 2.1 Introduce a darker overall theme for the install TUI, including pane, list, and help-region styling.
- [x] 2.2 Strengthen focused-pane styling so the active section is clearly distinguished through title, border, highlight, or accent treatment.
- [x] 2.3 Preserve non-color selection cues so selected items remain recognizable in terminals with limited color support.

## 3. Documentation and verification

- [x] 3.1 Update TUI-facing documentation to describe the vertically stacked layout, Hermes branding, and revised visual styling.
- [x] 3.2 Run `cargo build` and confirm the build passes.
