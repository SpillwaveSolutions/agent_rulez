use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};

use crate::config::Config;
use crate::models::Rule;

/// Diagnostic severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Severity {
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "ERROR"),
            Severity::Warning => write!(f, "WARN"),
            Severity::Info => write!(f, "INFO"),
        }
    }
}

/// A single lint diagnostic
#[derive(Debug)]
struct Diagnostic {
    severity: Severity,
    code: String,
    message: String,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Align: [ERROR] code, [WARN]  code, [INFO]  code — code starts at column 9
        let label = format!("[{}]", self.severity);
        write!(f, "{:<8}{}: {}", label, self.code, self.message)
    }
}

/// Run the lint command
pub async fn run(config_path: Option<String>, verbose: bool) -> Result<()> {
    let config_path = config_path.unwrap_or_else(|| ".claude/hooks.yaml".to_string());

    println!("rulez lint — Rule Quality Analysis");
    println!("==================================");
    println!();

    let config =
        Config::from_file(&config_path).context("Failed to load configuration for linting")?;

    println!("Loaded {} rules from {}", config.rules.len(), config_path);
    println!();

    let mut diagnostics = Vec::new();

    check_duplicate_names(&config.rules, &mut diagnostics);
    check_empty_matchers(&config.rules, &mut diagnostics);
    check_conflicting_actions(&config.rules, &mut diagnostics);
    check_overlapping_rules(&config.rules, &mut diagnostics);
    check_dead_rules(&config.rules, &mut diagnostics);
    check_missing_descriptions(&config.rules, &mut diagnostics);
    check_invalid_regex(&config.rules, &mut diagnostics);
    check_glob_consolidation(&config.rules, &mut diagnostics, verbose);
    check_missing_priority(&config.rules, &mut diagnostics);

    // Print diagnostics
    for diag in &diagnostics {
        println!("{}", diag);
    }

    // Summary
    let errors = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .count();
    let warnings = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .count();
    let infos = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Info)
        .count();

    if diagnostics.is_empty() {
        println!("No issues found. Configuration looks good!");
    } else {
        println!();
        println!(
            "Summary: {} error{}, {} warning{}, {} info",
            errors,
            if errors == 1 { "" } else { "s" },
            warnings,
            if warnings == 1 { "" } else { "s" },
            infos
        );
    }

    if errors > 0 {
        std::process::exit(1);
    }

    Ok(())
}

/// Check for duplicate rule names
fn check_duplicate_names(rules: &[Rule], diagnostics: &mut Vec<Diagnostic>) {
    let mut seen: HashMap<&str, usize> = HashMap::new();
    for (i, rule) in rules.iter().enumerate() {
        if let Some(&prev_idx) = seen.get(rule.name.as_str()) {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: "duplicate-rule-name".to_string(),
                message: format!(
                    "Rules at positions {} and {} both have the name '{}'",
                    prev_idx + 1,
                    i + 1,
                    rule.name
                ),
            });
        } else {
            seen.insert(&rule.name, i);
        }
    }
}

/// Check for rules with no matchers (empty matchers match everything)
fn check_empty_matchers(rules: &[Rule], diagnostics: &mut Vec<Diagnostic>) {
    for rule in rules {
        let m = &rule.matchers;
        let has_matchers = m.tools.is_some()
            || m.extensions.is_some()
            || m.directories.is_some()
            || m.operations.is_some()
            || m.command_match.is_some()
            || m.prompt_match.is_some()
            || m.require_fields.is_some()
            || m.field_types.is_some();

        if !has_matchers {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: "no-matchers".to_string(),
                message: format!(
                    "Rule '{}' has no matchers — it will match all events",
                    rule.name
                ),
            });
        }
    }
}

/// Check for conflicting actions (block + inject)
fn check_conflicting_actions(rules: &[Rule], diagnostics: &mut Vec<Diagnostic>) {
    for rule in rules {
        let blocks = rule.actions.block == Some(true);
        let injects = rule.actions.inject.is_some()
            || rule.actions.inject_inline.is_some()
            || rule.actions.inject_command.is_some();

        if blocks && injects {
            diagnostics.push(Diagnostic {
                severity: Severity::Error,
                code: "conflicting-actions".to_string(),
                message: format!(
                    "Rule '{}' has both block and inject actions — blocked operations cannot inject context",
                    rule.name
                ),
            });
        }
    }
}

/// Check for overlapping rules (same operations + tools)
fn check_overlapping_rules(rules: &[Rule], diagnostics: &mut Vec<Diagnostic>) {
    for i in 0..rules.len() {
        for j in (i + 1)..rules.len() {
            let a = &rules[i];
            let b = &rules[j];

            // Both must be enabled
            if !a.is_enabled() || !b.is_enabled() {
                continue;
            }

            let ops_overlap = match (&a.matchers.operations, &b.matchers.operations) {
                (Some(a_ops), Some(b_ops)) => {
                    let a_set: HashSet<_> = a_ops.iter().collect();
                    let b_set: HashSet<_> = b_ops.iter().collect();
                    !a_set.is_disjoint(&b_set)
                }
                (None, _) | (_, None) => {
                    // None means "match all" so it overlaps with everything
                    true
                }
            };

            let tools_overlap = match (&a.matchers.tools, &b.matchers.tools) {
                (Some(a_tools), Some(b_tools)) => {
                    let a_set: HashSet<_> = a_tools.iter().collect();
                    let b_set: HashSet<_> = b_tools.iter().collect();
                    !a_set.is_disjoint(&b_set)
                }
                (None, _) | (_, None) => true,
            };

            let cmd_overlap = match (&a.matchers.command_match, &b.matchers.command_match) {
                (Some(a_cmd), Some(b_cmd)) => a_cmd == b_cmd,
                _ => false,
            };

            // Only flag if both operations and tools overlap AND they have similar command patterns
            if ops_overlap && tools_overlap && cmd_overlap {
                diagnostics.push(Diagnostic {
                    severity: Severity::Warning,
                    code: "overlapping-rules".to_string(),
                    message: format!(
                        "Rules '{}' and '{}' have overlapping matchers",
                        a.name, b.name
                    ),
                });
            }
        }
    }
}

/// Check for dead rules (metadata.enabled: false)
fn check_dead_rules(rules: &[Rule], diagnostics: &mut Vec<Diagnostic>) {
    for rule in rules {
        if !rule.is_enabled() {
            diagnostics.push(Diagnostic {
                severity: Severity::Warning,
                code: "dead-rule".to_string(),
                message: format!(
                    "Rule '{}' is disabled (metadata.enabled: false) — consider removing it",
                    rule.name
                ),
            });
        }
    }
}

/// Check for rules missing descriptions
fn check_missing_descriptions(rules: &[Rule], diagnostics: &mut Vec<Diagnostic>) {
    for rule in rules {
        if rule.description.is_none() || rule.description.as_deref() == Some("") {
            diagnostics.push(Diagnostic {
                severity: Severity::Warning,
                code: "no-description".to_string(),
                message: format!("Rule '{}' has no description", rule.name),
            });
        }
    }
}

/// Check for invalid regex patterns in command_match
fn check_invalid_regex(rules: &[Rule], diagnostics: &mut Vec<Diagnostic>) {
    for rule in rules {
        if let Some(ref pattern) = rule.matchers.command_match {
            if regex::Regex::new(pattern).is_err() {
                diagnostics.push(Diagnostic {
                    severity: Severity::Warning,
                    code: "invalid-regex".to_string(),
                    message: format!(
                        "Rule '{}' has invalid command_match regex: '{}'",
                        rule.name, pattern
                    ),
                });
            }
        }
    }
}

/// Suggest glob consolidation when multiple rules have same action but different extensions
fn check_glob_consolidation(rules: &[Rule], diagnostics: &mut Vec<Diagnostic>, verbose: bool) {
    // Group rules by their action signature (block vs inject vs run)
    let mut action_groups: HashMap<String, Vec<&Rule>> = HashMap::new();

    for rule in rules {
        if !rule.is_enabled() {
            continue;
        }
        if rule.matchers.extensions.is_none() {
            continue;
        }

        // Create a simple action key
        let action_key = if rule.actions.block == Some(true) {
            "block".to_string()
        } else if let Some(ref inject) = rule.actions.inject {
            format!("inject:{}", inject)
        } else if let Some(ref inline) = rule.actions.inject_inline {
            format!("inject_inline:{}", &inline[..inline.len().min(50)])
        } else {
            continue;
        };

        action_groups.entry(action_key).or_default().push(rule);
    }

    for group in action_groups.values() {
        if group.len() >= 2 {
            let names: Vec<&str> = group.iter().map(|r| r.name.as_str()).collect();
            if verbose {
                diagnostics.push(Diagnostic {
                    severity: Severity::Info,
                    code: "glob-consolidation".to_string(),
                    message: format!(
                        "Rules {} have the same action with different extensions — consider merging",
                        names.join(", ")
                    ),
                });
            }
        }
    }
}

/// Check for rules without explicit priority
fn check_missing_priority(rules: &[Rule], diagnostics: &mut Vec<Diagnostic>) {
    for rule in rules {
        let has_priority =
            rule.priority.is_some() || rule.metadata.as_ref().is_some_and(|m| m.priority != 0);

        if !has_priority {
            diagnostics.push(Diagnostic {
                severity: Severity::Info,
                code: "missing-priority".to_string(),
                message: format!(
                    "Rule '{}' has no explicit priority (using default 0)",
                    rule.name
                ),
            });
        }
    }
}
