mod cli;
mod commands;
mod github;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Search(args) => commands::search::run(args),
    }
}
