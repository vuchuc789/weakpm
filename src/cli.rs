use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "weakpm")]
#[command(version = "0.1.0")]
#[command(about = "A simple package manager for non-root users 🥴", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for a release on Github
    Search(SearchArgs),
}

#[derive(clap::Args)]
pub struct SearchArgs {
    /// Github repo (e.g. junegunn/fzf)
    pub repo: String,
}
