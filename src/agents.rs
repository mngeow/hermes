use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::frontmatter::parse_agent_file;
use crate::models::{DiscoveredAgent, Inspection};

pub fn inspect_agents(source_root: &Path) -> Result<Inspection<DiscoveredAgent>> {
    let mut entries = fs::read_dir(source_root)
        .with_context(|| {
            format!(
                "failed to read agents source root {}",
                source_root.display()
            )
        })?
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    let mut inspection = Inspection::default();
    for entry in entries {
        let file_type = entry.file_type()?;
        if !file_type.is_file() {
            continue;
        }

        let path = entry.path();
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
    }

    Ok(inspection)
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
