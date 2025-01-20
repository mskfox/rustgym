use clap::{Parser, Subcommand};
use anyhow::Result;

mod config;
mod constants;
mod search;
mod display;
mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Search {
        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        id: Option<String>,

        #[arg(short, long)]
        difficulty: Option<String>,

        #[arg(short, long)]
        platform: Option<String>,
    },
    Test {
        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        id: Option<String>,

        #[arg(short, long)]
        platform: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search {
            name,
            id,
            difficulty,
            platform,
        } => commands::handle_search(name, id, difficulty, platform),

        Commands::Test {
            name,
            id,
            platform,
        } => commands::handle_test(name, id, platform),
    }
}