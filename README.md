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

Run it with:

```bash
cargo run -- init --skills-source ./repo --agents-source ./agents
```

Example install flow inside a target project directory:

```bash
hermes init --skills-source /path/to/hermes/repo --agents-source /path/to/hermes/agents
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
