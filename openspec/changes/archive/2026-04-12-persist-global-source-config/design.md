## Context

Hermes currently resolves `skills_source_root` and `agents_source_root` from CLI flags, then the project-local `.opencode/catalog.toml`, then environment variables. That makes source-root configuration project-scoped even though the source libraries are usually user-scoped directories reused across many projects.

The current install flow already creates `.opencode/` when needed, but users still have to run `hermes init` in each new project to populate source roots before `hermes install` can discover artifacts. This change separates reusable source-root configuration from project-local installed-state tracking.

## Goals / Non-Goals

**Goals:**
- Store default skills and agents source roots in a user-level config file at `~/.config/hermes_cli/config.toml`.
- Let `hermes install` resolve source roots from that config and proceed in a fresh project without a prior `hermes init`.
- Keep `.opencode/catalog.toml` focused on project-local install state.
- Preserve existing CLI flags and environment-variable support.

**Non-Goals:**
- Adding richer configuration profile management beyond a single default config file.
- Changing install semantics for copying, hashing, sync, or removal.
- Introducing XDG portability beyond the explicitly requested `~/.config/hermes_cli/` path.

## Decisions

### 1. Add a separate user config model and file
Hermes will read and write a TOML config file at `~/.config/hermes_cli/config.toml` with optional `skills_source_root` and `agents_source_root` fields.

- **Why:** the roots describe the user's central libraries, not per-project state.
- **Alternative considered:** keep storing roots in `.opencode/catalog.toml`. Rejected because it forces repeated setup in every workspace.
- **Alternative considered:** rely only on environment variables. Rejected because it does not give Hermes a persistent default configuration.

### 2. Update source-root precedence to CLI flags → user config → environment variables
Source roots remain independently resolvable for skills and agents, but project catalog values are removed from the precedence chain.

- **Why:** this preserves explicit overrides while making the new config the normal reusable default.
- **Alternative considered:** CLI flags → environment variables → user config. Rejected to stay aligned with the current pattern where persisted configuration outranks environment fallback.

### 3. Keep the project catalog for installed-state only
`.opencode/catalog.toml` will continue to store install mode and installed artifact metadata, but it will stop being the long-term home for default source-root configuration.

- **Why:** duplicating the same roots in both global and project-local files creates drift and makes source resolution harder to reason about.
- **Alternative considered:** store roots in both files. Rejected because it adds unnecessary synchronization rules.

### 4. Add `configure` as the configuration writer and keep `install` zero-init
`hermes configure` will create or update `~/.config/hermes_cli/config.toml` when source-root flags are provided. `hermes init` remains responsible for creating the local workspace, while `hermes install` and other source-resolving commands will read the user config when flags are absent and bootstrap `.opencode/` if the project has not been initialized yet.

- **Why:** it gives users an explicit, discoverable way to manage persistent settings without overloading project initialization semantics.
- **Alternative considered:** keep using `hermes init` as the config writer. Rejected because it couples global settings changes to a project-scoped command and leaves no clear command for updating settings later.

### 5. Treat existing catalogs as backward-compatible input
Older `.opencode/catalog.toml` files that still contain `skills_source_root` or `agents_source_root` can be read during transition, but new writes should omit those top-level fields.

- **Why:** this avoids a hard migration step for existing projects.
- **Alternative considered:** require an explicit migration command. Rejected because the data can be phased out safely during normal writes.

## Risks / Trade-offs

- **[Global config can become stale]** → `doctor` and normal source-root validation should continue surfacing missing paths clearly.
- **[Ignoring old catalog roots may surprise existing users]** → document the new precedence and keep CLI flags as the highest-priority escape hatch.
- **[Using `~/.config/hermes_cli/` directly is less portable than full XDG handling]** → accept this as intentional scope because the requested behavior explicitly targets that location.

## Migration Plan

1. Add the user config model and filesystem helpers.
2. Update source-root resolution to read CLI flags, then user config, then environment variables.
3. Remove project-catalog writes for default source roots while keeping existing catalog parsing compatible.
4. Add the `configure` command to write user config and confirm `install` can initialize a new workspace from resolved roots.
5. Keep `init` focused on workspace creation while updating docs/help text to explain the new config location and zero-init install flow.

## Open Questions

- None.
