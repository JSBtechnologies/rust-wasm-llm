use anyhow::Result;
use super::{Chunk, ChunkMetadata, Document};

/// Chunking strategy
#[derive(Debug, Clone, Copy)]
pub enum ChunkingStrategy {
    FixedSize { size: usize, overlap: usize },
    Recursive { size: usize, overlap: usize },
    Semantic { threshold: f32 },
}

impl Default for ChunkingStrategy {
    fn default() -> Self {
        Self::FixedSize {
            size: 512,
            overlap: 50,
        }
    }
}

/// Document chunker
pub struct DocumentChunker {
    strategy: ChunkingStrategy,
}

impl DocumentChunker {
    /// Create a new document chunker
    pub fn new(strategy: ChunkingStrategy) -> Self {
        Self { strategy }
    }

    /// Chunk a document into smaller pieces
    pub fn chunk(&self, document: &Document) -> Result<Vec<Chunk>> {
        match self.strategy {
            ChunkingStrategy::FixedSize { size, overlap } => {
                self.chunk_fixed_size(document, size, overlap)
            }
            ChunkingStrategy::Recursive { size, overlap } => {
                self.chunk_recursive(document, size, overlap)
            }
            ChunkingStrategy::Semantic { threshold } => {
                self.chunk_semantic(document, threshold)
            }
        }
    }

    /// Fixed-size chunking
    fn chunk_fixed_size(
        &self,
        document: &Document,
        size: usize,
        overlap: usize,
    ) -> Result<Vec<Chunk>> {
        let content = &document.content;
        let mut chunks = Vec::new();
        let mut chunk_index = 0;

        let mut start = 0;
        while start < content.len() {
            let end = (start + size).min(content.len());
            let chunk_content = content[start..end].to_string();

            let chunk = Chunk {
                id: format!("{}_{}", document.id, chunk_index),
                content: chunk_content,
                embedding: None,
                metadata: ChunkMetadata {
                    document_id: document.id.clone(),
                    document_name: document.name.clone(),
                    chunk_index,
                    start_char: start,
                    end_char: end,
                    created_at: Self::current_timestamp(),
                },
            };

            chunks.push(chunk);
            chunk_index += 1;

            // Move start position
            if end >= content.len() {
                break;
            }
            start = end - overlap;
        }

        log::info!(
            "Chunked document '{}' into {} chunks using fixed-size strategy",
            document.name,
            chunks.len()
        );

        Ok(chunks)
    }

    /// Recursive chunking (preserves structure)
    fn chunk_recursive(
        &self,
        document: &Document,
        size: usize,
        overlap: usize,
    ) -> Result<Vec<Chunk>> {
        // TODO: Implement recursive chunking with separators
        // Separators: ["\n\n", "\n", ". ", " "]
        // For now, fall back to fixed-size
        log::warn!("Recursive chunking not yet implemented, using fixed-size");
        self.chunk_fixed_size(document, size, overlap)
    }

    /// Semantic chunking (based on embedding similarity)
    fn chunk_semantic(&self, document: &Document, _threshold: f32) -> Result<Vec<Chunk>> {
        // TODO: Implement semantic chunking
        // Requires embedding model integration
        log::warn!("Semantic chunking not yet implemented, using fixed-size");
        self.chunk_fixed_size(document, 512, 50)
    }

    /// Get current timestamp as ISO 8601 string
    fn current_timestamp() -> String {
        // TODO: Use proper timestamp
        "2025-01-01T00:00:00Z".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_size_chunking() {
        let document = Document {
            id: "test_doc".to_string(),
            name: "Test Document".to_string(),
            content: "a".repeat(1000),
            metadata: super::super::DocumentMetadata {
                file_type: "txt".to_string(),
                size_bytes: 1000,
                uploaded_at: "2025-01-01".to_string(),
                num_chunks: 0,
            },
        };

        let chunker = DocumentChunker::new(ChunkingStrategy::FixedSize {
            size: 100,
            overlap: 10,
        });

        let chunks = chunker.chunk(&document).unwrap();

        assert!(!chunks.is_empty());
        assert!(chunks[0].content.len() <= 100);
    }
}
