use tokenopt_core::prelude::*;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TokenOpt Basic Usage Examples ===\n");

    // 1. Create TokenOpt instance
    let tokenopt = TokenOpt::new()?;
    
    // 2. Basic token counting
    println!("--- Token Counting ---");
    let text = "Hello, world! This is a test message for token counting.";
    let count = tokenopt.counter().count_tokens(text, Some("gpt-4"))?;
    println!("Text: \"{}\"", text);
    println!("Tokens: {}", count);
    println!();

    // 3. Conversation token counting
    println!("--- Conversation Token Counting ---");
    let messages = vec![
        Message {
            role: Role::System,
            content: "You are a helpful coding assistant specialized in Rust.".to_string(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        Message {
            role: Role::User,
            content: "How do I implement a binary search in Rust?".to_string(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        Message {
            role: Role::Assistant,
            content: r#"Here's a binary search implementation in Rust:

```rust
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}
```"#.to_string(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        Message {
            role: Role::User,
            content: "Can you explain the time complexity?".to_string(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        Message {
            role: Role::Assistant,
            content: "Binary search has O(log n) time complexity because we halve the search space with each iteration. This is significantly faster than linear search O(n) for large arrays.".to_string(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
    ];
    
    let conv_count = tokenopt.counter().count_conversation(&messages, Some("gpt-4"))?;
    println!("Conversation tokens: {}", conv_count.total);
    println!("Prompt tokens: {}", conv_count.prompt);
    println!("Completion tokens: {}", conv_count.completion);
    println!();

    // 4. Context Selection
    println!("--- Context Selection ---");
    let context_config = ContextConfig {
        strategy: SelectionStrategy::Balanced,
        max_tokens: 100, // Very small budget for demo
        reserve_tokens: 20,
        min_messages: 2,
        compress_old_messages: true,
        ..Default::default()
    };
    
    let selector = ContextSelector::new(context_config, tokenopt.counter().clone());
    let selected = selector.select(&messages, Some("binary search"), None)?;
    
    println!("Selected {} of {} messages", selected.messages.len(), messages.len());
    println!("Tokens: {} (saved {})", selected.token_count.total, selected.omitted_tokens);
    println!("Strategy: {:?}", selected.strategy_used);
    if let Some(summary) = &selected.summary {
        println!("Summary:\n{}", summary);
    }
    println!();

    // 5. Prompt Optimization
    println!("--- Prompt Optimization ---");
    let prompt = "Please could you kindly explain in great detail how to write a binary search function in Rust step by step with examples?";
    let template = PromptTemplate::from_string(prompt);
    let optimized = tokenopt.prompt_optimizer().optimize(&template, "gpt-4")?;
    
    println!("Original ({} tokens):", optimized.original_token_count);
    println!("{}", optimized.original);
    println!();
    println!("Optimized ({} tokens):", optimized.token_count);
    println!("{}", optimized.optimized);
    println!();
    println!("Saved: {} tokens ({:.1}%)", optimized.tokens_saved, optimized.savings_percentage);
    println!("Techniques: {:?}", optimized.techniques_applied);
    println!();

    // 6. Model Routing
    println!("--- Model Routing ---");
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
    println!();

    // 7. Conversation Summarization
    println!("--- Conversation Summarization ---");
    let summary_config = SummaryConfig {
        strategy: SummaryStrategy::Extractive,
        max_summary_tokens: 200,
        preserve_code_blocks: true,
        preserve_key_decisions: true,
        preserve_errors: true,
        ..Default::default()
    };
    
    let summary = tokenopt.summarizer().summarize(&messages, &summary_config)?;
    println!("Original tokens: {}", summary.original_token_count);
    println!("Summary tokens: {}", summary.token_count);
    println!("Compression: {:.1}%", summary.compression_ratio * 100.0);
    println!("Messages summarized: {}", summary.messages_summarized);
    println!("Summary:\n{}", summary.summary);
    println!();

    // 8. Cost Estimation
    println!("--- Cost Estimation ---");
    if let Some(model) = tokenopt.registry().get("gpt-4o") {
        if let Some(pricing) = &model.pricing {
            let cost = (conv_count.prompt as f64 * pricing.input_per_million / 1_000_000.0) +
                       (conv_count.completion as f64 * pricing.output_per_million / 1_000_000.0);
            println!("Model: {}", model.name);
            println!("Input cost: ${:.6}/M tokens", pricing.input_per_million);
            println!("Output cost: ${:.6}/M tokens", pricing.output_per_million);
            println!("Estimated cost for conversation: ${:.6}", cost);
        }
    }
    println!();

    // 9. Full Optimization Pipeline
    println!("--- Full Optimization Pipeline ---");
    let mut long_messages = messages.clone();
    // Add more messages to make it longer
    for i in 0..20 {
        long_messages.push(Message {
            role: if i % 2 == 0 { Role::User } else { Role::Assistant },
            content: format!("Follow-up question {} about the implementation details", i + 1),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        });
    }
    
    let opt_result = tokenopt.optimizer().optimize_conversation(&long_messages, None, Some("gpt-4"))?;
    println!("Original tokens: {}", opt_result.original_token_count.total);
    println!("Optimized tokens: {}", opt_result.optimized_token_count.total);
    println!("Tokens saved: {}", opt_result.tokens_saved);
    println!("Savings: {:.1}%", opt_result.savings_percentage);
    if let Some(cost_savings) = opt_result.estimated_cost_savings_usd {
        println!("Cost savings: ${:.6}", cost_savings);
    }
    println!("Strategies applied:");
    for strategy in &opt_result.metadata.strategies_applied {
        println!("  - {:?}", strategy);
    }
    println!("Optimization time: {}ms", opt_result.metadata.optimization_time_ms);

    Ok(())
}