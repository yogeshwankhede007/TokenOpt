# TokenOpt Architecture

## Overview

TokenOpt is a high-performance, multi-language token optimization library designed for LLM coding assistants. It provides a unified interface for token counting, context optimization, prompt engineering, model routing, and conversation summarization across all major LLM providers.

## Design Principles

1. **Performance First** - Rust core with zero-cost abstractions
2. **Provider Agnostic** - Works with any LLM provider
3. **Type Safety** - Strong typing across all language bindings
4. **Observability** - Built-in metrics, tracing, and logging
5. **Extensibility** - Plugin architecture for custom strategies
6. **Reliability** - Comprehensive testing, benchmarking, and monitoring

## System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Language Bindings Layer                             │
├──────────────┬──────────────┬──────────────┬──────────────┬────────────────┤
│   Rust       │  TypeScript  │   Python     │     Go       │      CLI       │
│  (Native)    │  (NAPI-RS)   │   (PyO3)     │   (CGO)      │   (StructOpt)  │
└──────┬───────┴──────┬───────┴──────┬───────┴──────┬───────┴───────┬────────┘
       │              │              │              │               │
       └──────────────┴──────────────┴──────────────┴───────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           FFI Boundary (C API)                              │
│         cbindgen-generated headers │ cxx.rs bridge │ PyO3 modules          │
└─────────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Core Library (Rust)                               │
├─────────────┬─────────────┬─────────────┬─────────────┬────────────────────┤
│  Counter    │  Context    │  Optimizer  │  Prompt     │  Router            │
│  (tiktoken) │  Selector   │  Pipeline   │  Optimizer  │  (Decision Engine) │
├─────────────┼─────────────┼─────────────┼─────────────┼────────────────────┤
│  Summarizer │   Cache     │  Models     │  Metrics    │  Config            │
│  (Multi-str)│  (LRU/TTL)  │  Registry   │  (OpenTel)  │  (TOML/JSON)       │
└─────────────┴─────────────┴─────────────┴─────────────┴────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           External Dependencies                             │
├─────────────────────┬─────────────────────┬─────────────────────────────────┤
│  tiktoken-rs        │  xxhash-rust        │  OpenTelemetry                  │
│  (Tokenization)     │  (Fast Hashing)     │  (Observability)                │
├─────────────────────┼─────────────────────┼─────────────────────────────────┤
│  dashmap            │  parking_lot        │  serde                          │
│  (Concurrent Map)   │  (Fast Locks)       │  (Serialization)                │
├─────────────────────┼─────────────────────┼─────────────────────────────────┤
│  lru                │  similar            │  textwrap                       │
│  (Cache Eviction)   │  (Diff Algorithm)   │  (Text Formatting)              │
└─────────────────────┴─────────────────────┴─────────────────────────────────┘
```

## Core Components

### 1. Token Counter (`counter.rs`)

**Responsibility**: Accurate token counting for all supported models.

**Key Features**:
- Uses `tiktoken-rs` for OpenAI models (exact)
- Model aliasing for Anthropic, Google, Cohere, GitHub Copilot
- Conversation-level counting with role-based separation
- Template variable counting
- Cost estimation integration

**Performance**: ~50K tokens/ms single-threaded

**API**:
```rust
pub struct TokenCounter {
    pub fn count_tokens(&self, text: &str, model: Option<&str>) -> Result<usize>
    pub fn count_conversation(&self, messages: &[Message], model: Option<&str>) -> Result<TokenCount>
    pub fn count_template(&self, template: &str, vars: &HashMap<String, String>, model: Option<&str>) -> Result<usize>
    pub fn estimate_cost(&self, count: &TokenCount, pricing: Option<&ModelPricing>) -> Option<f64>
}
```

### 2. Model Registry (`models.rs`)

**Responsibility**: Centralized model metadata, capabilities, and pricing.

**Data Model**:
```rust
pub struct ModelCapabilities {
    id: String,
    name: String,
    provider: String,
    context_window: usize,
    max_output_tokens: usize,
    pricing: Option<ModelPricing>,
    features: ModelFeatures,
    family: String,
    is_fast_variant: bool,
    knowledge_cutoff: Option<String>,
}
```

**Features**:
- 50+ pre-configured models
- Pricing tiers (Free → UltraPremium)
- Feature matrix (function calling, vision, JSON mode, etc.)
- Provider/family grouping
- Dynamic registration for custom models

### 3. Context Selector (`context.rs`)

**Responsibility**: Select optimal conversation context within token budget.

**Strategies**:
| Strategy | Description | Use Case |
|----------|-------------|----------|
| `RecentFirst` | Most recent messages | Chat continuity |
| `RelevanceFirst` | Semantic similarity to query | RAG, search |
| `ImportanceFirst` | Heuristic importance scoring | Long conversations |
| `Balanced` | Weighted combination | General purpose |
| `Weighted` | Custom weights | Domain-specific |
| `SlidingWindow` | Fixed window + summaries | Streaming |

**Algorithm**:
1. Score all messages (importance, relevance, recency)
2. Sort by strategy
3. Select within token budget
4. Generate summary of omitted messages
5. Return `SelectedContext` with metadata

### 4. Prompt Optimizer (`prompt.rs`)

**Responsibility**: Reduce prompt tokens while preserving intent.

**Techniques**:
- Redundancy removal (politeness, filler words)
- Instruction compression ("step by step" → "stepwise")
- Language simplification ("in order to" → "to")
- Example removal (configurable)
- Synonym replacement ("utilize" → "use")
- Format restructuring (markdown → plain text)

**Aggressiveness Levels**:
- `Conservative`: Safe optimizations only (~10-15% reduction)
- `Balanced`: Good balance (~20-35% reduction)
- `Aggressive`: Maximum reduction (~35-50% reduction)

### 5. Model Router (`router.rs`)

**Responsibility**: Intelligent model selection based on task requirements.

**Routing Strategies**:
- `Fixed`: Always use requested model
- `CostOptimized`: Cheapest suitable model
- `SpeedOptimized`: Fastest suitable model
- `Balanced`: Cost/performance tradeoff
- `Cascade`: Try cheap, escalate on failure
- `Custom`: User-defined logic

**Decision Factors**:
- Task complexity (Simple/Moderate/Complex)
- Required features (function calling, vision, etc.)
- Model pricing tier
- Historical performance (latency, success rate)
- Context window requirements

### 6. Conversation Summarizer (`summarizer.rs`)

**Responsibility**: Compress conversations while preserving key information.

**Strategies**:
| Strategy | Description | Compression |
|----------|-------------|-------------|
| `Extractive` | Select key sentences | 60-80% |
| `Abstractive` | Generate new summary | 80-95% |
| `Hybrid` | Extractive + compress | 70-90% |
| `Rolling` | Fixed window + summary | Configurable |
| `Hierarchical` | Multi-level summaries | 80-95% |

**Key Point Categories**:
- Decision, Error, Code, Requirement, Question, Answer, Context, Other

### 7. Cache (`cache.rs`)

**Responsibility**: High-performance caching for computed results.

**Features**:
- LRU eviction with TTL
- Namespace/tag-based invalidation
- xxHash64 for fast key generation
- bincode serialization
- Hit/miss/eviction statistics
- Thread-safe (DashMap + RwLock)

**Specialized Caches**:
- `TokenCountCache`: Per-text token counts
- `ConversationCountCache`: Conversation-level counts
- Generic `TokenCache`: Arbitrary values

### 8. Optimizer Pipeline (`optimizer.rs`)

**Responsibility**: Orchestrate all optimization strategies.

**Pipeline**:
```
Input Conversation
       │
       ▼
┌──────────────────┐
│  Token Counting  │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ Context Selection│ (if over budget)
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Summarization   │ (if very long)
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ Prompt Optimize  │ (last user message)
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Model Routing   │ (if enabled)
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│   Cost Estimate  │
└────────┬─────────┘
         │
         ▼
OptimizationResult
```

## Data Flow

### Token Counting Flow
```
User Input (text/messages)
        │
        ▼
Model Resolution (alias → actual)
        │
        ▼
Tokenizer Selection (cached)
        │
        ▼
Encoding (special tokens + content)
        │
        ▼
Count + Metadata
        │
        ▼
Cache Storage (if enabled)
```

### Optimization Flow
```
Conversation + Query + Model
        │
        ▼
TokenCounter.count_conversation()
        │
        ▼
ContextSelector.select() ──→ SelectedContext
        │
        ▼
Summarizer.summarize() (if needed) ──→ SummaryResult
        │
        ▼
PromptOptimizer.optimize() ──→ OptimizedPrompt
        │
        ▼
ModelRouter.route() ──→ RoutingDecision
        │
        ▼
Cost Estimation
        │
        ▼
OptimizationResult (aggregated)
```

## Concurrency Model

- **TokenCounter**: Thread-safe, uses `DashMap` for tokenizer cache
- **Cache**: `DashMap` + `RwLock` for statistics
- **Registry**: Immutable after initialization (`Arc<ModelRegistry>`)
- **Optimizer**: Stateless pipeline, shared via `Arc`
- **Router**: Mutable performance cache, `&mut self` for recording

## Error Handling

- Typed errors per module (`CounterError`, `SelectionError`, etc.)
- `thiserror` for automatic `From` implementations
- `anyhow` for application-level errors
- Graceful degradation (fallback strategies)
- Structured error context for debugging

## Observability

### Metrics (Prometheus/OpenTelemetry)
- `tokenopt_tokens_counted_total` - Counter
- `tokenopt_optimization_duration_seconds` - Histogram
- `tokenopt_cache_hit_ratio` - Gauge
- `tokenopt_model_routing_decisions_total` - Counter by strategy
- `tokenopt_cost_savings_usd_total` - Counter

### Tracing
- `tokenopt.optimize` - Full pipeline span
- `tokenopt.count` - Token counting span
- `tokenopt.select` - Context selection span
- `tokenopt.summarize` - Summarization span

### Logging
- Structured JSON logging (tracing-subscriber)
- Configurable levels (ERROR/WARN/INFO/DEBUG/TRACE)
- Correlation IDs for request tracing

## Configuration

### File-based (TOML)
```toml
# tokenopt.toml
[counter]
default_model = "gpt-4o"
enable_cache = true
max_cache_size = 10000

[optimizer]
context_config = { strategy = "Balanced", max_tokens = 8000 }
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

[router]
default_strategy = "Balanced"
```

### Programmatic
```rust
let tokenopt = TokenOptBuilder::new()
    .counter_config(TokenCounterConfig { ... })
    .cache_config(CacheConfig { ... })
    .add_model(custom_model)
    .build()?;
```

## Security

### Threat Model
- **Input Validation**: All user inputs validated before processing
- **Resource Limits**: Max tokens, max messages, max cache size
- **No Code Execution**: Pure data processing, no eval/exec
- **Dependency Scanning**: Automated via `cargo audit` and `cargo deny`

### Data Handling
- No persistent storage by default
- Optional disk cache with encryption support
- No telemetry without explicit opt-in
- PII detection in conversation content (optional)

## Performance Targets

| Operation | Target | Current |
|-----------|--------|---------|
| Token counting (10K tokens) | <1ms | ~0.2ms |
| Context selection (100 msgs) | <5ms | ~2ms |
| Prompt optimization | <10ms | ~5ms |
| Summarization (100 msgs) | <50ms | ~30ms |
| Full optimization | <100ms | ~60ms |
| Memory (idle) | <10MB | ~8MB |
| Memory (10K cached) | <50MB | ~35MB |

## Scalability

- Horizontal scaling via stateless design
- Cache can be externalized (Redis)
- Registry supports dynamic model updates
- Pipeline stages independently scalable

## Deployment

### Library
```toml
[dependencies]
tokenopt-core = { version = "0.1", features = ["full"] }
```

### CLI
```bash
cargo install tokenopt-cli
# or download from releases
```

### Docker
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin tokenopt

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/tokenopt /usr/local/bin/
ENTRYPOINT ["tokenopt"]
```

## Future Extensions

1. **Streaming Optimization** - Incremental context management
2. **Semantic Caching** - Embedding-based cache keys
3. **Adaptive Routing** - ML-based model selection
4. **Multi-modal Support** - Image/video token counting
5. **Custom Tokenizers** - Plugin interface for proprietary models
6. **Distributed Cache** - Redis/Valkey backend
7. **Policy Engine** - Organizational token budgets
8. **Audit Trail** - Compliance logging

## References

- [OpenAI Tokenization](https://github.com/openai/tiktoken)
- [Anthropic Claude Models](https://docs.anthropic.com/claude/docs/models-overview)
- [Google Gemini](https://ai.google.dev/models/gemini)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [OpenTelemetry Rust](https://opentelemetry.io/docs/instrumentation/rust/)