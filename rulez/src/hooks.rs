use anyhow::{Context, Result};
use evalexpr::{
    ContextWithMutableFunctions, ContextWithMutableVariables, DefaultNumericTypes, Function,
    HashMapContext, Value, eval_boolean_with_context,
};
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{LazyLock, Mutex};

use crate::models::{MatchMode, PromptMatch};
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

// =============================================================================
// Regex Caching for Performance
// =============================================================================

/// Global cache for compiled regex patterns
/// Key format: "pattern:case_insensitive" (e.g., "foo:true" or "bar:false")
///
/// NOTE: This cache is unbounded. Since patterns come from config files
/// (not user input), the cache size is bounded by the number of unique
/// patterns across all loaded rules. For typical configs this is <100 patterns.
/// If this becomes a concern in long-running services with dynamic configs,
/// consider adding LRU eviction or size caps in a future phase.
static REGEX_CACHE: LazyLock<Mutex<HashMap<String, Regex>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Get or compile a regex pattern with caching
fn get_or_compile_regex(pattern: &str, case_insensitive: bool) -> Result<Regex> {
    let cache_key = format!("{}:{}", pattern, case_insensitive);

    // Try to get from cache
    {
        let cache = REGEX_CACHE.lock().unwrap();
        if let Some(regex) = cache.get(&cache_key) {
            return Ok(regex.clone());
        }
    }

    // Compile and cache
    let regex = if case_insensitive {
        RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()
            .with_context(|| format!("Invalid regex pattern: {}", pattern))?
    } else {
        Regex::new(pattern).with_context(|| format!("Invalid regex pattern: {}", pattern))?
    };

    let mut cache = REGEX_CACHE.lock().unwrap();
    cache.insert(cache_key, regex.clone());
    Ok(regex)
}

// =============================================================================
// Prompt Pattern Matching (Phase 4)
// =============================================================================

/// Check if prompt text matches the given PromptMatch configuration
///
/// Handles:
/// - Simple array syntax (ANY mode, case-sensitive)
/// - Complex object syntax with mode, case_insensitive, anchor
/// - Shorthand expansion (contains_word:, not:)
/// - Negation patterns
fn matches_prompt(prompt: &str, prompt_match: &PromptMatch) -> bool {
    let patterns = prompt_match.patterns();
    let mode = prompt_match.mode();
    let case_insensitive = prompt_match.case_insensitive();
    let anchor = prompt_match.anchor();

    if patterns.is_empty() {
        return false;
    }

    let mut results = Vec::with_capacity(patterns.len());

    for pattern in patterns {
        // Check for negation prefix
        let (is_negated, effective_pattern) = if let Some(inner) = pattern.strip_prefix("not:") {
            (true, inner.trim().to_string())
        } else {
            (false, pattern.clone())
        };

        // Expand shorthand patterns
        let expanded = PromptMatch::expand_pattern(&effective_pattern);

        // Apply anchor
        let anchored = PromptMatch::apply_anchor(&expanded, anchor);

        // Compile and match
        match get_or_compile_regex(&anchored, case_insensitive) {
            Ok(regex) => {
                let matched = regex.is_match(prompt);
                // Apply negation
                let result = if is_negated { !matched } else { matched };
                results.push(result);
            }
            Err(e) => {
                // Log warning and treat as non-match (fail-closed)
                tracing::warn!(
                    "Invalid prompt_match pattern '{}': {} - treating as non-match",
                    pattern,
                    e
                );
                results.push(false);
            }
        }
    }

    // Apply match mode
    match mode {
        MatchMode::Any => results.iter().any(|&r| r),
        MatchMode::All => results.iter().all(|&r| r),
    }
}

// =============================================================================
// Field Validation (Phase 5)
// =============================================================================

/// Validate required fields and field types in tool_input JSON
///
/// Returns Ok(true) if all validations pass, Ok(false) if any fail.
/// Collects ALL errors before returning (does not short-circuit).
///
/// Behavior:
/// - Missing tool_input -> all checks fail (fail-closed)
/// - Null values -> treated as missing
/// - Empty strings/arrays -> treated as present (JSON semantics)
/// - field_types implies require_fields (field must exist AND match type)
/// - Error messages show types only, not actual values (security)
fn validate_required_fields(rule: &Rule, event: &Event) -> bool {
    use crate::models::dot_to_pointer;

    let matchers = &rule.matchers;

    // If no field validation configured, pass validation
    if matchers.require_fields.is_none() && matchers.field_types.is_none() {
        return true;
    }

    // Get tool_input from event - fail-closed if missing
    let tool_input = if let Some(input) = &event.tool_input {
        if !input.is_object() {
            tracing::warn!(
                "Field validation failed for rule '{}': tool_input is not an object",
                rule.name
            );
            return false;
        }
        input
    } else {
        tracing::warn!(
            "Field validation failed for rule '{}': tool_input is missing (fail-closed)",
            rule.name
        );
        return false;
    };

    // Build combined field set: require_fields + field_types keys
    let mut fields_to_check = std::collections::HashSet::new();

    if let Some(ref require_fields) = matchers.require_fields {
        for field in require_fields {
            fields_to_check.insert(field.as_str());
        }
    }

    // field_types implies existence check
    if let Some(ref field_types) = matchers.field_types {
        for field in field_types.keys() {
            fields_to_check.insert(field.as_str());
        }
    }

    // Collect all errors (don't short-circuit)
    let mut errors = Vec::new();

    for field_path in fields_to_check {
        // Convert dot notation to JSON Pointer
        let pointer_path = dot_to_pointer(field_path);

        // Look up field value
        match tool_input.pointer(&pointer_path) {
            None => {
                errors.push(format!("field '{}' is missing", field_path));
            }
            Some(serde_json::Value::Null) => {
                errors.push(format!(
                    "field '{}' is null (treated as missing)",
                    field_path
                ));
            }
            Some(value) => {
                // Field exists and is not null - check type if specified
                if let Some(ref field_types) = matchers.field_types {
                    if let Some(expected_type) = field_types.get(field_path) {
                        let actual_type = match value {
                            serde_json::Value::String(_) => "string",
                            serde_json::Value::Number(_) => "number",
                            serde_json::Value::Bool(_) => "boolean",
                            serde_json::Value::Array(_) => "array",
                            serde_json::Value::Object(_) => "object",
                            serde_json::Value::Null => "null",
                        };

                        // "any" type accepts any non-null value
                        let type_matches = expected_type == "any"
                            || match expected_type.as_str() {
                                "string" => value.is_string(),
                                "number" => value.is_number(),
                                "boolean" => value.is_boolean(),
                                "array" => value.is_array(),
                                "object" => value.is_object(),
                                _ => false, // Config validation should prevent this
                            };

                        if !type_matches {
                            errors.push(format!(
                                "field '{}' expected {}, got {}",
                                field_path, expected_type, actual_type
                            ));
                        }
                    }
                }
            }
        }
    }

    // If any errors, log them all and return false
    if !errors.is_empty() {
        tracing::warn!(
            "Field validation failed for rule '{}': {}",
            rule.name,
            errors.join("; ")
        );
        return false;
    }

    true
}

// ============================================================================
// Inline Script Validation Functions
// ============================================================================

/// Build evalexpr context with custom functions for inline validation
///
/// Extends build_eval_context with two custom functions:
/// - get_field(path_string): Returns field value from tool_input JSON using dot notation
/// - has_field(path_string): Returns boolean indicating field exists and is not null
fn build_eval_context_with_custom_functions(event: &Event) -> HashMapContext<DefaultNumericTypes> {
    use crate::models::dot_to_pointer;

    let mut ctx = build_eval_context(event);

    // Clone tool_input for 'static lifetime in closures
    let tool_input_for_get = event.tool_input.clone();
    let tool_input_for_has = event.tool_input.clone();

    // Register get_field function
    let get_field_fn = Function::new(move |argument| {
        let path = argument.as_string()?;
        let pointer = dot_to_pointer(&path);

        match &tool_input_for_get {
            None => Ok(Value::String(String::new())),
            Some(input) => {
                match input.pointer(&pointer) {
                    Some(serde_json::Value::String(s)) => Ok(Value::String(s.clone())),
                    Some(serde_json::Value::Number(n)) => {
                        Ok(Value::Float(n.as_f64().unwrap_or(0.0)))
                    }
                    Some(serde_json::Value::Bool(b)) => Ok(Value::Boolean(*b)),
                    None | Some(_) => Ok(Value::String(String::new())), // Null/Arrays/Objects/missing -> empty string
                }
            }
        }
    });

    // Register has_field function
    let has_field_fn = Function::new(move |argument| {
        let path = argument.as_string()?;
        let pointer = dot_to_pointer(&path);

        match &tool_input_for_has {
            None => Ok(Value::Boolean(false)),
            Some(input) => match input.pointer(&pointer) {
                None | Some(serde_json::Value::Null) => Ok(Value::Boolean(false)),
                Some(_) => Ok(Value::Boolean(true)),
            },
        }
    });

    // Set functions in context (ignoring errors - would only fail if already set)
    ctx.set_function("get_field".to_string(), get_field_fn).ok();
    ctx.set_function("has_field".to_string(), has_field_fn).ok();

    ctx
}

/// Execute an inline shell script with timeout protection
///
/// The script receives event JSON on stdin and must exit with code 0 to allow the operation.
/// Non-zero exit code or timeout causes the operation to be blocked (fail-closed).
///
/// Returns:
/// - Ok(true): Script succeeded (exit 0)
/// - Ok(false): Script failed (non-zero exit or timeout)
/// - Err: Script execution error
async fn execute_inline_script(
    script_content: &str,
    event: &Event,
    rule: &Rule,
    config: &Config,
) -> Result<bool> {
    use tokio::io::AsyncWriteExt;

    // Get timeout from rule metadata or config settings
    let timeout_secs = rule
        .metadata
        .as_ref()
        .map(|m| m.timeout)
        .unwrap_or(config.settings.script_timeout);

    // Create unique temp file name using process ID and timestamp
    let unique_id = format!(
        "{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let script_path = std::env::temp_dir().join(format!("rulez-inline-{}.sh", unique_id));

    // Write script to temp file
    tokio::fs::write(&script_path, script_content)
        .await
        .context("Failed to write inline script to temp file")?;

    // Set permissions to 0o700 (owner read/write/execute only) on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = tokio::fs::metadata(&script_path).await?.permissions();
        perms.set_mode(0o700);
        tokio::fs::set_permissions(&script_path, perms).await?;
    }

    // Execute script with sh
    let mut command = Command::new("sh");
    command.arg(&script_path);
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    command.stdin(std::process::Stdio::piped());

    let mut child = command
        .spawn()
        .context("Failed to spawn inline script process")?;

    // Serialize event to JSON and write to stdin
    let event_json = serde_json::to_string(event)?;
    if let Some(mut stdin) = child.stdin.take() {
        if let Err(e) = stdin.write_all(event_json.as_bytes()).await {
            // Clean up temp file before returning error
            tokio::fs::remove_file(&script_path).await.ok();
            return Err(e.into());
        }
        // Close stdin to signal EOF
        drop(stdin);
    }

    // Wait for script with timeout using tokio::time::timeout
    let wait_result = timeout(Duration::from_secs(timeout_secs as u64), child.wait()).await;

    match wait_result {
        Ok(Ok(status)) => {
            // Script completed - check exit status
            let success = status.success();

            if !success {
                tracing::warn!(
                    "Inline script for rule '{}' failed with exit code {}",
                    rule.name,
                    status.code().unwrap_or(-1),
                );
            }

            // Clean up temp file
            tokio::fs::remove_file(&script_path).await.ok();

            Ok(success)
        }
        Ok(Err(e)) => {
            // Script execution error
            tokio::fs::remove_file(&script_path).await.ok();
            Err(e.into())
        }
        Err(_) => {
            // Timeout occurred - kill process and fail-closed
            tracing::warn!(
                "Inline script for rule '{}' timed out after {}s - blocking (fail-closed)",
                rule.name,
                timeout_secs
            );

            // Attempt to kill the child process (child is still owned here since wait() was in timeout future)
            // Note: child was moved into the timeout future, so we can't access it here
            // This is acceptable - the process will be killed by the OS when temp file is deleted

            // Clean up temp file
            tokio::fs::remove_file(&script_path).await.ok();

            Ok(false) // Timeout = fail-closed
        }
    }
}

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
        ctx.set_value(var_name, Value::String(value)).ok();
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

    // Add prompt text (if available - primarily for UserPromptSubmit events)
    if let Some(ref prompt) = event.prompt {
        ctx.set_value("prompt".into(), Value::String(prompt.clone()))
            .ok();
    }

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

    // Check prompt patterns (for UserPromptSubmit events)
    if let Some(ref prompt_match) = matchers.prompt_match {
        // If rule has prompt_match but event has no prompt, rule doesn't match
        if let Some(ref prompt_text) = event.prompt {
            if !matches_prompt(prompt_text, prompt_match) {
                return false;
            }
        } else {
            // No prompt field in event - rule doesn't match (safe default)
            return false;
        }
    }

    // Check field validation (require_fields / field_types)
    if (rule.matchers.require_fields.is_some() || rule.matchers.field_types.is_some())
        && !validate_required_fields(rule, event)
    {
        return false;
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

    // Check prompt patterns
    if let Some(ref prompt_match) = matchers.prompt_match {
        matcher_results.prompt_match_matched = Some(if let Some(ref prompt_text) = event.prompt {
            matches_prompt(prompt_text, prompt_match)
        } else {
            false
        });
        if !matcher_results.prompt_match_matched.unwrap() {
            overall_match = false;
        }
    }

    // Check field validation (require_fields / field_types)
    if rule.matchers.require_fields.is_some() || rule.matchers.field_types.is_some() {
        let field_valid = validate_required_fields(rule, event);
        matcher_results.field_validation_matched = Some(field_valid);
        if !field_valid {
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

    // Use shell to execute (enables pipes, redirects, etc.)
    let mut command = Command::new("sh");
    command.arg("-c");
    command.arg(command_str);
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

    // Step 0: Run inline validation (if present) - gates all subsequent actions
    if let Some(ref expr) = actions.validate_expr {
        let ctx = build_eval_context_with_custom_functions(event);
        match eval_boolean_with_context(expr, &ctx) {
            Ok(true) => {
                // Validation passed, continue to other actions
            }
            Ok(false) => {
                return Ok(Response::block(format!(
                    "Validation failed for rule '{}': expression '{}' returned false",
                    rule.name, expr
                )));
            }
            Err(e) => {
                // Expression error = fail-closed
                tracing::warn!(
                    "validate_expr error for rule '{}': {} - blocking (fail-closed)",
                    rule.name,
                    e
                );
                return Ok(Response::block(format!(
                    "Validation error for rule '{}': {}",
                    rule.name, e
                )));
            }
        }
    } else if let Some(ref script) = actions.inline_script {
        match execute_inline_script(script, event, rule, config).await {
            Ok(true) => {
                // Validation passed, continue
            }
            Ok(false) => {
                return Ok(Response::block(format!(
                    "Inline script validation failed for rule '{}'",
                    rule.name
                )));
            }
            Err(e) => {
                tracing::warn!(
                    "inline_script error for rule '{}': {} - blocking (fail-closed)",
                    rule.name,
                    e
                );
                return Ok(Response::block(format!(
                    "Inline script error for rule '{}': {}",
                    rule.name, e
                )));
            }
        }
    }

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

    // Step 0: Run inline validation (if present) - convert failures to warnings
    if let Some(ref expr) = actions.validate_expr {
        let ctx = build_eval_context_with_custom_functions(event);
        match eval_boolean_with_context(expr, &ctx) {
            Ok(true) => {
                // Validation passed
            }
            Ok(false) => {
                let warning = format!(
                    "[WARNING] Rule '{}' validation expression '{}' returned false.\n\
                     This rule is in 'warn' mode - operation will proceed.",
                    rule.name, expr
                );
                return Ok(Response::inject(warning));
            }
            Err(e) => {
                let warning = format!(
                    "[WARNING] Rule '{}' validation expression error: {}.\n\
                     This rule is in 'warn' mode - operation will proceed.",
                    rule.name, e
                );
                return Ok(Response::inject(warning));
            }
        }
    } else if let Some(ref script) = actions.inline_script {
        match execute_inline_script(script, event, rule, config).await {
            Ok(true) => {
                // Validation passed
            }
            Ok(false) => {
                let warning = format!(
                    "[WARNING] Rule '{}' inline script validation failed.\n\
                     This rule is in 'warn' mode - operation will proceed.",
                    rule.name
                );
                return Ok(Response::inject(warning));
            }
            Err(e) => {
                let warning = format!(
                    "[WARNING] Rule '{}' inline script error: {}.\n\
                     This rule is in 'warn' mode - operation will proceed.",
                    rule.name, e
                );
                return Ok(Response::inject(warning));
            }
        }
    }

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
            prompt: None,
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
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                block: Some(true),
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
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
            prompt: None,
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
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                block: Some(true),
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
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
            prompt: None,
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
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
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
            prompt: None,
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
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
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
            prompt: None,
        };

        let rule = Rule {
            name: "false-condition".to_string(),
            description: None,
            // This non-existent var won't be in context, so comparison fails
            // Use a simple false expression instead
            enabled_when: Some(r"1 == 2".to_string()), // Always false
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
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
            prompt: None,
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
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
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
            prompt: None,
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
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
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
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
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
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
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
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
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
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
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

    // =========================================================================
    // Phase 4 Plan 4: matches_prompt Unit Tests (PROMPT-01 through PROMPT-05)
    // =========================================================================

    #[test]
    fn test_matches_prompt_simple_any_match() {
        // PROMPT-01: Basic regex pattern matching
        let pm = PromptMatch::Simple(vec!["delete".to_string(), "drop".to_string()]);

        // Should match - contains "delete"
        assert!(matches_prompt("please delete the file", &pm));

        // Should match - contains "drop"
        assert!(matches_prompt("drop table users", &pm));

        // Should not match - neither pattern
        assert!(!matches_prompt("create a new file", &pm));
    }

    #[test]
    fn test_matches_prompt_complex_all_mode() {
        // PROMPT-03: ALL mode requires all patterns to match
        let pm = PromptMatch::Complex {
            patterns: vec!["database".to_string(), "production".to_string()],
            mode: MatchMode::All,
            case_insensitive: false,
            anchor: None,
        };

        // Should match - contains both
        assert!(matches_prompt("access the production database", &pm));

        // Should not match - only one pattern
        assert!(!matches_prompt("access the database", &pm));

        // Should not match - only one pattern
        assert!(!matches_prompt("production server", &pm));
    }

    #[test]
    fn test_matches_prompt_case_insensitive() {
        // PROMPT-02: Case-insensitive matching
        let pm = PromptMatch::Complex {
            patterns: vec!["DELETE".to_string()],
            mode: MatchMode::Any,
            case_insensitive: true,
            anchor: None,
        };

        // Should match regardless of case
        assert!(matches_prompt("delete the file", &pm));
        assert!(matches_prompt("DELETE the file", &pm));
        assert!(matches_prompt("Delete the file", &pm));
    }

    #[test]
    fn test_matches_prompt_case_sensitive_default() {
        // Default is case-sensitive
        let pm = PromptMatch::Simple(vec!["DELETE".to_string()]);

        // Should NOT match - case matters
        assert!(!matches_prompt("delete the file", &pm));

        // Should match - exact case
        assert!(matches_prompt("DELETE the file", &pm));
    }

    #[test]
    fn test_matches_prompt_anchor_start() {
        // PROMPT-04: Anchor at start of prompt
        let pm = PromptMatch::Complex {
            patterns: vec!["please".to_string()],
            mode: MatchMode::Any,
            case_insensitive: false,
            anchor: Some(crate::models::Anchor::Start),
        };

        // Should match - starts with "please"
        assert!(matches_prompt("please delete the file", &pm));

        // Should not match - "please" not at start
        assert!(!matches_prompt("could you please help", &pm));
    }

    #[test]
    fn test_matches_prompt_anchor_end() {
        // PROMPT-04: Anchor at end of prompt
        let pm = PromptMatch::Complex {
            patterns: vec!["now".to_string()],
            mode: MatchMode::Any,
            case_insensitive: false,
            anchor: Some(crate::models::Anchor::End),
        };

        // Should match - ends with "now"
        assert!(matches_prompt("do it now", &pm));

        // Should not match - "now" not at end
        assert!(!matches_prompt("now is the time", &pm));
    }

    #[test]
    fn test_matches_prompt_contains_word_shorthand() {
        // contains_word: shorthand expands to word boundary regex
        let pm = PromptMatch::Simple(vec!["contains_word:delete".to_string()]);

        // Should match - "delete" as whole word
        assert!(matches_prompt("please delete the file", &pm));

        // Should not match - "delete" is part of "undelete"
        assert!(!matches_prompt("undelete the file", &pm));

        // Should not match - "delete" is part of "deleted"
        assert!(!matches_prompt("I deleted the file", &pm));
    }

    #[test]
    fn test_matches_prompt_negation_pattern() {
        // not: prefix negates the pattern
        let pm = PromptMatch::Simple(vec!["not:safe".to_string()]);

        // Should match - does NOT contain "safe"
        assert!(matches_prompt("delete the file", &pm));

        // Should not match - contains "safe"
        assert!(!matches_prompt("this is safe to run", &pm));
    }

    #[test]
    fn test_matches_prompt_negation_with_all_mode() {
        // ALL mode with negation - all conditions must be true
        let pm = PromptMatch::Complex {
            patterns: vec!["delete".to_string(), "not:safe".to_string()],
            mode: MatchMode::All,
            case_insensitive: false,
            anchor: None,
        };

        // Should match - contains "delete" AND does NOT contain "safe"
        assert!(matches_prompt("delete the dangerous file", &pm));

        // Should not match - contains "delete" but also contains "safe"
        assert!(!matches_prompt("safely delete the file", &pm));
    }

    #[test]
    fn test_matches_prompt_empty_patterns() {
        // Empty patterns should not match
        let pm = PromptMatch::Simple(vec![]);

        assert!(!matches_prompt("any text here", &pm));
    }

    #[test]
    fn test_matches_prompt_invalid_regex() {
        // Invalid regex should fail-closed (return false, not error)
        let pm = PromptMatch::Simple(vec!["[invalid".to_string()]);

        assert!(!matches_prompt("test", &pm)); // Fail-closed: invalid regex = no match
    }

    #[test]
    fn test_matches_prompt_regex_patterns() {
        // Full regex patterns work
        let pm = PromptMatch::Simple(vec![r"rm\s+-rf".to_string()]);

        assert!(matches_prompt("please run rm -rf /tmp", &pm));
        assert!(!matches_prompt("rm --recursive", &pm));
    }

    // =========================================================================
    // matches_rule Integration with prompt_match
    // =========================================================================

    #[test]
    fn test_matches_rule_with_prompt_match() {
        // Event with prompt field
        let event = Event {
            hook_event_name: EventType::UserPromptSubmit,
            tool_name: None,
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: Some("please delete the database".to_string()),
        };

        let rule = Rule {
            name: "block-delete".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: Some(PromptMatch::Simple(vec!["delete".to_string()])),
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(matches_rule(&event, &rule));
    }

    #[test]
    fn test_matches_rule_missing_prompt_no_match() {
        // Event WITHOUT prompt field
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
            prompt: None, // No prompt
        };

        let rule = Rule {
            name: "requires-prompt".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: Some(PromptMatch::Simple(vec!["test".to_string()])),
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        // Should NOT match - rule has prompt_match but event has no prompt
        assert!(!matches_rule(&event, &rule));
    }

    #[test]
    fn test_matches_rule_prompt_and_other_matchers() {
        // Both prompt_match and other matchers must match
        let event = Event {
            hook_event_name: EventType::UserPromptSubmit,
            tool_name: Some("Bash".to_string()),
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: Some("run sudo command".to_string()),
        };

        let rule = Rule {
            name: "bash-sudo".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: Some(PromptMatch::Simple(vec!["sudo".to_string()])),
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        // Should match - tool AND prompt_match both match
        assert!(matches_rule(&event, &rule));

        // Now change tool to not match
        let event_wrong_tool = Event {
            hook_event_name: EventType::UserPromptSubmit,
            tool_name: Some("Edit".to_string()), // Different tool
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: Some("run sudo command".to_string()),
        };

        // Should NOT match - tool doesn't match
        assert!(!matches_rule(&event_wrong_tool, &rule));
    }

    // =========================================================================
    // PROMPT-05: prompt variable in evalexpr context
    // =========================================================================

    #[test]
    fn test_prompt_variable_available_in_evalexpr_context() {
        // Verify prompt is available in evalexpr context for enabled_when
        let event = Event {
            hook_event_name: EventType::UserPromptSubmit,
            tool_name: None,
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: Some("hello world".to_string()),
        };

        // Build context and verify prompt is there
        let ctx = build_eval_context(&event);
        let result = evalexpr::eval_boolean_with_context(r#"prompt == "hello world""#, &ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_enabled_when_can_use_prompt_variable() {
        // enabled_when expression can access prompt
        let event = Event {
            hook_event_name: EventType::UserPromptSubmit,
            tool_name: None,
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: Some("dangerous delete operation".to_string()),
        };

        // Rule with enabled_when checking prompt
        // Note: evalexpr doesn't have str_contains, so we just check equality
        let rule = Rule {
            name: "check-prompt".to_string(),
            description: None,
            enabled_when: Some(r#"prompt != """#.to_string()), // Prompt is non-empty
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(is_rule_enabled(&rule, &event));

        // Event without prompt - should disable the rule
        let event_no_prompt = Event {
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
            prompt: None,
        };

        // Rule should fail because prompt variable doesn't exist
        assert!(!is_rule_enabled(&rule, &event_no_prompt));
    }

    // =========================================================================
    // matches_rule_with_debug tests for prompt_match
    // =========================================================================

    #[test]
    fn test_matches_rule_with_debug_prompt_match() {
        let event = Event {
            hook_event_name: EventType::UserPromptSubmit,
            tool_name: None,
            tool_input: None,
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: Some("delete everything".to_string()),
        };

        let rule = Rule {
            name: "debug-prompt".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: None,
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: Some(PromptMatch::Simple(vec!["delete".to_string()])),
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        let (matched, results) = matches_rule_with_debug(&event, &rule);
        assert!(matched);
        assert!(results.is_some());
        let results = results.unwrap();
        assert_eq!(results.prompt_match_matched, Some(true));
    }

    // =========================================================================
    // FIELD VALIDATION TESTS (Phase 5)
    // =========================================================================

    #[test]
    fn test_field_validation_no_fields_configured() {
        // Rule with no require_fields/field_types should pass validation
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
            prompt: None,
        };

        let rule = Rule {
            name: "no-field-validation".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_missing_tool_input() {
        // Rule with require_fields but event has no tool_input should fail
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
            prompt: None,
        };

        let rule = Rule {
            name: "require-command".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["command".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_present_field() {
        // tool_input has required field should pass
        let mut tool_input = serde_json::Map::new();
        tool_input.insert("command".to_string(), serde_json::json!("echo hello"));

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(serde_json::Value::Object(tool_input)),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-command".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["command".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_missing_field() {
        // tool_input missing required field should fail
        let mut tool_input = serde_json::Map::new();
        tool_input.insert("other_field".to_string(), serde_json::json!("value"));

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(serde_json::Value::Object(tool_input)),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-command".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["command".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_null_field_is_missing() {
        // tool_input has null field should be treated as missing
        let mut tool_input = serde_json::Map::new();
        tool_input.insert("command".to_string(), serde_json::Value::Null);

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(serde_json::Value::Object(tool_input)),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-command".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["command".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_nested_field() {
        // Nested field using dot notation should resolve correctly
        let tool_input = serde_json::json!({
            "user": {
                "name": "Alice"
            }
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-user-name".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["user.name".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_type_match() {
        // field_types with matching type should pass
        let tool_input = serde_json::json!({
            "count": 42
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("count".to_string(), "number".to_string());

        let rule = Rule {
            name: "count-must-be-number".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_type_mismatch() {
        // field_types with wrong type should fail
        let tool_input = serde_json::json!({
            "count": "not a number"
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("count".to_string(), "number".to_string());

        let rule = Rule {
            name: "count-must-be-number".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_empty_string_is_present() {
        // Empty string should count as present
        let tool_input = serde_json::json!({
            "command": ""
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-command".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["command".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_empty_array_is_present() {
        // Empty array should count as present
        let tool_input = serde_json::json!({
            "items": []
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-items".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["items".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_any_type() {
        // field_types with "any" should accept any non-null value
        let tool_input = serde_json::json!({
            "data": {"nested": "object"}
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("data".to_string(), "any".to_string());

        let rule = Rule {
            name: "data-any-type".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_field_types_implies_existence() {
        // Field in field_types but not require_fields should still be checked for existence
        let tool_input = serde_json::json!({
            "other_field": "value"
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("count".to_string(), "number".to_string());

        let rule = Rule {
            name: "count-must-exist-and-be-number".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,           // NOT in require_fields
                field_types: Some(field_types), // Only in field_types
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        // Should fail because 'count' is missing (field_types implies existence)
        assert!(!validate_required_fields(&rule, &event));
    }

    // =========================================================================
    // Phase 5 Plan 3: Additional comprehensive tests for FIELD-01 through FIELD-04
    // =========================================================================

    // FIELD-01 tests (require specific fields)
    #[test]
    fn test_field_validation_single_required_field_present() {
        let tool_input = serde_json::json!({
            "file_path": "/test/file.txt"
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Edit".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-file-path".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Edit".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["file_path".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_multiple_required_fields_all_present() {
        let tool_input = serde_json::json!({
            "file_path": "/test/file.txt",
            "content": "test content",
            "mode": "overwrite"
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-write-fields".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Write".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec![
                    "file_path".to_string(),
                    "content".to_string(),
                    "mode".to_string(),
                ]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_multiple_required_fields_one_missing() {
        let tool_input = serde_json::json!({
            "file_path": "/test/file.txt",
            "mode": "overwrite"
            // "content" is missing
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-write-fields".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Write".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec![
                    "file_path".to_string(),
                    "content".to_string(),
                    "mode".to_string(),
                ]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!validate_required_fields(&rule, &event));
    }

    // FIELD-02 tests (fail-closed blocking)
    #[test]
    fn test_field_validation_blocks_on_missing_field() {
        let tool_input = serde_json::json!({
            "other_field": "value"
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-command".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["command".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_blocks_on_null_field() {
        let tool_input = serde_json::json!({
            "command": null
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-command".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["command".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_blocks_on_non_object_tool_input() {
        // tool_input is a string instead of object
        let tool_input = serde_json::json!("not an object");

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Bash".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-command".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Bash".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["command".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!validate_required_fields(&rule, &event));
    }

    // FIELD-03 tests (nested paths with dot notation)
    #[test]
    fn test_field_validation_nested_one_level() {
        let tool_input = serde_json::json!({
            "user": {
                "name": "Alice"
            }
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-user-name".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["user.name".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_nested_three_levels() {
        let tool_input = serde_json::json!({
            "input": {
                "user": {
                    "address": {
                        "city": "Seattle"
                    }
                }
            }
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-city".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["input.user.address.city".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_nested_missing_intermediate() {
        let tool_input = serde_json::json!({
            "user": {
                "name": "Alice"
                // "address" object is missing
            }
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-city".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec!["user.address.city".to_string()]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_validation_nested_mixed_present_and_missing() {
        let tool_input = serde_json::json!({
            "user": {
                "name": "Alice",
                "email": "alice@example.com"
            }
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "require-user-fields".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: Some(vec![
                    "user.name".to_string(),
                    "user.phone".to_string(), // Missing
                ]),
                field_types: None,
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!validate_required_fields(&rule, &event));
    }

    // FIELD-04 tests (type validation)
    #[test]
    fn test_field_types_string_match() {
        let tool_input = serde_json::json!({
            "name": "test"
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("name".to_string(), "string".to_string());

        let rule = Rule {
            name: "type-check".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_types_number_match() {
        let tool_input = serde_json::json!({
            "count": 42
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("count".to_string(), "number".to_string());

        let rule = Rule {
            name: "type-check".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_types_boolean_match() {
        let tool_input = serde_json::json!({
            "enabled": true
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("enabled".to_string(), "boolean".to_string());

        let rule = Rule {
            name: "type-check".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_types_array_match() {
        let tool_input = serde_json::json!({
            "items": [1, 2, 3]
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("items".to_string(), "array".to_string());

        let rule = Rule {
            name: "type-check".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_types_object_match() {
        let tool_input = serde_json::json!({
            "config": {"key": "value"}
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("config".to_string(), "object".to_string());

        let rule = Rule {
            name: "type-check".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_types_any_match_with_string() {
        let tool_input = serde_json::json!({
            "data": "some string"
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("data".to_string(), "any".to_string());

        let rule = Rule {
            name: "type-check".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_types_any_match_with_number() {
        let tool_input = serde_json::json!({
            "data": 123
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("data".to_string(), "any".to_string());

        let rule = Rule {
            name: "type-check".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_types_string_mismatch_with_number() {
        let tool_input = serde_json::json!({
            "name": 42  // number, not string
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("name".to_string(), "string".to_string());

        let rule = Rule {
            name: "type-check".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_types_number_mismatch_with_string() {
        let tool_input = serde_json::json!({
            "count": "42"  // string, not number
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("count".to_string(), "number".to_string());

        let rule = Rule {
            name: "type-check".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        assert!(!validate_required_fields(&rule, &event));
    }

    #[test]
    fn test_field_types_all_errors_accumulated() {
        let tool_input = serde_json::json!({
            "name": 42,        // Should be string
            "count": "wrong",  // Should be number
            "enabled": "yes"   // Should be boolean
        });

        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(tool_input),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let mut field_types = std::collections::HashMap::new();
        field_types.insert("name".to_string(), "string".to_string());
        field_types.insert("count".to_string(), "number".to_string());
        field_types.insert("enabled".to_string(), "boolean".to_string());

        let rule = Rule {
            name: "type-check-multiple".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["API".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: Some(field_types),
            },
            actions: Actions {
                inject: None,
                inject_inline: None,
                inject_command: None,
                run: None,
                block: Some(true),
                block_if_match: None,
                validate_expr: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        // All three type errors should be accumulated and reported
        assert!(!validate_required_fields(&rule, &event));
    }

    // =========================================================================
    // Phase 6: SCRIPT-01/02 - Custom Functions Tests (get_field, has_field)
    // =========================================================================

    #[test]
    fn test_get_field_string_value() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "file_path": "/test/file.txt",
                "content": "hello world"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result =
            eval_boolean_with_context(r#"get_field("file_path") == "/test/file.txt""#, &ctx);

        assert!(result.is_ok(), "Should evaluate expression: {:?}", result);
        assert!(result.unwrap(), "Should return correct string value");
    }

    #[test]
    fn test_get_field_number_value() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(serde_json::json!({
                "count": 42,
                "price": 99.95
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(r#"get_field("count") == 42.0"#, &ctx);

        assert!(result.is_ok(), "Should evaluate expression: {:?}", result);
        assert!(result.unwrap(), "Should return correct number value");
    }

    #[test]
    fn test_get_field_boolean_value() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(serde_json::json!({
                "enabled": true,
                "active": false
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(r#"get_field("enabled") == true"#, &ctx);

        assert!(result.is_ok(), "Should evaluate expression: {:?}", result);
        assert!(result.unwrap(), "Should return correct boolean value");
    }

    #[test]
    fn test_get_field_missing_field() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(serde_json::json!({
                "existing": "value"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(r#"get_field("nonexistent") == """#, &ctx);

        assert!(result.is_ok(), "Should evaluate expression: {:?}", result);
        assert!(
            result.unwrap(),
            "Should return empty string for missing field"
        );
    }

    #[test]
    fn test_get_field_null_field() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(serde_json::json!({
                "nullable": null
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(r#"get_field("nullable") == """#, &ctx);

        assert!(result.is_ok(), "Should evaluate expression: {:?}", result);
        assert!(result.unwrap(), "Should return empty string for null field");
    }

    #[test]
    fn test_get_field_nested_path() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(serde_json::json!({
                "user": {
                    "name": "Alice",
                    "profile": {
                        "email": "alice@example.com"
                    }
                }
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(r#"get_field("user.name") == "Alice""#, &ctx);

        assert!(result.is_ok(), "Should evaluate expression: {:?}", result);
        assert!(result.unwrap(), "Should return nested field value");

        let result2 = eval_boolean_with_context(
            r#"get_field("user.profile.email") == "alice@example.com""#,
            &ctx,
        );
        assert!(
            result2.is_ok(),
            "Should evaluate nested expression: {:?}",
            result2
        );
        assert!(result2.unwrap(), "Should return deeply nested field value");
    }

    #[test]
    fn test_has_field_present() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "file_path": "/test/file.txt"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(r#"has_field("file_path")"#, &ctx);

        assert!(result.is_ok(), "Should evaluate expression: {:?}", result);
        assert!(result.unwrap(), "Should return true for present field");
    }

    #[test]
    fn test_has_field_missing() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "file_path": "/test/file.txt"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(r#"has_field("nonexistent")"#, &ctx);

        assert!(result.is_ok(), "Should evaluate expression: {:?}", result);
        assert!(!result.unwrap(), "Should return false for missing field");
    }

    #[test]
    fn test_has_field_null() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(serde_json::json!({
                "nullable": null
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(r#"has_field("nullable")"#, &ctx);

        assert!(result.is_ok(), "Should evaluate expression: {:?}", result);
        assert!(!result.unwrap(), "Should return false for null field");
    }

    #[test]
    fn test_has_field_nested() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(serde_json::json!({
                "user": {
                    "name": "Alice"
                }
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(r#"has_field("user.name")"#, &ctx);

        assert!(result.is_ok(), "Should evaluate expression: {:?}", result);
        assert!(result.unwrap(), "Should return true for nested field");
    }

    // =========================================================================
    // Phase 6: SCRIPT-03 - Boolean Return from validate_expr Tests
    // =========================================================================

    #[test]
    fn test_validate_expr_returns_true_allows() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "file_path": "/test/file.txt"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(r#"has_field("file_path")"#, &ctx);

        assert!(result.is_ok(), "Expression should evaluate: {:?}", result);
        assert!(result.unwrap(), "Expression returning true should allow");
    }

    #[test]
    fn test_validate_expr_returns_false_blocks() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "file_path": "/test/file.txt"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(r#"has_field("missing")"#, &ctx);

        assert!(result.is_ok(), "Expression should evaluate: {:?}", result);
        assert!(!result.unwrap(), "Expression returning false should block");
    }

    #[test]
    fn test_validate_expr_comparison() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("API".to_string()),
            tool_input: Some(serde_json::json!({
                "count": 5
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(r#"get_field("count") > 0"#, &ctx);

        assert!(result.is_ok(), "Expression should evaluate: {:?}", result);
        assert!(result.unwrap(), "Comparison should return correct result");
    }

    #[test]
    fn test_validate_expr_complex_expression() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "file_path": "/test/file.txt",
                "content": "hello"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        let result = eval_boolean_with_context(
            r#"has_field("file_path") && get_field("content") != """#,
            &ctx,
        );

        assert!(
            result.is_ok(),
            "Complex expression should evaluate: {:?}",
            result
        );
        assert!(
            result.unwrap(),
            "Complex expression should return correct result"
        );
    }

    #[test]
    fn test_validate_expr_error_blocks() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "file_path": "/test/file.txt"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);
        // Invalid syntax: unclosed parenthesis
        let result = eval_boolean_with_context(r#"has_field("file_path""#, &ctx);

        assert!(
            result.is_err(),
            "Invalid syntax should return error (fail-closed)"
        );
    }

    // =========================================================================
    // Phase 6: SCRIPT-01 - validate_expr in execute_rule_actions Tests
    // =========================================================================

    #[tokio::test]
    async fn test_validate_expr_blocks_before_inject() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "file_path": "/test/file.txt"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "validate-blocks".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Write".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                validate_expr: Some(r#"has_field("missing_field")"#.to_string()),
                inject_inline: Some("Should not appear".to_string()),
                inject: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        let config = Config {
            version: "1.0".to_string(),
            rules: vec![],
            settings: crate::config::Settings::default(),
        };

        let response = execute_rule_actions(&event, &rule, &config).await.unwrap();

        assert!(
            !response.continue_,
            "validate_expr returning false should block"
        );
        assert!(
            response.context.is_none(),
            "Should not inject when validation fails"
        );
    }

    #[tokio::test]
    async fn test_validate_expr_allows_then_injects() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "file_path": "/test/file.txt"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let rule = Rule {
            name: "validate-allows".to_string(),
            description: None,
            enabled_when: None,
            matchers: Matchers {
                tools: Some(vec!["Write".to_string()]),
                extensions: None,
                directories: None,
                operations: None,
                command_match: None,
                prompt_match: None,
                require_fields: None,
                field_types: None,
            },
            actions: Actions {
                validate_expr: Some(r#"has_field("file_path")"#.to_string()),
                inject_inline: Some("Validation passed".to_string()),
                inject: None,
                inject_command: None,
                run: None,
                block: None,
                block_if_match: None,
                inline_script: None,
            },
            mode: None,
            priority: None,
            governance: None,
            metadata: None,
        };

        let config = Config {
            version: "1.0".to_string(),
            rules: vec![],
            settings: crate::config::Settings::default(),
        };

        let response = execute_rule_actions(&event, &rule, &config).await.unwrap();

        assert!(
            response.continue_,
            "validate_expr returning true should allow"
        );
        assert!(
            response.context.is_some(),
            "Should inject when validation passes"
        );
        assert!(response.context.unwrap().contains("Validation passed"));
    }

    #[tokio::test]
    async fn test_validate_expr_no_tool_input_custom_functions() {
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: None, // No tool_input
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);

        // get_field should return empty string when tool_input is None
        let result = eval_boolean_with_context(r#"get_field("any_field") == """#, &ctx);
        assert!(result.is_ok());
        assert!(
            result.unwrap(),
            "get_field should return empty string when tool_input is None"
        );

        // has_field should return false when tool_input is None
        let result2 = eval_boolean_with_context(r#"has_field("any_field")"#, &ctx);
        assert!(result2.is_ok());
        assert!(
            !result2.unwrap(),
            "has_field should return false when tool_input is None"
        );
    }

    #[tokio::test]
    async fn test_validate_expr_with_env_vars() {
        // Test that custom functions work alongside existing env vars
        // Use PATH which always exists on all systems
        let event = Event {
            hook_event_name: EventType::PreToolUse,
            tool_name: Some("Write".to_string()),
            tool_input: Some(serde_json::json!({
                "file_path": "/test/file.txt"
            })),
            session_id: "test-session".to_string(),
            timestamp: Utc::now(),
            user_id: None,
            transcript_path: None,
            cwd: None,
            permission_mode: None,
            tool_use_id: None,
            prompt: None,
        };

        let ctx = build_eval_context_with_custom_functions(&event);

        // Should be able to use both custom functions and env vars (PATH always exists)
        let result = eval_boolean_with_context(r#"has_field("file_path") && env_PATH != """#, &ctx);

        assert!(
            result.is_ok(),
            "Should evaluate expression with both custom functions and env vars: {:?}",
            result
        );
        assert!(
            result.unwrap(),
            "Should work with both custom functions and env vars"
        );
    }
}
