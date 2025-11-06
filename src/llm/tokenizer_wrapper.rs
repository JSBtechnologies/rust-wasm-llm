use anyhow::{Result, Context};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// Wrapper around the tokenizers crate for WASM compatibility
pub struct TokenizerWrapper {
    tokenizer: Option<tokenizers::Tokenizer>,
    tokenizer_url: String,
}

impl TokenizerWrapper {
    /// Create a new tokenizer wrapper
    pub fn new(tokenizer_url: String) -> Self {
        Self {
            tokenizer: None,
            tokenizer_url,
        }
    }

    /// Load the tokenizer from a URL
    pub async fn load(&mut self) -> Result<()> {
        log::info!("Loading tokenizer from: {}", self.tokenizer_url);

        // Step 1: Fetch tokenizer.json from URL
        let tokenizer_json = self.fetch_tokenizer_json(&self.tokenizer_url).await
            .context("Failed to fetch tokenizer.json")?;

        log::debug!("Fetched tokenizer.json: {} bytes", tokenizer_json.len());

        // Step 2: Parse JSON and create Tokenizer
        let tokenizer = tokenizers::Tokenizer::from_bytes(&tokenizer_json)
            .map_err(|e| anyhow::anyhow!("Failed to parse tokenizer: {:?}", e))?;

        log::info!("Tokenizer parsed successfully (vocab size: {})", tokenizer.get_vocab_size(true));

        // Step 3: Verify tokenizer works with a simple test
        let test_encoding = tokenizer.encode("Hello", false)
            .map_err(|e| anyhow::anyhow!("Tokenizer verification failed: {:?}", e))?;

        log::debug!("Tokenizer verification passed (test encoding: {} tokens)", test_encoding.len());

        self.tokenizer = Some(tokenizer);
        log::info!("✅ Tokenizer loaded successfully");

        Ok(())
    }

    /// Fetch tokenizer.json from URL
    async fn fetch_tokenizer_json(&self, url: &str) -> Result<Vec<u8>> {
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

        let uint8_array = js_sys::Uint8Array::new(&array_buffer);
        let bytes = uint8_array.to_vec();

        Ok(bytes)
    }

    /// Encode text to token IDs
    pub fn encode(&self, text: &str) -> Result<Vec<u32>> {
        let tokenizer = self.tokenizer.as_ref()
            .context("Tokenizer not loaded. Call load() first.")?;

        log::debug!("Encoding text: {} chars", text.len());

        let encoding = tokenizer.encode(text, false)
            .map_err(|e| anyhow::anyhow!("Encoding failed: {:?}", e))?;

        let ids = encoding.get_ids().to_vec();

        log::debug!("Encoded to {} tokens", ids.len());

        Ok(ids)
    }

    /// Decode token IDs to text
    pub fn decode(&self, token_ids: &[u32]) -> Result<String> {
        let tokenizer = self.tokenizer.as_ref()
            .context("Tokenizer not loaded. Call load() first.")?;

        log::debug!("Decoding {} tokens", token_ids.len());

        let text = tokenizer.decode(token_ids, true)
            .map_err(|e| anyhow::anyhow!("Decoding failed: {:?}", e))?;

        log::debug!("Decoded to {} chars", text.len());

        Ok(text)
    }

    /// Encode text and return both tokens and IDs
    pub fn encode_with_ids(&self, text: &str) -> Result<(Vec<String>, Vec<u32>)> {
        let tokenizer = self.tokenizer.as_ref()
            .context("Tokenizer not loaded. Call load() first.")?;

        let encoding = tokenizer.encode(text, false)
            .map_err(|e| anyhow::anyhow!("Encoding failed: {:?}", e))?;

        let ids = encoding.get_ids().to_vec();
        let tokens: Vec<String> = encoding.get_tokens()
            .iter()
            .map(|s| s.to_string())
            .collect();

        Ok((tokens, ids))
    }

    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.tokenizer
            .as_ref()
            .map(|t| t.get_vocab_size(true))
            .unwrap_or(0)
    }

    /// Check if tokenizer is loaded
    pub fn is_loaded(&self) -> bool {
        self.tokenizer.is_some()
    }

    /// Get the tokenizer reference (for advanced usage)
    pub fn tokenizer(&self) -> Option<&tokenizers::Tokenizer> {
        self.tokenizer.as_ref()
    }
}
