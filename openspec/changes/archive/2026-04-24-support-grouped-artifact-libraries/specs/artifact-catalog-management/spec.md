## MODIFIED Requirements

### Requirement: Record installed artifact state in a unified catalog
The system SHALL persist install mode and installed skills, agents, and commands in `.opencode/catalog.toml` with per-artifact source-relative paths, flat install paths, and hashes needed to manage them later, including grouped source paths for recursively discovered libraries, and SHALL not use the catalog as the persisted default source-root configuration store.

#### Scenario: Record installed skill metadata from a grouped source path
- **WHEN** a skill discovered from a grouped source path is installed
- **THEN** the catalog SHALL store its `name`, `description`, grouped `source_rel_path`, flat `installed_rel_path`, `source_hash`, and `installed_hash`

#### Scenario: Record installed agent metadata from a grouped source path
- **WHEN** an agent discovered from a grouped source path is installed
- **THEN** the catalog SHALL store its `name`, `description`, `mode`, grouped `source_rel_path`, flat `installed_rel_path`, `source_hash`, and `installed_hash`

#### Scenario: Record installed command metadata from a grouped source path
- **WHEN** a command discovered from a grouped source path is installed
- **THEN** the catalog SHALL store its `name`, optional `description`, grouped `source_rel_path`, flat `installed_rel_path`, `source_hash`, and `installed_hash`

#### Scenario: Keep default source roots out of the project catalog
- **WHEN** Hermes writes `.opencode/catalog.toml`
- **THEN** it SHALL omit top-level persisted default `skills_source_root`, `agents_source_root`, and `commands_source_root` fields
