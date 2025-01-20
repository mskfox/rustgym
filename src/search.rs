use crate::config::ProblemConfig;
use crate::constants::get_platform_patterns;
use glob::glob;

pub struct SearchCriteria {
    pub name: Option<String>,
    pub id: Option<String>,
    pub difficulty: Option<String>,
    pub platform: Option<String>,
}

pub fn find_problems(criteria: &SearchCriteria) -> anyhow::Result<Vec<ProblemConfig>> {
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
    // Check name
    if let Some(name) = &criteria.name {
        if !problem.name.to_lowercase().contains(&name.to_lowercase()) {
            return false;
        }
    }

    // Check ID
    if let Some(id) = &criteria.id {
        if !problem.id.to_lowercase().contains(&id.to_lowercase()) {
            return false;
        }
    }

    // Check difficulty
    if let Some(difficulty) = &criteria.difficulty {
        if let Some(prob_diff) = &problem.difficulty {
            if !prob_diff.to_lowercase().contains(&difficulty.to_lowercase()) {
                return false;
            }
        } else {
            return false;
        }
    }

    // Check platform
    if let Some(platform) = &criteria.platform {
        if !problem.platform.to_lowercase().contains(&platform.to_lowercase()) {
            return false;
        }
    }

    true
}