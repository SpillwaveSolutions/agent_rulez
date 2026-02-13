# OpenCode CLI Hooks Integration

RuleZ integrates with OpenCode through its hook system, enabling policy enforcement, context injection, and audit logging for OpenCode sessions.

## Prerequisites

- RuleZ CLI (`cch`) built and available on PATH
- OpenCode installed and configured
- A `hooks.yaml` file in your project (or global config)

## Quick Start

```bash
# Install hooks for the current project
cch opencode install --scope project

# Verify installation
cch opencode doctor

# Test with a sample event
echo '{"session_id":"test","hook_event_name":"tool.execute.before","tool_name":"bash","tool_input":{"command":"echo hello"}}' | cch opencode hook
```

## How It Works

```
OpenCode event  -->  stdin (JSON)  -->  cch opencode hook  -->  RuleZ policy engine
                                                                      |
                                            stdout (JSON response)  <--+
                                            exit 0 = allow
                                            exit 2 = deny
```

1. OpenCode triggers a lifecycle event (e.g., `tool.execute.before`)
2. The hook entry in `settings.json` invokes `cch opencode hook`
3. RuleZ reads the event JSON from stdin, maps it to internal event types
4. Rules are evaluated against the event payload
5. A JSON response is emitted on stdout (allow/deny/inject)
6. If denied, exit code 2 signals OpenCode to block the action

## Installation

### Project Scope (Recommended)

```bash
cch opencode install --scope project
```

Writes hooks to `.opencode/settings.json` in the project root.

### User Scope

```bash
cch opencode install --scope user
```

Writes hooks to `~/.config/opencode/plugins/rulez-plugin/settings.json`.

### Dry Run

```bash
cch opencode install --print
```

Prints the JSON snippet without writing to disk.

### Custom Binary Path

```bash
cch opencode install --binary /path/to/cch
```

## Configuration

### Settings File Format

```json
{
  "hooks": {
    "file.edited": [
      { "type": "command", "command": "cch opencode hook", "timeout": 5 }
    ],
    "tool.execute.before": [
      { "type": "command", "command": "cch opencode hook", "timeout": 5 }
    ],
    "tool.execute.after": [
      { "type": "command", "command": "cch opencode hook", "timeout": 5 }
    ],
    "session.updated": [
      { "type": "command", "command": "cch opencode hook", "timeout": 5 }
    ]
  }
}
```

### Plugin Configuration

Config file: `~/.config/opencode/plugins/rulez-plugin/settings.json`

```json
{
  "rulez_binary_path": "cch",
  "audit_log_path": "~/.config/opencode/plugins/rulez-plugin/audit.log",
  "event_filters": []
}
```

| Field | Default | Description |
|-------|---------|-------------|
| `rulez_binary_path` | `"cch"` | Path to the RuleZ binary |
| `audit_log_path` | `~/.config/opencode/plugins/rulez-plugin/audit.log` | JSONL audit log location |
| `event_filters` | `[]` | Event names to skip (e.g., `["session.updated"]`) |

### Environment Variable Overrides

| Variable | Overrides |
|----------|-----------|
| `RULEZ_BINARY_PATH` | `rulez_binary_path` in config |
| `RULEZ_AUDIT_LOG_PATH` | `audit_log_path` in config |

## Hook Events

| OpenCode Event | RuleZ Event Type | Description |
|----------------|------------------|-------------|
| `tool.execute.before` | `PreToolUse` | Before a tool executes; can block or inject context |
| `tool.execute.after` | `PostToolUse` | After a tool executes; audit only, does not block |
| `file.edited` | `Notification` | A file was edited; audit and context injection |
| `session.updated` | `Notification` | Session state changed; audit only |

## Response Format

### Allow (continue execution)

```json
{
  "continue": true,
  "reason": "Policy check passed",
  "context": "Optional context to inject into session",
  "tools": [
    { "name": "rulez.check", "description": "Run a RuleZ policy check on demand" },
    { "name": "rulez.explain", "description": "Explain why a policy decision was made" }
  ]
}
```

Exit code: `0`

### Deny (block execution)

```json
{
  "continue": false,
  "reason": "Blocked by security policy: destructive command detected"
}
```

Exit code: `2`

### Context Injection

When a rule has `inject_inline` or `inject_command`, the response includes a `context` field that OpenCode can use to augment the session:

```json
{
  "continue": true,
  "context": "SECURITY NOTICE: This file contains sensitive credentials. Do not commit."
}
```

## Tool Registration

Responses include tool definitions that OpenCode can register for on-demand policy checks:

| Tool | Description |
|------|-------------|
| `rulez.check` | Run a RuleZ policy check on demand |
| `rulez.explain` | Explain why a policy decision was made |

## Audit Logging

All RuleZ interactions are logged in JSONL format to the configured audit log path.

### Entry Format

```json
{
  "timestamp": "2026-02-13T10:00:00Z",
  "event_id": "550e8400-e29b-41d4-a716-446655440000",
  "event_name": "tool.execute.before",
  "decision": "allow",
  "reason": null,
  "latency_ms": 5,
  "plugin_name": "rulez-plugin",
  "plugin_version": "1.0.2",
  "session_id": "abc-123"
}
```

Audit logging is non-blocking: if the log file cannot be written, a warning is emitted to stderr and execution continues.

## Doctor Command

Diagnose hook installation across project and user scopes:

```bash
# Human-readable output
cch opencode doctor

# Machine-readable JSON
cch opencode doctor --json
```

The doctor checks:
- Whether config files exist at project and user scope
- Whether `hooks` sections contain cch command entries
- Whether hooks reference the correct `cch opencode hook` command
- Whether outdated cch commands are present

### Status Codes

| Status | Meaning |
|--------|---------|
| OK | Hooks installed correctly |
| MISSING | Config file or hooks section not found |
| WARN | Hooks present but misconfigured or outdated |
| ERROR | Config file cannot be read or parsed |

## Troubleshooting

### Hooks not firing

1. Run `cch opencode doctor` to check installation
2. Verify `settings.json` is in the correct location
3. Ensure `cch` is on PATH or use `--binary` to specify the path
4. Check that OpenCode reads hooks from the expected config path

### Policy not enforced

1. Verify `hooks.yaml` exists in the project root
2. Run `cch debug PreToolUse --tool Bash --command "test"` to test rules locally
3. Check stderr output for parsing errors

### Audit log not written

1. Check `RULEZ_AUDIT_LOG_PATH` environment variable
2. Verify the log directory exists and is writable
3. Check stderr for "Failed to write OpenCode audit log" warnings

### Outdated binary

If `cch opencode doctor` shows "WARN" with outdated entries:

1. Rebuild cch: `cd cch_cli && cargo build --release`
2. Reinstall hooks: `cch opencode install --scope project`
3. Re-run doctor: `cch opencode doctor`
