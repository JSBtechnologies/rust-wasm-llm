# Windows WSL Setup Guide

Complete guide for running the WebGPU demo on Windows using WSL2.

## Why WSL?

This project requires Linux tools (Rust, wasm-pack, etc.). WSL2 provides a full Linux environment on Windows with:
- Native Linux performance
- Easy access from Windows browsers
- Seamless file sharing

## Step 1: Install WSL2

### Check if WSL is Already Installed

Open PowerShell and run:
```powershell
wsl --version
```

If you see version info, skip to Step 2.

### Install WSL2 (New Installation)

**In PowerShell (Run as Administrator):**

```powershell
# Install WSL2 with Ubuntu
wsl --install

# Reboot when prompted
```

After reboot, Ubuntu will finish installation and ask for:
- Username (lowercase, no spaces)
- Password (for sudo commands)

### Upgrade WSL1 to WSL2 (If Needed)

Check version:
```powershell
wsl -l -v
```

If it shows VERSION 1:
```powershell
# Upgrade to WSL2
wsl --set-version Ubuntu 2

# Set WSL2 as default
wsl --set-default-version 2
```

## Step 2: Update Ubuntu

**In WSL terminal:**

```bash
# Update package list
sudo apt update

# Upgrade packages
sudo apt upgrade -y

# Install build essentials (required for Rust)
sudo apt install -y build-essential curl git
```

## Step 3: Run Setup Script

```bash
# Navigate to home directory
cd ~

# Clone the repository
git clone <repo-url> rust-wasm-llm
cd rust-wasm-llm

# Run setup (installs Rust, wasm-pack, etc.)
./setup.sh
```

The script will automatically detect WSL and provide WSL-specific instructions.

## Step 4: Build and Run

```bash
# Build the demo
cd candle-webgpu-demo
./build-wasm.sh

# Start server
basic-http-server .
```

## Step 5: Access from Windows Browser

**Open Chrome or Edge on Windows** (not in WSL) and go to:

```
http://localhost:4000
```

WSL2 automatically forwards `localhost` from WSL to Windows.

## Troubleshooting

### localhost Doesn't Work

**Solution 1: Use WSL IP Address**

In WSL terminal:
```bash
# Get WSL IP address
hostname -I | awk '{print $1}'
```

Start server on all interfaces:
```bash
basic-http-server . -a 0.0.0.0:4000
```

Open in Windows browser:
```
http://<WSL-IP>:4000
```

**Solution 2: Check Windows Firewall**

1. Open Windows Defender Firewall
2. Click "Allow an app through firewall"
3. Ensure WSL and vEthernet are allowed

### Slow Build Times

**Problem:** Project is on Windows filesystem (`/mnt/c/`)

**Solution:** Move to WSL filesystem

```bash
# BAD (slow) - on Windows drive
/mnt/c/Users/yourname/rust-wasm-llm

# GOOD (fast) - on WSL filesystem
/home/yourname/rust-wasm-llm
```

To move project:
```bash
# If already cloned to /mnt/c
cd ~
mv /mnt/c/Users/yourname/rust-wasm-llm .
```

### Can't Access WSL Files from Windows

Access WSL filesystem from Windows Explorer:

```
\\wsl$\Ubuntu\home\yourname\rust-wasm-llm
```

Or from WSL terminal:
```bash
# Open current directory in Windows Explorer
explorer.exe .
```

### Port Already in Use

Check what's using port 4000:
```bash
# In WSL
sudo lsof -i :4000

# Or use different port
basic-http-server . -a 127.0.0.1:8080
```

### Rust/Cargo Not Found After Install

Restart WSL terminal or source cargo env:
```bash
source "$HOME/.cargo/env"
```

## WSL Commands Cheat Sheet

```bash
# From Windows PowerShell:
wsl                          # Start default Linux distro
wsl -l -v                    # List installed distros and versions
wsl --shutdown               # Restart WSL
wsl --set-version Ubuntu 2   # Upgrade to WSL2

# From WSL terminal:
cd ~                         # Go to Linux home directory
cd /mnt/c/Users/yourname     # Access Windows C: drive
explorer.exe .               # Open current directory in Windows Explorer
```

## File System Tips

### Good Practices

```bash
# Store code in WSL filesystem (fast)
/home/yourname/rust-wasm-llm

# Access from Windows via:
\\wsl$\Ubuntu\home\yourname\rust-wasm-llm
```

### What to Avoid

```bash
# Don't store project on Windows filesystem (slow)
/mnt/c/Users/yourname/rust-wasm-llm

# 10-100x slower for builds due to cross-filesystem access
```

## Development Workflow

**Terminal Setup:**
- Use Windows Terminal (recommended) or Ubuntu app
- Split terminals: `Ctrl+Shift+D` (vertical) or `Ctrl+Shift+-` (horizontal)

**Typical workflow:**

```bash
# Terminal 1: Code changes
cd ~/rust-wasm-llm/candle-webgpu-demo
nano src/lib.rs  # or use VS Code

# Terminal 2: Build
./build-wasm.sh

# Terminal 3: Server (leave running)
basic-http-server .

# Windows browser: http://localhost:4000
# Hard refresh: Ctrl+Shift+R
```

## VS Code with WSL

Recommended setup for best development experience:

1. Install VS Code on Windows
2. Install "Remote - WSL" extension
3. Open WSL project:

```bash
# From WSL terminal
cd ~/rust-wasm-llm
code .
```

VS Code will:
- Run extensions in WSL (fast)
- Edit files in WSL filesystem
- Use WSL terminal
- Full Rust language support

## Performance Comparison

| Location | Build Time | File Access |
|----------|-----------|-------------|
| WSL filesystem (`/home/`) | Fast (~30s) | Fast |
| Windows filesystem (`/mnt/c/`) | Slow (~5min) | Slow |

**Always use WSL filesystem for active development!**

## Next Steps

Once setup is complete:
- Follow [GETTING_STARTED.md](GETTING_STARTED.md)
- See [README.md](README.md) for project overview
- Check [BROWSER_STATUS.md](BROWSER_STATUS.md) for current status

## Common Questions

**Q: Can I use Windows tools?**
A: Yes! Use Windows browser, VS Code, etc. Just run build commands in WSL.

**Q: Do I need Linux knowledge?**
A: Basic commands help but aren't required. The scripts handle most tasks.

**Q: WSL1 or WSL2?**
A: WSL2 strongly recommended. Better performance and localhost support.

**Q: Can I access Windows files?**
A: Yes, via `/mnt/c/`, but don't store active projects there (slow).

**Q: How do I copy files between WSL and Windows?**
A: Use `\\wsl$\Ubuntu\...` from Windows or `/mnt/c/...` from WSL.

## Resources

- [WSL Documentation](https://docs.microsoft.com/en-us/windows/wsl/)
- [WSL2 Installation Guide](https://docs.microsoft.com/en-us/windows/wsl/install)
- [VS Code WSL Extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-wsl)
- [Windows Terminal](https://aka.ms/terminal)

---

**Ready?** Run `./setup.sh` and get started!
