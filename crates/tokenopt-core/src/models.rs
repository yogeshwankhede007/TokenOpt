//! Model capabilities, registry, and pricing information

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

/// Model pricing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    /// Input tokens per million USD
    pub input_per_million: f64,
    /// Output tokens per million USD
    pub output_per_million: f64,
    /// Cache read tokens per million USD
    pub cache_read_per_million: Option<f64>,
    /// Cache write tokens per million USD
    pub cache_write_per_million: Option<f64>,
    /// Currency (default USD)
    pub currency: String,
}

impl ModelPricing {
    pub fn new(input: f64, output: f64) -> Self {
        Self {
            input_per_million: input,
            output_per_million: output,
            cache_read_per_million: None,
            cache_write_per_million: None,
            currency: "USD".to_string(),
        }
    }

    pub fn with_cache(mut self, read: f64, write: f64) -> Self {
        self.cache_read_per_million = Some(read);
        self.cache_write_per_million = Some(write);
        self
    }
}

/// Model context window and capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilities {
    /// Unique model identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Provider (openai, anthropic, google, cohere, etc.)
    pub provider: String,
    /// Maximum context window in tokens
    pub context_window: usize,
    /// Maximum output tokens
    pub max_output_tokens: usize,
    /// Pricing information
    pub pricing: Option<ModelPricing>,
    /// Supported features
    pub features: ModelFeatures,
    /// Model family/series
    pub family: String,
    /// Whether this is a "fast/cheap" model variant
    pub is_fast_variant: bool,
    /// Knowledge cutoff date (YYYY-MM-DD)
    pub knowledge_cutoff: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelFeatures {
    pub function_calling: bool,
    pub vision: bool,
    pub json_mode: bool,
    pub parallel_tool_calls: bool,
    pub streaming: bool,
    pub system_prompt: bool,
    pub logprobs: bool,
}

/// Pricing tier for model routing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PricingTier {
    Free,
    UltraCheap,
    Cheap,
    Standard,
    Premium,
    UltraPremium,
}

impl ModelCapabilities {
    /// Get the pricing tier for this model
    pub fn pricing_tier(&self) -> PricingTier {
        let Some(pricing) = &self.pricing else {
            return PricingTier::Standard;
        };
        
        let avg_cost = (pricing.input_per_million + pricing.output_per_million) / 2.0;
        
        match avg_cost {
            c if c == 0.0 => PricingTier::Free,
            c if c < 0.10 => PricingTier::UltraCheap,
            c if c < 0.50 => PricingTier::Cheap,
            c if c < 2.00 => PricingTier::Standard,
            c if c < 10.00 => PricingTier::Premium,
            _ => PricingTier::UltraPremium,
        }
    }

    /// Check if model supports a feature
    pub fn supports(&self, feature: ModelFeature) -> bool {
        match feature {
            ModelFeature::FunctionCalling => self.features.function_calling,
            ModelFeature::Vision => self.features.vision,
            ModelFeature::JsonMode => self.features.json_mode,
            ModelFeature::ParallelToolCalls => self.features.parallel_tool_calls,
            ModelFeature::Streaming => self.features.streaming,
            ModelFeature::SystemPrompt => self.features.system_prompt,
            ModelFeature::Logprobs => self.features.logprobs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelFeature {
    FunctionCalling,
    Vision,
    JsonMode,
    ParallelToolCalls,
    Streaming,
    SystemPrompt,
    Logprobs,
}

/// Registry of known models
pub struct ModelRegistry {
    models: HashMap<String, Arc<ModelCapabilities>>,
    by_provider: HashMap<String, Vec<String>>,
    by_family: HashMap<String, Vec<String>>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            models: HashMap::new(),
            by_provider: HashMap::new(),
            by_family: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }

    fn register_defaults(&mut self) {
        // OpenAI Models
        self.register(ModelCapabilities {
            id: "gpt-4o".to_string(),
            name: "GPT-4o".to_string(),
            provider: "openai".to_string(),
            context_window: 128_000,
            max_output_tokens: 16_384,
            pricing: Some(ModelPricing::new(2.50, 10.00).with_cache(1.25, 5.00)),
            features: ModelFeatures {
                function_calling: true,
                vision: true,
                json_mode: true,
                parallel_tool_calls: true,
                streaming: true,
                system_prompt: true,
                logprobs: true,
            },
            family: "gpt-4o".to_string(),
            is_fast_variant: true,
            knowledge_cutoff: Some("2024-10".to_string()),
        });

        self.register(ModelCapabilities {
            id: "gpt-4o-mini".to_string(),
            name: "GPT-4o Mini".to_string(),
            provider: "openai".to_string(),
            context_window: 128_000,
            max_output_tokens: 16_384,
            pricing: Some(ModelPricing::new(0.15, 0.60).with_cache(0.075, 0.30)),
            features: ModelFeatures {
                function_calling: true,
                vision: true,
                json_mode: true,
                parallel_tool_calls: true,
                streaming: true,
                system_prompt: true,
                logprobs: true,
            },
            family: "gpt-4o".to_string(),
            is_fast_variant: true,
            knowledge_cutoff: Some("2024-10".to_string()),
        });

        self.register(ModelCapabilities {
            id: "gpt-4-turbo".to_string(),
            name: "GPT-4 Turbo".to_string(),
            provider: "openai".to_string(),
            context_window: 128_000,
            max_output_tokens: 4_096,
            pricing: Some(ModelPricing::new(10.00, 30.00)),
            features: ModelFeatures {
                function_calling: true,
                vision: true,
                json_mode: true,
                parallel_tool_calls: true,
                streaming: true,
                system_prompt: true,
                logprobs: true,
            },
            family: "gpt-4".to_string(),
            is_fast_variant: false,
            knowledge_cutoff: Some("2023-12".to_string()),
        });

        self.register(ModelCapabilities {
            id: "gpt-3.5-turbo".to_string(),
            name: "GPT-3.5 Turbo".to_string(),
            provider: "openai".to_string(),
            context_window: 16_384,
            max_output_tokens: 4_096,
            pricing: Some(ModelPricing::new(0.50, 1.50)),
            features: ModelFeatures {
                function_calling: true,
                vision: false,
                json_mode: true,
                parallel_tool_calls: true,
                streaming: true,
                system_prompt: true,
                logprobs: true,
            },
            family: "gpt-3.5".to_string(),
            is_fast_variant: true,
            knowledge_cutoff: Some("2021-09".to_string()),
        });

        // Anthropic Models (using OpenAI tokenizer approximations)
        self.register(ModelCapabilities {
            id: "claude-3-5-sonnet-20241022".to_string(),
            name: "Claude 3.5 Sonnet".to_string(),
            provider: "anthropic".to_string(),
            context_window: 200_000,
            max_output_tokens: 8_192,
            pricing: Some(ModelPricing::new(3.00, 15.00).with_cache(0.30, 3.75)),
            features: ModelFeatures {
                function_calling: true,
                vision: true,
                json_mode: true,
                parallel_tool_calls: true,
                streaming: true,
                system_prompt: true,
                logprobs: false,
            },
            family: "claude-3.5".to_string(),
            is_fast_variant: false,
            knowledge_cutoff: Some("2024-04".to_string()),
        });

        self.register(ModelCapabilities {
            id: "claude-3-5-haiku-20241022".to_string(),
            name: "Claude 3.5 Haiku".to_string(),
            provider: "anthropic".to_string(),
            context_window: 200_000,
            max_output_tokens: 8_192,
            pricing: Some(ModelPricing::new(0.80, 4.00).with_cache(0.08, 1.00)),
            features: ModelFeatures {
                function_calling: true,
                vision: true,
                json_mode: true,
                parallel_tool_calls: true,
                streaming: true,
                system_prompt: true,
                logprobs: false,
            },
            family: "claude-3.5".to_string(),
            is_fast_variant: true,
            knowledge_cutoff: Some("2024-07".to_string()),
        });

        self.register(ModelCapabilities {
            id: "claude-3-opus-20240229".to_string(),
            name: "Claude 3 Opus".to_string(),
            provider: "anthropic".to_string(),
            context_window: 200_000,
            max_output_tokens: 4_096,
            pricing: Some(ModelPricing::new(15.00, 75.00).with_cache(1.50, 18.75)),
            features: ModelFeatures {
                function_calling: true,
                vision: true,
                json_mode: true,
                parallel_tool_calls: true,
                streaming: true,
                system_prompt: true,
                logprobs: false,
            },
            family: "claude-3".to_string(),
            is_fast_variant: false,
            knowledge_cutoff: Some("2023-08".to_string()),
        });

        // Google Models
        self.register(ModelCapabilities {
            id: "gemini-1.5-pro".to_string(),
            name: "Gemini 1.5 Pro".to_string(),
            provider: "google".to_string(),
            context_window: 2_000_000,
            max_output_tokens: 8_192,
            pricing: Some(ModelPricing::new(1.25, 5.00)),
            features: ModelFeatures {
                function_calling: true,
                vision: true,
                json_mode: true,
                parallel_tool_calls: true,
                streaming: true,
                system_prompt: true,
                logprobs: false,
            },
            family: "gemini-1.5".to_string(),
            is_fast_variant: false,
            knowledge_cutoff: Some("2024-05".to_string()),
        });

        self.register(ModelCapabilities {
            id: "gemini-1.5-flash".to_string(),
            name: "Gemini 1.5 Flash".to_string(),
            provider: "google".to_string(),
            context_window: 1_000_000,
            max_output_tokens: 8_192,
            pricing: Some(ModelPricing::new(0.075, 0.30)),
            features: ModelFeatures {
                function_calling: true,
                vision: true,
                json_mode: true,
                parallel_tool_calls: true,
                streaming: true,
                system_prompt: true,
                logprobs: false,
            },
            family: "gemini-1.5".to_string(),
            is_fast_variant: true,
            knowledge_cutoff: Some("2024-05".to_string()),
        });

        // Cohere Models
        self.register(ModelCapabilities {
            id: "command-r-plus".to_string(),
            name: "Command R+".to_string(),
            provider: "cohere".to_string(),
            context_window: 128_000,
            max_output_tokens: 4_096,
            pricing: Some(ModelPricing::new(2.50, 10.00)),
            features: ModelFeatures {
                function_calling: true,
                vision: false,
                json_mode: true,
                parallel_tool_calls: true,
                streaming: true,
                system_prompt: true,
                logprobs: false,
            },
            family: "command-r".to_string(),
            is_fast_variant: false,
            knowledge_cutoff: Some("2024-03".to_string()),
        });

        // GitHub Copilot / VS Code specific
        self.register(ModelCapabilities {
            id: "copilot-gpt-4".to_string(),
            name: "GitHub Copilot (GPT-4)".to_string(),
            provider: "github".to_string(),
            context_window: 128_000,
            max_output_tokens: 16_384,
            pricing: None, // Included in subscription
            features: ModelFeatures {
                function_calling: true,
                vision: true,
                json_mode: true,
                parallel_tool_calls: true,
                streaming: true,
                system_prompt: true,
                logprobs: true,
            },
            family: "copilot".to_string(),
            is_fast_variant: false,
            knowledge_cutoff: Some("2024-10".to_string()),
        });

        self.register(ModelCapabilities {
            id: "copilot-gpt-4o".to_string(),
            name: "GitHub Copilot (GPT-4o)".to_string(),
            provider: "github".to_string(),
            context_window: 128_000,
            max_output_tokens: 16_384,
            pricing: None,
            features: ModelFeatures {
                function_calling: true,
                vision: true,
                json_mode: true,
                parallel_tool_calls: true,
                streaming: true,
                system_prompt: true,
                logprobs: true,
            },
            family: "copilot".to_string(),
            is_fast_variant: true,
            knowledge_cutoff: Some("2024-10".to_string()),
        });
    }

    pub fn register(&mut self, model: ModelCapabilities) {
        let id = model.id.clone();
        let provider = model.provider.clone();
        let family = model.family.clone();
        
        self.models.insert(id.clone(), Arc::new(model));
        self.by_provider.entry(provider).or_default().push(id.clone());
        self.by_family.entry(family).or_default().push(id);
    }

    pub fn get(&self, id: &str) -> Option<Arc<ModelCapabilities>> {
        self.models.get(id).cloned()
    }

    pub fn get_by_provider(&self, provider: &str) -> Vec<Arc<ModelCapabilities>> {
        self.by_provider.get(provider)
            .map(|ids| ids.iter().filter_map(|id| self.models.get(id).cloned()).collect())
            .unwrap_or_default()
    }

    pub fn get_by_family(&self, family: &str) -> Vec<Arc<ModelCapabilities>> {
        self.by_family.get(family)
            .map(|ids| ids.iter().filter_map(|id| self.models.get(id).cloned()).collect())
            .unwrap_or_default()
    }

    pub fn all(&self) -> Vec<Arc<ModelCapabilities>> {
        self.models.values().cloned().collect()
    }

    pub fn get_cheapest(&self, required_features: &[ModelFeature]) -> Option<Arc<ModelCapabilities>> {
        self.models.values()
            .filter(|m| required_features.iter().all(|f| m.supports(*f)))
            .min_by_key(|m| m.pricing_tier())
            .cloned()
    }

    pub fn get_fastest(&self, required_features: &[ModelFeature]) -> Option<Arc<ModelCapabilities>> {
        self.models.values()
            .filter(|m| required_features.iter().all(|f| m.supports(*f)))
            .filter(|m| m.is_fast_variant)
            .min_by_key(|m| m.pricing_tier())
            .cloned()
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_defaults() {
        let registry = ModelRegistry::new();
        assert!(registry.get("gpt-4o").is_some());
        assert!(registry.get("claude-3-5-sonnet-20241022").is_some());
        assert!(registry.get("gemini-1.5-pro").is_some());
    }

    #[test]
    fn test_pricing_tier() {
        let registry = ModelRegistry::new();
        let gpt4o = registry.get("gpt-4o").unwrap();
        let gpt4o_mini = registry.get("gpt-4o-mini").unwrap();
        
        assert_eq!(gpt4o_mini.pricing_tier(), PricingTier::Cheap);
        assert_eq!(gpt4o.pricing_tier(), PricingTier::Standard);
    }

    #[test]
    fn test_feature_filtering() {
        let registry = ModelRegistry::new();
        let vision_models = registry.get_by_provider("openai");
        assert!(!vision_models.is_empty());
    }
}