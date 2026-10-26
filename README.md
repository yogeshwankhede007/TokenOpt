# TokenOpt

<div align="center">

![TokenOpt Banner](https://img.shields.io/badge/TokenOpt-Universal%20Token%20Optimization-00D4AA?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgc3Ryb2tlPSJ3aGl0ZSIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiIGNsYXNzPSJsdWNpZGUgbHVjaWRlLWNwdSI+PHBhdGggZD0iTTQgMTdoMTYiLz48cGF0aCBkPSJNNCAxMWwxNi00Ii8+PHBhdGggZD0iTTggMTZoOCIvPjwvc3ZnPg==)

[![Crates.io (Coming Soon)](https://img.shields.io/badge/crates.io-tokenopt__core-orange?style=for-the-badge&logo=rust)](https://crates.io/crates/tokenopt-core)
[![npm (Coming Soon)](https://img.shields.io/badge/npm-@tokenopt%2Fcore-red?style=for-the-badge&logo=npm)](https://www.npmjs.com/package/@tokenopt/core)
[![PyPI (Coming Soon)](https://img.shields.io/badge/pypi-tokenopt-blue?style=for-the-badge&logo=python)](https://pypi.org/project/tokenopt/)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85+-dea584?style=for-the-badge&logo=rust)](https://rust-lang.org)
[![TypeScript](https://img.shields.io/badge/typescript-5.0+-3178c6?style=for-the-badge&logo=typescript)](https://typescriptlang.org)
[![Python](https://img.shields.io/badge/python-3.9+-3776ab?style=for-the-badge&logo=python)](https://python.org)

**The universal, high-performance token optimization layer for the AI era.**  
Built for **Claude Code**, **GitHub Copilot**, **ChatGPT**, **Cursor**, **Windsurf**, and every LLM-powered coding assistant.

</div>

---

## 🌌 Why TokenOpt?

<div align="center">

| Problem | Solution |
|---------|----------|
| 💸 **Exploding API costs** | Reduce token usage **20-60%** with intelligent optimization |
| 🐌 **Context window limits** | Fit 10x more context with smart selection & summarization |
| ⚡ **Latency bottlenecks** | Route to faster models automatically with cascade strategies |
| 🔧 **Fragmented tooling** | One universal library: Rust, TypeScript, Python, Go, CLI |

</div>

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#00D4AA', 'edgeLabelBackground':'#0a0f1a', 'tertiaryColor': '#1a1f2e'}}}%%
graph TB
    subgraph "🎯 Input Layer"
        A[Raw Conversation] --> B{TokenOpt Core}
        C[Code Context] --> B
        D[System Prompts] --> B
    end
    
    subgraph "⚡ Optimization Pipeline"
        B --> E[🔢 Token Counter<br/>tiktoken + custom]
        E --> F[🎯 Context Selector<br/>6 strategies]
        F --> G[✂️ Prompt Optimizer<br/>3 aggressiveness levels]
        G --> H[🧠 Summarizer<br/>5 algorithms]
        H --> I[🔄 Model Router<br/>5 routing strategies]
    end
    
    subgraph "🎨 Output Layer"
        I --> J[Optimized Context]
        I --> K[Cost Estimate]
        I --> L[Routing Decision]
        I --> M[Summary]
    end
    
    style B fill:#00D4AA,color:#0a0f1a
    style E fill:#1a1f2e,stroke:#00D4AA
    style F fill:#1a1f2e,stroke:#00D4AA
    style G fill:#1a1f2e,stroke:#00D4AA
    style H fill:#1a1f2e,stroke:#00D4AA
    style I fill:#1a1f2e,stroke:#00D4AA
```

---

## 🚀 Quick Start — How to Use TokenOpt

Choose your language. The API is identical across all bindings.

### Installation

<details>
<summary><b>🦀 Rust (Core Library)</b></summary>

Add to your `Cargo.toml`:

```toml
[dependencies]
tokenopt-core = { version = "0.1", features = ["full"] }
```

Or install the CLI:

```bash
cargo install tokenopt-cli
```

</details>

<details>
<summary><b>📦 TypeScript / JavaScript</b></summary>

```bash
npm install @tokenopt/core
# or
yarn add @tokenopt/core
# or
pnpm add @tokenopt/core
```

</details>

<details>
<summary><b>🐍 Python</b></summary>

```bash
pip install tokenopt
```

</details>

<details>
<summary><b>🐹 Go (Planned)</b></summary>

```bash
go get github.com/tokenopt/tokenopt-go
```

</details>

---

## 📖 Detailed Usage Guide

### 1. Initialize TokenOpt

Create a `TokenOpt` instance — this is your entry point to all features.

<details>
<summary><b>Rust</b></summary>

```rust
use tokenopt_core::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokenopt = TokenOpt::new()?;
    
    // All modules are accessible via the instance:
    // tokenopt.counter()      - Token counting
    // tokenopt.optimizer()    - Full optimization pipeline
    // tokenopt.prompt_optimizer() - Prompt compression
    // tokenopt.router()       - Model routing
    // tokenopt.summarizer()   - Conversation summarization
    // tokenopt.registry()     - Model pricing & info
    
    Ok(())
}
```

</details>

<details>
<summary><b>TypeScript</b></summary>

```typescript
import { TokenOpt } from '@tokenopt/core';

const tokenopt = new TokenOpt();

// All modules are accessible:
// tokenopt.counter
// tokenopt.optimizer
// tokenopt.promptOptimizer
// tokenopt.router
// tokenopt.summarizer
// tokenopt.registry
```

</details>

<details>
<summary><b>Python</b></summary>

```python
from tokenopt import TokenOpt

tokenopt = TokenOpt()

# All modules are accessible:
# tokenopt.counter
# tokenopt.optimizer
# tokenopt.prompt_optimizer
# tokenopt.router
# tokenopt.summarizer
# tokenopt.registry
```

</details>

---

### 2. Token Counting

Count tokens for text or entire conversations using tiktoken-compatible encoders.

<details>
<summary><b>Rust</b></summary>

```rust
use tokenopt_core::prelude::*;

let tokenopt = TokenOpt::new()?;

// Count tokens for a single string
let text = "Hello, world! This is a test message.";
let count = tokenopt.counter().count_tokens(text, Some("gpt-4"))?;
println!("Tokens: {}", count);

// Count tokens for a conversation
let messages = vec![
    Message {
        role: Role::System,
        content: "You are a helpful coding assistant.".to_string(),
        ..Default::default()
    },
    Message {
        role: Role::User,
        content: "How do I implement a binary search in Rust?".to_string(),
        ..Default::default()
    },
    Message {
        role: Role::Assistant,
        content: "Here's the implementation...".to_string(),
        ..Default::default()
    },
];

let conv_count = tokenopt.counter().count_conversation(&messages, Some("gpt-4"))?;
println!("Total: {}", conv_count.total);
println!("Prompt: {}", conv_count.prompt);
println!("Completion: {}", conv_count.completion);
```

</details>

<details>
<summary><b>TypeScript</b></summary>

```typescript
import { TokenOpt } from '@tokenopt/core';

const tokenopt = new TokenOpt();

// Count tokens for a string
const count = tokenopt.counter.countTokens("Hello, world!", "gpt-4");
console.log(`Tokens: ${count}`);

// Count tokens for a conversation
const messages = [
  { role: 'system', content: 'You are a helpful coding assistant.' },
  { role: 'user', content: 'How do I implement a binary search in Rust?' },
  { role: 'assistant', content: 'Here is the implementation...' },
];

const convCount = tokenopt.counter.countConversation(messages, 'gpt-4');
console.log(`Total: ${convCount.total}`);
console.log(`Prompt: ${convCount.prompt}`);
console.log(`Completion: ${convCount.completion}`);
```

</details>

<details>
<summary><b>Python</b></summary>

```python
from tokenopt import TokenOpt

tokenopt = TokenOpt()

# Count tokens for a string
count = tokenopt.counter.count_tokens("Hello, world!", "gpt-4")
print(f"Tokens: {count}")

# Count tokens for a conversation
messages = [
    {"role": "system", "content": "You are a helpful coding assistant."},
    {"role": "user", "content": "How do I implement a binary search in Rust?"},
    {"role": "assistant", "content": "Here is the implementation..."},
]

conv_count = tokenopt.counter.count_conversation(messages, "gpt-4")
print(f"Total: {conv_count.total}")
print(f"Prompt: {conv_count.prompt}")
print(f"Completion: {conv_count.completion}")
```

</details>

---

### 3. Context Selection

Fit conversations within token budgets using 6 strategies: `RecentFirst`, `RelevanceFirst`, `ImportanceFirst`, `Balanced`, `Weighted`, `SlidingWindow`.

<details>
<summary><b>Rust</b></summary>

```rust
use tokenopt_core::prelude::*;

let tokenopt = TokenOpt::new()?;

let messages = vec![
    Message { role: Role::System, content: "You are a helpful assistant.".into(), ..Default::default() },
    Message { role: Role::User, content: "Question 1".into(), ..Default::default() },
    Message { role: Role::Assistant, content: "Answer 1".into(), ..Default::default() },
    Message { role: Role::User, content: "Question 2".into(), ..Default::default() },
];

let config = ContextConfig {
    strategy: SelectionStrategy::Balanced,
    max_tokens: 1000,
    reserve_tokens: 200,
    min_messages: 2,
    compress_old_messages: true,
    similarity_threshold: 0.7,
    ..Default::default()
};

let selector = ContextSelector::new(config, tokenopt.counter().clone());
let selected = selector.select(&messages, Some("binary search"), None)?;

println!("Selected {} of {} messages", selected.messages.len(), messages.len());
println!("Tokens: {} (saved {})", selected.token_count.total, selected.omitted_tokens);
```

</details>

<details>
<summary><b>TypeScript</b></summary>

```typescript
import { TokenOpt, ContextSelector } from '@tokenopt/core';

const tokenopt = new TokenOpt();

const messages = [
  { role: 'system', content: 'You are a helpful assistant.' },
  { role: 'user', content: 'Question 1' },
  { role: 'assistant', content: 'Answer 1' },
  { role: 'user', content: 'Question 2' },
];

const selector = new ContextSelector({
  strategy: 'Balanced',
  maxTokens: 1000,
  reserveTokens: 200,
  minMessages: 2,
  compressOldMessages: true,
  similarityThreshold: 0.7,
});

const selected = selector.select(messages, 'binary search');
console.log(`Selected ${selected.messages.length} of ${messages.length} messages`);
console.log(`Tokens: ${selected.tokenCount.total} (saved ${selected.omittedTokens})`);
```

</details>

<details>
<summary><b>Python</b></summary>

```python
from tokenopt import TokenOpt, ContextSelector, SelectionStrategy

tokenopt = TokenOpt()

messages = [
    {"role": "system", "content": "You are a helpful assistant."},
    {"role": "user", "content": "Question 1"},
    {"role": "assistant", "content": "Answer 1"},
    {"role": "user", "content": "Question 2"},
]

selector = ContextSelector({
    "strategy": SelectionStrategy.Balanced,
    "max_tokens": 1000,
    "reserve_tokens": 200,
    "min_messages": 2,
    "compress_old_messages": True,
})

selected = selector.select(messages, query="binary search")
print(f"Selected {len(selected.messages)} of {len(messages)} messages")
print(f"Tokens: {selected.token_count.total} (saved {selected.omitted_tokens})")
```

</details>

---

### 4. Prompt Optimization

Reduce prompt tokens by 20-50% while preserving intent. Three aggressiveness levels: `Conservative`, `Balanced`, `Aggressive`.

<details>
<summary><b>Rust</b></summary>

```rust
use tokenopt_core::prelude::*;

let tokenopt = TokenOpt::new()?;

let prompt = "Please could you kindly explain in great detail how to write a binary search function in Rust step by step with examples?";
let template = PromptTemplate::from_string(prompt);
let optimized = tokenopt.prompt_optimizer().optimize(&template, "gpt-4")?;

println!("Original ({} tokens): {}", optimized.original_token_count, optimized.original);
println!("Optimized ({} tokens): {}", optimized.token_count, optimized.optimized);
println!("Saved: {} tokens ({:.1}%)", optimized.tokens_saved, optimized.savings_percentage);
println!("Techniques: {:?}", optimized.techniques_applied);
```

</details>

<details>
<summary><b>TypeScript</b></summary>

```typescript
import { TokenOpt, PromptTask } from '@tokenopt/core';

const tokenopt = new TokenOpt();

// Simple string optimization
const prompt = "Please could you kindly explain in great detail how to write a binary search function in Rust?";
const optimized = tokenopt.promptOptimizer.optimizeString(prompt, "gpt-4");

console.log(`Original (${optimized.originalTokenCount} tokens): ${optimized.original}`);
console.log(`Optimized (${optimized.tokenCount} tokens): ${optimized.optimized}`);
console.log(`Saved: ${optimized.tokensSaved} tokens (${optimized.savingsPercentage.toFixed(1)}%)`);
console.log(`Techniques: ${optimized.techniquesApplied.join(', ')}`);

// Task-specific optimization
const taskOptimized = tokenopt.promptOptimizer.optimizeForTask(
  PromptTask.CodeGeneration,
  "Create a REST API endpoint with JWT authentication and rate limiting",
  "gpt-4"
);
console.log(`Task-optimized: ${taskOptimized.prompt}`);
console.log(`Saved: ${taskOptimized.tokensSaved} tokens (${taskOptimized.savingsPercentage.toFixed(1)}%)`);
```

</details>

<details>
<summary><b>Python</b></summary>

```python
from tokenopt import TokenOpt, PromptTask

tokenopt = TokenOpt()

# Simple string optimization
prompt = "Please could you kindly explain in great detail how to write a binary search function in Rust?"
optimized = tokenopt.prompt_optimizer.optimize_string(prompt, "gpt-4")

print(f"Original ({optimized.original_token_count} tokens): {optimized.original}")
print(f"Optimized ({optimized.token_count} tokens): {optimized.optimized}")
print(f"Saved: {optimized.tokens_saved} tokens ({optimized.savings_percentage:.1f}%)")
print(f"Techniques: {', '.join(optimized.techniques_applied)}")

# Task-specific optimization
task_optimized = tokenopt.prompt_optimizer.optimize_for_task(
    PromptTask.CodeGeneration,
    "Create a REST API endpoint with JWT authentication and rate limiting",
    "gpt-4"
)
print(f"Task-optimized: {task_optimized.prompt}")
print(f"Saved: {task_optimized.tokens_saved} tokens ({task_optimized.savings_percentage:.1f}%)")
```

</details>

---

### 5. Model Routing

Automatically route to the best model based on task complexity, cost, speed, or quality.

<details>
<summary><b>Rust</b></summary>

```rust
use tokenopt_core::prelude::*;

let tokenopt = TokenOpt::new()?;
let router = tokenopt.router();

let decision = router.route("gpt-4o", TaskComplexity::Simple, &messages)?;

println!("Requested: {}", decision.requested_model);
println!("Selected: {}", decision.selected_model);
println!("Strategy: {:?}", decision.strategy);
println!("Reasoning: {}", decision.reasoning);
println!("Confidence: {:.0}%", decision.confidence * 100.0);
if let Some(cost) = decision.estimated_cost_usd {
    println!("Estimated cost: ${:.6}", cost);
}
```

</details>

<details>
<summary><b>TypeScript</b></summary>

```typescript
import { TokenOpt, TaskComplexity } from '@tokenopt/core';

const tokenopt = new TokenOpt();
const router = new ModelRouter('Balanced');

const decision = router.route('gpt-4o', TaskComplexity.Simple, messages);

console.log(`Requested: ${decision.requestedModel}`);
console.log(`Selected: ${decision.selectedModel}`);
console.log(`Strategy: ${decision.strategy}`);
console.log(`Reasoning: ${decision.reasoning}`);
console.log(`Confidence: ${(decision.confidence * 100).toFixed(0)}%`);
if (decision.estimatedCostUsd) {
  console.log(`Estimated cost: $${decision.estimatedCostUsd.toFixed(6)}`);
}
```

</details>

<details>
<summary><b>Python</b></summary>

```python
from tokenopt import TokenOpt, ModelRouter, TaskComplexity, RoutingStrategy

tokenopt = TokenOpt()
router = ModelRouter(RoutingStrategy.Balanced)

decision = router.route("gpt-4o", TaskComplexity.Simple, messages)

print(f"Requested: {decision.requested_model}")
print(f"Selected: {decision.selected_model}")
print(f"Strategy: {decision.strategy}")
print(f"Reasoning: {decision.reasoning}")
print(f"Confidence: {decision.confidence * 100:.0f}%")
if decision.estimated_cost_usd:
    print(f"Estimated cost: ${decision.estimated_cost_usd:.6f}")
```

</details>

---

### 6. Conversation Summarization

Summarize long conversations using 5 algorithms: `Extractive`, `Abstractive`, `Hybrid`, `Rolling`, `Hierarchical`.

<details>
<summary><b>Rust</b></summary>

```rust
use tokenopt_core::prelude::*;

let tokenopt = TokenOpt::new()?;

let config = SummaryConfig {
    strategy: SummaryStrategy::Extractive,
    max_summary_tokens: 200,
    preserve_code_blocks: true,
    preserve_key_decisions: true,
    preserve_errors: true,
    ..Default::default()
};

let summary = tokenopt.summarizer().summarize(&messages, &config)?;

println!("Original tokens: {}", summary.original_token_count);
println!("Summary tokens: {}", summary.token_count);
println!("Compression: {:.1}%", summary.compression_ratio * 100.0);
println!("Messages summarized: {}", summary.messages_summarized);
println!("Summary:\n{}", summary.summary);
```

</details>

<details>
<summary><b>TypeScript</b></summary>

```typescript
import { TokenOpt, ConversationSummarizer } from '@tokenopt/core';

const tokenopt = new TokenOpt();
const summarizer = new ConversationSummarizer({
  strategy: 'Extractive',
  maxSummaryTokens: 200,
  preserveCodeBlocks: true,
  preserveKeyDecisions: true,
  preserveErrors: true,
});

const summary = summarizer.summarize(messages, {
  strategy: 'Extractive',
  maxSummaryTokens: 200,
  preserveCodeBlocks: true,
  preserveKeyDecisions: true,
  preserveErrors: true,
  minMessagesToSummarize: 5,
});

console.log(`Original tokens: ${summary.originalTokenCount}`);
console.log(`Summary tokens: ${summary.tokenCount}`);
console.log(`Compression: ${(summary.compressionRatio * 100).toFixed(1)}%`);
console.log(`Messages summarized: ${summary.messagesSummarized}`);
console.log(`Summary:\n${summary.summary}`);

// Rolling summary for very long conversations
const longMessages = [...messages, ...moreMessages];
const rolling = summarizer.createRollingSummary(longMessages, 5);
console.log(`Rolling summary: ${rolling.tokenCount} tokens`);
```

</details>

<details>
<summary><b>Python</b></summary>

```python
from tokenopt import TokenOpt, ConversationSummarizer, SummaryStrategy

tokenopt = TokenOpt()
summarizer = ConversationSummarizer({
    "strategy": SummaryStrategy.Extractive,
    "max_summary_tokens": 200,
    "preserve_code_blocks": True,
    "preserve_key_decisions": True,
    "preserve_errors": True,
})

summary = summarizer.summarize(messages, {
    "strategy": SummaryStrategy.Extractive,
    "max_summary_tokens": 200,
    "preserve_code_blocks": True,
    "preserve_key_decisions": True,
    "preserve_errors": True,
    "min_messages_to_summarize": 5,
})

print(f"Original tokens: {summary.original_token_count}")
print(f"Summary tokens: {summary.token_count}")
print(f"Compression: {summary.compression_ratio * 100:.1f}%")
print(f"Messages summarized: {summary.messages_summarized}")
print(f"Summary:\n{summary.summary}")

# Rolling summary for long conversations
long_messages = messages.copy()
# ... add more messages
rolling = summarizer.create_rolling_summary(long_messages, 5)
print(f"Rolling summary: {rolling.token_count} tokens")
```

</details>

---

### 7. Full Optimization Pipeline

Run the complete optimization stack in one call — context selection, prompt optimization, summarization, and model routing.

<details>
<summary><b>Rust</b></summary>

```rust
use tokenopt_core::prelude::*;

let tokenopt = TokenOpt::new()?;

let config = OptimizationConfig {
    context_config: ContextConfig {
        strategy: SelectionStrategy::Balanced,
        max_tokens: 8000,
        reserve_tokens: 1000,
        min_messages: 2,
        compress_old_messages: true,
        similarity_threshold: 0.7,
        ..Default::default()
    },
    enable_caching: true,
    enable_summarization: true,
    enable_prompt_optimization: true,
    enable_model_routing: true,
    default_model: "gpt-4o".to_string(),
    max_optimization_time_ms: 100,
};

let result = tokenopt.optimizer().optimize_conversation(&long_messages, None, Some("gpt-4"))?;

println!("Original tokens: {}", result.original_token_count.total);
println!("Optimized tokens: {}", result.optimized_token_count.total);
println!("Tokens saved: {}", result.tokens_saved);
println!("Savings: {:.1}%", result.savings_percentage);
if let Some(cost_savings) = result.estimated_cost_savings_usd {
    println!("Cost savings: ${:.6}", cost_savings);
}
println!("Strategies applied: {:?}", result.metadata.strategies_applied);
println!("Optimization time: {}ms", result.metadata.optimization_time_ms);
```

</details>

<details>
<summary><b>TypeScript</b></summary>

```typescript
import { TokenOpt, TokenOptimizer } from '@tokenopt/core';

const tokenopt = new TokenOpt();
const optimizer = new TokenOptimizer({
  enableCaching: true,
  enableSummarization: true,
  enablePromptOptimization: true,
  enableModelRouting: true,
  defaultModel: 'gpt-4',
  maxOptimizationTimeMs: 100,
});

const optResult = optimizer.optimizeConversation(longMessages, undefined, 'gpt-4');

console.log(`Original tokens: ${optResult.originalTokenCount.total}`);
console.log(`Optimized tokens: ${optResult.optimizedTokenCount.total}`);
console.log(`Tokens saved: ${optResult.tokensSaved}`);
console.log(`Savings: ${optResult.savingsPercentage.toFixed(1)}%`);
if (optResult.estimatedCostSavingsUsd) {
  console.log(`Cost savings: $${optResult.estimatedCostSavingsUsd.toFixed(6)}`);
}
console.log('Strategies applied:');
optResult.metadata.strategiesApplied.forEach(s => console.log(`  - ${JSON.stringify(s)}`));
console.log(`Optimization time: ${optResult.metadata.optimizationTimeMs}ms`);
```

</details>

<details>
<summary><b>Python</b></summary>

```python
from tokenopt import TokenOpt, TokenOptimizer

tokenopt = TokenOpt()
optimizer = TokenOptimizer({
    "enable_caching": True,
    "enable_summarization": True,
    "enable_prompt_optimization": True,
    "enable_model_routing": True,
    "default_model": "gpt-4",
    "max_optimization_time_ms": 100,
})

opt_result = optimizer.optimize_conversation(long_messages, model="gpt-4")

print(f"Original tokens: {opt_result.original_token_count.total}")
print(f"Optimized tokens: {opt_result.optimized_token_count.total}")
print(f"Tokens saved: {opt_result.tokens_saved}")
print(f"Savings: {opt_result.savings_percentage:.1f}%")
if opt_result.estimated_cost_savings_usd:
    print(f"Cost savings: ${opt_result.estimated_cost_savings_usd:.6f}")
print("Strategies applied:")
for strategy in opt_result.metadata.strategies_applied:
    print(f"  - {strategy}")
print(f"Optimization time: {opt_result.metadata.optimization_time_ms}ms")
```

</details>

---

### 8. Cost Estimation

Calculate real-time costs for any model using live pricing data.

<details>
<summary><b>Rust</b></summary>

```rust
use tokenopt_core::prelude::*;

let tokenopt = TokenOpt::new()?;

if let Some(model) = tokenopt.registry().get("gpt-4o") {
    if let Some(pricing) = &model.pricing {
        let input_cost = conv_count.prompt as f64 * pricing.input_per_million / 1_000_000.0;
        let output_cost = conv_count.completion as f64 * pricing.output_per_million / 1_000_000.0;
        println!("Estimated cost: ${:.6}", input_cost + output_cost);
    }
}
```

</details>

<details>
<summary><b>TypeScript</b></summary>

```typescript
const model = tokenopt.registry.all().find(m => m.id === 'gpt-4o');
if (model?.pricing) {
  const cost = (convCount.prompt * model.pricing.inputPerMillion / 1_000_000) +
               (convCount.completion * model.pricing.outputPerMillion / 1_000_000);
  console.log(`Estimated cost: $${cost.toFixed(6)}`);
}
```

</details>

<details>
<summary><b>Python</b></summary>

```python
for model in tokenopt.registry.all():
    if model.id == "gpt-4o" and model.pricing:
        cost = (conv_count.prompt * model.pricing.input_per_million / 1_000_000) + \
               (conv_count.completion * model.pricing.output_per_million / 1_000_000)
        print(f"Estimated cost: ${cost:.6f}")
        break
```

</details>

---

### 9. Configuration File (`tokenopt.toml`)

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
persist_to_disk = true
cache_dir = "~/.tokenopt/cache"

[router]
strategy = "Balanced"
cascade_threshold = 0.7
preferred_providers = ["openai", "anthropic", "google"]

[summarizer]
default_strategy = "hierarchical"
max_summary_tokens = 1000
llm_provider = "openai"
llm_model = "gpt-4o-mini"
```

---

### 10. CLI Tool

```bash
# Install
cargo install tokenopt-cli

# Count tokens in a file
tokenopt count --file conversation.json --conversation

# Optimize a conversation
tokenopt optimize --file conversation.json --model gpt-4 --output optimized.json

# Optimize a prompt
tokenopt prompt --text "Please write a function that..." --model gpt-4 --aggressiveness balanced

# Route to best model
tokenopt route --model gpt-4 --file conversation.json --complexity complex --strategy cost-optimized

# Summarize conversation
tokenopt summarize --file conversation.json --strategy hierarchical --max-tokens 1000

# List available models
tokenopt models --provider openai --tier cheap

# Estimate cost
tokenopt cost --model gpt-4 --prompt-tokens 1000 --completion-tokens 500
```

---

## 🏗 Architecture Deep Dive

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#00D4AA', 'background': '#0a0f1a', 'edgeLabelBackground':'#1a1f2e'}}}%%
graph TB
    subgraph "🌐 Language Bindings Layer"
        TS[TypeScript/JS<br/>NAPI-RS]
        PY[Python<br/>PyO3]
        GO[Go<br/>CGO]
        CLI[CLI<br/>Rust]
    end
    
    subgraph "🔗 FFI Bridge"
        TS -.->|napi| CORE
        PY -.->|pyo3| CORE
        GO -.->|cgo| CORE
        CLI -.->|native| CORE
    end
    
    subgraph "⚙️ tokenopt-core (Rust)"
        CORE[Core Engine]
        
        subgraph "📦 Modules"
            CNT[Token Counter<br/>tiktoken-rs + custom]
            CTX[Context Selector<br/>6 strategies]
            PRM[Prompt Optimizer<br/>3 levels]
            SUM[Summarizer<br/>5 algorithms]
            RTR[Model Router<br/>5 strategies]
            CCH[Cache Layer<br/>LRU + TTL + Disk]
            CST[Cost Engine<br/>Live pricing API]
        end
    end
    
    subgraph "☁️ External"
        PRICE[Pricing APIs<br/>OpenAI, Anthropic, Google]
        LLM[LLM Providers<br/>For abstractive summarization]
    end
    
    CNT --> CORE
    CTX --> CORE
    PRM --> CORE
    SUM --> CORE
    RTR --> CORE
    CCH --> CORE
    CST --> CORE
    
    SUM -.->|optional| LLM
    CST -.->|live| PRICE
    
    style CORE fill:#00D4AA,color:#0a0f1a
    style CNT fill:#1a1f2e,stroke:#00D4AA
    style CTX fill:#1a1f2e,stroke:#00D4AA
    style PRM fill:#1a1f2e,stroke:#00D4AA
    style SUM fill:#1a1f2e,stroke:#00D4AA
    style RTR fill:#1a1f2e,stroke:#00D4AA
    style CCH fill:#1a1f2e,stroke:#00D4AA
    style CST fill:#1a1f2e,stroke:#00D4AA
```

### 🔄 Data Flow Pipeline

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#00D4AA'}}}%%
sequenceDiagram
    participant App as Your App
    participant TO as TokenOpt
    participant CNT as Counter
    participant CTX as Context Selector
    participant PRM as Prompt Optimizer
    participant SUM as Summarizer
    participant RTR as Model Router
    participant CCH as Cache
    
    App->>TO: optimize(conversation, model)
    TO->>CNT: count_tokens(conversation)
    CNT-->>TO: token_count + breakdown
    
    alt tokens > budget
        TO->>CTX: select_context(conversation, budget)
        CTX-->>TO: selected_messages
        TO->>PRM: optimize_prompts(selected_messages)
        PRM-->>TO: optimized_prompts
        TO->>SUM: summarize(removed_messages)
        SUM-->>TO: summary
        TO->>TO: inject_summary(context)
    end
    
    TO->>RTR: route(requested_model, complexity, context)
    RTR->>CCH: check_cached_pricing()
    CCH-->>RTR: pricing_data
    RTR-->>TO: routing_decision
    
    TO-->>App: OptimizationResult {
        optimized_context,
        tokens_saved,
        savings_pct,
        routing_decision,
        cost_estimate,
        summary
    }
```

---

## ⚡ Optimization Strategies Visualized

### Context Selection Strategies

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#00D4AA'}}}%%
graph LR
    subgraph "RecentFirst"
        R1[Msg 1] -.-> R2[Msg 2] -.-> R3[Msg 3] --> R4[Msg 4 ✓] --> R5[Msg 5 ✓]
    end
    
    subgraph "RelevanceFirst"
        V1[Msg 1] -.-> V2[Msg 2 ✓] -.-> V3[Msg 3] --> V4[Msg 4 ✓] -.-> V5[Msg 5]
    end
    
    subgraph "ImportanceFirst"
        I1[Msg 1 ⭐] --> I2[Msg 2] -.-> I3[Msg 3 ⭐] --> I4[Msg 4] -.-> I5[Msg 5 ⭐]
    end
    
    subgraph "Balanced"
        B1[Msg 1] --> B2[Msg 2] --> B3[Msg 3 ✓] --> B4[Msg 4 ✓] --> B5[Msg 5 ✓]
    end
    
    subgraph "SlidingWindow"
        S1[Msg 1] --> S2[Msg 2] --> S3[Summary<br/>Msgs 1-2] --> S4[Msg 3] --> S5[Msg 4] --> S6[Msg 5]
    end
    
    style R4 fill:#00D4AA,color:#0a0f1a
    style R5 fill:#00D4AA,color:#0a0f1a
    style V2 fill:#00D4AA,color:#0a0f1a
    style V4 fill:#00D4AA,color:#0a0f1a
    style I1 fill:#00D4AA,color:#0a0f1a
    style I3 fill:#00D4AA,color:#0a0f1a
    style I5 fill:#00D4AA,color:#0a0f1a
    style B3 fill:#00D4AA,color:#0a0f1a
    style B4 fill:#00D4AA,color:#0a0f1a
    style B5 fill:#00D4AA,color:#0a0f1a
    style S3 fill:#FFA500,color:#0a0f1a
```

### Model Routing Decision Tree

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#00D4AA'}}}%%
flowchart TD
    START([Request Received]) --> COMPLEXITY{Task Complexity}
    
    COMPLEXITY -->|Simple| SIMPLE[Use Cheapest Model<br/>GPT-4o-mini / Haiku]
    COMPLEXITY -->|Moderate| MODERATE{Strategy?}
    COMPLEXITY -->|Complex| COMPLEX[Use Best Model<br/>GPT-4o / Sonnet / Opus]
    
    MODERATE -->|Cost Optimized| COST[Cheapest Capable<br/>GPT-4o-mini]
    MODERATE -->|Speed Optimized| SPEED[Fastest Capable<br/>GPT-4o Flash]
    MODERATE -->|Balanced| BALANCED[Best Price/Perf<br/>GPT-4o]
    MODERATE -->|Cascade| CASCADE[Try Cheap → Escalate<br/>if quality low]
    
    SIMPLE --> RESULT[Routing Decision]
    COST --> RESULT
    SPEED --> RESULT
    BALANCED --> RESULT
    CASCADE --> ESCALATE{Quality OK?}
    ESCALATE -->|Yes| RESULT
    ESCALATE -->|No| COMPLEX
    COMPLEX --> RESULT
    
    style START fill:#00D4AA,color:#0a0f1a
    style RESULT fill:#00D4AA,color:#0a0f1a
    style CASCADE fill:#FFA500,color:#0a0f1a
```

---

## 📊 Performance Benchmarks

<div align="center">

| Operation | Target | Typical | Throughput |
|-----------|--------|---------|------------|
| Token Counting (1KB) | <0.1ms | ~0.05ms | **50,000 tok/ms** |
| Token Counting (100KB) | <2ms | ~1ms | **100,000 tok/ms** |
| Conversation (100 msgs) | <5ms | ~2ms | — |
| Context Selection (100 msgs) | <10ms | ~5ms | — |
| Prompt Optimization (500 chars) | <5ms | ~2ms | — |
| Full Optimization (50 msgs) | <100ms | ~50ms | — |
| Summarization (100 msgs) | <100ms | ~60ms | — |

**Memory**: <10MB base, scales with cache size  
**Zero-copy overhead**: <0.1ms per FFI call

</div>

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#00D4AA'}}}%%
xychart-beta
    title "TokenOpt vs Naive Token Usage"
    x-axis ["Code Review", "Doc Gen", "Test Gen", "Refactor", "Debug"]
    y-axis "Tokens Used" 0 --> 50000
    bar [12000, 8000, 15000, 10000, 18000]
    bar [35000, 25000, 40000, 30000, 45000]
    legend ["With TokenOpt", "Without Optimization"]
```

---

## 🤖 Supported Models & Providers

<div align="center">

| Provider | Models | Pricing | Context |
|----------|--------|---------|---------|
| **OpenAI** | GPT-4o, GPT-4o-mini, GPT-4-turbo, GPT-3.5-turbo | ✅ Live | 128K |
| **Anthropic** | Claude 3.5 Sonnet, Haiku, Opus | ✅ Live | 200K |
| **Google** | Gemini 1.5 Pro, Flash, Flash-8B | ✅ Live | 2M |
| **Cohere** | Command R+, Command R | ✅ Live | 128K |
| **GitHub** | Copilot (GPT-4, GPT-4o) | ✅ Subscription | 128K |
| **Custom** | Any OpenAI-compatible API | ✅ Configurable | Custom |

</div>

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#00D4AA'}}}%%
pie showData
    title "Model Usage Distribution (Typical)"
    "GPT-4o" : 35
    "GPT-4o-mini" : 25
    "Claude 3.5 Sonnet" : 20
    "Gemini 1.5 Flash" : 10
    "Other" : 10
```

---

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace --all-features

# Run with nextest (faster)
cargo nextest run --workspace --all-features

# Property-based tests
cargo test --workspace --all-features proptest

# Benchmarks
cargo bench --workspace --all-features

# Coverage
cargo tarpaulin --workspace --all-features --out Html

# Lint & format
cargo clippy --workspace --all-features -- -D warnings
cargo fmt --all --check
```

---

## 🤝 Contributing

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#00D4AA'}}}%%
graph LR
    A[Fork Repo] --> B[Create Branch]
    B --> C[Make Changes]
    C --> D[Add Tests]
    D --> E[Run CI]
    E --> F[Open PR]
    F --> G[Code Review]
    G --> H[Merge]
    
    style A fill:#1a1f2e,stroke:#00D4AA
    style H fill:#00D4AA,color:#0a0f1a
```

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for development workflow, code style, testing requirements, and performance standards.

---

## 🔒 Security

See [SECURITY.md](docs/SECURITY.md) for vulnerability reporting, security measures, and bug bounty program.

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#00D4AA'}}}%%
timeline
    title Security Lifecycle
    2024 Q1 : Dependency Audit\nSBOM Generation
    2024 Q2 : Penetration Testing\nFuzzing Integration
    2024 Q3 : Supply Chain Hardening\nSigstore Signing
    2024 Q4 : Formal Verification\nCritical Path Proofs
```

---

## 📄 License

Licensed under either of:
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

## 🙏 Acknowledgments

<div align="center">

| Project | Contribution |
|---------|--------------|
| [tiktoken-rs](https://github.com/pico-dev/tiktoken-rs) | Blazing fast token counting |
| [OpenAI](https://openai.com/) | Tokenization standards |
| [Anthropic](https://anthropic.com/) | Claude model family |
| [NAPI-RS](https://napi.rs/) | Zero-cost TypeScript bindings |
| [PyO3](https://pyo3.rs/) | Seamless Python bindings |
| All Contributors | Making this possible ❤️ |

</div>

---

## 🔗 Links & Community

<div align="center">

[![Crates.io](https://img.shields.io/badge/Crates.io-tokenopt__core-orange?style=for-the-badge&logo=rust)](https://crates.io/crates/tokenopt-core)
[![npm](https://img.shields.io/badge/npm-@tokenopt%2Fcore-red?style=for-the-badge&logo=npm)](https://www.npmjs.com/package/@tokenopt/core)
[![PyPI](https://img.shields.io/badge/PyPI-tokenopt-blue?style=for-the-badge&logo=python)](https://pypi.org/project/tokenopt/)
[![GitHub](https://img.shields.io/badge/GitHub-TokenOpt-black?style=for-the-badge&logo=github)](https://github.com/yourorg/TokenOpt)
[![Discord](https://img.shields.io/badge/Discord-Join%20Us-5865F2?style=for-the-badge&logo=discord)](https://discord.gg/tokenopt)
[![Twitter](https://img.shields.io/badge/Twitter-@TokenOpt-1DA1F2?style=for-the-badge&logo=twitter)](https://twitter.com/TokenOpt)

**Documentation**: [docs.tokenopt.dev](https://docs.tokenopt.dev)  
**Playground**: [play.tokenopt.dev](https://play.tokenopt.dev)  
**Benchmarks**: [bench.tokenopt.dev](https://bench.tokenopt.dev)

</div>

---

<div align="center">

**Built with ❤️ for the AI Coding Revolution**

*TokenOpt — Because every token counts in the age of AI.*

![Powered by](https://img.shields.io/badge/Powered%20by-Rust%20%7C%20NAPI-RS%20%7C%20PyO3-00D4AA?style=flat-square)

</div>

---

<details>
<summary><b>📖 Full Documentation Index</b></summary>

| Document | Description |
|----------|-------------|
| [Architecture](docs/ARCHITECTURE.md) | System design, components, data flow, technical decisions |
| [API Reference](docs/API_REFERENCE.md) | Complete API docs for Rust, TypeScript, Python, CLI |
| [User Guide](docs/USER_GUIDE.md) | Comprehensive guide with examples & best practices |
| [Advanced Examples](docs/ADVANCED_EXAMPLES.md) | Code review, doc gen, test gen, refactoring, debugging |
| [Benchmarks](docs/BENCHMARKS.md) | Performance benchmarks, targets, regression detection |
| [Security](docs/SECURITY.md) | Security policy, vulnerability reporting, measures |
| [Contributing](docs/CONTRIBUTING.md) | Development workflow, code style, testing |
| [Changelog](docs/CHANGELOG.md) | Version history & release notes |
| [Code of Conduct](docs/CODE_OF_CONDUCT.md) | Community standards & enforcement |

</details>