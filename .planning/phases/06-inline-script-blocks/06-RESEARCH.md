# Phase 6: Inline Script Blocks - Research

**Researched:** 2026-02-09
**Domain:** Inline scripting for validation logic (evalexpr + shell scripts)
**Confidence:** HIGH

## Summary

Phase 6 enables users to write validation logic directly in YAML configuration files using two approaches: (1) evalexpr expressions with custom functions for simple field validation, and (2) inline shell scripts using YAML literal blocks for complex validation scenarios. The research confirms that **evalexpr 13.1 (already in use) with custom function extensions + tokio Command for shell scripts** is the optimal implementation path.

The decision to ship without sandboxing is locked per user constraints. Security implications are documented but implementation is deferred to v1.4. This approach prioritizes user flexibility and rapid iteration while clearly communicating risks. The fail-closed pattern (timeout = block) and timeout protection provide baseline safety, while custom evalexpr functions (get_field, has_field) enable field inspection without requiring users to learn JSON Pointer syntax.

**Primary recommendation:** Extend existing evalexpr usage in enabled_when with custom functions, add inline_script action using YAML literal block syntax (|), validate expressions at config load time, execute shell scripts with tokio Command and timeout protection (fail-closed on timeout).

<user_constraints>
## User Constraints (from phase_context)

### Locked Decisions

**Decision: evalexpr + shell scripts, NO sandboxing**
- Ship both evalexpr custom functions and inline shell scripts without sandboxing
- Document security implications clearly in CLAUDE.md and hooks.yaml comments
- Users accept risk — RuleZ targets developers, not untrusted environments
- This is a conscious tradeoff: flexibility and speed over isolation
- Baseline safety: timeout protection (fail-closed on timeout) + audit logging

**Implementation Freedom:**
- evalexpr custom functions implementation approach (Context extension pattern)
- Shell script execution mechanism (tokio Command pattern already established)
- Timeout implementation (already exists: config.settings.script_timeout)
- YAML syntax for inline scripts (use | literal block, consistent with inject_inline)
- Config validation approach (extend existing Config::validate pattern from Phase 5)

### Deferred Ideas (OUT OF SCOPE)

- Cross-platform sandboxing (seccomp/Landlock) — deferred to v1.4
- Windows-specific security considerations — Linux/macOS focus for v1.3
- Advanced evalexpr features (loops, complex control flow) — keep it simple
- Inline script caching/compilation — execute fresh each time for v1.3

</user_constraints>

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| evalexpr | 13.1 | Expression evaluation with custom functions | Already in use for enabled_when, supports custom function registration, mature crate (100+ contributors), no additional dependencies |
| tokio | (workspace) | Async shell script execution with timeout | Already in use for validator scripts and inject_command, provides timeout primitives, industry standard for async Rust |
| serde_json | (workspace) | Field inspection for custom functions | Already in use, provides Value type for get_field/has_field implementation |
| serde_yaml | (workspace) | YAML literal block parsing | Already handles inject_inline, will naturally parse | literal blocks as multiline strings |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| regex | (workspace) | Expression pattern validation | Already used for prompt_match, may be useful for basic script syntax validation at config load |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| evalexpr custom functions | rhai 1.x scripting engine | Rhai provides full scripting (loops, functions, modules) but adds significant complexity and cognitive overhead. evalexpr's expression-only model is simpler and sufficient for field validation. |
| evalexpr | Custom expression parser | Writing a parser is high-risk, low-reward. evalexpr is battle-tested with 8+ years of development. |
| Inline shell scripts | Embedded JavaScript (deno_core) | JS isolation is better but adds 40+ MB to binary and significant compilation time. Not worth it for v1.3. |
| YAML literal blocks | Separate script files | Defeats the purpose of "inline" scripts. User wants everything in hooks.yaml. |
| tokio Command | std::process::Command | std is synchronous, would block the async runtime. tokio Command is the right tool for async execution. |

**Installation:**

No additional dependencies required. All libraries already in workspace Cargo.toml.

## Architecture Patterns

### Recommended Project Structure

Inline script logic integrates into existing structure:

```
rulez/src/
├── models.rs           # Add inline_script to Actions, InlineScriptConfig type
├── config.rs           # Add validation for inline_script expressions and syntax
├── hooks.rs            # Add execute_inline_script, extend build_eval_context with custom functions
└── tests/
    ├── inline_script_unit.rs         # Unit tests for custom functions, syntax validation
    └── inline_script_integration.rs  # E2E tests with full stack
```

### Pattern 1: Custom evalexpr Functions for Field Inspection

**What:** Extend evalexpr context with get_field() and has_field() functions that inspect tool_input JSON

**When to use:** For simple inline validation expressions that need to check field values or existence

**Example:**
```rust
// Source: Adapted from https://docs.rs/evalexpr/13.1/evalexpr/ and existing build_eval_context
use evalexpr::{HashMapContext, Value, Function, EvalexprError, ContextWithMutableFunctions};

fn build_eval_context_with_custom_functions(
    event: &Event
) -> Result<HashMapContext<DefaultNumericTypes>, EvalexprError> {
    let mut ctx = build_eval_context(event); // Existing function from hooks.rs

    // Add get_field(path) function - returns field value or empty string if missing
    let tool_input_clone = event.tool_input.clone();
    ctx.set_function(
        "get_field".to_string(),
        Function::new(move |argument| {
            let path = argument.as_string()?;
            let pointer = dot_to_pointer(&path);

            match &tool_input_clone {
                Some(input) => {
                    match input.pointer(&pointer) {
                        Some(Value::String(s)) => Ok(Value::String(s.clone())),
                        Some(Value::Number(n)) => Ok(Value::Float(n.as_f64().unwrap_or(0.0))),
                        Some(Value::Bool(b)) => Ok(Value::Boolean(*b)),
                        _ => Ok(Value::String(String::new())),
                    }
                }
                None => Ok(Value::String(String::new())),
            }
        })
    )?;

    // Add has_field(path) function - returns true if field exists and is not null
    let tool_input_clone = event.tool_input.clone();
    ctx.set_function(
        "has_field".to_string(),
        Function::new(move |argument| {
            let path = argument.as_string()?;
            let pointer = dot_to_pointer(&path);

            match &tool_input_clone {
                Some(input) => {
                    Ok(Value::Boolean(
                        input.pointer(&pointer)
                            .map(|v| !v.is_null())
                            .unwrap_or(false)
                    ))
                }
                None => Ok(Value::Boolean(false)),
            }
        })
    )?;

    Ok(ctx)
}
```

**YAML Usage Example:**
```yaml
rules:
  - name: validate-user-email
    matchers:
      tools: [Write]
      extensions: [.json]
    actions:
      validate_expr: 'has_field("user.email") && get_field("user.email") != ""'
      block: true  # Block if expression evaluates to false
```

### Pattern 2: Inline Shell Scripts with YAML Literal Blocks

**What:** Execute shell scripts written directly in YAML using | literal block syntax

**When to use:** For complex validation that requires shell tools (grep, jq, custom scripts)

**Example:**
```rust
// Source: Adapted from existing execute_inject_command pattern in hooks.rs:693-753
async fn execute_inline_script(
    script_content: &str,
    event: &Event,
    rule: &Rule,
    config: &Config,
) -> Result<bool> {
    let timeout_secs = rule
        .metadata
        .as_ref()
        .map(|m| m.timeout)
        .unwrap_or(config.settings.script_timeout);

    // Write script to temp file (security: predictable temp path is OK, content is from config)
    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join(format!("rulez-inline-{}.sh", uuid::Uuid::new_v4()));
    tokio::fs::write(&script_path, script_content).await?;

    // Make executable (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = tokio::fs::metadata(&script_path).await?.permissions();
        perms.set_mode(0o700);
        tokio::fs::set_permissions(&script_path, perms).await?;
    }

    // Execute with timeout
    let mut command = Command::new("sh");
    command.arg(&script_path);
    command.stdin(std::process::Stdio::piped());
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());

    let mut child = command.spawn()?;

    // Send event JSON to stdin
    if let Some(stdin) = child.stdin.as_mut() {
        let event_json = serde_json::to_string(event)?;
        tokio::io::AsyncWriteExt::write_all(stdin, event_json.as_bytes()).await?;
    }
    drop(child.stdin.take());

    // Wait with timeout
    let output = match timeout(
        Duration::from_secs(timeout_secs as u64),
        child.wait_with_output(),
    ).await {
        Ok(Ok(o)) => o,
        Ok(Err(e)) => {
            // Clean up temp file
            let _ = tokio::fs::remove_file(&script_path).await;
            return Err(e.into());
        }
        Err(_) => {
            // Timeout - fail closed
            let _ = tokio::fs::remove_file(&script_path).await;
            tracing::warn!(
                "Inline script for rule '{}' timed out after {}s - blocking (fail-closed)",
                rule.name, timeout_secs
            );
            return Ok(false); // false = validation failed, block operation
        }
    };

    // Clean up temp file
    let _ = tokio::fs::remove_file(&script_path).await;

    // Exit code 0 = validation passed, non-zero = validation failed
    Ok(output.status.success())
}
```

**YAML Usage Example:**
```yaml
rules:
  - name: validate-no-secrets
    matchers:
      tools: [Write, Edit]
      extensions: [.py, .js, .ts]
    actions:
      inline_script: |
        #!/bin/bash
        # Read event JSON from stdin
        EVENT=$(cat)

        # Extract file content
        CONTENT=$(echo "$EVENT" | jq -r '.tool_input.content // ""')

        # Check for common secret patterns
        if echo "$CONTENT" | grep -qE "(api_key|password|secret).*=.*['\"][^'\"]{20,}['\"]"; then
          echo "Potential secret detected in file content" >&2
          exit 1
        fi

        exit 0
      block: true  # Block if script exits non-zero
```

### Pattern 3: Configuration-Time Validation for Inline Scripts

**What:** Validate expression syntax and script structure when loading hooks.yaml

**When to use:** In Config::validate() method, similar to enabled_when and prompt_match validation

**Example:**
```rust
// Source: Pattern established in config.rs:143-150 (Phase 3 enabled_when validation)
impl Config {
    pub fn validate(&self) -> Result<()> {
        for rule in &self.rules {
            // Validate validate_expr syntax (if present)
            if let Some(ref expr) = rule.actions.validate_expr {
                // Check syntax by building operator tree
                build_operator_tree::<DefaultNumericTypes>(expr).with_context(|| {
                    format!(
                        "Invalid validate_expr '{}' in rule '{}': syntax error",
                        expr, rule.name
                    )
                })?;

                // Verify custom functions are used correctly (optional: parse AST)
                // For v1.3, basic syntax check is sufficient
            }

            // Validate inline_script structure (if present)
            if let Some(ref script) = rule.actions.inline_script {
                // Basic validation: non-empty, reasonable length
                if script.trim().is_empty() {
                    return Err(anyhow::anyhow!(
                        "Empty inline_script in rule '{}'",
                        rule.name
                    ));
                }

                // Check for shebang (recommended but not required)
                if !script.trim_start().starts_with("#!") {
                    tracing::warn!(
                        "inline_script in rule '{}' missing shebang - may not execute correctly",
                        rule.name
                    );
                }

                // Warn if script is very large (potential config bloat)
                if script.len() > 10_000 {
                    tracing::warn!(
                        "inline_script in rule '{}' is very large ({} bytes) - consider external file",
                        rule.name, script.len()
                    );
                }
            }

            // Validate that validate_expr and inline_script aren't both present
            if rule.actions.validate_expr.is_some() && rule.actions.inline_script.is_some() {
                return Err(anyhow::anyhow!(
                    "Rule '{}' cannot have both validate_expr and inline_script - choose one",
                    rule.name
                ));
            }
        }
        Ok(())
    }
}
```

### Pattern 4: Integration with Action Pipeline

**What:** Execute inline validation as part of execute_rule_actions, similar to existing run action

**When to use:** After rule matches, alongside inject/block actions

**Architectural recommendation:**
1. **Execution order**: validate_expr/inline_script runs BEFORE inject actions (validation gates injection)
2. **Failure semantics**: Validation failure = block (exit code 2, no injection)
3. **Success semantics**: Validation success = continue to inject/allow actions
4. **Compatibility**: validate_expr/inline_script are mutually exclusive with run action (only one validation approach per rule)

**Example:**
```rust
// Source: Pattern from hooks.rs:790-840 (execute_rule_actions)
async fn execute_rule_actions_with_mode(
    event: &Event,
    rule: &Rule,
    config: &Config,
    mode: PolicyMode,
) -> Result<Response> {
    let mut response = Response::allow();

    // Step 1: Run inline validation (if present)
    if let Some(ref expr) = rule.actions.validate_expr {
        let ctx = build_eval_context_with_custom_functions(event)?;
        match eval_boolean_with_context(expr, &ctx) {
            Ok(true) => {
                // Validation passed, continue
            }
            Ok(false) => {
                // Validation failed, block
                return Ok(Response::block(format!(
                    "Validation expression failed: {}",
                    expr
                )));
            }
            Err(e) => {
                // Expression error, fail closed
                tracing::warn!(
                    "validate_expr failed for rule '{}': {} - blocking (fail-closed)",
                    rule.name, e
                );
                return Ok(Response::block(format!(
                    "Validation expression error: {}",
                    e
                )));
            }
        }
    } else if let Some(ref script) = rule.actions.inline_script {
        match execute_inline_script(script, event, rule, config).await {
            Ok(true) => {
                // Validation passed, continue
            }
            Ok(false) => {
                // Validation failed, block
                return Ok(Response::block(
                    "Inline script validation failed".to_string()
                ));
            }
            Err(e) => {
                // Script error, fail closed
                tracing::warn!(
                    "inline_script failed for rule '{}': {} - blocking (fail-closed)",
                    rule.name, e
                );
                return Ok(Response::block(format!(
                    "Inline script error: {}",
                    e
                )));
            }
        }
    }

    // Step 2: Execute inject actions (existing logic)
    // Step 3: Check block conditions (existing logic)

    Ok(response)
}
```

### Anti-Patterns to Avoid

- **Don't execute inline scripts without timeout protection:** Infinite loops can DoS the system. Always use tokio::time::timeout with fail-closed behavior.
- **Don't leave temp files on disk:** Clean up inline script temp files even on error paths.
- **Don't silently fail validation:** Always log validation failures with clear context (rule name, expression/script excerpt).
- **Don't allow both validate_expr and inline_script:** Force users to choose one approach per rule for clarity.
- **Don't expose sensitive data to scripts:** Event JSON may contain sensitive tool_input. Document this risk clearly.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Expression parsing | Custom tokenizer/parser | evalexpr with custom functions | Expression parsing is deceptively complex (operator precedence, parentheses, type coercion, error recovery). evalexpr is mature and tested. |
| Shell script sandboxing | seccomp/pledge wrappers | Document risk + timeout protection | Cross-platform sandboxing is v1.4 work. Timeout + audit logging is sufficient baseline for v1.3. |
| YAML literal block parsing | Custom multiline string handler | serde_yaml's built-in support | serde_yaml already handles \| and > block styles correctly. No custom logic needed. |
| Async script execution | std::process + thread pool | tokio Command | tokio Command integrates with async runtime, provides timeout primitives, handles zombie process reaping on Unix. |

**Key insight:** The temptation to build custom sandboxing is strong (security best practice), but it's a rabbit hole that delays shipping. For developer-targeted tooling, documenting risk + baseline safety (timeout, audit) is pragmatic. Ship fast, iterate on security in v1.4.

## Common Pitfalls

### Pitfall 1: Custom Function Lifetimes and Closures

**What goes wrong:** Custom functions capture `&Event` reference, but closure outlives the borrow

**Why it happens:** evalexpr Function closures must be 'static, but event references are not

**How to avoid:** Clone event data into closure captures:
```rust
// WRONG - won't compile
let tool_input_ref = &event.tool_input;
ctx.set_function("get_field", Function::new(move |arg| {
    tool_input_ref.pointer(...) // Error: lifetime issue
}));

// RIGHT - clone data into closure
let tool_input_clone = event.tool_input.clone();
ctx.set_function("get_field", Function::new(move |arg| {
    tool_input_clone.as_ref().and_then(|input| input.pointer(...)) // OK
}));
```

**Warning signs:** Compile errors about "borrowed value does not live long enough" or "closure may outlive"

### Pitfall 2: YAML Block Chomping with Shell Scripts

**What goes wrong:** Shell script has trailing newlines that interfere with execution or shebang line has indentation

**Why it happens:** YAML block chomping indicators (|, |-, |+) control trailing newlines, and YAML strips leading indentation

**How to avoid:**
- Use `|` (literal) not `>` (folded) for scripts — folded replaces newlines with spaces
- Use `|-` to strip trailing newlines if shebang line must be exact
- Always start inline_script on the line AFTER the `|` indicator
- Ensure shebang `#!/bin/bash` is the first line with no leading spaces

**Example:**
```yaml
# WRONG - shebang is indented
actions:
  inline_script: |
    #!/bin/bash  # This line has leading spaces from YAML indentation
    echo "test"

# RIGHT - shebang starts at column 0 of literal block
actions:
  inline_script: |
    #!/bin/bash
    echo "test"
```

**Warning signs:** "command not found" errors, script doesn't execute, shebang not recognized

### Pitfall 3: Timeout Default Too Short for Complex Scripts

**What goes wrong:** Complex shell scripts (network calls, large file processing) timeout prematurely

**Why it happens:** Default script_timeout is 5 seconds (sufficient for validators, too short for some inline scripts)

**How to avoid:**
- Allow per-rule timeout override in metadata field (already exists)
- Document timeout recommendations in CLAUDE.md
- Log timeout warnings prominently (stderr + tracing::warn)
- Fail closed on timeout (block operation)

**Example:**
```yaml
rules:
  - name: expensive-validation
    matchers:
      tools: [Write]
    actions:
      inline_script: |
        #!/bin/bash
        # Complex validation with network call
        curl -s https://api.example.com/validate | jq '.valid'
    metadata:
      timeout: 30  # Override default 5s timeout
```

**Warning signs:** Inconsistent validation results, timeouts in production but not in testing

### Pitfall 4: evalexpr Type Coercion Surprises

**What goes wrong:** get_field returns wrong type, expressions fail with "expected boolean, got string"

**Why it happens:** evalexpr has implicit type coercion rules that may not match user expectations

**How to avoid:**
- Return consistent types from get_field: strings for JSON strings, floats for numbers, booleans for bools
- For missing fields, return empty string (falsy in boolean context)
- Document type coercion in CLAUDE.md with examples
- Use explicit comparisons: `get_field("count") > 0` not `get_field("count")`

**Example:**
```yaml
# WRONG - ambiguous boolean coercion
validate_expr: 'get_field("active")'  # Returns "true" string, not boolean

# RIGHT - explicit comparison
validate_expr: 'get_field("active") == "true"'

# BETTER - has_field for existence checks
validate_expr: 'has_field("active")'
```

**Warning signs:** Expressions fail with type errors, unexpected boolean evaluation results

## Code Examples

Verified patterns from evalexpr and tokio official sources:

### Custom Function Registration with evalexpr

```rust
// Source: https://docs.rs/evalexpr/13.1/evalexpr/ and https://github.com/ISibboI/evalexpr
use evalexpr::{HashMapContext, Function, Value, ContextWithMutableFunctions};

fn main() {
    let mut context = HashMapContext::new();

    // Register custom function with closure
    context.set_function(
        "greet".to_string(),
        Function::new(|argument| {
            let name = argument.as_string()?;
            Ok(Value::String(format!("Hello, {}!", name)))
        })
    ).unwrap();

    // Use custom function in expression
    let result = evalexpr::eval_with_context(
        r#"greet("World")"#,
        &context
    ).unwrap();

    assert_eq!(result, Value::String("Hello, World!".to_string()));
}
```

### Tokio Command with Timeout

```rust
// Source: https://docs.rs/tokio/latest/tokio/process/struct.Command.html
use tokio::process::Command;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() {
    let mut cmd = Command::new("sh");
    cmd.arg("-c");
    cmd.arg("sleep 10");

    let child = cmd.spawn().expect("failed to spawn");

    // Wait for at most 5 seconds
    match timeout(Duration::from_secs(5), child.wait_with_output()).await {
        Ok(Ok(output)) => {
            println!("Command completed: {:?}", output.status);
        }
        Ok(Err(e)) => {
            eprintln!("Command failed: {}", e);
        }
        Err(_) => {
            eprintln!("Command timed out after 5s");
        }
    }
}
```

### YAML Literal Block Syntax

```yaml
# Source: https://yaml-multiline.info/ and https://yaml.org/spec/1.2/spec.html
# Literal block (|) - keeps newlines as-is
script: |
  #!/bin/bash
  echo "Line 1"
  echo "Line 2"
  # Result: "#!/bin/bash\necho \"Line 1\"\necho \"Line 2\"\n"

# Literal block with strip (|-) - removes trailing newline
script: |-
  #!/bin/bash
  echo "No trailing newline"
  # Result: "#!/bin/bash\necho \"No trailing newline\""

# Folded block (>) - joins lines with spaces (DON'T use for scripts!)
description: >
  This is a long
  description that
  spans multiple lines
  # Result: "This is a long description that spans multiple lines\n"
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| External validator scripts only | Inline scripts + external scripts | 2024+ trend | Modern infrastructure-as-code (Terraform, Pulumi) embraces inline scripts for simple cases. External files for complex reusable logic. RuleZ follows this pattern. |
| Sandboxing by default | Sandboxing opt-in | 2020+ | Developer tools (pre-commit, husky, taskfile) rarely sandbox by default. Performance and complexity costs outweigh benefits for trusted users. |
| Custom expression languages | evalexpr/rhai standard crates | 2018+ | Rust ecosystem converged on evalexpr (simple expressions) and rhai (full scripting). Custom parsers are anti-pattern. |
| std::process | tokio::process | 2019+ | Async Rust requires async-aware subprocess management. tokio Command is the standard for async runtimes. |

**Deprecated/outdated:**
- **shell-words crate for parsing**: Not needed when using `sh -c` — shell handles parsing
- **nix crate for Unix signals**: tokio handles process cleanup and signal propagation correctly

## Open Questions

1. **Should validate_expr and inline_script be mutually exclusive with run action?**
   - What we know: All three are validation mechanisms
   - What's unclear: Can users combine them meaningfully?
   - Recommendation: **Mutually exclusive** — forcing one approach per rule keeps configs simple and intent clear

2. **Should we support multiple inline scripts per rule (array syntax)?**
   - What we know: Single inline_script field in models.rs
   - What's unclear: Value of running multiple scripts in sequence
   - Recommendation: **Not in v1.3** — users can combine logic in one script or use multiple rules. Keep it simple.

3. **Should custom functions support nested field access implicitly?**
   - What we know: get_field("user.name") requires dot notation
   - What's unclear: Should get_field("user")["name"] work like JavaScript?
   - Recommendation: **No** — evalexpr doesn't support object property access. Keep it simple with dot notation only.

4. **Should we parse inline_script for dangerous commands (rm -rf, dd, etc.)?**
   - What we know: No sandboxing per user constraints
   - What's unclear: Value of static analysis warnings
   - Recommendation: **Not in v1.3** — false positives are annoying, static analysis is incomplete. Document risk instead.

## Security Considerations

**IMPORTANT: No sandboxing in v1.3 per user constraints. Document these risks:**

### Risk 1: Arbitrary Code Execution

**Risk:** Inline shell scripts execute with full user permissions (read files, network access, exec binaries)

**Mitigation:**
- Timeout protection (fail-closed on timeout)
- Audit logging captures all inline script executions
- User awareness: RuleZ targets developer laptops, not production servers
- Future: seccomp/Landlock sandboxing in v1.4

### Risk 2: Secret Exposure via Event JSON

**Risk:** Event JSON passed to inline scripts may contain sensitive tool_input (API keys in config files, credentials in prompts)

**Mitigation:**
- Document this risk prominently in CLAUDE.md
- Recommend users avoid logging event JSON in scripts
- Phase 5 field validation already avoids logging field values (only types)
- Future: field redaction system in v1.4

### Risk 3: Timeout Exhaustion DoS

**Risk:** Multiple rules with expensive inline scripts can delay hook responses, timeout Claude Code

**Mitigation:**
- Per-rule timeout with reasonable default (5s)
- Total hook processing timeout enforced by Claude Code (not RuleZ responsibility)
- Warn users about expensive validation in config comments

### Risk 4: Temp File Security

**Risk:** Inline scripts written to predictable temp paths could be race-condition targets

**Mitigation:**
- Use UUID in temp filename for uniqueness
- Set temp file permissions to 0o700 (owner-only)
- Clean up temp files on all exit paths
- Temp dir is already user-scoped on Unix (/tmp is protected by kernel)

**Reference:** [Sherlock — Rust Security & Auditing Guide by Sherlock: 2026](https://sherlock.xyz/post/rust-security-auditing-guide-2026) discusses Rust security best practices including process management and resource exhaustion.

## Sources

### Primary (HIGH confidence)

- [evalexpr crate documentation](https://docs.rs/evalexpr/13.1/evalexpr/) - custom function API, context management
- [evalexpr GitHub repository](https://github.com/ISibboI/evalexpr) - examples and patterns
- [tokio Command documentation](https://docs.rs/tokio/latest/tokio/process/struct.Command.html) - async subprocess execution
- [YAML Multiline Strings](https://yaml-multiline.info/) - literal block syntax and chomping
- RuleZ codebase:
  - rulez/src/hooks.rs:360-413 (build_eval_context, is_rule_enabled)
  - rulez/src/hooks.rs:693-753 (execute_inject_command with timeout)
  - rulez/src/hooks.rs:850-917 (execute_validator_script with timeout)
  - rulez/src/config.rs:143-150 (enabled_when validation pattern)

### Secondary (MEDIUM confidence)

- [Helm YAML Techniques](https://helm.sh/docs/chart_template_guide/yaml_techniques/) - YAML block scalar patterns in production
- [Ansible YAML Syntax](https://docs.ansible.com/projects/ansible/latest/reference_appendices/YAMLSyntax.html) - multiline string best practices
- [Sherlock Rust Security Audit Guide 2026](https://sherlock.xyz/post/rust-security-auditing-guide-2026) - process management security
- Phase 5 Research (field validation patterns) - dot_to_pointer, fail-closed philosophy

### Tertiary (LOW confidence)

- Web search results for evalexpr custom functions - community examples and patterns
- YAML specification 1.2 - block scalar chomping indicators

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - evalexpr already in use, tokio Command established pattern
- Architecture: HIGH - follows Phase 3 (enabled_when) and Phase 5 (field validation) patterns
- Pitfalls: MEDIUM - identified from evalexpr docs and tokio experience, but inline script pattern is new to RuleZ
- Security: HIGH - risks are well-understood (no sandboxing is explicit decision), mitigations follow industry best practices

**Research date:** 2026-02-09
**Valid until:** 60 days (evalexpr is stable, but inline script usage patterns may evolve with user feedback)
