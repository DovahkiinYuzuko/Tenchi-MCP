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
                // Sort by priority (higher priority first or lower value first? usually lower value = higher priority)
                // Let's assume lower value = higher priority as per usual conventions
                models.sort_by_key(|m| m.priority);

                let models_info = models.iter().map(|m| {
                    format!("Name: {}, Role: {}, Priority: {}, Description: {}", m.name, m.role, m.priority, m.description)
                }).collect::<Vec<_>>().join("\n");
                
                Ok(CallToolResult::text_content(vec![models_info.into()]))
            }
            "local_generate" => {
                let args: LocalGenerateTool = serde_json::from_value(
                    serde_json::to_value(&params.arguments).unwrap()
                ).map_err(|e| CallToolError::invalid_arguments("local_generate", Some(e.to_string())))?;

                let model_cfg = self.config.models.iter()
                    .find(|m| m.name == args.model_name)
                    .ok_or_else(|| CallToolError::unknown_tool(format!("Model {} not found in config", args.model_name)))?;

                match self.client.generate(
                    &model_cfg.name, 
                    &model_cfg.system_prompt, 
                    &args.prompt, 
                    model_cfg.options.clone(),
                    model_cfg.runtime.clone()
                ).await {
                    Ok(response) => Ok(CallToolResult::text_content(vec![response.into()])),
                    Err(e) => Ok(CallToolResult::text_content(vec![format!("Error: {}", e).into()])),
                }
            }
            _ => Err(CallToolError::unknown_tool(params.name)),
        }
    }
}

#[tokio::main]
async fn main() -> SdkResult<()> {
    let config = Config::from_file("models_config.toml").map_err(|e| {
        eprintln!("Failed to load config: {}", e);
        std::process::exit(1);
    }).unwrap();

    let client = OllamaClient::new(config.global.ollama_url.clone(), config.global.default_timeout);

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

    server.start().await
}
