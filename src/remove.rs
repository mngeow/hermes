use anyhow::Result;

use crate::cli::{RemoveArgs, RemoveTarget};
use crate::fs_ops::remove_path_if_exists;
use crate::manifest::{load_manifest, save_manifest};
use crate::models::ProjectPaths;

pub fn run(paths: &ProjectPaths, args: RemoveArgs) -> Result<()> {
    let mut manifest = load_manifest(paths)?.ok_or_else(|| {
        anyhow::anyhow!(
            "no catalog found at {}; run hermes init or hermes install first",
            paths.catalog_path.display()
        )
    })?;

    match args.target {
        RemoveTarget::Skills(item) => {
            let index = manifest
                .skills
                .iter()
                .position(|skill| skill.name == item.name)
                .ok_or_else(|| {
                    anyhow::anyhow!("installed skill '{}' not found in catalog", item.name)
                })?;
            let entry = manifest.skills.remove(index);
            remove_path_if_exists(&paths.installed_path(&entry.installed_rel_path))?;
            println!("Removed skill {}", entry.name);
        }
        RemoveTarget::Agents(item) => {
            let index = manifest
                .agents
                .iter()
                .position(|agent| agent.name == item.name)
                .ok_or_else(|| {
                    anyhow::anyhow!("installed agent '{}' not found in catalog", item.name)
                })?;
            let entry = manifest.agents.remove(index);
            remove_path_if_exists(&paths.installed_path(&entry.installed_rel_path))?;
            println!("Removed agent {}", entry.name);
        }
    }
    save_manifest(paths, &manifest)?;
    println!("Manifest updated: {}", paths.catalog_path.display());
    Ok(())
}
