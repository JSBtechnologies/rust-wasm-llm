// RAG (Retrieval Augmented Generation) module

pub mod chunking;
pub mod embeddings;
pub mod pipeline;
pub mod retrieval;
pub mod vector_db;

pub use chunking::{ChunkingStrategy, DocumentChunker};
pub use embeddings::EmbeddingModel;
pub use pipeline::RagPipeline;
pub use retrieval::Retriever;
pub use vector_db::VectorDatabase;

/// Document chunk with metadata
#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: String,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: ChunkMetadata,
}

/// Chunk metadata
#[derive(Debug, Clone)]
pub struct ChunkMetadata {
    pub document_id: String,
    pub document_name: String,
    pub chunk_index: usize,
    pub start_char: usize,
    pub end_char: usize,
    pub created_at: String,
}

/// Document for RAG system
#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub content: String,
    pub metadata: DocumentMetadata,
}

/// Document metadata
#[derive(Debug, Clone)]
pub struct DocumentMetadata {
    pub file_type: String,
    pub size_bytes: usize,
    pub uploaded_at: String,
    pub num_chunks: usize,
}

/// Search result with similarity score
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub chunk: Chunk,
    pub score: f32,
}
