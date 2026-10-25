//! Conversation summarization for token optimization

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::counter::{TokenCounter, Message, TokenCount, Role};
use crate::context::{ContextSelector, ContextConfig, SelectionStrategy};
use textwrap::wrap;

/// Summarization strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SummaryStrategy {
    /// Extractive: select key sentences/messages
    Extractive,
    /// Abstractive: generate new summary (requires LLM)
    Abstractive,
    /// Hybrid: extractive first, then compress
    Hybrid,
    /// Rolling: maintain running summary
    Rolling { window_size: usize },
    /// Hierarchical: summarize at multiple levels
    Hierarchical { levels: usize },
}

/// Configuration for summarization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryConfig {
    pub strategy: SummaryStrategy,
    pub max_summary_tokens: usize,
    pub target_compression_ratio: f32, // 0.0 to 1.0
    pub preserve_code_blocks: bool,
    pub preserve_key_decisions: bool,
    pub preserve_errors: bool,
    pub min_messages_to_summarize: usize,
    pub summary_prompt_template: Option<String>,
}

impl Default for SummaryConfig {
    fn default() -> Self {
        Self {
            strategy: SummaryStrategy::Extractive,
            max_summary_tokens: 1000,
            target_compression_ratio: 0.3,
            preserve_code_blocks: true,
            preserve_key_decisions: true,
            preserve_errors: true,
            min_messages_to_summarize: 10,
            summary_prompt_template: None,
        }
    }
}

/// Result of summarization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryResult {
    pub summary: String,
    pub token_count: usize,
    pub original_token_count: usize,
    pub compression_ratio: f32,
    pub strategy_used: SummaryStrategy,
    pub messages_summarized: usize,
    pub key_points: Vec<KeyPoint>,
    pub metadata: SummaryMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPoint {
    pub content: String,
    pub importance: f32,
    pub message_indices: Vec<usize>,
    pub category: KeyPointCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyPointCategory {
    Decision,
    Error,
    Code,
    Requirement,
    Question,
    Answer,
    Context,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryMetadata {
    pub processing_time_ms: u64,
    pub original_message_count: usize,
    pub messages_in_summary: usize,
    pub warnings: Vec<String>,
}

/// Conversation summarizer
pub struct ConversationSummarizer {
    counter: Arc<TokenCounter>,
    context_selector: ContextSelector,
    config: SummaryConfig,
}

impl ConversationSummarizer {
    pub fn new(counter: Arc<TokenCounter>) -> Self {
        let context_config = ContextConfig {
            strategy: SelectionStrategy::ImportanceFirst,
            max_tokens: 4000,
            ..Default::default()
        };
        let context_selector = ContextSelector::new(context_config, counter.clone());
        
        Self {
            counter,
            context_selector,
            config: SummaryConfig::default(),
        }
    }

    pub fn with_config(mut self, config: SummaryConfig) -> Self {
        self.config = config.clone();
        self.context_selector = ContextSelector::new(ContextConfig {
            max_tokens: config.max_summary_tokens,
            strategy: SelectionStrategy::ImportanceFirst,
            ..Default::default()
        }, self.counter.clone());
        self
    }

    /// Summarize a conversation
    pub fn summarize(&self, messages: &[Message], config: &SummaryConfig) -> Result<SummaryResult, SummaryError> {
        let start = std::time::Instant::now();
        
        if messages.len() < config.min_messages_to_summarize {
            return Err(SummaryError::TooFewMessages { min: config.min_messages_to_summarize });
        }

        let original_count = self.counter.count_conversation(messages, None)?;
        
        let result = match config.strategy {
            SummaryStrategy::Extractive => self.extractive_summarize(messages, config)?,
            SummaryStrategy::Abstractive => self.abstractive_summarize(messages, config)?,
            SummaryStrategy::Hybrid => self.hybrid_summarize(messages, config)?,
            SummaryStrategy::Rolling { window_size } => self.rolling_summarize(messages, config, window_size)?,
            SummaryStrategy::Hierarchical { levels } => self.hierarchical_summarize(messages, config, levels)?,
        };

        let processing_time = start.elapsed().as_millis() as u64;
        
        Ok(SummaryResult {
            metadata: SummaryMetadata {
                processing_time_ms: processing_time,
                original_message_count: messages.len(),
                messages_in_summary: result.key_points.iter().map(|kp| kp.message_indices.len()).sum(),
                warnings: Vec::new(),
            },
            ..result
        })
    }

    fn extractive_summarize(&self, messages: &[Message], config: &SummaryConfig) -> Result<SummaryResult, SummaryError> {
        // Score and select important messages
        let scored = self.score_messages(messages)?;
        
        // Select top messages within token budget
        let mut selected = Vec::new();
        let mut used_tokens = 0;
        
        for scored_msg in scored {
            let token_count = scored_msg.token_count;
            if used_tokens + token_count <= config.max_summary_tokens {
                selected.push(scored_msg);
                used_tokens += token_count;
            } else {
                break;
            }
        }

        // Build summary from selected messages
        let mut summary_parts = Vec::new();
        let mut key_points = Vec::new();
        
        for (idx, scored_msg) in selected.iter().enumerate() {
            let msg = &scored_msg.message;
            let category = self.categorize_message(msg);
            
            if config.preserve_code_blocks && msg.content.contains("```") {
                // Include code blocks in full
                summary_parts.push(format!("[CODE] {}", msg.content));
            } else if config.preserve_errors && self.is_error_message(msg) {
                summary_parts.push(format!("[ERROR] {}", self.truncate(&msg.content, 200)));
            } else if config.preserve_key_decisions && category == KeyPointCategory::Decision {
                summary_parts.push(format!("[DECISION] {}", self.truncate(&msg.content, 300)));
            } else {
                summary_parts.push(self.truncate(&msg.content, 150));
            }
            
            key_points.push(KeyPoint {
                content: self.truncate(&msg.content, 200),
                importance: scored_msg.importance_score,
                message_indices: vec![scored_msg.index],
                category,
            });
        }

        let summary = summary_parts.join("\n\n---\n\n");
        let token_count = self.counter.count_tokens(&summary, None)?;
        let original_count = self.counter.count_conversation(messages, None)?;

        Ok(SummaryResult {
            summary,
            token_count,
            original_token_count: original_count.total,
            compression_ratio: token_count as f32 / original_count.total as f32,
            strategy_used: SummaryStrategy::Extractive,
            messages_summarized: selected.len(),
            key_points,
            metadata: SummaryMetadata::default(),
        })
    }

    fn abstractive_summarize(&self, messages: &[Message], config: &SummaryConfig) -> Result<SummaryResult, SummaryError> {
        // For abstractive, we'd need an LLM call
        // Fall back to extractive with a note
        let mut result = self.extractive_summarize(messages, config)?;
        result.strategy_used = SummaryStrategy::Abstractive;
        result.summary = format!("[Abstractive Summary]\n{}", result.summary);
        result.metadata.warnings.push("Abstractive summarization not fully implemented; used extractive fallback".to_string());
        Ok(result)
    }

    fn hybrid_summarize(&self, messages: &[Message], config: &SummaryConfig) -> Result<SummaryResult, SummaryError> {
        // First extractive, then compress further
        let mut extractive = self.extractive_summarize(messages, config)?;
        
        if extractive.token_count > config.max_summary_tokens {
            // Compress the extractive summary
            let compressed = self.compress_text(&extractive.summary, config.max_summary_tokens)?;
            extractive.summary = compressed;
            extractive.token_count = self.counter.count_tokens(&extractive.summary, None)?;
            extractive.compression_ratio = extractive.token_count as f32 / extractive.original_token_count as f32;
            extractive.strategy_used = SummaryStrategy::Hybrid;
        }
        
        Ok(extractive)
    }

    fn rolling_summarize(&self, messages: &[Message], config: &SummaryConfig, window_size: usize) -> Result<SummaryResult, SummaryError> {
        // Maintain a rolling summary of the last window_size messages
        let recent_start = messages.len().saturating_sub(window_size);
        let recent_messages = &messages[recent_start..];
        let older_messages = &messages[..recent_start];
        
        // Compute original token count once
        let original_count = self.counter.count_conversation(messages, None)?;
        
        // Summarize older messages
        let older_summary = if !older_messages.is_empty() {
            self.extractive_summarize(older_messages, config)?
        } else {
            SummaryResult {
                summary: String::new(),
                token_count: 0,
                original_token_count: 0,
                compression_ratio: 1.0,
                strategy_used: SummaryStrategy::Rolling { window_size },
                messages_summarized: 0,
                key_points: Vec::new(),
                metadata: SummaryMetadata::default(),
            }
        };
        
        // Keep recent messages in full
        let _recent_count = self.counter.count_conversation(recent_messages, None)?;
        let recent_content: String = recent_messages.iter()
            .map(|m| format!("{}: {}", m.role.as_str(), m.content))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        let combined = if older_summary.summary.is_empty() {
            recent_content
        } else {
            format!("{}\n\n---\n\n[Recent Messages]\n{}", older_summary.summary, recent_content)
        };
        
        let token_count = self.counter.count_tokens(&combined, None)?;
        
        Ok(SummaryResult {
            summary: combined,
            token_count,
            original_token_count: original_count.total,
            compression_ratio: token_count as f32 / original_count.total as f32,
            strategy_used: SummaryStrategy::Rolling { window_size },
            messages_summarized: older_messages.len(),
            key_points: older_summary.key_points,
            metadata: SummaryMetadata::default(),
        })
    }

    fn hierarchical_summarize(&self, messages: &[Message], config: &SummaryConfig, levels: usize) -> Result<SummaryResult, SummaryError> {
        // Create summaries at multiple granularities
        let chunk_size = messages.len() / levels.max(1);
        let mut all_key_points = Vec::new();
        let mut level_summaries = Vec::new();
        
        for level in 0..levels {
            let start = level * chunk_size;
            let end = if level == levels - 1 { messages.len() } else { (level + 1) * chunk_size };
            let chunk = &messages[start..end];
            
            if chunk.is_empty() { continue; }
            
            let chunk_summary = self.extractive_summarize(chunk, config)?;
            all_key_points.extend(chunk_summary.key_points);
            level_summaries.push(format!("[Level {}] {}", level + 1, chunk_summary.summary));
        }
        
        let combined = level_summaries.join("\n\n===\n\n");
        let token_count = self.counter.count_tokens(&combined, None)?;
        
        Ok(SummaryResult {
            summary: combined,
            token_count,
            original_token_count: self.counter.count_conversation(messages, None)?.total,
            compression_ratio: token_count as f32 / self.counter.count_conversation(messages, None)?.total as f32,
            strategy_used: SummaryStrategy::Hierarchical { levels },
            messages_summarized: messages.len(),
            key_points: all_key_points,
            metadata: SummaryMetadata::default(),
        })
    }

    fn score_messages(&self, messages: &[Message]) -> Result<Vec<ScoredMessage>, SummaryError> {
        let mut scored = Vec::new();
        
        for (index, message) in messages.iter().enumerate() {
            let token_count = self.counter.count_tokens(&message.content, None)?;
            let importance = self.compute_importance(message, index, messages.len());
            
            scored.push(ScoredMessage {
                message: message.clone(),
                token_count,
                importance_score: importance,
                index,
            });
        }
        
        // Sort by importance descending
        scored.sort_by(|a, b| b.importance_score.partial_cmp(&a.importance_score).unwrap());
        
        Ok(scored)
    }

    fn compute_importance(&self, message: &Message, index: usize, total: usize) -> f32 {
        let mut score = 0.5;
        
        // Role-based
        score += match message.role {
            Role::System => 0.3,
            Role::User => 0.2,
            Role::Assistant => 0.15,
            Role::Tool => 0.1,
            Role::Function => 0.1,
        };
        
        // Recency bonus
        let recency = index as f32 / total.max(1) as f32;
        score += recency * 0.2;
        
        // Content-based
        let content = &message.content;
        let len = content.len() as f32;
        
        if len > 100.0 { score += 0.1; }
        if len > 500.0 { score += 0.1; }
        
        // Code blocks
        if content.contains("```") { score += 0.2; }
        
        // Errors
        if content.to_lowercase().contains("error") || content.to_lowercase().contains("exception") {
            score += 0.25;
        }
        
        // Decisions/keywords
        let decision_keywords = ["decide", "decided", "choose", "chosen", "will use", "going to", "approach", "strategy"];
        for kw in decision_keywords {
            if content.to_lowercase().contains(kw) {
                score += 0.15;
                break;
            }
        }
        
        // Questions from user
        if message.role == Role::User && content.contains('?') {
            score += 0.1;
        }
        
        // Tool calls/results
        if message.tool_calls.is_some() {
            score += 0.2;
        }
        
        score.min(1.0)
    }

    fn categorize_message(&self, message: &Message) -> KeyPointCategory {
        let content = message.content.to_lowercase();
        
        if message.tool_calls.is_some() {
            return KeyPointCategory::Code;
        }
        
        if content.contains("error") || content.contains("exception") || content.contains("failed") {
            return KeyPointCategory::Error;
        }
        
        if content.contains("```") {
            return KeyPointCategory::Code;
        }
        
        let decision_words = ["decide", "decided", "choose", "chosen", "will use", "going to", "approach"];
        if decision_words.iter().any(|w| content.contains(w)) {
            return KeyPointCategory::Decision;
        }
        
        if message.role == Role::User && content.contains('?') {
            return KeyPointCategory::Question;
        }
        
        if message.role == Role::Assistant && content.len() > 200 {
            return KeyPointCategory::Answer;
        }
        
        KeyPointCategory::Context
    }

    fn is_error_message(&self, message: &Message) -> bool {
        let content = message.content.to_lowercase();
        content.contains("error") || content.contains("exception") || content.contains("traceback") || content.contains("failed")
    }

    fn compress_text(&self, text: &str, max_tokens: usize) -> Result<String, SummaryError> {
        // Simple compression: truncate to token limit
        let tokens = self.counter.count_tokens(text, None)?;
        if tokens <= max_tokens {
            return Ok(text.to_string());
        }
        
        // Rough character estimate
        let chars_per_token = text.len() as f32 / tokens as f32;
        let target_chars = (max_tokens as f32 * chars_per_token) as usize;
        
        let truncated = self.truncate(text, target_chars);
        Ok(format!("{}... [truncated]", truncated))
    }

    fn truncate(&self, text: &str, max_chars: usize) -> String {
        if text.len() <= max_chars {
            return text.to_string();
        }
        
        // Try to truncate at sentence boundary
        let truncated = &text[..max_chars.min(text.len())];
        if let Some(last_period) = truncated.rfind('.') {
            if last_period > max_chars / 2 {
                return text[..last_period + 1].to_string();
            }
        }
        
        truncated.to_string()
    }

    /// Create a rolling summary that can be updated incrementally
    pub fn create_rolling_summary(&self, messages: &[Message], window_size: usize) -> Result<RollingSummary, SummaryError> {
        let config = SummaryConfig {
            strategy: SummaryStrategy::Rolling { window_size },
            ..self.config.clone()
        };
        
        let result = self.summarize(messages, &config)?;
        
        Ok(RollingSummary {
            summary: result.summary,
            token_count: result.token_count,
            window_size,
            message_count: messages.len(),
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    /// Update a rolling summary with new messages
    pub fn update_rolling_summary(&self, rolling: &mut RollingSummary, new_messages: &[Message]) -> Result<(), SummaryError> {
        let combined = format!("{}\n\n{}", rolling.summary, 
            new_messages.iter().map(|m| format!("{}: {}", m.role.as_str(), m.content)).collect::<Vec<_>>().join("\n\n"));
        
        let token_count = self.counter.count_tokens(&combined, None)?;
        
        // If too large, re-summarize
        let config = SummaryConfig {
            strategy: SummaryStrategy::Rolling { window_size: rolling.window_size },
            max_summary_tokens: self.config.max_summary_tokens,
            ..Default::default()
        };
        
        if token_count > self.config.max_summary_tokens {
            let all_messages: Vec<Message> = vec![]; // Would need to track all messages
            let result = self.summarize(&all_messages, &config)?;
            rolling.summary = result.summary;
            rolling.token_count = result.token_count;
        } else {
            rolling.summary = combined;
            rolling.token_count = token_count;
        }
        
        rolling.message_count += new_messages.len();
        rolling.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredMessage {
    pub message: Message,
    pub token_count: usize,
    pub importance_score: f32,
    pub index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollingSummary {
    pub summary: String,
    pub token_count: usize,
    pub window_size: usize,
    pub message_count: usize,
    pub last_updated: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum SummaryError {
    #[error("Token counting error: {0}")]
    CounterError(#[from] crate::counter::CounterError),
    #[error("Too few messages to summarize (minimum: {min})")]
    TooFewMessages { min: usize },
    #[error("Summarization failed: {0}")]
    SummarizationFailed(String),
    #[error("Context selection error: {0}")]
    ContextError(#[from] crate::context::SelectionError),
}

impl Default for SummaryMetadata {
    fn default() -> Self {
        Self {
            processing_time_ms: 0,
            original_message_count: 0,
            messages_in_summary: 0,
            warnings: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counter::{TokenCounter, TokenCounterConfig, Message, Role};

    fn setup() -> ConversationSummarizer {
        let counter = Arc::new(TokenCounter::new(TokenCounterConfig::default()).unwrap());
        ConversationSummarizer::new(counter)
    }

    #[test]
    fn test_extractive_summarize() {
        let summarizer = setup();
        let messages = vec![
            Message { role: Role::User, content: "How do I write a binary search in Rust?".to_string(), name: None, tool_calls: None, tool_call_id: None },
            Message { role: Role::Assistant, content: "Here's a binary search implementation:\n```rust\nfn binary_search(arr: &[i32], target: i32) -> Option<usize> {\n    let mut left = 0;\n    let mut right = arr.len();\n    while left < right {\n        let mid = left + (right - left) / 2;\n        if arr[mid] == target {\n            return Some(mid);\n        } else if arr[mid] < target {\n            left = mid + 1;\n        } else {\n            right = mid;\n        }\n    }\n    None\n}\n```".to_string(), name: None, tool_calls: None, tool_call_id: None },
            Message { role: Role::User, content: "Thanks! Can you explain the time complexity?".to_string(), name: None, tool_calls: None, tool_call_id: None },
            Message { role: Role::Assistant, content: "Binary search has O(log n) time complexity because we halve the search space each iteration. This is much faster than linear search O(n) for large arrays.".to_string(), name: None, tool_calls: None, tool_call_id: None },
        ];
        
        let config = SummaryConfig {
            min_messages_to_summarize: 2,
            ..Default::default()
        };
        let result = summarizer.summarize(&messages, &config).unwrap();
        
        assert!(result.token_count > 0);
        assert!(result.token_count < result.original_token_count);
        assert!(!result.summary.is_empty());
        assert!(result.key_points.len() > 0);
    }

    #[test]
    fn test_rolling_summary() {
        let summarizer = setup();
        let messages: Vec<Message> = (0..20).map(|i| Message {
            role: if i % 2 == 0 { Role::User } else { Role::Assistant },
            content: format!("Message {}", i),
            name: None, tool_calls: None, tool_call_id: None,
        }).collect();
        
        let rolling = summarizer.create_rolling_summary(&messages, 5).unwrap();
        assert!(rolling.token_count > 0);
        assert_eq!(rolling.window_size, 5);
    }

    #[test]
    fn test_key_point_categorization() {
        let summarizer = setup();
        
        let error_msg = Message { role: Role::Assistant, content: "Error: connection refused".to_string(), name: None, tool_calls: None, tool_call_id: None };
        assert_eq!(summarizer.categorize_message(&error_msg), KeyPointCategory::Error);
        
        let code_msg = Message { role: Role::Assistant, content: "```rust\nfn main() {}\n```".to_string(), name: None, tool_calls: None, tool_call_id: None };
        assert_eq!(summarizer.categorize_message(&code_msg), KeyPointCategory::Code);
        
        let decision_msg = Message { role: Role::Assistant, content: "We decided to use Redis for caching".to_string(), name: None, tool_calls: None, tool_call_id: None };
        assert_eq!(summarizer.categorize_message(&decision_msg), KeyPointCategory::Decision);
    }
}