use anyhow::Result;
use super::{EmbeddingModel, VectorDatabase, SearchResult};

/// Retriever for finding relevant chunks
pub struct Retriever {
    vector_db: VectorDatabase,
    embedding_model: EmbeddingModel,
}

impl Retriever {
    /// Create a new retriever
    pub fn new(vector_db: VectorDatabase, embedding_model: EmbeddingModel) -> Self {
        Self {
            vector_db,
            embedding_model,
        }
    }

    /// Retrieve top-k relevant chunks for a query
    pub async fn retrieve(&self, query: &str, top_k: usize) -> Result<Vec<SearchResult>> {
        log::info!("Retrieving top-{} chunks for query: {}", top_k, query);

        // Generate embedding for query
        let query_embedding = self.embedding_model.embed(query).await?;

        // Search vector database
        let results = self.vector_db.search(&query_embedding, top_k).await?;

        log::info!("Retrieved {} results", results.len());

        Ok(results)
    }

    /// Retrieve and format context for LLM
    pub async fn retrieve_context(&self, query: &str, top_k: usize) -> Result<String> {
        let results = self.retrieve(query, top_k).await?;

        // Format results as context
        let mut context = String::new();
        context.push_str("Relevant context:\n\n");

        for (i, result) in results.iter().enumerate() {
            context.push_str(&format!(
                "Document {}: {}\n",
                i + 1,
                result.chunk.metadata.document_name
            ));
            context.push_str(&format!("Content: {}\n\n", result.chunk.content));
        }

        Ok(context)
    }

    /// Get reference to vector database
    pub fn vector_db(&self) -> &VectorDatabase {
        &self.vector_db
    }

    /// Get mutable reference to vector database
    pub fn vector_db_mut(&mut self) -> &mut VectorDatabase {
        &mut self.vector_db
    }

    /// Get reference to embedding model
    pub fn embedding_model(&self) -> &EmbeddingModel {
        &self.embedding_model
    }
}
