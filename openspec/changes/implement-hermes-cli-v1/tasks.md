## 1. Project Setup

- [x] 1.1 Create the initial cargo project for the `hermes` binary and add the required Rust dependencies.
- [x] 1.2 Set up the source module structure for CLI parsing, discovery, manifest IO, install, sync, remove, doctor, hashing, and shared filesystem helpers.

## 2. Core Discovery And Catalog

- [x] 2.1 Implement shared frontmatter parsing plus skill and agent discovery and validation.
- [x] 2.2 Implement `.opencode/catalog.toml` models, path resolution, and manifest read/write behavior.
- [x] 2.3 Implement hashing for skill directories and agent markdown files with the documented ignore rules.

## 3. Command Implementation

- [x] 3.1 Implement `hermes init` to create `.opencode/` and persist configured source roots.
- [x] 3.2 Implement `hermes install` for explicit names and interactive selection, including copy-based installation for skills and agents.
- [x] 3.3 Implement `hermes list`, `hermes remove`, `hermes sync`, and `hermes doctor` against the manifest and source libraries.

## 4. Verification

- [x] 4.1 Update README usage details if the final command behavior differs from the current draft.
- [x] 4.2 Run `cargo build` and fix any build failures until it passes.
