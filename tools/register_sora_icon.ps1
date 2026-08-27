# PowerShell Script: Register .sora File Icon & Association Permanently on Windows
$ErrorActionPreference = "Stop"

$workspaceDir = Split-Path -Parent $PSScriptRoot
$icoPath = Join-Path $workspaceDir "assets\sora.ico"

if (-not (Test-Path $icoPath)) {
    Write-Error "Icon file not found at: $icoPath"
    exit 1
}

Write-Host "Registering .sora file extension with icon: $icoPath" -ForegroundColor Cyan

# 1. Register ProgID & Icon under HKCU (No admin rights required)
$progId = "Sorayunara.SourceFile"
$classesKey = "HKCU:\Software\Classes"

# Create Extension Association
New-Item -Path "$classesKey\.sora" -Force | Out-Null
Set-ItemProperty -Path "$classesKey\.sora" -Name "(default)" -Value $progId
Set-ItemProperty -Path "$classesKey\.sora" -Name "Content Type" -Value "text/x-sora"
Set-ItemProperty -Path "$classesKey\.sora" -Name "PerceivedType" -Value "text"

# Create ProgID
New-Item -Path "$classesKey\$progId" -Force | Out-Null
Set-ItemProperty -Path "$classesKey\$progId" -Name "(default)" -Value "Sorayunara Source File"
Set-ItemProperty -Path "$classesKey\$progId" -Name "FriendlyTypeName" -Value "Sorayunara Source File (.sora)"

# Set DefaultIcon
New-Item -Path "$classesKey\$progId\DefaultIcon" -Force | Out-Null
Set-ItemProperty -Path "$classesKey\$progId\DefaultIcon" -Name "(default)" -Value "`"$icoPath,0`""

Write-Host "Icon association registered successfully in HKCU." -ForegroundColor Green

# 2. Refresh Windows Explorer Shell Icon Cache
try {
    # Notify shell of association change
    Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class ShellNotifier {
    [DllImport("shell32.dll", CharSet = CharSet.Auto, SetLastError = true)]
    public static extern void SHChangeNotify(uint wEventId, uint uFlags, IntPtr dwItem1, IntPtr dwItem2);
}
"@
    [ShellNotifier]::SHChangeNotify(0x08000000, 0x0000, [IntPtr]::Zero, [IntPtr]::Zero) # SHCNE_ASSOCCHANGED
    Write-Host "Windows Shell Icon Cache notified & refreshed!" -ForegroundColor Green
} catch {
    Write-Warning "Shell notification error: $_"
}
