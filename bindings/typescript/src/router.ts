import { nativeBinding } from './index';
import {
  RoutingDecision,
  TaskComplexity,
  RoutingStrategy,
  RoutingRule,
  ModelFeature,
  Message,
} from './types';

export class ModelRouter {
  private handle: number;

  constructor(strategy?: RoutingStrategy) {
    this.handle = nativeBinding.model_router_new(strategy);
  }

  route(
    requestedModel: string,
    complexity: TaskComplexity,
    messages: Message[]
  ): RoutingDecision {
    return nativeBinding.model_router_route(this.handle, requestedModel, complexity, messages);
  }

  addRule(rule: RoutingRule): void {
    nativeBinding.model_router_add_rule(this.handle, rule);
  }

  recordPerformance(modelId: string, latencyMs: number, success: boolean): void {
    nativeBinding.model_router_record_performance(this.handle, modelId, latencyMs, success);
  }

  getDefaultStrategy(): RoutingStrategy {
    return nativeBinding.model_router_get_default_strategy(this.handle);
  }

  setDefaultStrategy(strategy: RoutingStrategy): void {
    nativeBinding.model_router_set_default_strategy(this.handle, strategy);
  }
}