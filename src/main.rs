mod agents;
mod app;
mod cli;
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

fn main() {
    if let Err(err) = app::run() {
        eprintln!("Error: {err:#}");
        std::process::exit(1);
    }
}
