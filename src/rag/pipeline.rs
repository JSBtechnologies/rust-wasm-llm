use anyhow::Result;
use super::{
    Document, DocumentChunker, ChunkingStrategy, EmbeddingModel,
    VectorDatabase, Retriever,
};

/// RAG pipeline that orchestrates the entire RAG workflow
pub struct RagPipeline {
    chunker: DocumentChunker,
    embedding_model: EmbeddingModel,
    vector_db: VectorDatabase,
}

impl RagPipeline {
    /// Create a new RAG pipeline
    pub fn new(
        chunking_strategy: ChunkingStrategy,
        embedding_model: EmbeddingModel,
        vector_db: VectorDatabase,
    ) -> Self {
        Self {
            chunker: DocumentChunker::new(chunking_strategy),
            embedding_model,
            vector_db,
        }
    }

    /// Index a document (chunk + embed + store)
    pub async fn index_document(&mut self, document: Document) -> Result<usize> {
        log::info!("Indexing document: {}", document.name);

        // Step 1: Chunk the document
        let mut chunks = self.chunker.chunk(&document)?;
        let num_chunks = chunks.len();

        log::info!("Created {} chunks", num_chunks);

        // Step 2: Generate embeddings for each chunk
        log::info!("Generating embeddings...");
        let texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
        let embeddings = self.embedding_model.embed_batch(&texts).await?;

        // Attach embeddings to chunks
        for (chunk, embedding) in chunks.iter_mut().zip(embeddings.iter()) {
            chunk.embedding = Some(embedding.clone());
        }

        log::info!("Generated {} embeddings", embeddings.len());

        // Step 3: Store chunks in vector database
        self.vector_db.add_chunks(chunks).await?;

        log::info!("Successfully indexed document with {} chunks", num_chunks);

        Ok(num_chunks)
    }

    /// Query the RAG system
    pub async fn query(&self, question: &str, top_k: usize) -> Result<String> {
        log::info!("RAG query: {} (top_k={})", question, top_k);

        // Create retriever
        let retriever = Retriever::new(
            self.vector_db.clone(), // TODO: Use Arc or reference
            EmbeddingModel::new("all-MiniLM-L6-v2".to_string()), // TODO: Clone embedding model
        );

        // Retrieve relevant context
        let context = retriever.retrieve_context(question, top_k).await?;

        Ok(context)
    }

    /// Delete a document from the RAG system
    pub async fn delete_document(&mut self, document_id: &str) -> Result<usize> {
        self.vector_db.delete_by_document(document_id).await
    }

    /// Get statistics about the RAG system
    pub fn stats(&self) -> RagStats {
        RagStats {
            total_chunks: self.vector_db.count(),
            total_documents: self.vector_db.get_document_ids().len(),
        }
    }

    /// Get reference to vector database
    pub fn vector_db(&self) -> &VectorDatabase {
        &self.vector_db
    }

    /// Get mutable reference to vector database
    pub fn vector_db_mut(&mut self) -> &mut VectorDatabase {
        &mut self.vector_db
    }

    /// Clear all indexed data
    pub async fn clear(&mut self) -> Result<()> {
        self.vector_db.clear().await
    }
}

/// RAG system statistics
#[derive(Debug, Clone)]
pub struct RagStats {
    pub total_chunks: usize,
    pub total_documents: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rag::DocumentMetadata;

    #[tokio::test]
    async fn test_rag_pipeline() {
        let pipeline = RagPipeline::new(
            ChunkingStrategy::default(),
            EmbeddingModel::new("test".to_string()),
            VectorDatabase::new(),
        );

        let document = Document {
            id: "test_doc".to_string(),
            name: "Test Document".to_string(),
            content: "This is a test document with some content.".to_string(),
            metadata: DocumentMetadata {
                file_type: "txt".to_string(),
                size_bytes: 43,
                uploaded_at: "2025-01-01".to_string(),
                num_chunks: 0,
            },
        };

        let stats = pipeline.stats();
        assert_eq!(stats.total_chunks, 0);
    }
}
