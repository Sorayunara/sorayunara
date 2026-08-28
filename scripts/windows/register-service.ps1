<#
.SYNOPSIS
    Registers a Sorayunara executable as a managed Windows Service in SCM.
.PARAMETER ServiceName
    The internal SCM service name.
.PARAMETER BinaryPath
    Full path to the compiled Sorayunara .exe binary.
.PARAMETER Port
    Application listener port (Default: 8080).
#>
param(
    [Parameter(Mandatory=$true)]
    [string]$ServiceName,
    [Parameter(Mandatory=$true)]
    [string]$BinaryPath,
    [int]$Port = 8080
)

$ErrorActionPreference = "Stop"

Write-Host "=== 🌌 Registering Windows Service: $ServiceName ===" -ForegroundColor Cyan

if (-not (Test-Path $BinaryPath)) {
    throw "Executable not found at path: $BinaryPath"
}

# 1. Register with Service Control Manager (SCM)
$existing = Get-Service -Name $ServiceName -ErrorAction SilentlyContinue
if ($existing) {
    Write-Host "  ⚠️ Service '$ServiceName' already exists. Stopping and removing..." -ForegroundColor Yellow
    Stop-Service -Name $ServiceName -Force -ErrorAction SilentlyContinue
    sc.exe delete $ServiceName | Out-Null
    Start-Sleep -Seconds 2
}

New-Service -Name $ServiceName `
            -BinaryPathName $BinaryPath `
            -DisplayName "Sorayunara Application: $ServiceName" `
            -Description "Managed Windows Service built with Sorayunara Systems Language." `
            -StartupType Automatic | Out-Null

# 2. Configure Failure Recovery (Auto-restart)
sc.exe failure $ServiceName reset= 86400 actions= restart/30000/restart/60000/restart/120000 | Out-Null

Write-Host "  ✅ Service '$ServiceName' registered and configured with auto-recovery." -ForegroundColor Green
Write-Host "  🚀 Starting service..." -ForegroundColor Cyan
Start-Service -Name $ServiceName
Write-Host "  ✅ Service status: $( (Get-Service -Name $ServiceName).Status )" -ForegroundColor Green
