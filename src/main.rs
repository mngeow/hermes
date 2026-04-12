mod agents;
mod app;
mod cli;
mod configure;
mod doctor;
mod frontmatter;
mod fs_ops;
mod hashing;
mod install;
mod manifest;
mod models;
mod remove;
mod skills;
mod sync;
mod tui;
mod user_config;

fn main() {
    if let Err(err) = app::run() {
        eprintln!("Error: {err:#}");
        std::process::exit(1);
    }
}
