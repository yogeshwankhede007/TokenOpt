//! Prompt optimization for reducing token usage while preserving intent

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::counter::{TokenCounter, TokenCount};
use regex::Regex;
use once_cell::sync::Lazy;

/// A prompt template with variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub template: String,
    pub variables: HashMap<String, String>,
    pub metadata: TemplateMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemplateMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub version: Option<String>,
}

impl PromptTemplate {
    pub fn from_string(template: &str) -> Self {
        // Extract variables like {{variable}}
        let var_regex = Regex::new(r"\{\{(\w+)\}\}").unwrap();
        let mut variables = HashMap::new();
        
        for cap in var_regex.captures_iter(template) {
            if let Some(var) = cap.get(1) {
                variables.insert(var.as_str().to_string(), String::new());
            }
        }

        Self {
            template: template.to_string(),
            variables,
            metadata: TemplateMetadata::default(),
        }
    }

    pub fn with_variable(mut self, key: &str, value: &str) -> Self {
        self.variables.insert(key.to_string(), value.to_string());
        self
    }

    pub fn render(&self) -> String {
        let mut result = self.template.clone();
        for (key, value) in &self.variables {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }
        result
    }

    pub fn variable_names(&self) -> Vec<String> {
        self.variables.keys().cloned().collect()
    }
}

/// Optimized prompt result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedPrompt {
    pub original: String,
    pub optimized: String,
    pub prompt: String, // Alias for optimized
    pub token_count: usize,
    pub original_token_count: usize,
    pub tokens_saved: usize,
    pub savings_percentage: f32,
    pub techniques_applied: Vec<OptimizationTechnique>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationTechnique {
    RemoveRedundancy,
    CompressInstructions,
    SimplifyLanguage,
    RemoveExamples,
    UseShorterSynonyms,
    RestructureFormat,
    Custom(String),
}

/// Configuration for prompt optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptOptimizationConfig {
    pub aggressiveness: AggressivenessLevel,
    pub preserve_examples: bool,
    pub preserve_formatting: bool,
    pub max_length_reduction: f32, // 0.0 to 1.0
    pub custom_rules: Vec<CustomRule>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggressivenessLevel {
    Conservative,  // Safe optimizations only
    Balanced,      // Good balance
    Aggressive,    // Maximum token reduction
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRule {
    pub pattern: String,
    pub replacement: String,
    pub description: String,
}

impl Default for PromptOptimizationConfig {
    fn default() -> Self {
        Self {
            aggressiveness: AggressivenessLevel::Balanced,
            preserve_examples: true,
            preserve_formatting: true,
            max_length_reduction: 0.5,
            custom_rules: Vec::new(),
        }
    }
}

/// Prompt optimizer
pub struct PromptOptimizer {
    counter: Arc<TokenCounter>,
    config: PromptOptimizationConfig,
    patterns: Vec<OptimizationPattern>,
}

impl PromptOptimizer {
    pub fn new(counter: Arc<TokenCounter>) -> Self {
        let mut optimizer = Self {
            counter,
            config: PromptOptimizationConfig::default(),
            patterns: Vec::new(),
        };
        optimizer.load_default_patterns();
        optimizer
    }

    pub fn with_config(mut self, config: PromptOptimizationConfig) -> Self {
        self.config = config;
        self.load_default_patterns();
        self
    }

    fn load_default_patterns(&mut self) {
        self.patterns = vec![
            // Remove redundant phrases
            OptimizationPattern {
                name: "remove_redundant_please",
                pattern: Regex::new(r"(?i)\b(please|kindly|would you mind|could you please)\b").unwrap(),
                replacement: "",
                technique: OptimizationTechnique::RemoveRedundancy,
                condition: PatternCondition::Always,
            },
            OptimizationPattern {
                name: "remove_filler_words",
                pattern: Regex::new(r"(?i)\b(basically|actually|literally|essentially|fundamentally)\b").unwrap(),
                replacement: "",
                technique: OptimizationTechnique::SimplifyLanguage,
                condition: PatternCondition::Always,
            },
            // Compress common instruction patterns
            OptimizationPattern {
                name: "compress_explain_in_detail",
                pattern: Regex::new(r"(?i)explain in (great |full |complete )?detail").unwrap(),
                replacement: "explain thoroughly",
                technique: OptimizationTechnique::CompressInstructions,
                condition: PatternCondition::Always,
            },
            OptimizationPattern {
                name: "compress_step_by_step",
                pattern: Regex::new(r"(?i)step.?by.?step").unwrap(),
                replacement: "stepwise",
                technique: OptimizationTechnique::CompressInstructions,
                condition: PatternCondition::Always,
            },
            // Simplify language
            OptimizationPattern {
                name: "simplify_in_order_to",
                pattern: Regex::new(r"(?i)in order to").unwrap(),
                replacement: "to",
                technique: OptimizationTechnique::SimplifyLanguage,
                condition: PatternCondition::Always,
            },
            OptimizationPattern {
                name: "simplify_due_to_the_fact",
                pattern: Regex::new(r"(?i)due to the fact that").unwrap(),
                replacement: "because",
                technique: OptimizationTechnique::SimplifyLanguage,
                condition: PatternCondition::Always,
            },
            // Remove excessive politeness
            OptimizationPattern {
                name: "remove_thank_you",
                pattern: Regex::new(r"(?i)^(thank you|thanks),?\s+").unwrap(),
                replacement: "",
                technique: OptimizationTechnique::RemoveRedundancy,
                condition: PatternCondition::Aggressive,
            },
            // Compress code-related instructions
            OptimizationPattern {
                name: "compress_code_instructions",
                pattern: Regex::new(r"(?i)write (a |the )?(function|method|class|component) that").unwrap(),
                replacement: "implement",
                technique: OptimizationTechnique::CompressInstructions,
                condition: PatternCondition::Always,
            },
        ];
    }

    /// Optimize a prompt template
    pub fn optimize(&self, template: &PromptTemplate, model: &str) -> Result<OptimizedPrompt, PromptError> {
        let original = template.render();
        let original_count = self.counter.count_tokens(&original, Some(model))?;
        
        let mut optimized = original.clone();
        let mut techniques_applied = Vec::new();
        let mut warnings = Vec::new();

        // Apply patterns based on aggressiveness
        for pattern in &self.patterns {
            let should_apply = match (self.config.aggressiveness, pattern.condition) {
                (AggressivenessLevel::Conservative, PatternCondition::Always) => true,
                (AggressivenessLevel::Balanced, PatternCondition::Always) => true,
                (AggressivenessLevel::Balanced, PatternCondition::Balanced) => true,
                (AggressivenessLevel::Aggressive, _) => true,
                _ => false,
            };

            if should_apply {
                let before = optimized.clone();
                optimized = pattern.pattern.replace_all(&optimized, pattern.replacement).to_string();
                if before != optimized {
                    techniques_applied.push(pattern.technique.clone());
                }
            }
        }

        // Apply custom rules
        for rule in &self.config.custom_rules {
            if let Ok(regex) = Regex::new(&rule.pattern) {
                optimized = regex.replace_all(&optimized, &rule.replacement).to_string();
                techniques_applied.push(OptimizationTechnique::Custom(rule.description.clone()));
            }
        }

        // Clean up extra whitespace
        optimized = self.clean_whitespace(&optimized);

        // Apply aggressive optimizations if enabled
        if self.config.aggressiveness == AggressivenessLevel::Aggressive {
            optimized = self.apply_aggressive_optimizations(&optimized, &mut techniques_applied, &mut warnings)?;
        }

        let optimized_count = self.counter.count_tokens(&optimized, Some(model))?;
        let tokens_saved = original_count.saturating_sub(optimized_count);
        let savings_percentage = if original_count > 0 {
            (tokens_saved as f32 / original_count as f32) * 100.0
        } else { 0.0 };

        // Check if we exceeded max reduction
        if savings_percentage / 100.0 > self.config.max_length_reduction {
            warnings.push(format!(
                "Optimization reduced prompt by {:.1}%, exceeding max of {:.1}%",
                savings_percentage, self.config.max_length_reduction * 100.0
            ));
        }

        Ok(OptimizedPrompt {
            original: original.clone(),
            optimized: optimized.clone(),
            prompt: optimized,
            token_count: optimized_count,
            original_token_count: original_count,
            tokens_saved,
            savings_percentage,
            techniques_applied,
            warnings,
        })
    }

    fn clean_whitespace(&self, text: &str) -> String {
        // Remove multiple newlines
        let text = Regex::new(r"\n{3,}").unwrap().replace_all(text, "\n\n");
        // Remove trailing whitespace on lines
        let text = Regex::new(r"[ \t]+\n").unwrap().replace_all(&text, "\n");
        // Remove leading/trailing whitespace
        text.trim().to_string()
    }

    fn apply_aggressive_optimizations(
        &self,
        text: &str,
        techniques: &mut Vec<OptimizationTechnique>,
        warnings: &mut Vec<String>,
    ) -> Result<String, PromptError> {
        let mut result = text.to_string();

        // Remove examples if not preserved
        if !self.config.preserve_examples {
            let example_regex = Regex::new(r"(?i)(example|for instance|e\.g\.).*?(?=\n\n|\n[A-Z]|\z)").unwrap();
            result = example_regex.replace_all(&result, "").to_string();
            techniques.push(OptimizationTechnique::RemoveExamples);
        }

        // Compress formatting if not preserved
        if !self.config.preserve_formatting {
            // Convert markdown headers to plain text
            result = Regex::new(r"^#{1,6}\s+").unwrap().replace_all(&result, "").to_string();
            // Remove bold/italic markers
            result = Regex::new(r"\*\*([^*]+)\*\*").unwrap().replace_all(&result, "$1").to_string();
            result = Regex::new(r"\*([^*]+)\*").unwrap().replace_all(&result, "$1").to_string();
            techniques.push(OptimizationTechnique::RestructureFormat);
        }

        // Use shorter synonyms
        let synonyms = [
            ("utilize", "use"),
            ("implement", "build"),
            ("demonstrate", "show"),
            ("illustrate", "show"),
            ("elaborate", "expand"),
            ("comprehensive", "full"),
            ("additional", "more"),
            ("necessary", "needed"),
            ("sufficient", "enough"),
        ];

        for (long, short) in synonyms {
            let regex = Regex::new(&format!(r"(?i)\b{}\b", regex::escape(long))).unwrap();
            if regex.is_match(&result) {
                result = regex.replace_all(&result, short).to_string();
                techniques.push(OptimizationTechnique::UseShorterSynonyms);
            }
        }

        Ok(self.clean_whitespace(&result))
    }

    /// Optimize a raw string prompt
    pub fn optimize_string(&self, prompt: &str, model: &str) -> Result<OptimizedPrompt, PromptError> {
        let template = PromptTemplate::from_string(prompt);
        self.optimize(&template, model)
    }

    /// Create an optimized version of common prompt patterns
    pub fn optimize_for_task(&self, task: PromptTask, context: &str, model: &str) -> Result<OptimizedPrompt, PromptError> {
        let template = task.to_template(context);
        self.optimize(&template, model)
    }
}

#[derive(Debug, Clone)]
struct OptimizationPattern {
    name: &'static str,
    pattern: Regex,
    replacement: &'static str,
    technique: OptimizationTechnique,
    condition: PatternCondition,
}

#[derive(Debug, Clone, Copy)]
enum PatternCondition {
    Always,
    Balanced,
    Aggressive,
}

/// Common prompt tasks with optimized templates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptTask {
    CodeGeneration,
    CodeReview,
    Debugging,
    Explanation,
    Refactoring,
    TestGeneration,
    Documentation,
    Architecture,
}

impl PromptTask {
    pub fn to_template(self, context: &str) -> PromptTemplate {
        let template = match self {
            PromptTask::CodeGeneration => {
                format!("Task: Generate code.\nContext: {}\nRequirements: {{requirements}}\nOutput: Clean, idiomatic code only.", context)
            }
            PromptTask::CodeReview => {
                format!("Task: Review code.\nCode: {}\nFocus: {{focus_areas}}\nOutput: Issues with severity and fixes.", context)
            }
            PromptTask::Debugging => {
                format!("Task: Debug issue.\nError: {}\nCode: {{code}}\nOutput: Root cause and fix.", context)
            }
            PromptTask::Explanation => {
                format!("Task: Explain concept.\nTopic: {}\nAudience: {{audience}}\nOutput: Clear explanation with examples.", context)
            }
            PromptTask::Refactoring => {
                format!("Task: Refactor code.\nCode: {}\nGoals: {{goals}}\nOutput: Refactored code with changes summary.", context)
            }
            PromptTask::TestGeneration => {
                format!("Task: Generate tests.\nCode: {}\nFramework: {{framework}}\nOutput: Comprehensive test suite.", context)
            }
            PromptTask::Documentation => {
                format!("Task: Write documentation.\nCode: {}\nFormat: {{format}}\nOutput: Documentation only.", context)
            }
            PromptTask::Architecture => {
                format!("Task: Design architecture.\nRequirements: {}\nConstraints: {{constraints}}\nOutput: Architecture diagram description and decisions.", context)
            }
        };
        
        PromptTemplate::from_string(&template)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PromptError {
    #[error("Token counting error: {0}")]
    CounterError(#[from] crate::counter::CounterError),
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Optimization failed: {0}")]
    OptimizationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::counter::{TokenCounter, TokenCounterConfig};

    fn setup() -> PromptOptimizer {
        let counter = Arc::new(TokenCounter::new(TokenCounterConfig::default()).unwrap());
        PromptOptimizer::new(counter)
    }

    #[test]
    fn test_optimize_simple_prompt() {
        let optimizer = setup();
        let prompt = "Please could you kindly explain in great detail how to write a binary search function in Rust?";
        let result = optimizer.optimize_string(prompt, "gpt-4").unwrap();
        
        assert!(result.tokens_saved > 0);
        assert!(result.savings_percentage > 0.0);
        assert!(!result.optimized.contains("please"));
        assert!(!result.optimized.contains("kindly"));
    }

    #[test]
    fn test_optimize_code_prompt() {
        let optimizer = setup();
        let prompt = "Please write a function that implements binary search in order to find an element in a sorted array";
        let result = optimizer.optimize_string(prompt, "gpt-4").unwrap();
        
        assert!(result.optimized.contains("implement"));
        assert!(!result.optimized.contains("in order to"));
    }

    #[test]
    fn test_prompt_template() {
        let template = PromptTemplate::from_string("Hello {{name}}, please write a {{language}} function")
            .with_variable("name", "User")
            .with_variable("language", "Rust");
        
        let rendered = template.render();
        assert_eq!(rendered, "Hello User, please write a Rust function");
    }

    #[test]
    fn test_task_templates() {
        let optimizer = setup();
        let result = optimizer.optimize_for_task(
            PromptTask::CodeGeneration, 
            "Create a REST API endpoint", 
            "gpt-4"
        ).unwrap();
        
        assert!(result.token_count > 0);
        assert!(result.optimized.contains("Generate code"));
    }

    #[test]
    fn test_aggressiveness_levels() {
        let counter = Arc::new(TokenCounter::new(TokenCounterConfig::default()).unwrap());
        
        let conservative = PromptOptimizer::new(counter.clone())
            .with_config(PromptOptimizationConfig { aggressiveness: AggressivenessLevel::Conservative, ..Default::default() });
        
        let aggressive = PromptOptimizer::new(counter)
            .with_config(PromptOptimizationConfig { aggressiveness: AggressivenessLevel::Aggressive, ..Default::default() });
        
        let prompt = "Thank you, could you please explain in great detail how to implement a binary search?";
        
        let cons_result = conservative.optimize_string(prompt, "gpt-4").unwrap();
        let agg_result = aggressive.optimize_string(prompt, "gpt-4").unwrap();
        
        // Aggressive should save more tokens
        assert!(agg_result.tokens_saved >= cons_result.tokens_saved);
    }
}