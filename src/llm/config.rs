use serde::{Deserialize, Serialize};

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Model URL (HuggingFace or custom)
    pub model_url: String,
    /// Tokenizer URL
    pub tokenizer_url: String,
    /// Model ID for identification
    pub model_id: String,
    /// Whether to use WebGPU (fallback to CPU if unavailable)
    pub use_webgpu: bool,
    /// Quantization type (Q4, Q8, etc.)
    pub quantization: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model_url: String::from(
                "https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf/resolve/main/Phi-3-mini-4k-instruct-q4.gguf"
            ),
            tokenizer_url: String::from(
                "https://huggingface.co/microsoft/Phi-3-mini-4k-instruct/resolve/main/tokenizer.json"
            ),
            model_id: String::from("Phi-3-mini-4k-instruct-q4"),
            use_webgpu: true,
            quantization: String::from("Q4"),
        }
    }
}

impl ModelConfig {
    /// Create a new model configuration
    pub fn new(model_url: String, tokenizer_url: String) -> Self {
        Self {
            model_url,
            tokenizer_url,
            ..Default::default()
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.model_url.is_empty() {
            return Err("Model URL cannot be empty".to_string());
        }
        if self.tokenizer_url.is_empty() {
            return Err("Tokenizer URL cannot be empty".to_string());
        }
        Ok(())
    }
}
