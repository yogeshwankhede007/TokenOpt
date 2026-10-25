//! TokenOpt CLI - Command-line interface for token optimization

use std::path::PathBuf;
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use tokenopt_core::prelude::*;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "tokenopt")]
#[command(about = "Universal Token Optimization for LLM Coding Assistants", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true, help = "Enable verbose output")]
    verbose: bool,

    #[arg(short, long, global = true, help = "Output format")]
    format: Option<OutputFormat>,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Json,
    Table,
    Text,
}

#[derive(Subcommand)]
enum Commands {
    /// Count tokens in text or conversation
    Count {
        #[arg(short, long, help = "Text to count tokens for")]
        text: Option<String>,

        #[arg(short, long, help = "File containing text/conversation")]
        file: Option<PathBuf>,

        #[arg(short, long, default_value = "gpt-4", help = "Model to use for counting")]
        model: String,

        #[arg(long, help = "Input is a conversation (JSON array of messages)")]
        conversation: bool,
    },

    /// Optimize a conversation for token usage
    Optimize {
        #[arg(short, long, help = "Conversation file (JSON)")]
        file: PathBuf,

        #[arg(short, long, default_value = "gpt-4", help = "Model to optimize for")]
        model: String,

        #[arg(long, help = "Query for relevance scoring")]
        query: Option<String>,

        #[arg(long, help = "Optimization strategy")]
        strategy: Option<SelectionStrategyCli>,

        #[arg(short, long, help = "Output optimized conversation to file")]
        output: Option<PathBuf>,
    },

    /// Optimize a prompt
    Prompt {
        #[arg(short, long, help = "Prompt text")]
        text: Option<String>,

        #[arg(short, long, help = "File containing prompt")]
        file: Option<PathBuf>,

        #[arg(short, long, default_value = "gpt-4", help = "Model to optimize for")]
        model: String,

        #[arg(long, value_enum, default_value = "balanced", help = "Aggressiveness level")]
        aggressiveness: AggressivenessCli,
    },

    /// Route to best model for a task
    Route {
        #[arg(short, long, default_value = "gpt-4", help = "Requested model")]
        model: String,

        #[arg(short, long, help = "Conversation file")]
        file: Option<PathBuf>,

        #[arg(long, value_enum, default_value = "moderate", help = "Task complexity")]
        complexity: TaskComplexityCli,

        #[arg(long, value_enum, default_value = "balanced", help = "Routing strategy")]
        strategy: RoutingStrategyCli,
    },

    /// Summarize a conversation
    Summarize {
        #[arg(short, long, help = "Conversation file")]
        file: PathBuf,

        #[arg(long, value_enum, default_value = "extractive", help = "Summarization strategy")]
        strategy: SummaryStrategyCli,

        #[arg(long, default_value = "1000", help = "Max summary tokens")]
        max_tokens: usize,
    },

    /// List available models
    Models {
        #[arg(long, help = "Filter by provider")]
        provider: Option<String>,

        #[arg(long, help = "Filter by pricing tier")]
        tier: Option<PricingTierCli>,
    },

    /// Estimate cost for token usage
    Cost {
        #[arg(short, long, default_value = "gpt-4", help = "Model")]
        model: String,

        #[arg(long, default_value = "1000", help = "Prompt tokens")]
        prompt_tokens: usize,

        #[arg(long, default_value = "500", help = "Completion tokens")]
        completion_tokens: usize,
    },

    /// Show cache statistics
    Cache {
        #[arg(long, help = "Clear cache")]
        clear: bool,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum SelectionStrategyCli {
    RecentFirst,
    RelevanceFirst,
    ImportanceFirst,
    Balanced,
}

impl From<SelectionStrategyCli> for tokenopt_core::context::SelectionStrategy {
    fn from(s: SelectionStrategyCli) -> Self {
        match s {
            SelectionStrategyCli::RecentFirst => Self::RecentFirst,
            SelectionStrategyCli::RelevanceFirst => Self::RelevanceFirst,
            SelectionStrategyCli::ImportanceFirst => Self::ImportanceFirst,
            SelectionStrategyCli::Balanced => Self::Balanced,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum AggressivenessCli {
    Conservative,
    Balanced,
    Aggressive,
}

impl From<AggressivenessCli> for tokenopt_core::prompt::AggressivenessLevel {
    fn from(a: AggressivenessCli) -> Self {
        match a {
            AggressivenessCli::Conservative => Self::Conservative,
            AggressivenessCli::Balanced => Self::Balanced,
            AggressivenessCli::Aggressive => Self::Aggressive,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum TaskComplexityCli {
    Simple,
    Moderate,
    Complex,
}

impl From<TaskComplexityCli> for tokenopt_core::router::TaskComplexity {
    fn from(t: TaskComplexityCli) -> Self {
        match t {
            TaskComplexityCli::Simple => Self::Simple,
            TaskComplexityCli::Moderate => Self::Moderate,
            TaskComplexityCli::Complex => Self::Complex,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum RoutingStrategyCli {
    Fixed,
    CostOptimized,
    SpeedOptimized,
    Balanced,
}

impl From<RoutingStrategyCli> for tokenopt_core::router::RoutingStrategy {
    fn from(r: RoutingStrategyCli) -> Self {
        match r {
            RoutingStrategyCli::Fixed => Self::Fixed,
            RoutingStrategyCli::CostOptimized => Self::CostOptimized,
            RoutingStrategyCli::SpeedOptimized => Self::SpeedOptimized,
            RoutingStrategyCli::Balanced => Self::Balanced,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum SummaryStrategyCli {
    Extractive,
    Abstractive,
    Hybrid,
}

impl From<SummaryStrategyCli> for tokenopt_core::summarizer::SummaryStrategy {
    fn from(s: SummaryStrategyCli) -> Self {
        match s {
            SummaryStrategyCli::Extractive => Self::Extractive,
            SummaryStrategyCli::Abstractive => Self::Abstractive,
            SummaryStrategyCli::Hybrid => Self::Hybrid,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum PricingTierCli {
    Free,
    UltraCheap,
    Cheap,
    Standard,
    Premium,
    UltraPremium,
}

impl From<PricingTierCli> for tokenopt_core::models::PricingTier {
    fn from(p: PricingTierCli) -> Self {
        match p {
            PricingTierCli::Free => Self::Free,
            PricingTierCli::UltraCheap => Self::UltraCheap,
            PricingTierCli::Cheap => Self::Cheap,
            PricingTierCli::Standard => Self::Standard,
            PricingTierCli::Premium => Self::Premium,
            PricingTierCli::UltraPremium => Self::UltraPremium,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        tracing_subscriber::fmt::init();
    }

    let format = cli.format.unwrap_or(OutputFormat::Text);
    let tokenopt = TokenOpt::new()?;

    match cli.command {
        Commands::Count { text, file, model, conversation } => {
            let content = read_input(text, file)?;
            let result = if conversation {
                let messages: Vec<tokenopt_core::counter::Message> = serde_json::from_str(&content)?;
                tokenopt.counter().count_conversation(&messages, Some(&model))?
            } else {
                let count = tokenopt.counter().count_tokens(&content, Some(&model))?;
                tokenopt_core::counter::TokenCount {
                    total: count,
                    prompt: count,
                    completion: 0,
                    model: model.clone(),
                    estimated_cost_usd: None,
                }
            };
            output_result(&result, &format)?;
        }

        Commands::Optimize { file, model, query, strategy, output } => {
            let content = std::fs::read_to_string(&file)?;
            let messages: Vec<tokenopt_core::counter::Message> = serde_json::from_str(&content)?;

            let mut config = tokenopt_core::optimizer::OptimizationConfig::default();
            if let Some(s) = strategy {
                config.context_config.strategy = s.into();
            }

            let mut optimizer = tokenopt_core::TokenOptimizer::new(
                tokenopt.counter_arc(),
                tokenopt.registry_arc(),
            );
            optimizer.with_config(config);

            let result = optimizer.optimize_conversation(&messages, query.as_deref(), Some(&model))?;

            if let Some(out_file) = output {
                let optimized_messages = result.selected_context.as_ref().map(|c| c.messages.clone()).unwrap_or(messages);
                std::fs::write(out_file, serde_json::to_string_pretty(&optimized_messages)?)?;
                println!("{}", "Optimized conversation saved".green());
            }

            output_result(&result, &format)?;
        }

        Commands::Prompt { text, file, model, aggressiveness } => {
            let prompt = read_input(text, file)?;
            let result = tokenopt.prompt_optimizer().optimize_string(&prompt, &model)?;
            output_result(&result, &format)?;
        }

        Commands::Route { model, file, complexity, strategy } => {
            let messages = if let Some(file) = file {
                let content = std::fs::read_to_string(file)?;
                serde_json::from_str(&content)?
            } else {
                vec![]
            };

            let mut router = tokenopt.router().clone();
            // Note: In real implementation, we'd need to apply strategy to router
            
            let decision = router.route(&model, complexity.into(), &messages)?;
            output_result(&decision, &format)?;
        }

        Commands::Summarize { file, strategy, max_tokens } => {
            let content = std::fs::read_to_string(&file)?;
            let messages: Vec<tokenopt_core::counter::Message> = serde_json::from_str(&content)?;

            let config = tokenopt_core::summarizer::SummaryConfig {
                strategy: strategy.into(),
                max_summary_tokens: max_tokens,
                ..Default::default()
            };

            let result = tokenopt.summarizer().summarize(&messages, &config)?;
            output_result(&result, &format)?;
        }

        Commands::Models { provider, tier } => {
            let models = if let Some(p) = provider {
                tokenopt.registry().get_by_provider(&p)
            } else {
                tokenopt.registry().all()
            };

            let filtered: Vec<_> = if let Some(t) = tier {
                let tier_val = t.into();
                models.into_iter().filter(|m| m.pricing_tier() == tier_val).map(|m| m.as_ref().clone()).collect()
            } else {
                models.into_iter().map(|m| m.as_ref().clone()).collect()
            };

            output_result(&filtered, &format)?;
        }

        Commands::Cost { model, prompt_tokens, completion_tokens } => {
            if let Some(model_caps) = tokenopt.registry().get(&model) {
                if let Some(pricing) = &model_caps.pricing {
                    let cost = (prompt_tokens as f64 * pricing.input_per_million / 1_000_000.0) +
                               (completion_tokens as f64 * pricing.output_per_million / 1_000_000.0);
                    println!("Estimated cost: ${:.6}", cost);
                } else {
                    println!("No pricing data for model: {}", model);
                }
            } else {
                println!("Model not found: {}", model);
            }
        }

        Commands::Cache { clear } => {
            if clear {
                tokenopt.cache().clear();
                println!("{}", "Cache cleared".green());
            } else {
                let stats = tokenopt.cache().stats();
                println!("Cache Statistics:");
                println!("  Entries: {}", stats.entry_count);
                println!("  Hits: {}", stats.hits);
                println!("  Misses: {}", stats.misses);
                println!("  Hit Rate: {:.2}%", tokenopt.cache().hit_rate() * 100.0);
                println!("  Total Size: {} bytes", stats.total_size_bytes);
            }
        }
    }

    Ok(())
}

fn read_input(text: Option<String>, file: Option<PathBuf>) -> Result<String> {
    match (text, file) {
        (Some(t), None) => Ok(t),
        (None, Some(f)) => Ok(std::fs::read_to_string(f)?),
        (Some(_), Some(_)) => anyhow::bail!("Provide either --text or --file, not both"),
        (None, None) => anyhow::bail!("Provide either --text or --file"),
    }
}

fn output_result<T: serde::Serialize>(result: &T, format: &OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(result)?);
        }
        OutputFormat::Table => {
            // For simple types, just use JSON
            println!("{}", serde_json::to_string_pretty(result)?);
        }
        OutputFormat::Text => {
            println!("{}", serde_json::to_string_pretty(result)?);
        }
    }
    Ok(())
}