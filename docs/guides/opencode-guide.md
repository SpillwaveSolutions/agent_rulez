---
last_modified: 2026-03-16
last_validated: 2026-03-16
---

# RuleZ for OpenCode

A complete guide to using RuleZ with OpenCode. Covers installation, plugin setup, event mapping, verification, and troubleshooting.

## Overview

RuleZ integrates with OpenCode through its plugin-based hook system. When OpenCode fires a lifecycle event (such as `tool.execute.before`), RuleZ translates it into a unified event type and evaluates your rules. This lets you enforce policies, inject context, audit tool usage, and block dangerous operations -- all using the same `hooks.yaml` configuration you use with other AI coding assistants.

The integration works through a plugin architecture: OpenCode invokes `rulez opencode hook` via stdin/stdout, and RuleZ returns a JSON response indicating whether the action should proceed, be blocked, or have context injected.

## Prerequisites

- **RuleZ binary** (`rulez`) installed and available on your PATH
- **OpenCode** installed and configured

To verify both are available:

```bash
rulez --version
opencode --version
```

## Quick Start

Set up RuleZ for OpenCode in under 5 minutes:

### 1. Initialize configuration

```bash
rulez init
```

This creates `.claude/hooks.yaml` with default rules.

### 2. Install hooks

```bash
# Project scope (default) -- writes to .opencode/settings.json
rulez opencode install

# User scope -- writes to ~/.config/opencode/plugins/rulez-plugin/settings.json
rulez opencode install --scope user
```

### 3. Verify installation

```bash
rulez opencode doctor
```

You should see `OK` for the scope where you installed hooks.

### 4. Test a rule

```bash
rulez debug PreToolUse --tool Write --path test.py -v
```

This simulates a `PreToolUse` event and shows which rules match.

## Plugin Setup

OpenCode uses a plugin-based integration model. The RuleZ plugin lives at `opencode-plugin/rulez-plugin/` in the RuleZ repository and consists of a TypeScript plugin with its configuration.

### How It Works

```
OpenCode event  -->  stdin (JSON)  -->  rulez opencode hook  -->  RuleZ policy engine
                                                                        |
                                          stdout (JSON response)  <-----+
                                          exit 0 = allow
                                          exit 2 = deny
```

1. OpenCode triggers a lifecycle event (e.g., `tool.execute.before`)
2. The hook entry in `settings.json` invokes `rulez opencode hook`
3. RuleZ reads the event JSON from stdin and maps it to internal event types
4. Rules are evaluated against the event payload
5. A JSON response is emitted on stdout (allow, deny, or inject)
6. If denied, exit code 2 signals OpenCode to block the action

### Plugin Configuration

Config file location: `~/.config/opencode/plugins/rulez-plugin/settings.json`

```json
{
  "rulez_binary_path": "rulez",
  "audit_log_path": "~/.config/opencode/plugins/rulez-plugin/audit.log",
  "event_filters": []
}
```

| Field | Default | Description |
|---|---|---|
| `rulez_binary_path` | `"rulez"` | Path to the RuleZ binary |
| `audit_log_path` | `~/.config/opencode/plugins/rulez-plugin/audit.log` | JSONL audit log location |
| `event_filters` | `[]` | Event names to skip (e.g., `["session.updated"]`) |

### Environment Variable Overrides

| Variable | Overrides |
|---|---|
| `RULEZ_BINARY_PATH` | `rulez_binary_path` in plugin config |
| `RULEZ_AUDIT_LOG_PATH` | `audit_log_path` in plugin config |

### Settings File Format

The hook entries in `settings.json` look like this:

```json
{
  "hooks": {
    "tool.execute.before": [
      { "type": "command", "command": "rulez opencode hook", "timeout": 5 }
    ],
    "tool.execute.after": [
      { "type": "command", "command": "rulez opencode hook", "timeout": 5 }
    ],
    "session.updated": [
      { "type": "command", "command": "rulez opencode hook", "timeout": 5 }
    ],
    "file.edited": [
      { "type": "command", "command": "rulez opencode hook", "timeout": 5 }
    ]
  }
}
```

## Event Mapping Reference

Complete mapping of OpenCode native events to RuleZ event types:

| OpenCode Native Event | RuleZ Event Type | Notes |
|---|---|---|
| `tool.execute.before` | `PreToolUse` | Before a tool executes; can block or inject context |
| `tool.execute.after` | `PostToolUse` | After a tool executes; audit only |
| `tool.execute.after` (on fail) | `PostToolUseFailure` | Dual-fire when payload has `error` or `success: false` |
| `session.created` | `SessionStart` | New session begins |
| `session.deleted` | `SessionEnd` | Session ends |
| `session.updated` | `UserPromptSubmit` | Session state changed (e.g., new prompt submitted) |
| `session.compacted` | `PreCompact` | Before context compaction |
| `file.edited` | `Notification` | A file was edited; audit and context injection |

### Dual-Fire on Tool Failure

OpenCode has one dual-fire scenario: `tool.execute.after` fires both `PostToolUse` and `PostToolUseFailure` when the tool result indicates failure. Failure is detected when the event payload contains any of:

- `tool_input.success == false`
- `tool_input.error` field exists
- `extra.success == false`
- `extra.error` field exists

This lets you have general post-tool logging on `PostToolUse` and specific failure alerting on `PostToolUseFailure`.

## Configuration

RuleZ uses the same `hooks.yaml` configuration file across all platforms. Your existing rules work with OpenCode automatically.

### OpenCode-Tailored Example

```yaml
version: "1"

rules:
  - name: block-force-push
    description: "Block force push to main"
    matchers:
      operations: [PreToolUse]
      tools: [Bash]
      command_match: "git push.*--force.*main"
    actions:
      block: true

  - name: inject-standards
    description: "Inject coding standards for Python files"
    matchers:
      operations: [PreToolUse]
      tools: [Write, Edit]
      extensions: [.py]
    actions:
      inject: .claude/context/python-standards.md

  - name: audit-session-changes
    description: "Log all prompt submissions"
    mode: audit
    matchers:
      operations: [UserPromptSubmit]
    actions:
      inject_inline: "Audit: prompt submitted"
```

### Response Format

RuleZ returns JSON responses to OpenCode:

**Allow** (exit code 0):
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

**Deny** (exit code 2):
```json
{
  "continue": false,
  "reason": "Blocked by security policy: destructive command detected"
}
```

**Context injection** (exit code 0):
```json
{
  "continue": true,
  "context": "SECURITY NOTICE: This file contains sensitive credentials. Do not commit."
}
```

## Verifying Hooks Fire

### Doctor command

Check that hooks are installed correctly:

```bash
# Human-readable output
rulez opencode doctor

# Machine-readable JSON (for scripting)
rulez opencode doctor --json
```

Doctor checks each scope and reports:

| Status | Meaning |
|---|---|
| OK | RuleZ hook commands found and correctly configured |
| MISSING | Config file or hooks section not found |
| WARN | Hooks present but misconfigured or outdated |
| ERROR | Config file cannot be read or parsed |

### Debug command

Simulate events without waiting for real OpenCode activity:

```bash
# Test a PreToolUse event
rulez debug PreToolUse --tool Write --path src/main.py -v

# Test a Bash command rule
rulez debug pre --tool Bash --command "rm -rf /" -v

# JSON output for scripting
rulez debug pre --tool Write --path test.py --json
```

### Raw stdin testing

You can test the hook runner directly with a raw JSON event:

```bash
echo '{"session_id":"test","hook_event_name":"tool.execute.before","tool_name":"bash","tool_input":{"command":"echo hello"}}' | rulez opencode hook
```

### Logs command

View recent RuleZ activity:

```bash
# Last 10 entries
rulez logs

# Last 50 entries
rulez logs --limit 50

# Only blocked decisions
rulez logs --decision blocked
```

## Audit Logging

All RuleZ interactions are logged in JSONL format to the configured audit log path (default: `~/.config/opencode/plugins/rulez-plugin/audit.log`).

### Log Entry Format

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

### Audit Log Behavior

- **Non-blocking writes**: If the log file cannot be written (permissions, disk full), a warning is emitted to stderr and execution continues. Policy enforcement is never interrupted by logging failures.
- **JSONL format**: One JSON object per line, easy to parse with `jq` or other tools.
- **Configurable path**: Set via plugin config or `RULEZ_AUDIT_LOG_PATH` environment variable.

## Troubleshooting

### Hooks not firing

1. Run `rulez opencode doctor` to check installation status
2. Verify `settings.json` is in the correct location:
   - Project scope: `.opencode/settings.json` in your project root
   - User scope: `~/.config/opencode/plugins/rulez-plugin/settings.json`
3. Ensure `rulez` is on your PATH: `which rulez`
4. Reinstall if needed: `rulez opencode install`

### Plugin config issues

1. Check that the plugin settings file exists at `~/.config/opencode/plugins/rulez-plugin/settings.json`
2. Verify JSON is valid: `cat ~/.config/opencode/plugins/rulez-plugin/settings.json | python3 -m json.tool`
3. Confirm `rulez_binary_path` points to a valid binary
4. Check `event_filters` is not filtering out the events you expect to fire

### Audit log not written

1. Check the `RULEZ_AUDIT_LOG_PATH` environment variable
2. Verify the log directory exists and is writable:
   ```bash
   ls -la ~/.config/opencode/plugins/rulez-plugin/
   ```
3. Look for "Failed to write OpenCode audit log" warnings in stderr
4. Try writing a test file to the directory to confirm permissions

### Binary path issues

If OpenCode cannot find the `rulez` binary:

```bash
# Specify the full path during install
rulez opencode install --binary /path/to/rulez
```

### Outdated binary

If hooks are not behaving as expected:

```bash
# Check for updates
rulez upgrade --check

# Install the latest version
rulez upgrade

# Reinstall hooks after upgrade
rulez opencode install
```

## Further Reading

- [Platform Adapters Reference](../../mastering-hooks/references/platform-adapters.md) -- Cross-platform event mapping and dual-fire details
- [CLI Commands Reference](../../mastering-hooks/references/cli-commands.md) -- Complete command and flag reference
- [Hooks YAML Schema](../../mastering-hooks/references/hooks-yaml-schema.md) -- Configuration file format
- [Quick Reference](../../mastering-hooks/references/quick-reference.md) -- One-page cheat sheet
