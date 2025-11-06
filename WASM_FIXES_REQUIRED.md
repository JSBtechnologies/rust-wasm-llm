# Required Candle-Local Fixes for WASM Support

The browser demo requires the following changes to `candle-local/candle-core` to work properly. These files are in the ignored `candle-local/` directory, so you'll need to apply these changes manually.

**3 fixes required:**
1. Add async device creation API
2. Add conditional compilation for WASM/native
3. Use browser-compatible GPU limits

## Files to Modify

### 1. `candle-local/candle-core/src/device.rs`

**Location:** Line 270-273

**Change:**
```rust
// BEFORE:
#[cfg(feature = "webgpu")]
pub fn new_webgpu(ordinal: usize) -> Result<Self> {
    Ok(Self::WebGpu(crate::WebGpuDevice::new(ordinal)?))
}

// AFTER:
#[cfg(all(feature = "webgpu", not(target_arch = "wasm32")))]
pub fn new_webgpu(ordinal: usize) -> Result<Self> {
    Ok(Self::WebGpu(crate::WebGpuDevice::new(ordinal)?))
}

#[cfg(feature = "webgpu")]
pub async fn new_webgpu_async(ordinal: usize) -> Result<Self> {
    Ok(Self::WebGpu(crate::WebGpuDevice::new_async(ordinal).await?))
}
```

**Why:** Adds an async version of device creation that works in WASM environments where blocking is not allowed.

---

### 2. `candle-local/candle-core/src/webgpu_backend/device.rs`

**Location 1:** Line 93-96

**Change:**
```rust
// BEFORE:
/// Create a new WebGPU device (blocking version using pollster)
pub fn new_blocking(ordinal: usize) -> Result<Self> {
    pollster::block_on(Self::new_async(ordinal))
}

// AFTER:
/// Create a new WebGPU device (blocking version using pollster)
#[cfg(not(target_arch = "wasm32"))]
pub fn new(ordinal: usize) -> Result<Self> {
    pollster::block_on(Self::new_async(ordinal))
}

/// Create a new WebGPU device (blocking version using pollster)
#[cfg(not(target_arch = "wasm32"))]
pub fn new_blocking(ordinal: usize) -> Result<Self> {
    Self::new(ordinal)
}
```

**Why:** Adds a `new()` method that works on native platforms (non-WASM) using blocking operations.

---

**Location 2:** Line 175-179 (in `impl BackendDevice for WebGpuDevice`)

**Change:**
```rust
// BEFORE:
fn new(ordinal: usize) -> Result<Self> {
    Self::new_blocking(ordinal)
}

// AFTER:
#[cfg(not(target_arch = "wasm32"))]
fn new(ordinal: usize) -> Result<Self> {
    WebGpuDevice::new(ordinal)
}

#[cfg(target_arch = "wasm32")]
fn new(_ordinal: usize) -> Result<Self> {
    crate::bail!("WebGPU device creation in WASM must use Device::new_webgpu_async(). Synchronous creation is not supported in browsers.")
}
```

**Why:** The trait implementation needs to call the static method, not itself (to avoid infinite recursion). On WASM, it provides a helpful error message.

---

### 3. `candle-local/candle-core/src/webgpu_backend/device.rs` (Browser Limits Fix)

**Location 3:** Line 69-77 (device request)

**Change:**
```rust
// BEFORE:
// Request device and queue
let (device, queue) = adapter
    .request_device(
        &DeviceDescriptor {
            label: Some(&format!("Candle WebGPU Device {}", ordinal)),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
        },
        None,
    )
    .await
    .map_err(|e| WebGpuError::DeviceRequestFailed(e.to_string()))?;

// AFTER:
// Request device and queue
// For WASM, use downlevel defaults to be compatible with all browsers
#[cfg(target_arch = "wasm32")]
let limits = wgpu::Limits::downlevel_webgl2_defaults();

#[cfg(not(target_arch = "wasm32"))]
let limits = wgpu::Limits::default();

let (device, queue) = adapter
    .request_device(
        &DeviceDescriptor {
            label: Some(&format!("Candle WebGPU Device {}", ordinal)),
            required_features: wgpu::Features::empty(),
            required_limits: limits,
            memory_hints: wgpu::MemoryHints::default(),
        },
        None,
    )
    .await
    .map_err(|e| WebGpuError::DeviceRequestFailed(e.to_string()))?;
```

**Why:** Browser WebGPU implementations don't support all the limits that `wgpu::Limits::default()` requests (like `maxInterStageShaderComponents`). Using `downlevel_webgl2_defaults()` provides conservative limits that work across all browsers.

---

## How to Apply

1. **Edit the files:**
   ```bash
   cd candle-local/candle-core
   # Edit src/device.rs and src/webgpu_backend/device.rs
   ```

2. **Test the build:**
   ```bash
   # Test native build
   cargo build --features webgpu

   # Test WASM build
   cargo build --target wasm32-unknown-unknown --features webgpu
   ```

3. **Verify WASM demo compiles:**
   ```bash
   cd ../../candle-webgpu-demo
   cargo build --target wasm32-unknown-unknown --lib
   ```

## Summary of Changes

These changes add **conditional compilation** to:
- ✅ Provide blocking API on native platforms (uses `pollster::block_on`)
- ✅ Provide async-only API on WASM (avoids blocking primitives)
- ✅ Allow browser demo to use `Device::new_webgpu_async()`
- ✅ Prevent accidental use of blocking calls in WASM
- ✅ Use browser-compatible GPU limits (fixes `maxInterStageShaderComponents` error)

## Why Not Committed

The `candle-local/` directory is git-ignored because it's meant for local development and experimentation. Once these changes are tested and finalized, they could be contributed back to the upstream Candle repository.

## Testing

After applying these fixes, the browser demo should:
1. Successfully compile for `wasm32-unknown-unknown`
2. Create WebGPU device asynchronously without blocking
3. Run GPU operations without "condvar wait not supported" errors
4. Display operation completion messages in the browser console

The demo validates operations complete but doesn't read results back (async buffer mapping will be added in a future update).
