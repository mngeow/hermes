## 1. Recursive source discovery

- [x] 1.1 Refactor skill, agent, and command inspection to walk grouped source folders recursively and ignore pure grouping directories that are not artifacts.
- [x] 1.2 Add per-kind duplicate-name detection so ambiguous grouped artifacts are excluded from install choices and surfaced as clear inspection or doctor issues.

## 2. Install and catalog behavior

- [x] 2.1 Update install planning and explicit name resolution so uniquely named grouped artifacts can still be installed into the existing flat `.opencode/` destinations.
- [x] 2.2 Preserve nested `source_rel_path` values in catalog entries for grouped artifacts while keeping sync, remove, list, and doctor behavior consistent with the current installed layout.

## 3. Interactive TUI grouping support

- [x] 3.1 Replace flat TUI pane data with grouped folder and leaf artifact nodes for skills, agents, and commands.
- [x] 3.2 Implement keyboard-driven subtree selection so users can select either an individual artifact or an entire folder before Hermes computes the install plan.
- [x] 3.3 Update TUI rendering to display grouped folders, leaf metadata, and visible selection state without breaking the existing pane focus and confirmation flow.

## 4. Documentation and verification

- [x] 4.1 Update README and supporting docs to describe grouped source-library folders, duplicate-name constraints, and folder-level TUI selection.
- [x] 4.2 Add or update automated tests covering recursive discovery, duplicate-name rejection, grouped catalog paths, and grouped TUI selection behavior.
- [x] 4.3 Run `cargo build` and confirm the build passes.
