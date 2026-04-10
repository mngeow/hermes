## Purpose

Define how `hermes` discovers, selects, and installs reusable skills from a central library into a project-local `.opencode/skills/` directory.

## Requirements

### Requirement: Discover installable skills from a configured source library
The system SHALL treat direct child directories of the configured skills source root as candidate skills and only expose a directory as installable when it contains a top-level `SKILL.md` with YAML frontmatter containing `name` and `description`, and the `name` matches the directory name.

#### Scenario: List valid skills from the source library
- **WHEN** the configured skills source root contains valid skill folders such as `code-review/` and `python-testing/`
- **THEN** the CLI SHALL list them as available skills using their `name` and `description`

#### Scenario: Skip invalid skill directories
- **WHEN** a child directory is missing `SKILL.md`, missing required frontmatter fields, or has a mismatched `name`
- **THEN** the CLI SHALL exclude it from installation choices
- **AND** the `doctor` command SHALL report the validation problem

### Requirement: Install selected skills into the project-local OpenCode directory
The system SHALL install selected skills into `<project>/.opencode/skills/<skill-name>/` as project-local copies.

#### Scenario: Install named skills non-interactively
- **WHEN** the user runs `hermes install` with explicit skill names
- **THEN** the CLI SHALL copy each selected skill into `.opencode/skills/<skill-name>/`
- **AND** the CLI SHALL report which skills were installed

#### Scenario: Select skills interactively
- **WHEN** the user runs `hermes install` without explicit skill names and a skills source root is configured
- **THEN** the CLI SHALL present an interactive multi-select list of available skills
- **AND** install only the selected skills

### Requirement: Preserve skill resources during installation
The system SHALL copy the full skill directory recursively so installed skills remain self-contained.

#### Scenario: Preserve nested skill resources
- **WHEN** a selected skill contains nested resources such as `references/`, `scripts/`, `checklists/`, `guidelines/`, or `assets/`
- **THEN** the CLI SHALL copy those resources into the installed skill directory
- **AND** preserve their relative paths

#### Scenario: Ignore generated junk during copy
- **WHEN** a selected skill directory contains ignored generated files such as `.DS_Store`, `__pycache__`, `*.pyc`, `.git`, or `node_modules`
- **THEN** the CLI SHALL exclude those paths from copy and hashing operations
