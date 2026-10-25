// TypeScript type definitions for TokenOpt

export interface TokenCounterConfig {
  default_model: string;
  enable_cache: boolean;
  max_cache_size: number;
  model_aliases: Record<string, string>;
}

export interface TokenCount {
  total: number;
  prompt: number;
  completion: number;
  model: string;
  estimated_cost_usd?: number;
}

export type Role = 'system' | 'user' | 'assistant' | 'tool' | 'function';

export interface Message {
  role: Role;
  content: string;
  name?: string;
  tool_calls?: ToolCall[];
  tool_call_id?: string;
}

export interface ToolCall {
  id: string;
  function: FunctionCall;
  type: string;
}

export interface FunctionCall {
  name: string;
  arguments: string;
}

export interface ModelPricing {
  input_per_million: number;
  output_per_million: number;
  cache_read_per_million?: number;
  cache_write_per_million?: number;
  currency: string;
}

export interface ModelFeatures {
  function_calling: boolean;
  vision: boolean;
  json_mode: boolean;
  parallel_tool_calls: boolean;
  streaming: boolean;
  system_prompt: boolean;
  logprobs: boolean;
}

export interface ModelCapabilities {
  id: string;
  name: string;
  provider: string;
  context_window: number;
  max_output_tokens: number;
  pricing?: ModelPricing;
  features: ModelFeatures;
  family: string;
  is_fast_variant: boolean;
  knowledge_cutoff?: string;
}

export type PricingTier = 'Free' | 'UltraCheap' | 'Cheap' | 'Standard' | 'Premium' | 'UltraPremium';

export interface OptimizationConfig {
  context_config: ContextConfig;
  cache_config: CacheConfig;
  enable_caching: boolean;
  enable_summarization: boolean;
  enable_prompt_optimization: boolean;
  enable_model_routing: boolean;
  default_model: string;
  max_optimization_time_ms: number;
}

export interface ContextConfig {
  strategy: SelectionStrategy;
  max_tokens: number;
  reserve_tokens: number;
  system_prompt_tokens: number;
  min_messages: number;
  max_messages?: number;
  include_system: boolean;
  compress_old_messages: boolean;
  similarity_threshold: number;
}

export type SelectionStrategy = 
  | 'RecentFirst'
  | 'RelevanceFirst'
  | 'ImportanceFirst'
  | 'Balanced'
  | { Weighted: { recency: number; relevance: number; importance: number } }
  | { SlidingWindow: { window_size: number; summary_interval: number } };

export interface SelectedContext {
  messages: Message[];
  token_count: TokenCount;
  strategy_used: SelectionStrategy;
  omitted_count: number;
  omitted_tokens: number;
  summary?: string;
  metadata: ContextMetadata;
}

export interface ContextMetadata {
  total_available_messages: number;
  total_available_tokens: number;
  selection_time_ms: number;
  compression_ratio: number;
  scores: MessageScore[];
}

export interface MessageScore {
  index: number;
  role: Role;
  token_count: number;
  importance: number;
  relevance: number;
  recency: number;
  composite: number;
  selected: boolean;
}

export interface OptimizationResult {
  original_token_count: TokenCount;
  optimized_token_count: TokenCount;
  tokens_saved: number;
  savings_percentage: number;
  estimated_cost_savings_usd?: number;
  selected_context?: SelectedContext;
  optimized_prompt?: OptimizedPrompt;
  routing_decision?: RoutingDecision;
  summary?: string;
  metadata: OptimizationMetadata;
}

export interface OptimizationMetadata {
  optimization_time_ms: number;
  cache_hit: boolean;
  strategies_applied: AppliedStrategy[];
  warnings: string[];
}

export type AppliedStrategy =
  | { ContextSelection: { strategy: SelectionStrategy; tokens_saved: number } }
  | { PromptOptimization: { tokens_saved: number } }
  | { Summarization: { tokens_saved: number } }
  | { ModelRouting: { from_model: string; to_model: string; cost_savings_usd: number } }
  | { Caching: { cache_hit: boolean } };

export interface OptimizedPrompt {
  original: string;
  optimized: string;
  prompt: string;
  token_count: number;
  original_token_count: number;
  tokens_saved: number;
  savings_percentage: number;
  techniques_applied: OptimizationTechnique[];
  warnings: string[];
}

export type OptimizationTechnique =
  | 'RemoveRedundancy'
  | 'CompressInstructions'
  | 'SimplifyLanguage'
  | 'RemoveExamples'
  | 'UseShorterSynonyms'
  | 'RestructureFormat'
  | { Custom: string };

export interface PromptOptimizationConfig {
  aggressiveness: AggressivenessLevel;
  preserve_examples: boolean;
  preserve_formatting: boolean;
  max_length_reduction: number;
  custom_rules: CustomRule[];
}

export type AggressivenessLevel = 'Conservative' | 'Balanced' | 'Aggressive';

export interface CustomRule {
  pattern: string;
  replacement: string;
  description: string;
}

export interface PromptTemplate {
  template: string;
  variables: Record<string, string>;
  metadata: TemplateMetadata;
}

export interface TemplateMetadata {
  name?: string;
  description?: string;
  tags: string[];
  version?: string;
}

export type TaskComplexity = 'Simple' | 'Moderate' | 'Complex';

export type RoutingStrategy =
  | 'Fixed'
  | 'CostOptimized'
  | 'SpeedOptimized'
  | 'Balanced'
  | { Cascade: { max_attempts: number } }
  | 'Custom';

export interface RoutingDecision {
  requested_model: string;
  selected_model: string;
  strategy: RoutingStrategy;
  complexity: TaskComplexity;
  reasoning: string;
  required_features: ModelFeature[];
  estimated_cost_usd?: number;
  fallback_models: string[];
  confidence: number;
}

export type ModelFeature =
  | 'FunctionCalling'
  | 'Vision'
  | 'JsonMode'
  | 'ParallelToolCalls'
  | 'Streaming'
  | 'SystemPrompt'
  | 'Logprobs';

export interface RoutingRule {
  name: string;
  condition: RoutingCondition;
  action: RoutingAction;
  priority: number;
}

export type RoutingCondition =
  | { TaskComplexity: TaskComplexity }
  | { RequiredFeatures: ModelFeature[] }
  | { MaxCostPerMillion: number }
  | { MinContextWindow: number }
  | { ModelFamily: string }
  | { Custom: string };

export type RoutingAction =
  | { UseModel: string }
  | 'UseCheapest'
  | 'UseFastest'
  | { UseTier: PricingTier }
  | { EscalateTo: string }
  | { BlockModel: string };

export type SummaryStrategy =
  | 'Extractive'
  | 'Abstractive'
  | 'Hybrid'
  | { Rolling: { window_size: number } }
  | { Hierarchical: { levels: number } };

export interface SummaryConfig {
  strategy: SummaryStrategy;
  max_summary_tokens: number;
  target_compression_ratio: number;
  preserve_code_blocks: boolean;
  preserve_key_decisions: boolean;
  preserve_errors: boolean;
  min_messages_to_summarize: number;
  summary_prompt_template?: string;
}

export interface SummaryResult {
  summary: string;
  token_count: number;
  original_token_count: number;
  compression_ratio: number;
  strategy_used: SummaryStrategy;
  messages_summarized: number;
  key_points: KeyPoint[];
  metadata: SummaryMetadata;
}

export interface KeyPoint {
  content: string;
  importance: number;
  message_indices: number[];
  category: KeyPointCategory;
}

export type KeyPointCategory =
  | 'Decision'
  | 'Error'
  | 'Code'
  | 'Requirement'
  | 'Question'
  | 'Answer'
  | 'Context'
  | 'Other';

export interface SummaryMetadata {
  processing_time_ms: number;
  original_message_count: number;
  messages_in_summary: number;
  warnings: string[];
}

export interface CacheConfig {
  max_entries: number;
  ttl: number; // seconds
  persist_to_disk: boolean;
  cache_dir?: string;
  key_prefix: string;
}

export interface CacheStats {
  hits: number;
  misses: number;
  evictions: number;
  expired: number;
  total_size_bytes: number;
  entry_count: number;
}