//! Token optimization orchestrator

use std::sync::Arc;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use crate::cache::{CacheConfig, TokenCache, TokenCountCache};
use crate::context::{ContextConfig, ContextSelector, SelectedContext, SelectionStrategy};
use crate::counter::{TokenCounter, TokenCounterConfig, Message, TokenCount, Role};
use crate::models::{ModelCapabilities, ModelRegistry, PricingTier};
use crate::prompt::{PromptOptimizer, PromptTemplate, OptimizedPrompt};
use crate::router::{ModelRouter, RoutingDecision, RoutingStrategy, TaskComplexity};
use crate::summarizer::{ConversationSummarizer, SummaryConfig, SummaryStrategy};

/// Configuration for the token optimizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    pub context_config: ContextConfig,
    pub cache_config: CacheConfig,
    pub enable_caching: bool,
    pub enable_summarization: bool,
    pub enable_prompt_optimization: bool,
    pub enable_model_routing: bool,
    pub default_model: String,
    pub max_optimization_time_ms: u64,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            context_config: ContextConfig::default(),
            cache_config: CacheConfig::default(),
            enable_caching: true,
            enable_summarization: true,
            enable_prompt_optimization: true,
            enable_model_routing: true,
            default_model: "gpt-4".to_string(),
            max_optimization_time_ms: 100,
        }
    }
}

/// Result of token optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub original_token_count: TokenCount,
    pub optimized_token_count: TokenCount,
    pub tokens_saved: usize,
    pub savings_percentage: f32,
    pub estimated_cost_savings_usd: Option<f64>,
    pub selected_context: Option<SelectedContext>,
    pub optimized_prompt: Option<OptimizedPrompt>,
    pub routing_decision: Option<RoutingDecision>,
    pub summary: Option<String>,
    pub metadata: OptimizationMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationMetadata {
    pub optimization_time_ms: u64,
    pub cache_hit: bool,
    pub strategies_applied: Vec<AppliedStrategy>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppliedStrategy {
    ContextSelection { strategy: SelectionStrategy, tokens_saved: usize },
    PromptOptimization { tokens_saved: usize },
    Summarization { tokens_saved: usize },
    ModelRouting { from_model: String, to_model: String, cost_savings_usd: f64 },
    Caching { cache_hit: bool },
}

/// Main token optimizer orchestrating all optimization strategies
pub struct TokenOptimizer {
    config: OptimizationConfig,
    counter: Arc<TokenCounter>,
    registry: Arc<ModelRegistry>,
    context_selector: ContextSelector,
    prompt_optimizer: PromptOptimizer,
    router: ModelRouter,
    summarizer: ConversationSummarizer,
    token_cache: TokenCountCache,
    general_cache: TokenCache,
}

impl TokenOptimizer {
    pub fn new(counter: Arc<TokenCounter>, registry: Arc<ModelRegistry>) -> Self {
        let config = OptimizationConfig::default();
        let context_selector = ContextSelector::new(config.context_config.clone(), counter.clone());
        let prompt_optimizer = PromptOptimizer::new(counter.clone());
        let router = ModelRouter::new(registry.clone());
        let summarizer = ConversationSummarizer::new(counter.clone());
        let token_cache = TokenCountCache::new(config.cache_config.clone());
        let general_cache = TokenCache::new(config.cache_config.clone());

        Self {
            config,
            counter,
            registry,
            context_selector,
            prompt_optimizer,
            router,
            summarizer,
            token_cache,
            general_cache,
        }
    }

    pub fn with_config(&mut self, config: OptimizationConfig) {
        self.context_selector = ContextSelector::new(config.context_config.clone(), self.counter.clone());
        self.config = config;
    }

    /// Optimize a conversation for token usage
    pub fn optimize_conversation(
        &self,
        messages: &[Message],
        query: Option<&str>,
        model: Option<&str>,
    ) -> Result<OptimizationResult, OptimizationError> {
        let start = Instant::now();
        let model_name = model.unwrap_or(&self.config.default_model);
        let model_caps = self.registry.get(model_name);

        // Get original token count
        let original_count = self.counter.count_conversation(messages, Some(model_name))?;

        // Check cache first
        let cache_key = format!("opt:{}:{}", model_name, 
            messages.iter().map(|m| m.content.as_str()).collect::<Vec<_>>().join("|"));
        let mut cache_hit = false;
        let mut strategies_applied = Vec::new();
        let mut warnings = Vec::new();

        if self.config.enable_caching {
            if let Some(cached) = self.general_cache.get::<OptimizationResult>(&crate::cache::CacheKey::new("optimization", &cache_key))? {
                let mut metadata = cached.metadata.clone();
                metadata.cache_hit = true;
                return Ok(OptimizationResult { metadata, ..cached });
            }
        }

        let mut current_messages = messages.to_vec();
        let mut current_count = original_count.clone();
        let mut total_saved = 0;

        // 1. Context Selection
        if current_count.total > self.config.context_config.max_tokens {
            let selected = self.context_selector.select(&current_messages, query, model_caps)?;
            let saved = current_count.total.saturating_sub(selected.token_count.total);
            total_saved += saved;
            strategies_applied.push(AppliedStrategy::ContextSelection {
                strategy: selected.strategy_used,
                tokens_saved: saved,
            });
            current_messages = selected.messages.clone();
            current_count = selected.token_count.clone();
        }

        // 2. Summarization for very long conversations
        if self.config.enable_summarization && current_messages.len() > 20 {
            let summary_config = SummaryConfig {
                strategy: SummaryStrategy::Extractive,
                max_summary_tokens: 500,
                ..Default::default()
            };
            if let Ok(summary) = self.summarizer.summarize(&current_messages, &summary_config) {
                if summary.token_count < current_count.total {
                    let saved = current_count.total - summary.token_count;
                    total_saved += saved;
                    strategies_applied.push(AppliedStrategy::Summarization { tokens_saved: saved });
                    // Replace with summarized version
                    current_messages = vec![
                        Message { role: Role::System, content: summary.summary, name: None, tool_calls: None, tool_call_id: None }
                    ];
                    current_count = self.counter.count_conversation(&current_messages, Some(model_name))?;
                }
            }
        }

        // 3. Prompt Optimization (if last message is user prompt)
        if self.config.enable_prompt_optimization {
            if let Some(last_msg) = current_messages.last() {
                if last_msg.role == Role::User && last_msg.content.len() > 200 {
                    let template = PromptTemplate::from_string(&last_msg.content);
                    if let Ok(optimized) = self.prompt_optimizer.optimize(&template, model_name) {
                        if optimized.token_count < self.counter.count_tokens(&last_msg.content, Some(model_name))? {
                            let saved = self.counter.count_tokens(&last_msg.content, Some(model_name))? - optimized.token_count;
                            total_saved += saved;
                            strategies_applied.push(AppliedStrategy::PromptOptimization { tokens_saved: saved });
                            
                            // Update last message
                            if let Some(msg) = current_messages.last_mut() {
                                msg.content = optimized.prompt.clone();
                            }
                            current_count = self.counter.count_conversation(&current_messages, Some(model_name))?;
                        }
                    }
                }
            }
        }

        // 4. Model Routing
        let routing_decision = if self.config.enable_model_routing {
            let complexity = self.analyze_complexity(&current_messages);
            let decision = self.router.route(model_name, complexity, &current_messages)?;
            
            if decision.selected_model != model_name {
                let from_model = self.registry.get(model_name);
                let to_model = self.registry.get(&decision.selected_model);
                
                let cost_savings = from_model.and_then(|f| to_model.and_then(|t| {
                    f.pricing.as_ref().and_then(|fp| t.pricing.as_ref().map(|tp| {
                        let original_cost = (current_count.prompt as f64 * fp.input_per_million / 1_000_000.0) +
                                          (current_count.completion as f64 * fp.output_per_million / 1_000_000.0);
                        let new_cost = (current_count.prompt as f64 * tp.input_per_million / 1_000_000.0) +
                                     (current_count.completion as f64 * tp.output_per_million / 1_000_000.0);
                        original_cost - new_cost
                    }))
                })).unwrap_or(0.0);
                
                strategies_applied.push(AppliedStrategy::ModelRouting {
                    from_model: model_name.to_string(),
                    to_model: decision.selected_model.clone(),
                    cost_savings_usd: cost_savings,
                });
            }
            Some(decision)
        } else {
            None
        };

        let optimized_count = self.counter.count_conversation(&current_messages, Some(model_name))?;
        let savings_percentage = if original_count.total > 0 {
            (total_saved as f32 / original_count.total as f32) * 100.0
        } else { 0.0 };

        let estimated_cost_savings = self.estimate_cost_savings(&original_count, &optimized_count, model_name);

        let result = OptimizationResult {
            original_token_count: original_count,
            optimized_token_count: optimized_count,
            tokens_saved: total_saved,
            savings_percentage,
            estimated_cost_savings_usd: estimated_cost_savings,
            selected_context: None, // Could include the SelectedContext here
            optimized_prompt: None,
            routing_decision,
            summary: None,
            metadata: OptimizationMetadata {
                optimization_time_ms: start.elapsed().as_millis() as u64,
                cache_hit,
                strategies_applied,
                warnings,
            },
        };

        // Cache the result
        if self.config.enable_caching {
            let _ = self.general_cache.put(
                &crate::cache::CacheKey::new("optimization", &cache_key),
                &result,
                vec!["optimization".to_string(), model_name.to_string()],
            );
        }

        Ok(result)
    }

    /// Optimize a single prompt
    pub fn optimize_prompt(&self, prompt: &str, model: Option<&str>) -> Result<OptimizedPrompt, OptimizationError> {
        let model_name = model.unwrap_or(&self.config.default_model);
        let template = PromptTemplate::from_string(prompt);
        self.prompt_optimizer.optimize(&template, model_name)
            .map_err(OptimizationError::PromptError)
    }

    /// Quick token count with caching
    pub fn count_tokens_cached(&self, text: &str, model: Option<&str>) -> Result<usize, OptimizationError> {
        let model_name = model.unwrap_or(&self.config.default_model);
        
        if let Some(count) = self.token_cache.get_count(text, model_name) {
            return Ok(count);
        }
        
        let count = self.counter.count_tokens(text, Some(model_name))?;
        self.token_cache.set_count(text, model_name, count)?;
        Ok(count)
    }

    fn analyze_complexity(&self, messages: &[Message]) -> TaskComplexity {
        let total_chars: usize = messages.iter().map(|m| m.content.len()).sum();
        let has_code = messages.iter().any(|m| m.content.contains("```"));
        let has_tools = messages.iter().any(|m| m.tool_calls.is_some());
        let user_msgs = messages.iter().filter(|m| m.role == Role::User).count();
        
        if total_chars > 10000 || has_tools || user_msgs > 5 {
            TaskComplexity::Complex
        } else if total_chars > 3000 || has_code || user_msgs > 2 {
            TaskComplexity::Moderate
        } else {
            TaskComplexity::Simple
        }
    }

    fn estimate_cost_savings(&self, original: &TokenCount, optimized: &TokenCount, model: &str) -> Option<f64> {
        let model_caps = self.registry.get(model)?;
        let pricing = model_caps.pricing.as_ref()?;
        
        let original_cost = (original.prompt as f64 * pricing.input_per_million / 1_000_000.0) +
                           (original.completion as f64 * pricing.output_per_million / 1_000_000.0);
        let optimized_cost = (optimized.prompt as f64 * pricing.input_per_million / 1_000_000.0) +
                            (optimized.completion as f64 * pricing.output_per_million / 1_000_000.0);
        
        Some(original_cost - optimized_cost)
    }

    pub fn config(&self) -> &OptimizationConfig {
        &self.config
    }

    pub fn counter(&self) -> &TokenCounter {
        &self.counter
    }

    pub fn registry(&self) -> &ModelRegistry {
        &self.registry
    }

    pub fn context_selector(&self) -> &ContextSelector {
        &self.context_selector
    }

    pub fn prompt_optimizer(&self) -> &PromptOptimizer {
        &self.prompt_optimizer
    }

    pub fn router(&self) -> &ModelRouter {
        &self.router
    }

    pub fn summarizer(&self) -> &ConversationSummarizer {
        &self.summarizer
    }

    pub fn cache(&self) -> &TokenCache {
        &self.general_cache
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OptimizationError {
    #[error("Counter error: {0}")]
    CounterError(#[from] crate::counter::CounterError),
    #[error("Context selection error: {0}")]
    ContextError(#[from] crate::context::SelectionError),
    #[error("Prompt optimization error: {0}")]
    PromptError(#[from] crate::prompt::PromptError),
    #[error("Routing error: {0}")]
    RoutingError(#[from] crate::router::RoutingError),
    #[error("Summarization error: {0}")]
    SummarizationError(#[from] crate::summarizer::SummaryError),
    #[error("Cache error: {0}")]
    CacheError(#[from] crate::cache::CacheError),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counter::{TokenCounter, TokenCounterConfig, Message, Role};
    use crate::models::ModelRegistry;

    fn setup() -> TokenOptimizer {
        let mut config = TokenCounterConfig::default();
        config.default_model = "gpt-4".to_string();
        let counter = Arc::new(TokenCounter::new(config).unwrap());
        let registry = Arc::new(ModelRegistry::new());
        TokenOptimizer::new(counter, registry)
    }

    #[test]
    fn test_optimize_simple_conversation() {
        let optimizer = setup();
        let messages = vec![
            Message { role: Role::User, content: "Hello".to_string(), name: None, tool_calls: None, tool_call_id: None },
            Message { role: Role::Assistant, content: "Hi there!".to_string(), name: None, tool_calls: None, tool_call_id: None },
        ];
        
        let result = optimizer.optimize_conversation(&messages, None, None).unwrap();
        assert!(result.optimized_token_count.total > 0);
    }

    #[test]
    fn test_optimize_long_conversation() {
        let optimizer = setup();
        let mut messages = Vec::new();
        for i in 0..30 {
            messages.push(Message {
                role: if i % 2 == 0 { Role::User } else { Role::Assistant },
                content: format!("Message number {} with some content to make it longer", i),
                name: None, tool_calls: None, tool_call_id: None,
            });
        }
        
        let result = optimizer.optimize_conversation(&messages, None, None).unwrap();
        assert!(result.tokens_saved >= 0);
    }

    #[test]
    fn test_cached_count() {
        let optimizer = setup();
        let count1 = optimizer.count_tokens_cached("hello world", Some("gpt-4")).unwrap();
        let count2 = optimizer.count_tokens_cached("hello world", Some("gpt-4")).unwrap();
        assert_eq!(count1, count2);
    }
}