use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::constants::{get_problem_config_path, SUPPORTED_PLATFORMS};
use anyhow::{Context, Result};

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
    pub fn load(path: PathBuf) -> Result<Option<Self>> {
        let config_path = get_problem_config_path(&path);
        if !config_path.exists() {
            return Ok(None);
        }

        let contents = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {}", config_path.display()))?;

        let config_value: toml::Value = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse TOML config at {}", config_path.display()))?;

        Self::validate_config(&config_value, &config_path)?;

        let mut config: ProblemConfig = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse TOML config at {}", config_path.display()))?;

        config.path = path;

        Ok(Some(config))
    }

    fn validate_config(config: &toml::Value, path: &PathBuf)  -> Result<(), anyhow::Error> {
        let required_fields = ["name", "id", "platform"];
        let optional_fields = ["difficulty", "url"];

        let mut warnings = Vec::new();
        let mut errors = Vec::new();

        // Check for required fields
        for field in required_fields {
            if !config.get(field).is_some() {
                errors.push(format!("Required field '{}' is missing", field));
            }
        }

        // Check for unknown fields
        if let toml::Value::Table(table) = config {
            for key in table.keys() {
                if !required_fields.contains(&key.as_str()) && !optional_fields.contains(&key.as_str()) {
                    warnings.push(format!("Unknown field '{}' found", key));
                }
            }
        }

        // Check for unsupported platform
        if let Some(platform) = config.get("platform").and_then(|v| v.as_str()) {
            if !SUPPORTED_PLATFORMS.contains(&platform) {
                warnings.push(format!("Unsupported platform: {}", platform));
            }
        }

        // Display errors if any
        if !errors.is_empty() {
            println!("\n{} in {}:", "Configuration errors".red().bold(), path.display());
            for error in &errors {
                println!("  {} {}", "•".red(), error);
            }
            anyhow::bail!("\nConfiguration errors detected. Please fix them and try again.");
        }

        // Display warnings if any
        if !warnings.is_empty() {
            println!("\n{} in {}:", "Configuration warnings".yellow().bold(), path.display());
            for warning in warnings {
                println!("  {} {}", "•".yellow(), warning);
            }
        }

        Ok(())
    }

    pub fn get_url(&self) -> String {
        if let Some(url) = &self.url {
            return url.clone();
        }

        self.generate_platform_url()
    }

    fn generate_platform_url(&self) -> String {
        match self.platform.to_lowercase().as_str() {
            "leetcode" => self.generate_leetcode_url(),
            _ => String::new()
        }
    }

    fn generate_leetcode_url(&self) -> String {
        let slug = self.name
            .to_lowercase()
            .replace(' ', "-")
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "");

        format!("https://leetcode.com/problems/{}/", slug)
    }
}