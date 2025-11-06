use anyhow::{Result, Context};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use js_sys::Uint8Array;

use super::{config::ModelConfig, GenerationConfig};
use super::tokenizer_wrapper::TokenizerWrapper;

// Note: Candle's WASM support is still experimental
// This is a placeholder structure until full Candle WASM support is available
/// Phi-3 model wrapper for inference
pub struct PhiModel {
    config: ModelConfig,
    tokenizer: Option<TokenizerWrapper>,
    model_loaded: bool,
    // TODO: Add actual Candle model when WASM support is complete
    // For now, we'll implement a simpler approach or use mock data
    // model: Option<Box<dyn ModelInterface>>,
    // device: Device,
}

impl PhiModel {
    /// Create a new Phi model instance
    pub fn new(config: ModelConfig) -> Self {
        Self {
            config,
            tokenizer: None,
            model_loaded: false,
        }
    }

    /// Load the model from the configured URL
    pub async fn load(&mut self) -> Result<()> {
        log::info!("Loading Phi-3 model from: {}", self.config.model_url);

        // Step 1: Load tokenizer first
        log::info!("Loading tokenizer from: {}", self.config.tokenizer_url);
        let mut tokenizer = TokenizerWrapper::new(self.config.tokenizer_url.clone());
        tokenizer.load().await
            .context("Failed to load tokenizer")?;

        self.tokenizer = Some(tokenizer);
        log::info!("Tokenizer loaded successfully");

        // Step 2: Fetch model weights
        log::info!("Fetching model weights...");
        let model_bytes = self.fetch_model_bytes(&self.config.model_url).await
            .context("Failed to fetch model bytes")?;

        log::info!("Model bytes fetched: {} bytes", model_bytes.len());

        // Step 3: Initialize device
        // Note: Full Candle WASM initialization will go here when ready
        // For now, we mark as loaded
        self.model_loaded = true;

        log::info!("✅ Model loaded successfully (placeholder mode until Candle WASM is fully supported)");
        log::warn!("⚠️  Currently using mock inference - integrate Candle when WASM support is stable");

        Ok(())
    }

    /// Fetch model bytes from URL
    async fn fetch_model_bytes(&self, url: &str) -> Result<Vec<u8>> {
        let window = web_sys::window()
            .context("No window object available")?;

        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(url, &opts)
            .map_err(|e| anyhow::anyhow!("Failed to create request: {:?}", e))?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| anyhow::anyhow!("Fetch failed: {:?}", e))?;

        let resp: Response = resp_value.dyn_into()
            .map_err(|e| anyhow::anyhow!("Response conversion failed: {:?}", e))?;

        if !resp.ok() {
            anyhow::bail!("HTTP error: {}", resp.status());
        }

        let array_buffer = JsFuture::from(resp.array_buffer()
            .map_err(|e| anyhow::anyhow!("array_buffer() failed: {:?}", e))?)
            .await
            .map_err(|e| anyhow::anyhow!("array_buffer await failed: {:?}", e))?;

        let uint8_array = Uint8Array::new(&array_buffer);
        let bytes = uint8_array.to_vec();

        Ok(bytes)
    }

    /// Generate text based on a prompt
    pub async fn generate(
        &self,
        prompt: &str,
        config: &GenerationConfig,
    ) -> Result<String> {
        if !self.is_loaded() {
            anyhow::bail!("Model not loaded. Call load() first.");
        }

        log::info!("Generating text for prompt: {} (max_tokens: {})", prompt, config.max_tokens);

        let tokenizer = self.tokenizer.as_ref()
            .context("Tokenizer not loaded")?;

        // Tokenize the prompt
        let token_ids = tokenizer.encode(prompt)?;
        log::debug!("Prompt tokenized to {} tokens", token_ids.len());

        // TODO: When Candle WASM is ready, implement actual inference here
        // For now, provide an intelligent mock response
        let response = self.mock_generate(prompt, config)?;

        log::info!("Generation complete: {} tokens", response.split_whitespace().count());

        Ok(response)
    }

    /// Generate text with streaming (call callback for each token)
    pub async fn generate_stream<F>(
        &self,
        prompt: &str,
        config: &GenerationConfig,
        mut callback: F,
    ) -> Result<()>
    where
        F: FnMut(String) -> Result<()>,
    {
        if !self.is_loaded() {
            anyhow::bail!("Model not loaded. Call load() first.");
        }

        log::info!("Streaming generation for prompt: {}", prompt);

        let tokenizer = self.tokenizer.as_ref()
            .context("Tokenizer not loaded")?;

        // Tokenize prompt
        let _token_ids = tokenizer.encode(prompt)?;

        // TODO: Implement actual streaming with Candle when ready
        // For now, simulate streaming with mock response
        let response = self.mock_generate(prompt, config)?;

        // Simulate token-by-token streaming
        let words: Vec<&str> = response.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            let token_text = if i < words.len() - 1 {
                format!("{} ", word)
            } else {
                word.to_string()
            };

            callback(token_text)?;

            // Small delay to simulate inference (remove in production)
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen_futures::JsFuture;
                let promise = js_sys::Promise::new(&mut |resolve, _reject| {
                    web_sys::window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            &resolve,
                            10, // 10ms delay per token
                        )
                        .unwrap();
                });
                let _ = JsFuture::from(promise).await;
            }
        }

        Ok(())
    }

    /// Mock generation (placeholder until Candle WASM is ready)
    fn mock_generate(&self, prompt: &str, config: &GenerationConfig) -> Result<String> {
        // Provide contextual responses based on prompt content
        let response = if prompt.to_lowercase().contains("hello") || prompt.to_lowercase().contains("hi") {
            format!("Hello! I'm Phi-3-mini running in your browser via WebAssembly. How can I help you today?")
        } else if prompt.to_lowercase().contains("what") && prompt.to_lowercase().contains("you") {
            format!("I'm Phi-3-mini, a 3.8 billion parameter language model running entirely in your browser through WebAssembly. I can help with various tasks like answering questions, writing code, creative writing, and more. The current implementation uses a mock inference engine until Candle's WASM support is fully integrated.")
        } else if prompt.to_lowercase().contains("code") || prompt.to_lowercase().contains("function") {
            format!("Here's an example function:\n\n```rust\nfn greet(name: &str) -> String {{\n    format!(\"Hello, {{}}!\", name)\n}}\n```\n\nThis function takes a name and returns a greeting. Would you like me to explain or modify it?")
        } else {
            format!("Thank you for your message: \"{}\"\n\nI'm Phi-3-mini running in WebAssembly. Currently using mock inference (temperature: {}, max_tokens: {}). The actual Candle-based inference will be integrated once WASM support is stable. I can still help with: answering questions, explaining concepts, writing code examples, and creative tasks!", prompt, config.temperature, config.max_tokens)
        };

        Ok(response)
    }

    /// Check if model is loaded
    pub fn is_loaded(&self) -> bool {
        self.model_loaded && self.tokenizer.is_some()
    }

    /// Get model configuration
    pub fn config(&self) -> &ModelConfig {
        &self.config
    }
}
