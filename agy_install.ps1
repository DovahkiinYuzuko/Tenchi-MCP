# One-liner Installer for Tenchi-MCP on Windows
$ErrorActionPreference = "Stop"

Write-Host ">>> Installing Tenchi-MCP plugin via Antigravity CLI..." -ForegroundColor Cyan
agy plugin install https://github.com/DovahkiinYuzuko/Tenchi-MCP

$pluginDir = "$HOME\.gemini\antigravity-cli\plugins\tenchi-mcp"
if (!(Test-Path $pluginDir)) {
    $pluginDir = "$HOME\.gemini\config\plugins\tenchi-mcp"
}
if (!(Test-Path $pluginDir)) {
    New-Item -ItemType Directory -Path $pluginDir -Force | Out-Null
}

Write-Host ">>> Downloading latest pre-compiled binary from GitHub Releases..." -ForegroundColor Cyan
try {
    $releaseUrl = "https://api.github.com/repos/DovahkiinYuzuko/Tenchi-MCP/releases/latest"
    $latestRelease = Invoke-RestMethod -Uri $releaseUrl
    $winAsset = $latestRelease.assets | Where-Object { $_.name -like "*win32*" -or $_.name -like "*windows*" }

    if ($winAsset) {
        $zipPath = Join-Path $pluginDir "binary.zip"
        Invoke-WebRequest -Uri $winAsset.browser_download_url -OutFile $zipPath
        Expand-Archive -Path $zipPath -DestinationPath $pluginDir -Force
        Remove-Item $zipPath -Force
    } else {
        Write-Host ">>> Release binary asset not found. Building locally..." -ForegroundColor Yellow
        cargo build --release
        Copy-Item "target\release\tenchi-mcp.exe" "$pluginDir\tenchi-mcp.exe" -Force
    }
} catch {
    Write-Host ">>> Error fetching release binary ($_.Exception.Message). Building locally..." -ForegroundColor Yellow
    cargo build --release
    Copy-Item "target\release\tenchi-mcp.exe" "$pluginDir\tenchi-mcp.exe" -Force
}

$binPath = Join-Path $pluginDir "tenchi-mcp.exe"
$binEscaped = $binPath.Replace("\", "\\")

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
Write-Host ">>> Absolute binary path set: $binPath" -ForegroundColor Green
