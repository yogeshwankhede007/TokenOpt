# TokenOpt API Reference

Complete API documentation for all TokenOpt modules and language bindings.

## Table of Contents

1. [Rust Core API](#rust-core-api)
2. [TypeScript/JavaScript API](#typescriptjavascript-api)
3. [Python API](#python-api)
4. [CLI Reference](#cli-reference)
5. [Configuration](#configuration)
6. [Error Handling](#error-handling)

---

## Rust Core API

### TokenOpt (Main Entry Point)

```rust
use tokenopt_core::prelude::*;

/// Create with defaults
let tokenopt = TokenOpt::new()?;

/// Create with builder
let tokenopt = TokenOptBuilder::new()
    .counter_config(TokenCounterConfig { ... })
    .cache_config(CacheConfig { ... })
    .add_model(custom_model)
    .build()?;
```

### TokenCounter

```rust
pub struct TokenCounter {
    // Count tokens in text
    pub fn count_tokens(&self, text: &str, model: Option<&str>) -> Result<usize, CounterError>
    
    // Count tokens in conversation
    pub fn count_conversation(&self, messages: &[Message], model: Option<&str>) -> Result<TokenCount, CounterError>
    
    // Count tokens in template with variables
    pub fn count_template(&self, template: &str, variables: &HashMap<String, String>, model: Option<&str>) -> Result<usize, CounterError>
    
    // Estimate cost from token count
    pub fn estimate_cost(&self, count: &TokenCount, pricing: Option<&ModelPricing>) -> Option<f64>
    
    // Clear tokenizer cache
    pub fn clear_tokenizers(&self)
}
```

### TokenOptimizer

```rust
pub struct TokenOptimizer {
    // Full optimization pipeline
    pub fn optimize_conversation(
        &self,
        messages: &[Message],
        query: Option<&str>,
        model: Option<&str>
    ) -> Result<OptimizationResult, OptimizationError>
    
    // Optimize single prompt
    pub fn optimize_prompt(&self, prompt: &str, model: Option<&str>) -> Result<OptimizedPrompt, PromptError>
    
    // Cached token counting
    pub fn count_tokens_cached(&self, text: &str, model: Option<&str>) -> Result<usize, OptimizationError>
    
    // Update configuration
    pub fn with_config(&mut self, config: OptimizationConfig)
}
```

### PromptOptimizer

```rust
pub struct PromptOptimizer {
    // Optimize from template
    pub fn optimize(&self, template: &PromptTemplate, model: &str) -> Result<OptimizedPrompt, PromptError>
    
    // Optimize raw string
    pub fn optimize_string(&self, prompt: &str, model: &str) -> Result<OptimizedPrompt, PromptError>
    
    // Task-specific optimization
    pub fn optimize_for_task(&self, task: PromptTask, context: &str, model: &str) -> Result<OptimizedPrompt, PromptError>
    
    // Set aggressiveness
    pub fn set_aggressiveness(&self, level: AggressivenessLevel)
}
```

### ContextSelector

```rust
pub struct ContextSelector {
    // Select optimal context
    pub fn select(
        &self,
        messages: &[Message],
        query: Option<&str>,
        model: Option<Arc<ModelCapabilities>>
    ) -> Result<SelectedContext, SelectionError>
}
```

### ModelRouter

```rust
pub struct ModelRouter {
    // Route to best model
    pub fn route(
        &self,
        requested_model: &str,
        complexity: TaskComplexity,
        messages: &[Message]
    ) -> Result<RoutingDecision, RoutingError>
    
    // Add custom routing rule
    pub fn add_rule(&mut self, rule: RoutingRule)
    
    // Record performance for adaptive routing
    pub fn record_performance(&mut self, model_id: &str, latency_ms: u64, success: bool)
}
```

### ConversationSummarizer

```rust
pub struct ConversationSummarizer {
    // Summarize conversation
    pub fn summarize(
        &self,
        messages: &[Message],
        config: &SummaryConfig
    ) -> Result<SummaryResult, SummaryError>
    
    // Create rolling summary
    pub fn create_rolling_summary(
        &self,
        messages: &[Message],
        window_size: usize
    ) -> Result<RollingSummary, SummaryError>
    
    // Update rolling summary
    pub fn update_rolling_summary(
        &self,
        rolling: &mut RollingSummary,
        new_messages: &[Message]
    ) -> Result<(), SummaryError>
}
```

### TokenCache

```rust
pub struct TokenCache {
    // Store value
    pub fn put<T: Serialize>(&self, key: &CacheKey, value: &T, tags: Vec<String>) -> Result<(), CacheError>
    
    // Retrieve value
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &CacheKey) -> Result<Option<T>, CacheError>
    
    // Get or compute (memoization)
    pub fn get_or_compute<T, F>(&self, key: &CacheKey, compute: F) -> Result<T, CacheError>
    where T: Serialize + for<'de> Deserialize<'de> + Clone, F: FnOnce() -> Result<T, CacheError>
    
    // Invalidate by tag
    pub fn invalidate_by_tag(&self, tag: &str) -> usize
    
    // Invalidate namespace
    pub fn invalidate_namespace(&self, namespace: &str) -> usize
    
    // Clear all
    pub fn clear(&self)
    
    // Statistics
    pub fn stats(&self) -> CacheStats
    pub fn hit_rate(&self) -> f64
    
    // Cleanup expired
    pub fn cleanup_expired(&self) -> usize
}
```

### Data Types

```rust
// Message in conversation
pub struct Message {
    pub role: Role,
    pub content: String,
    pub name: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_id: Option<String>,
}

pub enum Role {
    System, User, Assistant, Tool, Function,
}

// Token count result
pub struct TokenCount {
    pub total: usize,
    pub prompt: usize,
    pub completion: usize,
    pub model: String,
    pub estimated_cost_usd: Option<f64>,
}

// Context selection result
pub struct SelectedContext {
    pub messages: Vec<Message>,
    pub token_count: TokenCount,
    pub strategy_used: SelectionStrategy,
    pub omitted_count: usize,
    pub omitted_tokens: usize,
    pub summary: Option<String>,
    pub metadata: ContextMetadata,
}

// Prompt optimization result
pub struct OptimizedPrompt {
    pub original: String,
    pub optimized: String,
    pub prompt: String,
    pub token_count: usize,
    pub original_token_count: usize,
    pub tokens_saved: usize,
    pub savings_percentage: f32,
    pub techniques_applied: Vec<OptimizationTechnique>,
    pub warnings: Vec<String>,
}

// Model routing decision
pub struct RoutingDecision {
    pub requested_model: String,
    pub selected_model: String,
    pub strategy: RoutingStrategy,
    pub complexity: TaskComplexity,
    pub reasoning: String,
    pub required_features: Vec<ModelFeature>,
    pub estimated_cost_usd: Option<f64>,
    pub fallback_models: Vec<String>,
    pub confidence: f32,
}

// Summarization result
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

// Full optimization result
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
```

---

## TypeScript/JavaScript API

### Installation

```bash
npm install @tokenopt/core
```

### Basic Usage

```typescript
import { 
    TokenOpt, 
    TokenCounter, 
    TokenOptimizer, 
    PromptOptimizer, 
    ModelRouter, 
    ConversationSummarizer, 
    ContextSelector,
    PromptTask,
    TaskComplexity,
    SelectionStrategy,
    RoutingStrategy,
    SummaryStrategy
} from '@tokenopt/core';

const tokenopt = new TokenOpt();
```

### TokenCounter

```typescript
const counter = new TokenCounter();

// Count tokens
const count = counter.countTokens("Hello world", "gpt-4");

// Count conversation
const convCount = counter.countConversation([
    { role: 'user', content: 'Hello' },
    { role: 'assistant', content: 'Hi!' }
], "gpt-4");

// Count template
const templateCount = counter.countTemplate("Hello {{name}}", { name: "World" }, "gpt-4");
```

### TokenOptimizer

```typescript
const optimizer = new TokenOptimizer({
    enableCaching: true,
    enableSummarization: true,
    enablePromptOptimization: true,
    enableModelRouting: true,
    defaultModel: 'gpt-4',
    maxOptimizationTimeMs: 100,
});

// Full optimization
const result = optimizer.optimizeConversation(messages, 'query', 'gpt-4');

// Optimize prompt
const optimized = optimizer.optimizePrompt('Please write a function...', 'gpt-4');

// Cached counting
const count = optimizer.countTokensCached('text', 'gpt-4');
```

### PromptOptimizer

```typescript
const promptOptimizer = new PromptOptimizer({
    aggressiveness: 'Balanced',
    preserveExamples: true,
    preserveFormatting: true,
});

// Optimize string
const result = promptOptimizer.optimizeString(prompt, 'gpt-4');

// Task-specific
const result = promptOptimizer.optimizeForTask(
    PromptTask.CodeGeneration,
    'Create REST API endpoint',
    'gpt-4'
);
```

### ModelRouter

```typescript
const router = new ModelRouter('Balanced');

const decision = router.route('gpt-4o', TaskComplexity.Simple, messages);

router.addRule({
    name: 'prefer-cheap-for-simple',
    condition: { TaskComplexity: 'Simple' },
    action: 'UseCheapest',
    priority: 10,
});

router.recordPerformance('gpt-4o', 150, true);
```

### ConversationSummarizer

```typescript
const summarizer = new ConversationSummarizer({
    strategy: 'Extractive',
    maxSummaryTokens: 1000,
    preserveCodeBlocks: true,
    preserveKeyDecisions: true,
    preserveErrors: true,
});

const summary = summarizer.summarize(messages, {
    strategy: 'Extractive',
    maxSummaryTokens: 500,
});

const rolling = summarizer.createRollingSummary(messages, 50);
const updated = summarizer.updateRollingSummary(rolling, newMessages);
```

### ContextSelector

```typescript
const selector = new ContextSelector({
    strategy: 'Balanced',
    maxTokens: 4000,
    reserveTokens: 500,
    minMessages: 2,
    compressOldMessages: true,
    similarityThreshold: 0.7,
});

const selected = selector.select(messages, 'query', modelCapabilities);
```

### TokenCache

```typescript
const cache = new TokenCache({
    maxEntries: 100000,
    ttl: 3600,
    persistToDisk: false,
});

cache.put('key', 'namespace', value, ['tag1', 'tag2']);
const value = cache.get('key', 'namespace');
cache.invalidateByTag('tag1');
cache.invalidateNamespace('namespace');
cache.clear();
const stats = cache.stats();
const hitRate = cache.hitRate();
```

---

## Python API

### Installation

```bash
pip install tokenopt
```

### Basic Usage

```python
from tokenopt import (
    TokenOpt, TokenCounter, TokenOptimizer,
    PromptOptimizer, ModelRouter, ConversationSummarizer,
    ContextSelector, PromptTask, TaskComplexity,
    SelectionStrategy, RoutingStrategy, SummaryStrategy
)

tokenopt = TokenOpt()
```

### TokenCounter

```python
counter = TokenCounter()

count = counter.count_tokens("Hello world", "gpt-4")

conv_count = counter.count_conversation([
    {"role": "user", "content": "Hello"},
    {"role": "assistant", "content": "Hi!"}
], "gpt-4")
```

### TokenOptimizer

```python
optimizer = TokenOptimizer({
    "enable_caching": True,
    "enable_summarization": True,
    "enable_prompt_optimization": True,
    "enable_model_routing": True,
    "default_model": "gpt-4",
    "max_optimization_time_ms": 100,
})

result = optimizer.optimize_conversation(messages, model="gpt-4")
optimized = optimizer.optimize_prompt("Please write...", "gpt-4")
count = optimizer.count_tokens_cached("text", "gpt-4")
```

### PromptOptimizer

```python
prompt_optimizer = PromptOptimizer({
    "aggressiveness": "Balanced",
    "preserve_examples": True,
    "preserve_formatting": True,
})

result = prompt_optimizer.optimize_string(prompt, "gpt-4")

result = prompt_optimizer.optimize_for_task(
    PromptTask.CodeGeneration,
    "Create REST API endpoint",
    "gpt-4"
)
```

### ModelRouter

```python
router = ModelRouter(RoutingStrategy.Balanced)

decision = router.route("gpt-4o", TaskComplexity.Simple, messages)

router.add_rule({
    "name": "prefer-cheap-for-simple",
    "condition": {"TaskComplexity": "Simple"},
    "action": "UseCheapest",
    "priority": 10,
})

router.record_performance("gpt-4o", 150, True)
```

### ConversationSummarizer

```python
summarizer = ConversationSummarizer({
    "strategy": SummaryStrategy.Extractive,
    "max_summary_tokens": 1000,
    "preserve_code_blocks": True,
    "preserve_key_decisions": True,
    "preserve_errors": True,
})

summary = summarizer.summarize(messages, {
    "strategy": SummaryStrategy.Extractive,
    "max_summary_tokens": 500,
})

rolling = summarizer.create_rolling_summary(messages, 50)
updated = summarizer.update_rolling_summary(rolling, new_messages)
```

### ContextSelector

```python
selector = ContextSelector(
    strategy=SelectionStrategy.Balanced,
    max_tokens=4000,
    reserve_tokens=500,
    min_messages=2,
    compress_old_messages=True,
)

selected = selector.select(messages, query="binary search")
```

---

## CLI Reference

### Installation

```bash
cargo install tokenopt-cli
# or download binary from releases
```

### Commands

```bash
# Count tokens
tokenopt count [OPTIONS]
  --text TEXT           Text to count
  --file FILE           File with text/conversation
  --model MODEL         Model (default: gpt-4)
  --conversation        Input is JSON conversation

# Optimize conversation
tokenopt optimize [OPTIONS]
  --file FILE           Conversation file (JSON)
  --model MODEL         Target model
  --query QUERY         Query for relevance
  --strategy STRATEGY   Selection strategy
  --output FILE         Output file

# Optimize prompt
tokenopt prompt [OPTIONS]
  --text TEXT           Prompt text
  --file FILE           Prompt file
  --model MODEL         Target model
  --aggressiveness LEVEL conservative|balanced|aggressive

# Route model
tokenopt route [OPTIONS]
  --model MODEL         Requested model
  --file FILE           Conversation file
  --complexity LEVEL    simple|moderate|complex
  --strategy STRATEGY   fixed|cost-optimized|speed-optimized|balanced

# Summarize
tokenopt summarize [OPTIONS]
  --file FILE           Conversation file
  --strategy STRATEGY   extractive|abstractive|hybrid
  --max-tokens INT      Max summary tokens

# List models
tokenopt models [OPTIONS]
  --provider PROVIDER   Filter by provider
  --tier TIER           Filter by pricing tier

# Estimate cost
tokenopt cost [OPTIONS]
  --model MODEL         Model name
  --prompt-tokens INT   Prompt tokens
  --completion-tokens INT Completion tokens

# Cache management
tokenopt cache [OPTIONS]
  --clear               Clear cache
```

### Examples

```bash
# Count tokens in file
tokenopt count --file conversation.json --conversation

# Optimize conversation
tokenopt optimize --file chat.json --model gpt-4o --output optimized.json

# Optimize prompt
tokenopt prompt --text "Please explain..." --model gpt-4 --aggressiveness balanced

# Route to cheaper model
tokenopt route --model gpt-4o --complexity simple --strategy cost-optimized

# Summarize long conversation
tokenopt summarize --file long_chat.json --strategy extractive --max-tokens 1000

# List cheap OpenAI models
tokenopt models --provider openai --tier cheap

# Estimate cost
tokenopt cost --model gpt-4o --prompt-tokens 1000 --completion-tokens 500
```

---

## Configuration

### TOML Configuration (tokenopt.toml)

```toml
[counter]
default_model = "gpt-4o"
enable_cache = true
max_cache_size = 10000

[optimizer]
context_config = { 
    strategy = "Balanced", 
    max_tokens = 8000,
    reserve_tokens = 1000,
    min_messages = 2,
    compress_old_messages = true,
}
enable_caching = true
enable_summarization = true
enable_prompt_optimization = true
enable_model_routing = true
default_model = "gpt-4o"
max_optimization_time_ms = 100

[cache]
max_entries = 100000
ttl_seconds = 3600
persist_to_disk = false
cache_dir = "/tmp/tokenopt"
key_prefix = "tokenopt"

[router]
default_strategy = "Balanced"

[summarizer]
default_strategy = "Extractive"
max_summary_tokens = 1000
preserve_code_blocks = true
preserve_key_decisions = true
preserve_errors = true

[prompt]
default_aggressiveness = "Balanced"
preserve_examples = true
preserve_formatting = true
```

### Programmatic Configuration

```rust
let config = OptimizationConfig {
    context_config: ContextConfig {
        strategy: SelectionStrategy::Balanced,
        max_tokens: 8000,
        reserve_tokens: 1000,
        min_messages: 2,
        ..Default::default()
    },
    cache_config: CacheConfig {
        max_entries: 100000,
        ttl: Duration::from_secs(3600),
        ..Default::default()
    },
    enable_caching: true,
    enable_summarization: true,
    enable_prompt_optimization: true,
    enable_model_routing: true,
    default_model: "gpt-4o".to_string(),
    max_optimization_time_ms: 100,
};
```

---

## Error Handling

### Rust Errors

```rust
// Token counting errors
pub enum CounterError {
    ModelNotFound(String),
    TokenizerError(String),
    EncodingError(String),
    ConfigError(String),
}

// Context selection errors
pub enum SelectionError {
    CounterError(CounterError),
    EmbeddingError(String),
    ConfigError(String),
}

// Prompt optimization errors
pub enum PromptError {
    CounterError(CounterError),
    RegexError(regex::Error),
    OptimizationFailed(String),
}

// Routing errors
pub enum RoutingError {
    ModelNotFound(String),
    NoSuitableModel,
    RuleError(String),
}

// Summarization errors
pub enum SummaryError {
    CounterError(CounterError),
    TooFewMessages { min: usize },
    SummarizationFailed(String),
    ContextError(SelectionError),
}

// Optimization errors
pub enum OptimizationError {
    CounterError(CounterError),
    ContextError(SelectionError),
    PromptError(PromptError),
    RoutingError(RoutingError),
    SummarizationError(SummaryError),
    CacheError(CacheError),
    ConfigError(String),
}

// Cache errors
pub enum CacheError {
    SerializationError(String),
    DeserializationError(String),
    IoError(std::io::Error),
    KeyNotFound,
}
```

### TypeScript Errors

All methods return `Result<T, Error>` or throw `Error` with structured message:

```typescript
try {
    const result = optimizer.optimizeConversation(messages, 'query', 'gpt-4');
} catch (error) {
    if (error.code === 'MODEL_NOT_FOUND') {
        // Handle missing model
    } else if (error.code === 'TOKENIZER_ERROR') {
        // Handle tokenizer issue
    }
}
```

### Python Errors

```python
from tokenopt import (
    CounterError, SelectionError, PromptError,
    RoutingError, SummaryError, OptimizationError, CacheError
)

try:
    result = optimizer.optimize_conversation(messages, model="gpt-4")
except CounterError as e:
    print(f"Token counting error: {e}")
except OptimizationError as e:
    print(f"Optimization failed: {e}")
```

---

## Version Compatibility

| Rust | TypeScript | Python | Protocol |
|------|------------|--------|----------|
| 0.1.x | 0.1.x | 0.1.x | v1 |

All bindings for a given version are guaranteed to be compatible.

---

## Migration Guide

### From 0.0.x to 0.1.x

- `TokenOpt::default()` → `TokenOpt::new()?`
- `OptimizationConfig::default()` now uses `"gpt-4o"` as default model
- `SelectionStrategy::Weighted` now uses struct syntax
- `SummaryStrategy::Rolling` now uses struct syntax