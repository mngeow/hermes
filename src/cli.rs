use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "hermes")]
#[command(about = "Install curated OpenCode skills and agents into a local .opencode workspace")]
pub struct Cli {
    #[arg(long, global = true, value_name = "PATH")]
    pub skills_source: Option<PathBuf>,

    #[arg(long, global = true, value_name = "PATH")]
    pub agents_source: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init(InitArgs),
    Configure(ConfigureArgs),
    Install(InstallArgs),
    List(ListArgs),
    Sync(SyncArgs),
    Remove(RemoveArgs),
    Doctor,
}

#[derive(Debug, Args)]
pub struct InitArgs {}

#[derive(Debug, Args)]
pub struct ConfigureArgs {
    #[arg(long, value_name = "PATH")]
    pub skills_source: Option<PathBuf>,

    #[arg(long, value_name = "PATH")]
    pub agents_source: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct InstallArgs {
    #[arg(long, value_name = "SKILL", num_args = 1..)]
    pub skills: Vec<String>,

    #[arg(long, value_name = "AGENT", num_args = 1..)]
    pub agents: Vec<String>,

    #[arg(long)]
    pub force: bool,

    #[command(subcommand)]
    pub target: Option<InstallTarget>,
}

#[derive(Debug, Subcommand)]
pub enum InstallTarget {
    Skills(NameList),
    Agents(NameList),
}

#[derive(Debug, Args)]
pub struct NameList {
    #[arg(value_name = "NAME")]
    pub names: Vec<String>,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[arg(long, value_enum, conflicts_with = "installed")]
    pub available: Option<ListTarget>,

    #[arg(long, value_enum, conflicts_with = "available")]
    pub installed: Option<ListTarget>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ListTarget {
    Skills,
    Agents,
    All,
}

#[derive(Debug, Args)]
pub struct SyncArgs {
    #[arg(long)]
    pub all: bool,

    #[arg(long)]
    pub skills: bool,

    #[arg(long)]
    pub agents: bool,

    #[arg(long)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    #[command(subcommand)]
    pub target: RemoveTarget,
}

#[derive(Debug, Subcommand)]
pub enum RemoveTarget {
    Skills(SingleName),
    Agents(SingleName),
}

#[derive(Debug, Args)]
pub struct SingleName {
    pub name: String,
}
