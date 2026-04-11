use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::fs_ops::{is_ignored_relative, normalize_relative_path};

pub fn hash_agent_file(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Ok(format!("sha256:{:x}", hasher.finalize()))
}

pub fn hash_skill_dir(path: &Path) -> Result<String> {
    let mut files = Vec::new();
    let walker = WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_entry(|entry| {
            entry
                .path()
                .strip_prefix(path)
                .map(|rel| !is_ignored_relative(rel))
                .unwrap_or(true)
        });

    for entry in walker {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        let rel_path = entry.path().strip_prefix(path)?;
        if is_ignored_relative(rel_path) {
            continue;
        }

        files.push(entry.path().to_path_buf());
    }

    files.sort_by(|left, right| {
        let left_rel = left.strip_prefix(path).unwrap_or(left.as_path());
        let right_rel = right.strip_prefix(path).unwrap_or(right.as_path());
        normalize_relative_path(left_rel).cmp(&normalize_relative_path(right_rel))
    });

    let mut hasher = Sha256::new();
    for file in files {
        let rel_path = file.strip_prefix(path)?;
        hasher.update(normalize_relative_path(rel_path).as_bytes());
        hasher.update([0]);
        let bytes =
            fs::read(&file).with_context(|| format!("failed to read {}", file.display()))?;
        hasher.update(&bytes);
        hasher.update([0]);
    }

    Ok(format!("sha256:{:x}", hasher.finalize()))
}
