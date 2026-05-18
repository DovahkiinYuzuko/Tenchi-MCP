# Tenchi-MCP

Tenchi-MCP (天地-MCP) is a hybrid cloud/local MCP server that bridges cloud LLMs with local Ollama instances.

## Features

- **Task Delegation**: Automatically routes tasks to specific local models based on descriptions.
- **Enhanced Configuration**: Per-model settings for temperature, context size, and system prompts.
- **Inference Monitoring**: Track elapsed time for local inferences.
- **Flexible TOML**: Easy-to-manage model configuration.

## Setup

1. Install [Ollama](https://ollama.com/).
2. Pull the models defined in your `models_config.toml`.
3. Build the project:
   ```bash
   cargo build --release
   ```
4. Configure your MCP client (like Gemini CLI or Claude Desktop) to use the binary.

## Configuration

Edit `models_config.toml` to add your local models:

```toml
[[models]]
name = "your-model-name"
role = "Coder"
description = "What this model is good at"
system_prompt = "Custom behavior"
[models.options]
temperature = 0.5
```
