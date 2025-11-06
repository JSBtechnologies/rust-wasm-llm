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
