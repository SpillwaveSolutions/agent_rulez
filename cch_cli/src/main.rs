use anyhow::Result;
use clap::{Parser, Subcommand};
use std::io::{self, Read};
use tracing::{error, info};

mod cli;
mod config;
mod hooks;
mod logging;
mod models;

#[derive(Parser)]
#[command(name = "cch")]
#[command(about = "Claude Code Hooks - High-performance policy engine")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    /// Enable debug logging with full event and rule details
    #[arg(long, global = true)]
    debug_logs: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate configuration file
    Validate {
        /// Path to configuration file
        #[arg(short, long)]
        config: Option<String>,
    },
    /// Query and display logs
    Logs {
        /// Number of recent log entries to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
        /// Show logs since timestamp
        #[arg(long)]
        since: Option<String>,
    },
    /// Explain why rules fired for a given event
    Explain {
        /// Event ID to explain
        event_id: String,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Initialize the global logger for audit trails
    if let Err(e) = logging::init_global_logger() {
        tracing::warn!("Failed to initialize logger: {}", e);
    }

    let cli = Cli::parse();

    // Load config to get settings for DebugConfig
    let config = config::Config::load(None)?;

    match cli.command {
        Some(Commands::Validate { config }) => {
            cli::validate::run(config).await?;
        }
        Some(Commands::Logs { limit, since }) => {
            cli::logs::run(limit, since).await?;
        }
        Some(Commands::Explain { event_id }) => {
            cli::explain::run(event_id).await?;
        }
        None => {
            // No subcommand provided, read from stdin for hook processing
            process_hook_event(&cli, &config).await?;
        }
    }

    Ok(())
}

async fn process_hook_event(cli: &Cli, config: &config::Config) -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    if buffer.trim().is_empty() {
        error!("No input received on stdin");
        std::process::exit(1);
    }

    let event: models::Event = serde_json::from_str(&buffer).map_err(|e| {
        error!("Failed to parse hook event: {}", e);
        e
    })?;

    info!(
        "Processing event: {} ({})",
        event.event_type, event.session_id
    );

    let debug_config = models::DebugConfig::new(cli.debug_logs, config.settings.debug_logs);
    let response = hooks::process_event(event, &debug_config).await?;

    let json = serde_json::to_string(&response)?;
    println!("{}", json);

    Ok(())
}
