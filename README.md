# Tenchi-MCP

[日本語](#日本語) | [English](#english)

---

## 日本語

Tenchi-MCP（天地-MCP）は、クラウド上の強力なLLM（GeminiやClaudeなど）と、ローカル環境で動作するLLM（Ollama経由）を連携させるためのハイブリッド推論オーケストレーターです。

単純なタスクや機密性の高い処理をローカルモデルに委譲することで、クラウドのトークン消費を抑えつつ、セキュアで効率的な開発環境を実現します。

### 🚀 特徴

- **ハイブリッド推論**: タスクの内容に応じて、クラウドモデルからローカルモデルへ処理を自動的に委譲。
- **自動ビルド対応**: Gemini CLIの拡張機能としてインストールする場合、ビルドからバイナリの配置まで全自動で完了。
- **柔軟な設定**: `models_config.toml` でモデルごとの役割、優先度、システムプロンプト、各種パラメータを詳細に設定可能。
- **リアルタイム監視**: ローカル推論の経過時間やステータスをリアルタイムで表示。

### 📦 インストール方法

お使いのクライアントに合わせて、以下の手順でインストールしてください。

#### 1. Gemini CLI ユーザー（推奨・最も簡単）
Gemini CLIを使用している場合、コマンド一発でインストールからビルドまで完了します。

```powershell
gemini extensions install https://github.com/DovahkiinYuzuko/Tenchi-MCP
```
※インストール時に自動的に Rust のビルドが走り、`bin` フォルダにバイナリが生成されます。

#### 2. Claude Code ユーザー
1. リポジトリをクローンしてビルドします。
   ```powershell
   git clone https://github.com/DovahkiinYuzuko/Tenchi-MCP
   cd Tenchi-MCP
   cargo build --release
   New-Item -ItemType Directory -Force bin
   Copy-Item target/release/tenchi-mcp.exe bin/tenchi-mcp.exe
   ```
2. MCPサーバーを追加します。
   ```powershell
   claude mcp add tenchi-mcp -- ./bin/tenchi-mcp.exe
   ```

#### 3. Codex CLI (Copilot) ユーザー
1. クローンしてビルドします（上記「Claude Code」の手順 1 と同じ）。
2. MCPサーバーを追加します。
   ```powershell
   codex mcp add tenchi-mcp --command ./bin/tenchi-mcp.exe
   ```

#### 4. Claude Desktop ユーザー
1. クローンしてビルドします（上記「Claude Code」の手順 1 と同じ）。
2. 設定ファイル（`claude_desktop_config.json`）を編集します。
   ```json
   {
     "mcpServers": {
       "tenchi-mcp": {
         "command": "C:\\絶対パス\\Tenchi-MCP\\bin\\tenchi-mcp.exe"
       }
     }
   }
   ```
   ※パスは必ず **絶対パス** で指定してください。

### ⚙️ 設定方法 (`models_config.toml`)

`models_config.toml` を編集することで、使用するローカルモデルを管理できます。

```toml
[global]
ollama_url = "http://localhost:11434" # OllamaのAPI URL
default_timeout = 60                   # タイムアウト（秒）

[[models]]
name = "llama3:8b"           # Ollamaでのモデル名
role = "Generalist"           # モデルの役割
description = "一般的なタスク向け" # エージェントが判断するための説明
priority = 1                  # 表示優先度（昇順）
system_prompt = "あなたは有能なアシスタントです。" # 専用のシステムプロンプト

[models.options]
temperature = 0.7             # 創造性の調整
num_ctx = 4096                # コンテキストサイズ
```

### 🛠️ 利用可能なツール

- `list_local_models`: 利用可能なローカルモデルの一覧と、それぞれの役割・説明を取得します。
- `local_generate`: 指定したローカルモデルに対して推論をリクエストします。

---

## English

Tenchi-MCP is a hybrid inference orchestrator designed to bridge powerful cloud-based LLMs (e.g., Gemini, Claude) with local LLM instances (via Ollama).

By delegating simple tasks or sensitive data processing to local models, it enables a secure and efficient development environment while reducing cloud token consumption.

### 🚀 Features

- **Hybrid Inference**: Automatically delegate tasks from cloud models to local models based on requirements.
- **Automatic Build Support**: Full automation from building to binary placement when installed as a Gemini CLI extension.
- **Flexible Configuration**: Fine-grained control over roles, priorities, system prompts, and parameters per model via `models_config.toml`.
- **Real-time Monitoring**: Real-time display of elapsed time and status for local inferences.

### 📦 Installation

Choose the installation method based on your client.

#### 1. Gemini CLI Users (Recommended / Easiest)
If you are using Gemini CLI, installation and building are completed with a single command.

```powershell
gemini extensions install https://github.com/DovahkiinYuzuko/Tenchi-MCP
```
*The Rust project will be built automatically upon installation, and the binary will be generated in the `bin` folder.*

#### 2. Claude Code Users
1. Clone the repository and build it.
   ```powershell
   git clone https://github.com/DovahkiinYuzuko/Tenchi-MCP
   cd Tenchi-MCP
   cargo build --release
   New-Item -ItemType Directory -Force bin
   Copy-Item target/release/tenchi-mcp.exe bin/tenchi-mcp.exe
   ```
2. Add the MCP server.
   ```powershell
   claude mcp add tenchi-mcp -- ./bin/tenchi-mcp.exe
   ```

#### 3. Codex CLI (Copilot) Users
1. Clone and build (same as step 1 for "Claude Code").
2. Add the MCP server.
   ```powershell
   codex mcp add tenchi-mcp --command ./bin/tenchi-mcp.exe
   ```

#### 4. Claude Desktop Users
1. Clone and build (same as step 1 for "Claude Code").
2. Edit the configuration file (`claude_desktop_config.json`).
   ```json
   {
     "mcpServers": {
       "tenchi-mcp": {
         "command": "C:\\Absolute\\Path\\Tenchi-MCP\\bin\\tenchi-mcp.exe"
       }
     }
   }
   ```
   *Note: Ensure you use the **absolute path** for the command.*

### ⚙️ Configuration (`models_config.toml`)

Manage your local models by editing `models_config.toml`.

```toml
[global]
ollama_url = "http://localhost:11434" # Ollama API URL
default_timeout = 60                   # Timeout in seconds

[[models]]
name = "llama3:8b"           # Model name in Ollama
role = "Generalist"           # Role of the model
description = "For general tasks" # Description for the agent to decide
priority = 1                  # Display priority (ascending)
system_prompt = "You are a helpful assistant." # Custom system prompt

[models.options]
temperature = 0.7             # Adjust creativity
num_ctx = 4096                # Context size
```

### 🛠️ Available Tools

- `list_local_models`: Retrieves a list of available local models with their roles and descriptions.
- `local_generate`: Requests inference from a specified local model.
