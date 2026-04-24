# hermes

<img src="assets/hermes-logo.svg" alt="Hermes logo concept" width="180" />

A first-pass logo concept inspired by a public-domain 1880 Hermes illustration. See `assets/README.md` for source details.

This repository contains the `hermes` CLI, reusable OpenCode skills, reusable OpenCode agents, and the design and specification docs for the project.

`hermes` is a CLI for installing selected shared artifacts into a project-local `.opencode/` directory.

## Why Hermes

`hermes` is named after the Greek messenger god associated with travel, movement, boundaries, and exchange.

That fits this CLI because it moves curated artifacts across a boundary:

- from the shared `repo/` skill library into `.opencode/skills/`
- from the shared `agents/` library into `.opencode/agents/`
- from the shared `commands/` library into `.opencode/commands/`

It is effectively the delivery and transport layer between a central reusable library and a project's local OpenCode workspace.

## What Hermes Does

- discovers installable skills from `repo/` recursively, including grouped subfolders
- discovers installable agents from `agents/` recursively, including grouped subfolders
- discovers installable commands from `commands/` recursively, including grouped subfolders
- persists default source roots in `~/.config/hermes_cli/config.toml`
- lets a project choose which skills, agents, and commands to install locally
- copies them into the local `.opencode/` directory
- records project-local installed state in `.opencode/catalog.toml`
- supports follow-up management operations like `list`, `sync`, `remove`, and `doctor`

## Build And Run

Build the CLI locally with:

```bash
cargo build
```

Configure default source roots once with:

```bash
cargo run -- configure --skills-source ./repo --agents-source ./agents --commands-source ./commands
```

This writes `~/.config/hermes_cli/config.toml`, which Hermes uses as the default source-root config for later commands.

From a target project directory, install artifacts with:

```bash
hermes install --skills code-review --agents review --commands test
```

`hermes install` can bootstrap `.opencode/` in a fresh project as long as source roots are resolvable. Use `hermes init` only if you want to create the local workspace before installing anything.

## Configuration Model

Hermes resolves source roots in this order:

- CLI flags
- `~/.config/hermes_cli/config.toml`
- `OPENCODE_SKILLS_SOURCE`, `OPENCODE_AGENTS_SOURCE`, and `OPENCODE_COMMANDS_SOURCE`

The user-level config file stores reusable `skills_source_root`, `agents_source_root`, and `commands_source_root` defaults. The project-local `.opencode/catalog.toml` tracks installed artifacts and local workspace state.

## Grouped Source Libraries

Hermes supports organizing skills, agents, and commands into subfolders inside their respective source roots. Discovery walks the source tree recursively:

- **Skills**: any descendant directory containing a top-level `SKILL.md` is treated as an installable skill. Intermediate directories without `SKILL.md` are grouping folders and are ignored.
- **Agents**: any descendant `.md` file with valid frontmatter is treated as an installable agent.
- **Commands**: any descendant `.md` file with a non-empty template body is treated as an installable command.

Installed artifacts are always copied into the flat `.opencode/skills/`, `.opencode/agents/`, and `.opencode/commands/` destinations. The catalog records the nested `source_rel_path` so `sync`, `remove`, and `doctor` continue to work correctly.

### Duplicate-Name Constraints

Because install destinations remain flat, artifact names must be unique within each kind. If recursive discovery finds two skills (or agents, or commands) that would install under the same name, Hermes excludes them from install choices and reports the ambiguity in inspection, `doctor`, and explicit install attempts. Resolve collisions by renaming or regrouping the source artifacts.

## Interactive Mode

`hermes` can also boot into an interactive terminal UI for install selection.

After you have configured default source roots, run `hermes install` with no explicit artifact names to open the TUI:

```bash
# configure once
hermes configure --skills-source /path/to/hermes/repo --agents-source /path/to/hermes/agents --commands-source /path/to/hermes/commands

# then inside a target project directory
hermes install
```

If you are running directly from this repository instead of an installed binary, use:

```bash
cargo run -- configure --skills-source ./repo --agents-source ./agents --commands-source ./commands
cargo run -- install
```

The TUI lets you browse skills, agents, and commands in one screen. When source libraries contain nested folders, the TUI displays them as a tree:

- Use `Tab` to switch focus between panes.
- Use `Up` and `Down` or `j` and `k` to navigate.
- Use `Space` or `Enter` to toggle selection of a leaf artifact or an entire folder subtree.
- Use `c` to confirm and `q` or `Esc` to cancel.

Folder nodes show `[-]` when some but not all descendants are selected, `[x]` when all descendants are selected, and `[ ]` when none are selected.

Example install flow inside a target project directory:

```bash
# configure once
hermes configure --skills-source /path/to/hermes/repo --agents-source /path/to/hermes/agents --commands-source /path/to/hermes/commands

# inside any target project directory
hermes install
hermes install --skills code-review --agents review --commands test
hermes list --installed all
hermes doctor
```

## Repository Layout

- `repo/`: shared skill library
- `agents/`: shared OpenCode agent library
- `commands/`: shared OpenCode command library (markdown command files)
- `docs/`: implementation and design drafts for `hermes`
- `openspec/specs/`: initial OpenSpec capability specs for `hermes`
- `src/`: Rust implementation of `hermes`
- `Cargo.toml`: Rust package definition for `hermes`

## Current Direction

The current implementation of `hermes` is a Rust CLI that:

- persists default source roots in `~/.config/hermes_cli/config.toml`
- installs skills as directories into `.opencode/skills/`
- installs agents as markdown files into `.opencode/agents/`
- installs commands as markdown files into `.opencode/commands/`
- keeps project-local copies self-contained
- tracks project-local installs through a unified `.opencode/catalog.toml` manifest

### Command Support (v1)

Hermes now supports installing OpenCode custom commands. Commands are markdown files with optional YAML frontmatter (containing `description`, `agent`, `subtask`, `model`) followed by a template body. Commands are discovered from the configured `commands_source_root` and installed into `.opencode/commands/` as standalone markdown files. This v1 implementation supports markdown command files only; JSON-configured commands from `opencode.json` are not supported.
