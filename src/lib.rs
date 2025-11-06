// Main library entry point for WASM LLM
// Headless WASM module - UI is separate (Vue/Quasar)
#![allow(unused_imports)]
#![allow(dead_code)]

use wasm_bindgen::prelude::*;

// Module declarations
pub mod llm;
pub mod rag;
pub mod storage;
pub mod utils;

// Candle WASM capability testing
// getrandom 0.3 works now, but Candle 0.9.1 doesn't have WebGPU support yet
// #[cfg(target_arch = "wasm32")]
// pub mod test_candle;

// Re-exports for easy access
pub use llm::{ModelConfig, PhiModel, GenerationConfig};
pub use rag::{RagPipeline, Document, Chunk};
pub use storage::{IndexedDbStorage, MemoryCache};

/// Initialize the WASM module
/// This sets up panic hooks and logging for better debugging
#[wasm_bindgen(start)]
pub fn main() {
    // Set up console error panic hook for better error messages in browser
    console_error_panic_hook::set_once();

    // Initialize logging
    console_log::init_with_level(log::Level::Debug)
        .expect("Failed to initialize logging");

    log::info!("WASM LLM Core initialized");
}

/// Get the version of the WASM module
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ============================================================================
// LLM WASM Bindings
// ============================================================================

/// WASM wrapper for PhiModel
#[wasm_bindgen]
pub struct WasmPhiModel {
    inner: PhiModel,
}

#[wasm_bindgen]
impl WasmPhiModel {
    /// Create a new Phi model with default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let config = ModelConfig::default();
        Self {
            inner: PhiModel::new(config),
        }
    }

    /// Create a new Phi model with custom configuration
    #[wasm_bindgen]
    pub fn with_config(model_url: String, tokenizer_url: String) -> Self {
        let config = ModelConfig::new(model_url, tokenizer_url);
        Self {
            inner: PhiModel::new(config),
        }
    }

    /// Load the model from configured URLs
    #[wasm_bindgen]
    pub async fn load(&mut self) -> Result<(), JsValue> {
        self.inner
            .load()
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to load model: {}", e)))
    }

    /// Generate text from a prompt
    #[wasm_bindgen]
    pub async fn generate(&self, prompt: String, config: JsValue) -> Result<String, JsValue> {
        // Parse generation config from JavaScript
        let gen_config: GenerationConfig = if config.is_undefined() || config.is_null() {
            GenerationConfig::default()
        } else {
            serde_wasm_bindgen::from_value(config)
                .map_err(|e| JsValue::from_str(&format!("Invalid config: {}", e)))?
        };

        self.inner
            .generate(&prompt, &gen_config)
            .await
            .map_err(|e| JsValue::from_str(&format!("Generation failed: {}", e)))
    }

    /// Generate text with streaming (calls callback for each token)
    #[wasm_bindgen]
    pub async fn generate_stream(
        &self,
        prompt: String,
        callback: js_sys::Function,
        config: JsValue,
    ) -> Result<(), JsValue> {
        // Parse generation config
        let gen_config: GenerationConfig = if config.is_undefined() || config.is_null() {
            GenerationConfig::default()
        } else {
            serde_wasm_bindgen::from_value(config)
                .map_err(|e| JsValue::from_str(&format!("Invalid config: {}", e)))?
        };

        // Create Rust closure that calls the JavaScript callback
        let js_callback = move |token: String| -> anyhow::Result<()> {
            let this = JsValue::null();
            let token_js = JsValue::from_str(&token);

            callback
                .call1(&this, &token_js)
                .map_err(|e| anyhow::anyhow!("Callback error: {:?}", e))?;

            Ok(())
        };

        self.inner
            .generate_stream(&prompt, &gen_config, js_callback)
            .await
            .map_err(|e| JsValue::from_str(&format!("Streaming generation failed: {}", e)))
    }

    /// Check if the model is loaded
    #[wasm_bindgen]
    pub fn is_loaded(&self) -> bool {
        self.inner.is_loaded()
    }

    /// Get model configuration as JSON
    #[wasm_bindgen]
    pub fn get_config(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(self.inner.config())
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize config: {}", e)))
    }
}

/// Create generation configuration
#[wasm_bindgen]
pub fn create_generation_config(
    max_tokens: Option<usize>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    top_k: Option<usize>,
    repetition_penalty: Option<f64>,
) -> JsValue {
    let mut config = GenerationConfig::default();

    if let Some(mt) = max_tokens {
        config.max_tokens = mt;
    }
    if let Some(temp) = temperature {
        config.temperature = temp;
    }
    if let Some(p) = top_p {
        config.top_p = p;
    }
    if let Some(k) = top_k {
        config.top_k = k;
    }
    if let Some(rp) = repetition_penalty {
        config.repetition_penalty = rp;
    }

    serde_wasm_bindgen::to_value(&config).unwrap_or(JsValue::NULL)
}
