## Context

The repository already defines Hermes at the documentation and specification level:

- `docs/rust-skill-installer-cli.md` defines the desired command surface, manifest shape, and filesystem behavior.
- `docs/sample-agents-library.md` defines the initial `agents/` library structure.
- `openspec/specs/` defines the installation and catalog-management requirements.

What is missing is the executable CLI itself. There is no cargo project, no command parser, and no implementation of discovery, installation, catalog persistence, or sync behavior.

The first version should stay intentionally small, but it still needs to be complete enough to exercise the documented end-to-end flow:

- initialize a project-local `.opencode/` workspace
- discover skills and agents from configurable source roots
- install selected artifacts into `.opencode/`
- list, sync, remove, and validate managed artifacts

## Goals / Non-Goals

**Goals:**

- Deliver a working Rust binary named `hermes`.
- Implement the documented v1 command surface: `init`, `install`, `list`, `sync`, `remove`, and `doctor`.
- Support both explicit artifact names and interactive selection when names are omitted.
- Use copy-based installs and a unified `.opencode/catalog.toml` manifest.
- Detect local modifications with hashes and skip silent overwrites unless `--force` is used.
- Finish the implementation with a passing `cargo build`.

**Non-Goals:**

- Link-mode installs.
- Multi-file agent support.
- Remote registries or publishing.
- Automatic merging of locally edited installed artifacts.
- JSON output, diff commands, or export commands.

## Decisions

### Use a single cargo binary crate named `hermes`

The repository currently has no Rust workspace, so the smallest viable implementation is a single binary crate at the repository root. This keeps setup simple and makes `cargo build` the project-wide verification step.

Alternative considered:
- Rust workspace with multiple crates. Rejected for v1 because the codebase is still small and the extra structure adds friction without solving a current problem.

### Keep the implementation modular but lightweight

The code should follow the documented module split closely enough to keep responsibilities clear: CLI parsing, discovery, frontmatter parsing, manifest IO, installation, sync, removal, doctor, hashing, and filesystem helpers.

Alternative considered:
- One large `main.rs`. Rejected because the CLI spans enough behaviors that a monolithic file would become hard to maintain quickly.

### Implement copy-based installs only

The docs and specs already define copy mode as the default and recommended v1 behavior. Implementing only copy mode keeps installation semantics stable and avoids symlink-specific edge cases.

Alternative considered:
- Add `link` mode immediately. Rejected because it increases scope and changes sync semantics without being required for the first release.

### Parse frontmatter manually with `serde_yaml`

Both skills and agents only need the first YAML frontmatter block parsed from a markdown file. A small shared parser that splits the opening `---` block is enough, keeps dependencies modest, and gives tight control over validation.

Alternative considered:
- Introduce a higher-level markdown/frontmatter parsing crate. Rejected for v1 because the required parsing surface is small.

### Use hashes to power sync and overwrite safety

The manifest should store source and installed hashes for skills and agents. Sync and reinstall operations will recompute hashes and only overwrite an installed artifact automatically when the installed copy still matches the recorded manifest state.

Alternative considered:
- Blindly overwrite existing installs. Rejected because it would violate the documented safety model and make local customization unsafe.

### Use atomic replacement via temp paths

Skills are directories and agents are files, but both should be installed through a temp path and renamed into place. This matches the documented approach and avoids leaving partially written artifacts on interruption.

Alternative considered:
- Copy directly into the target path. Rejected because it increases the chance of partial writes and inconsistent catalog state.

### Support interactive selection with `dialoguer`

The documented install flow expects interactive selection when names are omitted. `dialoguer` provides a straightforward multi-select prompt without pulling in a large UI abstraction.

Alternative considered:
- Non-interactive-only v1. Rejected because interactive selection is already part of the documented first version.

## Risks / Trade-offs

- [Interactive prompts may fail in non-TTY contexts] → Keep explicit `--skills` and `--agents` flows fully supported so automation does not depend on interactivity.
- [Frontmatter parsing is strict enough to exclude some malformed legacy files] → Report validation errors clearly in discovery and `doctor` so users can fix the source artifact.
- [Atomic copy and hashing add implementation complexity] → Keep the helper layer small and reuse the same copy and hashing codepaths across install and sync.
- [The first version may expose edge cases around relative and absolute source paths] → Normalize paths early and centralize source-root resolution logic.
