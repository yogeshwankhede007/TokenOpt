import { TokenOpt, TokenCounter, TokenOptimizer, PromptOptimizer, ModelRouter, ConversationSummarizer, ContextSelector, PromptTask, TaskComplexity } from '@tokenopt/core';

// Initialize TokenOpt
const tokenopt = new TokenOpt();

console.log('=== TokenOpt TypeScript Examples ===\n');

// 1. Basic Token Counting
console.log('--- Token Counting ---');
const text = "Hello, world! This is a test message for token counting.";
const count = tokenopt.counter.countTokens(text, 'gpt-4');
console.log(`Text: "${text}"`);
console.log(`Tokens: ${count}\n`);

// 2. Conversation Token Counting
console.log('--- Conversation Token Counting ---');
const messages = [
  { role: 'system', content: 'You are a helpful coding assistant specialized in Rust.' },
  { role: 'user', content: 'How do I implement a binary search in Rust?' },
  { role: 'assistant', content: `Here's a binary search implementation in Rust:

\`\`\`rust
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
\`\`\`` },
  { role: 'user', content: 'Can you explain the time complexity?' },
  { role: 'assistant', content: 'Binary search has O(log n) time complexity because we halve the search space with each iteration. This is significantly faster than linear search O(n) for large arrays.' },
];

const convCount = tokenopt.counter.countConversation(messages, 'gpt-4');
console.log(`Conversation tokens: ${convCount.total}`);
console.log(`Prompt tokens: ${convCount.prompt}`);
console.log(`Completion tokens: ${convCount.completion}\n`);

// 3. Context Selection
console.log('--- Context Selection ---');
const selector = new ContextSelector({
  strategy: 'Balanced',
  maxTokens: 100, // Very small budget for demo
  reserveTokens: 20,
  minMessages: 2,
  compressOldMessages: true,
  similarityThreshold: 0.7,
});

const selected = selector.select(messages, 'binary search');
console.log(`Selected ${selected.messages.length} of ${messages.length} messages`);
console.log(`Tokens: ${selected.tokenCount.total} (saved ${selected.omittedTokens})`);
console.log(`Strategy: ${selected.strategyUsed}`);
if (selected.summary) {
  console.log(`Summary:\n${selected.summary}`);
}
console.log('');

// 4. Prompt Optimization
console.log('--- Prompt Optimization ---');
const prompt = "Please could you kindly explain in great detail how to write a binary search function in Rust step by step with examples?";
const optimized = tokenopt.promptOptimizer.optimizeString(prompt, 'gpt-4');

console.log(`Original (${optimized.originalTokenCount} tokens):`);
console.log(optimized.original);
console.log('');
console.log(`Optimized (${optimized.tokenCount} tokens):`);
console.log(optimized.optimized);
console.log('');
console.log(`Saved: ${optimized.tokensSaved} tokens (${optimized.savingsPercentage.toFixed(1)}%)`);
console.log(`Techniques: ${optimized.techniquesApplied.join(', ')}\n`);

// 5. Task-specific Prompt Optimization
console.log('--- Task-Specific Prompt Optimization ---');
const taskOptimized = tokenopt.promptOptimizer.optimizeForTask(
  PromptTask.CodeGeneration,
  'Create a REST API endpoint with authentication and rate limiting',
  'gpt-4'
);
console.log(`Task: CodeGeneration`);
console.log(`Optimized (${taskOptimized.tokenCount} tokens):`);
console.log(taskOptimized.prompt);
console.log(`Saved: ${taskOptimized.tokensSaved} tokens (${taskOptimized.savingsPercentage.toFixed(1)}%)\n`);

// 6. Model Routing
console.log('--- Model Routing ---');
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
console.log('');

// 7. Conversation Summarization
console.log('--- Conversation Summarization ---');
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
console.log(`Summary:\n${summary.summary}\n`);

// 8. Rolling Summary for Long Conversations
console.log('--- Rolling Summary ---');
const longMessages = [...messages];
for (let i = 0; i < 20; i++) {
  longMessages.push({
    role: i % 2 === 0 ? 'user' : 'assistant',
    content: `Follow-up question ${i + 1} about the implementation details`,
  });
}

const rolling = summarizer.createRollingSummary(longMessages, 5);
console.log(`Rolling summary (window=5): ${rolling.tokenCount} tokens`);
console.log(`Summary:\n${rolling.summary}\n`);

// 9. Full Optimization Pipeline
console.log('--- Full Optimization Pipeline ---');
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
optResult.metadata.strategiesApplied.forEach(strategy => {
  console.log(`  - ${JSON.stringify(strategy)}`);
});
console.log(`Optimization time: ${optResult.metadata.optimizationTimeMs}ms\n`);

// 10. Cost Estimation
console.log('--- Cost Estimation ---');
const model = tokenopt.registry.all().find(m => m.id === 'gpt-4o');
if (model?.pricing) {
  const cost = (convCount.prompt * model.pricing.inputPerMillion / 1_000_000) +
               (convCount.completion * model.pricing.outputPerMillion / 1_000_000);
  console.log(`Model: ${model.name}`);
  console.log(`Input cost: $${model.pricing.inputPerMillion.toFixed(2)}/M tokens`);
  console.log(`Output cost: $${model.pricing.outputPerMillion.toFixed(2)}/M tokens`);
  console.log(`Estimated cost: $${cost.toFixed(6)}`);
}

console.log('\n=== All Examples Complete ===');