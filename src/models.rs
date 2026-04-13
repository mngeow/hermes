use std::fmt;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum InstallMode {
    #[default]
    Copy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentMode {
    Primary,
    Subagent,
    All,
}

impl AgentMode {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "primary" => Ok(Self::Primary),
            "subagent" => Ok(Self::Subagent),
            "all" => Ok(Self::All),
            other => bail!("unsupported agent mode '{other}'"),
        }
    }
}

impl fmt::Display for AgentMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Primary => "primary",
            Self::Subagent => "subagent",
            Self::All => "all",
        };
        write!(f, "{value}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogManifest {
    pub version: u32,
    pub install_mode: InstallMode,
    pub skills_source_root: Option<PathBuf>,
    pub agents_source_root: Option<PathBuf>,
    pub commands_source_root: Option<PathBuf>,
    #[serde(default)]
    pub skills: Vec<InstalledSkill>,
    #[serde(default)]
    pub agents: Vec<InstalledAgent>,
    #[serde(default)]
    pub commands: Vec<InstalledCommand>,
}

impl Default for CatalogManifest {
    fn default() -> Self {
        Self {
            version: 1,
            install_mode: InstallMode::Copy,
            skills_source_root: None,
            agents_source_root: None,
            commands_source_root: None,
            skills: Vec::new(),
            agents: Vec::new(),
            commands: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledSkill {
    pub name: String,
    pub description: String,
    pub source_rel_path: PathBuf,
    pub installed_rel_path: PathBuf,
    pub source_hash: String,
    pub installed_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledAgent {
    pub name: String,
    pub description: String,
    pub mode: Option<AgentMode>,
    pub source_rel_path: PathBuf,
    pub installed_rel_path: PathBuf,
    pub source_hash: String,
    pub installed_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledCommand {
    pub name: String,
    pub description: Option<String>,
    pub source_rel_path: PathBuf,
    pub installed_rel_path: PathBuf,
    pub source_hash: String,
    pub installed_hash: String,
}

#[derive(Debug, Clone)]
pub struct DiscoveredSkill {
    pub name: String,
    pub description: String,
    pub source_path: PathBuf,
    pub source_rel_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DiscoveredAgent {
    pub name: String,
    pub description: String,
    pub mode: Option<AgentMode>,
    pub source_path: PathBuf,
    pub source_rel_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DiscoveredCommand {
    pub name: String,
    pub description: Option<String>,
    pub source_path: PathBuf,
    pub source_rel_path: PathBuf,
}

#[derive(Debug, Clone, Default)]
pub struct SourceOverrides {
    pub skills: Option<PathBuf>,
    pub agents: Option<PathBuf>,
    pub commands: Option<PathBuf>,
}

#[derive(Debug, Clone, Default)]
pub struct SourceRoots {
    pub skills: Option<PathBuf>,
    pub agents: Option<PathBuf>,
    pub commands: Option<PathBuf>,
}

impl SourceRoots {
    pub fn is_empty(&self) -> bool {
        self.skills.is_none() && self.agents.is_none() && self.commands.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct ProjectPaths {
    pub opencode_dir: PathBuf,
    pub skills_dir: PathBuf,
    pub agents_dir: PathBuf,
    pub commands_dir: PathBuf,
    pub tmp_dir: PathBuf,
    pub catalog_path: PathBuf,
}

impl ProjectPaths {
    pub fn new(root: PathBuf) -> Self {
        let opencode_dir = root.join(".opencode");
        let skills_dir = opencode_dir.join("skills");
        let agents_dir = opencode_dir.join("agents");
        let commands_dir = opencode_dir.join("commands");
        let tmp_dir = opencode_dir.join(".tmp");
        let catalog_path = opencode_dir.join("catalog.toml");
        Self {
            opencode_dir,
            skills_dir,
            agents_dir,
            commands_dir,
            tmp_dir,
            catalog_path,
        }
    }

    pub fn installed_path(&self, rel_path: &Path) -> PathBuf {
        self.opencode_dir.join(rel_path)
    }
}

#[derive(Debug)]
pub struct Inspection<T> {
    pub items: Vec<T>,
    pub issues: Vec<String>,
}

impl<T> Default for Inspection<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            issues: Vec::new(),
        }
    }
}

/// User-level Hermes configuration stored at ~/.config/hermes_cli/config.toml
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserConfig {
    pub skills_source_root: Option<PathBuf>,
    pub agents_source_root: Option<PathBuf>,
    pub commands_source_root: Option<PathBuf>,
}
