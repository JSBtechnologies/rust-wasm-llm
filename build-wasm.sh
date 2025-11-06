#!/bin/bash
# Build script for Rust WASM LLM

set -e  # Exit on error

echo "🦀 Building Rust WASM LLM..."

# Source Rust environment
source "$HOME/.cargo/env"

# Note: getrandom 0.3 with wasm_js feature handles WASM support automatically
# No need for --cfg wasm_js rustflags anymore

# Build with wasm-pack
echo "Running wasm-pack build..."
wasm-pack build --target web --out-dir pkg

echo "✅ WASM build complete!"
echo "📦 Output: pkg/"
echo ""
echo "Test the build:"
echo "  python3 -m http.server 8000"
echo "  Open http://localhost:8000/test.html"
