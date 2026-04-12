## Why

Hermes currently persists skills and agents source roots in each project's `.opencode/catalog.toml`, so users have to run `hermes init` in every workspace before `hermes install` can discover artifacts. That adds avoidable setup friction when the source libraries are stable user-level paths that should be configured once and reused across projects.

## What Changes

- Add a user-level Hermes config file at `~/.config/hermes_cli/config.toml` to store the default `skills_source_root` and `agents_source_root`.
- Add a dedicated `hermes configure` command to create and update that user-level config from the CLI.
- Update source root resolution so commands can use the user config file instead of requiring a project-local catalog entry for those paths.
- Allow `hermes install` to work in a fresh project without a prior `hermes init` as long as source roots are available from flags, user config, or environment variables.
- Keep `.opencode/catalog.toml` focused on project-local installed artifact state instead of treating it as the primary long-term home for reusable source root configuration.
- Keep `hermes init` focused on project workspace creation instead of using it as the main way to edit persistent user settings.

## Capabilities

### New Capabilities
- `user-source-config`: Persist, update, and load default skills and agents source roots from a user-level Hermes config file under `~/.config/hermes_cli/`.

### Modified Capabilities
- `artifact-catalog-management`: Move source root persistence away from the project catalog and update source root resolution to use the user config file.
- `hermes-command-surface`: Add a `configure` command for user settings and update `init` and `install` behavior so install no longer depends on a prior init when source roots are otherwise resolvable.

## Impact

- Affected code: CLI command flow, source-root resolution, manifest/config models, and filesystem setup behavior.
- Affected files: `src/cli.rs`, `src/app.rs`, `src/install.rs`, `src/manifest.rs`, `src/models.rs`, related CLI/docs, and new OpenSpec deltas for command surface, artifact catalog management, and user config behavior.
- User impact: users can run `hermes configure` once to manage source roots in `~/.config/hermes_cli/config.toml` and then run `hermes install` directly in new projects.
