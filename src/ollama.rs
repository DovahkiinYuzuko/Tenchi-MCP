use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use crate::config::{ModelOptions, ModelRuntime};

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
    options: serde_json::Value,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    pub fn new(base_url: String, timeout: u64) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(timeout))
                .build()
                .unwrap_or_else(|_| Client::new()),
            base_url,
        }
    }

    pub async fn generate(
        &self, 
        model: &str, 
        system: &str, 
        prompt: &str, 
        options: Option<ModelOptions>,
        runtime: Option<ModelRuntime>,
    ) -> anyhow::Result<String> {
        let url = format!("{}/api/generate", self.base_url);
        
        // Merge options and runtime into a single JSON object for Ollama
        let mut combined_options = serde_json::json!({});
        
        if let Some(opts) = options {
            if let serde_json::Value::Object(mut map) = serde_json::to_value(opts)? {
                if let serde_json::Value::Object(ref mut combined_map) = combined_options {
                    combined_map.append(&mut map);
                }
            }
        }
        
        if let Some(run) = runtime {
            if let serde_json::Value::Object(mut map) = serde_json::to_value(run)? {
                if let serde_json::Value::Object(ref mut combined_map) = combined_options {
                    combined_map.append(&mut map);
                }
            }
        }
        
        let start = std::time::Instant::now();
        eprintln!(">>> Tenchi-MCP: Starting inference using model '{}'...", model);

        let req = GenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            system: system.to_string(),
            stream: false,
            options: combined_options,
        };

        let res = self.client.post(url).json(&req).send().await?;
        let data: GenerateResponse = res.json().await?;
        
        let duration = start.elapsed();
        eprintln!(">>> Tenchi-MCP: Inference complete in {:.2}s", duration.as_secs_f32());

        // Basic cleanup: remove <think> blocks and end tokens
        let mut response = data.response;
        
        // Remove <think>...</think> blocks
        while let (Some(start), Some(end)) = (response.find("<think>"), response.find("</think>")) {
            response.replace_range(start..end + 8, "");
        }

        if let Some(pos) = response.find("<|endoftext|>") {
            response.truncate(pos);
        }
        if let Some(pos) = response.find("<|im_start|>") {
            response.truncate(pos);
        }
        
        Ok(response.trim().to_string())
    }
}
