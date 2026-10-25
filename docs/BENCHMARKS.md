# Benchmark Suite

Comprehensive performance benchmarks for TokenOpt core operations.

## Benchmark Categories

```rust
// benches/token_counting.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use tokenopt_core::prelude::*;

fn bench_token_counting(c: &mut Criterion) {
    let tokenopt = TokenOpt::new().unwrap();
    let counter = tokenopt.counter();
    
    let texts = [
        ("small", "Hello world"),
        ("medium", "This is a medium length text that contains several sentences. It should be representative of typical user queries."),
        ("large", include_str!("../test_data/large_text.txt")),
        ("code", include_str!("../test_data/sample_code.rs")),
    ];
    
    let mut group = c.benchmark_group("token_counting");
    
    for (name, text) in texts {
        group.throughput(Throughput::Bytes(text.len() as u64));
        group.bench_with_input(BenchmarkId::new("gpt-4", name), &text, |b, text| {
            b.iter(|| counter.count_tokens(black_box(text), Some("gpt-4")))
        });
        group.bench_with_input(BenchmarkId::new("gpt-4o", name), &text, |b, text| {
            b.iter(|| counter.count_tokens(black_box(text), Some("gpt-4o")))
        });
        group.bench_with_input(BenchmarkId::new("claude-3-sonnet", name), &text, |b, text| {
            b.iter(|| counter.count_tokens(black_box(text), Some("claude-3-sonnet")))
        });
    }
    
    group.finish();
}

fn bench_conversation_counting(c: &mut Criterion) {
    let tokenopt = TokenOpt::new().unwrap();
    let counter = tokenopt.counter();
    
    let conversations = vec![
        ("short", create_conversation(3)),
        ("medium", create_conversation(10)),
        ("long", create_conversation(50)),
        ("very_long", create_conversation(200)),
    ];
    
    let mut group = c.benchmark_group("conversation_counting");
    
    for (name, messages) in conversations {
        group.bench_with_input(BenchmarkId::new("gpt-4", name), &messages, |b, messages| {
            b.iter(|| counter.count_conversation(black_box(messages), Some("gpt-4")))
        });
    }
    
    group.finish();
}

fn bench_cached_counting(c: &mut Criterion) {
    let tokenopt = TokenOpt::new().unwrap();
    
    let texts = vec!["test"; 1000];
    
    c.bench_function("cached_count_cold", |b| {
        b.iter(|| {
            for text in &texts {
                tokenopt.optimizer().count_tokens_cached(text, Some("gpt-4")).unwrap();
            }
        })
    });
    
    // Warm up cache
    for text in &texts {
        tokenopt.optimizer().count_tokens_cached(text, Some("gpt-4")).unwrap();
    }
    
    c.bench_function("cached_count_warm", |b| {
        b.iter(|| {
            for text in &texts {
                tokenopt.optimizer().count_tokens_cached(text, Some("gpt-4")).unwrap();
            }
        })
    });
}

criterion_group!(benches, bench_token_counting, bench_conversation_counting, bench_cached_counting);
criterion_main!(benches);

fn create_conversation(num_messages: usize) -> Vec<Message> {
    (0..num_messages).map(|i| Message {
        role: if i % 2 == 0 { Role::User } else { Role::Assistant },
        content: format!("Message number {} with some content to make it realistic. This contains code examples and technical details.", i),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }).collect()
}
```

```rust
// benches/context_selection.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use tokenopt_core::prelude::*;

fn bench_context_selection(c: &mut Criterion) {
    let tokenopt = TokenOpt::new().unwrap();
    let counter = tokenopt.counter();
    
    let conversations = vec![
        ("small", create_conversation(10)),
        ("medium", create_conversation(50)),
        ("large", create_conversation(200)),
        ("xlarge", create_conversation(1000)),
    ];
    
    let strategies = vec![
        ("RecentFirst", SelectionStrategy::RecentFirst),
        ("RelevanceFirst", SelectionStrategy::RelevanceFirst),
        ("ImportanceFirst", SelectionStrategy::ImportanceFirst),
        ("Balanced", SelectionStrategy::Balanced),
    ];
    
    let mut group = c.benchmark_group("context_selection");
    
    for (conv_name, messages) in conversations {
        for (strat_name, strategy) in &strategies {
            let config = ContextConfig {
                strategy: *strategy,
                max_tokens: 4000,
                ..Default::default()
            };
            let selector = ContextSelector::new(config, counter.clone());
            
            group.throughput(Throughput::Elements(messages.len() as u64));
            group.bench_with_input(
                BenchmarkId::new(format!("{}_{}", strat_name, conv_name), ""),
                &messages,
                |b, messages| {
                    b.iter(|| selector.select(black_box(messages), Some("test query"), None))
                }
            );
        }
    }
    
    group.finish();
}

criterion_group!(benches, bench_context_selection);
criterion_main!(benches);

fn create_conversation(num_messages: usize) -> Vec<Message> {
    (0..num_messages).map(|i| Message {
        role: if i % 2 == 0 { Role::User } else { Role::Assistant },
        content: format!("Message {} with technical content and code examples", i),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }).collect()
}
```

```rust
// benches/prompt_optimization.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use tokenopt_core::prelude::*;

fn bench_prompt_optimization(c: &mut Criterion) {
    let tokenopt = TokenOpt::new().unwrap();
    let optimizer = tokenopt.prompt_optimizer();
    
    let prompts = vec![
        ("simple", "Please write a hello world function in Rust"),
        ("verbose", "Could you please kindly explain in great detail how to implement a binary search algorithm in Rust step by step with comprehensive examples?"),
        ("code_review", "Please review this code and provide detailed feedback on performance, security, and maintainability issues:\n```rust\nfn process(data: &[u8]) -> Vec<u8> {\n    let mut result = Vec::new();\n    for byte in data {\n        result.push(byte.wrapping_add(1));\n    }\n    result\n}\n```"),
        ("complex", "I need you to act as a senior software architect and design a distributed system for real-time data processing that can handle 1 million events per second with exactly-once semantics, including detailed component diagrams, technology choices, and failure scenarios."),
    ];
    
    let aggressiveness_levels = vec![
        ("Conservative", AggressivenessLevel::Conservative),
        ("Balanced", AggressivenessLevel::Balanced),
        ("Aggressive", AggressivenessLevel::Aggressive),
    ];
    
    let mut group = c.benchmark_group("prompt_optimization");
    
    for (prompt_name, prompt) in prompts {
        for (level_name, level) in &aggressiveness_levels {
            let config = PromptOptimizationConfig {
                aggressiveness: *level,
                ..Default::default()
            };
            let optimizer = PromptOptimizer::new(tokenopt.counter().clone()).with_config(config);
            
            group.bench_with_input(
                BenchmarkId::new(format!("{}_{}", prompt_name, level_name), ""),
                &prompt,
                |b, prompt| {
                    b.iter(|| optimizer.optimize_string(black_box(prompt), "gpt-4"))
                }
            );
        }
    }
    
    group.finish();
}

criterion_group!(benches, bench_prompt_optimization);
criterion_main!(benches);
```

```rust
// benches/full_optimization.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use tokenopt_core::prelude::*;

fn bench_full_optimization(c: &mut Criterion) {
    let conversations = vec![
        ("short", create_conversation(5)),
        ("medium", create_conversation(20)),
        ("long", create_conversation(100)),
        ("very_long", create_conversation(500)),
    ];
    
    let mut group = c.benchmark_group("full_optimization");
    
    for (name, messages) in conversations {
        group.bench_with_input(
            BenchmarkId::new("gpt-4", name),
            &messages,
            |b, messages| {
                let tokenopt = TokenOpt::new().unwrap();
                let optimizer = tokenopt.optimizer();
                b.iter(|| optimizer.optimize_conversation(black_box(messages), None, Some("gpt-4")))
            }
        );
    }
    
    group.finish();
}

fn bench_model_routing(c: &mut Criterion) {
    let tokenopt = TokenOpt::new().unwrap();
    let router = tokenopt.router();
    
    let complexities = vec![
        ("Simple", TaskComplexity::Simple),
        ("Moderate", TaskComplexity::Moderate),
        ("Complex", TaskComplexity::Complex),
    ];
    
    let strategies = vec![
        ("Fixed", RoutingStrategy::Fixed),
        ("CostOptimized", RoutingStrategy::CostOptimized),
        ("SpeedOptimized", RoutingStrategy::SpeedOptimized),
        ("Balanced", RoutingStrategy::Balanced),
    ];
    
    let mut group = c.benchmark_group("model_routing");
    
    for (complexity_name, complexity) in complexities {
        for (strategy_name, strategy) in &strategies {
            let router = ModelRouter::new(tokenopt.registry().clone()).with_strategy(*strategy);
            let messages = create_conversation(5);
            
            group.bench_with_input(
                BenchmarkId::new(format!("{}_{}", complexity_name, strategy_name), ""),
                &messages,
                |b, messages| {
                    b.iter(|| router.route("gpt-4o", complexity, black_box(messages)))
                }
            );
        }
    }
    
    group.finish();
}

criterion_group!(benches, bench_full_optimization, bench_model_routing);
criterion_main!(benches);

fn create_conversation(num_messages: usize) -> Vec<Message> {
    (0..num_messages).map(|i| Message {
        role: if i % 2 == 0 { Role::User } else { Role::Assistant },
        content: format!("Message {} with technical content", i),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }).collect()
}
```

```rust
// benches/summarization.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use tokenopt_core::prelude::*;

fn bench_summarization(c: &mut Criterion) {
    let tokenopt = TokenOpt::new().unwrap();
    let summarizer = tokenopt.summarizer();
    
    let conversations = vec![
        ("small", create_conversation(20)),
        ("medium", create_conversation(100)),
        ("large", create_conversation(500)),
        ("xlarge", create_conversation(2000)),
    ];
    
    let strategies = vec![
        ("Extractive", SummaryStrategy::Extractive),
        ("Hybrid", SummaryStrategy::Hybrid),
        ("Rolling", SummaryStrategy::Rolling { window_size: 50 }),
    ];
    
    let mut group = c.benchmark_group("summarization");
    
    for (conv_name, messages) in conversations {
        for (strat_name, strategy) in &strategies {
            let config = SummaryConfig {
                strategy: *strategy,
                max_summary_tokens: 1000,
                ..Default::default()
            };
            
            group.bench_with_input(
                BenchmarkId::new(format!("{}_{}", strat_name, conv_name), ""),
                &messages,
                |b, messages| {
                    b.iter(|| summarizer.summarize(black_box(messages), &config))
                }
            );
        }
    }
    
    group.finish();
}

fn bench_rolling_summary(c: &mut Criterion) {
    let tokenopt = TokenOpt::new().unwrap();
    let summarizer = tokenopt.summarizer();
    
    let mut messages = create_conversation(1000);
    
    c.bench_function("rolling_summary_create", |b| {
        b.iter(|| summarizer.create_rolling_summary(black_box(&messages), 100))
    });
    
    let rolling = summarizer.create_rolling_summary(&messages, 100).unwrap();
    let new_messages = create_conversation(50);
    
    c.bench_function("rolling_summary_update", |b| {
        let mut rolling = rolling.clone();
        b.iter(|| summarizer.update_rolling_summary(black_box(&mut rolling), black_box(&new_messages)))
    });
}

criterion_group!(benches, bench_summarization, bench_rolling_summary);
criterion_main!(benches);

fn create_conversation(num_messages: usize) -> Vec<Message> {
    (0..num_messages).map(|i| Message {
        role: if i % 2 == 0 { Role::User } else { Role::Assistant },
        content: format!("Message {} with technical details and code examples", i),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }).collect()
}
```

```rust
// benches/cache.rs
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use tokenopt_core::prelude::*;

fn bench_cache_operations(c: &mut Criterion) {
    let cache = TokenCache::new(CacheConfig::default());
    
    let sizes = vec![100, 1000, 10000, 100000];
    
    let mut group = c.benchmark_group("cache_put");
    
    for size in sizes {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(BenchmarkId::new("put", size), &size, |b, &size| {
            b.iter(|| {
                for i in 0..size {
                    let key = format!("key_{}", i);
                    cache.put(&key, "namespace", &format!("value_{}", i), vec![]).unwrap();
                }
            })
        });
    }
    
    group.finish();
}

fn bench_cache_hit_rate(c: &mut Criterion) {
    let cache = TokenCache::new(CacheConfig::default());
    
    // Pre-populate
    for i in 0..10000 {
        cache.put(&format!("key_{}", i), "namespace", &format!("value_{}", i), vec![]).unwrap();
    }
    
    let hit_keys: Vec<_> = (0..5000).map(|i| format!("key_{}", i)).collect();
    let miss_keys: Vec<_> = (10000..15000).map(|i| format!("key_{}", i)).collect();
    
    c.bench_function("cache_hit", |b| {
        b.iter(|| {
            for key in &hit_keys {
                let _: Option<String> = cache.get(key, "namespace").unwrap();
            }
        })
    });
    
    c.bench_function("cache_miss", |b| {
        b.iter(|| {
            for key in &miss_keys {
                let _: Option<String> = cache.get(key, "namespace").unwrap();
            }
        })
    });
    
    c.bench_function("cache_mixed", |b| {
        let mut rng = rand::thread_rng();
        b.iter(|| {
            for _ in 0..1000 {
                let key = if rng.gen_bool(0.7) {
                    &hit_keys[rng.gen_range(0..hit_keys.len())]
                } else {
                    &miss_keys[rng.gen_range(0..miss_keys.len())]
                };
                let _: Option<String> = cache.get(key, "namespace").unwrap();
            }
        })
    });
}

fn bench_token_count_cache(c: &mut Criterion) {
    let tokenopt = TokenOpt::new().unwrap();
    let cache = tokenopt.optimizer().cache();
    
    let texts: Vec<_> = (0..10000).map(|i| format!("text_{}", i)).collect();
    
    // Cold cache
    c.bench_function("token_count_cache_cold", |b| {
        b.iter(|| {
            for text in &texts {
                tokenopt.optimizer().count_tokens_cached(text, Some("gpt-4")).unwrap();
            }
        })
    });
    
    // Warm cache
    for text in &texts {
        tokenopt.optimizer().count_tokens_cached(text, Some("gpt-4")).unwrap();
    }
    
    c.bench_function("token_count_cache_warm", |b| {
        b.iter(|| {
            for text in &texts {
                tokenopt.optimizer().count_tokens_cached(text, Some("gpt-4")).unwrap();
            }
        })
    });
}

criterion_group!(benches, bench_cache_operations, bench_cache_hit_rate, bench_token_count_cache);
criterion_main!(benches);
```

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench --workspace --all-features

# Run specific benchmark
cargo bench --bench token_counting

# Compare with baseline
cargo bench -- --save-baseline main
cargo bench -- --baseline main

# Generate HTML reports
cargo bench -- --output-format=html
```

## Expected Results (Target)

| Benchmark | Target | Unit |
|-----------|--------|------|
| Token counting (1KB) | < 0.1ms | ms |
| Token counting (100KB) | < 2ms | ms |
| Conversation counting (100 msgs) | < 5ms | ms |
| Context selection (100 msgs) | < 10ms | ms |
| Prompt optimization (500 chars) | < 5ms | ms |
| Full optimization (50 msgs) | < 50ms | ms |
| Summarization (100 msgs) | < 100ms | ms |
| Cache put (10K entries) | < 10ms | ms |
| Cache hit rate | > 95% | % |
| Memory (100K cached) | < 50MB | MB |

## Regression Detection

CI automatically compares benchmarks against `main` branch baseline. Failures trigger alerts.