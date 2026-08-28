#!/usr/bin/env bash
set -e
echo "🔨 Building Sorayunara toolchain..."
cargo build --release --all-targets
echo "✅ Build completed successfully."
