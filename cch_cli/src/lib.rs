//! Claude Code Hooks (CCH) - High-performance policy engine for development workflows
//!
//! This crate provides a policy engine that executes user-configured YAML rules
//! to control Claude Code behavior. It does NOT have built-in blocking or injection
//! features - all behavior is defined by user YAML configuration.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

pub mod cli;
pub mod config;
pub mod hooks;
pub mod logging;
pub mod models;