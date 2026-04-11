## Why

Hermes currently supports interactive selection with prompt-style multi-selects, but the install experience is still split and lightweight rather than a real terminal UI. As the shared `repo/` and `agents/` libraries grow, a richer full-screen selection flow will make it easier to browse, compare, filter, and choose artifacts without pre-specifying names in command arguments.

## What Changes

- Add a full-screen interactive terminal UI for `hermes install` when the user does not provide explicit skill or agent names.
- Introduce a dedicated install-selection UI that can browse both skills and agents in one interaction and surface names, descriptions, and agent modes clearly.
- Keep explicit argument-driven install flows available for scripting and non-interactive environments.
- Replace the current prompt-only selection approach with a TUI-oriented implementation built on a Rust library suited for richer terminal interfaces.
- Update implementation docs and tests to reflect the new TUI-first interactive install workflow.

## Capabilities

### New Capabilities
- `interactive-artifact-selection-ui`: Define the keyboard-driven terminal UI used to browse and choose skills and agents for installation.

### Modified Capabilities
- `hermes-command-surface`: Change the default interactive install behavior from prompt-style multi-selects to a richer terminal UI while preserving explicit non-interactive install arguments.

## Impact

- Affects the `hermes install` user experience and the install-selection implementation in the Rust CLI.
- Adds a terminal UI dependency and supporting event-loop/state-management code.
- Requires command-surface and UX documentation updates.
- Requires validation of interactive and non-interactive install flows.
