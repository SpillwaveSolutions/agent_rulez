use anyhow::Result;
use clap::{Parser, Subcommand};
use std::io::{self, Read};
use tracing::{error, info};

mod adapters;
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
    /// Initialize CCH configuration in current project
    Init {
        /// Overwrite existing configuration
        #[arg(short, long)]
        force: bool,
        /// Create example context and validator files
        #[arg(long)]
        with_examples: bool,
    },
    /// Install CCH hook into Claude Code settings
    Install {
        /// Install globally instead of project-local
        #[arg(short, long)]
        global: bool,
        /// Path to CCH binary (auto-detected if not specified)
        #[arg(short, long)]
        binary: Option<String>,
    },
    /// Uninstall CCH hook from Claude Code settings
    Uninstall {
        /// Uninstall from global settings instead of project-local
        #[arg(short, long)]
        global: bool,
    },
    /// Simulate an event to test rules
    Debug {
        /// Event type: PreToolUse, PostToolUse, SessionStart, PermissionRequest
        event_type: String,
        /// Tool name (e.g., Bash, Write, Read)
        #[arg(short, long)]
        tool: Option<String>,
        /// Command or pattern to test (for Bash/Glob/Grep)
        #[arg(short, long)]
        command: Option<String>,
        /// File path (for Write/Edit/Read)
        #[arg(short, long)]
        path: Option<String>,
        /// Show verbose rule evaluation
        #[arg(short, long)]
        verbose: bool,
    },
    /// Start interactive debug mode
    Repl,
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
        /// Show logs since timestamp (RFC3339 format)
        #[arg(long)]
        since: Option<String>,
        /// Filter by policy mode (enforce, warn, audit)
        #[arg(long)]
        mode: Option<String>,
        /// Filter by decision (allowed, blocked, warned, audited)
        #[arg(long)]
        decision: Option<String>,
    },
    /// Explain rules or events (use 'cch explain --help' for subcommands)
    Explain {
        #[command(subcommand)]
        subcommand: Option<ExplainSubcommand>,
        /// Event/session ID to explain (legacy usage)
        event_id: Option<String>,
    },
    /// Gemini CLI utilities
    Gemini {
        #[command(subcommand)]
        subcommand: GeminiSubcommand,
    },
}

/// Subcommands for the explain command
#[derive(Subcommand)]
enum ExplainSubcommand {
    /// Explain a specific rule's configuration and governance
    Rule {
        /// Name of the rule to explain
        name: String,
        /// Output as JSON for machine parsing
        #[arg(long)]
        json: bool,
        /// Skip activity statistics (faster)
        #[arg(long)]
        no_stats: bool,
    },
    /// List all configured rules
    Rules,
    /// Explain an event by session ID
    Event {
        /// Session/event ID
        event_id: String,
    },
}

/// Subcommands for Gemini CLI utilities
#[derive(Subcommand)]
enum GeminiSubcommand {
    /// Diagnose Gemini hook installation and configuration
    Doctor {
        /// Output machine-readable JSON
        #[arg(long)]
        json: bool,
    },
    /// Run Gemini hook runner (stdin -> Gemini JSON response)
    Hook,
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
        Some(Commands::Init {
            force,
            with_examples,
        }) => {
            cli::init::run(force, with_examples).await?;
        }
        Some(Commands::Install { global, binary }) => {
            let scope = if global {
                cli::install::Scope::Global
            } else {
                cli::install::Scope::Project
            };
            cli::install::run(scope, binary).await?;
        }
        Some(Commands::Uninstall { global }) => {
            let scope = if global {
                cli::install::Scope::Global
            } else {
                cli::install::Scope::Project
            };
            cli::install::uninstall(scope).await?;
        }
        Some(Commands::Debug {
            event_type,
            tool,
            command,
            path,
            verbose,
        }) => {
            cli::debug::run(event_type, tool, command, path, verbose).await?;
        }
        Some(Commands::Repl) => {
            cli::debug::interactive().await?;
        }
        Some(Commands::Validate { config }) => {
            cli::validate::run(config).await?;
        }
        Some(Commands::Logs {
            limit,
            since,
            mode,
            decision,
        }) => {
            cli::logs::run(limit, since, mode, decision).await?;
        }
        Some(Commands::Explain {
            subcommand,
            event_id,
        }) => {
            match subcommand {
                Some(ExplainSubcommand::Rule {
                    name,
                    json,
                    no_stats,
                }) => {
                    cli::explain::explain_rule(name, json, no_stats).await?;
                }
                Some(ExplainSubcommand::Rules) => {
                    cli::explain::list_rules().await?;
                }
                Some(ExplainSubcommand::Event { event_id }) => {
                    cli::explain::run(event_id).await?;
                }
                None => {
                    // Legacy: if event_id provided directly
                    if let Some(id) = event_id {
                        cli::explain::run(id).await?;
                    } else {
                        println!("Usage: cch explain <event_id>");
                        println!("       cch explain rule <rule_name>");
                        println!("       cch explain rules");
                        println!("       cch explain event <event_id>");
                        println!();
                        println!("Use 'cch explain --help' for more information.");
                    }
                }
            }
        }
        Some(Commands::Gemini { subcommand }) => match subcommand {
            GeminiSubcommand::Doctor { json } => {
                cli::gemini_doctor::run(json).await?;
            }
            GeminiSubcommand::Hook => {
                cli::gemini_hook::run(cli.debug_logs).await?;
            }
        },
        None => {
            // No subcommand provided, read from stdin for hook processing
            process_hook_event(&cli, &config).await?;
        }
    }

    Ok(())
}

async fn process_hook_event(cli: &Cli, _config: &config::Config) -> Result<()> {
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
        event.hook_event_name, event.session_id
    );

    // Reload config using the event's cwd so we read the correct project's hooks.yaml
    let project_config =
        config::Config::load(event.cwd.as_ref().map(|p| std::path::Path::new(p.as_str())))?;
    let debug_config = models::DebugConfig::new(cli.debug_logs, project_config.settings.debug_logs);
    let response = hooks::process_event(event, &debug_config).await?;

    if !response.continue_ {
        // Claude Code hooks protocol: exit code 2 BLOCKS the tool call.
        // Only stderr is used as the error message and fed back to Claude.
        // Exit code 0 with "continue":false only stops the conversation,
        // it does NOT prevent the tool from executing.
        let reason = response
            .reason
            .as_deref()
            .unwrap_or("Blocked by CCH policy");
        eprintln!("{}", reason);
        std::process::exit(2);
    }

    // For allowed responses (with or without context injection), output JSON to stdout
    let json = serde_json::to_string(&response)?;
    println!("{}", json);

    Ok(())
}
