# One-liner Installer for Tenchi-MCP on Windows
$ErrorActionPreference = "Stop"

Write-Host ">>> Installing Tenchi-MCP for Antigravity 2.0 / CLI..." -ForegroundColor Cyan

$tempDir = Join-Path $env:TEMP "tenchi-mcp-install"
if (Test-Path $tempDir) {
    Remove-Item -Recurse -Force $tempDir
}

Write-Host ">>> Cloning repository..." -ForegroundColor Cyan
git clone https://github.com/DovahkiinYuzuko/Tenchi-MCP.git $tempDir

Set-Location $tempDir
Write-Host ">>> Building release binary..." -ForegroundColor Cyan
cargo build --release

$pluginDir = "$HOME\.gemini\config\plugins\tenchi-mcp"
if (!(Test-Path $pluginDir)) {
    New-Item -ItemType Directory -Path $pluginDir -Force | Out-Null
}

$binPath = Join-Path $pluginDir "tenchi-mcp.exe"
$binEscaped = $binPath.Replace("\", "\\")

Copy-Item "target\release\tenchi-mcp.exe" "$pluginDir\tenchi-mcp.exe" -Force
Copy-Item "models_config.toml" "$pluginDir\models_config.toml" -Force
Copy-Item "plugin.json" "$pluginDir\plugin.json" -Force

$mcpConfigJson = @"
{
  "mcpServers": {
    "tenchi": {
      "command": "$binEscaped",
      "args": [],
      "env": {}
    }
  }
}
"@

Set-Content -Path "$pluginDir\mcp_config.json" -Value $mcpConfigJson -Encoding UTF8

Write-Host ">>> Tenchi-MCP successfully installed to $pluginDir" -ForegroundColor Green
Write-Host ">>> Absolute binary path resolved: $binPath" -ForegroundColor Green
