# scripts/build/build.ps1 - Build Sorayunara toolchain
Write-Host "🔨 Building Sorayunara toolchain..." -ForegroundColor Cyan
cargo build --release --all-targets
Write-Host "✅ Build completed successfully." -ForegroundColor Green
