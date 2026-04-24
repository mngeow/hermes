## Context

Hermes currently treats `repo/` as a flat list of skill directories and `agents/` and `commands/` as flat lists of markdown files. The discovery code in `src/skills.rs`, `src/agents.rs`, and `src/commands.rs` only inspects direct children, while the install TUI in `src/tui.rs` renders three flat panes of selectable artifacts.

The current project-local `.opencode/` layout is also flat per kind: skills install into `.opencode/skills/<skill-name>/`, agents into `.opencode/agents/<agent-name>.md`, and commands into `.opencode/commands/<command-name>.md`. That layout already matches the rest of Hermes, the current docs, and the manifest model. The manifest also already stores both `source_rel_path` and `installed_rel_path`, which gives Hermes enough information to manage nested source files without changing the installed workspace shape.

## Goals / Non-Goals

**Goals:**
- Recursively discover valid skills, agents, and commands beneath grouped source folders.
- Keep the existing flat install destinations in `.opencode/` so current project behavior remains stable.
- Let the install TUI browse grouped folders and select either individual artifacts or an entire folder subtree.
- Preserve enough source-path information for sync, remove, and doctor to keep working with grouped libraries.
- Handle duplicate artifact names clearly when grouped folders would otherwise create install-path collisions.

**Non-Goals:**
- Introducing nested install directories inside `.opencode/skills/`, `.opencode/agents/`, or `.opencode/commands/`.
- Adding tag-based, virtual, or metadata-only grouping that is not backed by real folders.
- Introducing new path-based CLI install syntax in this change.
- Expanding agent or command packaging beyond the existing standalone markdown-file model.

## Decisions

### 1. Grouping is a source-library feature; installed artifacts stay flat

Hermes will support nested folders only in the configured source roots. Installed skills, agents, and commands will keep their current flat destinations, while catalog entries preserve the nested `source_rel_path` needed to find the original artifact later.

- **Why:** This is the smallest change that satisfies grouped browsing and folder-level selection without changing Hermes's existing workspace model or relying on unproven nested `.opencode/` runtime behavior.
- **Alternative considered:** Preserve the full source folder tree under `.opencode/`. Rejected because it would change install destinations, increase compatibility risk, and force broader updates across remove, doctor, and user expectations.

### 2. Recursive discovery will use kind-specific candidate rules and ignore pure grouping folders

Hermes will walk each configured source root recursively. For skills, any descendant directory with a top-level `SKILL.md` is a candidate skill. For agents and commands, any descendant `.md` file is a candidate artifact. Directories that only exist to group descendants are not artifacts and are not validation errors by themselves.

- **Why:** Users want normal folders for organization, not extra metadata files just to make a group valid.
- **Alternative considered:** Require explicit marker files for folder groups. Rejected because it would add authoring overhead without improving install behavior.

### 3. Hermes will keep leaf install names and reject duplicate names within a kind

Because installed destinations remain flat, the install name for each artifact kind will remain the current leaf name: the skill directory name, agent filename stem, or command filename stem. If recursive discovery finds two artifacts of the same kind that would install to the same path, Hermes will exclude them from install choices and report an ambiguity issue.

- **Why:** This preserves the current CLI and manifest model while making collisions explicit instead of silently overwriting one artifact with another.
- **Alternative considered:** Add source-relative identifiers such as `backend/review` to the CLI and manifest. Rejected for this change because the user asked for grouped folders and TUI support, and path-based command-surface changes would broaden scope significantly.

### 4. Folder selection in the TUI expands to artifact selections before install planning

Each TUI pane will present a folder tree for its artifact kind. Leaf nodes represent installable artifacts; folder nodes represent source-library groups. Selecting a folder applies selection to all valid descendant leaf artifacts, and deselecting the folder clears that subtree. Hermes will expand folder selections into concrete artifact selections before it computes the install plan, so the catalog continues to track artifacts rather than folders.

- **Why:** The user wants to copy whole subfolders, but Hermes lifecycle operations still act on concrete artifacts.
- **Alternative considered:** Flatten all items into slash-delimited labels and add a separate "select all in prefix" action. Rejected because it makes folder-level browsing less obvious and does not map cleanly to the requested grouped TUI behavior.

### 5. Existing lifecycle flows will remain artifact-based but use nested source paths

Install, sync, remove, and doctor will continue to manage one artifact per catalog entry. New grouped installs will record nested `source_rel_path` values such as `backend/review.md` or `quality/python-testing`, while `installed_rel_path` keeps the existing flat project-local destination. Sync will resolve grouped sources through `source_rel_path`; remove and doctor will continue to use `installed_rel_path` and manifest membership to manage installed copies.

- **Why:** The manifest already has the right shape for this change, so Hermes can extend source discovery without inventing a second grouping data model.
- **Alternative considered:** Track selected folders as first-class manifest entries. Rejected because sync and remove operate on installed artifacts, not abstract source groups.

## Risks / Trade-offs

- [Grouped folders increase the chance of duplicate names] → Exclude ambiguous artifacts from install choices and report collisions clearly in inspection and doctor output.
- [Recursive scans can traverse irrelevant files and folders] → Reuse existing ignore rules and only validate paths that match the candidate rules for the artifact kind.
- [Hierarchical panes are more complex than flat lists] → Keep the current three-pane layout and confirmation model, adding only the minimum folder navigation and subtree-selection behavior.
- [Selecting a folder may install more artifacts than the user expected] → Render folders distinctly from leaf artifacts and show subtree selection state so bulk selections stay visible.

## Migration Plan

No explicit workspace migration is required. Existing flat source libraries and existing installed workspaces continue to work unchanged.

New grouped installs will start recording nested `source_rel_path` values in `.opencode/catalog.toml`, but existing catalog entries remain valid because their relative paths are already supported. If a source library introduces duplicate names across groups, those artifacts will remain unmanaged until the collision is resolved by renaming one of them.

## Open Questions

None.
