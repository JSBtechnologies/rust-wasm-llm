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

    // Create WebGPU device asynchronously (WASM requires async)
    let device = Device::new_webgpu_async(0).await
        .map_err(|e| JsValue::from_str(&format!("Failed to create device: {}", e)))?;

    console_log!("✅ WebGPU device created!");

    // Run some basic operations
    run_demo(&device)
        .map_err(|e| JsValue::from_str(&format!("Demo failed: {}", e)))?;

    Ok(JsValue::from_str("Demo completed successfully!"))
}

fn run_demo(device: &Device) -> CandleResult<()> {
    console_log!("--- Running Matrix Multiplication ---");

    // Create test matrices on GPU
    let a = Tensor::from_slice(&[1.0f32, 2.0, 3.0, 4.0], (2, 2), device)?;
    let b = Tensor::from_slice(&[5.0f32, 6.0, 7.0, 8.0], (2, 2), device)?;

    console_log!("Created 2×2 input matrices on GPU");

    // Matrix multiplication on GPU!
    let c = a.matmul(&b)?;
    console_log!("✓ Matrix multiplication completed on GPU");
    console_log!("  Result shape: {:?}", c.dims());

    // Note: Reading back from GPU requires async operations in WASM
    // For this demo, we verify operations complete without errors

    // Test activations
    console_log!("\n--- Testing Activation Functions ---");
    let data = Tensor::from_slice(&[-2.0f32, -1.0, 0.0, 1.0, 2.0], 5, device)?;

    let relu_result = data.relu()?;
    console_log!("✓ ReLU activation completed on GPU");
    console_log!("  Input: 5 elements, Output shape: {:?}", relu_result.dims());

    let gelu_result = data.gelu()?;
    console_log!("✓ GELU activation completed on GPU");
    console_log!("  Output shape: {:?}", gelu_result.dims());

    // Test element-wise operations
    console_log!("\n--- Testing Element-wise Operations ---");
    let x = Tensor::from_slice(&[1.0f32, 2.0, 3.0, 4.0], 4, device)?;
    let y = Tensor::from_slice(&[4.0f32, 3.0, 2.0, 1.0], 4, device)?;

    let sum = (&x + &y)?;
    console_log!("✓ Addition completed on GPU");
    console_log!("  Result shape: {:?}", sum.dims());

    let prod = (&x * &y)?;
    console_log!("✓ Multiplication completed on GPU");
    console_log!("  Result shape: {:?}", prod.dims());

    // Test chained operations
    console_log!("\n--- Testing Chained Operations ---");
    let result = a.matmul(&b)?.relu()?;
    console_log!("✓ Chained matmul → relu completed on GPU");
    console_log!("  Final shape: {:?}", result.dims());

    console_log!("\n✨ All GPU operations completed successfully!");
    console_log!("\nOperations tested:");
    console_log!("  • Matrix multiplication (16×16 workgroups)");
    console_log!("  • Activation functions (ReLU, GELU)");
    console_log!("  • Element-wise ops (add, multiply)");
    console_log!("  • Chained operations");
    console_log!("\n💡 All computations ran on your GPU via WebGPU!");

    console_log!("\nNote: This demo verifies operations complete without errors.");
    console_log!("Full GPU ↔ CPU data transfer requires async buffer mapping,");
    console_log!("which will be added in a future update.");

    Ok(())
}
