import { nativeBinding } from './index';
import {
  ContextConfig,
  SelectionStrategy,
  SelectedContext,
  Message,
  ModelCapabilities,
} from './types';

export class ContextSelector {
  private handle: number;

  constructor(config: ContextConfig) {
    this.handle = nativeBinding.context_selector_new(config);
  }

  select(
    messages: Message[],
    query?: string,
    model?: ModelCapabilities
  ): SelectedContext {
    return nativeBinding.context_selector_select(this.handle, messages, query, model);
  }

  getConfig(): ContextConfig {
    return nativeBinding.context_selector_get_config(this.handle);
  }
}