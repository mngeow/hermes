use anyhow::{bail, Result};

use crate::cli::SyncArgs;
use crate::fs_ops::{atomic_install_agent, atomic_install_command, atomic_install_skill};
use crate::hashing::{hash_agent_file, hash_command_file, hash_skill_dir};
use crate::manifest::{load_manifest, resolve_source_roots, save_manifest};
use crate::models::{ProjectPaths, SourceOverrides};

pub fn run(paths: &ProjectPaths, overrides: &SourceOverrides, args: SyncArgs) -> Result<()> {
    let mut manifest = load_manifest(paths)?.ok_or_else(|| {
        anyhow::anyhow!(
            "no catalog found at {}; run hermes init or hermes install first",
            paths.catalog_path.display()
        )
    })?;
    let roots = resolve_source_roots(overrides, Some(&manifest))?;

    let sync_skills = args.skills || (!args.skills && !args.agents && !args.commands);
    let sync_agents = args.agents || (!args.skills && !args.agents && !args.commands);
    let sync_commands = args.commands || (!args.skills && !args.agents && !args.commands);

    if sync_skills {
        match roots.skills.as_deref() {
            Some(root) => {
                // No longer store source root in manifest
                for skill in &mut manifest.skills {
                    let source_path = root.join(&skill.source_rel_path);
                    let installed_path = paths.installed_path(&skill.installed_rel_path);

                    if !source_path.exists() {
                        eprintln!(
                            "Skipping {}\nKind: skill\nReason: source skill no longer exists",
                            skill.name
                        );
                        continue;
                    }
                    if !installed_path.exists() {
                        eprintln!(
                            "Skipping {}\nKind: skill\nReason: installed skill path is missing",
                            skill.name
                        );
                        continue;
                    }

                    let current_installed_hash = hash_skill_dir(&installed_path)?;
                    if !args.force && current_installed_hash != skill.installed_hash {
                        eprintln!(
                            "Skipping {}\nKind: skill\nReason: local installed copy differs from the last recorded manifest hash",
                            skill.name
                        );
                        continue;
                    }

                    let current_source_hash = hash_skill_dir(&source_path)?;
                    if current_source_hash == skill.source_hash
                        && current_installed_hash == skill.installed_hash
                    {
                        continue;
                    }

                    atomic_install_skill(&source_path, &installed_path, &paths.tmp_dir)?;
                    skill.source_hash = current_source_hash;
                    skill.installed_hash = hash_skill_dir(&installed_path)?;
                    println!("Synced skill {}", skill.name);
                }
            }
            None if args.skills => {
                bail!("no skills source root configured; pass --skills-source or run hermes configure")
            }
            None => println!("Skipping skills: no skills source root configured"),
        }
    }

    if sync_agents {
        match roots.agents.as_deref() {
            Some(root) => {
                // No longer store source root in manifest
                for agent in &mut manifest.agents {
                    let source_path = root.join(&agent.source_rel_path);
                    let installed_path = paths.installed_path(&agent.installed_rel_path);

                    if !source_path.exists() {
                        eprintln!(
                            "Skipping {}\nKind: agent\nReason: source agent no longer exists",
                            agent.name
                        );
                        continue;
                    }
                    if !installed_path.exists() {
                        eprintln!(
                            "Skipping {}\nKind: agent\nReason: installed agent path is missing",
                            agent.name
                        );
                        continue;
                    }

                    let current_installed_hash = hash_agent_file(&installed_path)?;
                    if !args.force && current_installed_hash != agent.installed_hash {
                        eprintln!(
                            "Skipping {}\nKind: agent\nReason: local installed copy differs from the last recorded manifest hash",
                            agent.name
                        );
                        continue;
                    }

                    let current_source_hash = hash_agent_file(&source_path)?;
                    if current_source_hash == agent.source_hash
                        && current_installed_hash == agent.installed_hash
                    {
                        continue;
                    }

                    atomic_install_agent(&source_path, &installed_path, &paths.tmp_dir)?;
                    agent.source_hash = current_source_hash;
                    agent.installed_hash = hash_agent_file(&installed_path)?;
                    println!("Synced agent {}", agent.name);
                }
            }
            None if args.agents => {
                bail!("no agents source root configured; pass --agents-source or run hermes configure")
            }
            None => println!("Skipping agents: no agents source root configured"),
        }
    }

    if sync_commands {
        match roots.commands.as_deref() {
            Some(root) => {
                // No longer store source root in manifest
                for command in &mut manifest.commands {
                    let source_path = root.join(&command.source_rel_path);
                    let installed_path = paths.installed_path(&command.installed_rel_path);

                    if !source_path.exists() {
                        eprintln!(
                            "Skipping {}\nKind: command\nReason: source command no longer exists",
                            command.name
                        );
                        continue;
                    }
                    if !installed_path.exists() {
                        eprintln!(
                            "Skipping {}\nKind: command\nReason: installed command path is missing",
                            command.name
                        );
                        continue;
                    }

                    let current_installed_hash = hash_command_file(&installed_path)?;
                    if !args.force && current_installed_hash != command.installed_hash {
                        eprintln!(
                            "Skipping {}\nKind: command\nReason: local installed copy differs from the last recorded manifest hash",
                            command.name
                        );
                        continue;
                    }

                    let current_source_hash = hash_command_file(&source_path)?;
                    if current_source_hash == command.source_hash
                        && current_installed_hash == command.installed_hash
                    {
                        continue;
                    }

                    atomic_install_command(&source_path, &installed_path, &paths.tmp_dir)?;
                    command.source_hash = current_source_hash;
                    command.installed_hash = hash_command_file(&installed_path)?;
                    println!("Synced command {}", command.name);
                }
            }
            None if args.commands => {
                bail!("no commands source root configured; pass --commands-source or run hermes configure")
            }
            None => println!("Skipping commands: no commands source root configured"),
        }
    }

    save_manifest(paths, &manifest)?;
    println!("Manifest updated: {}", paths.catalog_path.display());
    Ok(())
}
