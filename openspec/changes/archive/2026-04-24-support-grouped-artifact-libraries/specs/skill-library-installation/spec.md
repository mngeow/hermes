## MODIFIED Requirements

### Requirement: Discover installable skills from a configured source library
The system SHALL recursively scan the configured skills source root, treat any descendant directory containing a top-level `SKILL.md` as a candidate skill, and only expose a skill as installable when its `SKILL.md` frontmatter contains `name` and `description`, the `name` matches the skill directory name, and that skill name is unique across the full source tree.

#### Scenario: List valid skills from grouped folders
- **WHEN** the configured skills source root contains valid nested skill folders such as `review/code-review/` and `testing/python-testing/`
- **THEN** the CLI SHALL list them as available skills using their `name` and `description`

#### Scenario: Skip grouping-only folders
- **WHEN** the configured skills source root contains intermediate folders such as `review/` or `testing/` that do not have their own top-level `SKILL.md` but contain descendant skill folders
- **THEN** the CLI SHALL treat those folders as grouping containers instead of installable skills
- **AND** SHALL NOT report those grouping folders as invalid by themselves

#### Scenario: Reject invalid nested skill directories
- **WHEN** a descendant skill directory contains `SKILL.md` but is missing required frontmatter fields or has a mismatched `name`
- **THEN** the CLI SHALL exclude that skill from installation choices
- **AND** the `doctor` command SHALL report the validation problem

#### Scenario: Reject duplicate skill names across groups
- **WHEN** two valid nested skill directories would both install as the same skill name
- **THEN** the CLI SHALL exclude that duplicate skill name from installation choices
- **AND** the `doctor` command SHALL report the ambiguity

### Requirement: Install selected skills into the project-local OpenCode directory
The system SHALL install selected skills into `<project>/.opencode/skills/<skill-name>/` as flat project-local copies even when they were discovered in grouped source folders.

#### Scenario: Install named grouped skills non-interactively
- **WHEN** the user runs `hermes install` with an explicit skill name that resolves to a uniquely named nested skill
- **THEN** the CLI SHALL copy that skill into `.opencode/skills/<skill-name>/`
- **AND** preserve its grouped source-relative path for later management

#### Scenario: Select grouped skills interactively
- **WHEN** the user runs `hermes install` without explicit skill names and a grouped skills source root is configured
- **THEN** the CLI SHALL let the user browse grouped skill folders through the interactive install selection UI
- **AND** install only the selected descendant skills into `.opencode/skills/<skill-name>/`
