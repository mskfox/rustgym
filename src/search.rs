use anyhow::Result;
use glob::glob;
use crate::config::ProblemConfig;
use crate::constants::get_platform_patterns;

pub struct SearchCriteria {
    pub name: Option<String>,
    pub id: Option<String>,
    pub difficulty: Option<String>,
    pub platform: Option<String>,
}

pub fn find_problems(criteria: &SearchCriteria) -> Result<Vec<ProblemConfig>> {
    let mut problems = Vec::new();

    for pattern in get_platform_patterns() {
        for entry in glob(&pattern)? {
            if let Ok(path) = entry {
                if let Ok(Some(config)) = ProblemConfig::load(path) {
                    if matches_criteria(&config, criteria) {
                        problems.push(config);
                    }
                }
            }
        }
    }

    problems.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(problems)
}

fn matches_criteria(problem: &ProblemConfig, criteria: &SearchCriteria) -> bool {
    matches_name(problem, &criteria.name)
        && matches_id(problem, &criteria.id)
        && matches_difficulty(problem, &criteria.difficulty)
        && matches_platform(problem, &criteria.platform)
}

fn matches_name(problem: &ProblemConfig, name: &Option<String>) -> bool {
    name.as_ref().map_or(true, |n|
        problem.name.to_lowercase().contains(&n.to_lowercase())
    )
}

fn matches_id(problem: &ProblemConfig, id: &Option<String>) -> bool {
    id.as_ref().map_or(true, |i|
        problem.id.to_lowercase().contains(&i.to_lowercase())
    )
}

fn matches_difficulty(problem: &ProblemConfig, difficulty: &Option<String>) -> bool {
    match (difficulty, &problem.difficulty) {
        (Some(d), Some(pd)) => pd.to_lowercase().contains(&d.to_lowercase()),
        (Some(_), None) => false,
        (None, _) => true,
    }
}

fn matches_platform(problem: &ProblemConfig, platform: &Option<String>) -> bool {
    platform.as_ref().map_or(true, |p|
        problem.platform.to_lowercase().contains(&p.to_lowercase())
    )
}