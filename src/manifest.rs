use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::models::{CatalogManifest, ProjectPaths, SourceOverrides, SourceRoots};
use crate::user_config::load_user_config;

pub fn load_manifest(paths: &ProjectPaths) -> Result<Option<CatalogManifest>> {
    if !paths.catalog_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&paths.catalog_path)
        .with_context(|| format!("failed to read {}", paths.catalog_path.display()))?;
    let manifest = toml::from_str::<CatalogManifest>(&content)
        .with_context(|| format!("failed to parse {}", paths.catalog_path.display()))?;
    Ok(Some(manifest))
}

pub fn load_or_default(paths: &ProjectPaths) -> Result<CatalogManifest> {
    Ok(load_manifest(paths)?.unwrap_or_default())
}

pub fn save_manifest(paths: &ProjectPaths, manifest: &CatalogManifest) -> Result<()> {
    if let Some(parent) = paths.catalog_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(manifest)?;
    fs::write(&paths.catalog_path, content)
        .with_context(|| format!("failed to write {}", paths.catalog_path.display()))?;
    Ok(())
}

/// Resolve source roots using precedence: CLI flags → user config → environment variables
/// Note: project catalog source root values are no longer used for resolution
pub fn resolve_source_roots(
    overrides: &SourceOverrides,
    _manifest: Option<&CatalogManifest>,
) -> Result<SourceRoots> {
    // Load user config for fallback
    let user_config = load_user_config().unwrap_or_default();

    Ok(SourceRoots {
        skills: resolve_source_root(
            overrides.skills.as_deref(),
            user_config.skills_source_root.as_deref(),
            "OPENCODE_SKILLS_SOURCE",
        )?,
        agents: resolve_source_root(
            overrides.agents.as_deref(),
            user_config.agents_source_root.as_deref(),
            "OPENCODE_AGENTS_SOURCE",
        )?,
    })
}

pub fn absolutize_existing_dir(path: &Path) -> Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    };

    if !absolute.exists() {
        bail!("path {} does not exist", absolute.display())
    }
    if !absolute.is_dir() {
        bail!("path {} is not a directory", absolute.display())
    }

    absolute
        .canonicalize()
        .with_context(|| format!("failed to resolve source directory {}", absolute.display()))
}

fn resolve_source_root(
    override_path: Option<&Path>,
    user_config_path: Option<&Path>,
    env_var: &str,
) -> Result<Option<PathBuf>> {
    // CLI flag takes highest precedence
    if let Some(path) = override_path {
        return Ok(Some(absolutize_existing_dir(path)?));
    }

    // User config is next
    if let Some(path) = user_config_path {
        return Ok(Some(absolutize_existing_dir(path)?));
    }

    // Environment variable is last fallback
    match env::var_os(env_var) {
        Some(value) => Ok(Some(absolutize_existing_dir(Path::new(&value))?)),
        None => Ok(None),
    }
}
