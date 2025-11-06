use anyhow::Result;
use super::{Chunk, SearchResult, embeddings::cosine_similarity};

/// Simple in-memory vector database
/// TODO: Integrate with Voy or custom IndexedDB implementation
#[derive(Clone)]
pub struct VectorDatabase {
    chunks: Vec<Chunk>,
}

impl VectorDatabase {
    /// Create a new vector database
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
        }
    }

    /// Add a chunk to the database
    pub async fn add_chunk(&mut self, chunk: Chunk) -> Result<()> {
        if chunk.embedding.is_none() {
            log::warn!("Adding chunk without embedding: {}", chunk.id);
        }

        self.chunks.push(chunk);
        log::debug!("Added chunk to vector database. Total: {}", self.chunks.len());

        Ok(())
    }

    /// Add multiple chunks
    pub async fn add_chunks(&mut self, chunks: Vec<Chunk>) -> Result<()> {
        for chunk in chunks {
            self.add_chunk(chunk).await?;
        }
        Ok(())
    }

    /// Search for similar chunks using cosine similarity
    pub async fn search(
        &self,
        query_embedding: &[f32],
        top_k: usize,
    ) -> Result<Vec<SearchResult>> {
        let mut results: Vec<SearchResult> = self
            .chunks
            .iter()
            .filter_map(|chunk| {
                chunk.embedding.as_ref().map(|emb| {
                    let score = cosine_similarity(query_embedding, emb);
                    SearchResult {
                        chunk: chunk.clone(),
                        score,
                    }
                })
            })
            .collect();

        // Sort by score (descending)
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        // Take top k
        results.truncate(top_k);

        log::debug!(
            "Search returned {} results out of {} chunks",
            results.len(),
            self.chunks.len()
        );

        Ok(results)
    }

    /// Delete chunks by document ID
    pub async fn delete_by_document(&mut self, document_id: &str) -> Result<usize> {
        let initial_count = self.chunks.len();
        self.chunks.retain(|chunk| chunk.metadata.document_id != document_id);
        let deleted = initial_count - self.chunks.len();

        log::info!("Deleted {} chunks for document {}", deleted, document_id);

        Ok(deleted)
    }

    /// Get total number of chunks
    pub fn count(&self) -> usize {
        self.chunks.len()
    }

    /// Clear all chunks
    pub async fn clear(&mut self) -> Result<()> {
        self.chunks.clear();
        log::info!("Cleared vector database");
        Ok(())
    }

    /// Get all unique document IDs
    pub fn get_document_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .chunks
            .iter()
            .map(|c| c.metadata.document_id.clone())
            .collect();
        ids.sort();
        ids.dedup();
        ids
    }

    /// Get chunk count for a specific document
    pub fn count_by_document(&self, document_id: &str) -> usize {
        self.chunks
            .iter()
            .filter(|c| c.metadata.document_id == document_id)
            .count()
    }

    /// Save to IndexedDB (TODO)
    pub async fn save(&self) -> Result<()> {
        // TODO: Serialize and save to IndexedDB using Rexie
        log::warn!("Vector database persistence not yet implemented");
        Ok(())
    }

    /// Load from IndexedDB (TODO)
    pub async fn load() -> Result<Self> {
        // TODO: Load from IndexedDB using Rexie
        log::warn!("Vector database persistence not yet implemented");
        Ok(Self::new())
    }
}

impl Default for VectorDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rag::{ChunkMetadata};

    #[tokio::test]
    async fn test_add_and_search() {
        let mut db = VectorDatabase::new();

        let chunk1 = Chunk {
            id: "1".to_string(),
            content: "Hello world".to_string(),
            embedding: Some(vec![1.0, 0.0, 0.0]),
            metadata: ChunkMetadata {
                document_id: "doc1".to_string(),
                document_name: "Doc 1".to_string(),
                chunk_index: 0,
                start_char: 0,
                end_char: 11,
                created_at: "2025-01-01".to_string(),
            },
        };

        let chunk2 = Chunk {
            id: "2".to_string(),
            content: "Goodbye world".to_string(),
            embedding: Some(vec![0.0, 1.0, 0.0]),
            metadata: ChunkMetadata {
                document_id: "doc1".to_string(),
                document_name: "Doc 1".to_string(),
                chunk_index: 1,
                start_char: 12,
                end_char: 25,
                created_at: "2025-01-01".to_string(),
            },
        };

        db.add_chunk(chunk1).await.unwrap();
        db.add_chunk(chunk2).await.unwrap();

        assert_eq!(db.count(), 2);

        let query = vec![1.0, 0.0, 0.0];
        let results = db.search(&query, 1).await.unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].chunk.id, "1");
    }
}
