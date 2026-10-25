//! Token counting module with support for multiple tokenizers

use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tiktoken_rs::{get_bpe_from_model, CoreBPE};
use once_cell::sync::Lazy;

/// Errors that can occur during token counting
#[derive(Error, Debug)]
pub enum CounterError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("Tokenizer error: {0}")]
    TokenizerError(String),
    #[error("Encoding error: {0}")]
    EncodingError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, CounterError>;

/// Configuration for token counting
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenCounterConfig {
    /// Default model to use for token counting
    pub default_model: String,
    /// Whether to cache token counts
    pub enable_cache: bool,
    /// Maximum cache size
    pub max_cache_size: usize,
    /// Custom model mappings (alias -> actual model name)
    pub model_aliases: HashMap<String, String>,
}

impl Default for TokenCounterConfig {
    fn default() -> Self {
        let mut aliases = HashMap::new();
        // OpenAI models
        aliases.insert("gpt-4".to_string(), "gpt-4".to_string());
        aliases.insert("gpt-4-turbo".to_string(), "gpt-4-turbo-preview".to_string());
        aliases.insert("gpt-3.5-turbo".to_string(), "gpt-3.5-turbo".to_string());
        aliases.insert("gpt-4o".to_string(), "gpt-4o".to_string());
        aliases.insert("gpt-4o-mini".to_string(), "gpt-4o-mini".to_string());
        aliases.insert("default".to_string(), "gpt-4".to_string());
        // Anthropic models (use cl100k_base as approximation)
        aliases.insert("claude-3-opus".to_string(), "gpt-4".to_string());
        aliases.insert("claude-3-sonnet".to_string(), "gpt-4".to_string());
        aliases.insert("claude-3-haiku".to_string(), "gpt-3.5-turbo".to_string());
        aliases.insert("claude-3.5-sonnet".to_string(), "gpt-4".to_string());
        // GitHub Copilot models
        aliases.insert("copilot".to_string(), "gpt-4".to_string());
        aliases.insert("copilot-chat".to_string(), "gpt-4".to_string());

        Self {
            default_model: "gpt-4".to_string(),
            enable_cache: true,
            max_cache_size: 10000,
            model_aliases: aliases,
        }
    }
}

/// Token count result with metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenCount {
    /// Total token count
    pub total: usize,
    /// Token count for prompt/input
    pub prompt: usize,
    /// Token count for completion/output
    pub completion: usize,
    /// Model used for counting
    pub model: String,
    /// Estimated cost in USD (if pricing available)
    pub estimated_cost_usd: Option<f64>,
}

/// Message role in a conversation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
    Function,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
            Role::Tool => "tool",
            Role::Function => "function",
        }
    }
}

/// A message in a conversation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub function: FunctionCall,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

/// Token counter with support for multiple models
pub struct TokenCounter {
    config: TokenCounterConfig,
    tokenizers: dashmap::DashMap<String, Arc<CoreBPE>>,
    cache: lru::LruCache<String, TokenCount>,
}

impl TokenCounter {
    /// Create a new token counter with the given configuration
    pub fn new(config: TokenCounterConfig) -> Result<Self> {
        Ok(Self {
            config,
            tokenizers: dashmap::DashMap::new(),
            cache: lru::LruCache::new(
                std::num::NonZeroUsize::new(10000).unwrap()
            ),
        })
    }

    /// Get or create a tokenizer for the given model
    fn get_tokenizer(&self, model: &str) -> Result<Arc<CoreBPE>> {
        let resolved_model = self.config.model_aliases.get(model).map(|s| s.as_str()).unwrap_or(model);
        
        if let Some(tokenizer) = self.tokenizers.get(resolved_model) {
            return Ok(tokenizer.clone());
        }

        let tokenizer = get_bpe_from_model(resolved_model)
            .map_err(|e| CounterError::TokenizerError(format!("Failed to load tokenizer for {}: {}", resolved_model, e)))?;
        
        let arc_tokenizer = Arc::new(tokenizer);
        self.tokenizers.insert(resolved_model.to_string(), arc_tokenizer.clone());
        Ok(arc_tokenizer)
    }

    /// Count tokens in a single string
    pub fn count_tokens(&self, text: &str, model: Option<&str>) -> Result<usize> {
        let model = model.unwrap_or(&self.config.default_model);
        let tokenizer = self.get_tokenizer(model)?;
        let tokens = tokenizer.encode_with_special_tokens(text);
        Ok(tokens.len())
    }

    /// Count tokens in a conversation (array of messages)
    pub fn count_conversation(&self, messages: &[Message], model: Option<&str>) -> Result<TokenCount> {
        let model = model.unwrap_or(&self.config.default_model);
        let tokenizer = self.get_tokenizer(model)?;
        
        let mut total = 0;
        let mut prompt = 0;
        let mut completion = 0;

        for msg in messages {
            let role_tokens = tokenizer.encode_with_special_tokens(msg.role.as_str()).len();
            let content_tokens = tokenizer.encode_with_special_tokens(&msg.content).len();
            let msg_tokens = role_tokens + content_tokens + 4; // overhead for message formatting
            
            total += msg_tokens;
            
            match msg.role {
                Role::System | Role::User | Role::Tool | Role::Function => {
                    prompt += msg_tokens;
                }
                Role::Assistant => {
                    completion += msg_tokens;
                }
            }
        }

        // Add priming tokens for assistant response
        total += 3;
        prompt += 3;

        Ok(TokenCount {
            total,
            prompt,
            completion,
            model: model.to_string(),
            estimated_cost_usd: None,
        })
    }

    /// Count tokens for a prompt template with variables
    pub fn count_template(&self, template: &str, variables: &HashMap<String, String>, model: Option<&str>) -> Result<usize> {
        let model = model.unwrap_or(&self.config.default_model);
        let mut rendered = template.to_string();
        
        for (key, value) in variables {
            rendered = rendered.replace(&format!("{{{{{}}}}}", key), value);
        }
        
        self.count_tokens(&rendered, Some(model))
    }

    /// Estimate cost based on token count and model pricing
    pub fn estimate_cost(&self, count: &TokenCount, pricing: Option<&crate::models::ModelPricing>) -> Option<f64> {
        pricing.map(|p| {
            (count.prompt as f64 * p.input_per_million / 1_000_000.0) +
            (count.completion as f64 * p.output_per_million / 1_000_000.0)
        })
    }

    /// Get the configuration
    pub fn config(&self) -> &TokenCounterConfig {
        &self.config
    }

    /// Clear the tokenizer cache
    pub fn clear_tokenizers(&self) {
        self.tokenizers.clear();
    }
}

/// Global default token counter instance
static DEFAULT_COUNTER: Lazy<Result<TokenCounter>> = Lazy::new(|| TokenCounter::new(TokenCounterConfig::default()));

/// Get the global default token counter
pub fn default_counter() -> Result<&'static TokenCounter> {
    DEFAULT_COUNTER.as_ref().map_err(|e| CounterError::ConfigError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_simple_text() {
        let counter = TokenCounter::new(TokenCounterConfig::default()).unwrap();
        let count = counter.count_tokens("Hello, world!", None).unwrap();
        assert!(count > 0);
    }

    #[test]
    fn test_count_conversation() {
        let counter = TokenCounter::new(TokenCounterConfig::default()).unwrap();
        let messages = vec![
            Message { role: Role::System, content: "You are a helpful assistant.".to_string(), name: None, tool_calls: None, tool_call_id: None },
            Message { role: Role::User, content: "Hello!".to_string(), name: None, tool_calls: None, tool_call_id: None },
            Message { role: Role::Assistant, content: "Hi there! How can I help?".to_string(), name: None, tool_calls: None, tool_call_id: None },
        ];
        let count = counter.count_conversation(&messages, None).unwrap();
        assert!(count.total > 0);
        assert!(count.prompt > 0);
        assert!(count.completion > 0);
    }

    #[test]
    fn test_model_aliases() {
        let counter = TokenCounter::new(TokenCounterConfig::default()).unwrap();
        // These should all work without error
        let _ = counter.count_tokens("test", Some("gpt-4")).unwrap();
        let _ = counter.count_tokens("test", Some("claude-3-sonnet")).unwrap();
        let _ = counter.count_tokens("test", Some("copilot")).unwrap();
    }
}