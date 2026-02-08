# Architecture Integration: v1.3 Features

**Domain:** RuleZ AI Policy Engine
**Researched:** 2026-02-08
**Confidence:** HIGH

## Executive Summary

v1.3 adds three new features to the existing RuleZ matching and action pipeline:

1. **prompt_match** (Matcher) - Match rules against user prompt text from UserPromptSubmit events
2. **require_fields** (Action) - Validate required fields exist in tool_input before allowing operations
3. **Inline script blocks** (Action) - Write validator scripts directly in YAML using HEREDOC syntax

All three features integrate cleanly into the existing architecture with **minimal structural changes**. The architecture already supports the patterns needed:

- Matchers pipeline is extensible (add `prompt_match` alongside `command_match`)
- Actions execute sequentially with well-defined precedence
- `tool_input` is already parsed as `serde_json::Value` for flexible field access

## Current Architecture Overview

### Component Structure

```
Event (from Claude Code)
  ↓
hooks.rs::process_event()
  ↓
config.rs::Config::load() ────→ YAML parsing
  ↓                              serde_yaml
config::enabled_rules() ────────→ Priority sorting
  ↓                              Phase 2 feature
hooks.rs::evaluate_rules()
  ↓
For each rule:
  1. is_rule_enabled() ────────→ enabled_when (Phase 3)
  2. matches_rule() ────────────→ Matchers pipeline
  3. execute_rule_actions() ───→ Actions pipeline
  ↓
Response (to Claude Code)
```

### Existing Matchers Pipeline (models.rs)

```rust
pub struct Matchers {
    pub tools: Option<Vec<String>>,           // Tool name matching
    pub extensions: Option<Vec<String>>,      // File extension matching
    pub directories: Option<Vec<String>>,     // Directory glob matching
    pub operations: Option<Vec<String>>,      // Event type matching
    pub command_match: Option<String>,        // Regex on Bash commands
}
```

**Match evaluation:** ALL defined matchers must match (AND logic). Missing matchers are ignored.

**Location:** `hooks.rs::matches_rule()` (lines 230-303)

### Existing Actions Pipeline (models.rs)

```rust
pub struct Actions {
    pub inject: Option<String>,              // File-based injection
    pub inject_inline: Option<String>,       // Inline markdown (Phase 1)
    pub inject_command: Option<String>,      // Command output (Phase 2)
    pub run: Option<RunAction>,              // Validator script
    pub block: Option<bool>,                 // Unconditional block
    pub block_if_match: Option<String>,      // Conditional block
}
```

**Execution order (hooks.rs::execute_rule_actions, lines 483-560):**

1. **Block checks** (lines 488-515)
   - `block: true` → immediate block
   - `block_if_match` → regex on content → block if match
2. **Injection checks** (lines 517-541)
   - `inject_inline` → return immediately (precedence)
   - `inject_command` → execute shell, return output
   - `inject` → read file, return content
3. **Validator execution** (lines 543-557)
   - `run` → execute script with stdin, return response

**Key insight:** Actions execute in order until one returns. First matching action wins.

## Integration Points

### 1. prompt_match (Matcher)

**What it does:** Match rules against user prompt text from `UserPromptSubmit` events.

**Where it integrates:** `models.rs::Matchers` struct + `hooks.rs::matches_rule()`

**Event structure (from Claude Code):**

```json
{
  "hook_event_name": "UserPromptSubmit",
  "tool_name": null,
  "tool_input": {
    "prompt_text": "Can you refactor the authentication module?"
  },
  "session_id": "abc123",
  "timestamp": "2026-02-08T10:00:00Z"
}
```

**Integration architecture:**

```rust
// models.rs (add to Matchers)
pub struct Matchers {
    pub tools: Option<Vec<String>>,
    pub extensions: Option<Vec<String>>,
    pub directories: Option<Vec<String>>,
    pub operations: Option<Vec<String>>,
    pub command_match: Option<String>,
    pub prompt_match: Option<String>,  // NEW: Regex pattern for prompt text
}
```

**Evaluation logic (hooks.rs::matches_rule):**

```rust
// Add after command_match check (around line 256)
if let Some(ref pattern) = matchers.prompt_match {
    if let Some(ref tool_input) = event.tool_input {
        if let Some(prompt_text) = tool_input.get("prompt_text").and_then(|p| p.as_str()) {
            if let Ok(regex) = Regex::new(pattern) {
                if !regex.is_match(prompt_text) {
                    return false;
                }
            }
        }
    }
}
```

**Dependencies:**
- Existing `regex` crate (already in use for `command_match`)
- Existing `tool_input` JSON parsing (already handles arbitrary fields)
- Existing matcher AND logic (all matchers must match)

**No new components needed.** Follows exact pattern of `command_match`.

---

### 2. require_fields (Action)

**What it does:** Validate that required fields exist in `tool_input` before allowing operation.

**Where it integrates:** `models.rs::Actions` struct + `hooks.rs::execute_rule_actions()`

**Use case example:**

```yaml
rules:
  - name: validate-write-params
    matchers:
      tools: [Write]
    actions:
      require_fields: [file_path, content]  # Both must exist
```

**Integration architecture:**

```rust
// models.rs (add to Actions)
pub struct Actions {
    pub inject: Option<String>,
    pub inject_inline: Option<String>,
    pub inject_command: Option<String>,
    pub run: Option<RunAction>,
    pub block: Option<bool>,
    pub block_if_match: Option<String>,
    pub require_fields: Option<Vec<String>>,  // NEW: Required field names
}
```

**Validation logic (hooks.rs::execute_rule_actions):**

```rust
// Add BEFORE block checks (lines 484-486)
// Validation happens first - missing fields block immediately
if let Some(ref required_fields) = actions.require_fields {
    if let Some(ref tool_input) = event.tool_input {
        for field in required_fields {
            if !tool_input.get(field).is_some() {
                return Ok(Response::block(format!(
                    "Required field '{}' missing (rule '{}')",
                    field, rule.name
                )));
            }
        }
    } else {
        // No tool_input but fields required = block
        return Ok(Response::block(format!(
            "Required fields {:?} missing (no tool_input, rule '{}')",
            required_fields, rule.name
        )));
    }
}
```

**Execution order implications:**

Current order:
1. Block checks → 2. Injection → 3. Validators

**New order with require_fields:**
1. **Field validation (NEW)** → 2. Block checks → 3. Injection → 4. Validators

**Rationale:** Field validation is a precondition check. If required fields are missing, no other actions should execute.

**Dependencies:**
- Existing `tool_input: Option<serde_json::Value>` (already supports arbitrary field access)
- Existing `Response::block()` (already used for blocking)

**No new components needed.** Pure validation logic using existing structures.

---

### 3. Inline Script Blocks (Action)

**What it does:** Write validator scripts directly in YAML using HEREDOC syntax instead of separate files.

**Where it integrates:** `models.rs::RunAction` enum + `hooks.rs::execute_validator_script()`

**Use case example:**

```yaml
rules:
  - name: validate-json
    matchers:
      tools: [Write]
      extensions: [.json]
    actions:
      run:
        inline: |
          #!/usr/bin/env python3
          import json, sys
          event = json.load(sys.stdin)
          content = event['tool_input'].get('content', '')
          try:
              json.loads(content)
              sys.exit(0)  # Valid JSON
          except:
              print("Invalid JSON", file=sys.stderr)
              sys.exit(1)  # Block
        interpreter: python3  # Optional, default: detect from shebang
```

**Integration architecture:**

```rust
// models.rs (extend RunAction enum)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RunAction {
    Simple(String),              // Existing: "path/to/script.py"
    Extended {                   // Existing: { script: "...", trust: ... }
        script: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        trust: Option<TrustLevel>,
    },
    Inline {                     // NEW: Inline script with interpreter
        inline: String,          // Script text (HEREDOC in YAML)
        #[serde(skip_serializing_if = "Option::is_none")]
        interpreter: Option<String>,  // "python3", "bash", "node", etc.
    },
}

impl RunAction {
    pub fn script_path(&self) -> Option<&str> {
        match self {
            RunAction::Simple(path) => Some(path),
            RunAction::Extended { script, .. } => Some(script),
            RunAction::Inline { .. } => None,  // No path for inline
        }
    }

    pub fn is_inline(&self) -> bool {
        matches!(self, RunAction::Inline { .. })
    }

    pub fn inline_script(&self) -> Option<(&str, Option<&str>)> {
        match self {
            RunAction::Inline { inline, interpreter } => {
                Some((inline.as_str(), interpreter.as_deref()))
            }
            _ => None,
        }
    }
}
```

**Execution logic (hooks.rs::execute_validator_script):**

Current function assumes script is a file path. Need to add inline handling:

```rust
// hooks.rs - new helper function (add before execute_validator_script)
async fn execute_inline_script(
    event: &Event,
    script_text: &str,
    interpreter: Option<&str>,
    rule: &Rule,
    config: &Config,
) -> Result<Response> {
    let timeout_duration = rule
        .metadata
        .as_ref()
        .map(|m| m.timeout)
        .unwrap_or(config.settings.script_timeout);

    // Detect interpreter from shebang if not specified
    let detected_interpreter = interpreter.or_else(|| {
        script_text.lines().next()
            .and_then(|line| {
                if line.starts_with("#!") {
                    line.trim_start_matches("#!").trim().split_whitespace().next()
                } else {
                    None
                }
            })
    }).unwrap_or("sh");  // Default to sh

    // Write script to temp file (needed for execution)
    let temp_dir = std::env::temp_dir();
    let script_id = uuid::Uuid::new_v4().to_string();
    let temp_script = temp_dir.join(format!("rulez-inline-{}.sh", script_id));
    tokio::fs::write(&temp_script, script_text).await?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = tokio::fs::metadata(&temp_script).await?.permissions();
        perms.set_mode(0o755);
        tokio::fs::set_permissions(&temp_script, perms).await?;
    }

    // Execute via interpreter
    let mut command = Command::new(detected_interpreter);
    command.arg(&temp_script);
    command.stdin(std::process::Stdio::piped());
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());

    // Rest is same as execute_validator_script (spawn, send stdin, wait, parse response)
    // ... (reuse existing logic from lines 586-656)

    // Cleanup temp file
    let _ = tokio::fs::remove_file(&temp_script).await;

    result
}
```

**Modified execute_rule_actions (hooks.rs, around line 543):**

```rust
// Handle script execution
if let Some(ref run_action) = actions.run {
    if let Some((script_text, interpreter)) = run_action.inline_script() {
        // NEW: Inline script execution
        match execute_inline_script(event, script_text, interpreter, rule, config).await {
            Ok(script_response) => return Ok(script_response),
            Err(e) => {
                tracing::warn!("Inline script execution failed for rule '{}': {}", rule.name, e);
                if !config.settings.fail_open {
                    return Err(e);
                }
            }
        }
    } else if let Some(script_path) = run_action.script_path() {
        // EXISTING: File-based script execution
        match execute_validator_script(event, script_path, rule, config).await {
            Ok(script_response) => return Ok(script_response),
            Err(e) => {
                tracing::warn!("Script execution failed for rule '{}': {}", rule.name, e);
                if !config.settings.fail_open {
                    return Err(e);
                }
            }
        }
    }
}
```

**Dependencies:**
- Existing `tokio::fs` (already in use)
- Existing `uuid` crate (already in `Cargo.toml` for session IDs)
- Existing script execution infrastructure (stdin piping, timeout, response parsing)

**New component:** `execute_inline_script()` function (reuses 90% of existing script execution logic)

**Security considerations:**
- Temp files written to system temp directory (standard practice)
- File permissions set to 0o755 on Unix (executable by owner)
- Temp files cleaned up after execution
- Same timeout and fail_open semantics as file-based scripts
- No trust level for inline scripts (considered "local" since defined in YAML config)

---

## Data Flow Changes

### Current Flow (v1.2)

```
1. Event arrives → hooks.rs::process_event()
2. Load config → config.rs::Config::load()
3. Get enabled rules → config.enabled_rules() [sorted by priority]
4. For each rule:
   a. Check enabled_when → is_rule_enabled()
   b. Check matchers → matches_rule()
      - tools, extensions, directories, operations, command_match
   c. Execute actions → execute_rule_actions()
      - block/block_if_match
      - inject_inline > inject_command > inject
      - run (validator script)
5. Merge responses → Response
6. Log audit entry → logging.rs
7. Return response to Claude Code
```

### New Flow (v1.3)

```
1. Event arrives → hooks.rs::process_event()
2. Load config → config.rs::Config::load()
3. Get enabled rules → config.enabled_rules() [sorted by priority]
4. For each rule:
   a. Check enabled_when → is_rule_enabled()
   b. Check matchers → matches_rule()
      - tools, extensions, directories, operations
      - command_match
      - prompt_match [NEW: matches UserPromptSubmit events]
   c. Execute actions → execute_rule_actions()
      - require_fields [NEW: validate tool_input fields FIRST]
      - block/block_if_match
      - inject_inline > inject_command > inject
      - run (validator script OR inline script) [NEW: inline variant]
5. Merge responses → Response
6. Log audit entry → logging.rs
7. Return response to Claude Code
```

**Changes highlighted in [NEW] markers.**

---

## Component Boundaries

### Modified Components

| Component | File | Change Type | Description |
|-----------|------|-------------|-------------|
| **Matchers** | `models.rs` | **Extend struct** | Add `prompt_match: Option<String>` field |
| **Actions** | `models.rs` | **Extend struct** | Add `require_fields: Option<Vec<String>>` field |
| **RunAction** | `models.rs` | **Extend enum** | Add `Inline { inline: String, interpreter: Option<String> }` variant |
| **matches_rule()** | `hooks.rs` | **Add logic** | Add prompt_match evaluation (follows command_match pattern) |
| **execute_rule_actions()** | `hooks.rs` | **Add logic** | Add require_fields validation (before block checks) + inline script dispatch |

### New Components

| Component | File | Type | Purpose |
|-----------|------|------|---------|
| **execute_inline_script()** | `hooks.rs` | Function | Execute inline YAML scripts via temp files (reuses execute_validator_script logic) |

### Unchanged Components

| Component | File | Why Unchanged |
|-----------|------|---------------|
| **Config loading** | `config.rs` | YAML parsing handles new fields automatically (serde) |
| **Rule validation** | `config.rs::validate()` | New fields are optional, no validation needed |
| **Logging** | `logging.rs` | Logs actions regardless of implementation |
| **Response** | `models.rs` | No new response types needed |
| **Event** | `models.rs` | Already supports arbitrary tool_input fields |

---

## Suggested Build Order

### Rationale for Order

1. **Start with simplest** (prompt_match) to validate integration patterns
2. **Build on existing patterns** (require_fields extends validation)
3. **Most complex last** (inline scripts require temp file handling)

### Phase 4: prompt_match (Matcher)

**Why first:**
- Cleanest integration (mirrors command_match exactly)
- No new execution logic, just matcher evaluation
- Validates that Matchers pipeline is extensible as expected

**Build steps:**
1. Add `prompt_match: Option<String>` to `models.rs::Matchers`
2. Add evaluation logic to `hooks.rs::matches_rule()` (after line 256)
3. Add evaluation logic to `hooks.rs::matches_rule_with_debug()` (for debug mode)
4. Write integration tests (UserPromptSubmit event with prompt_match rule)

**Estimated complexity:** Low
**Lines of code:** ~30-40 (mostly test code)

---

### Phase 5: require_fields (Action)

**Why second:**
- Extends Actions pipeline (validates extensibility)
- No external dependencies (pure validation)
- Tests action execution order (must execute before blocks)

**Build steps:**
1. Add `require_fields: Option<Vec<String>>` to `models.rs::Actions`
2. Add validation logic to `hooks.rs::execute_rule_actions()` (before line 488)
3. Add same logic to `hooks.rs::execute_rule_actions_warn_mode()` (warn mode support)
4. Write integration tests (various tool_input scenarios, missing fields, warn mode)

**Estimated complexity:** Low
**Lines of code:** ~50-60 (validation + tests)

---

### Phase 6: Inline Script Blocks (Action)

**Why last:**
- Most complex (temp file management, interpreter detection)
- Extends RunAction enum (validates enum extensibility)
- Builds on existing script execution infrastructure

**Build steps:**
1. Add `Inline` variant to `models.rs::RunAction` enum
2. Add helper methods to `RunAction` impl (`is_inline()`, `inline_script()`)
3. Implement `hooks.rs::execute_inline_script()` (new function)
4. Modify `hooks.rs::execute_rule_actions()` to dispatch inline vs file scripts
5. Add temp file cleanup logic (even on errors)
6. Write integration tests (Python script, Bash script, shebang detection, cleanup)
7. Add security tests (verify temp file permissions, isolation)

**Estimated complexity:** Medium-High
**Lines of code:** ~150-200 (implementation + extensive tests)

**Dependencies:**
- `uuid` crate (already in project, used for temp file names)
- `tokio::fs` (already in project)

---

## Architecture Patterns Validated

### 1. Matcher Extensibility ✅

**Pattern:** Add new matcher fields to `Matchers` struct, extend `matches_rule()` with new evaluation logic.

**Evidence:**
- `command_match` added in Phase 1 (baseline release)
- `prompt_match` follows identical pattern
- AND logic works regardless of matcher count

**Confidence:** HIGH - Pattern proven by existing code.

---

### 2. Action Extensibility ✅

**Pattern:** Add new action fields to `Actions` struct, extend `execute_rule_actions()` with new logic respecting execution order.

**Evidence:**
- `inject_inline` added in v1.2 Phase 1 (recently shipped)
- `inject_command` added in v1.2 Phase 2 (recently shipped)
- Execution order is explicit and maintained
- `require_fields` and inline scripts fit into existing order

**Confidence:** HIGH - Pattern proven by recent additions.

---

### 3. RunAction Enum Extensibility ✅

**Pattern:** Add new variants to `RunAction` enum using `#[serde(untagged)]` for YAML flexibility.

**Evidence:**
- `Simple(String)` vs `Extended { script, trust }` already exists (Phase 2.4)
- `#[serde(untagged)]` allows YAML to use either string or object syntax
- Adding `Inline { inline, interpreter }` follows same pattern

**Confidence:** HIGH - Enum already has two variants, tested in production.

---

### 4. Event Flexibility ✅

**Pattern:** `event.tool_input: Option<serde_json::Value>` supports arbitrary field access.

**Evidence:**
- Already used for `command`, `filePath`, `content`, `newString`, `pattern`, `path`, etc.
- No schema enforcement - any field can be accessed
- `prompt_text` for UserPromptSubmit fits naturally

**Confidence:** HIGH - Architecture designed for extensibility.

---

## Integration Risks and Mitigations

### Risk 1: prompt_match on Non-UserPromptSubmit Events

**Issue:** If a rule has `prompt_match`, but the event is not `UserPromptSubmit`, matcher will fail (no `prompt_text` field).

**Mitigation:**
- Matches existing behavior (command_match fails on non-Bash events)
- Document best practice: Combine `prompt_match` with `operations: [UserPromptSubmit]`

**Example:**

```yaml
# GOOD: Explicit event type filtering
matchers:
  operations: [UserPromptSubmit]
  prompt_match: "refactor|optimize"

# BAD: Will match UserPromptSubmit but fail on PreToolUse
matchers:
  prompt_match: "refactor"
```

**Code:** No special handling needed. Existing matcher AND logic handles this correctly.

---

### Risk 2: require_fields Execution Order

**Issue:** If `require_fields` doesn't execute first, other actions might run even though fields are missing.

**Mitigation:**
- **Place validation before all other actions** (line 484, before blocks)
- Test execution order explicitly
- Document in code comments

**Test case:**

```rust
// Test that require_fields blocks before inject_inline runs
let rule = Rule {
    actions: Actions {
        require_fields: Some(vec!["file_path".to_string()]),
        inject_inline: Some("Should not see this".to_string()),
        ..
    },
    ..
};
let event = Event {
    tool_input: None,  // No fields
    ..
};
let response = execute_rule_actions(&event, &rule, &config).await?;
assert!(!response.continue_);  // Blocked
assert!(response.context.is_none());  // No injection happened
```

---

### Risk 3: Inline Script Temp File Cleanup

**Issue:** If script execution panics or times out, temp file might not be cleaned up.

**Mitigation:**
- Use `tokio::fs::remove_file()` in cleanup (non-blocking)
- Ignore cleanup errors (temp dir will eventually be purged by OS)
- Consider adding periodic cleanup task (future enhancement)

**Code pattern:**

```rust
// Create temp file
let temp_script = temp_dir.join(format!("rulez-inline-{}.sh", script_id));
tokio::fs::write(&temp_script, script_text).await?;

// Execute script
let result = execute_script_with_timeout(&temp_script, ...).await;

// ALWAYS cleanup, even on error
let _ = tokio::fs::remove_file(&temp_script).await;  // Ignore errors

result
```

---

### Risk 4: Inline Script Interpreter Detection

**Issue:** If shebang is malformed or interpreter not in PATH, execution will fail.

**Mitigation:**
- Default to `sh` if shebang missing/invalid
- Allow explicit `interpreter` field to override detection
- Log warning if interpreter not found (fail_open will allow)

**Example:**

```yaml
# Explicit interpreter (safest)
run:
  inline: |
    import sys
    sys.exit(0)
  interpreter: python3

# Shebang detection (flexible)
run:
  inline: |
    #!/usr/bin/env python3
    import sys
    sys.exit(0)

# Default to sh (fallback)
run:
  inline: echo "simple script"
```

---

## Performance Implications

### prompt_match

**Impact:** Negligible
- Regex compilation cached by existing code (same as command_match)
- UserPromptSubmit events are infrequent (user-paced)
- No additional I/O or async operations

**Measurement:** Same as command_match (< 1ms per rule evaluation)

---

### require_fields

**Impact:** Negligible
- Simple field existence check (HashMap lookup)
- Executes before other actions (may short-circuit, improving performance)
- No I/O or computation

**Measurement:** < 0.1ms per rule evaluation

---

### Inline Script Blocks

**Impact:** Moderate (same as file-based scripts)
- Temp file write: ~1-5ms (OS-dependent)
- Script execution: bounded by timeout (default 5s)
- Temp file cleanup: async, non-blocking

**Optimization opportunity:** Cache parsed scripts (future enhancement)

**Measurement:** Same as existing `run` action + temp file overhead (~1-5ms)

---

## Testing Strategy

### Unit Tests (Per Feature)

**prompt_match:**
- Match success (UserPromptSubmit with matching prompt)
- Match failure (non-matching prompt)
- No match (non-UserPromptSubmit event)
- Invalid regex (should fail validation in config.rs)

**require_fields:**
- All fields present → allow
- One field missing → block
- No tool_input → block
- Empty field list → allow (no validation)
- Warn mode → inject warning instead of block

**Inline scripts:**
- Python script execution (shebang detection)
- Bash script execution (explicit interpreter)
- Script blocking (exit code 1)
- Script injecting context (stdout)
- Timeout handling
- Temp file cleanup (even on error)
- Shebang parsing edge cases

### Integration Tests (Cross-Feature)

**Matcher + Action combos:**
- `prompt_match` + `inject_inline` (inject context on certain prompts)
- `prompt_match` + `require_fields` (validate fields for specific prompts)
- `require_fields` + inline script (validate, then run custom logic)

**Priority + Mode combos:**
- Multiple rules with `prompt_match` (priority determines order)
- `require_fields` in warn mode (inject warning, don't block)
- Inline script in audit mode (log only, no blocking)

### End-to-End Tests

**Real-world scenarios:**
1. User asks "Can you refactor X?" → inject coding standards
2. Write tool without `file_path` → block with helpful error
3. JSON file write → inline Python validator checks JSON syntax

---

## Migration Path

### From Existing Configurations

**All new features are additive** - existing configs continue to work unchanged.

**prompt_match:**
- No migration needed
- Optional field, ignored if not present

**require_fields:**
- No migration needed
- Optional field, ignored if not present

**Inline scripts:**
- File-based scripts (`run: "script.py"`) continue to work
- Can migrate incrementally to inline syntax
- Both styles can coexist in same config

**Example migration:**

```yaml
# v1.2 (file-based)
rules:
  - name: validate-json
    matchers:
      extensions: [.json]
    actions:
      run: .claude/validators/json-check.py

# v1.3 (inline - optional migration)
rules:
  - name: validate-json
    matchers:
      extensions: [.json]
    actions:
      run:
        inline: |
          #!/usr/bin/env python3
          import json, sys
          event = json.load(sys.stdin)
          content = event['tool_input'].get('content', '')
          try:
              json.loads(content)
              sys.exit(0)
          except:
              print("Invalid JSON", file=sys.stderr)
              sys.exit(1)
```

No breaking changes.

---

## Documentation Requirements

### User Documentation

1. **Matchers Reference** (update existing)
   - Add `prompt_match` to matchers table
   - Document pattern syntax (regex)
   - Show example with `operations: [UserPromptSubmit]`

2. **Actions Reference** (update existing)
   - Add `require_fields` to actions table
   - Document execution order (validation → blocking → injection → validators)
   - Show examples of field validation

3. **RunAction Formats** (update existing)
   - Add inline script syntax
   - Document `interpreter` field (optional)
   - Show shebang detection examples
   - Document temp file behavior (for advanced users)

### Developer Documentation

1. **Architecture Guide** (update existing)
   - Document matcher extensibility pattern
   - Document action execution order
   - Document RunAction enum extensibility

2. **Testing Guide** (update existing)
   - Add test patterns for new matchers
   - Add test patterns for action validation
   - Add test patterns for inline scripts (temp file cleanup)

---

## Future Extensibility

### Matcher Pipeline

**Pattern established:** Add field to `Matchers`, extend `matches_rule()`.

**Future matchers could include:**
- `response_match` - Match against tool responses (PostToolUse events)
- `file_content_match` - Regex on file content (for Write/Edit)
- `env_match` - Environment variable patterns

**Architecture supports:** Unlimited matchers, all using AND logic.

---

### Action Pipeline

**Pattern established:** Add field to `Actions`, extend `execute_rule_actions()` respecting execution order.

**Future actions could include:**
- `inject_template` - Template-based injection (with variable substitution)
- `modify_input` - Transform tool parameters before execution
- `rate_limit` - Throttle operations per time window

**Architecture supports:** Sequential action execution with well-defined precedence.

---

### RunAction Variants

**Pattern established:** `#[serde(untagged)]` enum with multiple syntax styles.

**Future variants could include:**
- `Remote { url, method }` - HTTP-based validators
- `Container { image, command }` - Docker-based validators
- `Cached { script, ttl }` - Cached script results

**Architecture supports:** Unlimited variants, YAML can use any syntax.

---

## Summary

All three v1.3 features integrate cleanly into existing architecture:

| Feature | Integration | Complexity | New Components | Dependencies |
|---------|-------------|------------|----------------|--------------|
| **prompt_match** | Extend Matchers | Low | 0 | regex (existing) |
| **require_fields** | Extend Actions | Low | 0 | serde_json (existing) |
| **Inline scripts** | Extend RunAction | Medium | 1 function | tokio::fs, uuid (existing) |

**Total new code estimate:** ~250-300 lines (including tests)

**Architecture changes:** None (all extensions, no modifications)

**Build order:** 4 → 5 → 6 (simplest to most complex)

**Risk level:** Low (proven patterns, additive changes, no breaking changes)

**Migration:** Zero - existing configs work unchanged
