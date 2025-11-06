/// Quantization utilities for reducing memory usage
pub struct Quantizer;

impl Quantizer {
    /// Quantize f32 vector to int8
    pub fn quantize_int8(data: &[f32]) -> Vec<i8> {
        data.iter()
            .map(|&v| (v * 127.0).clamp(-128.0, 127.0) as i8)
            .collect()
    }

    /// Dequantize int8 vector to f32
    pub fn dequantize_int8(data: &[i8]) -> Vec<f32> {
        data.iter().map(|&v| v as f32 / 127.0).collect()
    }

    /// Quantize f32 vector to uint8 (0-255)
    pub fn quantize_uint8(data: &[f32]) -> Vec<u8> {
        // Assume data is normalized to [-1, 1]
        data.iter()
            .map(|&v| ((v + 1.0) * 127.5).clamp(0.0, 255.0) as u8)
            .collect()
    }

    /// Dequantize uint8 vector to f32
    pub fn dequantize_uint8(data: &[u8]) -> Vec<f32> {
        data.iter()
            .map(|&v| (v as f32 / 127.5) - 1.0)
            .collect()
    }

    /// Binary quantization (1 bit per value)
    pub fn quantize_binary(data: &[f32]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut current_byte = 0u8;
        let mut bit_index = 0;

        for &value in data {
            if value >= 0.0 {
                current_byte |= 1 << bit_index;
            }

            bit_index += 1;
            if bit_index == 8 {
                result.push(current_byte);
                current_byte = 0;
                bit_index = 0;
            }
        }

        if bit_index > 0 {
            result.push(current_byte);
        }

        result
    }

    /// Dequantize binary to f32 (-1.0 or 1.0)
    pub fn dequantize_binary(data: &[u8], original_length: usize) -> Vec<f32> {
        let mut result = Vec::new();

        for &byte in data {
            for bit_index in 0..8 {
                if result.len() >= original_length {
                    break;
                }

                let value = if (byte & (1 << bit_index)) != 0 {
                    1.0
                } else {
                    -1.0
                };
                result.push(value);
            }
        }

        result
    }

    /// Calculate compression ratio
    pub fn compression_ratio(original_size: usize, compressed_size: usize) -> f64 {
        original_size as f64 / compressed_size as f64
    }

    /// Calculate size reduction percentage
    pub fn size_reduction(original_size: usize, compressed_size: usize) -> f64 {
        ((original_size - compressed_size) as f64 / original_size as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int8_quantization() {
        let data = vec![0.5, -0.5, 1.0, -1.0, 0.0];
        let quantized = Quantizer::quantize_int8(&data);
        let dequantized = Quantizer::dequantize_int8(&quantized);

        for (orig, deq) in data.iter().zip(dequantized.iter()) {
            assert!((orig - deq).abs() < 0.02);
        }
    }

    #[test]
    fn test_binary_quantization() {
        let data = vec![0.5, -0.5, 1.0, -1.0, 0.0, 0.3, -0.7, 0.1];
        let quantized = Quantizer::quantize_binary(&data);
        let dequantized = Quantizer::dequantize_binary(&quantized, data.len());

        assert_eq!(data.len(), dequantized.len());

        // Check signs are preserved
        for (orig, deq) in data.iter().zip(dequantized.iter()) {
            assert_eq!(orig.signum(), *deq);
        }
    }

    #[test]
    fn test_compression_metrics() {
        let original = 1000;
        let compressed = 250;

        assert_eq!(Quantizer::compression_ratio(original, compressed), 4.0);
        assert_eq!(Quantizer::size_reduction(original, compressed), 75.0);
    }
}
