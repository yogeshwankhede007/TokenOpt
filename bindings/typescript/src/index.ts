import { join } from 'path';
import { existsSync } from 'fs';

// Load the native module
let nativeBinding: any;

const possiblePaths = [
  join(__dirname, '../tokenopt.node'),
  join(__dirname, '../../crates/tokenopt-core/target/release/tokenopt.node'),
  join(__dirname, '../../crates/tokenopt-core/target/debug/tokenopt.node'),
  join(process.cwd(), 'tokenopt.node'),
];

for (const path of possiblePaths) {
  if (existsSync(path)) {
    try {
      nativeBinding = require(path);
      break;
    } catch (e) {
      // Continue to next path
    }
  }
}

if (!nativeBinding) {
  throw new Error(
    'TokenOpt native binding not found. Please run `npm run build` to compile the native module.'
  );
}

export { nativeBinding };

// Re-export types
export * from './types';
export { TokenCounter } from './counter';
export { TokenOptimizer } from './optimizer';
export { PromptOptimizer } from './prompt';
export { ModelRouter } from './router';
export { ConversationSummarizer } from './summarizer';
export { ContextSelector } from './context';
export { TokenCache } from './cache';