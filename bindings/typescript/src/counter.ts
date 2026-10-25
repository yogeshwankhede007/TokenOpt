import { nativeBinding } from './index';
import {
  TokenCounterConfig,
  TokenCount,
  Message,
  Role,
} from './types';

export class TokenCounter {
  private handle: number;

  constructor(config?: Partial<TokenCounterConfig>) {
    this.handle = nativeBinding.token_counter_new(config || {});
  }

  countTokens(text: string, model?: string): number {
    return nativeBinding.token_counter_count_tokens(this.handle, text, model);
  }

  countConversation(messages: Message[], model?: string): TokenCount {
    return nativeBinding.token_counter_count_conversation(this.handle, messages, model);
  }

  countTemplate(template: string, variables: Record<string, string>, model?: string): number {
    return nativeBinding.token_counter_count_template(this.handle, template, variables, model);
  }

  estimateCost(count: TokenCount, pricing?: { input_per_million: number; output_per_million: number }): number | null {
    return nativeBinding.token_counter_estimate_cost(this.handle, count, pricing);
  }

  getConfig(): TokenCounterConfig {
    return nativeBinding.token_counter_get_config(this.handle);
  }

  clearTokenizers(): void {
    nativeBinding.token_counter_clear_tokenizers(this.handle);
  }

  // Static method for default counter
  static getDefault(): TokenCounter {
    if (!TokenCounter._defaultInstance) {
      TokenCounter._defaultInstance = new TokenCounter();
    }
    return TokenCounter._defaultInstance;
  }

  private static _defaultInstance: TokenCounter | null = null;
}