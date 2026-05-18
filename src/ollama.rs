use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
    options: Option<HashMap<String, serde_json::Value>>,
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
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn generate(
        &self, 
        model: &str, 
        system: &str, 
        prompt: &str, 
        options: Option<HashMap<String, serde_json::Value>>
    ) -> anyhow::Result<String> {
        let url = format!("{}/api/generate", self.base_url);
        let req = GenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            system: system.to_string(),
            stream: false,
            options,
        };

        let res = self.client.post(url).json(&req).send().await?;
        let data: GenerateResponse = res.json().await?;
        Ok(data.response)
    }
}
