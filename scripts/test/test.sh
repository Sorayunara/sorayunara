#!/usr/bin/env bash
set -e
echo "🧪 Running Sorayunara test suite..."
cargo test --all-targets
echo "🧪 Running sample verification..."
cargo run --quiet -- run main.sora
echo "✅ All tests and samples verified."
