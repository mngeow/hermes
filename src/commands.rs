use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::frontmatter::parse_command_file;
use crate::models::{DiscoveredCommand, Inspection};

pub fn inspect_commands(source_root: &Path) -> Result<Inspection<DiscoveredCommand>> {
    let mut entries = fs::read_dir(source_root)
        .with_context(|| {
            format!(
                "failed to read commands source root {}",
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

        match parse_command_markdown(&path, Some(source_root)) {
            Ok(command) => inspection.items.push(command),
            Err(err) => inspection.issues.push(format!(
                "invalid command '{}': {err:#}",
                entry.file_name().to_string_lossy()
            )),
        }
    }

    Ok(inspection)
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
