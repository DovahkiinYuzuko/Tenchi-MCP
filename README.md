# Tenchi-MCP

[日本語](#日本語) | [English](#english)

---

## 日本語

Tenchi-MCP（天地-MCP）は、クラウドベースの強力なLLM（GeminiやClaudeなど）と、ローカル環境で動作するLLM（Ollama経由）を連携させるためのハイブリッド推論オーケストレーターです。

単純なタスクや機密性の高いデータの処理をローカルモデルに委譲することで、クラウドのトークン消費を抑えつつ、セキュアで効率的な開発環境を実現します。

### 特徴

- **ハイブリッド推論**: タスクの性質に応じて、クラウドモデルからローカルモデルへ処理を自動的に委譲。
- **クロスプラットフォーム設計**: Windows, macOS, Linux をサポートするように設計されています（開発環境は Windows 11 です）。
- **柔軟な設定**: `models_config.toml` により、モデルごとの役割、優先度、システムプロンプト、各種パラメータを詳細に設定可能。
- **推論モニタリング**: ローカル推論の経過時間やステータスをリアルタイムで表示。

### パフォーマンスと制限事項

- **推論速度**: ローカル環境での推論速度は、お使いのハードウェア（CPU/GPU/VRAM）に大きく依存します。クラウドモデルと比較して応答に時間がかかる場合があることをあらかじめご了承ください。
- **動作確認**: 現在、Windows 11 環境での動作を確認しています。macOS および Linux については設計上考慮されていますが、実機での検証は未実施です。

### 前提条件

本ツールを使用するには、ローカル環境に **Ollama** がインストールされている必要があります。

- **Ollama 公式サイト**: [https://ollama.com/](https://ollama.com/)

インストール後、`models_config.toml` に設定する予定のモデルをあらかじめ `ollama run <model_name>` 等でプルしておいてください。

### インストール方法

お使いのクライアントに合わせて、以下の手順でインストールしてください。

> [!WARNING]
> Gemini CLIは2026年6月にサービス終了予定です。後継ツールである **Antigravity CLI** への移行を強く推奨します。

#### Antigravity 2.0 / Antigravity CLI ユーザー (推奨)
以下の1行コマンドを実行するだけで、自動的にプラグイン登録、最新ビルド済みバイナリのダウンロードおよび設定が完了します。

**Windows 11 (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/DovahkiinYuzuko/Tenchi-MCP/main/agy_install.ps1 | iex
```

**macOS / Linux (Bash):**
```bash
curl -fsSL https://raw.githubusercontent.com/DovahkiinYuzuko/Tenchi-MCP/main/agy_install.sh | bash
```

このワンライナーにより、`agy plugin install` が実行され、`~/.gemini/antigravity-cli/plugins/tenchi-mcp` に最新のリリースバイナリ、`plugin.json`、`mcp_config.json`、`models_config.toml` が全自動で配置・設定されます。

※ また、MCP サーバー起動時に Ollama (`ollama serve`) が未起動の場合、バックグラウンドでの自動起動を試みます。

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
# Ollama APIのベースURL / Base URL for Ollama API
ollama_url = "http://localhost:11434"
# デフォルトのタイムアウト（秒） / Default timeout in seconds
default_timeout = 300

# コーディング・ロジック実装用 / Specialized for coding and logic implementation
[[models]]
name = "qwen3.6-coder:27b"
role = "Coder"
description = "Specialized in software engineering. High performance in SWE-bench and logical reasoning."
priority = 1
system_prompt = "You are an expert software engineer. Provide accurate implementation and rigorous logic."

[models.options]
temperature = 0.2
top_p = 0.95
num_ctx = 32768
num_predict = -1

# 高度な推論・汎用アシスタント / Advanced reasoning and general assistant
[[models]]
name = "gemma4:31b"
role = "Expert"
description = "Google's high-density model. Strong reasoning and general intelligence."
priority = 2
system_prompt = "You are a highly intelligent and precise assistant. Provide detailed, accurate, and well-reasoned information."

[models.options]
temperature = 0.7
num_ctx = 32768

# エッジ・軽量環境用 / For edge and lightweight environments
[[models]]
name = "gemma4:e4b"
role = "Lite"
description = "Fast and efficient model for simple tasks and low-resource environments."
priority = 3
system_prompt = "You are a concise and efficient assistant. Provide short, direct, and accurate answers."

[models.options]
temperature = 0.5
num_ctx = 8192
low_vram = true
```

### 利用可能なツール

- `list_local_models`: 利用可能なローカルモデルの一覧と、それぞれの役割・説明をJSON形式で取得します。
- `local_generate`: 指定したローカルモデルに対して推論をリクエストします。

---

## English

Tenchi-MCP is a hybrid inference orchestrator that bridges powerful cloud-based LLMs (e.g., Gemini, Claude) with local LLM instances running via Ollama.

It enables a secure and efficient development environment by delegating simple tasks or sensitive data processing to local models, thereby reducing cloud token consumption.

### Features

- **Hybrid Inference**: Automatically delegate tasks from cloud models to local models based on task requirements.
- **Cross-Platform Design**: Designed to support Windows, macOS, and Linux (Primary development environment is Windows 11).
- **Flexible Configuration**: Fine-grained control over roles, priorities, system prompts, and parameters for each model via `models_config.toml`.
- **Inference Monitoring**: Real-time display of elapsed time and status for local inferences.

### Performance and Limitations

- **Inference Speed**: The speed of local inference heavily depends on your hardware (CPU/GPU/VRAM). Please be aware that responses may take significantly longer compared to cloud-based models.
- **Platform Verification**: Currently, operation has been verified in a Windows 11 environment. Support for macOS and Linux is included in the design, but has not yet been verified on actual hardware.

### Prerequisites

To use this tool, you must have **Ollama** installed on your local machine.

- **Ollama Official Website**: [https://ollama.com/](https://ollama.com/)

After installation, please ensure you have pulled the models you plan to use (e.g., via `ollama run <model_name>`) before starting the MCP server.

### Installation

Choose the appropriate installation method for your client.

> [!WARNING]
> Gemini CLI will be deprecated in June 2026. Transitioning to the successor, **Antigravity CLI**, is strongly recommended.

#### Antigravity 2.0 / Antigravity CLI Users (Recommended)
Run the following one-liner command in your terminal to automatically register the plugin, download the latest pre-compiled release binary, and configure absolute paths.

**Windows 11 (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/DovahkiinYuzuko/Tenchi-MCP/main/agy_install.ps1 | iex
```

**macOS / Linux (Bash):**
```bash
curl -fsSL https://raw.githubusercontent.com/DovahkiinYuzuko/Tenchi-MCP/main/agy_install.sh | bash
```

This one-liner executes `agy plugin install` and deploys the latest release binary along with `plugin.json`, `mcp_config.json`, and `models_config.toml` to `~/.gemini/antigravity-cli/plugins/tenchi-mcp`.

*Note: If `ollama serve` is not running when the MCP server starts, it will automatically attempt to launch Ollama in the background.*

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
# Ollama APIのベースURL / Base URL for Ollama API
ollama_url = "http://localhost:11434"
# デフォルトのタイムアウト（秒） / Default timeout in seconds
default_timeout = 300

# Specialized for coding and logic implementation
[[models]]
name = "qwen3.6-coder:27b"
role = "Coder"
description = "Specialized in software engineering. High performance in SWE-bench and logical reasoning."
priority = 1
system_prompt = "You are an expert software engineer. Provide accurate implementation and rigorous logic."

[models.options]
temperature = 0.2
top_p = 0.95
num_ctx = 32768
num_predict = -1

# Advanced reasoning and general assistant
[[models]]
name = "gemma4:31b"
role = "Expert"
description = "Google's high-density model. Strong reasoning and general intelligence."
priority = 2
system_prompt = "You are a highly intelligent and precise assistant. Provide detailed, accurate, and well-reasoned information."

[models.options]
temperature = 0.7
num_ctx = 32768

# For edge and lightweight environments
[[models]]
name = "gemma4:e4b"
role = "Lite"
description = "Fast and efficient model for simple tasks and low-resource environments."
priority = 3
system_prompt = "You are a concise and efficient assistant. Provide short, direct, and accurate answers."

[models.options]
temperature = 0.5
num_ctx = 8192
low_vram = true
```

### Available Tools

- `list_local_models`: Retrieves a list of available local models with their roles and descriptions in JSON format.
- `local_generate`: Requests inference from a specified local model.
