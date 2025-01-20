use colored::*;
use crate::config::ProblemConfig;

pub fn display_problems(problems: &[ProblemConfig]) {
    if problems.is_empty() {
        println!("{}", "No problems found matching the criteria.".yellow());
        return;
    }

    print_summary(problems);
    problems.iter().for_each(print_problem);
}

pub fn display_test_results(output: &std::process::Output) {
    if output.status.success() {
        println!("\n{}", "Tests passed successfully!".green());
    } else {
        println!("\n{}", "Tests failed:".red());
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

fn print_summary(problems: &[ProblemConfig]) {
    let problem_count = problems.len();
    let problem_label = if problem_count == 1 { "problem" } else { "problems" };
    println!("{}", format!("\nFound {} {}:", problem_count, problem_label).green());
}

fn print_problem(problem: &ProblemConfig) {
    println!(
        "\n{} {} ({})",
        "→".blue(),
        problem.name.bold(),
        problem.platform.cyan()
    );
    println!("  Path: {}", problem.path.display().to_string().underline());
    println!("  ID: {}", problem.id);

    if let Some(diff) = &problem.difficulty {
        print_difficulty(diff);
    }

    println!("  URL: {}", problem.get_url());
}

fn print_difficulty(difficulty: &str) {
    let colored_diff = match difficulty.to_lowercase().as_str() {
        "easy" => difficulty.green(),
        "medium" => difficulty.yellow(),
        "hard" => difficulty.red(),
        _ => difficulty.normal(),
    };
    println!("  Difficulty: {}", colored_diff);
}