# Phase 2: Command-Based Context Generation - Research

**Researched:** 2026-02-06
**Domain:** Rust async subprocess execution with Tokio
**Confidence:** HIGH

## Summary

This phase implements `inject_command`, a new action type that executes a shell command and injects its stdout as context. The implementation follows the same pattern as Phase 1 (inject_inline) - adding a field to Actions and handling it in hooks.rs.

The codebase already has robust subprocess execution infrastructure in `execute_validator_script()` that handles spawning, stdin/stdout/stderr capture, timeouts, and error handling. The inject_command implementation can reuse this pattern but simplified: no stdin writing needed, just capture stdout and inject as context.

Key insight: Unlike validators that return JSON responses, inject_command simply captures raw stdout and injects it directly. Error handling follows fail-open pattern: command failures log warnings but don't block the operation.

**Primary recommendation:** Implement inject_command by adapting execute_validator_script() into a simpler execute_inject_command() function that captures stdout and returns it as context.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| tokio | 1.x | Async runtime | Already used throughout codebase |
| tokio::process::Command | 1.x | Async subprocess | Already used in execute_validator_script |
| tokio::time::timeout | 1.x | Timeout protection | Already used in execute_validator_script |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| std::process::Stdio | std | Configure stdin/stdout/stderr pipes | Setting up subprocess I/O |
| tracing | 0.1.x | Logging warnings | Already used for validator script errors |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| tokio::process | std::process | Blocks async runtime - DO NOT USE |
| Shell execution | Direct command | Shell allows pipes, redirects - PREFERRED |

**No new dependencies needed** - all required functionality exists in the codebase.

## Architecture Patterns

### Existing Pattern: execute_validator_script()

The codebase already implements subprocess execution with timeout in `hooks.rs` lines 414-502.

**Pattern characteristics:**
- Spawn Command with stdin/stdout/stderr piped
- Write event JSON to stdin (NOT needed for inject_command)
- Wait with timeout using `tokio::time::timeout()`
- Handle timeout, spawn failure, execution failure
- Parse output (for validators it's JSON, for inject_command it's raw text)

```rust
// Source: rulez/src/hooks.rs execute_validator_script()
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

    // ... spawn, timeout, handle output ...
}
```

### Pattern for inject_command (Simplified)

The inject_command execution differs from validators:

| Aspect | Validator (run:) | inject_command |
|--------|------------------|----------------|
| Input | Receives event JSON on stdin | No stdin needed |
| Output | Returns JSON response | Raw text as context |
| Exit code meaning | 0=allow, non-0=block | 0=success, non-0=log warning |
| Failure handling | May block if fail_open=false | Always log warning, continue |

```rust
// Recommended pattern for inject_command
async fn execute_inject_command(
    command_str: &str,
    rule: &Rule,
    config: &Config,
) -> Result<Option<String>> {
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
    // No stdin needed - don't pipe it

    let child = match command.spawn() {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("Failed to spawn inject_command '{}': {}", command_str, e);
            return Ok(None); // Log and continue
        }
    };

    let output = match timeout(
        Duration::from_secs(timeout_secs as u64),
        child.wait_with_output(),
    ).await {
        Ok(Ok(output)) => output,
        Ok(Err(e)) => {
            tracing::warn!("inject_command '{}' failed: {}", command_str, e);
            return Ok(None);
        }
        Err(_) => {
            tracing::warn!("inject_command '{}' timed out after {}s", command_str, timeout_secs);
            return Ok(None);
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::warn!(
            "inject_command '{}' failed with exit code {}: {}",
            command_str,
            output.status.code().unwrap_or(-1),
            stderr.trim()
        );
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if stdout.trim().is_empty() {
        return Ok(None); // No content to inject
    }

    Ok(Some(stdout))
}
```

### Execution Order in execute_rule_actions

Based on Phase 1 implementation, the order is:
1. Handle blocking (block: true)
2. Handle conditional blocking (block_if_match)
3. **Handle inline injection (inject_inline)** - takes precedence
4. **Handle command injection (inject_command)** - NEW, after inline
5. Handle file injection (inject)
6. Handle script execution (run:)

Rationale for order:
- inject_inline: No I/O, fastest, takes precedence
- inject_command: Subprocess execution, before file read
- inject: File I/O, can fail
- run: Full validator script

### Anti-Patterns to Avoid

- **Direct command execution without shell:** Commands like `git branch --show-current` work, but users expect shell features (pipes, redirects). Use `sh -c` wrapper.
- **Blocking on stdin:** Don't pipe stdin for inject_command - there's no input to send, and waiting for stdin EOF causes hangs.
- **Blocking operations in async context:** Never use `std::process::Command` - it blocks the async runtime. Always use `tokio::process::Command`.
- **Ignoring kill_on_drop:** Consider using `kill_on_drop(true)` for the spawned child to prevent zombie processes if the handler is cancelled.

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Timeout protection | Manual timer + kill | `tokio::time::timeout()` | Race conditions, cleanup complexity |
| Process spawning | `std::process::Command` | `tokio::process::Command` | Blocks async runtime |
| Shell command parsing | Split on spaces | `sh -c "command"` | Quoting, pipes, redirects |
| Stderr capture | Ignore | Log with tracing | Debugging failed commands |

**Key insight:** The existing `execute_validator_script()` already solved these problems. Adapt rather than rewrite.

## Common Pitfalls

### Pitfall 1: Shell vs Direct Execution
**What goes wrong:** Command `git branch --show-current | head -1` fails because the pipe isn't interpreted.
**Why it happens:** Using `Command::new("git")` directly doesn't invoke a shell.
**How to avoid:** Use `sh -c "command"` pattern - `Command::new("sh").arg("-c").arg(command_str)`.
**Warning signs:** Commands with `|`, `>`, `&&`, or `$VAR` failing silently.

### Pitfall 2: Timeout on wait_with_output vs wait
**What goes wrong:** Timeout wraps `wait()` but then calls `wait_with_output()` after, leading to potential deadlock.
**Why it happens:** Misunderstanding of async subprocess flow.
**How to avoid:** Wrap `wait_with_output()` directly in timeout, not `wait()`.
**Warning signs:** Tests passing but production hangs on large outputs.

### Pitfall 3: Stdin Pipe Without Writing
**What goes wrong:** Process hangs waiting for stdin EOF.
**Why it happens:** `Stdio::piped()` for stdin creates expectation of input.
**How to avoid:** Don't configure stdin for inject_command, or explicitly drop it after spawn.
**Warning signs:** Commands that work in shell hang when executed via inject_command.

### Pitfall 4: UTF-8 Output Assumption
**What goes wrong:** Binary output from command causes panic.
**Why it happens:** Using `from_utf8()` instead of `from_utf8_lossy()`.
**How to avoid:** Always use `String::from_utf8_lossy()` for command output.
**Warning signs:** Crashes on certain commands that produce non-UTF8 bytes.

### Pitfall 5: Missing Error Context in Logs
**What goes wrong:** "Command failed" with no details.
**Why it happens:** Not including command string and stderr in log.
**How to avoid:** Log command string, exit code, and stderr on failure.
**Warning signs:** Can't debug failed commands in production logs.

## Code Examples

Verified patterns from the existing codebase:

### Subprocess with Timeout (from hooks.rs)
```rust
// Source: rulez/src/hooks.rs lines 454-481
let output_result = timeout(
    Duration::from_secs(timeout_duration as u64),
    child.wait_with_output(),
).await;

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
```

### Adding Field to Actions (from Phase 1)
```rust
// Source: rulez/src/models.rs Actions struct
/// Actions to take when rule matches
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Actions {
    /// Path to context file to inject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject: Option<String>,

    /// Inline markdown content to inject directly (no file read)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject_inline: Option<String>,

    // ... inject_command goes here ...
}
```

### Integration Test Pattern (from Phase 1)
```rust
// Source: rulez/tests/oq_us2_injection.rs test_us2_inline_content_injection
// Create hooks.yaml with inject_inline rule
let config_content = r#"version: "1.0"
rules:
  - name: inline-warning
    matchers:
      directories: ["/prod/"]
    actions:
      inject_inline: |
        ## Production Warning
        You are editing production files.
"#;
fs::write(claude_dir.join("hooks.yaml"), config_content).expect("write config");

// Run rulez binary with the event
Command::cargo_bin("rulez")
    .expect("binary exists")
    .current_dir(temp_dir.path())
    .write_stdin(event)
    .assert()
    .success()
    .stdout(predicate::str::contains("Production Warning"));
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Inline scripts via run: | Separate script files | Current | Scripts need external files |
| File-based context only | inject_inline added | Phase 1 | Inline content works |
| N/A | inject_command | Phase 2 (this phase) | Dynamic command output |

**Current gap:** No way to generate context dynamically from commands. Users must use full validator scripts with JSON output, which is heavyweight for simple commands like `git branch --show-current`.

## Open Questions

1. **Execution context (cwd)**
   - What we know: The event contains `cwd` field from Claude Code
   - What's unclear: Should inject_command execute in event.cwd or binary's cwd?
   - Recommendation: Use event.cwd if present, matching validate script behavior

2. **Environment variables**
   - What we know: Shell inherits parent's environment
   - What's unclear: Should we sanitize or allow full env access?
   - Recommendation: Inherit environment (same as run: validators)

3. **Command validation**
   - What we know: Users can put any command in inject_command
   - What's unclear: Should we validate/sanitize commands?
   - Recommendation: No validation - trust user config, same as run: scripts

## Sources

### Primary (HIGH confidence)
- `rulez/src/hooks.rs` - Existing subprocess execution pattern (execute_validator_script)
- `rulez/src/models.rs` - Actions struct pattern from inject_inline
- [tokio::process::Command documentation](https://docs.rs/tokio/latest/tokio/process/struct.Command.html)

### Secondary (MEDIUM confidence)
- [Spawn process with timeout and capture output in tokio](https://users.rust-lang.org/t/spawn-process-with-timeout-and-capture-output-in-tokio/128305) - Community patterns

### Project Documentation
- `.planning/ROADMAP.md` - Phase 2 requirements
- `.speckit/features/cch-advanced-rules/spec.md` - US-ADV-05 specification

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Using existing codebase patterns, no new dependencies
- Architecture: HIGH - Clear adaptation of execute_validator_script
- Pitfalls: HIGH - Based on existing code and tokio subprocess documentation

**Research date:** 2026-02-06
**Valid until:** 90 days (stable Rust/Tokio patterns)
