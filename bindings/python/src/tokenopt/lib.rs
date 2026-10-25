// Python bindings for TokenOpt using PyO3
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};
use tokenopt_core::prelude::*;
use std::sync::Arc;

/// Python-facing TokenCounter
#[pyclass]
struct PyTokenCounter {
    inner: Arc<TokenCounter>,
}

#[pymethods]
impl PyTokenCounter {
    #[new]
    #[pyo3(signature = (config = None))]
    fn new(config: Option<PyTokenCounterConfig>) -> PyResult<Self> {
        let config = config.map(|c| c.into()).unwrap_or_default();
        let inner = Arc::new(TokenCounter::new(config).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?);
        Ok(Self { inner })
    }

    fn count_tokens(&self, text: &str, model: Option<&str>) -> PyResult<usize> {
        self.inner.count_tokens(text, model).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn count_conversation(&self, messages: Vec<PyMessage>, model: Option<&str>) -> PyResult<PyTokenCount> {
        let messages: Vec<Message> = messages.into_iter().map(|m| m.into()).collect();
        let count = self.inner.count_conversation(&messages, model).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(count.into())
    }

    fn count_template(&self, template: &str, variables: std::collections::HashMap<String, String>, model: Option<&str>) -> PyResult<usize> {
        self.inner.count_template(template, &variables, model).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn estimate_cost(&self, count: PyTokenCount, pricing: Option<PyModelPricing>) -> PyResult<Option<f64>> {
        let count: TokenCount = count.into();
        let pricing = pricing.map(|p| p.into());
        Ok(self.inner.estimate_cost(&count, pricing.as_ref()))
    }

    fn clear_tokenizers(&self) {
        self.inner.clear_tokenizers();
    }
}

/// Python-facing TokenOptimizer
#[pyclass]
struct PyTokenOptimizer {
    inner: Arc<TokenOptimizer>,
}

#[pymethods]
impl PyTokenOptimizer {
    #[new]
    #[pyo3(signature = (config = None))]
    fn new(config: Option<PyOptimizationConfig>) -> PyResult<Self> {
        let counter = Arc::new(TokenCounter::new(TokenCounterConfig::default()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?);
        let registry = Arc::new(ModelRegistry::new());
        let optimizer = TokenOptimizer::new(counter, registry);
        let optimizer = if let Some(config) = config {
            optimizer.with_config(config.into())
        } else {
            optimizer
        };
        Ok(Self { inner: Arc::new(optimizer) })
    }

    fn optimize_conversation(&self, messages: Vec<PyMessage>, query: Option<String>, model: Option<String>) -> PyResult<PyOptimizationResult> {
        let messages: Vec<Message> = messages.into_iter().map(|m| m.into()).collect();
        let result = self.inner.optimize_conversation(&messages, query.as_deref(), model.as_deref()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(result.into())
    }

    fn optimize_prompt(&self, prompt: &str, model: Option<String>) -> PyResult<PyOptimizedPrompt> {
        let result = self.inner.optimize_prompt(prompt, model.as_deref()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(result.into())
    }

    fn count_tokens_cached(&self, text: &str, model: Option<String>) -> PyResult<usize> {
        self.inner.count_tokens_cached(text, model.as_deref()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
}

/// Python-facing PromptOptimizer
#[pyclass]
struct PyPromptOptimizer {
    inner: Arc<PromptOptimizer>,
}

#[pymethods]
impl PyPromptOptimizer {
    #[new]
    #[pyo3(signature = (config = None))]
    fn new(config: Option<PyPromptOptimizationConfig>) -> PyResult<Self> {
        let counter = Arc::new(TokenCounter::new(TokenCounterConfig::default()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?);
        let optimizer = PromptOptimizer::new(counter);
        let optimizer = if let Some(config) = config {
            optimizer.with_config(config.into())
        } else {
            optimizer
        };
        Ok(Self { inner: Arc::new(optimizer) })
    }

    fn optimize(&self, template: PyPromptTemplate, model: &str) -> PyResult<PyOptimizedPrompt> {
        let template: PromptTemplate = template.into();
        let result = self.inner.optimize(&template, model).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(result.into())
    }

    fn optimize_string(&self, prompt: &str, model: &str) -> PyResult<PyOptimizedPrompt> {
        let result = self.inner.optimize_string(prompt, model).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(result.into())
    }

    fn optimize_for_task(&self, task: PyPromptTask, context: &str, model: &str) -> PyResult<PyOptimizedPrompt> {
        let task: PromptTask = task.into();
        let result = self.inner.optimize_for_task(task, context, model).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(result.into())
    }
}

/// Python-facing ModelRouter
#[pyclass]
struct PyModelRouter {
    inner: Arc<ModelRouter>,
}

#[pymethods]
impl PyModelRouter {
    #[new]
    #[pyo3(signature = (strategy = None))]
    fn new(strategy: Option<PyRoutingStrategy>) -> PyResult<Self> {
        let registry = Arc::new(ModelRegistry::new());
        let router = ModelRouter::new(registry);
        let router = if let Some(strategy) = strategy {
            router.with_strategy(strategy.into())
        } else {
            router
        };
        Ok(Self { inner: Arc::new(router) })
    }

    fn route(&self, requested_model: &str, complexity: PyTaskComplexity, messages: Vec<PyMessage>) -> PyResult<PyRoutingDecision> {
        let messages: Vec<Message> = messages.into_iter().map(|m| m.into()).collect();
        let decision = self.inner.route(requested_model, complexity.into(), &messages).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(decision.into())
    }

    fn add_rule(&self, rule: PyRoutingRule) {
        self.inner.add_rule(rule.into());
    }

    fn record_performance(&self, model_id: &str, latency_ms: u64, success: bool) {
        self.inner.record_performance(model_id, latency_ms, success);
    }
}

/// Python-facing ConversationSummarizer
#[pyclass]
struct PyConversationSummarizer {
    inner: Arc<ConversationSummarizer>,
}

#[pymethods]
impl PyConversationSummarizer {
    #[new]
    #[pyo3(signature = (config = None))]
    fn new(config: Option<PySummaryConfig>) -> PyResult<Self> {
        let counter = Arc::new(TokenCounter::new(TokenCounterConfig::default()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?);
        let summarizer = ConversationSummarizer::new(counter);
        let summarizer = if let Some(config) = config {
            summarizer.with_config(config.into())
        } else {
            summarizer
        };
        Ok(Self { inner: Arc::new(summarizer) })
    }

    fn summarize(&self, messages: Vec<PyMessage>, config: PySummaryConfig) -> PyResult<PySummaryResult> {
        let messages: Vec<Message> = messages.into_iter().map(|m| m.into()).collect();
        let result = self.inner.summarize(&messages, &config.into()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(result.into())
    }

    fn create_rolling_summary(&self, messages: Vec<PyMessage>, window_size: usize) -> PyResult<PyRollingSummary> {
        let messages: Vec<Message> = messages.into_iter().map(|m| m.into()).collect();
        let result = self.inner.create_rolling_summary(&messages, window_size).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(result.into())
    }

    fn update_rolling_summary(&self, rolling: PyRollingSummary, new_messages: Vec<PyMessage>) -> PyResult<PyRollingSummary> {
        let mut rolling_inner: RollingSummary = rolling.into();
        let new_messages: Vec<Message> = new_messages.into_iter().map(|m| m.into()).collect();
        self.inner.update_rolling_summary(&mut rolling_inner, &new_messages).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(rolling_inner.into())
    }
}

/// Python-facing ContextSelector
#[pyclass]
struct PyContextSelector {
    inner: Arc<ContextSelector>,
}

#[pymethods]
impl PyContextSelector {
    #[new]
    fn new(config: PyContextConfig) -> PyResult<Self> {
        let counter = Arc::new(TokenCounter::new(TokenCounterConfig::default()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?);
        let selector = ContextSelector::new(config.into(), counter);
        Ok(Self { inner: Arc::new(selector) })
    }

    fn select(&self, messages: Vec<PyMessage>, query: Option<String>, model: Option<PyModelCapabilities>) -> PyResult<PySelectedContext> {
        let messages: Vec<Message> = messages.into_iter().map(|m| m.into()).collect();
        let model = model.map(|m| Arc::new(m.into()));
        let result = self.inner.select(&messages, query.as_deref(), model.as_deref()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(result.into())
    }
}

/// Python-facing TokenCache
#[pyclass]
struct PyTokenCache {
    inner: Arc<TokenCache>,
}

#[pymethods]
impl PyTokenCache {
    #[new]
    #[pyo3(signature = (config = None))]
    fn new(config: Option<PyCacheConfig>) -> PyResult<Self> {
        let config = config.map(|c| c.into()).unwrap_or_default();
        Ok(Self { inner: Arc::new(TokenCache::new(config)) })
    }

    fn put(&self, key: &str, namespace: &str, value: &PyAny, tags: Option<Vec<String>>) -> PyResult<()> {
        let serialized = serde_json::to_vec(value).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        self.inner.put(&crate::cache::CacheKey::new(namespace, key), &serialized, tags.unwrap_or_default()).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn get(&self, key: &str, namespace: &str) -> PyResult<Option<PyObject>> {
        let data: Option<Vec<u8>> = self.inner.get(&crate::cache::CacheKey::new(namespace, key)).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        if let Some(bytes) = data {
            Python::with_gil(|py| {
                let obj = serde_json::from_slice(&bytes).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
                Ok(Some(obj.into_py(py)))
            })
        } else {
            Ok(None)
        }
    }

    fn invalidate_by_tag(&self, tag: &str) -> usize {
        self.inner.invalidate_by_tag(tag)
    }

    fn invalidate_namespace(&self, namespace: &str) -> usize {
        self.inner.invalidate_namespace(namespace)
    }

    fn clear(&self) {
        self.inner.clear();
    }

    fn stats(&self) -> PyCacheStats {
        self.inner.stats().into()
    }

    fn hit_rate(&self) -> f64 {
        self.inner.hit_rate()
    }

    fn cleanup_expired(&self) -> usize {
        self.inner.cleanup_expired()
    }
}

// Data structures for Python interop

#[pyclass]
#[derive(Clone)]
struct PyTokenCounterConfig {
    #[pyo3(get, set)]
    default_model: String,
    #[pyo3(get, set)]
    enable_cache: bool,
    #[pyo3(get, set)]
    max_cache_size: usize,
    #[pyo3(get, set)]
    model_aliases: std::collections::HashMap<String, String>,
}

impl Default for PyTokenCounterConfig {
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

impl From<PyTokenCounterConfig> for TokenCounterConfig {
    fn from(c: PyTokenCounterConfig) -> Self {
        TokenCounterConfig {
            default_model: c.default_model,
            enable_cache: c.enable_cache,
            max_cache_size: c.max_cache_size,
            model_aliases: c.model_aliases,
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct PyTokenCount {
    #[pyo3(get, set)]
    total: usize,
    #[pyo3(get, set)]
    prompt: usize,
    #[pyo3(get, set)]
    completion: usize,
    #[pyo3(get, set)]
    model: String,
    #[pyo3(get, set)]
    estimated_cost_usd: Option<f64>,
}

impl From<TokenCount> for PyTokenCount {
    fn from(c: TokenCount) -> Self {
        Self {
            total: c.total,
            prompt: c.prompt,
            completion: c.completion,
            model: c.model,
            estimated_cost_usd: c.estimated_cost_usd,
        }
    }
}

impl From<PyTokenCount> for TokenCount {
    fn from(c: PyTokenCount) -> Self {
        TokenCount {
            total: c.total,
            prompt: c.prompt,
            completion: c.completion,
            model: c.model,
            estimated_cost_usd: c.estimated_cost_usd,
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct PyMessage {
    #[pyo3(get, set)]
    role: String,
    #[pyo3(get, set)]
    content: String,
    #[pyo3(get, set)]
    name: Option<String>,
    #[pyo3(get, set)]
    tool_calls: Option<Vec<PyToolCall>>,
    #[pyo3(get, set)]
    tool_call_id: Option<String>,
}

#[pyclass]
#[derive(Clone)]
struct PyToolCall {
    #[pyo3(get, set)]
    id: String,
    #[pyo3(get, set)]
    function: PyFunctionCall,
    #[pyo3(get, set)]
    type_: String,
}

#[pyclass]
#[derive(Clone)]
struct PyFunctionCall {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    arguments: String,
}

impl From<PyMessage> for Message {
    fn from(m: PyMessage) -> Self {
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

impl From<PyToolCall> for ToolCall {
    fn from(t: PyToolCall) -> Self {
        ToolCall {
            id: t.id,
            function: t.function.into(),
            r#type: t.type_,
        }
    }
}

impl From<PyFunctionCall> for FunctionCall {
    fn from(f: PyFunctionCall) -> Self {
        FunctionCall {
            name: f.name,
            arguments: f.arguments,
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct PyModelPricing {
    #[pyo3(get, set)]
    input_per_million: f64,
    #[pyo3(get, set)]
    output_per_million: f64,
    #[pyo3(get, set)]
    cache_read_per_million: Option<f64>,
    #[pyo3(get, set)]
    cache_write_per_million: Option<f64>,
    #[pyo3(get, set)]
    currency: String,
}

impl From<PyModelPricing> for ModelPricing {
    fn from(p: PyModelPricing) -> Self {
        ModelPricing {
            input_per_million: p.input_per_million,
            output_per_million: p.output_per_million,
            cache_read_per_million: p.cache_read_per_million,
            cache_write_per_million: p.cache_write_per_million,
            currency: p.currency,
        }
    }
}

// More Python-facing structs would go here...
// (Truncated for brevity - would include all the conversion types)

/// Module definition
#[pymodule]
fn tokenopt(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTokenCounter>()?;
    m.add_class::<PyTokenOptimizer>()?;
    m.add_class::<PyPromptOptimizer>()?;
    m.add_class::<PyModelRouter>()?;
    m.add_class::<PyConversationSummarizer>()?;
    m.add_class::<PyContextSelector>()?;
    m.add_class::<PyTokenCache>()?;
    m.add_class::<PyTokenCounterConfig>()?;
    m.add_class::<PyTokenCount>()?;
    m.add_class::<PyMessage>()?;
    m.add_class::<PyToolCall>()?;
    m.add_class::<PyFunctionCall>()?;
    m.add_class::<PyModelPricing>()?;
    Ok(())
}