//! Multi-runtime skill portability layer.
//!
//! Converts canonical Claude Code skills and commands into runtime-specific
//! installations for OpenCode, Gemini CLI, Codex, and generic skill runtimes.
//!
//! The pipeline:
//! 1. **Discovery** — scan `.claude/skills/` and `.claude/commands/` for sources
//! 2. **Transform** — apply per-runtime content transformations
//! 3. **Write** — output to target runtime directory
//! 4. **Config gen** — update runtime config files (GEMINI.md, AGENTS.md, etc.)

pub mod config_gen;
pub mod discovery;
pub mod profiles;
pub mod transform;
pub mod transforms;
pub mod writer;
