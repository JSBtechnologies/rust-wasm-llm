# Candle WebGPU Browser Demo

This demo shows the Candle ML framework running entirely in your browser with GPU acceleration via WebGPU!

## What This Demo Does

The demo runs on your GPU in the browser and performs:
- **Matrix multiplication** (2×2 matrices)
- **Activation functions** (ReLU, GELU)
- **Element-wise operations** (addition, multiplication)

All computations happen on your GPU using WebGPU, with zero server involvement!

## Quick Start

### 1. Install Prerequisites

**Install wasm-pack** (one-time setup):
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

**Install a local web server** (pick one):
```bash
# Option 1: Rust-based (recommended)
cargo install basic-http-server

# Option 2: Node.js-based
npm install -g http-server
```

### 2. Build the WASM Package

```bash
# Run the build script
./build-wasm.sh

# Or manually:
wasm-pack build --target web --out-dir pkg
```

This creates:
- `pkg/candle_webgpu_demo.js` - JavaScript bindings
- `pkg/candle_webgpu_demo_bg.wasm` - Compiled Rust code
- `pkg/candle_webgpu_demo.d.ts` - TypeScript definitions

### 3. Start Local Server

```bash
# If using basic-http-server:
basic-http-server .

# If using http-server:
http-server . -p 8080
```

### 4. Open in Browser

Open **http://localhost:8000** (or 8080) in a WebGPU-capable browser:
- Chrome/Edge 113+
- Firefox Nightly (enable `dom.webgpu.enabled` in `about:config`)
- Safari Technology Preview

### 5. Run the Demo

1. The page will check for WebGPU support
2. Click "Run WebGPU Demo"
3. Watch GPU operations execute in real-time!

## Expected Output

```
✅ WASM module loaded
🚀 Starting demo...
🚀 Starting WebGPU demo...
✅ WebGPU device created!
--- Running Matrix Multiplication ---
Input A: [[1.0, 2.0], [3.0, 4.0]]
Input B: [[5.0, 6.0], [7.0, 8.0]]
Result C = A × B: [[19.0, 22.0], [43.0, 50.0]]
--- Testing Activations ---
ReLU: [0.0, 0.0, 0.0, 1.0, 2.0]
GELU: [-0.045, -0.159, 0.0, 0.841, 1.955]
--- Testing Element-wise Operations ---
Add: [5.0, 5.0, 5.0, 5.0]
Mul: [4.0, 6.0, 6.0, 4.0]
✨ All operations completed successfully on GPU!
✨ Demo completed successfully!
```

## Browser Compatibility

| Browser | Version | Status |
|---------|---------|--------|
| Chrome | 113+ | ✅ Supported |
| Edge | 113+ | ✅ Supported |
| Firefox | Nightly | ⚠️ Requires flag |
| Safari | Tech Preview | ⚠️ Experimental |

## Troubleshooting

### "WebGPU not supported"
- Update your browser to the latest version
- Check `chrome://gpu/` to verify WebGPU status
- Try Chrome 113+ or Edge 113+

### "Failed to load WASM"
- Make sure you're using an HTTP server (not `file://`)
- Check browser console for specific errors
- Verify `pkg/` directory was created during build

### CORS errors
- Always use a proper HTTP server
- Don't open `index.html` directly with `file://`

### Build fails
- Make sure you're in the `candle-webgpu-demo` directory
- Run `cargo check` to verify dependencies
- Check that `wasm-pack` is installed: `wasm-pack --version`

## How It Works

1. **Rust Code**: Written using Candle's tensor API
2. **wasm-pack**: Compiles Rust to WebAssembly
3. **wasm-bindgen**: Creates JavaScript bindings
4. **WebGPU**: Executes operations on your GPU
5. **Browser**: Runs everything locally!

The entire ML computation pipeline runs in your browser with no server required.

## Technical Details

- **Framework**: Candle (Rust ML framework)
- **Backend**: WebGPU (cross-platform GPU API)
- **Target**: `wasm32-unknown-unknown`
- **Build Tool**: wasm-pack
- **GPU Shaders**: WGSL (WebGPU Shading Language)

### WASM Configuration

The project includes special configuration for WebAssembly builds:

**`.cargo/config.toml`** - Configures the getrandom backend for WASM:
```toml
[target.wasm32-unknown-unknown]
rustflags = ["--cfg", "getrandom_backend=\"wasm_js\""]
```

**`Cargo.toml`** - Includes getrandom with wasm_js feature:
```toml
getrandom = { version = "0.3", features = ["wasm_js"] }
```

This configuration is required for random number generation in WASM environments.

## Next Steps

Want to extend this demo?
- Add more operations (softmax, layer norm)
- Load real ML models
- Add visualization with Canvas API
- Run inference on user-uploaded images
- Use Web Workers for background processing

## License

Same as the parent project.
