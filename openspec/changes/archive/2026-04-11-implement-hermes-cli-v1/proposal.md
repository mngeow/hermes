## Why

The repository now has a curated skill library, a curated agent library, implementation docs, and baseline OpenSpec requirements, but no working `hermes` binary. Building the first version now turns the documented design into a usable tool and validates that the repository structure and specs support real project installation workflows.

## What Changes

- Create the first Rust implementation of `hermes` as a cargo-managed CLI binary.
- Implement `init`, `install`, `list`, `sync`, `remove`, and `doctor` commands around the documented `.opencode/` layout and catalog model.
- Implement skill discovery and installation from `repo/`, including validation of `SKILL.md` frontmatter and recursive copying of bundled resources.
- Implement agent discovery and installation from `agents/`, including validation of markdown frontmatter and preservation of installed agent files verbatim.
- Persist managed state in `.opencode/catalog.toml` and use hashes to support safe sync and overwrite detection.
- Add the project configuration and build-verification rules needed for future OpenSpec changes to keep the CLI buildable.

## Capabilities

### New Capabilities
- `hermes-command-surface`: Define the first supported `hermes` subcommands, argument shapes, and user-visible command behaviors.

### Modified Capabilities

## Impact

- Adds a new Rust project and its dependencies.
- Adds the first executable implementation of the Hermes installer workflow.
- Adds an OpenSpec change and command-surface delta spec for the first release scope.
- Updates OpenSpec project configuration so future implementation changes always end with a successful `cargo build`.
