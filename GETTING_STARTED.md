# Getting Started

Get the WebGPU browser demo running in 3 steps.

## Prerequisites

- **macOS**, **Linux**, or **Windows** (via WSL)
- **Chrome 113+** or **Edge 113+** (for WebGPU support)

### Windows Users

**Use WSL2 (Windows Subsystem for Linux)** for best experience.

**New to WSL?** See the complete guide: **[WSL_SETUP.md](WSL_SETUP.md)**

Quick WSL setup:
1. PowerShell (admin): `wsl --install`
2. Reboot and complete Ubuntu setup
3. Run all commands in WSL terminal
4. Access at `http://localhost:4000` from Windows browser

## Step 1: Setup (One-Time)

Run the setup script to install all dependencies:

```bash
./setup.sh
```

This installs:
- Rust and Cargo
- wasm32-unknown-unknown target
- wasm-pack
- basic-http-server

Takes ~5 minutes on first run.

## Step 2: Build

```bash
cd candle-webgpu-demo
./build-wasm.sh
```

Takes ~30 seconds after dependencies are cached.

## Step 3: Run

```bash
basic-http-server .
```

Then open: **http://localhost:4000**

## Testing the Demo

1. Click **"Run WebGPU Demo"**
2. See GPU operations execute in real-time
3. Check browser console for detailed logs

## What You Should See

```
✅ WebGPU is supported!
✅ WASM module loaded
🚀 Starting demo...
✅ WebGPU device created!
--- Running Matrix Multiplication ---
✓ Matrix multiplication completed on GPU
--- Testing Activation Functions ---
✓ ReLU activation completed on GPU
✓ GELU activation completed on GPU
✨ All GPU operations completed successfully!
```

## Troubleshooting

### "WebGPU not supported"

- Use Chrome/Edge 113 or later
- Check WebGPU status: `chrome://gpu/`
- Look for "WebGPU: Hardware accelerated"

### "Failed to load WASM"

- Hard refresh: **Cmd+Shift+R** (Mac) or **Ctrl+Shift+R** (Windows/Linux)
- Clear browser cache
- Try Incognito mode

### Build fails

```bash
# Clean rebuild
cd candle-webgpu-demo
rm -rf pkg target
./build-wasm.sh
```

### Server won't start

```bash
# Check if port 4000 is in use
lsof -i :4000

# Use a different port
basic-http-server . -a 127.0.0.1:8080
```

### WSL-Specific Issues

**For complete WSL troubleshooting, see [WSL_SETUP.md](WSL_SETUP.md)**

**localhost doesn't work from Windows browser:**

```bash
# Get WSL IP address
ip addr show eth0 | grep 'inet ' | awk '{print $2}' | cut -d/ -f1

# Start server on all interfaces
basic-http-server . -a 0.0.0.0:4000

# Access from Windows: http://<WSL-IP>:4000
```

**Slow file access in WSL:**
- Store project in WSL filesystem, not `/mnt/c/`
- Use: `/home/username/rust-wasm-llm`
- Not: `/mnt/c/Users/username/rust-wasm-llm`

**WSL1 vs WSL2:**
- WSL2 recommended (better performance, localhost works)
- Check version: `wsl -l -v` in PowerShell
- Upgrade to WSL2: `wsl --set-version Ubuntu 2`

## Next Steps

Once the demo works:

- Read [BROWSER_STATUS.md](BROWSER_STATUS.md) for technical details
- See [candle-webgpu-demo/README.md](candle-webgpu-demo/README.md) for demo documentation
- Review [WEBGPU_FINAL_SUMMARY.md](WEBGPU_FINAL_SUMMARY.md) for implementation details

## Development Workflow

After initial setup, the workflow is:

```bash
# Make changes to src/lib.rs

# Rebuild WASM
./build-wasm.sh

# Server is still running, just hard refresh browser
# Cmd+Shift+R (Mac) or Ctrl+Shift+R (Windows/Linux)
```

## Support

- Check browser console for errors
- Verify WebGPU: `chrome://gpu/`
- Test native build first: `cargo run --example webgpu_demo --features webgpu`

---

**That's it!** You should now have GPU-accelerated ML running in your browser.
