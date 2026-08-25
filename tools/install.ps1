# Sorayunara Windows PowerShell Installer
$ErrorActionPreference = "Stop"

Write-Host "🌸 Installing Sorayunara Programming Language Toolchain for Windows..." -ForegroundColor Cyan

$Target = "x86_64-pc-windows-msvc"
$InstallDir = "$HOME\.sorayunara\bin"

try {
    $Release = Invoke-RestMethod -Uri "https://api.github.com/repos/Sorayunara/sorayunara/releases/latest"
    $LatestTag = $Release.tag_name
} catch {
    $LatestTag = "v0.1.0"
}

$DownloadUrl = "https://github.com/Sorayunara/sorayunara/releases/download/$LatestTag/sora-$LatestTag-$Target.zip"
$TempZip = "$env:TEMP\sora-$LatestTag.zip"
$TempExtract = "$env:TEMP\sora-extract"

Write-Host "Downloading Sorayunara $LatestTag ($Target)..."
Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

Expand-Archive -Path $TempZip -DestinationPath $TempExtract -Force
Copy-Item -Path "$TempExtract\sora-$LatestTag-$Target\sorayunara.exe" -Destination "$InstallDir\sora.exe" -Force

Remove-Item -Path $TempZip -Force -ErrorAction SilentlyContinue
Remove-Item -Path $TempExtract -Recurse -Force -ErrorAction SilentlyContinue

# Add to User PATH if not already present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-Host "Added $InstallDir to User PATH." -ForegroundColor Green
}

Write-Host "✅ Sorayunara $LatestTag successfully installed to $InstallDir\sora.exe!" -ForegroundColor Green
Write-Host "Restart your terminal and run: sora --version" -ForegroundColor Yellow
