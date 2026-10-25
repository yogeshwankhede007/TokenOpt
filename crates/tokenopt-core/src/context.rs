//! Context selection strategies for optimizing token usage

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::counter::{Message, Role, TokenCounter, TokenCount};
use crate::models::ModelCapabilities;

/// Strategy for selecting context to fit within token budget
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SelectionStrategy {
    /// Keep most recent messages (chronological)
    RecentFirst,
    /// Keep most relevant messages (semantic similarity)
    RelevanceFirst,
    /// Keep messages with highest importance scores
    ImportanceFirst,
    /// Balance recency and relevance
    Balanced,
    /// Custom strategy with weights
    Weighted { recency: f32, relevance: f32, importance: f32 },
    /// Sliding window with summary
    SlidingWindow { window_size: usize, summary_interval: usize },
}

/// Context selection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    pub strategy: SelectionStrategy,
    pub max_tokens: usize,
    pub reserve_tokens: usize, // Tokens reserved for response
    pub system_prompt_tokens: usize,
    pub min_messages: usize,
    pub max_messages: Option<usize>,
    pub include_system: bool,
    pub compress_old_messages: bool,
    pub similarity_threshold: f32,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            strategy: SelectionStrategy::Balanced,
            max_tokens: 8000,
            reserve_tokens: 1000,
            system_prompt_tokens: 500,
            min_messages: 2,
            max_messages: None,
            include_system: true,
            compress_old_messages: true,
            similarity_threshold: 0.7,
        }
    }
}

/// A message with computed metadata for selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredMessage {
    pub message: Message,
    pub token_count: usize,
    pub importance_score: f32,
    pub relevance_score: f32,
    pub recency_score: f32,
    pub composite_score: f32,
    pub index: usize,
}

/// Result of context selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedContext {
    pub messages: Vec<Message>,
    pub token_count: TokenCount,
    pub strategy_used: SelectionStrategy,
    pub omitted_count: usize,
    pub omitted_tokens: usize,
    pub summary: Option<String>,
    pub metadata: ContextMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMetadata {
    pub total_available_messages: usize,
    pub total_available_tokens: usize,
    pub selection_time_ms: u64,
    pub compression_ratio: f32,
    pub scores: Vec<MessageScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageScore {
    pub index: usize,
    pub role: Role,
    pub token_count: usize,
    pub importance: f32,
    pub relevance: f32,
    pub recency: f32,
    pub composite: f32,
    pub selected: bool,
}

/// Context selector for optimizing conversation context
pub struct ContextSelector {
    config: ContextConfig,
    counter: Arc<TokenCounter>,
    embedder: Option<Arc<dyn Embedder>>,
}

impl ContextSelector {
    pub fn new(config: ContextConfig, counter: Arc<TokenCounter>) -> Self {
        Self {
            config,
            counter,
            embedder: None,
        }
    }

    pub fn with_embedder(mut self, embedder: Arc<dyn Embedder>) -> Self {
        self.embedder = Some(embedder);
        self
    }

    /// Select context from a conversation to fit within token budget
    pub fn select(&self, messages: &[Message], query: Option<&str>, model: Option<Arc<ModelCapabilities>>) -> Result<SelectedContext, SelectionError> {
        let start = std::time::Instant::now();
        
        let model = model.unwrap_or_else(|| {
            // Default to a reasonable model
            Arc::new(ModelCapabilities {
                id: "default".to_string(),
                name: "Default".to_string(),
                provider: "unknown".to_string(),
                context_window: self.config.max_tokens + self.config.reserve_tokens,
                max_output_tokens: self.config.reserve_tokens,
                pricing: None,
                features: Default::default(),
                family: "default".to_string(),
                is_fast_variant: false,
                knowledge_cutoff: None,
            })
        });

        let available_tokens = model.context_window.saturating_sub(self.config.reserve_tokens);
        let max_tokens = self.config.max_tokens.min(available_tokens);

        // Score all messages
        let mut scored_messages = self.score_messages(messages, query)?;
        
        // Apply selection strategy
        let selected = match self.config.strategy {
            SelectionStrategy::RecentFirst => self.select_recent_first(&mut scored_messages, max_tokens),
            SelectionStrategy::RelevanceFirst => self.select_relevance_first(&mut scored_messages, max_tokens),
            SelectionStrategy::ImportanceFirst => self.select_importance_first(&mut scored_messages, max_tokens),
            SelectionStrategy::Balanced => self.select_balanced(&mut scored_messages, max_tokens),
            SelectionStrategy::Weighted { recency, relevance, importance } => {
                self.select_weighted(&mut scored_messages, max_tokens, recency, relevance, importance)
            }
            SelectionStrategy::SlidingWindow { window_size, summary_interval } => {
                self.select_sliding_window(&mut scored_messages, max_tokens, window_size, summary_interval)
            }
        };

        let selection_time = start.elapsed().as_millis() as u64;
        
        // Build result
        let selected_messages: Vec<Message> = selected.iter().map(|s| s.message.clone()).collect();
        let token_count = self.counter.count_conversation(&selected_messages, Some(&model.id))?;
        
        let omitted_count = messages.len() - selected.len();
        let omitted_tokens = scored_messages.iter()
            .filter(|s| !selected.iter().any(|sel| sel.index == s.index))
            .map(|s| s.token_count)
            .sum();

        let summary = if self.config.compress_old_messages && omitted_count > 0 {
            Some(self.generate_summary(&scored_messages, &selected)?)
        } else {
            None
        };

        let metadata = ContextMetadata {
            total_available_messages: messages.len(),
            total_available_tokens: scored_messages.iter().map(|s| s.token_count).sum(),
            selection_time_ms: selection_time,
            compression_ratio: if messages.is_empty() { 1.0 } else { selected.len() as f32 / messages.len() as f32 },
            scores: scored_messages.iter().map(|s| MessageScore {
                index: s.index,
                role: s.message.role,
                token_count: s.token_count,
                importance: s.importance_score,
                relevance: s.relevance_score,
                recency: s.recency_score,
                composite: s.composite_score,
                selected: selected.iter().any(|sel| sel.index == s.index),
            }).collect(),
        };

        Ok(SelectedContext {
            messages: selected_messages,
            token_count,
            strategy_used: self.config.strategy,
            omitted_count,
            omitted_tokens,
            summary,
            metadata,
        })
    }

    fn score_messages(&self, messages: &[Message], query: Option<&str>) -> Result<Vec<ScoredMessage>, SelectionError> {
        let mut scored = Vec::with_capacity(messages.len());
        let total = messages.len();

        for (index, message) in messages.iter().enumerate() {
            let token_count = self.counter.count_tokens(&message.content, None)?;
            
            // Recency score (0.0 to 1.0, newer = higher)
            let recency_score = if total > 1 {
                index as f32 / (total - 1) as f32
            } else {
                1.0
            };

            // Importance score based on role and content
            let importance_score = self.compute_importance(message);

            // Relevance score (if query provided and embedder available)
            let relevance_score = if let (Some(query), Some(embedder)) = (query, &self.embedder) {
                self.compute_relevance(message, query, embedder.as_ref())?
            } else {
                0.5 // Neutral
            };

            // Composite score
            let composite_score = match self.config.strategy {
                SelectionStrategy::RecentFirst => recency_score,
                SelectionStrategy::RelevanceFirst => relevance_score,
                SelectionStrategy::ImportanceFirst => importance_score,
                SelectionStrategy::Balanced => (recency_score + relevance_score + importance_score) / 3.0,
                SelectionStrategy::Weighted { recency, relevance, importance } => {
                    recency * recency_score + relevance * relevance_score + importance * importance_score
                }
                SelectionStrategy::SlidingWindow { .. } => recency_score,
            };

            scored.push(ScoredMessage {
                message: message.clone(),
                token_count,
                importance_score,
                relevance_score,
                recency_score,
                composite_score,
                index,
            });
        }

        Ok(scored)
    }

    fn compute_importance(&self, message: &Message) -> f32 {
        let mut score: f32 = 0.5; // Base score

        // Role-based importance
        score += match message.role {
            Role::System => 0.3,
            Role::User => 0.2,
            Role::Assistant => 0.1,
            Role::Tool => 0.15,
            Role::Function => 0.15,
        };

        // Content-based heuristics
        let content = &message.content;
        let len = content.len() as f32;
        
        // Longer messages might be more important
        if len > 500.0 { score += 0.1; }
        if len > 2000.0 { score += 0.1; }

        // Code blocks indicate technical content
        if content.contains("```") { score += 0.15; }
        
        // Error messages, stack traces
        if content.to_lowercase().contains("error") || content.to_lowercase().contains("exception") {
            score += 0.1;
        }

        // Questions from user
        if message.role == Role::User && content.contains('?') {
            score += 0.1;
        }

        score.min(1.0)
    }

    fn compute_relevance(&self, message: &Message, query: &str, embedder: &dyn Embedder) -> Result<f32, SelectionError> {
        let query_embedding = embedder.embed(query)?;
        let message_embedding = embedder.embed(&message.content)?;
        
        let similarity = cosine_similarity(&query_embedding, &message_embedding);
        Ok(similarity.max(0.0) as f32)
    }

    fn select_recent_first(&self, scored: &mut [ScoredMessage], max_tokens: usize) -> Vec<ScoredMessage> {
        scored.sort_by_key(|s| std::cmp::Reverse(s.index));
        self.select_by_token_budget(scored, max_tokens)
    }

    fn select_relevance_first(&self, scored: &mut [ScoredMessage], max_tokens: usize) -> Vec<ScoredMessage> {
        scored.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        self.select_by_token_budget(scored, max_tokens)
    }

    fn select_importance_first(&self, scored: &mut [ScoredMessage], max_tokens: usize) -> Vec<ScoredMessage> {
        scored.sort_by(|a, b| b.importance_score.partial_cmp(&a.importance_score).unwrap());
        self.select_by_token_budget(scored, max_tokens)
    }

    fn select_balanced(&self, scored: &mut [ScoredMessage], max_tokens: usize) -> Vec<ScoredMessage> {
        scored.sort_by(|a, b| b.composite_score.partial_cmp(&a.composite_score).unwrap());
        self.select_by_token_budget(scored, max_tokens)
    }

    fn select_weighted(&self, scored: &mut [ScoredMessage], max_tokens: usize, recency: f32, relevance: f32, importance: f32) -> Vec<ScoredMessage> {
        for s in scored.iter_mut() {
            s.composite_score = recency * s.recency_score + relevance * s.relevance_score + importance * s.importance_score;
        }
        scored.sort_by(|a, b| b.composite_score.partial_cmp(&a.composite_score).unwrap());
        self.select_by_token_budget(scored, max_tokens)
    }

    fn select_sliding_window(&self, scored: &mut [ScoredMessage], max_tokens: usize, window_size: usize, summary_interval: usize) -> Vec<ScoredMessage> {
        // Always include the most recent window_size messages
        let recent_start = scored.len().saturating_sub(window_size);
        let recent: Vec<_> = scored[recent_start..].to_vec();
        
        let recent_tokens: usize = recent.iter().map(|s| s.token_count).sum();
        let remaining_budget = max_tokens.saturating_sub(recent_tokens);
        
        // For older messages, include every summary_interval-th message or high-importance ones
        let mut older_selected = Vec::new();
        let mut used_tokens = 0;
        
        for (i, s) in scored[..recent_start].iter().enumerate() {
            if used_tokens + s.token_count > remaining_budget {
                break;
            }
            if i % summary_interval == 0 || s.importance_score > 0.8 {
                older_selected.push(s.clone());
                used_tokens += s.token_count;
            }
        }
        
        let mut result = older_selected;
        result.extend(recent);
        result.sort_by_key(|s| s.index);
        result
    }

    fn select_by_token_budget(&self, scored: &[ScoredMessage], max_tokens: usize) -> Vec<ScoredMessage> {
        let mut selected = Vec::new();
        let mut used_tokens = 0;
        
        // Ensure we have at least min_messages
        let min_messages = self.config.min_messages.min(scored.len());
        
        for s in scored {
            if selected.len() < min_messages {
                selected.push(s.clone());
                used_tokens += s.token_count;
            } else if used_tokens + s.token_count <= max_tokens {
                selected.push(s.clone());
                used_tokens += s.token_count;
            } else if let Some(max_msgs) = self.config.max_messages {
                if selected.len() >= max_msgs {
                    break;
                }
            } else {
                break;
            }
        }
        
        // Sort selected by original index to maintain conversation order
        selected.sort_by_key(|s| s.index);
        selected
    }

    fn generate_summary(&self, all_messages: &[ScoredMessage], selected: &[ScoredMessage]) -> Result<String, SelectionError> {
        let omitted: Vec<_> = all_messages.iter()
            .filter(|s| !selected.iter().any(|sel| sel.index == s.index))
            .collect();

        if omitted.is_empty() {
            return Ok(String::new());
        }

        let mut summary = String::new();
        summary.push_str(&format!("[Summary of {} omitted messages]\n", omitted.len()));

        // Group by role
        let mut by_role: HashMap<Role, Vec<&ScoredMessage>> = HashMap::new();
        for msg in omitted {
            by_role.entry(msg.message.role).or_default().push(msg);
        }

        for (role, msgs) in by_role {
            summary.push_str(&format!("\n{} ({} messages):\n", role.as_str().to_uppercase(), msgs.len()));
            for msg in msgs.iter().take(3) {
                let preview = msg.message.content.chars().take(100).collect::<String>();
                summary.push_str(&format!("  - {}...\n", preview));
            }
            if msgs.len() > 3 {
                summary.push_str(&format!("  ... and {} more\n", msgs.len() - 3));
            }
        }

        Ok(summary)
    }

    pub fn config(&self) -> &ContextConfig {
        &self.config
    }
}

/// Trait for embedding providers
pub trait Embedder: Send + Sync {
    fn embed(&self, text: &str) -> Result<Vec<f32>, SelectionError>;
    fn dimensions(&self) -> usize;
}

/// Simple hash-based embedder for testing/fallback
pub struct HashEmbedder {
    dimensions: usize,
}

impl HashEmbedder {
    pub fn new(dimensions: usize) -> Self {
        Self { dimensions }
    }
}

impl Embedder for HashEmbedder {
    fn embed(&self, text: &str) -> Result<Vec<f32>, SelectionError> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Generate pseudo-random vector from hash
        let mut vec = Vec::with_capacity(self.dimensions);
        let mut x = hash;
        for _ in 0..self.dimensions {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            vec.push(((x >> 32) as f32) / (u32::MAX as f32) * 2.0 - 1.0);
        }
        
        // Normalize
        let norm: f32 = vec.iter().map(|v| v * v).sum::<f32>().sqrt();
        if norm > 0.0 {
            for v in &mut vec {
                *v /= norm;
            }
        }
        
        Ok(vec)
    }

    fn dimensions(&self) -> usize {
        self.dimensions
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SelectionError {
    #[error("Token counting error: {0}")]
    CounterError(#[from] crate::counter::CounterError),
    #[error("Embedding error: {0}")]
    EmbeddingError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counter::{TokenCounter, TokenCounterConfig, Message, Role};

    fn setup() -> (ContextSelector, Vec<Message>) {
        let counter = Arc::new(TokenCounter::new(TokenCounterConfig::default()).unwrap());
        let config = ContextConfig::default();
        let selector = ContextSelector::new(config, counter);
        
        let messages = vec![
            Message { role: Role::System, content: "You are a helpful coding assistant.".to_string(), name: None, tool_calls: None, tool_call_id: None },
            Message { role: Role::User, content: "How do I write a binary search in Rust?".to_string(), name: None, tool_calls: None, tool_call_id: None },
            Message { role: Role::Assistant, content: "Here's a binary search implementation:\n```rust\nfn binary_search(arr: &[i32], target: i32) -> Option<usize> {\n    let mut left = 0;\n    let mut right = arr.len();\n    while left < right {\n        let mid = left + (right - left) / 2;\n        if arr[mid] == target {\n            return Some(mid);\n        } else if arr[mid] < target {\n            left = mid + 1;\n        } else {\n            right = mid;\n        }\n    }\n    None\n}\n```".to_string(), name: None, tool_calls: None, tool_call_id: None },
            Message { role: Role::User, content: "Can you explain the time complexity?".to_string(), name: None, tool_calls: None, tool_call_id: None },
            Message { role: Role::Assistant, content: "Binary search has O(log n) time complexity because we halve the search space each iteration.".to_string(), name: None, tool_calls: None, tool_call_id: None },
        ];
        
        (selector, messages)
    }

    #[test]
    fn test_select_recent_first() {
        let (mut selector, messages) = setup();
        selector.config.strategy = SelectionStrategy::RecentFirst;
        selector.config.max_tokens = 100;
        
        let result = selector.select(&messages, None, None).unwrap();
        assert!(!result.messages.is_empty());
        assert!(result.token_count.total <= 100);
    }

    #[test]
    fn test_select_balanced() {
        let (selector, messages) = setup();
        let result = selector.select(&messages, None, None).unwrap();
        assert!(!result.messages.is_empty());
    }

    #[test]
    fn test_hash_embedder() {
        let embedder = HashEmbedder::new(384);
        let vec1 = embedder.embed("hello world").unwrap();
        let vec2 = embedder.embed("hello world").unwrap();
        let vec3 = embedder.embed("different text").unwrap();
        
        assert_eq!(vec1.len(), 384);
        assert_eq!(vec2.len(), 384);
        assert!((vec1[0] - vec2[0]).abs() < 0.001); // Same input = same embedding
        assert_ne!(vec1, vec3); // Different input = different embedding
    }
}