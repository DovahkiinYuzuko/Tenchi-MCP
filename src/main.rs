mod config;
mod ollama;

use async_trait::async_trait;
use rust_mcp_sdk::{
    error::SdkResult,
    macros::{mcp_tool, JsonSchema},
    mcp_server::{server_runtime, McpServerOptions, ServerHandler, ToMcpServerHandler},
    schema::*,
    McpServer, StdioTransport, TransportOptions,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::config::Config;
use crate::ollama::OllamaClient;

#[mcp_tool(name = "list_local_models", description = "List available local models and their roles from config")]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ListModelsTool {}

#[mcp_tool(name = "local_generate", description = "Generate text using a specific local model")]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct LocalGenerateTool {
    /// The name of the model to use (from list_local_models)
    pub model_name: String,
    /// The prompt to send to the model
    pub prompt: String,
}

struct TenchiHandler {
    config: Config,
    client: OllamaClient,
}

#[async_trait]
impl ServerHandler for TenchiHandler {
    async fn handle_list_tools_request(
        &self,
        _params: Option<PaginatedRequestParams>,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            tools: vec![ListModelsTool::tool(), LocalGenerateTool::tool()],
            meta: None,
            next_cursor: None,
        })
    }

    async fn handle_call_tool_request(
        &self,
        params: CallToolRequestParams,
        _runtime: Arc<dyn McpServer>,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        match params.name.as_str() {
            "list_local_models" => {
                let mut models = self.config.models.clone();
                models.sort_by_key(|m| m.priority);

                let models_json = json!(models.iter().map(|m| {
                    json!({
                        "name": m.name,
                        "role": m.role,
                        "priority": m.priority,
                        "description": m.description
                    })
                }).collect::<Vec<_>>());
                
                Ok(CallToolResult::text_content(vec![models_json.to_string().into()]))
            }
            "local_generate" => {
                let args: LocalGenerateTool = serde_json::from_value(
                    serde_json::to_value(&params.arguments).unwrap()
                ).map_err(|e| CallToolError::invalid_arguments("local_generate", Some(e.to_string())))?;

                let model_cfg = self.config.models.iter()
                    .find(|m| m.name == args.model_name)
                    .ok_or_else(|| CallToolError::unknown_tool(format!("Model {} not found in config. Available models: {:?}", args.model_name, self.config.models.iter().map(|m| &m.name).collect::<Vec<_>>())))?;

                match self.client.generate(
                    &model_cfg.name, 
                    &model_cfg.system_prompt, 
                    &args.prompt, 
                    model_cfg.options.clone(),
                    model_cfg.runtime.clone()
                ).await {
                    Ok(response) => Ok(CallToolResult::text_content(vec![response.into()])),
                    Err(e) => {
                        let error_msg = format!("Ollama generation failed for model '{}': {}. Please check if Ollama is running and the model is installed.", model_cfg.name, e);
                        let mut result = CallToolResult::text_content(vec![error_msg.into()]);
                        result.is_error = Some(true);
                        Ok(result)
                    }
                }
            }
            _ => Err(CallToolError::unknown_tool(params.name)),
        }
    }
}

#[tokio::main]
async fn main() -> SdkResult<()> {
    let exe_path = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let config_path = exe_path.parent().unwrap_or(std::path::Path::new(".")).join("models_config.toml");

    let config = Config::from_file(&config_path).map_err(|e| {
        eprintln!(">>> Tenchi-MCP: CRITICAL ERROR - Failed to load config at {:?}: {}", config_path, e);
        std::process::exit(1);
    }).unwrap();

    let client = OllamaClient::new(config.global.ollama_url.clone(), config.global.default_timeout);

    // Startup check: Verify if Ollama is running and models exist
    eprintln!(">>> Tenchi-MCP: Initializing and checking Ollama connectivity...");
    match client.list_models().await {
        Ok(installed_models) => {
            eprintln!(">>> Tenchi-MCP: Connected to Ollama successfully.");
            for model_cfg in &config.models {
                if !installed_models.contains(&model_cfg.name) && 
                   !installed_models.iter().any(|m| m.starts_with(&format!("{}:", model_cfg.name))) {
                    eprintln!(">>> Tenchi-MCP: WARNING - Model '{}' (Role: {}) is not installed in Ollama.", model_cfg.name, model_cfg.role);
                }
            }
        }
        Err(e) => {
            eprintln!(">>> Tenchi-MCP: WARNING - Could not connect to Ollama at {}: {}", config.global.ollama_url, e);
            eprintln!(">>> Tenchi-MCP: Please ensure Ollama is running.");
        }
    }

    let server_details = InitializeResult {
        server_info: Implementation {
            name: "Tenchi-MCP".into(),
            version: "0.1.0".into(),
            title: Some("Tenchi Hybrid Cloud/Local MCP Server".into()),
            description: Some("MCP server for delegating tasks to local Ollama models".into()),
            icons: vec![],
            website_url: None,
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: None,
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    let transport = StdioTransport::new(TransportOptions::default())?;
    let handler = TenchiHandler { config, client };

    let server = server_runtime::create_server(McpServerOptions {
        server_details,
        transport,
        handler: handler.to_mcp_server_handler(),
        task_store: None,
        client_task_store: None,
        message_observer: None,
    });

    eprintln!(">>> Tenchi-MCP: Server started and waiting for requests.");
    server.start().await
}
