use anyhow::{Result, bail};
use clap::Parser;

use crate::agents::inspect_agents;
use crate::cli::{Cli, Commands, InitArgs, ListArgs, ListTarget};
use crate::doctor;
use crate::fs_ops::ensure_workspace;
use crate::install;
use crate::manifest::{load_manifest, load_or_default, resolve_source_roots, save_manifest};
use crate::models::{ProjectPaths, SourceOverrides};
use crate::remove;
use crate::skills::inspect_skills;
use crate::sync;

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let project_root = std::env::current_dir()?;
    let paths = ProjectPaths::new(project_root);
    let overrides = SourceOverrides {
        skills: cli.skills_source.clone(),
        agents: cli.agents_source.clone(),
    };

    match cli.command {
        Commands::Init(args) => run_init(&paths, &overrides, args),
        Commands::Install(args) => install::run(&paths, &overrides, args),
        Commands::List(args) => run_list(&paths, &overrides, args),
        Commands::Sync(args) => sync::run(&paths, &overrides, args),
        Commands::Remove(args) => remove::run(&paths, args),
        Commands::Doctor => doctor::run(&paths, &overrides),
    }
}

fn run_init(paths: &ProjectPaths, overrides: &SourceOverrides, _args: InitArgs) -> Result<()> {
    let mut manifest = load_or_default(paths)?;
    let roots = resolve_source_roots(overrides, Some(&manifest))?;
    if roots.is_empty() {
        bail!(
            "hermes init requires at least one source root; pass --skills-source or --agents-source"
        )
    }

    ensure_workspace(
        &paths.opencode_dir,
        &paths.skills_dir,
        &paths.agents_dir,
        &paths.tmp_dir,
    )?;
    manifest.skills_source_root = roots.skills.clone();
    manifest.agents_source_root = roots.agents.clone();
    save_manifest(paths, &manifest)?;

    println!(
        "Initialized Hermes workspace at {}",
        paths.opencode_dir.display()
    );
    println!("Catalog updated: {}", paths.catalog_path.display());
    Ok(())
}

fn run_list(paths: &ProjectPaths, overrides: &SourceOverrides, args: ListArgs) -> Result<()> {
    let manifest = load_manifest(paths)?;

    if let Some(target) = args.available {
        let roots = resolve_source_roots(overrides, manifest.as_ref())?;
        match target {
            ListTarget::Skills => list_available_skills(roots.skills.as_deref())?,
            ListTarget::Agents => list_available_agents(roots.agents.as_deref())?,
            ListTarget::All => {
                list_available_skills(roots.skills.as_deref())?;
                println!();
                list_available_agents(roots.agents.as_deref())?;
            }
        }
        return Ok(());
    }

    let target = args.installed.unwrap_or(ListTarget::All);
    match manifest.as_ref() {
        Some(manifest) => match target {
            ListTarget::Skills => {
                println!("Installed skills:");
                if manifest.skills.is_empty() {
                    println!("(none)");
                } else {
                    for skill in &manifest.skills {
                        println!("- {}: {}", skill.name, skill.description);
                    }
                }
            }
            ListTarget::Agents => {
                println!("Installed agents:");
                if manifest.agents.is_empty() {
                    println!("(none)");
                } else {
                    for agent in &manifest.agents {
                        let suffix = agent
                            .mode
                            .map(|mode| format!(" ({mode})"))
                            .unwrap_or_default();
                        println!("- {}: {}{}", agent.name, agent.description, suffix);
                    }
                }
            }
            ListTarget::All => {
                println!("Installed skills:");
                if manifest.skills.is_empty() {
                    println!("(none)");
                } else {
                    for skill in &manifest.skills {
                        println!("- {}: {}", skill.name, skill.description);
                    }
                }
                println!();
                println!("Installed agents:");
                if manifest.agents.is_empty() {
                    println!("(none)");
                } else {
                    for agent in &manifest.agents {
                        let suffix = agent
                            .mode
                            .map(|mode| format!(" ({mode})"))
                            .unwrap_or_default();
                        println!("- {}: {}{}", agent.name, agent.description, suffix);
                    }
                }
            }
        },
        None => println!("No catalog found at {}", paths.catalog_path.display()),
    }

    Ok(())
}

fn list_available_skills(root: Option<&std::path::Path>) -> Result<()> {
    let root = root.ok_or_else(|| anyhow::anyhow!("no skills source root configured"))?;
    let inspection = inspect_skills(root)?;
    println!("Available skills:");
    if inspection.items.is_empty() {
        println!("(none)");
    } else {
        for skill in inspection.items {
            println!("- {}: {}", skill.name, skill.description);
        }
    }
    if !inspection.issues.is_empty() {
        println!();
        println!("Discovery warnings:");
        for issue in inspection.issues {
            println!("- {issue}");
        }
    }
    Ok(())
}

fn list_available_agents(root: Option<&std::path::Path>) -> Result<()> {
    let root = root.ok_or_else(|| anyhow::anyhow!("no agents source root configured"))?;
    let inspection = inspect_agents(root)?;
    println!("Available agents:");
    if inspection.items.is_empty() {
        println!("(none)");
    } else {
        for agent in inspection.items {
            let suffix = agent
                .mode
                .map(|mode| format!(" ({mode})"))
                .unwrap_or_default();
            println!("- {}: {}{}", agent.name, agent.description, suffix);
        }
    }
    if !inspection.issues.is_empty() {
        println!();
        println!("Discovery warnings:");
        for issue in inspection.issues {
            println!("- {issue}");
        }
    }
    Ok(())
}
