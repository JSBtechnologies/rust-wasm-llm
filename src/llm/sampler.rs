use anyhow::Result;
use std::collections::HashMap;

use super::GenerationConfig;

/// Token sampler for text generation
pub struct Sampler {
    /// Previously generated token IDs (for repetition penalty)
    generated_tokens: Vec<u32>,
    /// Token frequency count (for repetition penalty)
    token_counts: HashMap<u32, usize>,
}

impl Sampler {
    /// Create a new sampler
    pub fn new() -> Self {
        Self {
            generated_tokens: Vec::new(),
            token_counts: HashMap::new(),
        }
    }

    /// Reset the sampler state
    pub fn reset(&mut self) {
        self.generated_tokens.clear();
        self.token_counts.clear();
    }

    /// Sample the next token from logits
    ///
    /// # Arguments
    /// * `logits` - Raw logits from the model (vocab_size)
    /// * `config` - Generation configuration (temperature, top_k, top_p, etc.)
    ///
    /// # Returns
    /// The sampled token ID
    pub fn sample(&mut self, logits: &[f32], config: &GenerationConfig) -> Result<u32> {
        if logits.is_empty() {
            anyhow::bail!("Logits cannot be empty");
        }

        // Step 1: Apply repetition penalty
        let mut adjusted_logits = logits.to_vec();
        self.apply_repetition_penalty(&mut adjusted_logits, config.repetition_penalty);

        // Step 2: Apply temperature scaling
        if config.temperature > 0.0 {
            for logit in &mut adjusted_logits {
                *logit /= config.temperature as f32;
            }
        }

        // Step 3: Convert logits to probabilities (softmax)
        let probs = softmax(&adjusted_logits);

        // Step 4: Apply top-k filtering
        let probs = if config.top_k > 0 && config.top_k < probs.len() {
            top_k_filtering(&probs, config.top_k)
        } else {
            probs
        };

        // Step 5: Apply top-p (nucleus) filtering
        let probs = if config.top_p < 1.0 {
            top_p_filtering(&probs, config.top_p)
        } else {
            probs
        };

        // Step 6: Sample from the filtered distribution
        let token_id = if config.temperature == 0.0 {
            // Greedy sampling (temperature 0)
            argmax(&probs)
        } else {
            // Multinomial sampling
            multinomial_sample(&probs)?
        };

        // Step 7: Track this token for repetition penalty
        self.generated_tokens.push(token_id);
        *self.token_counts.entry(token_id).or_insert(0) += 1;

        Ok(token_id)
    }

    /// Apply repetition penalty to logits
    fn apply_repetition_penalty(&self, logits: &mut [f32], penalty: f64) {
        if penalty == 1.0 {
            return; // No penalty
        }

        for (token_id, &count) in &self.token_counts {
            let idx = *token_id as usize;
            if idx < logits.len() {
                // Apply penalty: divide logit by penalty for each occurrence
                let total_penalty = penalty.powi(count as i32) as f32;
                if logits[idx] > 0.0 {
                    logits[idx] /= total_penalty;
                } else {
                    logits[idx] *= total_penalty;
                }
            }
        }
    }

    /// Get the generated tokens so far
    pub fn generated_tokens(&self) -> &[u32] {
        &self.generated_tokens
    }
}

impl Default for Sampler {
    fn default() -> Self {
        Self::new()
    }
}

/// Softmax function to convert logits to probabilities
fn softmax(logits: &[f32]) -> Vec<f32> {
    // Find max for numerical stability
    let max_logit = logits.iter().copied().fold(f32::NEG_INFINITY, f32::max);

    // Compute exp(x - max) and sum
    let exp_logits: Vec<f32> = logits
        .iter()
        .map(|&x| (x - max_logit).exp())
        .collect();

    let sum: f32 = exp_logits.iter().sum();

    // Normalize
    exp_logits.iter().map(|&x| x / sum).collect()
}

/// Top-k filtering: keep only top k tokens
fn top_k_filtering(probs: &[f32], k: usize) -> Vec<f32> {
    // Create (index, prob) pairs and sort by probability descending
    let mut indexed_probs: Vec<(usize, f32)> = probs
        .iter()
        .enumerate()
        .map(|(i, &p)| (i, p))
        .collect();

    indexed_probs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Zero out probabilities outside top-k
    let mut filtered = vec![0.0; probs.len()];
    let mut sum = 0.0;
    for (i, &(idx, prob)) in indexed_probs.iter().take(k).enumerate() {
        filtered[idx] = prob;
        sum += prob;
    }

    // Renormalize
    if sum > 0.0 {
        for p in &mut filtered {
            *p /= sum;
        }
    }

    filtered
}

/// Top-p (nucleus) filtering: keep tokens with cumulative probability >= p
fn top_p_filtering(probs: &[f32], p: f64) -> Vec<f32> {
    // Create (index, prob) pairs and sort by probability descending
    let mut indexed_probs: Vec<(usize, f32)> = probs
        .iter()
        .enumerate()
        .map(|(i, &prob)| (i, prob))
        .collect();

    indexed_probs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Find cutoff index where cumulative probability >= p
    let mut cumulative = 0.0;
    let mut cutoff_idx = indexed_probs.len();

    for (i, &(_, prob)) in indexed_probs.iter().enumerate() {
        cumulative += prob;
        if cumulative >= p as f32 {
            cutoff_idx = i + 1;
            break;
        }
    }

    // Zero out probabilities beyond cutoff
    let mut filtered = vec![0.0; probs.len()];
    let mut sum = 0.0;
    for &(idx, prob) in indexed_probs.iter().take(cutoff_idx) {
        filtered[idx] = prob;
        sum += prob;
    }

    // Renormalize
    if sum > 0.0 {
        for prob in &mut filtered {
            *prob /= sum;
        }
    }

    filtered
}

/// Find index of maximum value (for greedy sampling)
fn argmax(probs: &[f32]) -> u32 {
    probs
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(idx, _)| idx as u32)
        .unwrap_or(0)
}

/// Multinomial sampling from a probability distribution
fn multinomial_sample(probs: &[f32]) -> Result<u32> {
    // Simple implementation using cumulative distribution
    // In a real implementation, you'd use a proper RNG
    // For WASM, we can use js_sys::Math::random()

    #[cfg(target_arch = "wasm32")]
    {
        let random_value = js_sys::Math::random() as f32;
        let mut cumulative = 0.0;

        for (idx, &prob) in probs.iter().enumerate() {
            cumulative += prob;
            if random_value <= cumulative {
                return Ok(idx as u32);
            }
        }

        // Fallback: return last non-zero token
        for (idx, &prob) in probs.iter().enumerate().rev() {
            if prob > 0.0 {
                return Ok(idx as u32);
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // For non-WASM (testing), use simple random
        use rand::Rng;
        let random_value: f32 = rand::thread_rng().gen();
        let mut cumulative = 0.0;

        for (idx, &prob) in probs.iter().enumerate() {
            cumulative += prob;
            if random_value <= cumulative {
                return Ok(idx as u32);
            }
        }
    }

    // Final fallback
    Ok(argmax(probs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_softmax() {
        let logits = vec![1.0, 2.0, 3.0];
        let probs = softmax(&logits);

        // Sum should be ~1.0
        let sum: f32 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);

        // Highest logit should have highest probability
        assert!(probs[2] > probs[1]);
        assert!(probs[1] > probs[0]);
    }

    #[test]
    fn test_argmax() {
        let probs = vec![0.1, 0.5, 0.3, 0.1];
        assert_eq!(argmax(&probs), 1);
    }

    #[test]
    fn test_top_k_filtering() {
        let probs = vec![0.1, 0.2, 0.3, 0.4];
        let filtered = top_k_filtering(&probs, 2);

        // Only top 2 should be non-zero
        assert!(filtered[3] > 0.0); // 0.4
        assert!(filtered[2] > 0.0); // 0.3
        assert_eq!(filtered[1], 0.0);
        assert_eq!(filtered[0], 0.0);
    }

    #[test]
    fn test_sampler_basic() {
        let mut sampler = Sampler::new();
        let logits = vec![1.0, 2.0, 3.0, 4.0];
        let config = GenerationConfig::default();

        let token = sampler.sample(&logits, &config).unwrap();

        // Should return a valid token ID
        assert!(token < 4);

        // Should track generated token
        assert_eq!(sampler.generated_tokens().len(), 1);
    }
}
