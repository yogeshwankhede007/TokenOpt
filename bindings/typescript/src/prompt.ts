import { nativeBinding } from './index';
import {
  PromptOptimizationConfig,
  OptimizedPrompt,
  PromptTemplate,
  PromptTask,
  AggressivenessLevel,
  OptimizationTechnique,
} from './types';

export class PromptOptimizer {
  private handle: number;

  constructor(config?: Partial<PromptOptimizationConfig>) {
    this.handle = nativeBinding.prompt_optimizer_new(config || {});
  }

  optimize(template: PromptTemplate, model: string): OptimizedPrompt {
    return nativeBinding.prompt_optimizer_optimize(this.handle, template, model);
  }

  optimizeString(prompt: string, model: string): OptimizedPrompt {
    return nativeBinding.prompt_optimizer_optimize_string(this.handle, prompt, model);
  }

  optimizeForTask(task: PromptTask, context: string, model: string): OptimizedPrompt {
    return nativeBinding.prompt_optimizer_optimize_for_task(this.handle, task, context, model);
  }

  getConfig(): PromptOptimizationConfig {
    return nativeBinding.prompt_optimizer_get_config(this.handle);
  }

  setAggressiveness(level: AggressivenessLevel): void {
    nativeBinding.prompt_optimizer_set_aggressiveness(this.handle, level);
  }
}