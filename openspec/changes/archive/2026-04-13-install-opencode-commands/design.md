## Context

Hermes currently installs reusable skills from `repo/` into `.opencode/skills/` and reusable agents from `agents/` into `.opencode/agents/`, with source-root resolution, catalog tracking, sync protection, and an interactive install UI built around those two artifact kinds. OpenCode also supports reusable markdown command files in `.opencode/commands/`, so adding command copying is a cross-cutting change that affects discovery, configuration, workspace bootstrap, catalog state, install flows, and the TUI.

Commands are closer to agents than skills: each command is a single markdown file whose filename becomes the command name and whose body is the prompt template. Unlike skills, commands do not require directory recursion or bundled resources, but they do need the same source-root configuration and local-change protection guarantees as the existing managed artifacts.

## Goals / Non-Goals

**Goals:**
- Add a third managed artifact kind for reusable OpenCode command markdown files.
- Extend configure, install, list, sync, remove, doctor, and interactive install flows so commands are handled consistently with skills and agents.
- Preserve installed command files verbatim while tracking enough metadata and hashes to manage them safely later.
- Keep the v1 implementation aligned with the existing copy-based Hermes architecture.

**Non-Goals:**
- Importing JSON-defined commands from `opencode.json` or `opencode.jsonc`.
- Managing global OpenCode command directories such as `~/.config/opencode/commands/`.
- Rewriting command templates, validating runtime placeholders like `$ARGUMENTS`, or executing referenced shell/file inputs during install.

## Decisions

### 1. Treat commands as standalone markdown artifacts from a dedicated source root

Hermes will discover commands from a configured `commands_source_root`, using direct child `.md` files as candidate artifacts. A command is installable when the file has a non-empty markdown body and any YAML frontmatter present parses successfully. The installed artifact path will be `.opencode/commands/<command-name>.md`, preserving the filename-based command name used by OpenCode.

- **Why:** This matches the OpenCode commands model, keeps discovery parallel to agent files, and avoids inventing a new packaging format.
- **Alternative considered:** Support JSON config-defined commands or nested command directories. Rejected because the user request is about copying command files into `.opencode/commands/`, and broader config import would expand scope significantly.

### 2. Add commands as a first-class kind in workspace bootstrap, source resolution, and catalog state

Hermes will add `.opencode/commands/` to the initialized workspace, `commands_source_root` to user-level config resolution, and a catalog record for installed commands containing the command name, optional description, relative source path, relative installed path, source hash, and installed hash. Source-root precedence remains CLI flag, then user config, then environment variable, with a new command-specific environment variable following the existing naming pattern.

- **Why:** Commands need the same lifecycle support as skills and agents, and folding them into the existing catalog keeps sync, remove, and doctor flows centralized.
- **Alternative considered:** Infer installed commands by scanning `.opencode/commands/` without catalog entries. Rejected because Hermes would lose consistent overwrite protection and deterministic sync behavior.

### 3. Reuse the existing single-file copy and hash workflow used for agents

Hermes will copy command markdown files verbatim, compute source and installed hashes, and reuse the same modified-local-copy safeguards already used for other managed artifacts. Sync will only overwrite unchanged local command files automatically, and `--force` will allow explicit replacement of modified copies.

- **Why:** Commands are single-file artifacts, so the agent-style workflow is already the smallest correct implementation.
- **Alternative considered:** Add command-specific overwrite logic. Rejected because it would duplicate existing safety behavior with no user benefit.

### 4. Extend the existing install interfaces instead of adding a separate command-specific workflow

`hermes install` will gain explicit command selection alongside the current skills and agents flows, including a `--commands` flag path, a kind-specific `hermes install commands ...` form, and interactive TUI support for browsing commands with their descriptions.

- **Why:** Users already learn one install workflow in Hermes; commands should slot into that mental model instead of introducing another top-level command.
- **Alternative considered:** Add a dedicated `hermes copy-commands` command. Rejected because it would fragment artifact management and duplicate install semantics.

## Risks / Trade-offs

- [OpenCode also supports JSON-configured commands] → Limit v1 to markdown command files and document that scope clearly in specs and user-facing docs.
- [Existing workspaces do not have `.opencode/commands/`] → Create the directory during init/bootstrap and lazily create it during command operations against older workspaces.
- [The install TUI becomes denser with a third artifact kind] → Reuse the current keyboard model and add commands without changing the confirmation flow.
- [Optional command frontmatter makes validation looser than agents] → Require parseable frontmatter when present and a non-empty template body so clearly broken files stay out of install choices.

## Migration Plan

No explicit migration command is required. New workspaces will include `.opencode/commands/`, while existing workspaces can be upgraded in place the first time Hermes initializes command support by creating the missing directory and extending catalog writes to include command entries when present.

## Open Questions

None.
