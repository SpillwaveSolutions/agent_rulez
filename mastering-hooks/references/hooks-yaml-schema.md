---
last_modified: 2026-03-16
last_validated: 2026-03-16
---

# hooks.yaml Schema Reference

Complete reference for the RuleZ configuration file format.

## File Location

```
.claude/hooks.yaml    # Project-level (recommended)
~/.claude/hooks.yaml  # User-level (global)
```

## Top-Level Structure

```yaml
version: "1"                    # Required: Schema version ("1" or "1.0")
rules: []                       # Required: List of hook rules
settings: {}                    # Optional: Global settings
```

## Rule Schema

```yaml
rules:
  - name: string                # Required: Unique kebab-case identifier
    description: string         # Optional: Human-readable explanation
    enabled_when: string        # Optional: Evalexpr boolean expression for conditional activation
    mode: string                # Optional: Policy mode (enforce, warn, audit). Default: enforce
    priority: integer           # Optional: Higher number = higher priority (default: 0)
    matchers:                   # Required: Conditions to match
      operations: [EventType]   # Filter by event type (e.g. [PreToolUse])
      tools: [ToolName]         # Filter by tool name
      extensions: [.ext]        # Filter by file extension
      directories: [path/]      # Filter by directory (glob patterns)
      command_match: "regex"    # Filter by Bash command regex
      prompt_match: "regex"     # Filter by user prompt regex
      require_fields: [path]    # Require fields in tool_input
      field_types: {}           # Validate field types in tool_input
    actions:                    # Required: What to do when matched
      block: true               # Block the operation
      block_if_match: "regex"   # Conditionally block
      inject: "path"            # Inject file content
      inject_inline: "content"  # Inject inline content
      inject_command: "cmd"     # Inject command output
      run: "script"             # Run validator script
      validate_expr: "expr"     # Evalexpr validation
      inline_script: "script"   # Inline shell validation
    governance:                 # Optional: Provenance metadata
      author: string
      reason: string
      confidence: string        # high, medium, low
      tags: [string]
    metadata:                   # Optional: Legacy metadata (deprecated)
      enabled: boolean          # Default: true
      priority: integer         # Legacy priority field
      timeout: integer          # Script timeout in seconds
```

---

## Event Types

RuleZ supports 16 event types. All platforms translate their native events into these unified types via adapters.

| Event | Description | Available Context | Platforms |
|-------|-------------|-------------------|-----------|
| `PreToolUse` | Before tool executes | tool_name, tool_input, file_path | All |
| `PostToolUse` | After tool completes | tool_name, tool_input, tool_output, file_path | All |
| `PostToolUseFailure` | After tool fails | tool_name, error | All |
| `PermissionRequest` | User approval requested | tool_name, permission_type | Claude Code, Gemini (dual) |
| `UserPromptSubmit` | User sends message | prompt_text | All |
| `BeforeAgent` | Agent/subagent launched | agent_type | Claude Code, Gemini (dual) |
| `AfterAgent` | Agent/subagent completed | agent_type | Claude Code, Gemini |
| `BeforeModel` | Before model inference | model_id | Gemini only |
| `AfterModel` | After model inference | model_id, response | Gemini only |
| `BeforeToolSelection` | Before tool selection | candidates | Gemini only |
| `SessionStart` | New session begins | session_id, project_path | All |
| `SessionEnd` | Session terminates | session_id, duration | All |
| `PreCompact` | Before context compaction | current_tokens, max_tokens | All |
| `Stop` | Session stop event | session_id | Claude Code only |
| `Notification` | System notification | message | All (fallback) |
| `Setup` | Initial setup event | configuration | Claude Code only |

### Deprecated Aliases

These event names still work in hooks.yaml but are deprecated. Use the new names instead.

| Deprecated Name | Use Instead | Notes |
|----------------|-------------|-------|
| `SubagentStart` | `BeforeAgent` | Serde alias, fully backward-compatible |
| `SubagentStop` | `AfterAgent` | Serde alias, fully backward-compatible |

### Platform Compatibility

Not all events fire on all platforms. See [platform-adapters.md](platform-adapters.md) for the full cross-platform mapping table and dual-fire behavior.

### Eval Context Variables

These variables are available in `enabled_when` expressions. Note: evalexpr uses underscores, not dots, as separators.

```yaml
# Available in all events
env_CI              # "true" if CI environment variable is set
env_USER            # Current username
env_HOME            # Home directory
tool_name           # "Write", "Bash", "Read", etc.
event_type          # "PreToolUse", "PostToolUse", etc.

# tool_input fields (prefixed with tool_input_)
tool_input_command  # Bash command string
tool_input_file_path # File path for file operations

# Examples
enabled_when: 'env_CI == "true"'
enabled_when: 'tool_name == "Bash"'
enabled_when: 'event_type == "PreToolUse"'
```

**Operators**: `==`, `!=`, `&&`, `||`, `!`, `>`, `<`, `>=`, `<=`

**Important:** evalexpr distinguishes `Float` from `Int`. Numbers from `tool_input` are exposed as Float. Use `30.0` not `30` in numeric comparisons, because `Float(30.0) != Int(30)`.

---

## Matchers Configuration

All matchers are optional. Multiple matchers use AND logic (all must match).

```yaml
matchers:
  operations: [PreToolUse]     # Filter by event type
  tools: [Tool, ...]           # Match specific tool names
  extensions: [.ext, ...]      # Match file extensions
  directories: [path/, ...]    # Match directory patterns (glob)
  command_match: "regex"       # Match Bash command
  prompt_match: "regex"        # Match user prompt
  require_fields: [field]      # Require fields in tool_input
  field_types:                 # Validate field types
    field_name: type
```

### operations

Array of event types to match (e.g. `[PreToolUse, PostToolUse]`).

```yaml
matchers:
  operations: [PreToolUse]
```

### tools

Array of tool names to match.

```yaml
matchers:
  tools: [Write, Edit, Read]   # Exact tool names
  tools: [Bash]                # Just Bash tool
```

**Valid tool names**: `Read`, `Write`, `Edit`, `Bash`, `Glob`, `Grep`, `Task`, `WebFetch`, `TodoRead`, `TodoWrite`

### extensions

Array of file extensions. Matches `file_path` in `tool_input`.

```yaml
matchers:
  extensions: [.py, .pyi]      # Python files
  extensions: [.js, .ts, .jsx, .tsx]  # JavaScript/TypeScript
```

### directories

Array of directory patterns using glob syntax (via `globset` crate).

```yaml
matchers:
  directories: [src/, lib/]    # Source directories
  directories: ["src/**"]      # All files under src/
  directories: ["**/*.rs"]     # All Rust files anywhere
```

### command_match

Regex pattern matched against full Bash command.

```yaml
matchers:
  command_match: "git push.*--force"     # Force push
  command_match: "rm -rf /"              # Dangerous delete
  command_match: "(?i)password"          # Case-insensitive
```

**Regex flavor**: Rust regex (similar to PCRE, no lookbehind)

### prompt_match

Pattern matching for user prompt text. Supports two formats:

#### Simple format (string or array)

```yaml
matchers:
  prompt_match: "(?i)deploy"             # Single regex
```

```yaml
matchers:
  prompt_match: ["deploy", "production"]  # ANY pattern matches (OR logic)
```

#### Complex object format

```yaml
matchers:
  prompt_match:
    patterns: ["deploy", "production"]
    mode: all              # "any" (OR, default) or "all" (AND)
    case_insensitive: true
    anchor: start          # "start", "end", or "contains" (default)
```

**Shorthand patterns:**

| Shorthand | Expansion | Description |
|-----------|-----------|-------------|
| `contains_word:deploy` | `\bdeploy\b` | Matches whole word only |
| `not:pattern` | Negation | Matches when pattern does NOT match |

### require_fields

Array of dot-notation field paths that must exist in `tool_input`.

```yaml
matchers:
  require_fields: ["file_path", "input.user.name"]
```

### field_types

Expected types for fields in `tool_input`. Keys are dot-notation paths, values are type specifiers. Implicitly requires the field to exist.

```yaml
matchers:
  field_types:
    file_path: string
    line_number: number
```

**Supported types**: `string`, `number`, `boolean`, `array`, `object`, `any`

---

## Actions Configuration

### inject

Inject a file's content into the AI assistant's context.

```yaml
actions:
  inject: .claude/context/standards.md
```

### inject_inline

Inject inline markdown content directly.

```yaml
actions:
  inject_inline: |
    ## Important Note
    Always follow these guidelines...
```

### inject_command

Inject output from a shell command.

```yaml
actions:
  inject_command: cat VERSION
```

### run

Execute a validator script. Supports two formats:

#### Simple format

```yaml
actions:
  run: .claude/validators/check.sh
```

#### Extended format with trust level

```yaml
actions:
  run:
    script: .claude/validators/check.sh
    trust: local    # "local" (default), "verified", or "untrusted"
```

**Script output format** (JSON to stdout):
```json
{
  "continue": true,
  "context": "Additional context for Claude",
  "reason": ""
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `continue` | boolean | Yes | `true` to proceed, `false` to block |
| `context` | string | No | Markdown injected into context |
| `reason` | string | No | Explanation if blocked |

### block

Unconditionally block the tool execution.

```yaml
actions:
  block: true
```

### block_if_match

Block if regex pattern matches in the command.

```yaml
actions:
  block_if_match: "(?i)(password|secret|api_key)"
```

### validate_expr

Evalexpr boolean expression for validation. `true` = allow, `false` = block.

```yaml
actions:
  validate_expr: 'len(tool_input_description) > 0'
```

### inline_script

Inline shell script for validation. Event JSON is passed on stdin. Exit code 0 = allow, non-zero = block.

```yaml
actions:
  inline_script: |
    #!/bin/bash
    jq -e '.tool_input.file_path' > /dev/null 2>&1
```

---

## Governance Schema

Optional provenance and documentation metadata attached to a rule.

```yaml
governance:
  author: "security-team"
  created_by: "react-skill@2.1.0"
  reason: "Prevent accidental data loss"
  confidence: high
  last_reviewed: "2026-03-01"
  ticket: "SEC-1234"
  tags: ["security", "git", "destructive"]
```

| Field | Type | Description |
|-------|------|-------------|
| `author` | string | Who authored this rule |
| `created_by` | string | Source that created this rule (e.g., a skill or automation) |
| `reason` | string | Why this rule exists |
| `confidence` | string | Confidence level: `high`, `medium`, or `low` |
| `last_reviewed` | string | ISO 8601 date of last review |
| `ticket` | string | Related ticket or issue reference |
| `tags` | array | Tags for categorization and filtering |

---

## Complete Example

```yaml
version: "1"

rules:
  # High priority: Block dangerous operations first
  - name: block-force-push
    description: Prevent force push to protected branches
    priority: 10
    mode: enforce
    matchers:
      operations: [PreToolUse]
      tools: [Bash]
      command_match: "git push.*(--force|-f).*main"
    actions:
      block: true
    governance:
      author: "platform-team"
      reason: "Protect main branch integrity"
      confidence: high
      tags: ["security"]

  # Medium priority: Inject context for code changes
  - name: python-standards
    priority: 50
    matchers:
      operations: [PreToolUse]
      tools: [Write, Edit]
      extensions: [.py]
    actions:
      inject: .claude/context/python-standards.md

  # Conditional: Only in CI
  - name: ci-strict-mode
    enabled_when: 'env_CI == "true"'
    matchers:
      operations: [PreToolUse]
      tools: [Bash]
    actions:
      run: .claude/validators/ci-check.sh

  # Session start: Load project context
  - name: load-project-context
    matchers:
      operations: [SessionStart]
    actions:
      inject: .claude/context/project-overview.md

  # Agent lifecycle: Inject policy before agent runs
  - name: agent-policy
    description: Inject project conventions before agent tasks
    matchers:
      operations: [BeforeAgent]
    actions:
      inject: .claude/context/agent-policy.md

settings:
  log_level: "info"
  script_timeout: 5
  fail_open: true
  debug_logs: false
```

---

## Engine Behavior (v2.0+)

These sections document internal engine behaviors that affect rule evaluation. No user configuration is required unless noted.

### Parallel Rule Evaluation

When 10 or more rules are enabled, RuleZ automatically switches to parallel matching using `tokio::join_all`. This improves evaluation latency for large rule sets.

- **Threshold:** `PARALLEL_THRESHOLD = 10` (compile-time constant)
- **Phase 1 (parallel):** All rule matchers are evaluated concurrently
- **Phase 2 (sequential):** Matched rule actions execute in priority order

Below the threshold, all evaluation is sequential. No user configuration is needed.

### Config Caching

`Config::from_file()` caches parsed configuration with mtime-based invalidation. If the hooks.yaml file has not been modified (same filesystem mtime), subsequent calls return the cached config without re-reading from disk.

- Caching is automatic and process-scoped (no cross-process shared state)
- Any file modification (save, touch) invalidates the cache immediately
- No user configuration is needed

### Globset Matching

The `directories` matcher uses the `globset` crate for file pattern matching instead of naive string contains. Patterns in `directories` fields support full glob syntax.

```yaml
matchers:
  directories:
    - "src/**"           # All files under src/
    - "**/*.rs"          # All Rust files anywhere
    - "tests/unit/**"    # Unit test files only
```

Internally, `build_glob_set()` compiles patterns into an optimized set. Multiple patterns use OR semantics (any pattern match satisfies the matcher). Invalid glob patterns are skipped with a warning.

### Regex Fail-Closed

Invalid regex patterns in matchers (`command_match`, `prompt_match`, `block_if_match`) cause the rule to **not match** rather than crashing or matching everything. This is a fail-closed safety design.

- `get_or_compile_regex()` returns an error for invalid patterns
- The calling matcher treats the error as "no match"
- A warning is logged for debugging
- This also applies to `enabled_when` expression errors and inline script failures

### tool_input Fields in Eval Context

`build_eval_context()` injects fields from the tool invocation's `tool_input` JSON object into the `enabled_when` evaluation context with a `tool_input_` prefix.

```yaml
# Rule-level enabled_when (NOT a matcher field)
enabled_when: 'tool_input_command == "git push"'
enabled_when: 'tool_input_file_path == "src/main.py"'
```

**Supported types:**

| JSON Type | evalexpr Type | Example |
|-----------|---------------|---------|
| string    | String        | `tool_input_command == "ls"` |
| boolean   | Boolean       | `tool_input_dangerouslyDisableSandbox == true` |
| number    | Float         | `tool_input_timeout > 30.0` |

**Important:** All numbers are exposed as `Float` (f64). Use `30.0` not `30` in comparisons, because `Float(30.0) != Int(30)` in evalexpr.

Arrays, objects, and null values in tool_input are skipped (not supported by evalexpr).

### External Logging

RuleZ supports forwarding audit logs to external backends via the `logging` section in the settings file. Backends send logs using `curl` (no TLS library dependency).

```yaml
settings:
  logging:
    backends:
      - type: otlp
        endpoint: "https://otel-collector.example.com/v1/logs"
        headers:
          Authorization: "Bearer $OTEL_TOKEN"
        timeout_secs: 5

      - type: datadog
        endpoint: "https://http-intake.logs.datadoghq.com/api/v2/logs"
        api_key: "$DD_API_KEY"
        timeout_secs: 5

      - type: splunk
        endpoint: "https://splunk.example.com:8088/services/collector"
        token: "$SPLUNK_HEC_TOKEN"
        sourcetype: "rulez"
        timeout_secs: 5
```

| Backend | Required Fields | Optional Fields |
|---------|----------------|-----------------|
| `otlp` | `endpoint` | `headers`, `timeout_secs` (default: 5) |
| `datadog` | `api_key` | `endpoint` (default: Datadog US), `timeout_secs` |
| `splunk` | `endpoint`, `token` | `sourcetype` (default: "rulez"), `timeout_secs` |

Environment variable references (e.g., `$DD_API_KEY`) are expanded at runtime.

---

## Validation

Validate your configuration:

```bash
rulez validate
```

Common validation errors:

| Error | Cause | Fix |
|-------|-------|-----|
| `unknown field` | Typo in field name | Check spelling |
| `invalid event type` | Wrong event name | Use exact event names from table above |
| `file not found` | Bad path in action | Verify file exists |
| `invalid regex` | Bad regex syntax | Test regex separately |
| `duplicate rule name` | Same name used twice | Use unique names |
