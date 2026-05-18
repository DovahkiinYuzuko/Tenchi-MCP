# Tenchi-MCP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust-based MCP server that bridges cloud LLMs with local Ollama instances using a TOML-based configuration for model selection and task delegation.

**Architecture:** A Rust MCP server using `mcp-sdk-rs` (or equivalent) that reads `models_config.toml`, interacts with Ollama's local API via `reqwest`, and provides tools for model discovery and generation.

**Tech Stack:** Rust, Tokio (async), Serde (TOML/JSON), Reqwest (HTTP), MCP SDK.

---

### Task 1: Project Initialization

**Files:**
- Create: `Cargo.toml`
- Create: `.gitignore`

- [ ] **Step 1: Create Cargo.toml with dependencies**

```toml
[package]
name = "tenchi-mcp"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
reqwest = { version = "0.11", features = ["json"] }
anyhow = "1.0"
mcp-sdk = "0.1.0" # Assuming a standard MCP SDK crate
```

- [ ] **Step 2: Create .gitignore**

```text
/target
Cargo.lock
.env
```

- [ ] **Step 3: Verify build**

Run: `cargo build`
Expected: Success (empty project)

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml .gitignore
git commit -m "chore: initialize rust project"
```

---

### Task 2: Configuration Mapping (TOML)

**Files:**
- Create: `src/config.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Define Config structs in src/config.rs**

```rust
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
```

- [ ] **Step 2: Write test for config parsing**

Create `tests/config_test.rs`:
```rust
use tenchi_mcp::config::Config;

#[test]
fn test_config_parsing() {
    let toml_str = r#"
[global]
ollama_url = "http://localhost:11434"
default_timeout = 300

[[models]]
name = "test-model"
role = "Coder"
description = "Test desc"
priority = 1
system_prompt = "Test prompt"
[models.options]
temperature = 0.5
"#;
    let config: Config = toml::from_str(toml_str).unwrap();
    assert_eq!(config.global.ollama_url, "http://localhost:11434");
    assert_eq!(config.models[0].options.as_ref().unwrap().get("temperature").unwrap(), &0.5);
}
```

- [ ] **Step 3: Commit**

```bash
git add src/config.rs tests/config_test.rs
git commit -m "feat: add enhanced TOML config parsing"
```

---

### Task 3: Ollama API Client

**Files:**
- Create: `src/ollama.rs`

- [ ] **Step 1: Implement Ollama client**

```rust
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
```

- [ ] **Step 2: Commit**

```bash
git add src/ollama.rs
git commit -m "feat: add Ollama API client"
```

---

### Task 4: MCP Server Core

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Implement MCP server loop and tools**

```rust
mod config;
mod ollama;

use mcp_sdk::server::{Server, Tool}; // Placeholder for actual SDK
use config::Config;
use ollama::OllamaClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_file("models_config.toml")?;
    let client = OllamaClient::new(config.global.ollama_url.clone());

    let mut server = Server::new("Tenchi-MCP");

    // Add tool to list models from config
    server.add_tool(Tool::new("list_local_models", "List available local models and their roles", |args| {
        // Return config models as JSON
    }));

    // Add tool to generate using a local model
    server.add_tool(Tool::new("local_generate", "Generate text using a specific local model", |args| {
        // Call client.generate and return result
    }));

    server.run().await?;
    Ok(())
}
```

- [ ] **Step 2: Commit**

```bash
git add src/main.rs
git commit -m "feat: implement MCP server and tools"
```
