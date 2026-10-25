# Advanced Examples

Real-world use cases demonstrating TokenOpt's capabilities.

## Table of Contents

1. [Code Review Assistant](#code-review-assistant)
2. [Documentation Generator](#documentation-generator)
3. [Test Generator](#test-generator)
4. [Refactoring Assistant](#refactoring-assistant)
5. [Debugging Helper](#debugging-helper)
5. [Architecture Design](#architecture-design)
6. [Multi-language Project](#multi-language-project)
7. [Cost Optimization Pipeline](#cost-optimization-pipeline)
8. [Streaming Context Management](#streaming-context-management)

---

## Code Review Assistant

Automated code review with token-optimized prompts.

```rust
use tokenopt_core::prelude::*;

struct CodeReviewAssistant {
    tokenopt: TokenOpt,
}

impl CodeReviewAssistant {
    fn new() -> Result<Self> {
        Ok(Self { tokenopt: TokenOpt::new()? })
    }

    fn review(&self, code: &str, language: &str, focus: &[&str]) -> Result<String> {
        let prompt = format!(
            r#"You are an expert code reviewer. Review the following {} code.

Focus areas: {}
Code:
```{}
{}
```

Provide:
1. Critical issues (security, bugs)
2. Performance improvements
3. Style/maintainability suggestions
4. Test coverage gaps
5. Overall rating (1-10)"#,
            language,
            focus.join(", "),
            language,
            code
        );

        // Optimize prompt for code review task
        let optimized = self.tokenopt.prompt_optimizer().optimize_for_task(
            PromptTask::CodeReview,
            &prompt,
            "gpt-4o"
        )?;

        // In real usage, send to LLM
        // let response = llm_client.complete(&optimized.prompt).await?;
        
        Ok(format!("Optimized prompt ({} tokens saved):\n{}", 
            optimized.tokens_saved, optimized.prompt))
    }

    fn batch_review(&self, files: Vec<(String, String)>) -> Result<Vec<String>> {
        let mut results = Vec::new();
        
        for (path, code) in files {
            let focus = self.detect_focus_areas(&code);
            let review = self.review(&code, &self.detect_language(&path), &focus)?;
            results.push(format!("=== {} ===\n{}", path, review));
        }
        
        Ok(results)
    }

    fn detect_language(&self, path: &str) -> &str {
        match path.split('.').last() {
            Some("rs") => "Rust",
            Some("py") => "Python",
            Some("ts") | Some("tsx") => "TypeScript",
            Some("js") | Some("jsx") => "JavaScript",
            Some("go") => "Go",
            _ => "Unknown",
        }
    }

    fn detect_focus_areas(&self, code: &str) -> Vec<&str> {
        let mut focus = vec!["security", "performance", "maintainability"];
        
        if code.contains("unsafe") { focus.push("unsafe-code"); }
        if code.contains("async") { focus.push("async-patterns"); }
        if code.contains("Unwrap") || code.contains("expect(") { focus.push("error-handling"); }
        if code.contains("Mutex") || code.contains("RwLock") { focus.push("concurrency"); }
        
        focus
    }
}

// Usage
fn main() -> Result<()> {
    let assistant = CodeReviewAssistant::new()?;
    
    let code = r#"
        pub fn process_user_input(input: &str) -> Result<String> {
            let cmd = format!("echo {}", input);
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()?;
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
    "#;
    
    println!("{}", assistant.review(code, "Rust", &["security", "performance"])?);
    Ok(())
}
```

---

## Documentation Generator

Generate documentation with optimized prompts.

```rust
use tokenopt_core::prelude::*;

struct DocGenerator {
    tokenopt: TokenOpt,
}

impl DocGenerator {
    fn new() -> Result<Self> {
        Ok(Self { tokenopt: TokenOpt::new()? })
    }

    fn generate_docs(&self, code: &str, format: DocFormat) -> Result<String> {
        let prompt = match format {
            DocFormat::Markdown => self.markdown_prompt(code),
            DocFormat::OpenAPI => self.openapi_prompt(code),
            DocFormat::RustDoc => self.rustdoc_prompt(code),
            DocFormat::JSDoc => self.jsdoc_prompt(code),
        };

        let optimized = self.tokenopt.prompt_optimizer().optimize_string(&prompt, "gpt-4o")?;
        
        // In real usage: send to LLM
        Ok(optimized.prompt)
    }

    fn markdown_prompt(&self, code: &str) -> String {
        format!(r#"Generate comprehensive Markdown documentation for this code:

```rust
{}
```

Include:
1. Module/class overview
2. Function signatures with descriptions
3. Parameter descriptions
4. Return value descriptions
5. Error conditions
6. Usage examples
7. Type definitions
8. Related items"#, code)
    }

    fn rustdoc_prompt(&self, code: &str) -> String {
        format!(r#"Generate idiomatic Rustdoc comments for this code:

```rust
{}
```

Follow Rust documentation conventions:
- Use /// for outer doc comments
- Include # Examples section
- Document panics, errors, safety
- Link related items with @see"#, code)
    }

    fn openapi_prompt(&self, code: &str) -> String {
        format!(r#"Generate OpenAPI 3.0 specification for this API code:

```rust
{}
```

Include:
- paths, operations, parameters
- request/response schemas
- security schemes
- examples
- server definitions"#, code)
    }

    fn jsdoc_prompt(&self, code: &str) -> String {
        format!(r#"Generate JSDoc comments for this JavaScript/TypeScript code:

```javascript
{}
```

Follow JSDoc conventions:
- @param, @returns, @throws
- @example with runnable code
- @see for related functions
- @deprecated if applicable"#, code)
    }
}

enum DocFormat {
    Markdown,
    OpenAPI,
    RustDoc,
    JSDoc,
}
```

---

## Test Generator

Generate comprehensive tests with token optimization.

```rust
use tokenopt_core::prelude::*;

struct TestGenerator {
    tokenopt: TokenOpt,
}

impl TestGenerator {
    fn new() -> Result<Self> {
        Ok(Self { tokenopt: TokenOpt::new()? })
    }

    fn generate_tests(&self, code: &str, framework: TestFramework, coverage: CoverageLevel) -> Result<String> {
        let prompt = self.build_prompt(code, framework, coverage);
        
        let optimized = self.tokenopt.prompt_optimizer().optimize_for_task(
            PromptTask::TestGeneration,
            &prompt,
            "gpt-4o"
        )?;

        Ok(optimized.prompt)
    }

    fn build_prompt(&self, code: &str, framework: TestFramework, coverage: CoverageLevel) -> String {
        let framework_name = match framework {
            TestFramework::CargoTest => "Rust (cargo test)",
            TestFramework::Pytest => "Python (pytest)",
            TestFramework::Jest => "TypeScript/JavaScript (Jest)",
            TestFramework::GoTest => "Go (testing)",
        };

        let coverage_desc = match coverage {
            CoverageLevel::Basic => "Basic happy-path tests",
            CoverageLevel::Comprehensive => "Comprehensive: happy path, edge cases, error conditions, property-based",
            CoverageLevel::PropertyBased => "Property-based tests with proptest/hypothesis",
        };

        format!(r#"Generate {} tests for this code using {}:

```rust
{}
```

Requirements:
- {}
- Test public API thoroughly
- Include edge cases: empty, null, max values, invalid input
- Test error conditions and panic scenarios
- Use descriptive test names
- Mock external dependencies
- {}"#, 
            coverage_desc, framework_name, code, coverage_desc,
            self.framework_specific_advice(framework))
    }

    fn framework_specific_advice(&self, framework: TestFramework) -> String {
        match framework {
            TestFramework::CargoTest => "Use #[test], #[tokio::test], proptest for property tests".to_string(),
            TestFramework::Pytest => "Use pytest fixtures, parametrize, hypothesis for property tests".to_string(),
            TestFramework::Jest => "Use describe/it, jest.mock, fast-check for property tests".to_string(),
            TestFramework::GoTest => "Use testing.T, testify, gopter for property tests".to_string(),
        }
    }
}

enum TestFramework {
    CargoTest,
    Pytest,
    Jest,
    GoTest,
}

enum CoverageLevel {
    Basic,
    Comprehensive,
    PropertyBased,
}
```

---

## Refactoring Assistant

Intelligent refactoring with token optimization.

```rust
use tokenopt_core::prelude::*;

struct RefactoringAssistant {
    tokenopt: TokenOpt,
}

impl RefactoringAssistant {
    fn new() -> Result<Self> {
        Ok(Self { tokenopt: TokenOpt::new()? })
    }

    fn suggest_refactoring(&self, code: &str, goals: &[RefactorGoal]) -> Result<RefactoringPlan> {
        let prompt = self.build_prompt(code, goals);
        
        let optimized = self.tokenopt.prompt_optimizer().optimize_for_task(
            PromptTask::Refactoring,
            &prompt,
            "gpt-4o"
        )?;

        Ok(RefactoringPlan {
            original_code: code.to_string(),
            optimized_prompt: optimized.prompt,
            goals: goals.to_vec(),
            tokens_saved: optimized.tokens_saved,
        })
    }

    fn build_prompt(&self, code: &str, goals: &[RefactorGoal]) -> String {
        let goals_desc = goals.iter()
            .map(|g| format!("- {}", g.description()))
            .collect::<Vec<_>>()
            .join("\n");

        format!(r#"Refactor this code with the following goals:

{}
Code:
```rust
{}
```

For each refactoring:
1. Show before/after code
2. Explain the change and rationale
3. Estimate impact (performance, readability, maintainability)
4. Note any breaking changes
5. Provide migration guide if needed"#, 
            goals_desc, code)
    }
}

#[derive(Debug, Clone)]
enum RefactorGoal {
    ImprovePerformance,
    ReduceComplexity,
    ImproveReadability,
    ModernizeSyntax,
    ExtractMethods,
    ReduceDuplication,
    ImproveErrorHandling,
    AddTypeSafety,
    OptimizeMemory,
    Custom(String),
}

impl RefactorGoal {
    fn description(&self) -> String {
        match self {
            Self::ImprovePerformance => "Improve runtime performance".to_string(),
            Self::ReduceComplexity => "Reduce cyclomatic complexity".to_string(),
            Self::ImproveReadability => "Improve code readability".to_string(),
            Self::ModernizeSyntax => "Modernize to latest Rust edition".to_string(),
            Self::ExtractMethods => "Extract large functions into smaller ones".to_string(),
            Self::ReduceDuplication => "Eliminate code duplication (DRY)".to_string(),
            Self::ImproveErrorHandling => "Improve error handling with proper types".to_string(),
            Self::AddTypeSafety => "Add stronger type safety".to_string(),
            Self::OptimizeMemory => "Optimize memory usage".to_string(),
            Self::Custom(s) => s.clone(),
        }
    }
}

struct RefactoringPlan {
    original_code: String,
    optimized_prompt: String,
    goals: Vec<RefactorGoal>,
    tokens_saved: usize,
}
```

---

## Debugging Helper

Debug assistant with context-aware optimization.

```rust
use tokenopt_core::prelude::*;

struct DebugAssistant {
    tokenopt: TokenOpt,
}

impl DebugAssistant {
    fn new() -> Result<Self> {
        Ok(Self { tokenopt: TokenOpt::new()? })
    }

    fn analyze_error(&self, error: &str, code: &str, context: &str) -> Result<DebugAnalysis> {
        let prompt = format!(r#"Analyze this error and provide debugging guidance:

Error:
```
{}
```

Code:
```rust
{}
```

Context: {}

Provide:
1. Root cause analysis
2. Likely fix locations
3. Step-by-step debugging approach
4. Prevention strategies
5. Related common issues"#, 
            error, code, context);

        let optimized = self.tokenopt.prompt_optimizer().optimize_for_task(
            PromptTask::Debugging,
            &prompt,
            "gpt-4o"
        )?;

        Ok(DebugAnalysis {
            error: error.to_string(),
            optimized_prompt: optimized.prompt,
            tokens_saved: optimized.tokens_saved,
        })
    }

    fn analyze_stack_trace(&self, stack_trace: &str, codebase: &str) -> Result<StackTraceAnalysis> {
        let prompt = format!(r#"Analyze this stack trace and identify the issue:

Stack Trace:
```
{}
```

Relevant Codebase:
```rust
{}
```

Provide:
1. Exact line causing the issue
2. Call chain explanation
3. Root cause
4. Fix recommendation
5. Similar patterns to check"#, 
            stack_trace, codebase);

        let optimized = self.tokenopt.prompt_optimizer().optimize_for_task(
            PromptTask::Debugging,
            &prompt,
            "gpt-4o"
        )?;

        Ok(StackTraceAnalysis {
            stack_trace: stack_trace.to_string(),
            optimized_prompt: optimized.prompt,
            tokens_saved: optimized.tokens_saved,
        })
    }

    fn suggest_debug_steps(&self, symptom: &str, code: &str) -> Result<Vec<DebugStep>> {
        let prompt = format!(r#"Debug this symptom:

Symptom: {}
Code:
```rust
{}
```

Provide step-by-step debugging plan:
1. Hypothesis
2. Verification method
3. Expected outcome
4. Next step if hypothesis wrong"#, 
            symptom, code);

        let optimized = self.tokenopt.prompt_optimizer().optimize_for_task(
            PromptTask::Debugging,
            &prompt,
            "gpt-4o"
        )?;

        Ok(vec![DebugStep {
            description: optimized.prompt,
            order: 1,
        }])
    }
}

struct DebugAnalysis {
    error: String,
    optimized_prompt: String,
    tokens_saved: usize,
}

struct StackTraceAnalysis {
    stack_trace: String,
    optimized_prompt: String,
    tokens_saved: usize,
}

struct DebugStep {
    description: String,
    order: usize,
}
```

---

## Architecture Design

System design assistant with token optimization.

```rust
use tokenopt_core::prelude::*;

struct ArchitectureDesigner {
    tokenopt: TokenOpt,
}

impl ArchitectureDesigner {
    fn new() -> Result<Self> {
        Ok(Self { tokenopt: TokenOpt::new()? })
    }

    fn design(&self, requirements: &Requirements) -> Result<ArchitectureDesign> {
        let prompt = self.build_prompt(requirements);
        
        let optimized = self.tokenopt.prompt_optimizer().optimize_for_task(
            PromptTask::Architecture,
            &prompt,
            "gpt-4o"
        )?;

        Ok(ArchitectureDesign {
            requirements: requirements.clone(),
            optimized_prompt: optimized.prompt,
            tokens_saved: optimized.tokens_saved,
        })
    }

    fn design_with_constraints(&self, requirements: &Requirements, constraints: &Constraints) -> Result<ArchitectureDesign> {
        let prompt = self.build_prompt_with_constraints(requirements, constraints);
        
        let optimized = self.tokenopt.prompt_optimizer().optimize_for_task(
            PromptTask::Architecture,
            &prompt,
            "gpt-4o"
        )?;

        Ok(ArchitectureDesign {
            requirements: requirements.clone(),
            optimized_prompt: optimized.prompt,
            tokens_saved: optimized.tokens_saved,
        })
    }

    fn build_prompt(&self, req: &Requirements) -> String {
        format!(r#"Design a software architecture for:

Requirements:
- Functional: {}
- Non-functional: {}
- Scale: {} users, {} req/s
- Data: {} TB, {} GB/s
- Team: {} engineers

Provide:
1. High-level architecture diagram (text description)
2. Component breakdown
3. Technology choices with justification
4. Data flow diagrams
5. API contracts
6. Deployment topology
6. Security boundaries
7. Observability strategy
8. Cost estimation
9. Migration/rollout plan
10. Risk assessment"#,
            req.functional.join(", "),
            req.non_functional.join(", "),
            req.expected_users,
            req.expected_rps,
            req.data_volume_tb,
            req.data_throughput_gbps,
            req.team_size,
        )
    }

    fn build_prompt_with_constraints(&self, req: &Requirements, constraints: &Constraints) -> String {
        let base = self.build_prompt(req);
        format!("{}\n\nConstraints:\n- Budget: ${}/month\n- Compliance: {}\n- Cloud provider: {}\n- Existing systems: {}\n- Timeline: {} weeks",
            base,
            constraints.budget_usd_per_month,
            constraints.compliance.join(", "),
            constraints.cloud_provider,
            constraints.existing_systems.join(", "),
            constraints.timeline_weeks,
        )
    }
}

struct Requirements {
    functional: Vec<String>,
    non_functional: Vec<String>,
    expected_users: u64,
    expected_rps: u64,
    data_volume_tb: f64,
    data_throughput_gbps: f64,
    team_size: usize,
}

struct Constraints {
    budget_usd_per_month: u64,
    compliance: Vec<String>,
    cloud_provider: String,
    existing_systems: Vec<String>,
    timeline_weeks: u32,
}

struct ArchitectureDesign {
    requirements: Requirements,
    optimized_prompt: String,
    tokens_saved: usize,
}
```

---

## Multi-language Project

Using TokenOpt across Rust, TypeScript, and Python.

### Project Structure

```
my-project/
├── Cargo.toml              # Rust workspace
├── package.json            # TypeScript workspace
├── pyproject.toml          # Python workspace
├── tokenopt.toml           # Shared TokenOpt config
├── rust/
│   ├── src/
│   │   └── main.rs
├── typescript/
│   ├── src/
│   │   └── index.ts
├── python/
│   ├── src/
│   │   └── main.py
└── shared/
    └── prompts/
        ├── code_review.txt
        ├── doc_gen.txt
        └── test_gen.txt
```

### Shared Configuration

```toml
# tokenopt.toml
[counter]
default_model = "gpt-4o"
enable_cache = true
max_cache_size = 50000

[optimizer]
context_config = { 
    strategy = "Balanced", 
    max_tokens = 8000,
    reserve_tokens = 2000,
}
enable_caching = true
enable_summarization = true
enable_prompt_optimization = true
enable_model_routing = true
default_model = "gpt-4o"
max_optimization_time_ms = 100

[router]
default_strategy = "Balanced"

[prompt]
default_aggressiveness = "Balanced"
```

### Rust Usage (rust/src/main.rs)

```rust
use tokenopt_core::prelude::*;
use std::fs;

fn main() -> Result<()> {
    let tokenopt = TokenOpt::new()?;
    
    // Load shared prompt
    let prompt = fs::read_to_string("../shared/prompts/code_review.txt")?;
    
    // Optimize for Rust
    let optimized = tokenopt.prompt_optimizer().optimize_string(&prompt, "gpt-4o")?;
    
    println!("Optimized prompt: {}", optimized.prompt);
    println!("Tokens saved: {}", optimized.tokens_saved);
    
    Ok(())
}
```

### TypeScript Usage (typescript/src/index.ts)

```typescript
import { TokenOpt, PromptTask } from '@tokenopt/core';
import * as fs from 'fs';

const tokenopt = new TokenOpt();

// Load shared prompt
const prompt = fs.readFileSync('../shared/prompts/doc_gen.txt', 'utf-8');

// Optimize for documentation
const optimized = tokenopt.promptOptimizer.optimizeString(prompt, 'gpt-4o');

console.log(`Optimized: ${optimized.prompt}`);
console.log(`Saved: ${optimized.tokensSaved} tokens`);

// Use task-specific optimization
const docOptimized = tokenopt.promptOptimizer.optimizeForTask(
    PromptTask.Documentation,
    'Generate OpenAPI spec for user service',
    'gpt-4o'
);
```

### Python Usage (python/src/main.py)

```python
from tokenopt import TokenOpt, PromptTask
import os

tokenopt = TokenOpt()

# Load shared prompt
with open("../shared/prompts/test_gen.txt") as f:
    prompt = f.read()

# Optimize for test generation
optimized = tokenopt.prompt_optimizer.optimize_string(prompt, "gpt-4o")

print(f"Optimized: {optimized.prompt}")
print(f"Saved: {optimized.tokens_saved} tokens")

# Use task-specific
test_optimized = tokenopt.prompt_optimizer.optimize_for_task(
    PromptTask.TestGeneration,
    "Generate property-based tests for user service",
    "gpt-4o"
)
```

### Shared Prompt Example (shared/prompts/code_review.txt)

```
You are an expert code reviewer. Review the following {{language}} code for {{focus_areas}}.

Code:
```{{language}}
{{code}}
```

Provide a structured review with:
1. **Critical Issues** - Security vulnerabilities, bugs, data loss risks
2. **Performance** - Algorithmic complexity, memory usage, I/O efficiency
3. **Maintainability** - Code clarity, naming, modularity, documentation
4. **Testing** - Coverage gaps, missing edge cases, flaky tests
5. **Architecture** - Design patterns, separation of concerns, SOLID principles
6. **Language-specific** - Idiomatic patterns, standard library usage

Format each finding as:
- **Severity**: Critical/High/Medium/Low
- **Location**: File:line or function name
- **Issue**: Clear description
- **Recommendation**: Specific fix with code example
- **Effort**: Low/Medium/High
```

---

## Cost Optimization Pipeline

Production-ready cost optimization pipeline.

```rust
use tokenopt_core::prelude::*;
use std::collections::HashMap;

struct CostOptimizationPipeline {
    tokenopt: TokenOpt,
    budgets: HashMap<String, Budget>,
    metrics: PipelineMetrics,
}

#[derive(Debug, Clone)]
struct Budget {
    project: String,
    max_monthly_usd: f64,
    current_spend_usd: f64,
    alert_threshold: f64, // 0.8 = 80%
}

#[derive(Debug, Default)]
struct PipelineMetrics {
    total_requests: u64,
    total_tokens_saved: u64,
    total_cost_saved_usd: f64,
    avg_optimization_ms: f64,
    model_usage: HashMap<String, u64>,
}

impl CostOptimizationPipeline {
    fn new() -> Result<Self> {
        Ok(Self {
            tokenopt: TokenOpt::new()?,
            budgets: HashMap::new(),
            metrics: PipelineMetrics::default(),
        })
    }

    fn set_budget(&mut self, project: String, max_monthly_usd: f64) {
        self.budgets.insert(project, Budget {
            project: project.clone(),
            max_monthly_usd,
            current_spend_usd: 0.0,
            alert_threshold: 0.8,
        });
    }

    fn process_request(
        &mut self,
        project: &str,
        messages: &[Message],
        preferred_model: &str,
    ) -> Result<OptimizedRequest> {
        // Check budget
        if let Some(budget) = self.budgets.get(project) {
            if budget.current_spend_usd >= budget.max_monthly_usd * budget.alert_threshold {
                // Force cheaper model
                return self.optimize_with_budget(project, messages, preferred_model, true);
            }
        }

        // Normal optimization
        self.optimize_with_budget(project, messages, preferred_model, false)
    }

    fn optimize_with_budget(
        &mut self,
        project: &str,
        messages: &[Message],
        preferred_model: &str,
        force_cheap: bool,
    ) -> Result<OptimizedRequest> {
        let mut optimizer = self.tokenopt.optimizer().clone();
        
        if force_cheap {
            // Force cost-optimized routing
            let mut router = self.tokenopt.router().clone();
            router = router.with_strategy(RoutingStrategy::CostOptimized);
            // Would need to integrate with optimizer...
        }

        let result = optimizer.optimize_conversation(messages, None, Some(preferred_model))?;
        
        // Update metrics
        self.metrics.total_requests += 1;
        self.metrics.total_tokens_saved += result.tokens_saved as u64;
        self.metrics.total_cost_saved_usd += result.estimated_cost_savings_usd.unwrap_or(0.0);
        self.metrics.avg_optimization_ms = 
            (self.metrics.avg_optimization_ms * (self.metrics.total_requests - 1) as f64 
             + result.metadata.optimization_time_ms as f64) / self.metrics.total_requests as f64;

        if let Some(routing) = &result.routing_decision {
            *self.metrics.model_usage.entry(routing.selected_model.clone()).or_insert(0) += 1;
        }

        // Update budget
        if let Some(budget) = self.budgets.get_mut(project) {
            if let Some(routing) = &result.routing_decision {
                if let Some(model) = self.tokenopt.registry().get(&routing.selected_model) {
                    if let Some(pricing) = &model.pricing {
                        let cost = (result.optimized_token_count.prompt as f64 * pricing.input_per_million / 1_000_000.0) +
                                   (result.optimized_token_count.completion as f64 * pricing.output_per_million / 1_000_000.0);
                        budget.current_spend_usd += cost;
                    }
                }
            }
        }

        Ok(OptimizedRequest {
            original_messages: messages.to_vec(),
            optimized_messages: result.selected_context.map(|c| c.messages).unwrap_or_else(|| messages.to_vec()),
            routing_decision: result.routing_decision,
            cost_estimate: result.estimated_cost_savings_usd,
            optimization_metadata: result.metadata,
        })
    }

    fn get_report(&self) -> PipelineReport {
        PipelineReport {
            total_requests: self.metrics.total_requests,
            total_tokens_saved: self.metrics.total_tokens_saved,
            total_cost_saved_usd: self.metrics.total_cost_saved_usd,
            avg_optimization_ms: self.metrics.avg_optimization_ms,
            model_usage: self.metrics.model_usage.clone(),
            budgets: self.budgets.clone(),
        }
    }
}

struct OptimizedRequest {
    original_messages: Vec<Message>,
    optimized_messages: Vec<Message>,
    routing_decision: Option<RoutingDecision>,
    cost_estimate: Option<f64>,
    optimization_metadata: OptimizationMetadata,
}

struct PipelineReport {
    total_requests: u64,
    total_tokens_saved: u64,
    total_cost_saved_usd: f64,
    avg_optimization_ms: f64,
    model_usage: HashMap<String, u64>,
    budgets: HashMap<String, Budget>,
}

// Usage
fn main() -> Result<()> {
    let mut pipeline = CostOptimizationPipeline::new()?;
    
    // Set budgets
    pipeline.set_budget("project-alpha".to_string(), 1000.0); // $1000/month
    pipeline.set_budget("project-beta".to_string(), 500.0);   // $500/month
    
    // Process requests
    let messages = vec![
        Message { role: Role::User, content: "Analyze this codebase...".to_string(), ..Default::default() },
    ];
    
    let result = pipeline.process_request("project-alpha", &messages, "gpt-4o")?;
    
    println!("Model used: {:?}", result.routing_decision.as_ref().map(|r| &r.selected_model));
    println!("Cost estimate: ${:.4}", result.cost_estimate.unwrap_or(0.0));
    
    // Generate report
    let report = pipeline.get_report();
    println!("Total saved: ${:.2}", report.total_cost_saved_usd);
    
    Ok(())
}
```

---

## Streaming Context Management

Real-time context management for streaming applications.

```rust
use tokenopt_core::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

struct StreamingContextManager {
    tokenopt: Arc<TokenOpt>,
    rolling_summary: Arc<RwLock<RollingSummary>>,
    config: StreamingConfig,
    message_buffer: Arc<RwLock<Vec<Message>>>,
}

#[derive(Debug, Clone)]
struct StreamingConfig {
    window_size: usize,
    max_buffer: usize,
    summarize_interval: usize,
    max_summary_tokens: usize,
    model: String,
}

impl StreamingContextManager {
    fn new(tokenopt: TokenOpt, config: StreamingConfig) -> Self {
        let mut messages = Vec::new();
        let rolling = tokenopt.summarizer().create_rolling_summary(&messages, config.window_size).unwrap();
        
        Self {
            tokenopt: Arc::new(tokenopt),
            rolling_summary: Arc::new(RwLock::new(rolling)),
            config,
            message_buffer: Arc::new(RwLock::new(Vec::new())),
        }
    }

    async fn add_message(&self, message: Message) -> Result<ContextSnapshot> {
        let mut buffer = self.message_buffer.write().await;
        buffer.push(message.clone());
        
        // Summarize if buffer exceeds threshold
        if buffer.len() >= self.config.summarize_interval {
            let to_summarize: Vec<Message> = buffer.drain(..self.config.summarize_interval).collect();
            self.update_rolling_summary(to_summarize).await?;
        }
        
        // Trim buffer if too large
        if buffer.len() > self.config.max_buffer {
            let excess = buffer.len() - self.config.max_buffer;
            buffer.drain(..excess);
        }
        
        self.get_context().await
    }

    async fn update_rolling_summary(&self, messages: Vec<Message>) -> Result<()> {
        let mut rolling = self.rolling_summary.write().await;
        self.tokenopt.summarizer().update_rolling_summary(&mut rolling, &messages)?;
        Ok(())
    }

    async fn get_context(&self) -> Result<ContextSnapshot> {
        let rolling = self.rolling_summary.read().await;
        let buffer = self.message_buffer.read().await;
        
        let context = format!("{}\n\n---\n\n{}", 
            rolling.summary,
            buffer.iter()
                .map(|m| format!("{}: {}", m.role.as_str(), m.content))
                .collect::<Vec<_>>()
                .join("\n\n")
        );
        
        let token_count = self.tokenopt.counter().count_tokens(&context, Some(&self.config.model))?;
        
        Ok(ContextSnapshot {
            context,
            token_count,
            rolling_summary_tokens: rolling.token_count,
            buffer_messages: buffer.len(),
            total_messages: rolling.message_count + buffer.len(),
        })
    }

    async fn force_summarize(&self) -> Result<()> {
        let buffer = self.message_buffer.write().await;
        let messages: Vec<Message> = buffer.drain(..).collect();
        drop(buffer);
        self.update_rolling_summary(messages).await
    }
}

struct ContextSnapshot {
    context: String,
    token_count: usize,
    rolling_summary_tokens: usize,
    buffer_messages: usize,
    total_messages: usize,
}

// Usage
async fn streaming_example() -> Result<()> {
    let tokenopt = TokenOpt::new()?;
    let config = StreamingConfig {
        window_size: 50,
        max_buffer: 100,
        summarize_interval: 20,
        max_summary_tokens: 1000,
        model: "gpt-4o".to_string(),
    };
    
    let manager = StreamingContextManager::new(tokenopt, config);
    
    // Simulate streaming messages
    for i in 0..100 {
        let message = Message {
            role: if i % 2 == 0 { Role::User } else { Role::Assistant },
            content: format!("Message {} in streaming conversation", i),
            ..Default::default()
        };
        
        let context = manager.add_message(message).await?;
        
        if i % 10 == 0 {
            println!("Message {}: {} tokens, {} buffered", 
                i, context.token_count, context.buffer_messages);
        }
    }
    
    Ok(())
}
```

---

## Running Examples

```bash
# Run specific example
cargo run --example code_review
cargo run --example doc_generator
cargo run --example test_generator
cargo run --example refactoring
cargo run --example debugging
cargo run --example architecture
cargo run --example cost_pipeline
cargo run --example streaming

# Run all examples
cargo run --examples
```