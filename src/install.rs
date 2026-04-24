use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, IsTerminal};
use std::path::PathBuf;

use anyhow::{bail, Result};
use dialoguer::{theme::ColorfulTheme, MultiSelect};

use crate::agents::inspect_agents;
use crate::cli::{InstallArgs, InstallTarget};
use crate::commands::inspect_commands;
use crate::fs_ops::{
    atomic_install_agent, atomic_install_command, atomic_install_skill, ensure_workspace,
};
use crate::hashing::{hash_agent_file, hash_command_file, hash_skill_dir};
use crate::manifest::{load_or_default, resolve_source_roots, save_manifest};
use crate::models::{
    CatalogManifest, DiscoveredAgent, DiscoveredCommand, DiscoveredSkill, InstalledAgent,
    InstalledCommand, InstalledSkill, ProjectPaths, SourceOverrides,
};
use crate::skills::inspect_skills;
use crate::tui::run_interactive_selection;

pub fn run(paths: &ProjectPaths, overrides: &SourceOverrides, args: InstallArgs) -> Result<()> {
    let mut manifest = load_or_default(paths)?;
    let roots = resolve_source_roots(overrides, Some(&manifest))?;

    // Bootstrap workspace if needed (no prior init required)
    ensure_workspace(
        &paths.opencode_dir,
        &paths.skills_dir,
        &paths.agents_dir,
        &paths.commands_dir,
        &paths.tmp_dir,
    )?;

    let selection = InstallSelection::from_args(&args)?;

    let mut installed_skills = Vec::new();
    let mut installed_agents = Vec::new();
    let mut installed_commands = Vec::new();
    let mut skipped = Vec::new();
    let mut notes = Vec::new();

    if selection.skills.enabled() || selection.agents.enabled() || selection.commands.enabled() {
        match resolve_interactive_selection(&selection, &roots)? {
            Some(result) => {
                if !result.selected_skills.is_empty() {
                    if let Some(root) = roots.skills.as_deref() {
                        let inspection = inspect_skills(root)?;
                        for issue in &inspection.issues {
                            eprintln!("Warning: {issue}");
                        }
                        let selected = select_named_skills(
                            &result.selected_skills,
                            inspection.items,
                            &inspection.duplicate_names,
                        )?;
                        for skill in selected {
                            match install_skill(paths, &mut manifest, &skill, args.force)? {
                                InstallOutcome::Installed(name) => installed_skills.push(name),
                                InstallOutcome::Skipped(name, reason) => skipped
                                    .push(format!("Skipped {name}\nKind: skill\nReason: {reason}")),
                            }
                        }
                    }
                }
                if !result.selected_agents.is_empty() {
                    if let Some(root) = roots.agents.as_deref() {
                        let inspection = inspect_agents(root)?;
                        for issue in &inspection.issues {
                            eprintln!("Warning: {issue}");
                        }
                        let selected = select_named_agents(
                            &result.selected_agents,
                            inspection.items,
                            &inspection.duplicate_names,
                        )?;
                        for agent in selected {
                            match install_agent(paths, &mut manifest, &agent, args.force)? {
                                InstallOutcome::Installed(name) => installed_agents.push(name),
                                InstallOutcome::Skipped(name, reason) => skipped
                                    .push(format!("Skipped {name}\nKind: agent\nReason: {reason}")),
                            }
                        }
                    }
                }
                if !result.selected_commands.is_empty() {
                    if let Some(root) = roots.commands.as_deref() {
                        let inspection = inspect_commands(root)?;
                        for issue in &inspection.issues {
                            eprintln!("Warning: {issue}");
                        }
                        let selected = select_named_commands(
                            &result.selected_commands,
                            inspection.items,
                            &inspection.duplicate_names,
                        )?;
                        for command in selected {
                            match install_command(paths, &mut manifest, &command, args.force)? {
                                InstallOutcome::Installed(name) => installed_commands.push(name),
                                InstallOutcome::Skipped(name, reason) => skipped.push(format!(
                                    "Skipped {name}\nKind: command\nReason: {reason}"
                                )),
                            }
                        }
                    }
                }
            }
            None => {
                notes.push("No artifacts selected for installation".to_string());
            }
        }
    }

    save_manifest(paths, &manifest)?;
    print_summary(
        paths,
        &installed_skills,
        &installed_agents,
        &installed_commands,
        &skipped,
        &notes,
    );
    Ok(())
}

enum InstallOutcome {
    Installed(String),
    Skipped(String, String),
}

#[derive(Debug, Clone)]
struct SelectedArtifacts {
    selected_skills: Vec<String>,
    selected_agents: Vec<String>,
    selected_commands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum RequestedArtifacts {
    Skip,
    Prompt,
    Named(Vec<String>),
}

impl RequestedArtifacts {
    fn enabled(&self) -> bool {
        !matches!(self, Self::Skip)
    }
}

struct InstallSelection {
    skills: RequestedArtifacts,
    agents: RequestedArtifacts,
    commands: RequestedArtifacts,
}

impl InstallSelection {
    fn from_args(args: &InstallArgs) -> Result<Self> {
        if args.target.is_some()
            && (!args.skills.is_empty() || !args.agents.is_empty() || !args.commands.is_empty())
        {
            bail!("cannot combine --skills/--agents/--commands with install kind subcommands")
        }

        let selection = match &args.target {
            Some(InstallTarget::Skills(names)) => Self {
                skills: if names.names.is_empty() {
                    RequestedArtifacts::Prompt
                } else {
                    RequestedArtifacts::Named(names.names.clone())
                },
                agents: RequestedArtifacts::Skip,
                commands: RequestedArtifacts::Skip,
            },
            Some(InstallTarget::Agents(names)) => Self {
                skills: RequestedArtifacts::Skip,
                agents: if names.names.is_empty() {
                    RequestedArtifacts::Prompt
                } else {
                    RequestedArtifacts::Named(names.names.clone())
                },
                commands: RequestedArtifacts::Skip,
            },
            Some(InstallTarget::Commands(names)) => Self {
                skills: RequestedArtifacts::Skip,
                agents: RequestedArtifacts::Skip,
                commands: if names.names.is_empty() {
                    RequestedArtifacts::Prompt
                } else {
                    RequestedArtifacts::Named(names.names.clone())
                },
            },
            None if args.skills.is_empty()
                && args.agents.is_empty()
                && args.commands.is_empty() =>
            {
                Self {
                    skills: RequestedArtifacts::Prompt,
                    agents: RequestedArtifacts::Prompt,
                    commands: RequestedArtifacts::Prompt,
                }
            }
            None => Self {
                skills: if args.skills.is_empty() {
                    RequestedArtifacts::Skip
                } else {
                    RequestedArtifacts::Named(args.skills.clone())
                },
                agents: if args.agents.is_empty() {
                    RequestedArtifacts::Skip
                } else {
                    RequestedArtifacts::Named(args.agents.clone())
                },
                commands: if args.commands.is_empty() {
                    RequestedArtifacts::Skip
                } else {
                    RequestedArtifacts::Named(args.commands.clone())
                },
            },
        };

        Ok(selection)
    }
}

fn resolve_interactive_selection(
    selection: &InstallSelection,
    roots: &crate::models::SourceRoots,
) -> Result<Option<SelectedArtifacts>> {
    let use_tui = selection.skills == RequestedArtifacts::Prompt
        && selection.agents == RequestedArtifacts::Prompt
        && selection.commands == RequestedArtifacts::Prompt;

    if !use_tui {
        let skills = resolve_skill_selection(&selection.skills, roots)?;
        let agents = resolve_agent_selection(&selection.agents, roots)?;
        let commands = resolve_command_selection(&selection.commands, roots)?;
        return Ok(Some(SelectedArtifacts {
            selected_skills: skills.into_iter().map(|s| s.name).collect(),
            selected_agents: agents.into_iter().map(|a| a.name).collect(),
            selected_commands: commands.into_iter().map(|c| c.name).collect(),
        }));
    }

    if !io::stdin().is_terminal() {
        bail!(
            "Interactive selection requires a terminal. \
             Provide explicit artifact names or run in a TTY."
        );
    }

    let skills: Vec<DiscoveredSkill> = match roots.skills.as_deref() {
        Some(root) => {
            let inspection = inspect_skills(root)?;
            for issue in &inspection.issues {
                eprintln!("Warning: {issue}");
            }
            inspection.items
        }
        None => Vec::new(),
    };

    let agents: Vec<DiscoveredAgent> = match roots.agents.as_deref() {
        Some(root) => {
            let inspection = inspect_agents(root)?;
            for issue in &inspection.issues {
                eprintln!("Warning: {issue}");
            }
            inspection.items
        }
        None => Vec::new(),
    };

    let commands: Vec<DiscoveredCommand> = match roots.commands.as_deref() {
        Some(root) => {
            let inspection = inspect_commands(root)?;
            for issue in &inspection.issues {
                eprintln!("Warning: {issue}");
            }
            inspection.items
        }
        None => Vec::new(),
    };

    if skills.is_empty() && agents.is_empty() && commands.is_empty() {
        return Ok(Some(SelectedArtifacts {
            selected_skills: Vec::new(),
            selected_agents: Vec::new(),
            selected_commands: Vec::new(),
        }));
    }

    match run_interactive_selection(skills, agents, commands)? {
        Some(result) => Ok(Some(SelectedArtifacts {
            selected_skills: result.selected_skills,
            selected_agents: result.selected_agents,
            selected_commands: result.selected_commands,
        })),
        None => Ok(None),
    }
}

fn resolve_skill_selection(
    request: &RequestedArtifacts,
    roots: &crate::models::SourceRoots,
) -> Result<Vec<DiscoveredSkill>> {
    match request {
        RequestedArtifacts::Skip => Ok(Vec::new()),
        RequestedArtifacts::Prompt => {
            let root = roots
                .skills
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("no skills source root configured"))?;
            let inspection = inspect_skills(root)?;
            for issue in &inspection.issues {
                eprintln!("Warning: {issue}");
            }
            prompt_skills(inspection.items)
        }
        RequestedArtifacts::Named(names) => {
            let root = roots
                .skills
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("no skills source root configured"))?;
            let inspection = inspect_skills(root)?;
            select_named_skills(names, inspection.items, &inspection.duplicate_names)
        }
    }
}

fn resolve_agent_selection(
    request: &RequestedArtifacts,
    roots: &crate::models::SourceRoots,
) -> Result<Vec<DiscoveredAgent>> {
    match request {
        RequestedArtifacts::Skip => Ok(Vec::new()),
        RequestedArtifacts::Prompt => {
            let root = roots
                .agents
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("no agents source root configured"))?;
            let inspection = inspect_agents(root)?;
            for issue in &inspection.issues {
                eprintln!("Warning: {issue}");
            }
            prompt_agents(inspection.items)
        }
        RequestedArtifacts::Named(names) => {
            let root = roots
                .agents
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("no agents source root configured"))?;
            let inspection = inspect_agents(root)?;
            select_named_agents(names, inspection.items, &inspection.duplicate_names)
        }
    }
}

fn resolve_command_selection(
    request: &RequestedArtifacts,
    roots: &crate::models::SourceRoots,
) -> Result<Vec<DiscoveredCommand>> {
    match request {
        RequestedArtifacts::Skip => Ok(Vec::new()),
        RequestedArtifacts::Prompt => {
            let root = roots
                .commands
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("no commands source root configured"))?;
            let inspection = inspect_commands(root)?;
            for issue in &inspection.issues {
                eprintln!("Warning: {issue}");
            }
            prompt_commands(inspection.items)
        }
        RequestedArtifacts::Named(names) => {
            let root = roots
                .commands
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("no commands source root configured"))?;
            let inspection = inspect_commands(root)?;
            select_named_commands(names, inspection.items, &inspection.duplicate_names)
        }
    }
}

fn select_named_skills(
    names: &[String],
    discovered: Vec<DiscoveredSkill>,
    duplicate_names: &[String],
) -> Result<Vec<DiscoveredSkill>> {
    let map = discovered
        .into_iter()
        .map(|skill| (skill.name.clone(), skill))
        .collect::<BTreeMap<_, _>>();
    let mut selected = Vec::new();
    for name in dedupe(names) {
        if duplicate_names.contains(&name) {
            bail!(
                "ambiguous skill '{name}': multiple artifacts share this name; \
                 rename or regroup the colliding source artifacts"
            );
        }
        let skill = map
            .get(&name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("unknown skill '{name}'"))?;
        selected.push(skill);
    }
    Ok(selected)
}

fn select_named_agents(
    names: &[String],
    discovered: Vec<DiscoveredAgent>,
    duplicate_names: &[String],
) -> Result<Vec<DiscoveredAgent>> {
    let map = discovered
        .into_iter()
        .map(|agent| (agent.name.clone(), agent))
        .collect::<BTreeMap<_, _>>();
    let mut selected = Vec::new();
    for name in dedupe(names) {
        if duplicate_names.contains(&name) {
            bail!(
                "ambiguous agent '{name}': multiple artifacts share this name; \
                 rename or regroup the colliding source artifacts"
            );
        }
        let agent = map
            .get(&name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("unknown agent '{name}'"))?;
        selected.push(agent);
    }
    Ok(selected)
}

fn select_named_commands(
    names: &[String],
    discovered: Vec<DiscoveredCommand>,
    duplicate_names: &[String],
) -> Result<Vec<DiscoveredCommand>> {
    let map = discovered
        .into_iter()
        .map(|command| (command.name.clone(), command))
        .collect::<BTreeMap<_, _>>();
    let mut selected = Vec::new();
    for name in dedupe(names) {
        if duplicate_names.contains(&name) {
            bail!(
                "ambiguous command '{name}': multiple artifacts share this name; \
                 rename or regroup the colliding source artifacts"
            );
        }
        let command = map
            .get(&name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("unknown command '{name}'"))?;
        selected.push(command);
    }
    Ok(selected)
}

fn prompt_skills(discovered: Vec<DiscoveredSkill>) -> Result<Vec<DiscoveredSkill>> {
    if discovered.is_empty() {
        return Ok(Vec::new());
    }

    let labels = discovered
        .iter()
        .map(|skill| format!("{:<24} {}", skill.name, skill.description))
        .collect::<Vec<_>>();
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select skills to install")
        .items(&labels)
        .interact()?;
    Ok(selection
        .into_iter()
        .map(|index| discovered[index].clone())
        .collect())
}

fn prompt_agents(discovered: Vec<DiscoveredAgent>) -> Result<Vec<DiscoveredAgent>> {
    if discovered.is_empty() {
        return Ok(Vec::new());
    }

    let labels = discovered
        .iter()
        .map(|agent| {
            let suffix = agent
                .mode
                .map(|mode| format!(" ({mode})"))
                .unwrap_or_default();
            format!("{:<24} {}{}", agent.name, agent.description, suffix)
        })
        .collect::<Vec<_>>();
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select agents to install")
        .items(&labels)
        .interact()?;
    Ok(selection
        .into_iter()
        .map(|index| discovered[index].clone())
        .collect())
}

fn prompt_commands(discovered: Vec<DiscoveredCommand>) -> Result<Vec<DiscoveredCommand>> {
    if discovered.is_empty() {
        return Ok(Vec::new());
    }

    let labels = discovered
        .iter()
        .map(|command| {
            let desc = command.description.as_deref().unwrap_or("");
            format!("{:<24} {}", command.name, desc)
        })
        .collect::<Vec<_>>();
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select commands to install")
        .items(&labels)
        .interact()?;
    Ok(selection
        .into_iter()
        .map(|index| discovered[index].clone())
        .collect())
}

fn install_skill(
    paths: &ProjectPaths,
    manifest: &mut CatalogManifest,
    skill: &DiscoveredSkill,
    force: bool,
) -> Result<InstallOutcome> {
    let installed_rel_path = PathBuf::from("skills").join(&skill.name);
    let installed_path = paths.installed_path(&installed_rel_path);
    let source_hash = hash_skill_dir(&skill.source_path)?;

    if installed_path.exists() && !force {
        match manifest
            .skills
            .iter()
            .find(|entry| entry.name == skill.name)
        {
            Some(existing) => {
                let current_hash = hash_skill_dir(&installed_path)?;
                if current_hash != existing.installed_hash {
                    return Ok(InstallOutcome::Skipped(
                        skill.name.clone(),
                        "local installed copy differs from the last recorded manifest hash"
                            .to_string(),
                    ));
                }
            }
            None => {
                return Ok(InstallOutcome::Skipped(
                    skill.name.clone(),
                    "an untracked local skill already exists at the install path".to_string(),
                ));
            }
        }
    }

    atomic_install_skill(&skill.source_path, &installed_path, &paths.tmp_dir)?;
    let installed_hash = hash_skill_dir(&installed_path)?;
    let entry = InstalledSkill {
        name: skill.name.clone(),
        description: skill.description.clone(),
        source_rel_path: skill.source_rel_path.clone(),
        installed_rel_path,
        source_hash,
        installed_hash,
    };
    upsert_skill(manifest, entry);
    Ok(InstallOutcome::Installed(skill.name.clone()))
}

fn install_agent(
    paths: &ProjectPaths,
    manifest: &mut CatalogManifest,
    agent: &DiscoveredAgent,
    force: bool,
) -> Result<InstallOutcome> {
    let installed_rel_path = PathBuf::from("agents").join(format!("{}.md", agent.name));
    let installed_path = paths.installed_path(&installed_rel_path);
    let source_hash = hash_agent_file(&agent.source_path)?;

    if installed_path.exists() && !force {
        match manifest
            .agents
            .iter()
            .find(|entry| entry.name == agent.name)
        {
            Some(existing) => {
                let current_hash = hash_agent_file(&installed_path)?;
                if current_hash != existing.installed_hash {
                    return Ok(InstallOutcome::Skipped(
                        agent.name.clone(),
                        "local installed copy differs from the last recorded manifest hash"
                            .to_string(),
                    ));
                }
            }
            None => {
                return Ok(InstallOutcome::Skipped(
                    agent.name.clone(),
                    "an untracked local agent already exists at the install path".to_string(),
                ));
            }
        }
    }

    atomic_install_agent(&agent.source_path, &installed_path, &paths.tmp_dir)?;
    let installed_hash = hash_agent_file(&installed_path)?;
    let entry = InstalledAgent {
        name: agent.name.clone(),
        description: agent.description.clone(),
        mode: agent.mode,
        source_rel_path: agent.source_rel_path.clone(),
        installed_rel_path,
        source_hash,
        installed_hash,
    };
    upsert_agent(manifest, entry);
    Ok(InstallOutcome::Installed(agent.name.clone()))
}

fn install_command(
    paths: &ProjectPaths,
    manifest: &mut CatalogManifest,
    command: &DiscoveredCommand,
    force: bool,
) -> Result<InstallOutcome> {
    let installed_rel_path = PathBuf::from("commands").join(format!("{}.md", command.name));
    let installed_path = paths.installed_path(&installed_rel_path);
    let source_hash = hash_command_file(&command.source_path)?;

    if installed_path.exists() && !force {
        match manifest
            .commands
            .iter()
            .find(|entry| entry.name == command.name)
        {
            Some(existing) => {
                let current_hash = hash_command_file(&installed_path)?;
                if current_hash != existing.installed_hash {
                    return Ok(InstallOutcome::Skipped(
                        command.name.clone(),
                        "local installed copy differs from the last recorded manifest hash"
                            .to_string(),
                    ));
                }
            }
            None => {
                return Ok(InstallOutcome::Skipped(
                    command.name.clone(),
                    "an untracked local command already exists at the install path".to_string(),
                ));
            }
        }
    }

    atomic_install_command(&command.source_path, &installed_path, &paths.tmp_dir)?;
    let installed_hash = hash_command_file(&installed_path)?;
    let entry = InstalledCommand {
        name: command.name.clone(),
        description: command.description.clone(),
        source_rel_path: command.source_rel_path.clone(),
        installed_rel_path,
        source_hash,
        installed_hash,
    };
    upsert_command(manifest, entry);
    Ok(InstallOutcome::Installed(command.name.clone()))
}

fn upsert_skill(manifest: &mut CatalogManifest, entry: InstalledSkill) {
    if let Some(existing) = manifest
        .skills
        .iter_mut()
        .find(|item| item.name == entry.name)
    {
        *existing = entry;
    } else {
        manifest.skills.push(entry);
        manifest
            .skills
            .sort_by(|left, right| left.name.cmp(&right.name));
    }
}

fn upsert_agent(manifest: &mut CatalogManifest, entry: InstalledAgent) {
    if let Some(existing) = manifest
        .agents
        .iter_mut()
        .find(|item| item.name == entry.name)
    {
        *existing = entry;
    } else {
        manifest.agents.push(entry);
        manifest
            .agents
            .sort_by(|left, right| left.name.cmp(&right.name));
    }
}

fn upsert_command(manifest: &mut CatalogManifest, entry: InstalledCommand) {
    if let Some(existing) = manifest
        .commands
        .iter_mut()
        .find(|item| item.name == entry.name)
    {
        *existing = entry;
    } else {
        manifest.commands.push(entry);
        manifest
            .commands
            .sort_by(|left, right| left.name.cmp(&right.name));
    }
}

fn dedupe(names: &[String]) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut ordered = Vec::new();
    for name in names {
        if seen.insert(name.clone()) {
            ordered.push(name.clone());
        }
    }
    ordered
}

fn print_summary(
    paths: &ProjectPaths,
    installed_skills: &[String],
    installed_agents: &[String],
    installed_commands: &[String],
    skipped: &[String],
    notes: &[String],
) {
    if !installed_skills.is_empty() {
        let label = if installed_skills.len() == 1 {
            "skill"
        } else {
            "skills"
        };
        println!(
            "Installed {} {} into {}",
            installed_skills.len(),
            label,
            paths.skills_dir.display()
        );
        for skill in installed_skills {
            println!("- {skill}");
        }
        println!();
    }

    if !installed_agents.is_empty() {
        let label = if installed_agents.len() == 1 {
            "agent"
        } else {
            "agents"
        };
        println!(
            "Installed {} {} into {}",
            installed_agents.len(),
            label,
            paths.agents_dir.display()
        );
        for agent in installed_agents {
            println!("- {agent}");
        }
        println!();
    }

    if !installed_commands.is_empty() {
        let label = if installed_commands.len() == 1 {
            "command"
        } else {
            "commands"
        };
        println!(
            "Installed {} {} into {}",
            installed_commands.len(),
            label,
            paths.commands_dir.display()
        );
        for command in installed_commands {
            println!("- {command}");
        }
        println!();
    }

    for note in notes {
        println!("{note}");
    }

    for item in skipped {
        println!("{item}");
        println!("Hint: rerun with --force if you want to overwrite local changes");
        println!();
    }

    println!("Manifest updated: {}", paths.catalog_path.display());
}
