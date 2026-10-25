import { nativeBinding } from './index';
import {
  SummaryConfig,
  SummaryResult,
  SummaryStrategy,
  Message,
  RollingSummary,
} from './types';

export class ConversationSummarizer {
  private handle: number;

  constructor(config?: Partial<SummaryConfig>) {
    this.handle = nativeBinding.conversation_summarizer_new(config || {});
  }

  summarize(messages: Message[], config: SummaryConfig): SummaryResult {
    return nativeBinding.conversation_summarizer_summarize(this.handle, messages, config);
  }

  createRollingSummary(messages: Message[], windowSize: number): RollingSummary {
    return nativeBinding.conversation_summarizer_create_rolling(this.handle, messages, windowSize);
  }

  updateRollingSummary(rolling: RollingSummary, newMessages: Message[]): RollingSummary {
    return nativeBinding.conversation_summarizer_update_rolling(this.handle, rolling, newMessages);
  }

  getConfig(): SummaryConfig {
    return nativeBinding.conversation_summarizer_get_config(this.handle);
  }
}