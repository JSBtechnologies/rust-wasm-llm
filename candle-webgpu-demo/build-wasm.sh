#!/bin/bash
set -e

echo "🔨 Building Candle WebGPU Demo for WASM..."
echo ""

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found!"
    echo "Install it with:"
    echo "  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# Build the WASM package
echo "Building WASM package..."
wasm-pack build --target web --out-dir pkg

echo ""
echo "✅ Build complete!"
echo ""
echo "📦 Generated files:"
echo "  - pkg/candle_webgpu_demo.js"
echo "  - pkg/candle_webgpu_demo_bg.wasm"
echo "  - pkg/candle_webgpu_demo.d.ts"
echo ""
echo "🚀 To test in browser:"
echo "  1. Install a local server:"
echo "     cargo install basic-http-server"
echo "     # or: npm install -g http-server"
echo ""
echo "  2. Run the server:"
echo "     basic-http-server ."
echo "     # or: http-server . -p 8080"
echo ""
echo "  3. Open: http://localhost:8000"
echo ""
echo "Make sure you're using a WebGPU-capable browser:"
echo "  • Chrome/Edge 113+"
echo "  • Firefox Nightly (enable in about:config)"
echo "  • Safari Technology Preview"
