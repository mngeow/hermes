use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::frontmatter::parse_skill_file;
use crate::models::{DiscoveredSkill, Inspection};

pub fn inspect_skills(source_root: &Path) -> Result<Inspection<DiscoveredSkill>> {
    let mut entries = fs::read_dir(source_root)
        .with_context(|| {
            format!(
                "failed to read skills source root {}",
                source_root.display()
            )
        })?
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    let mut inspection = Inspection::default();
    for entry in entries {
        let file_type = entry.file_type()?;
        if !file_type.is_dir() {
            continue;
        }

        let path = entry.path();
        match parse_skill_dir(&path, Some(source_root)) {
            Ok(skill) => inspection.items.push(skill),
            Err(err) => inspection.issues.push(format!(
                "invalid skill '{}': {err:#}",
                entry.file_name().to_string_lossy()
            )),
        }
    }

    Ok(inspection)
}

pub fn validate_installed_skill(path: &Path) -> Result<DiscoveredSkill> {
    parse_skill_dir(path, None)
}

fn parse_skill_dir(path: &Path, source_root: Option<&Path>) -> Result<DiscoveredSkill> {
    let dir_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("skill directory has no valid UTF-8 name"))?;

    let skill_md = path.join("SKILL.md");
    if !skill_md.is_file() {
        bail!("missing top-level SKILL.md")
    }

    let frontmatter = parse_skill_file(&skill_md)?;
    if frontmatter.name != dir_name {
        bail!(
            "frontmatter name '{}' does not match directory name '{}'",
            frontmatter.name,
            dir_name
        )
    }

    let source_rel_path = match source_root {
        Some(root) => path.strip_prefix(root)?.to_path_buf(),
        None => PathBuf::from(dir_name),
    };

    Ok(DiscoveredSkill {
        name: frontmatter.name,
        description: frontmatter.description,
        source_path: path.to_path_buf(),
        source_rel_path,
    })
}
