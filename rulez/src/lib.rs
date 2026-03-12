//! RuleZ - High-performance AI policy engine for development workflows
//!
//! This crate provides a policy engine that intercepts AI coding assistant tool
//! invocations (via CLI hooks) and applies user-defined YAML rules. It does NOT
//! have built-in blocking or injection features -- all behavior is defined by
//! user YAML configuration in `hooks.yaml`.
//!
//! # Architecture
//!
//! RuleZ operates as a stdin/stdout hook binary. When an AI coding assistant
//! (Claude Code, Gemini CLI, GitHub Copilot, OpenCode, or Codex) invokes a tool,
//! it sends a JSON event on stdin. RuleZ evaluates user-defined rules against
//! that event and writes a JSON response to stdout indicating whether the
//! operation should proceed, be blocked, or have context injected.
//!
//! ```text
//! ┌──────────────┐    JSON event     ┌────────┐    JSON response    ┌──────────────┐
//! │  AI Assistant │ ──── stdin ─────▶ │  RuleZ │ ──── stdout ─────▶ │  AI Assistant │
//! │ (Claude Code) │                   │ Engine │                    │ (continues)   │
//! └──────────────┘                    └────────┘                    └──────────────┘
//!                                         │
//!                                    reads hooks.yaml
//!                                    writes audit log
//! ```
//!
//! # Modules
//!
//! - [`config`] -- Configuration loading and caching. Reads `hooks.yaml` from
//!   project (`.claude/hooks.yaml`) or global (`~/.claude/hooks.yaml`) paths.
//!   Implements mtime-based caching so the file is only re-parsed when modified.
//!
//! - [`hooks`] -- The rule evaluation engine. Matches incoming events against
//!   configured rules (tools, extensions, directories, command patterns, prompt
//!   patterns, field validation) and executes actions (block, inject, run
//!   validator scripts). Supports parallel evaluation when rule count exceeds a
//!   threshold. Includes an LRU-cached regex compiler.
//!
//! - [`models`] -- All type definitions: `Event`, `EventType`, `Rule`,
//!   `Matchers`, `Actions`, `Response`, governance types (`PolicyMode`,
//!   `Decision`, `Confidence`, `TrustLevel`), logging types (`LogEntry`,
//!   `Outcome`), and prompt matching types (`PromptMatch`, `MatchMode`).
//!
//! - [`config`] -- The `Config` and `Settings` structs for `hooks.yaml` parsing,
//!   including validation of `enabled_when` expressions at load time.
//!
//! - [`logging`] -- Structured audit trail. Writes NDJSON log entries to
//!   `~/.claude/logs/rulez.log` and supports external backends (OTLP, Datadog,
//!   Splunk).
//!
//! - [`adapters`] -- Platform adapters that translate platform-specific event
//!   formats (Gemini CLI, Copilot, OpenCode) into the canonical RuleZ event
//!   model.
//!
//! - [`cli`] -- CLI subcommand implementations (`init`, `install`, `uninstall`,
//!   `debug`, `validate`, `logs`, `explain`, `repl`, `upgrade`).
//!
//! - [`opencode`] -- OpenCode plugin integration types and helpers.
//!
//! # Example usage
//!
//! ```no_run
//! use rulez::models::{Event, DebugConfig};
//! use rulez::hooks::process_event;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Parse the JSON event from stdin
//! let input = r#"{"hook_event_name":"PreToolUse","session_id":"abc","tool_name":"Bash","tool_input":{"command":"ls"}}"#;
//! let event: Event = serde_json::from_str(input)?;
//!
//! // Evaluate rules and get the response
//! let debug_config = DebugConfig::default();
//! let response = process_event(event, &debug_config).await?;
//!
//! // Serialize response to stdout
//! println!("{}", serde_json::to_string(&response)?);
//! # Ok(())
//! # }
//! ```
//!
//! # Exit codes
//!
//! | Code | Meaning              |
//! |------|----------------------|
//! | 0    | Success (allow)      |
//! | 1    | Configuration error  |
//! | 2    | Validation error (block) |
//! | 3    | Runtime error        |

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::unused_async)]
#![allow(clippy::doc_link_with_quotes)]
#![allow(clippy::unused_self)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::new_without_default)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::match_bool)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::regex_creation_in_loops)]
#![allow(clippy::unnecessary_map_or)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::if_not_else)]
#![allow(clippy::redundant_closure_for_method_calls)]

/// Platform adapters for Gemini CLI, Copilot, and OpenCode event translation.
pub mod adapters;
/// CLI subcommand implementations (init, install, debug, validate, logs, etc.).
pub mod cli;
/// Configuration loading, parsing, and mtime-based caching for hooks.yaml.
pub mod config;
/// Rule evaluation engine: matching, actions, regex caching, and parallel eval.
pub mod hooks;
/// Structured audit logging with NDJSON output and external backend support.
pub mod logging;
/// Type definitions for events, rules, matchers, actions, responses, and governance.
pub mod models;
/// OpenCode plugin integration types.
pub mod opencode;
