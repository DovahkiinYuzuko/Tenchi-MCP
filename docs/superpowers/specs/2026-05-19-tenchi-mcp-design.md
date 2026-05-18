# Tenchi-MCP 設計書 (v1.1)

## 1. 概要
「天地-MCP」は、クラウドLLM（Gemini等）とローカルLLM（Ollama）を連携させるためのRust製MCPサーバーである。
モデルごとに詳細なパラメータ設定（推論設定、環境設定）を可能にし、ユーザーのPC環境に最適化されたハイブリッド推論環境を提供する。

## 2. アーキテクチャ
- **Frontend**: MCPクライアント (Gemini CLI, etc.)
- **Middleware**: Tenchi-MCP (Rust)
- **Backend**: Ollama API

## 3. 設定ファイル仕様 (`models_config.toml`)

```toml
[global]
ollama_url = "http://localhost:11434"
default_timeout = 300 # 秒

[[models]]
name = "sushi-coder-custom:latest"
role = "Coder"
description = "コード生成と微修正のメインモデル。"
priority = 1
system_prompt = "あなたは優秀なエンジニアです。"

# モデル固有のパラメータ（省略時はOllamaのデフォルト）
[models.options]
num_ctx = 8192
temperature = 0.5
repeat_penalty = 1.2
num_predict = 2048
stop = ["\nUser:", "###"]

# モデル固有のランタイム設定（省略可能）
[models.runtime]
num_thread = 6
low_vram = true

[[models]]
name = "carstenuhlig/omnicoder-9b:latest"
role = "Reviewer"
description = "コードレビューとリサーチ用。"
priority = 2
# optionsを省略した場合はデフォルトで動作
```

## 4. 主要機能
- **階層的なパラメータ管理**: グローバル設定とモデル固有設定の統合。
- **動的なモデル選択**: `description` や `role` に基づく委譲。
- **推論監視**: 経過時間の表示とタイムアウト制御。
- **Ollama API 連携**: JSON形式でのリクエスト/レスポンス処理。

## 5. 実装詳細 (Rust)
- **Config Parser**: `serde` と `toml` クレートを使用した柔軟なパース。
- **API Client**: `reqwest` による非同期通信。
- **MCP Framework**: `mcp-sdk-rs` によるツール公開。
