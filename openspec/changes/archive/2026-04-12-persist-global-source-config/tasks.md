## 1. User config persistence

- [x] 1.1 Add a user-level Hermes config model and filesystem helpers for `~/.config/hermes_cli/config.toml`.
- [x] 1.2 Add a `hermes configure` command that writes canonical absolute `skills_source_root` and `agents_source_root` values while preserving unspecified existing roots.

## 2. Source resolution and command flow

- [x] 2.1 Update source-root resolution to use CLI flags, then user config, then environment variables, and stop using project catalog root values as defaults.
- [x] 2.2 Update `hermes init` to stay focused on workspace creation and stop acting as the primary writer for persistent user source-root settings.
- [x] 2.3 Update `hermes install` and other source-resolving commands to work in fresh projects from resolved roots without requiring a prior `hermes init`.

## 3. Catalog compatibility and verification

- [x] 3.1 Keep reads of older `.opencode/catalog.toml` files compatible, but omit top-level default source-root fields from new catalog writes.
- [x] 3.2 Update user-facing docs/help text to describe `hermes configure`, `~/.config/hermes_cli/config.toml`, and the no-init install flow.
- [x] 3.3 Run `cargo build` and confirm it passes.
