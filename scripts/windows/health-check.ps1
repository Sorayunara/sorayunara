<#
.SYNOPSIS
    Performs server health check on a Sorayunara service instance.
#>
param(
    [string]$Endpoint = "http://localhost:8080/health"
)

Write-Host "=== 🔍 Sorayunara Service Health Check ===" -ForegroundColor Cyan
try {
    $resp = Invoke-RestMethod -Uri $Endpoint -TimeoutSec 5
    Write-Host "  ✅ Status: $($resp.status)" -ForegroundColor Green
    Write-Host "  ⏱️ Uptime: $($resp.uptime_seconds)s" -ForegroundColor Cyan
    Write-Host "  💾 Memory: $($resp.memory_mb) MB" -ForegroundColor Cyan
} catch {
    Write-Host "  ❌ Health check failed: $_" -ForegroundColor Red
    exit 1
}
