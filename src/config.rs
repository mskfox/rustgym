use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::constants::get_problem_config_path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemConfig {
    pub name: String,
    pub id: String,
    pub platform: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip)]
    pub path: PathBuf,
}

impl ProblemConfig {
    pub fn load(path: PathBuf) -> anyhow::Result<Option<Self>> {
        let config_path = get_problem_config_path(&path);
        if !config_path.exists() {
            return Ok(None);
        }

        let contents = std::fs::read_to_string(config_path)?;
        let mut config: ProblemConfig = toml::from_str(&contents)?;
        config.path = path;
        Ok(Some(config))
    }
}