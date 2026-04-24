use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::frontmatter::parse_agent_file;
use crate::models::{DiscoveredAgent, Inspection};

pub fn inspect_agents(source_root: &Path) -> Result<Inspection<DiscoveredAgent>> {
    let mut inspection = Inspection::default();
    walk_agents(source_root, source_root, &mut inspection)?;
    deduplicate_agents(&mut inspection);
    Ok(inspection)
}

fn deduplicate_agents(inspection: &mut Inspection<DiscoveredAgent>) {
    use std::collections::HashMap;

    let mut counts: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for agent in &inspection.items {
        counts
            .entry(agent.name.clone())
            .or_default()
            .push(agent.source_rel_path.clone());
    }

    let mut duplicates = Vec::new();
    inspection.items.retain(|agent| {
        let paths = counts.get(&agent.name).expect("count exists");
        if paths.len() > 1 {
            duplicates.push((agent.name.clone(), paths.clone()));
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
                "duplicate agent name '{name}' found at: {}",
                paths
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }
}

fn walk_agents(
    source_root: &Path,
    dir: &Path,
    inspection: &mut Inspection<DiscoveredAgent>,
) -> Result<()> {
    let mut entries = fs::read_dir(dir)
        .with_context(|| format!("failed to read agents directory {}", dir.display()))?
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let file_type = entry.file_type()?;
        let path = entry.path();
        if file_type.is_file() {
            if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                continue;
            }
            match parse_agent_markdown(&path, Some(source_root)) {
                Ok(agent) => inspection.items.push(agent),
                Err(err) => inspection.issues.push(format!(
                    "invalid agent '{}': {err:#}",
                    entry.file_name().to_string_lossy()
                )),
            }
        } else if file_type.is_dir() {
            walk_agents(source_root, &path, inspection)?;
        }
    }

    Ok(())
}

pub fn validate_installed_agent(path: &Path) -> Result<DiscoveredAgent> {
    parse_agent_markdown(path, None)
}

fn parse_agent_markdown(path: &Path, source_root: Option<&Path>) -> Result<DiscoveredAgent> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("agent file has no valid UTF-8 name"))?;
    let stem = path
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("agent file has no valid UTF-8 stem"))?;

    let frontmatter = parse_agent_file(path)?;
    if frontmatter.has_external_prompt {
        bail!("multi-file agents using prompt: {{file:...}} are unsupported in v1")
    }
    if !frontmatter.has_body {
        bail!("agent prompt body must be contained in the markdown file")
    }

    let source_rel_path = match source_root {
        Some(root) => path.strip_prefix(root)?.to_path_buf(),
        None => PathBuf::from(file_name),
    };

    Ok(DiscoveredAgent {
        name: stem.to_string(),
        description: frontmatter.description,
        mode: frontmatter.mode,
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
            "hermes-agents-test-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::SeqCst)
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    fn write_agent(root: &Path, rel_path: &str, name: &str, description: &str) {
        let path = root.join(rel_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        let mut file = std::fs::File::create(&path).unwrap();
        write!(file, "---\ndescription: {description}\n---\n# {name}\n").unwrap();
    }

    #[test]
    fn discovers_flat_agents() {
        let root = temp_dir();
        write_agent(&root, "review.md", "review", "Review agent");
        write_agent(&root, "test.md", "test", "Test agent");

        let inspection = inspect_agents(&root).unwrap();
        assert_eq!(inspection.items.len(), 2);
        assert!(inspection.issues.is_empty());
        assert!(inspection.duplicate_names.is_empty());

        let names: Vec<_> = inspection.items.iter().map(|a| a.name.as_str()).collect();
        assert!(names.contains(&"review"));
        assert!(names.contains(&"test"));
    }

    #[test]
    fn discovers_grouped_agents() {
        let root = temp_dir();
        write_agent(&root, "review/review.md", "review", "Review agent");
        write_agent(&root, "quality/security.md", "security", "Security agent");

        let inspection = inspect_agents(&root).unwrap();
        assert_eq!(inspection.items.len(), 2);
        assert!(inspection.issues.is_empty());

        let paths: Vec<_> = inspection
            .items
            .iter()
            .map(|a| a.source_rel_path.to_string_lossy().to_string())
            .collect();
        assert!(paths.contains(&"review/review.md".to_string()));
        assert!(paths.contains(&"quality/security.md".to_string()));
    }

    #[test]
    fn detects_duplicate_agent_names() {
        let root = temp_dir();
        write_agent(&root, "group-a/review.md", "review", "Review A");
        write_agent(&root, "group-b/review.md", "review", "Review B");

        let inspection = inspect_agents(&root).unwrap();
        assert!(inspection.items.is_empty());
        assert_eq!(inspection.duplicate_names.len(), 1);
        assert!(inspection.duplicate_names.contains(&"review".to_string()));
        assert!(inspection
            .issues
            .iter()
            .any(|i| i.contains("duplicate agent name 'review'")));
    }
}
