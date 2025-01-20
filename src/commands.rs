use anyhow::Result;
use colored::Colorize;
use std::process::Command;
use crate::config::ProblemConfig;
use crate::display::{display_problems, display_test_results};
use crate::search::{find_problems, SearchCriteria};

pub fn handle_search(
    name: Option<String>,
    id: Option<String>,
    difficulty: Option<String>,
    platform: Option<String>,
) -> Result<()> {
    let criteria = SearchCriteria {
        name,
        id,
        difficulty,
        platform,
    };

    let problems = find_problems(&criteria)?;
    display_problems(&problems);
    Ok(())
}

pub fn handle_test(
    name: Option<String>,
    id: Option<String>,
    platform: Option<String>,
) -> Result<()> {
    let criteria = SearchCriteria {
        name,
        id,
        difficulty: None,
        platform,
    };

    let problems = find_problems(&criteria)?;

    match problems.len() {
        0 => println!("{}", "No problems found matching the criteria.".yellow()),
        1 => run_tests(&problems[0])?,
        _ => {
            println!("{}", "\nMultiple problems found:".yellow());
            display_problems(&problems);
            println!("\n{}", "Please refine your search to test a specific problem.".yellow());
        }
    }

    Ok(())
}

fn run_tests(problem: &ProblemConfig) -> Result<()> {
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

    display_test_results(&output);
    Ok(())
}