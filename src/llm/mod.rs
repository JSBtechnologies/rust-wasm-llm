// LLM module for Phi-3 model loading and inference

pub mod config;
pub mod phi_model;
pub mod sampler;
pub mod tokenizer_wrapper;

pub use config::ModelConfig;
pub use phi_model::PhiModel;
pub use sampler::Sampler;
pub use tokenizer_wrapper::TokenizerWrapper;

/// Model loading status
#[derive(Debug, Clone, PartialEq)]
pub enum ModelStatus {
    NotLoaded,
    Loading { progress: f32 },
    Loaded,
    Error { message: String },
}

/// Generation parameters
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenerationConfig {
    pub max_tokens: usize,
    pub temperature: f64,
    pub top_p: f64,
    pub top_k: usize,
    pub repetition_penalty: f64,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            max_tokens: 512,
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            repetition_penalty: 1.1,
        }
    }
}
