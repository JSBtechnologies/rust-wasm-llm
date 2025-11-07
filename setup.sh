#!/bin/bash
# Setup script for Rust WebGPU/WASM development
# Installs and configures all necessary tools
# Works on macOS, Linux, and Windows WSL

set -e  # Exit on error

echo "=========================================="
echo "🦀 Rust WebGPU/WASM Development Setup"
echo "=========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect WSL
IS_WSL=false
if grep -qi microsoft /proc/version 2>/dev/null; then
    IS_WSL=true
    echo -e "${BLUE}ℹ${NC} Detected Windows Subsystem for Linux (WSL)"
    echo ""
fi

# Check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Print status messages
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

# 1. Check/Install Rust
echo "Step 1: Checking Rust installation..."
if command_exists rustc; then
    RUST_VERSION=$(rustc --version)
    print_status "Rust is already installed: $RUST_VERSION"
else
    echo "Rust is not installed. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    print_status "Rust installed successfully"
fi

# Make sure cargo env is loaded
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

echo ""

# 2. Check/Install wasm32-unknown-unknown target
echo "Step 2: Checking wasm32-unknown-unknown target..."
if rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    print_status "wasm32-unknown-unknown target is already installed"
else
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
    print_status "wasm32-unknown-unknown target installed"
fi

echo ""

# 3. Check/Install wasm-pack
echo "Step 3: Checking wasm-pack..."
if command_exists wasm-pack; then
    WASM_PACK_VERSION=$(wasm-pack --version)
    print_status "wasm-pack is already installed: $WASM_PACK_VERSION"
else
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    print_status "wasm-pack installed successfully"
fi

echo ""

# 4. Check/Install basic-http-server (optional but recommended)
echo "Step 4: Checking basic-http-server..."
if command_exists basic-http-server; then
    print_status "basic-http-server is already installed"
else
    echo "Installing basic-http-server (for local testing)..."
    cargo install basic-http-server
    print_status "basic-http-server installed"
fi

echo ""

# 5. Verify installation
echo "Step 5: Verifying installation..."
echo ""
echo "Installed versions:"
echo "  Rust:        $(rustc --version)"
echo "  Cargo:       $(cargo --version)"
echo "  wasm-pack:   $(wasm-pack --version)"
if command_exists basic-http-server; then
    echo "  HTTP Server: $(basic-http-server --version 2>&1 | head -n1)"
fi

echo ""
echo "WASM target:"
rustup target list | grep wasm32-unknown-unknown

echo ""
echo "=========================================="
echo -e "${GREEN}✨ Setup complete!${NC}"
echo "=========================================="
echo ""
echo "Next steps:"
echo "  1. Build the WASM demo:"
echo "     cd candle-webgpu-demo"
echo "     ./build-wasm.sh"
echo ""
echo "  2. Start local server:"
echo "     basic-http-server ."
echo ""

if [ "$IS_WSL" = true ]; then
    echo "  3. Open in Windows browser:"
    echo "     http://localhost:4000"
    echo ""
    print_info "WSL detected - use Chrome/Edge on Windows to access the server"
    print_info "WSL2: localhost works directly from Windows"
    print_info "WSL1: May need to use WSL IP address instead"
    echo ""
    echo "To get WSL IP address (if localhost doesn't work):"
    echo "  ip addr show eth0 | grep 'inet ' | awk '{print \$2}' | cut -d/ -f1"
    echo ""
else
    echo "  3. Open in browser:"
    echo "     http://localhost:4000"
    echo ""
fi

echo "Browser requirements:"
echo "  - Chrome/Edge 113+ (recommended)"
echo "  - Firefox Nightly (with WebGPU enabled)"
echo "  - Safari Technology Preview"
echo ""

if [ "$IS_WSL" = true ]; then
    print_info "For WSL users:"
    echo "  - Install Chrome/Edge on Windows (not in WSL)"
    echo "  - The server runs in WSL, browser runs on Windows"
    echo "  - They communicate via localhost networking"
    echo ""
fi
