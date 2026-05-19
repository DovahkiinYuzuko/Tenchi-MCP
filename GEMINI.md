# Tenchi-MCP AI Agent Instructions

## 1. Introduction
Tenchi-MCP is a Model Context Protocol (MCP) server designed to enable a hybrid inference architecture between cloud-based Large Language Models (LLMs) and local LLM instances (via Ollama). 
As an AI agent, you should use this server to delegate specific tasks to local models to reduce cloud token consumption, handle sensitive data locally, or leverage specialized local models.

## 2. Architecture Overview
- **Primary Agent (You)**: Cloud LLM (e.g., Gemini, Claude).
- **Secondary Agents**: Local LLMs managed by Ollama.
- **Middleware**: Tenchi-MCP (Rust), providing tools to communicate with local models.

## 3. Available Tools

### 3.1. `list_local_models`
- **Purpose**: Retrieves the list of locally available models defined in `models_config.toml`.
- **Output**: A JSON string containing an array of model objects (name, role, priority, and description).
- **Usage**: Call this tool at the beginning of a task to understand which local resources are available and their specific strengths.

### 3.2. `local_generate`
- **Purpose**: Executes an inference request on a specific local model.
- **Arguments**:
  - `model_name`: The exact name of the model as returned by `list_local_models`.
  - `prompt`: The specific instruction or task to be performed by the local model.
- **Output**: The generated text content from the local model.
- **Behavior**: 
  - The server automatically strips reasoning/thought blocks (e.g., `<think>`) and metadata.
  - The server injects the model-specific `system_prompt` defined in the configuration.
  - Real-time progress is printed to `stderr` by the server.

## 4. Decision Logic for Task Delegation
You must decide whether to process a task yourself (Cloud) or delegate it to Tenchi-MCP (Local) based on the following criteria:

### 4.1. When to use Local Inference
- **Repetitive/Banal Tasks**: Simple code refactoring, boilerplate generation, or syntax checks.
- **Specialized Tasks**: When a local model's `description` explicitly mentions a specialty (e.g., "Sushi-Coder" for Rust logic).
- **Token Efficiency**: When the task is long and would consume significant cloud tokens, but doesn't require "Extreme Reasoning" only available in cloud models.

### 4.2. When to stay in Cloud
- **High-Level Architectural Planning**: Large-scale system design or complex cross-file analysis.
- **Extreme Context**: When the required context exceeds the `context_size` defined in the local model's configuration.
- **Uncertainty**: When local models have repeatedly failed or provided suboptimal results for the current task.

## 5. Execution Protocol
1. **Discover**: Call `list_local_models` to refresh your knowledge of the local environment.
2. **Evaluate**: Compare the user's request with the roles and descriptions of the local models.
3. **Execute**: Call `local_generate` with a clear, concise prompt.
4. **Integrate**: Incorporate the local model's output into your final response to the user.

## 6. Error Handling
- If a model is reported as "not installed" during startup, do not attempt to use it.
- If `local_generate` returns an error (e.g., connection timeout), fall back to cloud inference or notify the user.

---
Note: This document provides exhaustive guidance for AI agents and must not be truncated or compressed in future summaries.


---

## Gemini CLI Integration Protocol

To ensure reliable operation within Gemini-CLI, the following constraints MUST be followed:

### 1. Tool Naming
Gemini-CLI automatically prefixes MCP tools. You MUST invoke the tools using their fully qualified names:
- mcp_tenchi_list_local_models
- mcp_tenchi_local_generate

### 2. Execution Constraints (Crucial)
- **Wait for Previous**: You MUST set wait_for_previous: true for all calls to mcp_tenchi_* tools. This prevents parallel execution collisions and ensure stability.

### 3. Workflow
1. **Discovery**: Call mcp_tenchi_list_local_models first to identify available models and their roles.
2. **Evaluation**: Compare the user's request with the ole and description of each local model.
3. **Delegation**: If a task is suitable for local inference (e.g., boilerplate code, simple refactoring, syntax checks), use mcp_tenchi_local_generate.
4. **Integration**: Combine the local model's output into your final response to the user.

### 4. Error Handling
If a local model is unavailable or a call fails, inform the user about the specific issue (e.g., "Model not installed" or "Ollama not running") and offer to process the task using the Cloud LLM.