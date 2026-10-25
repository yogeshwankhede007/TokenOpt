//! TokenOpt Core - Universal Token Optimization Library
//!
//! Provides token counting, context optimization, conversation summarization,
//! prompt engineering, and multi-model routing for LLM coding assistants.

pub mod cache;
pub mod context;
pub mod counter;
pub mod models;
pub mod optimizer;
pub mod prompt;
pub mod router;
pub mod summarizer;

#[cfg(feature = "napi")]
pub mod napi;

pub use cache::{CacheConfig, TokenCache};
pub use context::{ContextSelector, SelectionStrategy, SelectedContext};
pub use counter::{TokenCounter, TokenCounterConfig, Message, Role, ToolCall, FunctionCall, TokenCount};
pub use models::{ModelCapabilities, ModelRegistry, ModelPricing, ModelFeatures, PricingTier, ModelFeature};
pub use optimizer::{OptimizationConfig, OptimizationResult, TokenOptimizer, AppliedStrategy, OptimizationMetadata};
pub use prompt::{PromptOptimizer, PromptTemplate, OptimizedPrompt, PromptOptimizationConfig, AggressivenessLevel, CustomRule, PromptTask, OptimizationTechnique, TemplateMetadata};
pub use router::{ModelRouter, RoutingDecision, RoutingStrategy, TaskComplexity, RoutingRule, RoutingCondition, RoutingAction};
pub use summarizer::{SummaryConfig, SummaryStrategy, ConversationSummarizer, SummaryResult, KeyPoint, KeyPointCategory, SummaryMetadata, RollingSummary};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::cache::{CacheConfig, TokenCache};
    pub use crate::context::{ContextSelector, SelectionStrategy, SelectedContext};
    pub use crate::counter::{TokenCounter, TokenCounterConfig, Message, Role, TokenCount};
    pub use crate::models::{ModelCapabilities, ModelRegistry, ModelPricing, PricingTier, ModelFeature};
    pub use crate::optimizer::{OptimizationConfig, OptimizationResult, TokenOptimizer};
    pub use crate::prompt::{PromptOptimizer, PromptTemplate, OptimizedPrompt};
    pub use crate::router::{ModelRouter, RoutingDecision, RoutingStrategy, TaskComplexity};
    pub use crate::summarizer::{SummaryConfig, SummaryStrategy, ConversationSummarizer};
    pub use crate::{TokenOpt, TokenOptBuilder};
}

use std::sync::Arc;

/// Main entry point for the TokenOpt library
pub struct TokenOpt {
    counter: Arc<TokenCounter>,
    optimizer: Arc<TokenOptimizer>,
    prompt_optimizer: Arc<PromptOptimizer>,
    router: Arc<ModelRouter>,
    summarizer: Arc<ConversationSummarizer>,
    cache: Arc<TokenCache>,
    registry: Arc<ModelRegistry>,
}

impl TokenOpt {
    /// Create a new TokenOpt instance with default configuration
    pub fn new() -> Result<Self, crate::counter::CounterError> {
        let counter = Arc::new(TokenCounter::new(TokenCounterConfig::default())?);
        let registry = Arc::new(ModelRegistry::default());
        let optimizer = Arc::new(TokenOptimizer::new(counter.clone(), registry.clone()));
        let prompt_optimizer = Arc::new(PromptOptimizer::new(counter.clone()));
        let router = Arc::new(ModelRouter::new(registry.clone()));
        let summarizer = Arc::new(ConversationSummarizer::new(counter.clone()));
        let cache = Arc::new(TokenCache::new(CacheConfig::default()));

        Ok(Self {
            counter,
            optimizer,
            prompt_optimizer,
            router,
            summarizer,
            cache,
            registry,
        })
    }

    /// Get the token counter (reference)
    pub fn counter(&self) -> &TokenCounter {
        &self.counter
    }

    /// Get the token counter (Arc clone)
    pub fn counter_arc(&self) -> Arc<TokenCounter> {
        self.counter.clone()
    }

    /// Get the token optimizer
    pub fn optimizer(&self) -> &TokenOptimizer {
        &self.optimizer
    }

    /// Get the prompt optimizer
    pub fn prompt_optimizer(&self) -> &PromptOptimizer {
        &self.prompt_optimizer
    }

    /// Get the model router
    pub fn router(&self) -> &ModelRouter {
        &self.router
    }

    /// Get the conversation summarizer
    pub fn summarizer(&self) -> &ConversationSummarizer {
        &self.summarizer
    }

    /// Get the token cache
    pub fn cache(&self) -> &TokenCache {
        &self.cache
    }

    /// Get the model registry (reference)
    pub fn registry(&self) -> &ModelRegistry {
        &self.registry
    }

    /// Get the model registry (Arc clone)
    pub fn registry_arc(&self) -> Arc<ModelRegistry> {
        self.registry.clone()
    }
}

impl Default for TokenOpt {
    fn default() -> Self {
        Self::new().expect("Failed to create default TokenOpt")
    }
}

/// Builder for TokenOpt with custom configuration
pub struct TokenOptBuilder {
    counter_config: Option<TokenCounterConfig>,
    cache_config: Option<CacheConfig>,
    custom_models: Vec<ModelCapabilities>,
}

impl TokenOptBuilder {
    pub fn new() -> Self {
        Self {
            counter_config: None,
            cache_config: None,
            custom_models: Vec::new(),
        }
    }

    pub fn counter_config(mut self, config: TokenCounterConfig) -> Self {
        self.counter_config = Some(config);
        self
    }

    pub fn cache_config(mut self, config: CacheConfig) -> Self {
        self.cache_config = Some(config);
        self
    }

    pub fn add_model(mut self, model: ModelCapabilities) -> Self {
        self.custom_models.push(model);
        self
    }

    pub fn build(self) -> Result<TokenOpt, crate::counter::CounterError> {
        let counter_config = self.counter_config.unwrap_or_default();
        let counter = Arc::new(TokenCounter::new(counter_config)?);

        let mut registry = ModelRegistry::default();
        for model in self.custom_models {
            registry.register(model);
        }
        let registry = Arc::new(registry);

        let optimizer = Arc::new(TokenOptimizer::new(counter.clone(), registry.clone()));
        let prompt_optimizer = Arc::new(PromptOptimizer::new(counter.clone()));
        let router = Arc::new(ModelRouter::new(registry.clone()));
        let summarizer = Arc::new(ConversationSummarizer::new(counter.clone()));
        let cache = Arc::new(TokenCache::new(self.cache_config.unwrap_or_default()));

        Ok(TokenOpt {
            counter,
            optimizer,
            prompt_optimizer,
            router,
            summarizer,
            cache,
            registry,
        })
    }
}

impl Default for TokenOptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenopt_creation() {
        let opt = TokenOpt::new();
        assert!(opt.is_ok());
    }

    #[test]
    fn test_tokenopt_builder() {
        let opt = TokenOptBuilder::new().build();
        assert!(opt.is_ok());
    }
}