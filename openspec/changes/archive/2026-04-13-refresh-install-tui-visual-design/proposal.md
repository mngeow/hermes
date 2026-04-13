## Why

The current Hermes install TUI is functional but visually flat, which makes it harder to scan and less aligned with the product identity. Improving the layout and presentation now will make interactive installation easier to navigate while giving the workflow a clearer Hermes-specific look and feel.

## What Changes

- Redesign the interactive install TUI so the selection panes are stacked vertically instead of shown side by side.
- Add a Hermes-branded ASCII art header to the install TUI.
- Introduce a darker visual theme with stronger use of color for focus state, headings, help text, and selection state.
- Clarify focused-pane styling so the active section is easier to identify at a glance.
- Update TUI-facing documentation to describe the revised visual layout and presentation.

## Capabilities

### New Capabilities
- None.

### Modified Capabilities
- `interactive-artifact-selection-ui`: Update the interactive install TUI requirements to cover vertically stacked panes, Hermes-branded ASCII presentation, and clearer color-driven focus styling.

## Impact

- Affected code: TUI rendering and layout logic in the interactive install flow.
- Affected files: likely `src/tui.rs` plus related docs such as `README.md` and `docs/rust-skill-installer-cli.md`.
- Dependencies: no new external dependencies are expected; changes should stay within the current `ratatui`-based implementation.
