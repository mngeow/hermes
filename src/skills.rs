use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::frontmatter::parse_skill_file;
use crate::models::{DiscoveredSkill, Inspection};

pub fn inspect_skills(source_root: &Path) -> Result<Inspection<DiscoveredSkill>> {
    let mut inspection = Inspection::default();
    walk_skills(source_root, source_root, &mut inspection)?;
    deduplicate_skills(&mut inspection);
    Ok(inspection)
}

fn deduplicate_skills(inspection: &mut Inspection<DiscoveredSkill>) {
    use std::collections::HashMap;

    let mut counts: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for skill in &inspection.items {
        counts
            .entry(skill.name.clone())
            .or_default()
            .push(skill.source_rel_path.clone());
    }

    let mut duplicates = Vec::new();
    inspection.items.retain(|skill| {
        let paths = counts.get(&skill.name).expect("count exists");
        if paths.len() > 1 {
            duplicates.push((skill.name.clone(), paths.clone()));
            false
        } else {
            true
        }
    });

    let mut seen = std::collections::HashSet::new();
    for (name, paths) in duplicates {
        if seen.insert(name.clone()) {
            inspection.duplicate_names.push(name.clone());
            inspection.issues.push(format!(
                "duplicate skill name '{name}' found at: {}",
                paths
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }
}

fn walk_skills(
    source_root: &Path,
    dir: &Path,
    inspection: &mut Inspection<DiscoveredSkill>,
) -> Result<()> {
    let mut entries = fs::read_dir(dir)
        .with_context(|| format!("failed to read skills directory {}", dir.display()))?
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let file_type = entry.file_type()?;
        if !file_type.is_dir() {
            continue;
        }

        let path = entry.path();
        if path.join("SKILL.md").is_file() {
            match parse_skill_dir(&path, Some(source_root)) {
                Ok(skill) => inspection.items.push(skill),
                Err(err) => inspection.issues.push(format!(
                    "invalid skill '{}': {err:#}",
                    entry.file_name().to_string_lossy()
                )),
            }
        } else {
            // Pure grouping folder: recurse without reporting an error
            walk_skills(source_root, &path, inspection)?;
        }
    }

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_dir() -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "hermes-skills-test-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::SeqCst)
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn write_skill(root: &Path, rel_path: &str, name: &str, description: &str) {
        let dir = root.join(rel_path);
        std::fs::create_dir_all(&dir).unwrap();
        let mut file = std::fs::File::create(dir.join("SKILL.md")).unwrap();
        write!(file, "---\nname: {name}\ndescription: {description}\n---\n").unwrap();
    }

    #[test]
    fn discovers_flat_skills() {
        let root = temp_dir();
        write_skill(&root, "code-review", "code-review", "Review code");
        write_skill(&root, "test", "test", "Run tests");

        let inspection = inspect_skills(&root).unwrap();
        assert_eq!(inspection.items.len(), 2);
        assert!(inspection.issues.is_empty());
        assert!(inspection.duplicate_names.is_empty());

        let names: Vec<_> = inspection.items.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"code-review"));
        assert!(names.contains(&"test"));
    }

    #[test]
    fn discovers_grouped_skills() {
        let root = temp_dir();
        write_skill(&root, "review/code-review", "code-review", "Review code");
        write_skill(
            &root,
            "review/security-review",
            "security-review",
            "Security review",
        );
        write_skill(
            &root,
            "testing/python-testing",
            "python-testing",
            "Python tests",
        );

        let inspection = inspect_skills(&root).unwrap();
        assert_eq!(inspection.items.len(), 3);
        assert!(inspection.issues.is_empty());

        let names: Vec<_> = inspection.items.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"code-review"));
        assert!(names.contains(&"security-review"));
        assert!(names.contains(&"python-testing"));

        let paths: Vec<_> = inspection
            .items
            .iter()
            .map(|s| s.source_rel_path.to_string_lossy().to_string())
            .collect();
        assert!(paths.contains(&"review/code-review".to_string()));
        assert!(paths.contains(&"testing/python-testing".to_string()));
    }

    #[test]
    fn ignores_grouping_folders() {
        let root = temp_dir();
        std::fs::create_dir_all(root.join("review")).unwrap();
        write_skill(&root, "review/code-review", "code-review", "Review code");

        let inspection = inspect_skills(&root).unwrap();
        assert_eq!(inspection.items.len(), 1);
        assert!(inspection.issues.is_empty());
    }

    #[test]
    fn detects_duplicate_skill_names() {
        let root = temp_dir();
        write_skill(&root, "group-a/code-review", "code-review", "Review A");
        write_skill(&root, "group-b/code-review", "code-review", "Review B");

        let inspection = inspect_skills(&root).unwrap();
        assert!(inspection.items.is_empty());
        assert_eq!(inspection.duplicate_names.len(), 1);
        assert!(inspection
            .duplicate_names
            .contains(&"code-review".to_string()));
        assert!(inspection
            .issues
            .iter()
            .any(|i| i.contains("duplicate skill name 'code-review'")));
    }
}
