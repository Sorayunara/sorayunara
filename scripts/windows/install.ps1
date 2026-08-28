<#
.SYNOPSIS
    Installs Sorayunara Server Runtime on Windows Server 2016..2025 (Server Core & Desktop Experience).
.PARAMETER InstallDir
    Target installation directory (Default: C:\Program Files\Sorayunara)
.PARAMETER DataDir
    Data and configuration directory (Default: C:\ProgramData\Sorayunara)
#>
param(
    [string]$InstallDir = "C:\Program Files\Sorayunara",
    [string]$DataDir = "C:\ProgramData\Sorayunara"
)

$ErrorActionPreference = "Stop"

Write-Host "=== 🌌 Sorayunara Windows Server Installation ===" -ForegroundColor Cyan

# 1. Create Core Directories
$binDir = Join-Path $InstallDir "bin"
$logsDir = Join-Path $DataDir "logs"
$configDir = Join-Path $DataDir "config"

foreach ($dir in @($binDir, $logsDir, $configDir)) {
    if (-not (Test-Path $dir)) {
        New-Item -Path $dir -ItemType Directory -Force | Out-Null
        Write-Host "  ✅ Created: $dir" -ForegroundColor Green
    }
}

# 2. Add to System PATH if not present
$currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
if ($currentPath -notlike "*$binDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$currentPath;$binDir", "Machine")
    Write-Host "  ✅ Added $binDir to Machine PATH." -ForegroundColor Green
}

Write-Host "=== Sorayunara Server Runtime Installed Successfully ===" -ForegroundColor Cyan
