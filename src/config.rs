use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub global: GlobalConfig,
    pub models: Vec<ModelConfig>,
}

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub ollama_url: String,
    pub default_timeout: u64,
}

#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub role: String,
    pub description: String,
    pub priority: i32,
    pub system_prompt: String,
    pub options: Option<HashMap<String, serde_json::Value>>,
    pub runtime: Option<HashMap<String, serde_json::Value>>,
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
