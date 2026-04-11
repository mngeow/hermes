use std::fs;

use anyhow::{Result, bail};

use crate::agents::{inspect_agents, validate_installed_agent};
use crate::manifest::{load_manifest, resolve_source_roots};
use crate::models::{ProjectPaths, SourceOverrides};
use crate::skills::{inspect_skills, validate_installed_skill};

pub fn run(paths: &ProjectPaths, overrides: &SourceOverrides) -> Result<()> {
    let manifest = load_manifest(paths)?;
    let roots = resolve_source_roots(overrides, manifest.as_ref())?;
    let mut issues = Vec::new();

    if roots.skills.is_none() && roots.agents.is_none() {
        issues.push("no source roots are configured".to_string());
    }

    if let Some(root) = roots.skills.as_deref() {
        if !root.exists() {
            issues.push(format!(
                "configured skills source root does not exist: {}",
                root.display()
            ));
        } else {
            let inspection = inspect_skills(root)?;
            issues.extend(inspection.issues);
        }
    }

    if let Some(root) = roots.agents.as_deref() {
        if !root.exists() {
            issues.push(format!(
                "configured agents source root does not exist: {}",
                root.display()
            ));
        } else {
            let inspection = inspect_agents(root)?;
            issues.extend(inspection.issues);
        }
    }

    match manifest.as_ref() {
        Some(manifest) => {
            for skill in &manifest.skills {
                let installed_path = paths.installed_path(&skill.installed_rel_path);
                if !installed_path.exists() {
                    issues.push(format!(
                        "installed skill path is missing: {}",
                        installed_path.display()
                    ));
                    continue;
                }
                if let Err(err) = validate_installed_skill(&installed_path) {
                    issues.push(format!(
                        "installed skill '{}' is invalid: {err:#}",
                        skill.name
                    ));
                }
            }

            for agent in &manifest.agents {
                let installed_path = paths.installed_path(&agent.installed_rel_path);
                if !installed_path.exists() {
                    issues.push(format!(
                        "installed agent path is missing: {}",
                        installed_path.display()
                    ));
                    continue;
                }
                if let Err(err) = validate_installed_agent(&installed_path) {
                    issues.push(format!(
                        "installed agent '{}' is invalid: {err:#}",
                        agent.name
                    ));
                }
            }

            if paths.skills_dir.exists() {
                let mut tracked = manifest
                    .skills
                    .iter()
                    .map(|skill| skill.name.as_str())
                    .collect::<Vec<_>>();
                tracked.sort_unstable();
                for entry in fs::read_dir(&paths.skills_dir)?.collect::<Result<Vec<_>, _>>()? {
                    if entry.file_type()?.is_dir() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if !tracked.iter().any(|tracked_name| *tracked_name == name) {
                            issues.push(format!(
                                "untracked installed skill directory: {}",
                                entry.path().display()
                            ));
                        }
                    }
                }
            }

            if paths.agents_dir.exists() {
                let mut tracked = manifest
                    .agents
                    .iter()
                    .map(|agent| agent.name.as_str())
                    .collect::<Vec<_>>();
                tracked.sort_unstable();
                for entry in fs::read_dir(&paths.agents_dir)?.collect::<Result<Vec<_>, _>>()? {
                    if entry.file_type()?.is_file()
                        && entry.path().extension().and_then(|ext| ext.to_str()) == Some("md")
                    {
                        let name = entry
                            .path()
                            .file_stem()
                            .and_then(|stem| stem.to_str())
                            .unwrap_or_default()
                            .to_string();
                        if !tracked.iter().any(|tracked_name| *tracked_name == name) {
                            issues.push(format!(
                                "untracked installed agent file: {}",
                                entry.path().display()
                            ));
                        }
                    }
                }
            }
        }
        None => issues.push(format!(
            "catalog file is missing: {}",
            paths.catalog_path.display()
        )),
    }

    if issues.is_empty() {
        println!("Doctor found no issues.");
        return Ok(());
    }

    for issue in &issues {
        eprintln!("- {issue}");
    }
    bail!("doctor found {} issue(s)", issues.len())
}
