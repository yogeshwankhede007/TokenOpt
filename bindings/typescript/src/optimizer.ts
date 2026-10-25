import { nativeBinding } from './index';
import {
  OptimizationConfig,
  OptimizationResult,
  Message,
  TokenCount,
  SelectedContext,
  OptimizedPrompt,
  RoutingDecision,
} from './types';

export class TokenOptimizer {
  private handle: number;

  constructor(config?: Partial<OptimizationConfig>) {
    this.handle = nativeBinding.token_optimizer_new(config || {});
  }

  optimizeConversation(
    messages: Message[],
    query?: string,
    model?: string
  ): OptimizationResult {
    return nativeBinding.token_optimizer_optimize_conversation(this.handle, messages, query, model);
  }

  optimizePrompt(prompt: string, model?: string): OptimizedPrompt {
    return nativeBinding.token_optimizer_optimize_prompt(this.handle, prompt, model);
  }

  countTokensCached(text: string, model?: string): number {
    return nativeBinding.token_optimizer_count_tokens_cached(this.handle, text, model);
  }

  getConfig(): OptimizationConfig {
    return nativeBinding.token_optimizer_get_config(this.handle);
  }
}