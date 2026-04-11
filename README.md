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

It is effectively the delivery and transport layer between a central reusable library and a project's local OpenCode workspace.

## What Hermes Does

- discovers installable skills from `repo/`
- discovers installable agents from `agents/`
- lets a project choose which skills and agents to install locally
- copies them into the local `.opencode/` directory
- records installed state in `.opencode/catalog.toml`
- supports follow-up management operations like `list`, `sync`, `remove`, and `doctor`

## Build And Run

Build the CLI locally with:

```bash
cargo build
```

Initialize a target project with:

```bash
cargo run -- init --skills-source ./repo --agents-source ./agents
```

## Interactive Mode

`hermes` can also boot into an interactive terminal UI for install selection.

After `init` has recorded the source roots for a project, run `hermes install` with no explicit artifact names to open the TUI:

```bash
hermes init --skills-source /path/to/hermes/repo --agents-source /path/to/hermes/agents
hermes install
```

If you are running directly from this repository instead of an installed binary, use:

```bash
cargo run -- init --skills-source ./repo --agents-source ./agents
cargo run -- install --skills-source ./repo --agents-source ./agents
```

The TUI lets you browse both skills and agents in one screen. Use `Tab` to switch focus, `Up` and `Down` or `j` and `k` to move, `Space` or `Enter` to toggle items, `c` to confirm, and `q` or `Esc` to cancel.

Example install flow inside a target project directory:

```bash
hermes init --skills-source /path/to/hermes/repo --agents-source /path/to/hermes/agents
hermes install
hermes install --skills code-review --agents review
hermes list --installed all
hermes doctor
```

## Repository Layout

- `repo/`: shared skill library
- `agents/`: shared OpenCode agent library
- `docs/`: implementation and design drafts for `hermes`
- `openspec/specs/`: initial OpenSpec capability specs for `hermes`
- `src/`: Rust implementation of `hermes`
- `Cargo.toml`: Rust package definition for `hermes`

## Current Direction

The current implementation of `hermes` is a Rust CLI that:

- installs skills as directories into `.opencode/skills/`
- installs agents as markdown files into `.opencode/agents/`
- keeps project-local copies self-contained
- tracks those installs through a unified `.opencode/catalog.toml` manifest
