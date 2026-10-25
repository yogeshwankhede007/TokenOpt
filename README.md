# TokenOpt - Universal Token Optimization for LLM Coding Assistants

[![Crates.io](https://img.shields.io/crates/v/tokenopt-core.svg)](https://crates.io/crates/tokenopt-core)
[![npm](https://img.shields.io/npm/v/@tokenopt/core.svg)](https://www.npmjs.com/package/@tokenopt/core)
[![PyPI](https://img.shields.io/pypi/v/tokenopt.svg)](https://pypi.org/project/tokenopt/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/yourorg/tokenoptimization/workflows/CI/badge.svg)](https://github.com/yourorg/tokenoptimization/actions)
[![Docs](https://docs.rs/tokenopt-core/badge.svg)](https://docs.rs/tokenopt-core)
[![Coverage](https://img.shields.io/codecov/c/github/yourorg/tokenoptimization)](https://codecov.io/gh/yourorg/tokenoptimization)

TokenOpt is a high-performance, multi-language library for optimizing token usage across LLM coding assistants like **Claude Code**, **GitHub Copilot**, **ChatGPT**, and any other LLM-based coding tool.

## 🚀 Features

- 🔢 **Accurate Token Counting** - Using tiktoken for OpenAI models, with approximations for Anthropic, Google, and other providers
- 🎯 **Context Selection** - Smart strategies to fit conversations within token budgets (recency, relevance, importance, balanced)
- ✂️ **Prompt Optimization** - Reduce prompt tokens by 20-50% while preserving intent
- 🧠 **Conversation Summarization** - Extractive, abstractive, rolling, and hierarchical summarization
- 🔄 **Model Routing** - Automatically route to cheaper/faster models based on task complexity
- 💰 **Cost Estimation** - Real-time cost calculations with caching
- ⚡ **High Performance** - Rust core with zero-copy bindings for TypeScript, Python, and Go
- 🔌 **Universal** - Works with any LLM provider (OpenAI, Anthropic, Google, Cohere, GitHub Copilot, etc.)

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [Architecture](docs/ARCHITECTURE.md) | System design, components, data flow, and technical decisions |
| [API Reference](docs/API_REFERENCE.md) | Complete API documentation for Rust, TypeScript, Python, and CLI |
| [User Guide](docs/USER_GUIDE.md) | Comprehensive guide with examples and best practices |
| [Advanced Examples](docs/ADVANCED_EXAMPLES.md) | Real-world use cases: code review, doc gen, test gen, refactoring, debugging |
| [Benchmarks](docs/BENCHMARKS.md) | Performance benchmarks, targets, and regression detection |
| [Security](docs/SECURITY.md) | Security policy, vulnerability reporting, and measures |
| [Contributing](docs/CONTRIBUTING.md) | Development workflow, code style, testing requirements |
| [Changelog](docs/CHANGELOG.md) | Version history and release notes |
| [Code of Conduct](docs/CODE_OF_CONDUCT.md) | Community standards and enforcement |
| [Examples](examples/) | Runnable examples in Rust, TypeScript, and Python |

## 🚀 Quick Start

### Rust (Core Library)

```toml
[dependencies]
tokenopt-core = { version = "0.1", features = ["full"] }
```

```rust
use tokenopt_core::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokenopt = TokenOpt::new()?;
    
    // Count tokens
    let count = tokenopt.counter().count_tokens("Hello, world!", Some("gpt-4"))?;
    println!("Tokens: {}", count);
    
    // Optimize a conversation
    let messages = vec![
        Message { role: Role::User, content: "Please write a binary search in Rust".to_string(), ..Default::default() },
        Message { role: Role::Assistant, content: "Here's the implementation...".to_string(), ..Default::default() },
    ];
    
    let result = tokenopt.optimizer().optimize_conversation(&messages, None, Some("gpt-4"))?;
    println!("Saved {} tokens ({:.1}%)", result.tokens_saved, result.savings_percentage);
    
    Ok(())
}
```

### TypeScript/JavaScript

```bash
npm install @tokenopt/core
# or
yarn add @tokenopt/core
# or
pnpm add @tokenopt/core
```

```typescript
import { TokenOpt, TokenCounter, TokenOptimizer, PromptTask } from '@tokenopt/core';

const tokenopt = new TokenOpt();

// Count tokens
const count = tokenopt.counter.countTokens("Hello, world!", "gpt-4");
console.log(`Tokens: ${count}`);

// Optimize a prompt
const optimized = tokenopt.promptOptimizer.optimizeForTask(
  PromptTask.CodeGeneration,
  "Create a REST API endpoint with authentication",
  "gpt-4"
);
console.log(`Saved ${optimized.tokensSaved} tokens (${optimized.savingsPercentage}%)`);
console.log(optimized.prompt);

// Route to best model
const decision = tokenopt.router.route("gpt-4", "moderate", messages);
console.log(`Use ${decision.selectedModel} instead of ${decision.requestedModel}`);
console.log(decision.reasoning);
```

### Python

```bash
pip install tokenopt
```

```python
from tokenopt import TokenOpt, TokenCounter, TokenOptimizer, PromptTask

tokenopt = TokenOpt()

# Count tokens
count = tokenopt.counter.count_tokens("Hello, world!", "gpt-4")
print(f"Tokens: {count}")

# Optimize conversation
messages = [
    {"role": "user", "content": "Please write a binary search in Rust"},
    {"role": "assistant", "content": "Here's the implementation..."},
]
result = tokenopt.optimizer.optimize_conversation(messages, model="gpt-4")
print(f"Saved {result.tokens_saved} tokens ({result.savings_percentage:.1f}%)")

# Summarize long conversation
summary = tokenopt.summarizer.summarize(messages, strategy="extractive", max_tokens=500)
print(summary.summary)
```

### Go (Planned)

```bash
go get github.com/tokenopt/tokenopt-go
```

### CLI Tool

```bash
cargo install tokenopt-cli
# or download binary from releases
```

```bash
# Count tokens in a file
tokenopt count --file conversation.json --conversation

# Optimize a conversation
tokenopt optimize --file conversation.json --model gpt-4 --output optimized.json

# Optimize a prompt
tokenopt prompt --text "Please write a function that..." --model gpt-4 --aggressiveness balanced

# Route to best model
tokenopt route --model gpt-4 --file conversation.json --complexity complex --strategy cost-optimized

# Summarize conversation
tokenopt summarize --file conversation.json --strategy extractive --max-tokens 1000

# List models
tokenopt models --provider openai --tier cheap

# Estimate cost
tokenopt cost --model gpt-4 --prompt-tokens 1000 --completion-tokens 500
```

## 🏗 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Language Bindings                        │
├──────────────┬──────────────┬──────────────┬────────────────┤
│  TypeScript  │   Python     │     Go       │      CLI       │
│  (NAPI-RS)   │   (PyO3)     │   (CGO)      │   (Rust CLI)   │
└──────┬───────┴──────┬───────┴──────┬───────┴───────┬────────┘
       │              │              │               │
       └──────────────┴──────────────┴───────────────┘
                           │
               ┌───────────▼───────────┐
               │   tokenopt-core       │
               │    (Rust Library)     │
               ├───────────────────────┤
               │ • Token Counter       │
               │ • Context Selector    │
               │ • Prompt Optimizer    │
               │ • Model Router        │
               │ • Summarizer          │
               │ • Cache               │
               └───────────────────────┘
```

## ⚡ Optimization Strategies

### Context Selection
- **RecentFirst** - Keep most recent messages
- **RelevanceFirst** - Keep messages most relevant to current query
- **ImportanceFirst** - Keep messages with highest importance scores
- **Balanced** - Weighted combination of recency, relevance, importance
- **Weighted** - Custom weights for each factor
- **SlidingWindow** - Fixed window + periodic summaries

### Prompt Optimization
- **Conservative** - Safe optimizations only (remove filler words)
- **Balanced** - Good balance of savings vs. quality
- **Aggressive** - Maximum token reduction (compress examples, shorten synonyms)

### Model Routing
- **Fixed** - Always use requested model
- **CostOptimized** - Cheapest suitable model
- **SpeedOptimized** - Fastest suitable model
- **Balanced** - Best cost/performance tradeoff
- **Cascade** - Try cheap model first, escalate if needed

### Summarization
- **Extractive** - Select key sentences/messages
- **Abstractive** - Generate new summary (requires LLM)
- **Hybrid** - Extractive then compress
- **Rolling** - Maintain running summary window
- **Hierarchical** - Multi-level summaries

## ⚙️ Configuration

```rust
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
```

Or via `tokenopt.toml`:

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
```

## 🤖 Supported Models

| Provider | Models | Pricing |
|----------|--------|---------|
| OpenAI | GPT-4o, GPT-4o-mini, GPT-4-turbo, GPT-3.5-turbo | ✅ |
| Anthropic | Claude 3.5 Sonnet, Haiku, Opus | ✅ |
| Google | Gemini 1.5 Pro, Flash | ✅ |
| Cohere | Command R+ | ✅ |
| GitHub | Copilot (GPT-4, GPT-4o) | ✅ (subscription) |

## 📊 Performance

| Operation | Target | Typical |
|-----------|--------|---------|
| Token Counting (1KB) | <0.1ms | ~0.05ms |
| Token Counting (100KB) | <2ms | ~1ms |
| Conversation Counting (100 msgs) | <5ms | ~2ms |
| Context Selection (100 msgs) | <10ms | ~5ms |
| Prompt Optimization (500 chars) | <5ms | ~2ms |
| Full Optimization (50 msgs) | <100ms | ~50ms |
| Summarization (100 msgs) | <100ms | ~60ms |

- **Memory**: <10MB base, scales with cache size
- **Throughput**: ~50,000 tokens/ms (single-threaded)

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
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](docs/CONTRIBUTING.md) for details on:
- Development workflow
- Code style guidelines
- Testing requirements
- Documentation standards
- Performance requirements

## 🔒 Security

Please see our [Security Policy](docs/SECURITY.md) for:
- Supported versions
- Vulnerability reporting
- Security measures
- Bug bounty program

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- [tiktoken-rs](https://github.com/pico-dev/tiktoken-rs) for fast token counting
- [OpenAI](https://openai.com/) for tokenization standards
- [Anthropic](https://anthropic.com/) for Claude models
- All contributors and users

---

**Links**: [Crates.io](https://crates.io/crates/tokenopt-core) • [npm](https://www.npmjs.com/package/@tokenopt/core) • [PyPI](https://pypi.org/project/tokenopt/) • [GitHub](https://github.com/yourorg/tokenoptimization) • [Discord](https://discord.gg/tokenopt)