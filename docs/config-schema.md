# RuleZ Configuration Schema

This document describes the `hooks.yaml` configuration format used by RuleZ. Configuration files are loaded from:

- **Project-level:** `.claude/hooks.yaml` (relative to `cwd` sent by the AI assistant)
- **Global:** `~/.claude/hooks.yaml`

Project-level configuration takes precedence. RuleZ uses mtime-based caching so the file is only re-parsed when its modification time changes.

## Top-level Structure

```yaml
version: "1"
rules:
  - name: rule-name
    # ... rule definition
settings:
  # ... global settings
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | string | Yes | Configuration format version. Use `"1"` or `"1.0"`. |
| `rules` | array | Yes | Array of [Rule](#rule-schema) objects defining policy enforcement logic. |
| `settings` | object | No | [Global settings](#settings-schema) for logging, timeouts, and behavior. |

## Rule Schema

Each rule defines a policy: what to match and what action to take.

```yaml
rules:
  - name: block-force-push
    description: "Prevent force push to protected branches"
    enabled_when: 'env_CI == "true"'
    mode: enforce
    priority: 10
    matchers:
      tools: ["Bash"]
      command_match: "git push.*(--force|-f).*(main|master)"
    actions:
      block: true
    governance:
      author: "security-team"
      reason: "Protect main branch integrity"
      confidence: high
      tags: ["security", "git"]
```

### Rule fields

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | string | Yes | -- | Unique identifier for the rule. Used in logs and debug output. |
| `description` | string | No | -- | Human-readable explanation of what the rule does. |
| `enabled_when` | string | No | -- | Evalexpr boolean expression. Rule is only active when this evaluates to true. See [Conditional Activation](#conditional-activation). |
| `matchers` | object | Yes | -- | Conditions that trigger the rule. See [Matchers](#matchers-schema). |
| `actions` | object | Yes | -- | Actions to take when the rule matches. See [Actions](#actions-schema). |
| `mode` | string | No | `"enforce"` | Policy mode: `enforce`, `warn`, or `audit`. See [Policy Modes](#policy-modes). |
| `priority` | integer | No | `0` | Evaluation priority. Higher numbers run first. |
| `governance` | object | No | -- | Provenance and documentation metadata. See [Governance](#governance-schema). |
| `metadata` | object | No | -- | Legacy metadata (deprecated, use `governance` instead). |

### Policy Modes

| Mode | Behavior |
|------|----------|
| `enforce` | Normal enforcement. Blocks, injects, or runs validators as configured. |
| `warn` | Never blocks. Injects warning context instead of blocking. |
| `audit` | Logs only. No blocking or context injection. |

### Conditional Activation

The `enabled_when` field accepts evalexpr expressions. Available context variables:

| Variable | Type | Description |
|----------|------|-------------|
| `env_*` | string | Any environment variable, prefixed with `env_`. Example: `env_CI`, `env_NODE_ENV`. |
| `tool_name` | string | Name of the tool being used. |
| `event_type` | string | The hook event type (e.g., `"PreToolUse"`). |
| `tool_input_*` | varies | Fields from `tool_input`, prefixed with `tool_input_`. Example: `tool_input_command`. |

Examples:

```yaml
# Only active in CI environments
enabled_when: 'env_CI == "true"'

# Only active for Bash tool
enabled_when: 'tool_name == "Bash"'

# Only active for PreToolUse events
enabled_when: 'event_type == "PreToolUse"'
```

**Note:** evalexpr uses `Float` vs `Int` types. Comparing `30.0` (float) with `30` (int) returns false. Use consistent types.

## Matchers Schema

Matchers define the conditions under which a rule fires. All specified matchers must match (AND logic). Omitted matchers are ignored (treated as wildcards).

```yaml
matchers:
  operations: ["PreToolUse"]
  tools: ["Bash", "Write"]
  extensions: [".rs", ".ts"]
  directories: ["src/**", "lib/**"]
  command_match: "git push.*--force"
  prompt_match: ["deploy", "production"]
  require_fields: ["file_path"]
  field_types:
    file_path: string
    line_number: number
```

### Matcher fields

| Field | Type | Description |
|-------|------|-------------|
| `operations` | array of strings | Event types to match (e.g., `["PreToolUse", "PostToolUse"]`). |
| `tools` | array of strings | Tool names to match (e.g., `["Bash", "Write", "Edit"]`). |
| `extensions` | array of strings | File extensions to match (e.g., `[".rs", ".py", ".ts"]`). Matched against `file_path` in `tool_input`. |
| `directories` | array of strings | Glob patterns for directory matching (e.g., `["src/**", "tests/**"]`). Uses globset for efficient multi-pattern matching. |
| `command_match` | string | Regex pattern matched against the `command` field in `tool_input`. |
| `prompt_match` | array or object | Pattern matching for user prompt text. See [Prompt Matching](#prompt-matching). |
| `require_fields` | array of strings | Field paths (dot notation) that must exist in `tool_input`. Example: `["file_path", "input.user.name"]`. |
| `field_types` | object | Expected types for `tool_input` fields. Keys are dot-notation paths, values are type names. See [Field Type Validation](#field-type-validation). |

### Prompt Matching

The `prompt_match` field supports two formats:

#### Simple array syntax

Matches if ANY pattern matches (OR logic), case-sensitive:

```yaml
matchers:
  prompt_match: ["deploy", "production", "release"]
```

#### Complex object syntax

Full control over matching behavior:

```yaml
matchers:
  prompt_match:
    patterns: ["deploy", "production"]
    mode: all           # "any" (OR, default) or "all" (AND)
    case_insensitive: true
    anchor: start       # "start", "end", or "contains" (default)
```

#### Shorthand patterns

| Shorthand | Expansion | Description |
|-----------|-----------|-------------|
| `contains_word:deploy` | `\bdeploy\b` | Matches whole word only. |
| `not:pattern` | Negation | Matches when pattern does NOT match. |

### Field Type Validation

Supported type specifiers for `field_types`:

| Type | Matches |
|------|---------|
| `string` | JSON string values |
| `number` | JSON number values (integer or float) |
| `boolean` | JSON `true` or `false` |
| `array` | JSON arrays |
| `object` | JSON objects |
| `any` | Any JSON type (only checks field existence) |

Field paths use dot notation, which is converted to JSON Pointer (RFC 6901) internally:

```yaml
field_types:
  file_path: string          # /file_path
  input.user.name: string    # /input/user/name
```

Specifying `field_types` implicitly requires the field to exist (no need to duplicate in `require_fields`).

## Actions Schema

Actions define what happens when a rule matches. Multiple actions can be specified on a single rule.

```yaml
actions:
  block: true
  inject: ".claude/context/standards.md"
  inject_inline: |
    ## Important
    Follow these coding standards.
  inject_command: "cat .claude/context/dynamic-rules.md"
  run: ".claude/validators/check-secrets.sh"
```

### Action fields

| Field | Type | Description |
|-------|------|-------------|
| `block` | boolean | If `true`, block the operation. Sets `continue: false` in the response. |
| `block_if_match` | string | Regex pattern. Block only if the pattern matches the command. |
| `inject` | string | Path to a file whose contents are injected as context. Relative paths are resolved from `cwd`. |
| `inject_inline` | string | Inline markdown content injected directly as context. No file read. |
| `inject_command` | string | Shell command to execute. Its stdout is injected as context. |
| `run` | string or object | Validator script to execute. See [Run Action](#run-action). |
| `validate_expr` | string | Evalexpr boolean expression. `true` = allow, `false` = block. |
| `inline_script` | string | Inline shell script for validation. Exit code 0 = allow, non-zero = block. Event JSON is passed on stdin. |

### Run Action

The `run` field supports two formats:

#### Simple format

```yaml
actions:
  run: .claude/validators/check-secrets.sh
```

#### Extended format with trust level

```yaml
actions:
  run:
    script: .claude/validators/check-secrets.sh
    trust: local    # "local" (default), "verified", or "untrusted"
```

The validator script receives the event JSON on stdin. Exit code 0 means validation passes (allow); non-zero means validation fails (block).

### Action examples

#### Block dangerous commands

```yaml
- name: block-force-push
  matchers:
    tools: ["Bash"]
    command_match: "git push.*(--force|-f).*(main|master)"
  actions:
    block: true
```

#### Inject context from file

```yaml
- name: python-standards
  matchers:
    tools: ["Write", "Edit"]
    extensions: [".py"]
  actions:
    inject: ".claude/context/python-standards.md"
```

#### Inject inline content

```yaml
- name: test-reminder
  matchers:
    tools: ["Write", "Edit"]
    directories: ["src/**"]
  actions:
    inject_inline: |
      **Reminder**: Source code modified. Run tests before committing.
```

#### Inject dynamic content from a command

```yaml
- name: inject-git-status
  matchers:
    tools: ["Bash"]
    command_match: "git commit"
  actions:
    inject_command: "git diff --stat"
```

#### Run a validator script

```yaml
- name: check-secrets
  matchers:
    tools: ["Bash"]
    command_match: "git commit"
  actions:
    run:
      script: .claude/validators/check-secrets.sh
      trust: local
```

#### Validate with an expression

```yaml
- name: require-description
  matchers:
    tools: ["Bash"]
  actions:
    validate_expr: 'len(tool_input_description) > 0'
```

#### Validate with an inline script

```yaml
- name: json-validation
  matchers:
    tools: ["Write"]
    extensions: [".json"]
  actions:
    inline_script: |
      #!/bin/bash
      jq -e '.version' > /dev/null 2>&1
```

#### Conditional blocking with regex

```yaml
- name: block-rm-rf-root
  matchers:
    tools: ["Bash"]
  actions:
    block_if_match: "rm\\s+(-rf|-fr)\\s+/"
```

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
| `author` | string | Who authored this rule. |
| `created_by` | string | Source that created this rule (e.g., a skill or automation). |
| `reason` | string | Why this rule exists. |
| `confidence` | string | Confidence level: `high`, `medium`, or `low`. |
| `last_reviewed` | string | ISO 8601 date of last review. |
| `ticket` | string | Related ticket or issue reference. |
| `tags` | array of strings | Tags for categorization and filtering. |

## Settings Schema

Global settings control RuleZ behavior across all rules.

```yaml
settings:
  log_level: "info"
  max_context_size: 1048576
  script_timeout: 5
  fail_open: true
  debug_logs: false
  logging:
    backends:
      - type: otlp
        endpoint: "http://localhost:4318/v1/logs"
```

### Settings fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `log_level` | string | `"info"` | Logging verbosity. Values: `error`, `warn`, `info`, `debug`, `trace`. |
| `max_context_size` | integer | `1048576` (1 MB) | Maximum size of injected context in bytes. Prevents accidentally injecting huge files. |
| `script_timeout` | integer | `5` | Default script execution timeout in seconds. Applies to `run` and `inline_script` actions. |
| `fail_open` | boolean | `true` | If `true`, errors during rule evaluation allow the operation to proceed. If `false`, errors block. |
| `debug_logs` | boolean | `false` | If `true`, log entries include full raw event JSON and per-rule evaluation details. |
| `logging` | object | -- | External logging backend configuration. See [Logging Backends](#logging-backends). |

### Logging Backends

RuleZ always writes to the local NDJSON log file (`~/.claude/logs/rulez.log`). Additionally, you can configure external backends to receive log entries.

```yaml
settings:
  logging:
    backends:
      - type: otlp
        endpoint: "http://localhost:4318/v1/logs"
        headers:
          Authorization: "Bearer ${OTEL_TOKEN}"
        timeout_secs: 5

      - type: datadog
        endpoint: "https://http-intake.logs.datadoghq.com/api/v2/logs"
        api_key: "${DD_API_KEY}"
        timeout_secs: 5

      - type: splunk
        endpoint: "https://splunk.example.com:8088/services/collector/event"
        token: "${SPLUNK_HEC_TOKEN}"
        sourcetype: "rulez"
        timeout_secs: 5
```

#### OTLP backend

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `type` | string | -- | Must be `"otlp"`. |
| `endpoint` | string | Required | OTLP HTTP endpoint URL. |
| `headers` | object | `{}` | Additional HTTP headers. Supports `${VAR}` environment variable expansion. |
| `timeout_secs` | integer | `5` | Request timeout in seconds. |

#### Datadog backend

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `type` | string | -- | Must be `"datadog"`. |
| `endpoint` | string | `"https://http-intake.logs.datadoghq.com/api/v2/logs"` | Datadog logs API endpoint. |
| `api_key` | string | Required | Datadog API key. Supports `${VAR}` expansion. |
| `timeout_secs` | integer | `5` | Request timeout in seconds. |

#### Splunk backend

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `type` | string | -- | Must be `"splunk"`. |
| `endpoint` | string | Required | Splunk HEC endpoint URL. |
| `token` | string | Required | Splunk HEC token. Supports `${VAR}` expansion. |
| `sourcetype` | string | `"rulez"` | Splunk sourcetype for events. |
| `timeout_secs` | integer | `5` | Request timeout in seconds. |

## Complete Example

```yaml
version: "1"

rules:
  # Security: block force push
  - name: block-force-push
    description: "Prevent force push to protected branches"
    priority: 10
    mode: enforce
    matchers:
      tools: ["Bash"]
      command_match: "git push.*(--force|-f).*(main|master)"
    actions:
      block: true
    governance:
      author: "platform-team"
      reason: "Protect main branch integrity"
      confidence: high
      tags: ["security"]

  # Code quality: inject Python standards
  - name: python-standards
    description: "Inject Python coding standards for .py files"
    matchers:
      tools: ["Write", "Edit"]
      extensions: [".py"]
    actions:
      inject: ".claude/context/python-standards.md"

  # CI-only: run secret scanner
  - name: secret-scanner
    description: "Check for secrets before git commit"
    enabled_when: 'env_CI == "true"'
    matchers:
      tools: ["Bash"]
      command_match: "git commit"
    actions:
      run: .claude/validators/check-secrets.sh

  # Audit: log all file writes (no blocking)
  - name: audit-writes
    description: "Log all file write operations"
    mode: audit
    matchers:
      tools: ["Write"]
    actions:
      inject_inline: "Audit: file write detected"

settings:
  log_level: "info"
  script_timeout: 10
  fail_open: true
  debug_logs: false
  logging:
    backends:
      - type: otlp
        endpoint: "http://localhost:4318/v1/logs"
```
