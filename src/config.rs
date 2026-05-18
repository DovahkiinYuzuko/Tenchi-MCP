use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub global: GlobalConfig,
    pub models: Vec<ModelConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GlobalConfig {
    pub ollama_url: String,
    pub default_timeout: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModelConfig {
    pub name: String,
    pub role: String,
    pub description: String,
    pub priority: i32,
    pub system_prompt: String,
    pub options: Option<ModelOptions>,
    pub runtime: Option<ModelRuntime>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelOptions {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub repeat_penalty: Option<f32>,
    pub num_ctx: Option<usize>,
    pub num_predict: Option<i32>,
    pub stop: Option<Vec<String>>,
    pub seed: Option<i32>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRuntime {
    pub num_thread: Option<i32>,
    pub num_gpu: Option<i32>,
    pub low_vram: Option<bool>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Config {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
