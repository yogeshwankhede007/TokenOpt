// NAPI-RS bindings for Node.js/TypeScript
#[cfg(feature = "napi")]
use napi_derive::napi;
#[cfg(feature = "napi")]
use napi::bindgen_prelude::*;
#[cfg(feature = "napi")]
use tokenopt_core::prelude::*;
#[cfg(feature = "napi")]
use std::sync::Arc;

#[cfg(feature = "napi")]
#[napi(object)]
pub struct JsTokenCounterConfig {
    pub default_model: String,
    pub enable_cache: bool,
    pub max_cache_size: u32,
    pub model_aliases: std::collections::HashMap<String, String>,
}

#[cfg(feature = "napi")]
impl Default for JsTokenCounterConfig {
    fn default() -> Self {
        let mut aliases = std::collections::HashMap::new();
        aliases.insert("gpt-4".to_string(), "gpt-4".to_string());
        aliases.insert("claude-3-sonnet".to_string(), "gpt-4".to_string());
        aliases.insert("copilot".to_string(), "gpt-4".to_string());
        Self {
            default_model: "gpt-4".to_string(),
            enable_cache: true,
            max_cache_size: 10000,
            model_aliases: aliases,
        }
    }
}

#[cfg(feature = "napi")]
impl From<JsTokenCounterConfig> for TokenCounterConfig {
    fn from(c: JsTokenCounterConfig) -> Self {
        TokenCounterConfig {
            default_model: c.default_model,
            enable_cache: c.enable_cache,
            max_cache_size: c.max_cache_size as usize,
            model_aliases: c.model_aliases,
        }
    }
}

#[cfg(feature = "napi")]
#[napi(object)]
pub struct JsTokenCount {
    pub total: u32,
    pub prompt: u32,
    pub completion: u32,
    pub model: String,
    pub estimated_cost_usd: Option<f64>,
}

#[cfg(feature = "napi")]
impl From<TokenCount> for JsTokenCount {
    fn from(c: TokenCount) -> Self {
        Self {
            total: c.total as u32,
            prompt: c.prompt as u32,
            completion: c.completion as u32,
            model: c.model,
            estimated_cost_usd: c.estimated_cost_usd,
        }
    }
}

#[cfg(feature = "napi")]
#[napi(object)]
pub struct JsMessage {
    pub role: String,
    pub content: String,
    pub name: Option<String>,
    pub tool_calls: Option<Vec<JsToolCall>>,
    pub tool_call_id: Option<String>,
}

#[cfg(feature = "napi")]
#[napi(object)]
pub struct JsToolCall {
    pub id: String,
    pub function: JsFunctionCall,
    pub r#type: String,
}

#[cfg(feature = "napi")]
#[napi(object)]
pub struct JsFunctionCall {
    pub name: String,
    pub arguments: String,
}

#[cfg(feature = "napi")]
impl From<JsMessage> for Message {
    fn from(m: JsMessage) -> Self {
        Message {
            role: match m.role.as_str() {
                "system" => Role::System,
                "user" => Role::User,
                "assistant" => Role::Assistant,
                "tool" => Role::Tool,
                "function" => Role::Function,
                _ => Role::User,
            },
            content: m.content,
            name: m.name,
            tool_calls: m.tool_calls.map(|calls| calls.into_iter().map(|c| c.into()).collect()),
            tool_call_id: m.tool_call_id,
        }
    }
}

#[cfg(feature = "napi")]
impl From<JsToolCall> for ToolCall {
    fn from(t: JsToolCall) -> Self {
        ToolCall {
            id: t.id,
            function: t.function.into(),
            r#type: t.r#type,
        }
    }
}

#[cfg(feature = "napi")]
impl From<JsFunctionCall> for FunctionCall {
    fn from(f: JsFunctionCall) -> Self {
        FunctionCall {
            name: f.name,
            arguments: f.arguments,
        }
    }
}

#[cfg(feature = "napi")]
#[napi]
pub struct JsTokenCounter {
    inner: Arc<TokenCounter>,
}

#[cfg(feature = "napi")]
#[napi]
impl JsTokenCounter {
    #[napi(constructor)]
    pub fn new(config: Option<JsTokenCounterConfig>) -> Result<Self> {
        let config = config.map(|c| c.into()).unwrap_or_default();
        let inner = Arc::new(TokenCounter::new(config)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?);
        Ok(Self { inner })
    }

    #[napi]
    pub fn count_tokens(&self, text: String, model: Option<String>) -> Result<u32> {
        self.inner.count_tokens(&text, model.as_deref())
            .map(|c| c as u32)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    #[napi]
    pub fn count_conversation(&self, messages: Vec<JsMessage>, model: Option<String>) -> Result<JsTokenCount> {
        let messages: Vec<Message> = messages.into_iter().map(|m| m.into()).collect();
        let count = self.inner.count_conversation(&messages, model.as_deref())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        Ok(count.into())
    }

    #[napi]
    pub fn count_template(&self, template: String, variables: std::collections::HashMap<String, String>, model: Option<String>) -> Result<u32> {
        self.inner.count_template(&template, &variables, model.as_deref())
            .map(|c| c as u32)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
    }

    #[napi]
    pub fn estimate_cost(&self, count: JsTokenCount, pricing: Option<JsModelPricing>) -> Result<Option<f64>> {
        let count: TokenCount = count.into();
        let pricing = pricing.map(|p| p.into());
        Ok(self.inner.estimate_cost(&count, pricing.as_ref()))
    }

    #[napi]
    pub fn clear_tokenizers(&self) {
        self.inner.clear_tokenizers();
    }
}

#[cfg(feature = "napi")]
impl From<JsTokenCount> for TokenCount {
    fn from(c: JsTokenCount) -> Self {
        TokenCount {
            total: c.total as usize,
            prompt: c.prompt as usize,
            completion: c.completion as usize,
            model: c.model,
            estimated_cost_usd: c.estimated_cost_usd,
        }
    }
}

#[cfg(feature = "napi")]
#[napi(object)]
pub struct JsModelPricing {
    pub input_per_million: f64,
    pub output_per_million: f64,
    pub cache_read_per_million: Option<f64>,
    pub cache_write_per_million: Option<f64>,
    pub currency: String,
}

#[cfg(feature = "napi")]
impl From<JsModelPricing> for ModelPricing {
    fn from(p: JsModelPricing) -> Self {
        ModelPricing {
            input_per_million: p.input_per_million,
            output_per_million: p.output_per_million,
            cache_read_per_million: p.cache_read_per_million,
            cache_write_per_million: p.cache_write_per_million,
            currency: p.currency,
        }
    }
}

// More NAPI bindings would go here for other components...
// (Optimizer, PromptOptimizer, ModelRouter, Summarizer, ContextSelector, Cache)