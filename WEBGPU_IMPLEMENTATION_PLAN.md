# Adding WebGPU Support to Candle

## Executive Summary

This document outlines a comprehensive plan to add WebGPU support to the Candle ML framework. WebGPU will enable high-performance GPU-accelerated inference in web browsers, significantly improving performance over CPU-only WASM execution.

## Current State

### Candle's Backend Architecture

Candle currently supports three backends:
- **CPU**: Optimized with optional MKL/Accelerate support
- **CUDA**: For NVIDIA GPUs with cuDNN optimizations
- **Metal**: For Apple Silicon GPUs

### Key Architecture Components

1. **Backend Trait System** (`backend.rs`)
   - `BackendDevice` trait defines the interface all backends must implement
   - `BackendStorage` trait handles tensor operations

2. **Device Enum** (`device.rs`)
   ```rust
   pub enum Device {
       Cpu,
       Cuda(crate::CudaDevice),
       Metal(crate::MetalDevice),
   }
   ```

3. **Backend Organization**
   - Separate directories: `cpu/`, `cuda_backend/`, `metal_backend/`
   - Dummy implementations when backends aren't available
   - Each backend implements the core trait interface

## WebGPU Integration Plan

### Phase 1: Foundation Setup

#### 1.1 Add wgpu Dependency
Add to `candle-core/Cargo.toml`:
```toml
[dependencies]
wgpu = { version = "0.19", optional = true }
pollster = { version = "0.3", optional = true }  # For async initialization

[features]
webgpu = ["wgpu", "pollster"]
```

#### 1.2 Create WebGPU Backend Directory
Create `candle-core/src/webgpu_backend/` with:
- `mod.rs` - Module definition and exports
- `device.rs` - WebGpuDevice struct implementation
- `storage.rs` - WebGpuStorage for tensor data
- `kernels.wgsl` - WGSL compute shaders
- `ops.rs` - Operation implementations

#### 1.3 Update Device Enum
In `candle-core/src/device.rs`:
```rust
pub enum Device {
    Cpu,
    Cuda(crate::CudaDevice),
    Metal(crate::MetalDevice),
    #[cfg(feature = "webgpu")]
    WebGpu(crate::WebGpuDevice),
}
```

### Phase 2: Core Backend Implementation

#### 2.1 WebGpuDevice Structure
```rust
pub struct WebGpuDevice {
    id: usize,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    adapter: Arc<wgpu::Adapter>,
}
```

#### 2.2 Implement BackendDevice Trait
Required methods:
- `new()` - Initialize WebGPU device and queue
- `location()` - Return device type
- `same_device()` - Compare devices
- `zeros_impl()` - Allocate zero-initialized buffers
- `alloc_uninit()` - Allocate uninitialized GPU buffers
- `storage_from_slice()` - Upload data to GPU
- `rand_uniform()` / `rand_normal()` - Random number generation
- `synchronize()` - Wait for GPU operations to complete
- `set_seed()` - Set RNG seed

#### 2.3 WebGpuStorage Implementation
```rust
pub struct WebGpuStorage {
    buffer: wgpu::Buffer,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    dtype: DType,
    elem_count: usize,
}
```

### Phase 3: Compute Shader Operations

#### 3.1 Priority Operations (WGSL Shaders)
Must implement in order of importance:

1. **Matrix Multiplication** (matmul)
   - Tile-based optimization for large matrices
   - Use workgroup shared memory
   - FMA instructions for efficiency

2. **Element-wise Operations**
   - Unary: ReLU, GELU, tanh, sigmoid, exp, log, sqrt
   - Binary: add, sub, mul, div, pow
   - Activation functions

3. **Reduction Operations**
   - Sum, mean, max, min across dimensions
   - Two-pass reduction for large tensors

4. **Convolution Operations**
   - 1D/2D convolutions
   - Transposed convolutions
   - Use compute shader workgroups efficiently

5. **Data Movement**
   - Transpose, reshape, concatenate
   - Gather, scatter operations
   - Index selection

#### 3.2 Shader Organization
Create modular WGSL files:
- `matmul.wgsl` - Matrix multiplication kernels
- `elementwise.wgsl` - Element-wise operations
- `reductions.wgsl` - Reduction operations
- `conv.wgsl` - Convolution operations
- `utils.wgsl` - Helper functions

### Phase 4: WASM Integration

#### 4.1 WebGPU Feature Detection
```rust
#[cfg(target_arch = "wasm32")]
async fn create_webgpu_device() -> Result<WebGpuDevice> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .ok_or(Error::WebGpuNotAvailable)?;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await?;

    Ok(WebGpuDevice { device, queue, adapter, id: 0 })
}
```

#### 4.2 Fallback Strategy
- Check WebGPU availability at runtime
- Fall back to CPU if WebGPU unavailable
- Provide clear error messages

### Phase 5: Optimization

#### 5.1 Performance Optimizations
- **Shader Compilation Caching**: Cache compiled pipelines
- **Buffer Pooling**: Reuse GPU buffers to reduce allocation overhead
- **Async Operations**: Pipeline GPU operations to maximize throughput
- **Workgroup Size Tuning**: Optimize workgroup sizes per operation

#### 5.2 Memory Management
- Implement proper buffer lifecycle management
- Support buffer sharing between operations
- Minimize CPU-GPU data transfers

### Phase 6: Testing & Validation

#### 6.1 Unit Tests
- Test each operation against CPU backend results
- Verify numerical accuracy (within floating-point tolerance)
- Test edge cases (empty tensors, large tensors, etc.)

#### 6.2 Integration Tests
- Run transformer model inference
- Compare output with CPU/CUDA backends
- Benchmark performance improvements

#### 6.3 WASM Browser Testing
- Test on Chrome, Firefox, Safari (WebGPU support varies)
- Verify fallback mechanisms work
- Profile memory usage

## Implementation Roadmap

### Milestone 1: Foundation (Week 1-2)
- [ ] Add wgpu dependency with feature flag
- [ ] Create webgpu_backend directory structure
- [ ] Implement WebGpuDevice and WebGpuStorage structs
- [ ] Update Device enum with WebGpu variant

### Milestone 2: Basic Operations (Week 3-4)
- [ ] Implement BackendDevice trait methods
- [ ] Create basic WGSL shaders (matmul, elementwise ops)
- [ ] Add buffer management and data transfer
- [ ] Write unit tests for basic operations

### Milestone 3: Advanced Operations (Week 5-6)
- [ ] Implement reduction operations
- [ ] Add convolution support
- [ ] Implement attention mechanisms
- [ ] Add all necessary transformer operations

### Milestone 4: WASM Integration (Week 7-8)
- [ ] Add WASM-specific initialization code
- [ ] Implement feature detection and fallback
- [ ] Create browser test harness
- [ ] Optimize for WASM binary size

### Milestone 5: Optimization & Testing (Week 9-10)
- [ ] Profile and optimize critical operations
- [ ] Add buffer pooling and caching
- [ ] Comprehensive testing across browsers
- [ ] Documentation and examples

## Technical Challenges & Solutions

### Challenge 1: Async Initialization
**Problem**: WebGPU requires async initialization in WASM
**Solution**: Use async/await with wasm-bindgen-futures, provide blocking wrapper for non-async contexts

### Challenge 2: Shader Compilation
**Problem**: WGSL shaders must be compiled at runtime
**Solution**: Lazy initialization of pipelines, cache compiled shaders, provide precompilation option

### Challenge 3: Buffer Size Limits
**Problem**: WebGPU has maximum buffer size limits
**Solution**: Implement buffer splitting for large tensors, chunk operations as needed

### Challenge 4: Browser Compatibility
**Problem**: WebGPU support varies across browsers
**Solution**: Feature detection, clear error messages, fallback to CPU, document browser requirements

### Challenge 5: Debugging
**Problem**: GPU debugging is harder than CPU
**Solution**: Add verbose logging, validation layers, CPU-side verification mode, shader debugging tools

## Example Usage

```rust
use candle_core::{Device, Tensor};

// Create WebGPU device
let device = Device::new_webgpu(0)?;

// Create tensors on GPU
let a = Tensor::randn(0f32, 1.0, (1024, 1024), &device)?;
let b = Tensor::randn(0f32, 1.0, (1024, 1024), &device)?;

// Perform operations on GPU
let c = a.matmul(&b)?;

// Synchronize and get results
device.synchronize()?;
let result = c.to_vec1::<f32>()?;
```

## Benefits

1. **Performance**: 10-100x speedup over CPU-only WASM
2. **Browser Native**: No plugins or external dependencies
3. **Cross-Platform**: Works on any browser with WebGPU support
4. **Memory Efficient**: Operations stay on GPU, minimal data transfer
5. **Future-Proof**: WebGPU is the standard for web GPU computing

## Resources

- **wgpu Documentation**: https://docs.rs/wgpu/
- **WebGPU Fundamentals**: https://webgpufundamentals.org/
- **WGSL Spec**: https://www.w3.org/TR/WGSL/
- **Candle Repository**: https://github.com/huggingface/candle
- **WebGPU Matrix Optimization**: https://www.nuss-and-bolts.com/p/optimizing-a-webgpu-matmul-kernel

## Next Steps

1. Fork the Candle repository
2. Create a feature branch: `feat/webgpu-backend`
3. Start with Milestone 1 implementation
4. Submit incremental PRs to upstream
5. Engage with Candle maintainers for feedback

---

**Note**: This is a significant undertaking requiring expertise in GPU programming, WGSL, Rust, and the Candle internals. Estimated effort: 2-3 months for a single experienced developer, or 4-6 weeks with a small team.
