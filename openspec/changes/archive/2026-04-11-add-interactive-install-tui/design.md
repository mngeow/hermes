## Context

Hermes already supports interactive install behavior, but the current implementation uses `dialoguer` multi-select prompts in two separate steps: one for skills and one for agents. That works for small libraries, but it becomes limiting as the number of artifacts grows because it does not provide a richer layout, clearer comparison space, or a unified terminal-driven browsing experience.

I reviewed several Rust libraries for this change:

- `ratatui`: a dedicated Rust library for building fast, lightweight, rich terminal user interfaces. Its site emphasizes full TUI layouts, widgets, and broad real-world use.
- `dialoguer`: a command-line prompting library focused on small dialogs like confirm, select, multi-select, and fuzzy select prompts.
- `inquire`: an interactive terminal prompt library with configurable select and multi-select prompts, still centered on prompt widgets rather than full application layouts.
- `cursive`: a higher-level TUI library focused on ease-of-use and view-driven terminal applications.

For Hermes, the goal is not just better prompts. It is a fuller install-selection interface that can present skills and agents together, navigate quickly, and evolve into a more capable selection workflow over time.

## Goals / Non-Goals

**Goals:**

- Make `hermes install` open a terminal UI when no explicit artifact names are provided.
- Let users browse both skills and agents interactively without passing names in command arguments.
- Surface key metadata in the UI, including skill descriptions and agent modes.
- Keep explicit `--skills`, `--agents`, and kind-specific install commands working for scripting and automation.
- Choose a Rust terminal UI library that supports richer layouts than prompt-only libraries.

**Non-Goals:**

- Convert the entire Hermes CLI into a TUI application.
- Remove explicit non-interactive install arguments.
- Add link-mode installs, publishing, or remote registries.
- Redesign non-install commands like `list`, `sync`, `remove`, or `doctor` as full-screen UIs.

## Decisions

### Use `ratatui` for the install selection UI

`ratatui` is the best fit for this change because it is specifically built for rich terminal user interfaces rather than just isolated prompts. Compared with `dialoguer` and `inquire`, it gives Hermes the layout flexibility needed for a real selector screen. Compared with `cursive`, it is a stronger fit for a modern, custom install-flow UI where Hermes owns the rendering model rather than composing a higher-level form toolkit.

Alternatives considered:

- `dialoguer`: already in use and easy to keep, but it remains a prompt library rather than a full-screen UI framework.
- `inquire`: more configurable than a basic prompt library and includes good select widgets, but it is still prompt-first rather than layout-first.
- `cursive`: viable for traditional TUI apps, but less aligned with a custom, visually structured selection screen that Hermes can grow over time.

### Keep explicit CLI arguments as the non-interactive path

The TUI should become the default install experience when names are omitted, but explicit `--skills`, `--agents`, `install skills ...`, and `install agents ...` flows should remain intact. This preserves scriptability and avoids breaking automation.

Alternative considered:

- Remove explicit name-based selection entirely. Rejected because Hermes still needs a stable automation path for CI, shell scripts, and non-TTY usage.

### Build a unified selection screen for both skills and agents

The TUI should let the user browse both kinds of installable artifacts in one place instead of forcing two disconnected prompts. The interface can still organize skills and agents as separate panes or tabs, but the overall flow should feel like one install session.

Alternative considered:

- Replace each existing prompt with a slightly nicer single-kind prompt. Rejected because it improves polish without really changing the fragmented selection workflow.

### Fall back cleanly in non-interactive environments

When Hermes cannot open a terminal UI, it should fail clearly if the user omitted explicit names, telling them to either run the command in a TTY or use explicit artifact arguments.

Alternative considered:

- Try to silently degrade to the current prompt behavior. Rejected because prompt libraries still depend on terminal interaction and do not solve non-TTY execution.

## Risks / Trade-offs

- [A richer TUI adds implementation complexity] → Keep the first version focused on navigation, toggling, confirm, cancel, and simple filtering instead of over-designing the UI.
- [Terminal behavior varies across environments] → Use a well-supported backend and keep a clear non-TTY fallback path.
- [A new dependency increases maintenance surface] → Use a library with strong adoption and documentation, and isolate the TUI behind a small module boundary.
- [Testing interactive UI logic is harder than prompt logic] → Keep selection state and install-plan generation separate from rendering so most behavior can still be validated without a live terminal.

## Migration Plan

- Add the new TUI dependency and implement the install selection screen behind the existing `hermes install` flow.
- Keep existing explicit argument forms working unchanged.
- Update docs to describe TUI-first interactive selection and the non-TTY fallback behavior.
- Verify the change with `cargo build` and targeted install-flow checks.

## Open Questions

- Should the first TUI version include inline search/filtering immediately, or land first with navigation and selection only?
- Should skills and agents appear as separate panes, separate tabs, or one combined list with type labels?
