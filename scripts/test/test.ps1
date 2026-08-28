# scripts/test/test.ps1 - Test suite runner
Write-Host "🧪 Running Sorayunara test suite..." -ForegroundColor Cyan
cargo test --all-targets
Write-Host "🧪 Running sample verification..." -ForegroundColor Cyan
cargo run --quiet -- run main.sora
Write-Host "✅ All tests and samples verified." -ForegroundColor Green
