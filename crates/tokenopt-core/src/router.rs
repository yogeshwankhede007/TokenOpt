//! Model routing for cost optimization

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::counter::{TokenCounter, Message};
use crate::models::{ModelCapabilities, ModelRegistry, ModelFeature, PricingTier};

/// Task complexity for routing decisions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskComplexity {
    Simple,      // Basic Q&A, simple code snippets
    Moderate,    // Multi-step reasoning, code generation
    Complex,     // Architecture, debugging, refactoring
}

/// Routing strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoutingStrategy {
    /// Always use the requested model
    Fixed,
    /// Use cheapest model that meets requirements
    CostOptimized,
    /// Use fastest model that meets requirements
    SpeedOptimized,
    /// Balance cost and capability
    Balanced,
    /// Cascade: try cheap model first, escalate if needed
    Cascade { max_attempts: usize },
    /// Custom routing function
    Custom,
}

/// Routing decision with reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    pub requested_model: String,
    pub selected_model: String,
    pub strategy: RoutingStrategy,
    pub complexity: TaskComplexity,
    pub reasoning: String,
    pub required_features: Vec<ModelFeature>,
    pub estimated_cost_usd: Option<f64>,
    pub fallback_models: Vec<String>,
    pub confidence: f32,
}

/// Model router for intelligent model selection
pub struct ModelRouter {
    registry: Arc<ModelRegistry>,
    default_strategy: RoutingStrategy,
    custom_rules: Vec<RoutingRule>,
    performance_cache: HashMap<String, ModelPerformance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModelPerformance {
    avg_latency_ms: u64,
    success_rate: f32,
    error_rate: f32,
    last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    pub name: String,
    pub condition: RoutingCondition,
    pub action: RoutingAction,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingCondition {
    TaskComplexity(TaskComplexity),
    RequiredFeatures(Vec<ModelFeature>),
    MaxCostPerMillion(f64),
    MinContextWindow(usize),
    ModelFamily(String),
    Custom(String), // Expression string
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingAction {
    UseModel(String),
    UseCheapest,
    UseFastest,
    UseTier(PricingTier),
    EscalateTo(String),
    BlockModel(String),
}

impl ModelRouter {
    pub fn new(registry: Arc<ModelRegistry>) -> Self {
        Self {
            registry,
            default_strategy: RoutingStrategy::Balanced,
            custom_rules: Vec::new(),
            performance_cache: HashMap::new(),
        }
    }

    pub fn with_strategy(mut self, strategy: RoutingStrategy) -> Self {
        self.default_strategy = strategy;
        self
    }

    pub fn add_rule(mut self, rule: RoutingRule) -> Self {
        self.custom_rules.push(rule);
        self.custom_rules.sort_by_key(|r| -r.priority);
        self
    }

    /// Route a request to the best model
    pub fn route(
        &self,
        requested_model: &str,
        complexity: TaskComplexity,
        messages: &[Message],
    ) -> Result<RoutingDecision, RoutingError> {
        let requested = self.registry.get(requested_model)
            .ok_or_else(|| RoutingError::ModelNotFound(requested_model.to_string()))?;

        // Determine required features from messages
        let required_features = self.infer_required_features(messages);
        
        // Check custom rules first
        for rule in &self.custom_rules {
            if self.matches_condition(&rule.condition, complexity, &required_features, requested_model) {
                if let Some(decision) = self.apply_action(&rule.action, requested_model, complexity, &required_features)? {
                    return Ok(decision);
                }
            }
        }

        // Apply default strategy
        match self.default_strategy {
            RoutingStrategy::Fixed => Ok(self.fixed_routing(requested_model, complexity, &required_features)),
            RoutingStrategy::CostOptimized => self.cost_optimized_routing(requested_model, complexity, &required_features),
            RoutingStrategy::SpeedOptimized => self.speed_optimized_routing(requested_model, complexity, &required_features),
            RoutingStrategy::Balanced => self.balanced_routing(requested_model, complexity, &required_features),
            RoutingStrategy::Cascade { max_attempts } => self.cascade_routing(requested_model, complexity, &required_features, max_attempts),
            RoutingStrategy::Custom => self.custom_routing(requested_model, complexity, &required_features),
        }
    }

    fn infer_required_features(&self, messages: &[Message]) -> Vec<ModelFeature> {
        let mut features = Vec::new();
        
        for msg in messages {
            if msg.tool_calls.is_some() {
                features.push(ModelFeature::FunctionCalling);
            }
            if msg.content.contains("```") || msg.content.contains("code") {
                // Might need code understanding
            }
            // Check for image references (simplified)
            if msg.content.contains("image") || msg.content.contains("screenshot") || msg.content.contains("diagram") {
                features.push(ModelFeature::Vision);
            }
        }

        // Deduplicate
        features.sort();
        features.dedup();
        features
    }

    fn matches_condition(
        &self,
        condition: &RoutingCondition,
        complexity: TaskComplexity,
        required_features: &[ModelFeature],
        requested_model: &str,
    ) -> bool {
        match condition {
            RoutingCondition::TaskComplexity(c) => *c == complexity,
            RoutingCondition::RequiredFeatures(f) => f.iter().all(|rf| required_features.contains(rf)),
            RoutingCondition::MaxCostPerMillion(max) => {
                if let Some(model) = self.registry.get(requested_model) {
                    if let Some(pricing) = &model.pricing {
                        return (pricing.input_per_million + pricing.output_per_million) / 2.0 <= *max;
                    }
                }
                false
            }
            RoutingCondition::MinContextWindow(min) => {
                if let Some(model) = self.registry.get(requested_model) {
                    return model.context_window >= *min;
                }
                false
            }
            RoutingCondition::ModelFamily(family) => {
                if let Some(model) = self.registry.get(requested_model) {
                    return model.family == *family;
                }
                false
            }
            RoutingCondition::Custom(_) => false, // Would need expression evaluator
        }
    }

    fn apply_action(
        &self,
        action: &RoutingAction,
        requested_model: &str,
        complexity: TaskComplexity,
        required_features: &[ModelFeature],
    ) -> Result<Option<RoutingDecision>, RoutingError> {
        match action {
            RoutingAction::UseModel(model) => {
                if self.registry.get(model).is_some() {
                    Ok(Some(RoutingDecision {
                        requested_model: requested_model.to_string(),
                        selected_model: model.clone(),
                        strategy: RoutingStrategy::Custom,
                        complexity,
                        reasoning: format!("Custom rule: use {}", model),
                        required_features: required_features.to_vec(),
                        estimated_cost_usd: None,
                        fallback_models: vec![],
                        confidence: 1.0,
                    }))
                } else {
                    Ok(None)
                }
            }
            RoutingAction::UseCheapest => {
                self.cost_optimized_routing(requested_model, complexity, required_features).map(Some)
            }
            RoutingAction::UseFastest => {
                self.speed_optimized_routing(requested_model, complexity, required_features).map(Some)
            }
            RoutingAction::UseTier(tier) => {
                self.tier_routing(requested_model, complexity, required_features, *tier).map(Some)
            }
            RoutingAction::EscalateTo(model) => {
                if self.registry.get(model).is_some() {
                    Ok(Some(RoutingDecision {
                        requested_model: requested_model.to_string(),
                        selected_model: model.clone(),
                        strategy: RoutingStrategy::Cascade { max_attempts: 1 },
                        complexity,
                        reasoning: format!("Escalated to {}", model),
                        required_features: required_features.to_vec(),
                        estimated_cost_usd: None,
                        fallback_models: vec![],
                        confidence: 0.8,
                    }))
                } else {
                    Ok(None)
                }
            }
            RoutingAction::BlockModel(model) => {
                if model == requested_model {
                    self.cost_optimized_routing(requested_model, complexity, required_features).map(Some)
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn fixed_routing(
        &self,
        requested_model: &str,
        complexity: TaskComplexity,
        required_features: &[ModelFeature],
    ) -> RoutingDecision {
        RoutingDecision {
            requested_model: requested_model.to_string(),
            selected_model: requested_model.to_string(),
            strategy: RoutingStrategy::Fixed,
            complexity,
            reasoning: "Fixed routing: using requested model".to_string(),
            required_features: required_features.to_vec(),
            estimated_cost_usd: None,
            fallback_models: self.get_fallbacks(requested_model, required_features),
            confidence: 1.0,
        }
    }

    fn cost_optimized_routing(
        &self,
        requested_model: &str,
        complexity: TaskComplexity,
        required_features: &[ModelFeature],
    ) -> Result<RoutingDecision, RoutingError> {
        let candidates: Vec<_> = self.registry.all()
            .into_iter()
            .filter(|m| required_features.iter().all(|f| m.supports(*f)))
            .filter(|m| self.meets_complexity(m.as_ref(), complexity))
            .collect();

        if candidates.is_empty() {
            return Err(RoutingError::NoSuitableModel);
        }

        // Sort by pricing tier, then by cost
        let mut sorted = candidates;
        sorted.sort_by(|a, b| {
            let tier_cmp = a.pricing_tier().cmp(&b.pricing_tier());
            if tier_cmp != std::cmp::Ordering::Equal {
                return tier_cmp;
            }
            // Within same tier, compare actual pricing
            let a_cost = a.pricing.as_ref().map(|p| p.input_per_million + p.output_per_million).unwrap_or(f64::MAX);
            let b_cost = b.pricing.as_ref().map(|p| p.input_per_million + p.output_per_million).unwrap_or(f64::MAX);
            a_cost.partial_cmp(&b_cost).unwrap()
        });

        let selected = &sorted[0];
        let fallbacks: Vec<String> = sorted.iter().skip(1).take(3).map(|m| m.id.clone()).collect();

        let cost = self.estimate_cost(selected.as_ref(), 1000, 500);

        Ok(RoutingDecision {
            requested_model: requested_model.to_string(),
            selected_model: selected.id.clone(),
            strategy: RoutingStrategy::CostOptimized,
            complexity,
            reasoning: format!("Selected cheapest suitable model: {}", selected.name),
            required_features: required_features.to_vec(),
            estimated_cost_usd: cost,
            fallback_models: fallbacks,
            confidence: 0.9,
        })
    }

    fn speed_optimized_routing(
        &self,
        requested_model: &str,
        complexity: TaskComplexity,
        required_features: &[ModelFeature],
    ) -> Result<RoutingDecision, RoutingError> {
        let candidates: Vec<_> = self.registry.all()
            .into_iter()
            .filter(|m| required_features.iter().all(|f| m.supports(*f)))
            .filter(|m| self.meets_complexity(m.as_ref(), complexity))
            .filter(|m| m.is_fast_variant)
            .collect();

        if candidates.is_empty() {
            // Fall back to cost optimized
            return self.cost_optimized_routing(requested_model, complexity, required_features);
        }

        // Sort by performance (latency)
        let mut sorted = candidates;
        sorted.sort_by_key(|m| self.get_avg_latency(&m.id));

        let selected = &sorted[0];
        let fallbacks: Vec<String> = sorted.iter().skip(1).take(3).map(|m| m.id.clone()).collect();

        Ok(RoutingDecision {
            requested_model: requested_model.to_string(),
            selected_model: selected.id.clone(),
            strategy: RoutingStrategy::SpeedOptimized,
            complexity,
            reasoning: format!("Selected fastest suitable model: {}", selected.name),
            required_features: required_features.to_vec(),
            estimated_cost_usd: self.estimate_cost(selected.as_ref(), 1000, 500),
            fallback_models: fallbacks,
            confidence: 0.85,
        })
    }

    fn balanced_routing(
        &self,
        requested_model: &str,
        complexity: TaskComplexity,
        required_features: &[ModelFeature],
    ) -> Result<RoutingDecision, RoutingError> {
        let candidates: Vec<_> = self.registry.all()
            .into_iter()
            .filter(|m| required_features.iter().all(|f| m.supports(*f)))
            .filter(|m| self.meets_complexity(m.as_ref(), complexity))
            .collect();

        if candidates.is_empty() {
            return Err(RoutingError::NoSuitableModel);
        }

        // Score models based on balanced criteria
        let mut scored: Vec<_> = candidates.into_iter().map(|m| {
            let mut score = 0.0;
            
            // Prefer fast variants for simple/moderate tasks
            if m.is_fast_variant && complexity != TaskComplexity::Complex {
                score += 10.0;
            }
            
            // Prefer lower pricing tiers
            score += match m.pricing_tier() {
                PricingTier::Free => 20.0,
                PricingTier::UltraCheap => 15.0,
                PricingTier::Cheap => 10.0,
                PricingTier::Standard => 5.0,
                PricingTier::Premium => 0.0,
                PricingTier::UltraPremium => -5.0,
            };
            
            // Prefer models with good performance
            let latency = self.get_avg_latency(&m.id);
            if latency < 1000 { score += 5.0; }
            else if latency < 3000 { score += 2.0; }
            
            (m, score)
        }).collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let selected = &scored[0].0;
        let fallbacks: Vec<String> = scored.iter().skip(1).take(3).map(|(m, _)| m.id.clone()).collect();

        Ok(RoutingDecision {
            requested_model: requested_model.to_string(),
            selected_model: selected.id.clone(),
            strategy: RoutingStrategy::Balanced,
            complexity,
            reasoning: format!("Balanced selection: {} (score: {:.1})", selected.name, scored[0].1),
            required_features: required_features.to_vec(),
            estimated_cost_usd: self.estimate_cost(selected.as_ref(), 1000, 500),
            fallback_models: fallbacks,
            confidence: 0.85,
        })
    }

    fn cascade_routing(
        &self,
        requested_model: &str,
        complexity: TaskComplexity,
        required_features: &[ModelFeature],
        max_attempts: usize,
    ) -> Result<RoutingDecision, RoutingError> {
        // Start with cheapest, have fallbacks ready
        let cost_decision = self.cost_optimized_routing(requested_model, complexity, required_features)?;
        let selected_model = cost_decision.selected_model.clone();
        
        let mut fallbacks = cost_decision.fallback_models.clone();
        
        // Add more capable models as fallbacks
        let more_capable: Vec<_> = self.registry.all()
            .into_iter()
            .filter(|m| required_features.iter().all(|f| m.supports(*f)))
            .filter(|m| m.pricing_tier() > self.registry.get(&cost_decision.selected_model).map(|m| m.pricing_tier()).unwrap_or(PricingTier::Free))
            .map(|m| m.id.clone())
            .collect();
        
        fallbacks.extend(more_capable.into_iter().take(max_attempts.saturating_sub(1)));

        Ok(RoutingDecision {
            requested_model: requested_model.to_string(),
            selected_model: selected_model.clone(),
            strategy: RoutingStrategy::Cascade { max_attempts },
            complexity,
            reasoning: format!("Cascade: start with {}, {} fallbacks ready", selected_model, fallbacks.len()),
            required_features: required_features.to_vec(),
            estimated_cost_usd: cost_decision.estimated_cost_usd,
            fallback_models: fallbacks,
            confidence: 0.8,
        })
    }

    fn tier_routing(
        &self,
        requested_model: &str,
        complexity: TaskComplexity,
        required_features: &[ModelFeature],
        tier: PricingTier,
    ) -> Result<RoutingDecision, RoutingError> {
        let candidates: Vec<_> = self.registry.all()
            .into_iter()
            .filter(|m| required_features.iter().all(|f| m.supports(*f)))
            .filter(|m| self.meets_complexity(m.as_ref(), complexity))
            .filter(|m| m.pricing_tier() <= tier)
            .collect();

        if candidates.is_empty() {
            return Err(RoutingError::NoSuitableModel);
        }

        let selected = candidates.iter().min_by_key(|m| m.pricing_tier()).unwrap();
        
        Ok(RoutingDecision {
            requested_model: requested_model.to_string(),
            selected_model: selected.id.clone(),
            strategy: RoutingStrategy::CostOptimized,
            complexity,
            reasoning: format!("Tier-limited selection: {} (tier: {:?})", selected.name, tier),
            required_features: required_features.to_vec(),
            estimated_cost_usd: self.estimate_cost(selected.as_ref(), 1000, 500),
            fallback_models: vec![],
            confidence: 0.9,
        })
    }

    fn custom_routing(
        &self,
        requested_model: &str,
        complexity: TaskComplexity,
        required_features: &[ModelFeature],
    ) -> Result<RoutingDecision, RoutingError> {
        // Default to balanced for custom
        self.balanced_routing(requested_model, complexity, required_features)
    }

    fn meets_complexity(&self, model: &ModelCapabilities, complexity: TaskComplexity) -> bool {
        match complexity {
            TaskComplexity::Simple => true, // Any model can handle simple
            TaskComplexity::Moderate => {
                // Prefer models with function calling and decent context
                model.features.function_calling && model.context_window >= 16_000
            }
            TaskComplexity::Complex => {
                // Need high capability model
                model.features.function_calling 
                && model.context_window >= 64_000
                && model.max_output_tokens >= 4096
                && (model.pricing_tier() >= PricingTier::Standard || model.is_fast_variant)
            }
        }
    }

    fn get_fallbacks(&self, model_id: &str, required_features: &[ModelFeature]) -> Vec<String> {
        self.registry.all()
            .into_iter()
            .filter(|m| m.id != model_id)
            .filter(|m| required_features.iter().all(|f| m.supports(*f)))
            .take(3)
            .map(|m| m.id.clone())
            .collect()
    }

    fn get_avg_latency(&self, model_id: &str) -> u64 {
        self.performance_cache.get(model_id)
            .map(|p| p.avg_latency_ms)
            .unwrap_or(2000) // Default 2s
    }

    fn estimate_cost(&self, model: &ModelCapabilities, prompt_tokens: usize, completion_tokens: usize) -> Option<f64> {
        model.pricing.as_ref().map(|p| {
            (prompt_tokens as f64 * p.input_per_million / 1_000_000.0) +
            (completion_tokens as f64 * p.output_per_million / 1_000_000.0)
        })
    }

    /// Record performance metrics for a model
    pub fn record_performance(&mut self, model_id: &str, latency_ms: u64, success: bool) {
        let entry = self.performance_cache.entry(model_id.to_string()).or_insert(ModelPerformance {
            avg_latency_ms: latency_ms,
            success_rate: if success { 1.0 } else { 0.0 },
            error_rate: if success { 0.0 } else { 1.0 },
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        
        // Exponential moving average
        entry.avg_latency_ms = (entry.avg_latency_ms as f64 * 0.9 + latency_ms as f64 * 0.1) as u64;
        entry.success_rate = entry.success_rate * 0.9 + if success { 0.1 } else { 0.0 };
        entry.error_rate = entry.error_rate * 0.9 + if success { 0.0 } else { 0.1 };
        entry.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RoutingError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("No suitable model found for requirements")]
    NoSuitableModel,
    #[error("Routing rule error: {0}")]
    RuleError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ModelRegistry;
    use crate::counter::{Message, Role};

    fn setup() -> ModelRouter {
        let registry = Arc::new(ModelRegistry::new());
        ModelRouter::new(registry)
    }

    #[test]
    fn test_fixed_routing() {
        let router = setup().with_strategy(RoutingStrategy::Fixed);
        let messages = vec![Message { role: Role::User, content: "Hello".to_string(), name: None, tool_calls: None, tool_call_id: None }];
        
        let decision = router.route("gpt-4o", TaskComplexity::Simple, &messages).unwrap();
        assert_eq!(decision.selected_model, "gpt-4o");
        assert_eq!(decision.strategy, RoutingStrategy::Fixed);
    }

    #[test]
    fn test_cost_optimized_routing() {
        let router = setup().with_strategy(RoutingStrategy::CostOptimized);
        let messages = vec![Message { role: Role::User, content: "Hello".to_string(), name: None, tool_calls: None, tool_call_id: None }];
        
        let decision = router.route("gpt-4o", TaskComplexity::Simple, &messages).unwrap();
        // Should select a cheaper model than gpt-4o (Premium tier)
        let selected = router.registry.get(&decision.selected_model).unwrap();
        assert!(selected.pricing_tier() < PricingTier::Premium);
    }

    #[test]
    fn test_complexity_routing() {
        let router = setup().with_strategy(RoutingStrategy::Balanced);
        
        // Simple task - should prefer fast/cheap
        let simple_msg = vec![Message { role: Role::User, content: "What is 2+2?".to_string(), name: None, tool_calls: None, tool_call_id: None }];
        let simple_decision = router.route("gpt-4o", TaskComplexity::Simple, &simple_msg).unwrap();
        
        // Complex task - should prefer capable model
        let complex_msg = vec![
            Message { role: Role::User, content: "Design a distributed system".to_string(), name: None, tool_calls: None, tool_call_id: None },
            Message { role: Role::User, content: "With these requirements...".to_string(), name: None, tool_calls: None, tool_call_id: None },
        ];
        let complex_decision = router.route("gpt-4o-mini", TaskComplexity::Complex, &complex_msg).unwrap();
        
        println!("Simple: {}, Complex: {}", simple_decision.selected_model, complex_decision.selected_model);
    }

    #[test]
    fn test_required_features() {
        let router = setup();
        let messages = vec![
            Message { 
                role: Role::User, 
                content: "Use the search tool to find info".to_string(), 
                name: None, 
                tool_calls: Some(vec![]), 
                tool_call_id: None 
            },
        ];
        
        let decision = router.route("gpt-3.5-turbo", TaskComplexity::Moderate, &messages).unwrap();
        // Should select a model with function calling
        let model = router.registry.get(&decision.selected_model).unwrap();
        assert!(model.features.function_calling);
    }
}