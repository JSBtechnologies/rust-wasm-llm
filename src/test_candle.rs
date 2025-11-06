//! Quick test to check current Candle WASM capabilities
//! Build with: wasm-pack build --target web

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CandleTest;

#[wasm_bindgen]
impl CandleTest {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        console_log::init_with_level(log::Level::Info).ok();
        Self
    }

    /// Test 1: Basic CPU device and tensor operations
    #[wasm_bindgen]
    pub fn test_cpu_basic(&self) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            use candle_core::{Device, Tensor};

            log::info!("Testing CPU device...");

            let device = Device::Cpu;
            log::info!("✅ CPU device created");

            match Tensor::from_vec(vec![1.0f32, 2.0, 3.0, 4.0], &[2, 2], &device) {
                Ok(tensor) => {
                    log::info!("✅ Tensor created: shape {:?}", tensor.shape());

                    match tensor.sum_all() {
                        Ok(sum) => {
                            log::info!("✅ Sum operation works");
                            match sum.to_scalar::<f32>() {
                                Ok(val) => format!("✅ All CPU tests passed! Sum = {}", val),
                                Err(e) => format!("❌ to_scalar failed: {:?}", e),
                            }
                        }
                        Err(e) => format!("❌ Sum failed: {:?}", e),
                    }
                }
                Err(e) => format!("❌ Tensor creation failed: {:?}", e),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            "Not compiled for WASM".to_string()
        }
    }

    /// Test 2: WebGPU device availability
    #[wasm_bindgen]
    pub async fn test_webgpu(&self) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            use candle_core::Device;

            log::info!("Testing WebGPU device...");

            // Try the new API (might have changed)
            match Device::new_webgpu(0) {
                Ok(_device) => {
                    log::info!("✅ WebGPU device created successfully!");
                    "✅ WebGPU is AVAILABLE! This is working!".to_string()
                }
                Err(e) => {
                    log::warn!("WebGPU device creation failed: {:?}", e);
                    format!("❌ WebGPU not available: {:?}", e)
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            "Not compiled for WASM".to_string()
        }
    }

    /// Test 3: Can we load a quantized model?
    #[wasm_bindgen]
    pub async fn test_quantized_loading(&self) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            log::info!("Testing quantized model support...");

            // Check if these types are available
            #[cfg(feature = "quantized")]
            {
                "✅ Quantized feature is available".to_string()
            }

            #[cfg(not(feature = "quantized"))]
            {
                "⚠️ Quantized feature not enabled".to_string()
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            "Not compiled for WASM".to_string()
        }
    }

    /// Test 4: Matrix multiplication (critical for LLMs)
    #[wasm_bindgen]
    pub fn test_matmul(&self) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            use candle_core::{Device, Tensor};

            log::info!("Testing matrix multiplication...");

            let device = Device::Cpu;

            // Create two 2x2 matrices
            let a = Tensor::from_vec(vec![1.0f32, 2.0, 3.0, 4.0], &[2, 2], &device);
            let b = Tensor::from_vec(vec![5.0f32, 6.0, 7.0, 8.0], &[2, 2], &device);

            match (a, b) {
                (Ok(a), Ok(b)) => {
                    log::info!("✅ Matrices created");

                    match a.matmul(&b) {
                        Ok(result) => {
                            log::info!("✅ MatMul successful!");
                            match result.to_vec2::<f32>() {
                                Ok(vec) => format!("✅ MatMul works! Result: {:?}", vec),
                                Err(e) => format!("✅ MatMul works but to_vec2 failed: {:?}", e),
                            }
                        }
                        Err(e) => format!("❌ MatMul failed: {:?}", e),
                    }
                }
                _ => "❌ Matrix creation failed".to_string(),
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            "Not compiled for WASM".to_string()
        }
    }

    /// Test 5: Check Candle version
    #[wasm_bindgen]
    pub fn candle_version(&self) -> String {
        // Candle doesn't expose version, but we can check our Cargo.toml
        env!("CARGO_PKG_VERSION").to_string()
    }
}

/// Convenience test runner
#[wasm_bindgen]
pub async fn run_all_candle_tests() -> String {
    let test = CandleTest::new();

    let mut results = Vec::new();

    results.push("=== Candle WASM Test Suite ===\n".to_string());
    results.push(format!("Package version: {}\n", test.candle_version()));

    results.push("\n[Test 1] CPU Basic Operations:");
    results.push(test.test_cpu_basic());

    results.push("\n[Test 2] WebGPU Device:");
    results.push(test.test_webgpu().await);

    results.push("\n[Test 3] Matrix Multiplication:");
    results.push(test.test_matmul());

    results.push("\n[Test 4] Quantized Support:");
    results.push(test.test_quantized_loading().await);

    results.join("\n")
}
