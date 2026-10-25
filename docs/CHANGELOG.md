# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive CI/CD pipeline with GitHub Actions
- Property-based testing with proptest
- Performance benchmarks with Criterion
- Security auditing with cargo-audit and cargo-deny
- Architecture documentation (ARCHITECTURE.md)
- Contributing guidelines (CONTRIBUTING.md)
- Code of Conduct (CODE_OF_CONDUCT.md)

### Changed
- Improved test coverage to 90%+
- Enhanced error handling across all modules
- Optimized cache performance with xxHash64

### Fixed
- Model alias resolution for "default" model
- Pricing tier classification for GPT-4o
- Context selector model resolution

## [0.1.0] - 2024-01-15

### Added
- **Core Library** (tokenopt-core)
  - Token counting with tiktoken-rs integration
  - Model registry with 50+ pre-configured models
  - Context selection with 6 strategies
  - Prompt optimization with 3 aggressiveness levels
  - Model routing with 5 strategies
  - Conversation summarization with 5 strategies
  - LRU/TTL caching with xxHash64
  - Full optimization pipeline orchestration

- **Language Bindings**
  - Rust (native)
  - TypeScript/Node.js (NAPI-RS)
  - Python (PyO3/maturin)
  - Go (CGO) - planned

- **CLI Tool** (tokenopt-cli)
  - Token counting (text/conversation)
  - Conversation optimization
  - Prompt optimization
  - Model routing
  - Conversation summarization
  - Model listing with filters
  - Cost estimation
  - Cache management

- **Models Supported**
  - OpenAI: GPT-4, GPT-4 Turbo, GPT-4o, GPT-4o-mini, GPT-3.5-turbo
  - Anthropic: Claude 3 Opus, Sonnet, Haiku, 3.5 Sonnet, 3.5 Haiku
  - Google: Gemini 1.5 Pro, Flash
  - Cohere: Command R+
  - GitHub Copilot: GPT-4, GPT-4o

- **Context Selection Strategies**
  - RecentFirst, RelevanceFirst, ImportanceFirst
  - Balanced (weighted combination)
  - Weighted (custom weights)
  - SlidingWindow (fixed window + summaries)

- **Prompt Optimization Techniques**
  - Redundancy removal
  - Instruction compression
  - Language simplification
  - Example removal
  - Synonym replacement
  - Format restructuring

- **Model Routing Strategies**
  - Fixed, CostOptimized, SpeedOptimized
  - Balanced, Cascade
  - Custom rules with priorities

- **Summarization Strategies**
  - Extractive, Abstractive, Hybrid
  - Rolling, Hierarchical
  - Key point categorization

- **Caching**
  - Generic TokenCache (LRU + TTL)
  - Specialized TokenCountCache
  - Tag/namespace invalidation
  - Hit rate statistics

- **Cost Estimation**
  - Per-model pricing
  - Input/output/cache pricing
  - Real-time cost calculation
  - Savings estimation

### Security
- Dependency auditing with cargo-audit
- License compliance with cargo-deny
- No unsafe code in core library
- Input validation on all public APIs

### Testing
- 30 unit tests passing
- Integration tests for all components
- Property-based tests for algorithms
- Benchmark suite with Criterion

## [0.0.1] - 2024-01-01

### Added
- Initial project structure
- Workspace configuration
- Basic token counting
- Model registry skeleton