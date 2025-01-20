use std::path::PathBuf;

pub const PROBLEM_CONFIG_FILE: &str = "rustgym.toml";
pub const SUPPORTED_PLATFORMS: &[&str] = &["leetcode"];

pub fn get_platform_patterns() -> Vec<String> {
    SUPPORTED_PLATFORMS
        .iter()
        .map(|platform| format!("{}/*", platform))
        .collect()
}

pub fn get_problem_config_path(problem_path: &PathBuf) -> PathBuf {
    problem_path.join(PROBLEM_CONFIG_FILE)
}