# User Guide

Comprehensive guide for using TokenOpt in your projects.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Core Concepts](#core-concepts)
3. [Token Counting](#token-counting)
4. [Context Optimization](#context-optimization)
5. [Prompt Optimization](#prompt-optimization)
6. [Model Routing](#model-routing)
7. [Conversation Summarization](#conversation-summarization)
8. [Caching](#caching)
9. [Cost Estimation](#cost-estimation)
10. [Advanced Usage](#advanced-usage)
11. [Best Practices](#best-practices)
12. [Troubleshooting](#troubleshooting)

---

## Quick Start

### Rust

```bash
# Add to Cargo.toml
[dependencies]
tokenopt-core = { version = "0.1", features = ["full"] }
```

```rust
use tokenopt_core::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokenopt = TokenOpt::new()?;
    
    // Count tokens
    let count = tokenopt.counter().count_tokens("Hello world!", Some("gpt-4"))?;
    println!("Tokens: {}", count);
    
    // Optimize conversation
    let messages = vec![
        Message { role: Role::User, content: "Write a binary search in Rust".to_string(), ..Default::default() },
        Message { role: Role::Assistant, content: "Here's the implementation...".to_string(), ..Default::default() },
    ];
    
    let result = tokenopt.optimizer().optimize_conversation(&messages, None, Some("gpt-4"))?;
    println!("Saved {} tokens ({:.1}%)", result.tokens_saved, result.savings_percentage);
    
    Ok(())
}
```

### TypeScript

```bash
npm install @tokenopt/core
```

```typescript
import { TokenOpt, PromptTask, TaskComplexity } from '@tokenopt/core';

const tokenopt = new TokenOpt();

// Count tokens
const count = tokenopt.counter.countTokens("Hello world!", "gpt-4");
console.log(`Tokens: ${count}`);

// Optimize prompt for code generation
const optimized = tokenopt.promptOptimizer.optimizeForTask(
    PromptTask.CodeGeneration,
    "Create a REST API endpoint with authentication",
    "gpt-4"
);
console.log(`Saved ${optimized.tokensSaved} tokens`);
console.log(optimized.prompt);
```

### Python

```bash
pip install tokenopt
```

```python
from tokenopt import TokenOpt, PromptTask, TaskComplexity

tokenopt = TokenOpt()

# Count tokens
count = tokenopt.counter.count_tokens("Hello world!", "gpt-4")
print(f"Tokens: {count}")

# Optimize conversation
messages = [
    {"role": "user", "content": "Write a binary search in Rust"},
    {"role": "assistant", "content": "Here's the implementation..."},
]
result = tokenopt.optimizer.optimize_conversation(messages, model="gpt-4")
print(f"Saved {result.tokens_saved} tokens ({result.savings_percentage:.1f}%)")
```

### CLI

```bash
# Count tokens
tokenopt count --text "Hello world!" --model gpt-4

# Optimize conversation
tokenopt optimize --file conversation.json --model gpt-4 --output optimized.json

# Optimize prompt
tokenopt prompt --text "Please write a function..." --model gpt-4 --aggressiveness balanced
```

---

## Core Concepts

### Tokens

Tokens are the basic units that LLMs process. Understanding token usage is crucial for:
- **Cost control** - Providers charge per token
- **Context limits** - Models have maximum context windows
- **Performance** - Fewer tokens = faster responses

### Token Budget

Every conversation has a token budget:
```
Total Context Window = Prompt Tokens + Completion Tokens + Reserve
```

TokenOpt helps you stay within budget while preserving quality.

### Optimization Strategies

TokenOpt applies multiple strategies:
1. **Context Selection** - Keep most relevant messages
2. **Prompt Compression** - Reduce prompt size
3. **Summarization** - Compress long conversations
4. **Model Routing** - Use cheaper models for simple tasks
5. **Caching** - Avoid redundant computations

---

## Token Counting

### Basic Counting

```rust
let count = tokenopt.counter().count_tokens("Hello world!", Some("gpt-4"))?;
// Returns: 3 (for "Hello world!")
```

### Conversation Counting

```rust
let messages = vec![
    Message { role: Role::System, content: "You are a helpful assistant.".to_string(), ..Default::default() },
    Message { role: Role::User, content: "Hello!".to_string(), ..Default::default() },
    Message { role: Role::Assistant, content: "Hi there!".to_string(), ..Default::default() },
];

let count = tokenopt.counter().count_conversation(&messages, Some("gpt-4"))?;
println!("Total: {}, Prompt: {}, Completion: {}", 
    count.total, count.prompt, count.completion);
```

### Template Counting

```rust
let template = "Hello {{name}}, please {{action}}";
let variables = hashmap! {
    "name" => "User",
    "action" => "write a function"
};

let count = tokenopt.counter().count_template(template, &variables, Some("gpt-4"))?;
```

### Cached Counting

```rust
// First call - computes and caches
let count1 = tokenopt.optimizer().count_tokens_cached("repeated text", Some("gpt-4"))?;

// Subsequent calls - returns from cache
let count2 = tokenopt.optimizer().count_tokens_cached("repeated text", Some("gpt-4"))?;
assert_eq!(count1, count2);
```

### Supported Models

| Provider | Models |
|----------|--------|
| OpenAI | gpt-4, gpt-4-turbo, gpt-4o, gpt-4o-mini, gpt-3.5-turbo |
| Anthropic | claude-3-opus, claude-3-sonnet, claude-3-haiku, claude-3.5-sonnet, claude-3.5-haiku |
| Google | gemini-1.5-pro, gemini-1.5-flash |
| Cohere | command-r-plus |
| GitHub | copilot-gpt-4, copilot-gpt-4o |

---

## Context Optimization

### Strategies

```rust
use tokenopt_core::context::SelectionStrategy;

// Keep most recent messages
let config = ContextConfig { strategy: SelectionStrategy::RecentFirst, ..Default::default() };

// Keep most relevant to query
let config = ContextConfig { strategy: SelectionStrategy::RelevanceFirst, ..Default::default() };

// Keep most important messages
let config = ContextConfig { strategy: SelectionStrategy::ImportanceFirst, ..Default::default() };

// Balanced approach (recommended)
let config = ContextConfig { strategy: SelectionStrategy::Balanced, ..Default::default() };

// Custom weights
let config = ContextConfig { 
    strategy: SelectionStrategy::Weighted { recency: 0.3, relevance: 0.5, importance: 0.2 },
    ..Default::default() 
};

// Sliding window with summaries
let config = ContextConfig { 
    strategy: SelectionStrategy::SlidingWindow { window_size: 20, summary_interval: 5 },
    ..Default::default() 
};
```

### Usage

```rust
let selector = ContextSelector::new(config, tokenopt.counter().clone());

let selected = selector.select(&messages, Some("binary search"), None)?;

println!("Selected {} of {} messages", selected.messages.len(), messages.len());
println!("Tokens: {} (saved {})", selected.token_count.total, selected.omitted_tokens);
if let Some(summary) = selected.summary {
    println!("Omitted summary:\n{}", summary);
}
```

### Configuration

```rust
let config = ContextConfig {
    strategy: SelectionStrategy::Balanced,
    max_tokens: 8000,           // Max tokens for context
    reserve_tokens: 1000,       // Reserve for response
    system_prompt_tokens: 500,  // System prompt budget
    min_messages: 2,            // Always keep at least 2
    max_messages: Some(50),     // Optional max
    include_system: true,       // Always include system
    compress_old_messages: true, // Summarize old messages
    similarity_threshold: 0.7,  // For relevance scoring
};
```

---

## Prompt Optimization

### Aggressiveness Levels

```rust
use tokenopt_core::prompt::AggressivenessLevel;

let config = PromptOptimizationConfig {
    aggressiveness: AggressivenessLevel::Conservative, // ~10-15% reduction
    // aggressiveness: AggressivenessLevel::Balanced,    // ~20-35% reduction
    // aggressiveness: AggressivenessLevel::Aggressive,  // ~35-50% reduction
    
    preserve_examples: true,
    preserve_formatting: true,
    max_length_reduction: 0.5,
    custom_rules: vec![],
};

let optimizer = PromptOptimizer::new(tokenopt.counter().clone()).with_config(config);
```

### Usage

```rust
// Optimize raw string
let prompt = "Please could you kindly explain in great detail how to write a binary search?";
let optimized = optimizer.optimize_string(prompt, "gpt-4")?;

println!("Original ({} tokens): {}", optimized.original_token_count, optimized.original);
println!("Optimized ({} tokens): {}", optimized.token_count, optimized.optimized);
println!("Saved: {} tokens ({:.1}%)", optimized.tokens_saved, optimized.savings_percentage);
println!("Techniques: {:?}", optimized.techniques_applied);
```

### Task-Specific Templates

```rust
use tokenopt_core::prompt::PromptTask;

let optimized = optimizer.optimize_for_task(
    PromptTask::CodeGeneration,
    "Create a REST API endpoint with authentication and rate limiting",
    "gpt-4"
)?;

println!("{}", optimized.prompt);
```

### Custom Rules

```rust
let config = PromptOptimizationConfig {
    custom_rules: vec![
        CustomRule {
            pattern: r"(?i)\b(as soon as possible)\b".to_string(),
            replacement: "ASAP".to_string(),
            description: "Abbreviate ASAP".to_string(),
        },
        CustomRule {
            pattern: r"(?i)\b(in order to)\b".to_string(),
            replacement: "to".to_string(),
            description: "Simplify 'in order to'".to_string(),
        },
    ],
    ..Default::default()
};
```

---

## Model Routing

### Basic Routing

```rust
let router = tokenopt.router();

let decision = router.route("gpt-4o", TaskComplexity::Simple, &messages)?;

println!("Requested: {}", decision.requested_model);
println!("Selected: {}", decision.selected_model);
println!("Reasoning: {}", decision.reasoning);
println!("Cost: ${:.6}", decision.estimated_cost_usd.unwrap_or(0.0));
```

### Routing Strategies

```rust
use tokenopt_core::router::RoutingStrategy;

// Always use requested model
let router = ModelRouter::new(registry.clone()).with_strategy(RoutingStrategy::Fixed);

// Cheapest suitable model
let router = ModelRouter::new(registry.clone()).with_strategy(RoutingStrategy::CostOptimized);

// Fastest suitable model
let router = ModelRouter::new(registry.clone()).with_strategy(RoutingStrategy::SpeedOptimized);

// Best cost/performance balance
let router = ModelRouter::new(registry.clone()).with_strategy(RoutingStrategy::Balanced);

// Cascade: try cheap, escalate if needed
let router = ModelRouter::new(registry.clone()).with_strategy(RoutingStrategy::Cascade { max_attempts: 3 });
```

### Custom Rules

```rust
let mut router = ModelRouter::new(registry.clone());

router.add_rule(RoutingRule {
    name: "prefer-cheap-for-simple".to_string(),
    condition: RoutingCondition::TaskComplexity(TaskComplexity::Simple),
    action: RoutingAction::UseCheapest,
    priority: 10,
});

router.add_rule(RoutingRule {
    name: "require-function-calling".to_string(),
    condition: RoutingCondition::RequiredFeatures(vec![ModelFeature::FunctionCalling]),
    action: RoutingAction::UseModel("gpt-4o".to_string()),
    priority: 20,
});

router.add_rule(RoutingRule {
    name: "block-expensive".to_string(),
    condition: RoutingCondition::MaxCostPerMillion(1.0),
    action: RoutingAction::BlockModel("gpt-4".to_string()),
    priority: 5,
});
```

### Performance Tracking

```rust
// Record actual performance for adaptive routing
router.record_performance("gpt-4o", 1200, true);  // 1.2s, success
router.record_performance("gpt-3.5-turbo", 800, true);  // 0.8s, success
router.record_performance("gpt-4o", 5000, false); // 5s, timeout
```

---

## Conversation Summarization

### Strategies

```rust
use tokenopt_core::summarizer::SummaryStrategy;

// Extract key sentences/messages
let config = SummaryConfig { strategy: SummaryStrategy::Extractive, ..Default::default() };

// Generate new summary (requires LLM)
let config = SummaryConfig { strategy: SummaryStrategy::Abstractive, ..Default::default() };

// Extractive then compress
let config = SummaryConfig { strategy: SummaryStrategy::Hybrid, ..Default::default() };

// Rolling window
let config = SummaryConfig { 
    strategy: SummaryStrategy::Rolling { window_size: 20 },
    ..Default::default() 
};

// Hierarchical (multi-level)
let config = SummaryConfig { 
    strategy: SummaryStrategy::Hierarchical { levels: 3 },
    ..Default::default() 
};
```

### Usage

```rust
let summarizer = tokenopt.summarizer();

let config = SummaryConfig {
    strategy: SummaryStrategy::Extractive,
    max_summary_tokens: 1000,
    preserve_code_blocks: true,
    preserve_key_decisions: true,
    preserve_errors: true,
    min_messages_to_summarize: 10,
    ..Default::default()
};

let summary = summarizer.summarize(&messages, &config)?;

println!("Original: {} tokens", summary.original_token_count);
println!("Summary: {} tokens", summary.token_count);
println!("Compression: {:.1}%", summary.compression_ratio * 100.0);
println!("Key points: {}", summary.key_points.len());
println!("{}", summary.summary);
```

### Rolling Summary (for streaming)

```rust
// Create initial rolling summary
let mut rolling = summarizer.create_rolling_summary(&messages, 50)?;

// Update with new messages
let new_messages = vec![/* new messages */];
summarizer.update_rolling_summary(&mut rolling, &new_messages)?;

println!("Updated summary: {} tokens", rolling.token_count);
```

---

## Caching

### Basic Cache

```rust
let cache = TokenCache::new(CacheConfig::default());

// Store
cache.put(&CacheKey::new("namespace", "key"), &value, vec!["tag1"])?;

// Retrieve
let value: Option<String> = cache.get(&CacheKey::new("namespace", "key"))?;

// Get or compute (memoization)
let value = cache.get_or_compute(&CacheKey::new("ns", "key"), || {
    expensive_computation()
})?;

// Invalidate
cache.invalidate_by_tag("tag1");
cache.invalidate_namespace("namespace");
cache.clear();

// Statistics
let stats = cache.stats();
println!("Hit rate: {:.1}%", cache.hit_rate() * 100.0);
```

### Token Count Cache

```rust
let cache = TokenCountCache::new(CacheConfig::default());

// Store/retrieve token counts
cache.set_count("text", "gpt-4", 42)?;
let count = cache.get_count("text", "gpt-4");

// Store/retrieve conversation counts
cache.set_conversation_count(&messages, "gpt-4", &count)?;
let count = cache.get_conversation_count(&messages, "gpt-4");
```

### Configuration

```rust
let config = CacheConfig {
    max_entries: 100_000,
    ttl: Duration::from_secs(3600),  // 1 hour
    persist_to_disk: false,
    cache_dir: None,
    key_prefix: "tokenopt".to_string(),
};
```

---

## Cost Estimation

### Per-Model Pricing

```rust
if let Some(model) = tokenopt.registry().get("gpt-4o") {
    if let Some(pricing) = &model.pricing {
        println!("Input: ${:.4}/M tokens", pricing.input_per_million);
        println!("Output: ${:.4}/M tokens", pricing.output_per_million);
        println!("Cache read: ${:.4}/M", pricing.cache_read_per_million.unwrap_or(0.0));
        println!("Cache write: ${:.4}/M", pricing.cache_write_per_million.unwrap_or(0.0));
    }
}
```

### Conversation Cost

```rust
let count = tokenopt.counter().count_conversation(&messages, Some("gpt-4o"))?;

if let Some(model) = tokenopt.registry().get("gpt-4o") {
    if let Some(pricing) = &model.pricing {
        let cost = (count.prompt as f64 * pricing.input_per_million / 1_000_000.0) +
                   (count.completion as f64 * pricing.output_per_million / 1_000_000.0);
        println!("Estimated cost: ${:.6}", cost);
    }
}
```

### Optimization Savings

```rust
let result = tokenopt.optimizer().optimize_conversation(&messages, None, Some("gpt-4"))?;

if let Some(savings) = result.estimated_cost_savings_usd {
    println!("Cost savings: ${:.6}", savings);
}
```

---

## Advanced Usage

### Custom Models

```rust
let custom_model = ModelCapabilities {
    id: "my-custom-model".to_string(),
    name: "My Custom Model".to_string(),
    provider: "custom".to_string(),
    context_window: 32_000,
    max_output_tokens: 4096,
    pricing: Some(ModelPricing::new(0.50, 1.50)),
    features: ModelFeatures {
        function_calling: true,
        vision: false,
        json_mode: true,
        parallel_tool_calls: true,
        streaming: true,
        system_prompt: true,
        logprobs: false,
    },
    family: "custom".to_string(),
    is_fast_variant: false,
    knowledge_cutoff: Some("2024-01".to_string()),
};

let tokenopt = TokenOptBuilder::new()
    .add_model(custom_model)
    .build()?;
```

### Custom Embedder for Relevance

```rust
use tokenopt_core::context::Embedder;

struct MyEmbedder;
impl Embedder for MyEmbedder {
    fn embed(&self, text: &str) -> Result<Vec<f32>, SelectionError> {
        // Call your embedding API
        Ok(my_embedding_api(text))
    }
    fn dimensions(&self) -> usize { 1536 }
}

let selector = ContextSelector::new(config, counter)
    .with_embedder(Arc::new(MyEmbedder));
```

### Custom Prompt Templates

```rust
let template = PromptTemplate::from_string(
    "Task: {{task}}\nContext: {{context}}\nRequirements: {{requirements}}\nOutput: Code only."
)
.with_variable("task", "Create REST API")
.with_variable("context", "Rust with Axum")
.with_variable("requirements", "Auth, rate limiting, tests");

let optimized = tokenopt.prompt_optimizer().optimize(&template, "gpt-4")?;
```

### Pipeline Customization

```rust
let mut config = OptimizationConfig::default();
config.enable_caching = true;
config.enable_summarization = false;  // Disable if not needed
config.enable_prompt_optimization = true;
config.enable_model_routing = true;
config.context_config.max_tokens = 4000;  // Smaller budget
config.max_optimization_time_ms = 50;     // Faster

let optimizer = TokenOptimizer::new(counter, registry).with_config(config);
```

### Streaming with Rolling Summary

```rust
let mut rolling = summarizer.create_rolling_summary(&initial_messages, 20)?;

loop {
    let new_message = receive_message()?;
    rolling = summarizer.update_rolling_summary(rolling, &[new_message])?;
    
    // Use rolling.summary as context for next request
    let context = rolling.summary;
}
```

---

## Best Practices

### 1. Choose Right Strategy

| Scenario | Recommended Strategy |
|----------|---------------------|
| Chat with short history | `SelectionStrategy::RecentFirst` |
| RAG/Retrieval | `SelectionStrategy::RelevanceFirst` |
| Long technical discussions | `SelectionStrategy::ImportanceFirst` |
| General purpose | `SelectionStrategy::Balanced` |
| Streaming/real-time | `SelectionStrategy::SlidingWindow` |

### 2. Configure Token Budget

```rust
// For gpt-4o (128K context)
let config = ContextConfig {
    max_tokens: 100_000,  // Leave room for response
    reserve_tokens: 4096,
    ..Default::default()
};

// For gpt-3.5-turbo (16K context)
let config = ContextConfig {
    max_tokens: 12_000,
    reserve_tokens: 2048,
    ..Default::default()
};
```

### 3. Enable Caching for Repeated Work

```rust
let optimizer = TokenOptimizer::new(counter, registry)
    .with_config(OptimizationConfig {
        enable_caching: true,
        ..Default::default()
    });

// Repeated calls will use cache
for query in queries {
    let result = optimizer.optimize_conversation(&messages, Some(query), Some("gpt-4"))?;
}
```

### 4. Use Appropriate Aggressiveness

```rust
// User-facing: conservative
let config = PromptOptimizationConfig { aggressiveness: AggressivenessLevel::Conservative, .. };

// Internal/batch: aggressive
let config = PromptOptimizationConfig { aggressiveness: AggressivenessLevel::Aggressive, .. };
```

### 5. Route Based on Task

```rust
let decision = router.route(requested_model, 
    match task_type {
        "simple_qa" => TaskComplexity::Simple,
        "code_gen" => TaskComplexity::Moderate,
        "architecture" => TaskComplexity::Complex,
    },
    &messages
)?;
```

### 6. Monitor Performance

```rust
// Track optimization metrics
let result = optimizer.optimize_conversation(&messages, None, Some("gpt-4"))?;

println!("Optimization time: {}ms", result.metadata.optimization_time_ms);
println!("Strategies: {:?}", result.metadata.strategies_applied);
println!("Cache hit: {}", result.metadata.cache_hit);

// Track model performance
router.record_performance(&decision.selected_model, latency_ms, success);
```

---

## Troubleshooting

### "Model not found" Error

```rust
// Ensure model is registered
if tokenopt.registry().get("gpt-4o").is_none() {
    // Register or use alias
    tokenopt.counter().count_tokens("text", Some("gpt-4o"))?;  // Auto-registers via alias
}
```

### Token Count Seems Wrong

```rust
// Check model alias
let count = counter.count_tokens("text", Some("claude-3-sonnet"))?;
// Uses gpt-4 tokenizer as approximation

// For exact count, use correct model
let count = counter.count_tokens("text", Some("gpt-4o"))?;
```

### Optimization Not Saving Tokens

```rust
// Check if already under budget
if messages_token_count <= config.max_tokens {
    // No optimization needed
}

// Increase budget or change strategy
config.context_config.max_tokens = 4000;  // Smaller = more aggressive
config.context_config.strategy = SelectionStrategy::ImportanceFirst;
```

### Slow Performance

```rust
// Enable caching
config.enable_caching = true;

// Reduce optimization time limit
config.max_optimization_time_ms = 50;

// Use simpler strategy
config.context_config.strategy = SelectionStrategy::RecentFirst;

// Disable expensive features
config.enable_summarization = false;
config.enable_model_routing = false;
```

### Memory Issues

```rust
// Reduce cache size
let config = CacheConfig {
    max_entries: 10_000,
    ttl: Duration::from_secs(1800),  // 30 min
    ..Default::default()
};

// Clear periodically
cache.cleanup_expired();
```

### Debugging

```rust
// Enable debug logging
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

// Check optimization metadata
let result = optimizer.optimize_conversation(&messages, None, Some("gpt-4"))?;
println!("{:#?}", result.metadata);
```

---

## Performance Tuning

### Benchmark Your Workload

```bash
cargo bench --workspace --all-features
```

### Profile

```bash
cargo install flamegraph
cargo flamegraph --bench full_optimization
```

### Common Optimizations

1. **Reuse TokenOpt instance** - Avoid creating new instances
2. **Enable caching** - For repeated queries
3. **Use appropriate models** - gpt-4o-mini for simple tasks
4. **Batch operations** - Process multiple conversations together
5. **Limit context** - Smaller `max_tokens` = faster selection

---

## Examples

See `examples/` directory for complete working examples:

- `basic_usage.rs` - Rust examples
- `basic_usage.ts` - TypeScript examples  
- `basic_usage.py` - Python examples
- `sample_conversation.json` - Sample conversation for testing

Run examples:

```bash
# Rust
cargo run --example basic_usage

# TypeScript
cd examples && npx ts-node basic_usage.ts

# Python
python examples/basic_usage.py
```