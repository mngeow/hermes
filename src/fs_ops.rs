use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use walkdir::WalkDir;

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn ensure_dir(path: &Path) -> Result<()> {
    fs::create_dir_all(path)
        .with_context(|| format!("failed to create directory {}", path.display()))
}

pub fn ensure_workspace(
    opencode_dir: &Path,
    skills_dir: &Path,
    agents_dir: &Path,
    tmp_dir: &Path,
) -> Result<()> {
    ensure_dir(opencode_dir)?;
    ensure_dir(skills_dir)?;
    ensure_dir(agents_dir)?;
    ensure_dir(tmp_dir)?;
    Ok(())
}

pub fn remove_path_if_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    if path.is_dir() {
        fs::remove_dir_all(path)
            .with_context(|| format!("failed to remove directory {}", path.display()))?;
    } else {
        fs::remove_file(path)
            .with_context(|| format!("failed to remove file {}", path.display()))?;
    }
    Ok(())
}

pub fn is_ignored_relative(rel_path: &Path) -> bool {
    if rel_path.as_os_str().is_empty() {
        return false;
    }

    if rel_path.file_name().and_then(|name| name.to_str()) == Some(".DS_Store") {
        return true;
    }

    if matches!(
        rel_path.extension().and_then(|ext| ext.to_str()),
        Some("pyc" | "pyo" | "pyd")
    ) {
        return true;
    }

    rel_path.components().any(|component| match component {
        Component::Normal(name) => {
            matches!(name.to_str(), Some("__pycache__" | ".git" | "node_modules"))
        }
        _ => false,
    })
}

pub fn copy_dir_filtered(source: &Path, destination: &Path) -> Result<()> {
    ensure_dir(destination)?;

    let walker = WalkDir::new(source)
        .min_depth(1)
        .into_iter()
        .filter_entry(|entry| {
            entry
                .path()
                .strip_prefix(source)
                .map(|rel| !is_ignored_relative(rel))
                .unwrap_or(true)
        });

    for entry in walker {
        let entry = entry?;
        let rel_path = entry.path().strip_prefix(source)?;
        if is_ignored_relative(rel_path) {
            continue;
        }

        let target = destination.join(rel_path);
        if entry.file_type().is_dir() {
            ensure_dir(&target)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = target.parent() {
                ensure_dir(parent)?;
            }
            fs::copy(entry.path(), &target).with_context(|| {
                format!(
                    "failed to copy {} to {}",
                    entry.path().display(),
                    target.display()
                )
            })?;
        }
    }

    Ok(())
}

pub fn copy_file(source: &Path, destination: &Path) -> Result<()> {
    if let Some(parent) = destination.parent() {
        ensure_dir(parent)?;
    }
    fs::copy(source, destination).with_context(|| {
        format!(
            "failed to copy {} to {}",
            source.display(),
            destination.display()
        )
    })?;
    Ok(())
}

pub fn atomic_install_skill(source: &Path, destination: &Path, tmp_root: &Path) -> Result<()> {
    ensure_dir(tmp_root)?;
    let temp_path = unique_temp_path(tmp_root, "skill", None);
    let backup_path = unique_temp_path(tmp_root, "skill-backup", None);
    copy_dir_filtered(source, &temp_path)?;
    swap_directory(&temp_path, destination, &backup_path)
}

pub fn atomic_install_agent(source: &Path, destination: &Path, tmp_root: &Path) -> Result<()> {
    ensure_dir(tmp_root)?;
    let temp_path = unique_temp_path(tmp_root, "agent", Some("md"));
    let backup_path = unique_temp_path(tmp_root, "agent-backup", Some("md"));
    copy_file(source, &temp_path)?;
    swap_file(&temp_path, destination, &backup_path)
}

fn swap_directory(temp_path: &Path, destination: &Path, backup_path: &Path) -> Result<()> {
    if let Some(parent) = destination.parent() {
        ensure_dir(parent)?;
    }

    let had_existing = destination.exists();
    if had_existing {
        fs::rename(destination, backup_path).with_context(|| {
            format!(
                "failed to move existing directory {} to {}",
                destination.display(),
                backup_path.display()
            )
        })?;
    }

    if let Err(err) = fs::rename(temp_path, destination) {
        if had_existing && backup_path.exists() {
            let _ = fs::rename(backup_path, destination);
        }
        return Err(err).with_context(|| {
            format!(
                "failed to move staged directory {} to {}",
                temp_path.display(),
                destination.display()
            )
        });
    }

    if had_existing {
        remove_path_if_exists(backup_path)?;
    }
    Ok(())
}

fn swap_file(temp_path: &Path, destination: &Path, backup_path: &Path) -> Result<()> {
    if let Some(parent) = destination.parent() {
        ensure_dir(parent)?;
    }

    let had_existing = destination.exists();
    if had_existing {
        fs::rename(destination, backup_path).with_context(|| {
            format!(
                "failed to move existing file {} to {}",
                destination.display(),
                backup_path.display()
            )
        })?;
    }

    if let Err(err) = fs::rename(temp_path, destination) {
        if had_existing && backup_path.exists() {
            let _ = fs::rename(backup_path, destination);
        }
        return Err(err).with_context(|| {
            format!(
                "failed to move staged file {} to {}",
                temp_path.display(),
                destination.display()
            )
        });
    }

    if had_existing {
        remove_path_if_exists(backup_path)?;
    }
    Ok(())
}

fn unique_temp_path(tmp_root: &Path, prefix: &str, extension: Option<&str>) -> PathBuf {
    let suffix = format!(
        "{}-{}-{}",
        process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
        TEMP_COUNTER.fetch_add(1, Ordering::Relaxed)
    );

    let mut path = tmp_root.join(format!("{prefix}-{suffix}"));
    if let Some(ext) = extension {
        path.set_extension(ext);
    }
    path
}

pub fn normalize_relative_path(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy().to_string()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}
