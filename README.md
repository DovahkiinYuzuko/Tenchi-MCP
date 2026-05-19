# Tenchi-MCP

[日本語](#日本語) | [English](#english)

---

## 日本語

Tenchi-MCP（天地-MCP）は、クラウドベースの強力なLLM（GeminiやClaudeなど）と、ローカル環境で動作するLLM（Ollama経由）を連携させるためのハイブリッド推論オーケストレーターです。

単純なタスクや機密性の高いデータの処理をローカルモデルに委譲することで、クラウドのトークン消費を抑えつつ、セキュアで効率的な開発環境を実現します。

### 主な機能

- **ハイブリッド推論**: タスクの性質に応じて、クラウドモデルからローカルモデルへ処理を自動的に委譲。
- **クロスプラットフォーム対応**: Windows、macOS、Linuxのすべてで動作確認済み。
- **柔軟な設定**: `models_config.toml` により、モデルごとの役割、優先度、システムプロンプト、各種パラメータを詳細に設定可能。
- **推論モニタリング**: ローカル推論の経過時間やステータスをリアルタイムで表示。

### インストール方法

お使いのクライアントに合わせて、以下の手順でインストールしてください。

#### 1. Gemini CLI ユーザー
Gemini CLIを使用している場合、拡張機能としてインストールすることで自動ビルドが利用可能です。

```bash
gemini extensions install https://github.com/DovahkiinYuzuko/Tenchi-MCP
```
※インストール時にRustのビルドが自動的に実行されます。

#### 2. Claude Code ユーザー
1. リポジトリをクローンしてビルドします。
   ```bash
   git clone https://github.com/DovahkiinYuzuko/Tenchi-MCP
   cd Tenchi-MCP
   cargo build --release
   ```
2. MCPサーバーを追加します。
   ```bash
   # Windowsの場合
   claude mcp add tenchi-mcp -- ./target/release/tenchi-mcp.exe
   
   # macOS / Linux の場合
   claude mcp add tenchi-mcp -- ./target/release/tenchi-mcp
   ```

#### 3. Codex CLI (OpenAI) ユーザー
Codex CLIは、OpenAIが提供する自律型ソフトウェアエンジニアリングエージェントプラットフォームです。

1. クローンしてビルドします（上記「Claude Code」の手順 1 と同じ）。
2. `~/.codex/config.toml` にサーバーを追加します。
   ```bash
   codex mcp add tenchi-mcp --command ./target/release/tenchi-mcp
   ```
   ※Windowsの場合は拡張子 `.exe` を含めてください。

#### 4. Claude Desktop ユーザー
1. クローンしてビルドします（上記「Claude Code」の手順 1 と同じ）。
2. 設定ファイル（`claude_desktop_config.json`）を編集します。
   ```json
   {
     "mcpServers": {
       "tenchi-mcp": {
         "command": "/path/to/Tenchi-MCP/target/release/tenchi-mcp"
       }
     }
   }
   ```
   ※パスは必ず **絶対パス** で指定し、OSに合わせて実行ファイルの拡張子（`.exe` 等）を適切に設定してください。

### 設定方法 (`models_config.toml`)

`models_config.toml` で使用するローカルモデルを詳細に制御できます。

```toml
[global]
ollama_url = "http://localhost:11434" # OllamaのAPI URL
default_timeout = 60                   # タイムアウト（秒）

[[models]]
name = "llama3:8b"           # Ollamaでのモデル名
role = "Generalist"           # モデルの役割
description = "一般的なタスク向け" # エージェントが判断するための説明
priority = 1                  # 表示優先度
system_prompt = "あなたは有能なアシスタントです。" # システムプロンプト

[models.options]
temperature = 0.7             # 生成の多様性
num_ctx = 4096                # コンテキストサイズ
```

### 利用可能なツール

- `list_local_models`: 利用可能なローカルモデルの一覧と、それぞれの役割・説明を取得します。
- `local_generate`: 指定したローカルモデルに対して推論をリクエストします。

---

## English

Tenchi-MCP is a hybrid inference orchestrator that bridges powerful cloud-based LLMs (e.g., Gemini, Claude) with local LLM instances running via Ollama.

It enables a secure and efficient development environment by delegating simple tasks or sensitive data processing to local models, thereby reducing cloud token consumption.

### Features

- **Hybrid Inference**: Automatically delegate tasks from cloud models to local models based on task requirements.
- **Cross-Platform**: Fully supported on Windows, macOS, and Linux.
- **Flexible Configuration**: Fine-grained control over roles, priorities, system prompts, and parameters for each model via `models_config.toml`.
- **Inference Monitoring**: Real-time display of elapsed time and status for local inferences.

### Installation

Choose the appropriate installation method for your client.

#### 1. Gemini CLI Users
When using Gemini CLI, you can take advantage of automatic building by installing it as an extension.

```bash
gemini extensions install https://github.com/DovahkiinYuzuko/Tenchi-MCP
```
*The Rust project will be built automatically upon installation.*

#### 2. Claude Code Users
1. Clone the repository and build it.
   ```bash
   git clone https://github.com/DovahkiinYuzuko/Tenchi-MCP
   cd Tenchi-MCP
   cargo build --release
   ```
2. Add the MCP server.
   ```bash
   # On Windows
   claude mcp add tenchi-mcp -- ./target/release/tenchi-mcp.exe
   
   # On macOS / Linux
   claude mcp add tenchi-mcp -- ./target/release/tenchi-mcp
   ```

#### 3. Codex CLI (OpenAI) Users
Codex CLI is an autonomous software engineering agent platform provided by OpenAI.

1. Clone and build (same as step 1 for "Claude Code").
2. Add the server to `~/.codex/config.toml`.
   ```bash
   codex mcp add tenchi-mcp --command ./target/release/tenchi-mcp
   ```
   *Note: Include the `.exe` extension on Windows.*

#### 4. Claude Desktop Users
1. Clone and build (same as step 1 for "Claude Code").
2. Edit the configuration file (`claude_desktop_config.json`).
   ```json
   {
     "mcpServers": {
       "tenchi-mcp": {
         "command": "/path/to/Tenchi-MCP/target/release/tenchi-mcp"
       }
     }
   }
   ```
   *Note: Ensure you use an **absolute path** and include the appropriate file extension for your OS.*

### Configuration (`models_config.toml`)

Manage your local models by editing `models_config.toml`.

```toml
[global]
ollama_url = "http://localhost:11434" # Ollama API URL
default_timeout = 60                   # Timeout in seconds

[[models]]
name = "llama3:8b"           # Model name in Ollama
role = "Generalist"           # Role of the model
description = "For general tasks" # Description for the agent's decision logic
priority = 1                  # Display priority
system_prompt = "You are a helpful assistant." # Custom system prompt

[models.options]
temperature = 0.7             # Creativity adjustment
num_ctx = 4096                # Context size
```

### Available Tools

- `list_local_models`: Retrieves a list of available local models with their roles and descriptions.
- `local_generate`: Requests inference from a specified local model.
