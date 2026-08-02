#!/usr/bin/env bash
# One-liner Installer for Tenchi-MCP on macOS / Linux
set -e

echo ">>> Installing Tenchi-MCP for Antigravity 2.0 / CLI..."

TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo ">>> Cloning repository..."
git clone https://github.com/DovahkiinYuzuko/Tenchi-MCP.git "$TEMP_DIR"

cd "$TEMP_DIR"
echo ">>> Building release binary..."
cargo build --release

PLUGIN_DIR="$HOME/.gemini/config/plugins/tenchi-mcp"
mkdir -p "$PLUGIN_DIR"

BIN_PATH="$PLUGIN_DIR/tenchi-mcp"

cp "target/release/tenchi-mcp" "$PLUGIN_DIR/tenchi-mcp"
chmod +x "$PLUGIN_DIR/tenchi-mcp"
cp "models_config.toml" "$PLUGIN_DIR/models_config.toml"
cp "plugin.json" "$PLUGIN_DIR/plugin.json"

cat <<EOF > "$PLUGIN_DIR/mcp_config.json"
{
  "mcpServers": {
    "tenchi": {
      "command": "$BIN_PATH",
      "args": [],
      "env": {}
    }
  }
}
EOF

echo ">>> Tenchi-MCP successfully installed to $PLUGIN_DIR"
echo ">>> Binary path resolved: $BIN_PATH"
