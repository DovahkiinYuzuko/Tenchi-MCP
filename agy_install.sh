#!/usr/bin/env bash
# One-liner Installer for Tenchi-MCP on macOS / Linux
set -e

echo ">>> Installing Tenchi-MCP plugin via Antigravity CLI..."
agy plugin install https://github.com/DovahkiinYuzuko/Tenchi-MCP >/dev/null 2>&1 || true

PLUGIN_DIR="$HOME/.gemini/config/plugins/tenchi-mcp"
if [ ! -d "$PLUGIN_DIR" ]; then
    PLUGIN_DIR="$HOME/.gemini/antigravity-cli/plugins/tenchi-mcp"
fi
mkdir -p "$PLUGIN_DIR"

echo ">>> Downloading latest pre-compiled binary from GitHub Releases..."
OS_TYPE=$(uname -s | tr '[:upper:]' '[:lower:]')

if [ "$OS_TYPE" = "darwin" ]; then
    PATTERN="darwin"
else
    PATTERN="linux"
fi

RELEASE_JSON=$(curl -s https://api.github.com/repos/DovahkiinYuzuko/Tenchi-MCP/releases/latest || echo "")
DOWNLOAD_URL=$(echo "$RELEASE_JSON" | grep -o "https://[^\"]*${PATTERN}[^\"]*" | head -n 1 || echo "")

if [ -n "$DOWNLOAD_URL" ]; then
    curl -L "$DOWNLOAD_URL" -o "$PLUGIN_DIR/binary.tar.gz"
    tar -xzf "$PLUGIN_DIR/binary.tar.gz" -C "$PLUGIN_DIR"
    rm -f "$PLUGIN_DIR/binary.tar.gz"
    chmod +x "$PLUGIN_DIR/tenchi-mcp"
else
    echo ">>> Release binary asset not found. Building locally via cargo..."
    cargo build --release
    cp "target/release/tenchi-mcp" "$PLUGIN_DIR/tenchi-mcp"
    chmod +x "$PLUGIN_DIR/tenchi-mcp"
fi

BIN_PATH="$PLUGIN_DIR/tenchi-mcp"

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
echo ">>> Binary path set: $BIN_PATH"
