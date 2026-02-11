use anyhow::Result;
use evalexpr::{
    ContextWithMutableVariables, DefaultNumericTypes, HashMapContext, Value,
    eval_boolean_with_context,
};
use regex::Regex;

use std::path::Path;
use tokio::process::Command;
use tokio::time::{Duration, timeout};

use crate::config::Config;
use crate::logging::log_entry;
use crate::models::LogMetadata;
use crate::models::{
    DebugConfig, Decision, Event, EventDetails, GovernanceMetadata, LogEntry, LogTiming,
    MatcherResults, Outcome, PolicyMode, Response, ResponseSummary, Rule, RuleEvaluation, Timing,
    TrustLevel,
};

/// Process a hook event and return the appropriate response
pub async fn process_event(event: Event, debug_config: &DebugConfig) -> Result<Response> {
    let start_time = std::time::Instant::now();

    // Load configuration using the event's cwd (sent by Claude Code) for project-level config
    let config = Config::load(event.cwd.as_ref().map(|p| Path::new(p.as_str())))?;

    // Evaluate rules (with optional debug tracking)
    let (matched_rules, response, rule_evaluations) =
        evaluate_rules(&event, &config, debug_config).await?;

    let processing_time = start_time.elapsed().as_millis() as u64;

    // Build enhanced logging fields
    let event_details = EventDetails::extract(&event);
    let response_summary = ResponseSummary::from_response(&response);

    // Extract governance data from the primary matched rule (first/highest priority)
    let (primary_mode, primary_priority, primary_governance, trust_level) =
        extract_governance_data(&matched_rules);

    // Determine decision based on response and mode
    let decision = primary_mode.map(|m| determine_decision(&response, m));

    // Log the event with enhanced fields
    let entry = LogEntry {
        timestamp: event.timestamp,
        event_type: format!("{:?}", event.hook_event_name),
        session_id: event.session_id.clone(),
        tool_name: event.tool_name.clone(),
        rules_matched: matched_rules.iter().map(|r| r.name.clone()).collect(),
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
        // Enhanced logging fields (CRD-001)
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
        // Phase 2.2 Governance logging fields
        mode: primary_mode,
        priority: primary_priority,
        decision,
        governance: primary_governance,
        trust_level,
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

/// Extract governance data from matched rules
/// Returns (mode, priority, governance, trust_level) from the primary (first) matched rule
fn extract_governance_data(
    matched_rules: &[&Rule],
) -> (
    Option<PolicyMode>,
    Option<i32>,
    Option<GovernanceMetadata>,
    Option<TrustLevel>,
) {
    if let Some(primary) = matched_rules.first() {
        let mode = Some(primary.effective_mode());
        let priority = Some(primary.effective_priority());
        let governance = primary.governance.clone();
        let trust_level = primary.actions.trust_level();
        (mode, priority, governance, trust_level)
    } else {
        (None, None, None, None)
    }
}

/// Build evaluation context for enabled_when expressions
///
/// Creates a context with:
/// - env_* variables for all environment variables
/// - tool_name: the tool being used (or empty string)
/// - event_type: the hook event type
fn build_eval_context(event: &Event) -> HashMapContext<DefaultNumericTypes> {
    let mut ctx = HashMapContext::new();

    // Add environment variables with env_ prefix
    for (key, value) in std::env::vars() {
        let var_name = format!("env_{}", key);
        ctx.set_value(var_name.into(), Value::String(value)).ok();
    }

    // Add tool name (empty string if none)
    let tool_name = event.tool_name.as_deref().unwrap_or("").to_string();
    ctx.set_value("tool_name".into(), Value::String(tool_name))
        .ok();

    // Add event type
    ctx.set_value(
        "event_type".into(),
        Value::String(event.hook_event_name.to_string()),
    )
    .ok();

    ctx
}

/// Check if a rule is enabled based on its enabled_when expression
///
/// Returns true if:
/// - No enabled_when expression (always enabled)
/// - enabled_when expression evaluates to true
///
/// Returns false if:
/// - enabled_when expression evaluates to false
/// - Expression evaluation fails (fail-closed for safety)
fn is_rule_enabled(rule: &Rule, event: &Event) -> bool {
    match &rule.enabled_when {
        None => true, // No condition = always enabled
        Some(expr) => {
            let ctx = build_eval_context(event);
            match eval_boolean_with_context(expr, &ctx) {
                Ok(result) => result,
                Err(e) => {
                    tracing::warn!(
                        "enabled_when expression failed for rule '{}': {} - treating as disabled",
                        rule.name,
                        e
                    );
                    false // Fail-closed: invalid expression disables rule
                }
            }
        }
    }
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
        // Check enabled_when before matchers (Phase 3: conditional rule activation)
        if !is_rule_enabled(rule, event) {
            if debug_config.enabled {
                rule_evaluations.push(RuleEvaluation {
                    rule_name: rule.name.clone(),
                    matched: false,
                    matcher_results: None,
                });
            }
            continue; // Skip rule entirely
        }

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
        let event_type_str = event.hook_event_name.to_string();
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
            let event_type_str = event.hook_event_name.to_string();
            operations.contains(&event_type_str)
        });
        if !matcher_results.operations_matched.unwrap() {
            overall_match = false;
        }
    }

    (overall_match, Some(matcher_results))
}

/// Execute a shell command and capture stdout for context injection
///
/// Unlike validators:
/// - No stdin input needed
/// - Raw text output (not JSON)
/// - Fail-open: command failures log warning but don't block
async fn execute_inject_command(command_str: &str, rule: &Rule, config: &Config) -> Option<String> {
    let timeout_secs = rule
        .metadata
        .as_ref()
        .map(|m| m.timeout)
        .unwrap_or(config.settings.script_timeout);

    // Use platform-specific shell to execute (enables pipes, redirects, etc.)
    let mut command = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C");
        cmd.arg(command_str);
        cmd
    } else {
        let mut cmd = Command::new("sh");
        cmd.arg("-c");
        cmd.arg(command_str);
        cmd
    };
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    // No stdin - don't pipe it (causes hangs)

    let child = match command.spawn() {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!(
                "Failed to spawn inject_command '{}' for rule '{}': {}",
                command_str,
                rule.name,
                e
            );
            return None;
        }
    };

    let output = match timeout(
        Duration::from_secs(timeout_secs as u64),
        child.wait_with_output(),
    )
    .await
    {
        Ok(Ok(output)) => output,
        Ok(Err(e)) => {
            tracing::warn!(
                "inject_command '{}' for rule '{}' failed: {}",
                command_str,
                rule.name,
                e
            );
            return None;
        }
        Err(_) => {
            tracing::warn!(
                "inject_command '{}' for rule '{}' timed out after {}s",
                command_str,
                rule.name,
                timeout_secs
            );
            return None;
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::warn!(
            "inject_command '{}' for rule '{}' failed with exit code {}: {}",
            command_str,
            rule.name,
            output.status.code().unwrap_or(-1),
            stderr.trim()
        );
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if stdout.trim().is_empty() {
        return None; // No content to inject
    }

    Some(stdout)
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

    // Handle inline content injection (takes precedence over inject)
    if let Some(ref inline_content) = actions.inject_inline {
        return Ok(Response::inject(inline_content.clone()));
    }

    // Handle command-based injection (after inject_inline, before inject file)
    if let Some(ref command_str) = actions.inject_command {
        if let Some(output) = execute_inject_command(command_str, rule, config).await {
            return Ok(Response::inject(output));
        }
        // Command failed or produced no output - continue to next action
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
    if let Some(script_path) = actions.script_path() {
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

    // Handle inline content injection (takes precedence over inject)
    if let Some(ref inline_content) = actions.inject_inline {
        return Ok(Response::inject(inline_content.clone()));
    }

    // Handle command-based injection (after inject_inline, before inject file)
    if let Some(ref command_str) = actions.inject_command {
        if let Some(output) = execute_inject_command(command_str, rule, config).await {
            return Ok(Response::inject(output));
        }
        // Command failed or produced no output - continue to next action
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
    if let Some(script_path) = actions.script_path() {
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

// =============================================================================
// Phase 2 Governance: Conflict Resolution
// =============================================================================

/// Mode precedence for conflict resolution
/// Returns a numeric value where higher = wins
#[allow(dead_code)] // Used in conflict resolution tests and future enhancements
pub fn mode_precedence(mode: PolicyMode) -> u8 {
    match mode {
        PolicyMode::Enforce => 3, // Highest - always wins
        PolicyMode::Warn => 2,    // Middle
        PolicyMode::Audit => 1,   // Lowest - only logs
    }
}

/// Represents a potential rule response for conflict resolution
#[allow(dead_code)] // Used in conflict resolution tests and future multi-rule scenarios
#[derive(Debug, Clone)]
pub struct RuleConflictEntry<'a> {
    pub rule: &'a Rule,
    pub response: Response,
    pub mode: PolicyMode,
    pub priority: i32,
}

/// Resolve conflicts between multiple matched rules
///
/// Resolution order:
/// 1. Enforce mode wins over warn and audit (regardless of priority)
/// 2. Among same modes, higher priority wins
/// 3. For multiple blocks, use highest priority block's message
/// 4. Warnings and injections are accumulated
#[allow(dead_code)] // Used when multiple rules need explicit conflict resolution
pub fn resolve_conflicts(entries: &[RuleConflictEntry]) -> Response {
    if entries.is_empty() {
        return Response::allow();
    }

    // Separate by mode
    let enforce_entries: Vec<_> = entries
        .iter()
        .filter(|e| e.mode == PolicyMode::Enforce)
        .collect();
    let warn_entries: Vec<_> = entries
        .iter()
        .filter(|e| e.mode == PolicyMode::Warn)
        .collect();

    // Check for enforce blocks (highest precedence)
    for entry in &enforce_entries {
        if !entry.response.continue_ {
            // First enforce block wins (entries are pre-sorted by priority)
            return entry.response.clone();
        }
    }

    // Accumulate all injections (from enforce and warn modes)
    let mut accumulated_context: Option<String> = None;

    // Add enforce injections first
    for entry in &enforce_entries {
        if let Some(ref ctx) = entry.response.context {
            if let Some(ref mut acc) = accumulated_context {
                acc.push_str("\n\n");
                acc.push_str(ctx);
            } else {
                accumulated_context = Some(ctx.clone());
            }
        }
    }

    // Add warn injections
    for entry in &warn_entries {
        if let Some(ref ctx) = entry.response.context {
            if let Some(ref mut acc) = accumulated_context {
                acc.push_str("\n\n");
                acc.push_str(ctx);
            } else {
                accumulated_context = Some(ctx.clone());
            }
        }
    }

    // Return accumulated response
    if let Some(context) = accumulated_context {
        Response::inject(context)
    } else {
        Response::allow()
    }
}

/// Compare two rules for conflict resolution
/// Returns true if rule_a should take precedence over rule_b
#[allow(dead_code)] // Used in conflict resolution tests and future multi-rule scenarios
pub fn rule_takes_precedence(rule_a: &Rule, rule_b: &Rule) -> bool {
    let mode_a = rule_a.effective_mode();
    let mode_b = rule_b.effective_mode();

    // First compare by mode precedence
    let prec_a = mode_precedence(mode_a);
    let prec_b = mode_precedence(mode_b);

    if prec_a != prec_b {
        return prec_a > prec_b;
    }

    // Same mode: compare by priority
    rule_a.effective_priority() > rule_b.effective_priority()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Actions, EventType, Matchers};
    use chrono::Utc;

    #[tokio::test]
    async fn test_rule_matching() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(serde_json::json!({
                "command": "git push --force"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let rule = Rule {
            name: "block-force-push".to_string(),
            description: Some("Block force push".to_string()),
            enabled_when: None,
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
                inject_inline: None,
                inject_command: None,
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
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(serde_json::json!({
                "command": "git status"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let rule = Rule {
            name: "block-force-push".to_string(),
            description: Some("Block force push".to_string()),
            enabled_when: None,
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
                inject_inline: None,
                inject_command: None,
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
    // Phase 3: is_rule_enabled Tests
    // =========================================================================

    #[test]
    fn test_is_rule_enabled_no_condition() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let rule = Rule {
            name: "no-condition".to_string(),
            description: None,
            enabled_when: None, // No condition = always enabled
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(is_rule_enabled(&rule, &event));
    }

    #[test]
    fn test_is_rule_enabled_true_condition() {
        // Use existing PATH env var (always exists on all systems)
        // Check that it's not empty (which is always true)
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let rule = Rule {
            name: "true-condition".to_string(),
            description: None,
            // PATH exists and is not empty on all systems
            enabled_when: Some(r#"env_PATH != """#.to_string()),
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(is_rule_enabled(&rule, &event));
    }

    #[test]
    fn test_is_rule_enabled_false_condition() {
        // Test a condition that evaluates to false
        // Check that a non-existent env var returns empty string and fails condition
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let rule = Rule {
            name: "false-condition".to_string(),
            description: None,
            // This non-existent var won't be in context, so comparison fails
            // Use a simple false expression instead
            enabled_when: Some(r#"1 == 2"#.to_string()), // Always false
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!is_rule_enabled(&rule, &event));
    }

    #[test]
    fn test_is_rule_enabled_invalid_expression() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let rule = Rule {
            name: "invalid-expression".to_string(),
            description: None,
            enabled_when: Some("this is not a valid expression !!!".to_string()),
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        // Invalid expressions should return false (fail-closed)
        assert!(!is_rule_enabled(&rule, &event));
    }

    #[test]
    fn test_is_rule_enabled_tool_name_context() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
        };

        let rule = Rule {
            name: "tool-name-check".to_string(),
            description: None,
            enabled_when: Some(r#"tool_name == "Bash""#.to_string()),
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(is_rule_enabled(&rule, &event));

        // Test with different tool name in expression
        let rule_edit = Rule {
            name: "tool-name-check-edit".to_string(),
            description: None,
            enabled_when: Some(r#"tool_name == "Edit""#.to_string()),
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        // Should be false because event.tool_name is "Bash", not "Edit"
        assert!(!is_rule_enabled(&rule_edit, &event));
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
            enabled_when: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
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
            enabled_when: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
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

    // =========================================================================
    // Phase 2 Governance: Conflict Resolution Tests
    // =========================================================================

    fn create_rule_with_mode(name: &str, mode: PolicyMode, priority: i32) -> Rule {
        Rule {
            name: name.to_string(),
            description: Some(format!("{} rule", name)),
            enabled_when: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
            },
            mode: Some(mode),
            priority: Some(priority),
            governance: None,
            metadata: None,
        }
    }

    #[test]
    fn test_mode_precedence() {
        assert!(mode_precedence(PolicyMode::Enforce) > mode_precedence(PolicyMode::Warn));
        assert!(mode_precedence(PolicyMode::Warn) > mode_precedence(PolicyMode::Audit));
        assert!(mode_precedence(PolicyMode::Enforce) > mode_precedence(PolicyMode::Audit));
    }

    #[test]
    fn test_rule_takes_precedence_mode_wins() {
        let enforce_rule = create_rule_with_mode("enforce", PolicyMode::Enforce, 0);
        let warn_rule = create_rule_with_mode("warn", PolicyMode::Warn, 100);

        // Enforce wins over warn even with lower priority
        assert!(rule_takes_precedence(&enforce_rule, &warn_rule));
        assert!(!rule_takes_precedence(&warn_rule, &enforce_rule));
    }

    #[test]
    fn test_rule_takes_precedence_same_mode_priority_wins() {
        let high_priority = create_rule_with_mode("high", PolicyMode::Enforce, 100);
        let low_priority = create_rule_with_mode("low", PolicyMode::Enforce, 0);

        assert!(rule_takes_precedence(&high_priority, &low_priority));
        assert!(!rule_takes_precedence(&low_priority, &high_priority));
    }

    #[test]
    fn test_resolve_conflicts_enforce_block_wins() {
        let enforce_rule = create_rule_with_mode("enforce", PolicyMode::Enforce, 100);
        let warn_rule = create_rule_with_mode("warn", PolicyMode::Warn, 50);

        let entries = vec![
            RuleConflictEntry {
                rule: &enforce_rule,
                response: Response::block("Blocked by enforce rule"),
                mode: PolicyMode::Enforce,
                priority: 100,
            },
            RuleConflictEntry {
                rule: &warn_rule,
                response: Response::inject("Warning from warn rule"),
                mode: PolicyMode::Warn,
                priority: 50,
            },
        ];

        let resolved = resolve_conflicts(&entries);
        assert!(!resolved.continue_); // Block wins
        assert!(resolved.reason.as_ref().unwrap().contains("enforce"));
    }

    #[test]
    fn test_resolve_conflicts_warnings_accumulate() {
        let warn_rule1 = create_rule_with_mode("warn1", PolicyMode::Warn, 100);
        let warn_rule2 = create_rule_with_mode("warn2", PolicyMode::Warn, 50);

        let entries = vec![
            RuleConflictEntry {
                rule: &warn_rule1,
                response: Response::inject("Warning 1"),
                mode: PolicyMode::Warn,
                priority: 100,
            },
            RuleConflictEntry {
                rule: &warn_rule2,
                response: Response::inject("Warning 2"),
                mode: PolicyMode::Warn,
                priority: 50,
            },
        ];

        let resolved = resolve_conflicts(&entries);
        assert!(resolved.continue_); // No blocking in warn mode
        let context = resolved.context.unwrap();
        assert!(context.contains("Warning 1"));
        assert!(context.contains("Warning 2"));
    }

    #[test]
    fn test_resolve_conflicts_empty_allows() {
        let resolved = resolve_conflicts(&[]);
        assert!(resolved.continue_);
        assert!(resolved.context.is_none());
    }

    #[test]
    fn test_resolve_conflicts_audit_only_allows() {
        let audit_rule = create_rule_with_mode("audit", PolicyMode::Audit, 100);

        let entries = vec![RuleConflictEntry {
            rule: &audit_rule,
            response: Response::allow(), // Audit mode produces allow
            mode: PolicyMode::Audit,
            priority: 100,
        }];

        let resolved = resolve_conflicts(&entries);
        assert!(resolved.continue_);
    }

    #[test]
    fn test_resolve_conflicts_mixed_modes() {
        let enforce_rule = create_rule_with_mode("enforce", PolicyMode::Enforce, 50);
        let warn_rule = create_rule_with_mode("warn", PolicyMode::Warn, 100);
        let audit_rule = create_rule_with_mode("audit", PolicyMode::Audit, 200);

        // Enforce injects, warn injects, audit does nothing
        let entries = vec![
            RuleConflictEntry {
                rule: &enforce_rule,
                response: Response::inject("Enforce context"),
                mode: PolicyMode::Enforce,
                priority: 50,
            },
            RuleConflictEntry {
                rule: &warn_rule,
                response: Response::inject("Warning context"),
                mode: PolicyMode::Warn,
                priority: 100,
            },
            RuleConflictEntry {
                rule: &audit_rule,
                response: Response::allow(),
                mode: PolicyMode::Audit,
                priority: 200,
            },
        ];

        let resolved = resolve_conflicts(&entries);
        assert!(resolved.continue_);
        let context = resolved.context.unwrap();
        // Enforce comes first, then warn
        assert!(context.contains("Enforce context"));
        assert!(context.contains("Warning context"));
    }
}
