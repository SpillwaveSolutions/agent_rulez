use anyhow::Result;
use regex::Regex;

use std::path::Path;
use tokio::process::Command;
use tokio::time::{Duration, timeout};

use crate::config::Config;
use crate::logging::log_entry;
use crate::models::LogMetadata;
use crate::models::{
    DebugConfig, Decision, Event, EventDetails, LogEntry, LogTiming, MatcherResults, Outcome,
    PolicyMode, Response, ResponseSummary, Rule, RuleEvaluation, Timing,
};

/// Process a hook event and return the appropriate response
pub async fn process_event(event: Event, debug_config: &DebugConfig) -> Result<Response> {
    let start_time = std::time::Instant::now();

    // Load configuration
    let config = Config::load(None)?;

    // Evaluate rules (with optional debug tracking)
    let (matched_rules, response, rule_evaluations) =
        evaluate_rules(&event, &config, debug_config).await?;

    let processing_time = start_time.elapsed().as_millis() as u64;

    // Build enhanced logging fields
    let event_details = EventDetails::extract(&event);
    let response_summary = ResponseSummary::from_response(&response);

    // Log the event with enhanced fields
    let entry = LogEntry {
        timestamp: event.timestamp,
        event_type: format!("{:?}", event.event_type),
        session_id: event.session_id.clone(),
        tool_name: event.tool_name.clone(),
        rules_matched: matched_rules.into_iter().map(|r| r.name.clone()).collect(),
        outcome: match response.continue_ {
            true if response.context.is_some() => Outcome::Inject,
            true => Outcome::Allow,
            false => Outcome::Block,
        },
        timing: LogTiming {
            processing_ms: processing_time,
            rules_evaluated: config.enabled_rules().len(),
        },
        metadata: Some(LogMetadata {
            injected_files: response
                .context
                .as_ref()
                .map(|_| vec!["injected".to_string()]),
            validator_output: None,
        }),
        // New enhanced logging fields
        event_details: Some(event_details),
        response: Some(response_summary),
        raw_event: if debug_config.enabled {
            Some(serde_json::to_value(&event).unwrap_or_default())
        } else {
            None
        },
        rule_evaluations: if debug_config.enabled {
            Some(rule_evaluations)
        } else {
            None
        },
    };

    // Log asynchronously (don't fail the response if logging fails)
    let _ = log_entry(entry).await;

    // Add timing to response
    let mut response = response;
    response.timing = Some(Timing {
        processing_ms: processing_time,
        rules_evaluated: config.enabled_rules().len(),
    });

    Ok(response)
}

/// Evaluate all enabled rules against an event
/// Rules are sorted by priority (higher first) by config.enabled_rules()
async fn evaluate_rules<'a>(
    event: &'a Event,
    config: &'a Config,
    debug_config: &DebugConfig,
) -> Result<(Vec<&'a Rule>, Response, Vec<RuleEvaluation>)> {
    let mut matched_rules = Vec::new();
    let mut response = Response::allow();
    let mut rule_evaluations = Vec::new();

    // Get enabled rules (already sorted by priority in Config::enabled_rules)
    for rule in config.enabled_rules() {
        let (matched, matcher_results) = if debug_config.enabled {
            matches_rule_with_debug(event, rule)
        } else {
            (matches_rule(event, rule), None)
        };

        let rule_evaluation = RuleEvaluation {
            rule_name: rule.name.clone(),
            matched,
            matcher_results,
        };
        rule_evaluations.push(rule_evaluation);

        if matched {
            matched_rules.push(rule);

            // Execute rule actions based on mode (Phase 2 Governance)
            let mode = rule.effective_mode();
            let rule_response = execute_rule_actions_with_mode(event, rule, config, mode).await?;

            // Merge responses based on mode (block takes precedence, inject accumulates)
            response = merge_responses_with_mode(response, rule_response, mode);
        }
    }

    Ok((matched_rules, response, rule_evaluations))
}

/// Check if a rule matches the given event
fn matches_rule(event: &Event, rule: &Rule) -> bool {
    let matchers = &rule.matchers;

    // Check tool name
    if let Some(ref tools) = matchers.tools {
        if let Some(ref tool_name) = event.tool_name {
            if !tools.contains(tool_name) {
                return false;
            }
        } else {
            return false; // Rule requires tool but event has none
        }
    }

    // Check command patterns (for Bash tool)
    if let Some(ref pattern) = matchers.command_match {
        if let Some(ref tool_input) = event.tool_input {
            if let Some(command) = tool_input.get("command").and_then(|c| c.as_str()) {
                if let Ok(regex) = Regex::new(pattern) {
                    if !regex.is_match(command) {
                        return false;
                    }
                }
            }
        }
    }

    // Check file extensions
    if let Some(ref extensions) = matchers.extensions {
        if let Some(ref tool_input) = event.tool_input {
            if let Some(file_path) = tool_input.get("filePath").and_then(|p| p.as_str()) {
                let path_ext = Path::new(file_path)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("");

                if !extensions
                    .iter()
                    .any(|ext| ext == &format!(".{}", path_ext))
                {
                    return false;
                }
            }
        }
    }

    // Check directory patterns
    if let Some(ref directories) = matchers.directories {
        if let Some(ref tool_input) = event.tool_input {
            if let Some(file_path) = tool_input.get("filePath").and_then(|p| p.as_str()) {
                let path = Path::new(file_path);
                let path_str = path.to_string_lossy();

                if !directories.iter().any(|dir| {
                    // Simple glob matching - in production, use a proper glob library
                    path_str.contains(dir.trim_end_matches("/**"))
                        || path_str.contains(dir.trim_end_matches("/*"))
                }) {
                    return false;
                }
            }
        }
    }

    // Check operations (event types)
    if let Some(ref operations) = matchers.operations {
        let event_type_str = event.event_type.to_string();
        if !operations.contains(&event_type_str) {
            return false;
        }
    }

    true
}

/// Check if a rule matches the given event (debug version with matcher results)
fn matches_rule_with_debug(event: &Event, rule: &Rule) -> (bool, Option<MatcherResults>) {
    let matchers = &rule.matchers;
    let mut matcher_results = MatcherResults::default();
    let mut overall_match = true;

    // Check tool name
    if let Some(ref tools) = matchers.tools {
        matcher_results.tools_matched = Some(if let Some(ref tool_name) = event.tool_name {
            tools.contains(tool_name)
        } else {
            false // Rule requires tool but event has none
        });
        if !matcher_results.tools_matched.unwrap() {
            overall_match = false;
        }
    }

    // Check command patterns (for Bash tool)
    if let Some(ref pattern) = matchers.command_match {
        matcher_results.command_match_matched =
            Some(if let Some(ref tool_input) = event.tool_input {
                if let Some(command) = tool_input.get("command").and_then(|c| c.as_str()) {
                    if let Ok(regex) = Regex::new(pattern) {
                        regex.is_match(command)
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            });
        if !matcher_results.command_match_matched.unwrap() {
            overall_match = false;
        }
    }

    // Check file extensions
    if let Some(ref extensions) = matchers.extensions {
        matcher_results.extensions_matched = Some(if let Some(ref tool_input) = event.tool_input {
            if let Some(file_path) = tool_input.get("filePath").and_then(|p| p.as_str()) {
                let path_ext = Path::new(file_path)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("");

                extensions
                    .iter()
                    .any(|ext| ext == &format!(".{}", path_ext))
            } else {
                false
            }
        } else {
            false
        });
        if !matcher_results.extensions_matched.unwrap() {
            overall_match = false;
        }
    }

    // Check directory patterns
    if let Some(ref directories) = matchers.directories {
        matcher_results.directories_matched =
            Some(if let Some(ref tool_input) = event.tool_input {
                if let Some(file_path) = tool_input.get("filePath").and_then(|p| p.as_str()) {
                    let path = Path::new(file_path);
                    let path_str = path.to_string_lossy();

                    directories.iter().any(|dir| {
                        // Simple glob matching - in production, use a proper glob library
                        path_str.contains(dir.trim_end_matches("/**"))
                            || path_str.contains(dir.trim_end_matches("/*"))
                    })
                } else {
                    false
                }
            } else {
                false
            });
        if !matcher_results.directories_matched.unwrap() {
            overall_match = false;
        }
    }

    // Check operations (event types)
    if let Some(ref operations) = matchers.operations {
        matcher_results.operations_matched = Some({
            let event_type_str = event.event_type.to_string();
            operations.contains(&event_type_str)
        });
        if !matcher_results.operations_matched.unwrap() {
            overall_match = false;
        }
    }

    (overall_match, Some(matcher_results))
}

/// Execute actions for a matching rule
async fn execute_rule_actions(event: &Event, rule: &Rule, config: &Config) -> Result<Response> {
    let actions = &rule.actions;

    // Handle blocking
    if let Some(block) = actions.block {
        if block {
            return Ok(Response::block(format!(
                "Blocked by rule '{}': {}",
                rule.name,
                rule.description.as_deref().unwrap_or("No description")
            )));
        }
    }

    // Handle conditional blocking
    if let Some(ref pattern) = actions.block_if_match {
        if let Some(ref tool_input) = event.tool_input {
            if let Some(content) = tool_input
                .get("newString")
                .or_else(|| tool_input.get("content"))
                .and_then(|c| c.as_str())
            {
                if let Ok(regex) = Regex::new(pattern) {
                    if regex.is_match(content) {
                        return Ok(Response::block(format!(
                            "Content blocked by rule '{}': matches pattern '{}'",
                            rule.name, pattern
                        )));
                    }
                }
            }
        }
    }

    // Handle context injection
    if let Some(ref inject_path) = actions.inject {
        match read_context_file(inject_path).await {
            Ok(context) => {
                return Ok(Response::inject(context));
            }
            Err(e) => {
                tracing::warn!("Failed to read context file '{}': {}", inject_path, e);
                // Continue without injection rather than failing
            }
        }
    }

    // Handle script execution
    if let Some(ref script_path) = actions.run {
        match execute_validator_script(event, script_path, rule, config).await {
            Ok(script_response) => {
                return Ok(script_response);
            }
            Err(e) => {
                tracing::warn!("Script execution failed for rule '{}': {}", rule.name, e);
                if !config.settings.fail_open {
                    return Err(e);
                }
                // Continue if fail_open is enabled
            }
        }
    }

    Ok(Response::allow())
}

/// Read context file for injection
async fn read_context_file(path: &str) -> Result<String> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}

/// Execute a validator script
async fn execute_validator_script(
    event: &Event,
    script_path: &str,
    rule: &Rule,
    config: &Config,
) -> Result<Response> {
    let timeout_duration = rule
        .metadata
        .as_ref()
        .map(|m| m.timeout)
        .unwrap_or(config.settings.script_timeout);

    let mut command = Command::new(script_path);
    command.stdin(std::process::Stdio::piped());
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());

    let child_result = command.spawn();

    let mut child = match child_result {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Failed to spawn validator script '{}': {}", script_path, e);
            if config.settings.fail_open {
                return Ok(Response::allow());
            }
            return Err(e.into());
        }
    };

    // Send event as JSON to script stdin
    if let Some(stdin) = child.stdin.as_mut() {
        let event_json = serde_json::to_string(event)?;
        tokio::io::AsyncWriteExt::write_all(stdin, event_json.as_bytes()).await?;
    }

    // Close stdin to signal end of input
    drop(child.stdin.take());

    // Wait for script completion with timeout
    let output_result = timeout(
        Duration::from_secs(timeout_duration as u64),
        child.wait_with_output(),
    )
    .await;

    let output = match output_result {
        Ok(Ok(o)) => o,
        Ok(Err(e)) => {
            tracing::warn!("Validator script '{}' failed: {}", script_path, e);
            if config.settings.fail_open {
                return Ok(Response::allow());
            }
            return Err(e.into());
        }
        Err(_) => {
            tracing::warn!(
                "Validator script '{}' timed out after {}s",
                script_path,
                timeout_duration
            );
            if config.settings.fail_open {
                return Ok(Response::allow());
            }
            return Err(anyhow::anyhow!("Script timed out"));
        }
    };

    let exit_code = output.status.code().unwrap_or(-1);

    if exit_code == 0 {
        // Script allowed the operation - check if stdout has context to inject
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.trim().is_empty() {
            Ok(Response::allow())
        } else {
            Ok(Response::inject(stdout.trim().to_string()))
        }
    } else {
        // Script blocked the operation
        let stderr = String::from_utf8_lossy(&output.stderr);
        let reason = if stderr.is_empty() {
            format!("Blocked by validator script '{}'", script_path)
        } else {
            format!("Blocked by validator script: {}", stderr.trim())
        };
        Ok(Response::block(reason))
    }
}

/// Merge two responses (block takes precedence, inject accumulates)
fn merge_responses(mut existing: Response, new: Response) -> Response {
    // Block takes precedence
    if !new.continue_ {
        return new;
    }

    // Accumulate context
    if let Some(new_context) = new.context {
        if let Some(existing_context) = existing.context.as_mut() {
            existing_context.push_str("\n\n");
            existing_context.push_str(&new_context);
        } else {
            existing.context = Some(new_context);
        }
    }

    existing
}

// =============================================================================
// Phase 2 Governance: Mode-Based Action Execution
// =============================================================================

/// Execute rule actions respecting the policy mode
///
/// Mode behavior:
/// - Enforce: Normal execution (block, inject, run validators)
/// - Warn: Never blocks, injects warning context instead
/// - Audit: Logs only, no blocking or injection
async fn execute_rule_actions_with_mode(
    event: &Event,
    rule: &Rule,
    config: &Config,
    mode: PolicyMode,
) -> Result<Response> {
    match mode {
        PolicyMode::Enforce => {
            // Normal execution - delegate to existing function
            execute_rule_actions(event, rule, config).await
        }
        PolicyMode::Warn => {
            // Never block, inject warning instead
            execute_rule_actions_warn_mode(event, rule, config).await
        }
        PolicyMode::Audit => {
            // Log only, no blocking or injection
            Ok(Response::allow())
        }
    }
}

/// Execute rule actions in warn mode (never blocks, injects warnings)
async fn execute_rule_actions_warn_mode(
    event: &Event,
    rule: &Rule,
    config: &Config,
) -> Result<Response> {
    let actions = &rule.actions;

    // Convert blocks to warnings
    if let Some(block) = actions.block {
        if block {
            let warning = format!(
                "[WARNING] Rule '{}' would block this operation: {}\n\
                 This rule is in 'warn' mode - operation will proceed.",
                rule.name,
                rule.description.as_deref().unwrap_or("No description")
            );
            return Ok(Response::inject(warning));
        }
    }

    // Convert conditional blocks to warnings
    if let Some(ref pattern) = actions.block_if_match {
        if let Some(ref tool_input) = event.tool_input {
            if let Some(content) = tool_input
                .get("newString")
                .or_else(|| tool_input.get("content"))
                .and_then(|c| c.as_str())
            {
                if let Ok(regex) = Regex::new(pattern) {
                    if regex.is_match(content) {
                        let warning = format!(
                            "[WARNING] Rule '{}' would block this content (matches pattern '{}').\n\
                             This rule is in 'warn' mode - operation will proceed.",
                            rule.name, pattern
                        );
                        return Ok(Response::inject(warning));
                    }
                }
            }
        }
    }

    // Context injection still works in warn mode
    if let Some(ref inject_path) = actions.inject {
        match read_context_file(inject_path).await {
            Ok(context) => {
                return Ok(Response::inject(context));
            }
            Err(e) => {
                tracing::warn!("Failed to read context file '{}': {}", inject_path, e);
            }
        }
    }

    // Script execution - convert blocks to warnings
    if let Some(ref script_path) = actions.run {
        match execute_validator_script(event, script_path, rule, config).await {
            Ok(script_response) => {
                if !script_response.continue_ {
                    // Convert block to warning
                    let warning = format!(
                        "[WARNING] Validator script '{}' would block this operation: {}\n\
                         This rule is in 'warn' mode - operation will proceed.",
                        script_path,
                        script_response.reason.as_deref().unwrap_or("No reason")
                    );
                    return Ok(Response::inject(warning));
                }
                return Ok(script_response);
            }
            Err(e) => {
                tracing::warn!("Script execution failed for rule '{}': {}", rule.name, e);
                if !config.settings.fail_open {
                    // Even in warn mode, respect fail_open setting
                    return Err(e);
                }
            }
        }
    }

    Ok(Response::allow())
}

/// Merge responses with mode awareness
///
/// Mode affects merge behavior:
/// - Enforce: Normal merge (blocks take precedence)
/// - Warn: Blocks become warnings (never blocks)
/// - Audit: No merging (allow always)
fn merge_responses_with_mode(existing: Response, new: Response, mode: PolicyMode) -> Response {
    match mode {
        PolicyMode::Enforce => {
            // Normal merge behavior
            merge_responses(existing, new)
        }
        PolicyMode::Warn | PolicyMode::Audit => {
            // In warn/audit mode, new response should never block
            // (execute_rule_actions_with_mode ensures this)
            merge_responses(existing, new)
        }
    }
}

/// Determine the decision outcome based on response and mode
#[allow(dead_code)] // Used in Phase 2.2 (enhanced logging)
pub fn determine_decision(response: &Response, mode: PolicyMode) -> Decision {
    match mode {
        PolicyMode::Audit => Decision::Audited,
        PolicyMode::Warn => {
            if response.context.is_some() {
                Decision::Warned
            } else {
                Decision::Allowed
            }
        }
        PolicyMode::Enforce => {
            if !response.continue_ {
                Decision::Blocked
            } else {
                // Both injection and no-injection count as allowed
                Decision::Allowed
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Actions, EventType, Matchers};
    use chrono::Utc;

    #[tokio::test]
    async fn test_rule_matching() {
        let event = Event {
            event_type: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(serde_json::json!({
                "command": "git push --force"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
        };

        let rule = Rule {
            name: "block-force-push".to_string(),
            description: Some("Block force push".to_string()),
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                command_match: Some(r"git push.*--force".to_string()),
                extensions: None,
                directories: None,
                operations: None,
            },
            actions: Actions {
                block: Some(true),
                inject: None,
                run: None,
                block_if_match: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(matches_rule(&event, &rule));
    }

    #[tokio::test]
    async fn test_rule_non_matching() {
        let event = Event {
            event_type: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(serde_json::json!({
                "command": "git status"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
        };

        let rule = Rule {
            name: "block-force-push".to_string(),
            description: Some("Block force push".to_string()),
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                command_match: Some(r"git push.*--force".to_string()),
                extensions: None,
                directories: None,
                operations: None,
            },
            actions: Actions {
                block: Some(true),
                inject: None,
                run: None,
                block_if_match: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!matches_rule(&event, &rule));
    }

    #[tokio::test]
    async fn test_response_merging() {
        let allow = Response::allow();
        let block = Response::block("blocked");
        let inject = Response::inject("context");

        // Block takes precedence
        let merged = merge_responses(allow.clone(), block.clone());
        assert!(!merged.continue_);

        // Inject accumulates
        let merged = merge_responses(inject.clone(), inject.clone());
        assert!(merged.continue_);
        assert!(merged.context.as_ref().unwrap().contains("context"));
    }

    // =========================================================================
    // Phase 2 Governance: Mode-Based Execution Tests
    // =========================================================================

    #[test]
    fn test_determine_decision_enforce_blocked() {
        let response = Response::block("blocked");
        let decision = determine_decision(&response, PolicyMode::Enforce);
        assert_eq!(decision, Decision::Blocked);
    }

    #[test]
    fn test_determine_decision_enforce_allowed() {
        let response = Response::allow();
        let decision = determine_decision(&response, PolicyMode::Enforce);
        assert_eq!(decision, Decision::Allowed);
    }

    #[test]
    fn test_determine_decision_warn_mode() {
        let response = Response::inject("warning context");
        let decision = determine_decision(&response, PolicyMode::Warn);
        assert_eq!(decision, Decision::Warned);
    }

    #[test]
    fn test_determine_decision_audit_mode() {
        // In audit mode, everything is Audited regardless of response
        let response = Response::block("would block");
        let decision = determine_decision(&response, PolicyMode::Audit);
        assert_eq!(decision, Decision::Audited);
    }

    #[test]
    fn test_merge_responses_with_mode_enforce() {
        let allow = Response::allow();
        let block = Response::block("blocked");

        // In enforce mode, block takes precedence
        let merged = merge_responses_with_mode(allow, block, PolicyMode::Enforce);
        assert!(!merged.continue_);
    }

    #[test]
    fn test_merge_responses_with_mode_warn() {
        let allow = Response::allow();
        let warning = Response::inject("warning");

        // In warn mode, warnings accumulate but never block
        let merged = merge_responses_with_mode(allow, warning, PolicyMode::Warn);
        assert!(merged.continue_);
        assert!(merged.context.is_some());
    }

    #[test]
    fn test_rule_effective_mode_defaults_to_enforce() {
        let rule = Rule {
            name: "test".to_string(),
            description: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None, // No mode specified
            priority: None,
            governance: None,
            metadata: None,
        };
        assert_eq!(rule.effective_mode(), PolicyMode::Enforce);
    }

    #[test]
    fn test_rule_effective_mode_explicit_audit() {
        let rule = Rule {
            name: "test".to_string(),
            description: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: Some(PolicyMode::Audit),
            priority: None,
            governance: None,
            metadata: None,
        };
        assert_eq!(rule.effective_mode(), PolicyMode::Audit);
    }
}
