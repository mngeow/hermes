use anyhow::{bail, Result};

use crate::cli::ConfigureArgs;
use crate::user_config::{canonicalize_source_root, load_user_config, save_user_config};

pub fn run(args: ConfigureArgs) -> Result<()> {
    // Require at least one source root to be provided
    if args.skills_source.is_none() && args.agents_source.is_none() {
        bail!(
            "hermes configure requires at least one source root; \
             pass --skills-source or --agents-source"
        );
    }

    // Load existing config to preserve unspecified roots
    let mut config = load_user_config()?;

    // Update skills source root if provided
    if let Some(path) = args.skills_source {
        let canonical = canonicalize_source_root(&path)?;
        config.skills_source_root = Some(canonical);
        println!(
            "Skills source root configured: {}",
            config.skills_source_root.as_ref().unwrap().display()
        );
    }

    // Update agents source root if provided
    if let Some(path) = args.agents_source {
        let canonical = canonicalize_source_root(&path)?;
        config.agents_source_root = Some(canonical);
        println!(
            "Agents source root configured: {}",
            config.agents_source_root.as_ref().unwrap().display()
        );
    }

    // Save the updated config
    save_user_config(&config)?;
    println!("Configuration saved to ~/.config/hermes_cli/config.toml");

    Ok(())
}
