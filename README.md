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

`models_config.toml` を編集することで、使用するローカルモデルと推論パラメータを詳細に制御できます。

#### グローバル設定 (`[global]`)
- `ollama_url`: Ollama APIのベースURL（デフォルト: "http://localhost:11434"）
- `default_timeout`: HTTPリクエストのタイムアウト秒数。

#### モデル設定 (`[[models]]`)
- `name`: Ollamaに登録されているモデル名。
- `role`: モデルの役割（例: "Coder", "Reviewer", "Generalist"）。
- `description`: クラウドエージェントがタスク委譲を判断するための詳細な説明。
- `priority`: モデル一覧の表示優先度。
- `system_prompt`: 該当モデルでの推論時に使用される固有のシステムプロンプト。

#### 推論オプション (`[models.options]`)
LLMの生成挙動を制御します。
- `temperature`: 生成の多様性（0.0 - 1.0）。
- `top_p`: 核サンプリングの閾値。
- `top_k`: 候補単語数の制限。
- `repeat_penalty`: 繰り返しの抑制。
- `num_ctx`: コンテキストサイズ（トークン数）。
- `num_predict`: 最大生成トークン数。
- `stop`: 停止シークエンスのリスト（例: `["\nUser:"]`）。
- `seed`: 乱数シード。
- その他、Ollama APIがサポートする任意のオプション。

#### 実行時設定 (`[models.runtime]`)
実行リソースを制御します。
- `num_thread`: 使用するCPUスレッド数。
- `num_gpu`: GPUにオフロードするレイヤー数。
- `low_vram`: VRAM節約モードの有効化（true/false）。

#### 設定例 (`models_config.toml`)

```toml
[global]
ollama_url = "http://localhost:11434"
default_timeout = 300

# コード生成に特化したモデルの例
[[models]]
name = "sushi-coder-custom:latest"
role = "Coder"
description = "コード生成およびロジック実装用のメインモデル"
priority = 1
system_prompt = "あなたは熟練したソフトウェアエンジニアです。簡潔で正確なコードを提供してください。"

[models.options]
temperature = 0.2
num_ctx = 8192

# コードレビューに特化したリソース制限付きの例
[[models]]
name = "carstenuhlig/omnicoder-9b:latest"
role = "Reviewer"
description = "コードレビューや調査に特化"
priority = 2
system_prompt = "あなたはシニアコードレビュアーです。批判的かつ建設的なフィードバックを提供してください。"

[models.options]
temperature = 0.5
num_ctx = 16384

[models.runtime]
num_thread = 6
low_vram = true
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

### Prerequisites

To use this tool, you must have **Ollama** installed on your local machine.

- **Ollama Official Website**: [https://ollama.com/](https://ollama.com/)

After installation, please ensure you have pulled the models you plan to use (e.g., via `ollama run <model_name>`) before starting the MCP server.

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
2. Add the server using the `codex mcp add` command.
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

Manage your local models and inference parameters in detail by editing `models_config.toml`.

#### Global Configuration (`[global]`)
- `ollama_url`: Base URL for the Ollama API (default: "http://localhost:11434").
- `default_timeout`: Timeout for HTTP requests in seconds.

#### Model Configuration (`[[models]]`)
- `name`: Model name registered in Ollama.
- `role`: Role of the model (e.g., "Coder", "Reviewer", "Generalist").
- `description`: Detailed description for the cloud agent's delegation logic.
- `priority`: Display priority in the model list.
- `system_prompt`: Specific system prompt used during inference with this model.

#### Inference Options (`[models.options]`)
Controls the behavior of the LLM generation.
- `temperature`: Creativity adjustment (0.0 - 1.0).
- `top_p`: Threshold for nucleus sampling.
- `top_k`: Limit on the number of candidate words.
- `repeat_penalty`: Penalty for repeating sequences.
- `num_ctx`: Context window size (in tokens).
- `num_predict`: Maximum number of tokens to generate.
- `stop`: List of stop sequences (e.g., `["\nUser:"]`).
- `seed`: Random seed for reproducibility.
- Any other options supported by the Ollama API.

#### Runtime Configuration (`[models.runtime]`)
Controls execution resources.
- `num_thread`: Number of CPU threads to use.
- `num_gpu`: Number of layers to offload to the GPU.
- `low_vram`: Enable low VRAM mode (true/false).

#### Configuration Example (`models_config.toml`)

```toml
[global]
ollama_url = "http://localhost:11434"
default_timeout = 300

# Example of a model specialized for code generation
[[models]]
name = "sushi-coder-custom:latest"
role = "Coder"
description = "Main model for code generation and logic implementation"
priority = 1
system_prompt = "You are an expert software engineer. Provide concise and accurate code."

[models.options]
temperature = 0.2
num_ctx = 8192

# Example of a model specialized for code review with resource limits
[[models]]
name = "carstenuhlig/omnicoder-9b:latest"
role = "Reviewer"
description = "Specialized in code review and research"
priority = 2
system_prompt = "You are a senior code reviewer. Provide critical and constructive feedback."

[models.options]
temperature = 0.5
num_ctx = 16384

[models.runtime]
num_thread = 6
low_vram = true
```

### Available Tools

- `list_local_models`: Retrieves a list of available local models with their roles and descriptions.
- `local_generate`: Requests inference from a specified local model.
