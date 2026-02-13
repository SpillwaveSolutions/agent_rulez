//! Claude Code Hooks (CCH) - High-performance policy engine for development workflows
//!
//! This crate provides a policy engine that executes user-configured YAML rules
//! to control Claude Code behavior. It does NOT have built-in blocking or injection
//! features - all behavior is defined by user YAML configuration.

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

pub mod adapters;
pub mod cli;
pub mod config;
pub mod hooks;
pub mod logging;
pub mod models;
pub mod opencode;
