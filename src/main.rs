use clap::{Parser, Subcommand};
use colored::*;
use std::process::Command;

mod config;
mod constants;
mod search;

use crate::config::ProblemConfig;
use crate::search::{find_problems, SearchCriteria};

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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search {
            name,
            id,
            difficulty,
            platform,
        } => {
            let criteria = SearchCriteria {
                name,
                id,
                difficulty,
                platform,
            };

            let problems = find_problems(&criteria)?;
            display_problems(&problems);
        }
        Commands::Test {
            name,
            id,
            platform,
        } => {
            let criteria = SearchCriteria {
                name,
                id,
                difficulty: None,
                platform,
            };

            let problems = find_problems(&criteria)?;

            if problems.is_empty() {
                println!("{}", "No problems found matching the criteria.".yellow());
                return Ok(());
            }

            if problems.len() > 1 {
                println!("{}", "\nMultiple problems found:".yellow());
                display_problems(&problems);
                println!("\n{}", "Please refine your search to test a specific problem.".yellow());
                return Ok(());
            }

            let problem = &problems[0];
            println!("\n{} Running tests for {} ({})",
                "→".blue(),
                problem.name.bold(),
                problem.platform.cyan()
            );

            let output = Command::new("cargo")
                .arg("test")
                .arg("--package")
                .arg(problem.path.file_stem().unwrap().to_str().unwrap())
                .output()?;

            if output.status.success() {
                println!("\n{}", "Tests passed successfully!".green());
            } else {
                println!("\n{}", "Tests failed:".red());
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }

    Ok(())
}

fn display_problems(problems: &[ProblemConfig]) {
    if problems.is_empty() {
        println!("{}", "No problems found matching the criteria.".yellow());
        return;
    }

    let problem_count = problems.len();
    let problem_label = if problem_count == 1 { "problem" } else { "problems" };

    println!("{}", format!("\nFound {} {}:", problem_count, problem_label).green());
    for problem in problems {
        println!(
            "\n{} {} ({})",
            "→".blue(),
            problem.name.bold(),
            problem.platform.cyan()
        );
        println!("  Path: {}", problem.path.display().to_string().underline());
        println!("  ID: {}", problem.id);
        if let Some(diff) = &problem.difficulty {
            println!(
                "  Difficulty: {}",
                match diff.to_lowercase().as_str() {
                    "easy" => diff.green(),
                    "medium" => diff.yellow(),
                    "hard" => diff.red(),
                    _ => diff.normal(),
                }
            );
        }
        if let Some(url) = &problem.url {
            println!("  URL: {}", url);
        }
    }
}