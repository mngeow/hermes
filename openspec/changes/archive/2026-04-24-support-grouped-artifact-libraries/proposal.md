## Why

Hermes currently assumes flat source libraries for agents and commands, and only top-level skill folders under `repo/`. As those libraries grow, it becomes hard to organize related artifacts, and the install TUI cannot copy a whole category of artifacts in one action.

## What Changes

- Allow grouped subfolders inside the configured skill, agent, and command source roots, with recursive discovery of installable artifacts.
- Keep the existing flat `.opencode/skills/`, `.opencode/agents/`, and `.opencode/commands/` install layout while recording nested source paths for managed artifacts.
- Extend install, list, remove, sync, and doctor flows so grouped source libraries remain manageable and name collisions are handled clearly.
- Update the interactive install TUI to browse grouped folders and let users copy either individual artifacts or an entire folder subtree.
- Keep scope limited to filesystem-based grouping; adding tag systems, virtual categories, or non-filesystem package formats remains out of scope.

## Capabilities

### New Capabilities
- None.

### Modified Capabilities
- `skill-library-installation`: Discover valid skills recursively beneath grouped source folders while preserving the existing flat skill install destination.
- `agent-library-installation`: Discover valid agents recursively beneath grouped source folders while preserving the existing flat agent install destination.
- `command-library-installation`: Discover valid commands recursively beneath grouped source folders while preserving the existing flat command install destination.
- `interactive-artifact-selection-ui`: Support grouped folder browsing and folder-level selection in the install TUI.
- `artifact-catalog-management`: Track nested source paths alongside the existing installed paths so grouped artifacts remain manageable across install, sync, remove, and doctor.
- `hermes-command-surface`: Allow nested artifacts to be selected and referenced consistently across install, list, and remove flows.

## Impact

- Affected code: discovery and validation logic, install planning, manifest/catalog models, list/remove/sync/doctor flows, and the ratatui-based install UI.
- Affected files: likely `src/skills.rs`, `src/agents.rs`, `src/commands.rs`, `src/install.rs`, `src/remove.rs`, `src/sync.rs`, `src/doctor.rs`, `src/models.rs`, `src/app.rs`, `src/tui.rs`, and supporting docs/specs.
- Existing flat libraries remain supported, but grouped libraries will require recursive discovery, collision detection, and richer TUI navigation.
