#!/usr/bin/env python3
"""TokenOpt Python Examples"""

from tokenopt import (
    TokenOpt, TokenCounter, TokenOptimizer, PromptOptimizer,
    ModelRouter, ConversationSummarizer, ContextSelector,
    PromptTask, TaskComplexity, SelectionStrategy,
    SummaryStrategy, RoutingStrategy
)

def main():
    print("=== TokenOpt Python Examples ===\n")
    
    # Initialize TokenOpt
    tokenopt = TokenOpt()
    
    # 1. Basic Token Counting
    print("--- Token Counting ---")
    text = "Hello, world! This is a test message for token counting."
    count = tokenopt.counter.count_tokens(text, "gpt-4")
    print(f'Text: "{text}"')
    print(f"Tokens: {count}\n")
    
    # 2. Conversation Token Counting
    print("--- Conversation Token Counting ---")
    messages = [
        {"role": "system", "content": "You are a helpful coding assistant specialized in Rust."},
        {"role": "user", "content": "How do I implement a binary search in Rust?"},
        {"role": "assistant", "content": """Here's a binary search implementation in Rust:

```rust
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0
    let mut right = arr.len()
    
    while left < right:
        mid = left + (right - left) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid
    return None
}"""},
        {"role": "user", "content": "Can you explain the time complexity?"},
        {"role": "assistant", "content": "Binary search has O(log n) time complexity because we halve the search space with each iteration. This is significantly faster than linear search O(n) for large arrays."},
    ]
    
    conv_count = tokenopt.counter.count_conversation(messages, "gpt-4")
    print(f"Conversation tokens: {conv_count.total}")
    print(f"Prompt tokens: {conv_count.prompt}")
    print(f"Completion tokens: {conv_count.completion}\n")
    
    # 3. Context Selection
    print("--- Context Selection ---")
    selector = ContextSelector(
        strategy=SelectionStrategy.Balanced,
        max_tokens=100,  # Very small budget for demo
        reserve_tokens=20,
        min_messages=2,
        compress_old_messages=True,
    )
    
    selected = selector.select(messages, query="binary search")
    print(f"Selected {len(selected.messages)} of {len(messages)} messages")
    print(f"Tokens: {selected.token_count.total} (saved {selected.omitted_tokens})")
    print(f"Strategy: {selected.strategy_used}")
    if selected.summary:
        print(f"Summary:\n{selected.summary}")
    print()
    
    # 4. Prompt Optimization
    print("--- Prompt Optimization ---")
    prompt = "Please could you kindly explain in great detail how to write a binary search function in Rust step by step with examples?"
    optimized = tokenopt.prompt_optimizer.optimize_string(prompt, "gpt-4")
    
    print(f"Original ({optimized.original_token_count} tokens):")
    print(optimized.original)
    print()
    print(f"Optimized ({optimized.token_count} tokens):")
    print(optimized.optimized)
    print()
    print(f"Saved: {optimized.tokens_saved} tokens ({optimized.savings_percentage:.1f}%)")
    print(f"Techniques: {', '.join(optimized.techniques_applied)}\n")
    
    # 5. Task-specific Prompt Optimization
    print("--- Task-Specific Prompt Optimization ---")
    task_optimized = tokenopt.prompt_optimizer.optimize_for_task(
        PromptTask.CodeGeneration,
        "Create a REST API endpoint with authentication and rate limiting",
        "gpt-4"
    )
    print(f"Task: CodeGeneration")
    print(f"Optimized ({task_optimized.token_count} tokens):")
    print(task_optimized.prompt)
    print(f"Saved: {task_optimized.tokens_saved} tokens ({task_optimized.savings_percentage:.1f}%)\n")
    
    # 6. Model Routing
    print("--- Model Routing ---")
    router = ModelRouter(RoutingStrategy.Balanced)
    decision = router.route("gpt-4o", TaskComplexity.Simple, messages)
    
    print(f"Requested: {decision.requested_model}")
    print(f"Selected: {decision.selected_model}")
    print(f"Strategy: {decision.strategy}")
    print(f"Reasoning: {decision.reasoning}")
    print(f"Confidence: {decision.confidence * 100:.0f}%")
    if decision.estimated_cost_usd:
        print(f"Estimated cost: ${decision.estimated_cost_usd:.6f}")
    print()
    
    # 7. Conversation Summarization
    print("--- Conversation Summarization ---")
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
    print(f"Summary:\n{summary.summary}\n")
    
    # 8. Rolling Summary for Long Conversations
    print("--- Rolling Summary ---")
    long_messages = messages.copy()
    for i in range(20):
        long_messages.append({
            "role": "user" if i % 2 == 0 else "assistant",
            "content": f"Follow-up question {i + 1} about the implementation details"
        })
    
    rolling = summarizer.create_rolling_summary(long_messages, 5)
    print(f"Rolling summary (window=5): {rolling.token_count} tokens")
    print(f"Summary:\n{rolling.summary}\n")
    
    # 9. Full Optimization Pipeline
    print("--- Full Optimization Pipeline ---")
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
    print(f"Optimization time: {opt_result.metadata.optimization_time_ms}ms\n")
    
    # 10. Cost Estimation
    print("--- Cost Estimation ---")
    for model in tokenopt.registry.all():
        if model.id == "gpt-4o" and model.pricing:
            cost = (conv_count.prompt * model.pricing.input_per_million / 1_000_000) + \
                   (conv_count.completion * model.pricing.output_per_million / 1_000_000)
            print(f"Model: {model.name}")
            print(f"Input cost: ${model.pricing.input_per_million:.2f}/M tokens")
            print(f"Output cost: ${model.pricing.output_per_million:.2f}/M tokens")
            print(f"Estimated cost: ${cost:.6f}")
            break
    
    print("\n=== All Examples Complete ===")

if __name__ == "__main__":
    main()