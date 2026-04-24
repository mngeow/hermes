use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::frontmatter::parse_command_file;
use crate::models::{DiscoveredCommand, Inspection};

pub fn inspect_commands(source_root: &Path) -> Result<Inspection<DiscoveredCommand>> {
    let mut inspection = Inspection::default();
    walk_commands(source_root, source_root, &mut inspection)?;
    deduplicate_commands(&mut inspection);
    Ok(inspection)
}

fn deduplicate_commands(inspection: &mut Inspection<DiscoveredCommand>) {
    use std::collections::HashMap;

    let mut counts: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for command in &inspection.items {
        counts
            .entry(command.name.clone())
            .or_default()
            .push(command.source_rel_path.clone());
    }

    let mut duplicates = Vec::new();
    inspection.items.retain(|command| {
        let paths = counts.get(&command.name).expect("count exists");
        if paths.len() > 1 {
            duplicates.push((command.name.clone(), paths.clone()));
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
                "duplicate command name '{name}' found at: {}",
                paths
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }
}

fn walk_commands(
    source_root: &Path,
    dir: &Path,
    inspection: &mut Inspection<DiscoveredCommand>,
) -> Result<()> {
    let mut entries = fs::read_dir(dir)
        .with_context(|| format!("failed to read commands directory {}", dir.display()))?
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let file_type = entry.file_type()?;
        let path = entry.path();
        if file_type.is_file() {
            if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                continue;
            }
            match parse_command_markdown(&path, Some(source_root)) {
                Ok(command) => inspection.items.push(command),
                Err(err) => inspection.issues.push(format!(
                    "invalid command '{}': {err:#}",
                    entry.file_name().to_string_lossy()
                )),
            }
        } else if file_type.is_dir() {
            walk_commands(source_root, &path, inspection)?;
        }
    }

    Ok(())
}

pub fn validate_installed_command(path: &Path) -> Result<DiscoveredCommand> {
    parse_command_markdown(path, None)
}

fn parse_command_markdown(path: &Path, source_root: Option<&Path>) -> Result<DiscoveredCommand> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("command file has no valid UTF-8 name"))?;
    let stem = path
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("command file has no valid UTF-8 stem"))?;

    let frontmatter = parse_command_file(path)?;

    let source_rel_path = match source_root {
        Some(root) => path.strip_prefix(root)?.to_path_buf(),
        None => PathBuf::from(file_name),
    };

    Ok(DiscoveredCommand {
        name: stem.to_string(),
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
            "hermes-commands-test-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::SeqCst)
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn write_command(root: &Path, rel_path: &str, body: &str) {
        let path = root.join(rel_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        let mut file = std::fs::File::create(&path).unwrap();
        write!(file, "---\n---\n{}", body).unwrap();
    }

    #[test]
    fn discovers_flat_commands() {
        let root = temp_dir();
        write_command(&root, "test.md", "# Test command\n");
        write_command(&root, "review.md", "# Review command\n");

        let inspection = inspect_commands(&root).unwrap();
        assert_eq!(inspection.items.len(), 2);
        assert!(inspection.issues.is_empty());
        assert!(inspection.duplicate_names.is_empty());

        let names: Vec<_> = inspection.items.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"test"));
        assert!(names.contains(&"review"));
    }

    #[test]
    fn discovers_grouped_commands() {
        let root = temp_dir();
        write_command(&root, "git/review-changes.md", "# Git review\n");
        write_command(&root, "project/test.md", "# Project test\n");

        let inspection = inspect_commands(&root).unwrap();
        assert_eq!(inspection.items.len(), 2);
        assert!(inspection.issues.is_empty());

        let paths: Vec<_> = inspection
            .items
            .iter()
            .map(|c| c.source_rel_path.to_string_lossy().to_string())
            .collect();
        assert!(paths.contains(&"git/review-changes.md".to_string()));
        assert!(paths.contains(&"project/test.md".to_string()));
    }

    #[test]
    fn detects_duplicate_command_names() {
        let root = temp_dir();
        write_command(&root, "group-a/test.md", "# Test A\n");
        write_command(&root, "group-b/test.md", "# Test B\n");

        let inspection = inspect_commands(&root).unwrap();
        assert!(inspection.items.is_empty());
        assert_eq!(inspection.duplicate_names.len(), 1);
        assert!(inspection.duplicate_names.contains(&"test".to_string()));
        assert!(inspection
            .issues
            .iter()
            .any(|i| i.contains("duplicate command name 'test'")));
    }
}
