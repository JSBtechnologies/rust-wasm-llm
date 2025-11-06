# Testing WebGPU Backend in Browser

## Prerequisites

1. **Browser with WebGPU support:**
   - Chrome/Edge 113+ (enabled by default)
   - Firefox Nightly (enable in about:config)
   - Safari Technology Preview

2. **Install wasm-pack:**
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

3. **Install a local server:**
   ```bash
   cargo install basic-http-server
   # or
   npm install -g http-server
   ```

## Step 1: Create WASM Package

First, let's create a WASM-compatible project structure:

```bash
cd /Users/jeffriebudde/rust-wasm-llm
mkdir candle-webgpu-demo
cd candle-webgpu-demo
```

### Create Cargo.toml:
```toml
[package]
name = "candle-webgpu-demo"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
candle-core = { path = "../candle-local/candle-core", features = ["webgpu"] }
wasm-bindgen = "0.2"
console_error_panic_hook = "0.1"
wasm-bindgen-futures = "0.4"

[dependencies.web-sys]
version = "0.3"
features = [
    "console",
    "Window",
    "Document",
    "HtmlElement",
]
```

### Create src/lib.rs:
```rust
use wasm_bindgen::prelude::*;
use candle_core::{Device, Tensor, Result as CandleResult};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub async fn run_webgpu_demo() -> Result<JsValue, JsValue> {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    console_log!("🚀 Starting WebGPU demo...");

    // Create WebGPU device
    let device = Device::new_webgpu(0)
        .map_err(|e| JsValue::from_str(&format!("Failed to create device: {}", e)))?;

    console_log!("✅ WebGPU device created!");

    // Run some basic operations
    run_demo(&device)
        .map_err(|e| JsValue::from_str(&format!("Demo failed: {}", e)))?;

    Ok(JsValue::from_str("Demo completed successfully!"))
}

fn run_demo(device: &Device) -> CandleResult<()> {
    console_log!("--- Running Matrix Multiplication ---");

    // Create test matrices
    let a = Tensor::from_slice(&[1.0f32, 2.0, 3.0, 4.0], (2, 2), device)?;
    let b = Tensor::from_slice(&[5.0f32, 6.0, 7.0, 8.0], (2, 2), device)?;

    console_log!("Input A: {:?}", a.to_vec2::<f32>()?);
    console_log!("Input B: {:?}", b.to_vec2::<f32>()?);

    // Matrix multiplication on GPU!
    let c = a.matmul(&b)?;
    console_log!("Result C = A × B: {:?}", c.to_vec2::<f32>()?);

    // Test activations
    console_log!("--- Testing Activations ---");
    let data = Tensor::from_slice(&[-2.0f32, -1.0, 0.0, 1.0, 2.0], 5, device)?;

    let relu_result = data.relu()?;
    console_log!("ReLU: {:?}", relu_result.to_vec1::<f32>()?);

    let gelu_result = data.gelu()?;
    console_log!("GELU: {:?}", gelu_result.to_vec1::<f32>()?);

    // Test element-wise operations
    console_log!("--- Testing Element-wise Operations ---");
    let x = Tensor::from_slice(&[1.0f32, 2.0, 3.0, 4.0], 4, device)?;
    let y = Tensor::from_slice(&[4.0f32, 3.0, 2.0, 1.0], 4, device)?;

    let sum = (&x + &y)?;
    console_log!("Add: {:?}", sum.to_vec1::<f32>()?);

    let prod = (&x * &y)?;
    console_log!("Mul: {:?}", prod.to_vec1::<f32>()?);

    console_log!("✨ All operations completed successfully on GPU!");

    Ok(())
}
```

## Step 2: Build for WASM

```bash
# Build the WASM package
wasm-pack build --target web --out-dir pkg

# This creates:
# - pkg/candle_webgpu_demo.js
# - pkg/candle_webgpu_demo_bg.wasm
# - pkg/candle_webgpu_demo.d.ts
```

## Step 3: Create HTML Page

Create `index.html`:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Candle WebGPU Demo</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background: #f5f5f5;
        }
        .container {
            background: white;
            padding: 30px;
            border-radius: 10px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        h1 {
            color: #333;
            border-bottom: 3px solid #4CAF50;
            padding-bottom: 10px;
        }
        button {
            background: #4CAF50;
            color: white;
            border: none;
            padding: 12px 24px;
            font-size: 16px;
            border-radius: 5px;
            cursor: pointer;
            margin: 10px 5px;
        }
        button:hover {
            background: #45a049;
        }
        button:disabled {
            background: #cccccc;
            cursor: not-allowed;
        }
        #output {
            background: #1e1e1e;
            color: #d4d4d4;
            padding: 15px;
            border-radius: 5px;
            font-family: 'Courier New', monospace;
            font-size: 14px;
            max-height: 400px;
            overflow-y: auto;
            margin-top: 20px;
        }
        .log-entry {
            margin: 5px 0;
            padding: 3px 0;
        }
        .success { color: #4CAF50; }
        .error { color: #f44336; }
        .info { color: #2196F3; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 Candle WebGPU Demo</h1>

        <p>This demo runs a machine learning framework entirely in your browser using WebGPU for GPU acceleration!</p>

        <div id="webgpu-check"></div>

        <button id="runButton" onclick="runDemo()">Run WebGPU Demo</button>
        <button onclick="clearOutput()">Clear Output</button>

        <div id="output"></div>
    </div>

    <script type="module">
        import init, { run_webgpu_demo } from './pkg/candle_webgpu_demo.js';

        let wasmInitialized = false;

        // Check WebGPU support
        async function checkWebGPU() {
            const checkDiv = document.getElementById('webgpu-check');

            if (!navigator.gpu) {
                checkDiv.innerHTML = `
                    <div style="background: #fff3cd; padding: 15px; border-radius: 5px; border-left: 4px solid #ffc107;">
                        <strong>⚠️ WebGPU not supported</strong><br>
                        Your browser doesn't support WebGPU. Please use:
                        <ul>
                            <li>Chrome/Edge 113+</li>
                            <li>Firefox Nightly (with flag enabled)</li>
                            <li>Safari Technology Preview</li>
                        </ul>
                    </div>
                `;
                document.getElementById('runButton').disabled = true;
                return false;
            }

            const adapter = await navigator.gpu.requestAdapter();
            if (!adapter) {
                checkDiv.innerHTML = `
                    <div style="background: #f8d7da; padding: 15px; border-radius: 5px; border-left: 4px solid #dc3545;">
                        <strong>❌ No WebGPU adapter found</strong><br>
                        Your device doesn't have a compatible GPU.
                    </div>
                `;
                document.getElementById('runButton').disabled = true;
                return false;
            }

            checkDiv.innerHTML = `
                <div style="background: #d4edda; padding: 15px; border-radius: 5px; border-left: 4px solid #28a745;">
                    <strong>✅ WebGPU is supported!</strong><br>
                    Adapter: ${adapter.info.description || adapter.info.vendor || 'Available'}
                </div>
            `;
            return true;
        }

        // Initialize WASM
        async function initWasm() {
            if (wasmInitialized) return true;

            try {
                await init();
                wasmInitialized = true;
                addLog('✅ WASM module loaded', 'success');
                return true;
            } catch (e) {
                addLog(`❌ Failed to load WASM: ${e}`, 'error');
                return false;
            }
        }

        // Run the demo
        window.runDemo = async function() {
            const button = document.getElementById('runButton');
            button.disabled = true;
            button.textContent = 'Running...';

            try {
                if (!await initWasm()) {
                    throw new Error('WASM initialization failed');
                }

                addLog('🚀 Starting demo...', 'info');
                const result = await run_webgpu_demo();
                addLog(`✨ ${result}`, 'success');

            } catch (e) {
                addLog(`❌ Error: ${e}`, 'error');
                console.error(e);
            } finally {
                button.disabled = false;
                button.textContent = 'Run WebGPU Demo';
            }
        };

        // Output logging
        function addLog(message, type = 'info') {
            const output = document.getElementById('output');
            const entry = document.createElement('div');
            entry.className = `log-entry ${type}`;
            entry.textContent = message;
            output.appendChild(entry);
            output.scrollTop = output.scrollHeight;
        }

        window.clearOutput = function() {
            document.getElementById('output').innerHTML = '';
        };

        // Override console.log to capture output
        const originalLog = console.log;
        console.log = function(...args) {
            originalLog.apply(console, args);
            addLog(args.join(' '), 'info');
        };

        // Check WebGPU support on load
        checkWebGPU();
    </script>
</body>
</html>
```

## Step 4: Run Local Server

```bash
# From the candle-webgpu-demo directory
basic-http-server .
# or
# http-server . -p 8080
```

Then open: **http://localhost:8000** (or 8080)

## Step 5: Test in Browser

1. Open the URL in a WebGPU-capable browser
2. You should see "✅ WebGPU is supported!"
3. Click "Run WebGPU Demo"
4. Watch the GPU operations execute!

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

## Troubleshooting

### WASM not loading
- Make sure you're using a web server (not `file://`)
- Check browser console for errors
- Verify `pkg/` directory exists with .wasm and .js files

### WebGPU not available
- Update browser to latest version
- Check chrome://gpu/ to verify WebGPU status
- Try different browser (Chrome 113+)

### CORS errors
- Use a proper HTTP server, not `file://`
- Add `--cors` flag if using http-server

## Browser Compatibility

| Browser | Version | Status |
|---------|---------|--------|
| Chrome | 113+ | ✅ Supported |
| Edge | 113+ | ✅ Supported |
| Firefox | Nightly | ⚠️ Requires flag |
| Safari | Tech Preview | ⚠️ Experimental |

## Performance Notes

- First run may be slower (shader compilation)
- Subsequent runs use cached pipelines
- GPU operations are fully async
- All computation happens on your GPU!

## Next Steps

1. **Add neural network demo** - Run inference in browser
2. **File upload** - Load custom models
3. **Visualization** - Display results with Canvas API
4. **Worker threads** - Run in background for better UX

## Resources

- [WebGPU Spec](https://www.w3.org/TR/webgpu/)
- [wasm-bindgen Book](https://rustwasm.github.io/docs/wasm-bindgen/)
- [MDN WebGPU](https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API)
