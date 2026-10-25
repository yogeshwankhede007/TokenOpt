//! Token caching for optimization results and computed values

use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{Duration, Instant};
use dashmap::DashMap;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::Xxh3;

/// Configuration for the token cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum number of entries in the cache
    pub max_entries: usize,
    /// Time-to-live for cache entries
    pub ttl: Duration,
    /// Enable persistent caching to disk
    pub persist_to_disk: bool,
    /// Directory for persistent cache
    pub cache_dir: Option<String>,
    /// Cache key prefix for namespacing
    pub key_prefix: String,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 100_000,
            ttl: Duration::from_secs(3600), // 1 hour
            persist_to_disk: false,
            cache_dir: None,
            key_prefix: "tokenopt".to_string(),
        }
    }
}

/// Cached value with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedValue<T> {
    pub value: T,
    pub created_at: u64, // Unix timestamp
    pub access_count: u64,
    pub size_bytes: usize,
    pub tags: Vec<String>,
}

impl<T> CachedValue<T> {
    pub fn new(value: T, size_bytes: usize) -> Self {
        Self {
            value,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            access_count: 0,
            size_bytes,
            tags: Vec::new(),
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn is_expired(&self, ttl: Duration) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now - self.created_at > ttl.as_secs()
    }
}

/// Cache key with namespacing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey {
    namespace: String,
    key: String,
}

impl CacheKey {
    pub fn new(namespace: &str, key: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
            key: key.to_string(),
        }
    }

    pub fn from_parts(prefix: &str, parts: &[&str]) -> Self {
        let key = parts.join(":");
        Self::new(prefix, &key)
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = Xxh3::new();
        hasher.write(self.namespace.as_bytes());
        hasher.write(b":");
        hasher.write(self.key.as_bytes());
        hasher.finish()
    }
}

/// Generic token cache with TTL and LRU eviction
pub struct TokenCache {
    config: CacheConfig,
    data: DashMap<u64, CachedValue<Vec<u8>>>,
    metadata: DashMap<u64, CacheMetadata>,
    stats: RwLock<CacheStats>,
}

#[derive(Debug)]
struct CacheMetadata {
    key: String,
    namespace: String,
    tags: Vec<String>,
    created_at: Instant,
    last_accessed: Instant,
}

impl Default for CacheMetadata {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            key: String::new(),
            namespace: String::new(),
            tags: Vec::new(),
            created_at: now,
            last_accessed: now,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub expired: u64,
    pub total_size_bytes: usize,
    pub entry_count: usize,
}

impl TokenCache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            config,
            data: DashMap::new(),
            metadata: DashMap::new(),
            stats: RwLock::new(CacheStats::default()),
        }
    }

    /// Generate a cache key from input data
    pub fn make_key(&self, namespace: &str, data: &[u8]) -> CacheKey {
        let mut hasher = Xxh3::new();
        hasher.write(data);
        let hash = hasher.finish();
        let key = format!("{:x}", hash);
        CacheKey::new(namespace, &key)
    }

    /// Store a value in the cache
    pub fn put<T: Serialize>(&self, key: &CacheKey, value: &T, tags: Vec<String>) -> Result<(), CacheError> {
        let serialized = bincode::serialize(value)
            .map_err(|e| CacheError::SerializationError(e.to_string()))?;
        
        let size = serialized.len();
        let hash = key.hash();
        
        // Check capacity and evict if needed
        if self.data.len() >= self.config.max_entries {
            self.evict_lru();
        }

        let cached = CachedValue::new(serialized, size).with_tags(tags.clone());
        self.data.insert(hash, cached);
        
        self.metadata.insert(hash, CacheMetadata {
            key: key.key.clone(),
            namespace: key.namespace.clone(),
            tags,
            created_at: Instant::now(),
            last_accessed: Instant::now(),
        });

        let mut stats = self.stats.write();
        stats.entry_count = self.data.len();
        stats.total_size_bytes += size;

        Ok(())
    }

    /// Retrieve a value from the cache
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &CacheKey) -> Result<Option<T>, CacheError> {
        let hash = key.hash();
        
        if let Some(mut entry) = self.data.get_mut(&hash) {
            if entry.is_expired(self.config.ttl) {
                self.remove_entry(hash);
                let mut stats = self.stats.write();
                stats.expired += 1;
                stats.misses += 1;
                return Ok(None);
            }

            entry.access_count += 1;
            let value: T = bincode::deserialize(&entry.value)
                .map_err(|e| CacheError::DeserializationError(e.to_string()))?;

            if let Some(mut meta) = self.metadata.get_mut(&hash) {
                meta.last_accessed = Instant::now();
            }

            let mut stats = self.stats.write();
            stats.hits += 1;
            
            Ok(Some(value))
        } else {
            let mut stats = self.stats.write();
            stats.misses += 1;
            Ok(None)
        }
    }

    /// Get or compute a value (memoization pattern)
    pub fn get_or_compute<T, F>(&self, key: &CacheKey, compute: F) -> Result<T, CacheError>
    where
        T: Serialize + for<'de> Deserialize<'de> + Clone,
        F: FnOnce() -> Result<T, CacheError>,
    {
        if let Some(value) = self.get(key)? {
            return Ok(value);
        }

        let value = compute()?;
        self.put(key, &value, Vec::new())?;
        Ok(value)
    }

    /// Invalidate cache entries by tag
    pub fn invalidate_by_tag(&self, tag: &str) -> usize {
        let keys_to_remove: Vec<u64> = self.metadata.iter()
            .filter(|entry| entry.tags.contains(&tag.to_string()))
            .map(|entry| *entry.key())
            .collect();

        let count = keys_to_remove.len();
        for key in keys_to_remove {
            self.remove_entry(key);
        }

        let mut stats = self.stats.write();
        stats.evictions += count as u64;
        stats.entry_count = self.data.len();
        
        count
    }

    /// Invalidate cache entries by namespace
    pub fn invalidate_namespace(&self, namespace: &str) -> usize {
        let keys_to_remove: Vec<u64> = self.metadata.iter()
            .filter(|entry| entry.namespace == namespace)
            .map(|entry| *entry.key())
            .collect();

        let count = keys_to_remove.len();
        for key in keys_to_remove {
            self.remove_entry(key);
        }

        let mut stats = self.stats.write();
        stats.evictions += count as u64;
        stats.entry_count = self.data.len();
        
        count
    }

    /// Clear all cache entries
    pub fn clear(&self) {
        self.data.clear();
        self.metadata.clear();
        let mut stats = self.stats.write();
        *stats = CacheStats::default();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let stats = self.stats.read();
        CacheStats {
            entry_count: self.data.len(),
            ..*stats
        }
    }

    /// Get hit rate
    pub fn hit_rate(&self) -> f64 {
        let stats = self.stats.read();
        let total = stats.hits + stats.misses;
        if total == 0 {
            0.0
        } else {
            stats.hits as f64 / total as f64
        }
    }

    fn remove_entry(&self, hash: u64) {
        if let Some((_, entry)) = self.data.remove(&hash) {
            let mut stats = self.stats.write();
            stats.total_size_bytes = stats.total_size_bytes.saturating_sub(entry.size_bytes);
        }
        self.metadata.remove(&hash);
    }

    fn evict_lru(&self) {
        // Find least recently used entry
        let lru_key = self.metadata.iter()
            .min_by_key(|entry| entry.last_accessed)
            .map(|entry| *entry.key());

        if let Some(key) = lru_key {
            self.remove_entry(key);
            let mut stats = self.stats.write();
            stats.evictions += 1;
        }
    }

    /// Clean up expired entries
    pub fn cleanup_expired(&self) -> usize {
        let expired_keys: Vec<u64> = self.data.iter()
            .filter(|entry| entry.is_expired(self.config.ttl))
            .map(|entry| *entry.key())
            .collect();

        let count = expired_keys.len();
        for key in expired_keys {
            self.remove_entry(key);
        }

        let mut stats = self.stats.write();
        stats.expired += count as u64;
        stats.entry_count = self.data.len();
        
        count
    }
}

/// Specialized cache for token counts
pub struct TokenCountCache {
    cache: TokenCache,
}

impl TokenCountCache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: TokenCache::new(config),
        }
    }

    pub fn get_count(&self, text: &str, model: &str) -> Option<usize> {
        let key = self.cache.make_key("token_count", format!("{}:{}", model, text).as_bytes());
        self.cache.get(&key).ok().flatten()
    }

    pub fn set_count(&self, text: &str, model: &str, count: usize) -> Result<(), CacheError> {
        let key = self.cache.make_key("token_count", format!("{}:{}", model, text).as_bytes());
        self.cache.put(&key, &count, vec!["token_count".to_string(), model.to_string()])
    }

    pub fn get_conversation_count(&self, messages: &[crate::counter::Message], model: &str) -> Option<crate::counter::TokenCount> {
        let serialized = serde_json::to_vec(messages).ok()?;
        let key = self.cache.make_key("conversation_count", format!("{}:{}", model, String::from_utf8_lossy(&serialized)).as_bytes());
        self.cache.get(&key).ok().flatten()
    }

    pub fn set_conversation_count(&self, messages: &[crate::counter::Message], model: &str, count: &crate::counter::TokenCount) -> Result<(), CacheError> {
        let serialized = serde_json::to_vec(messages).map_err(|e| CacheError::SerializationError(e.to_string()))?;
        let key = self.cache.make_key("conversation_count", format!("{}:{}", model, String::from_utf8_lossy(&serialized)).as_bytes());
        self.cache.put(&key, count, vec!["conversation_count".to_string(), model.to_string()])
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Cache key not found")]
    KeyNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let cache = TokenCache::new(CacheConfig::default());
        let key = CacheKey::new("test", "key1");
        
        cache.put(&key, &"hello world", vec![]).unwrap();
        let value: Option<String> = cache.get(&key).unwrap();
        assert_eq!(value, Some("hello world".to_string()));
    }

    #[test]
    fn test_cache_miss() {
        let cache = TokenCache::new(CacheConfig::default());
        let key = CacheKey::new("test", "nonexistent");
        
        let value: Option<String> = cache.get(&key).unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn test_cache_stats() {
        let cache = TokenCache::new(CacheConfig::default());
        let key = CacheKey::new("test", "key1");
        
        cache.put(&key, &"value", vec![]).unwrap();
        let _: Option<String> = cache.get(&key).unwrap(); // hit
        let _: Option<String> = cache.get(&CacheKey::new("test", "key2")).unwrap(); // miss
        
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(cache.hit_rate(), 0.5);
    }

    #[test]
    fn test_token_count_cache() {
        let cache = TokenCountCache::new(CacheConfig::default());
        
        cache.set_count("hello world", "gpt-4", 3).unwrap();
        let count = cache.get_count("hello world", "gpt-4");
        assert_eq!(count, Some(3));
        
        let count = cache.get_count("different text", "gpt-4");
        assert_eq!(count, None);
    }
}