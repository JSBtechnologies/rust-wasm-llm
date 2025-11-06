use std::collections::HashMap;

/// Simple in-memory cache for frequently accessed data
pub struct MemoryCache<K, V> {
    data: HashMap<K, V>,
    max_size: usize,
}

impl<K, V> MemoryCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new memory cache with max size
    pub fn new(max_size: usize) -> Self {
        Self {
            data: HashMap::new(),
            max_size,
        }
    }

    /// Get a value from the cache
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    /// Set a value in the cache
    pub fn set(&mut self, key: K, value: V) {
        // Simple eviction: remove oldest if at capacity
        if self.data.len() >= self.max_size && !self.data.contains_key(&key) {
            if let Some(first_key) = self.data.keys().next().cloned() {
                self.data.remove(&first_key);
            }
        }

        self.data.insert(key, value);
    }

    /// Check if cache contains key
    pub fn contains(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Remove a value from the cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get current cache size
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<K, V> Default for MemoryCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic_operations() {
        let mut cache = MemoryCache::new(2);

        cache.set("key1", "value1");
        assert_eq!(cache.get(&"key1"), Some(&"value1"));

        cache.set("key2", "value2");
        assert_eq!(cache.size(), 2);

        cache.set("key3", "value3");
        assert_eq!(cache.size(), 2); // Should evict oldest
    }
}
