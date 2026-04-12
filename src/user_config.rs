use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::models::UserConfig;

/// Returns the path to the user config directory: ~/.config/hermes_cli
pub fn user_config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("home directory not found")
        .join(".config")
        .join("hermes_cli")
}

/// Returns the path to the user config file: ~/.config/hermes_cli/config.toml
pub fn user_config_path() -> PathBuf {
    user_config_dir().join("config.toml")
}

/// Load the user config if it exists, otherwise return a default empty config
pub fn load_user_config() -> Result<UserConfig> {
    let path = user_config_path();
    if !path.exists() {
        return Ok(UserConfig::default());
    }

    let content =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    let config = toml::from_str::<UserConfig>(&content)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    Ok(config)
}

/// Save the user config to ~/.config/hermes_cli/config.toml
pub fn save_user_config(config: &UserConfig) -> Result<()> {
    let dir = user_config_dir();
    fs::create_dir_all(&dir)
        .with_context(|| format!("failed to create directory {}", dir.display()))?;

    let path = user_config_path();
    let content = toml::to_string_pretty(config)?;
    fs::write(&path, content).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

/// Canonicalize a path and verify it exists and is a directory
pub fn canonicalize_source_root(path: &Path) -> Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };

    if !absolute.exists() {
        anyhow::bail!("path {} does not exist", absolute.display());
    }
    if !absolute.is_dir() {
        anyhow::bail!("path {} is not a directory", absolute.display());
    }

    absolute
        .canonicalize()
        .with_context(|| format!("failed to resolve source directory {}", absolute.display()))
}
