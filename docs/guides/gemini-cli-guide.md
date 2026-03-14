# RuleZ for Gemini CLI

A complete guide to using RuleZ with the Gemini CLI. Covers installation, configuration, dual-fire events, verification, and troubleshooting.

## Overview

RuleZ is a policy engine that intercepts Gemini CLI tool invocations and applies user-defined YAML rules. When Gemini fires a lifecycle event (such as `BeforeTool` or `AfterTool`), RuleZ translates it into a unified event type and evaluates your rules against it. This adapter-based approach means you write rules once using RuleZ event types and the Gemini adapter handles all translation automatically.

RuleZ can block dangerous operations, inject context into the session, audit tool usage, and enforce coding standards -- all without modifying Gemini CLI itself.

## Prerequisites

- **RuleZ binary** (`rulez`) installed and available on your PATH
- **Gemini CLI** installed and configured

To verify both are available:

```bash
rulez --version
gemini --version
```

## Quick Start

Set up RuleZ for Gemini CLI in under 5 minutes:

### 1. Initialize configuration

```bash
rulez init
```

This creates `.claude/hooks.yaml` with default rules. You can customize rules later.

### 2. Install hooks

```bash
# Project scope (default) -- writes to .gemini/settings.json
rulez gemini install

# User scope -- writes to ~/.gemini/settings.json
rulez gemini install --scope user
```

### 3. Verify installation

```bash
rulez gemini doctor
```

You should see `OK` for the scope where you installed hooks.

### 4. Test a rule

```bash
rulez debug PreToolUse --tool Write --path test.py -v
```

This simulates a `PreToolUse` event for writing to `test.py` and shows which rules match.

## Understanding Dual-Fire Events

Dual-fire is a Gemini-specific behavior where a single Gemini event maps to **multiple** RuleZ event types. When dual-fire occurs, rules for all mapped types are evaluated. If any rule blocks, the event is blocked.

### Dual-Fire Scenarios

| Gemini Event | Primary RuleZ Type | Also Fires | Condition |
|---|---|---|---|
| `BeforeAgent` | `BeforeAgent` | `UserPromptSubmit` | Always (payload contains `prompt`) |
| `AfterTool` | `PostToolUse` | `PostToolUseFailure` | When the tool result indicates failure |
| `Notification` | `Notification` | `PermissionRequest` | When `notification_type` is `"ToolPermission"` |

### Practical Implications

**Avoid duplicate context injection.** If you have rules on both `BeforeAgent` and `UserPromptSubmit` that inject context, both will fire when Gemini sends a `BeforeAgent` event. This can cause the same context to appear twice in the session. To avoid this:

- Use only one of `BeforeAgent` or `UserPromptSubmit` for context injection, not both
- Or add conditions to your rules so they inject different context for each event type

**Tool failure handling.** If you have rules on both `PostToolUse` and `PostToolUseFailure`, both will fire when a tool fails. This is often useful -- you can have general post-tool logging on `PostToolUse` and specific failure alerting on `PostToolUseFailure`.

**Permission requests.** The `PermissionRequest` event type is only available on Gemini through dual-fire from `Notification` when the notification type is `"ToolPermission"`. Write `PermissionRequest` rules to intercept tool permission prompts.

### What Triggers Failure Detection?

RuleZ detects tool failures when the event payload contains any of:

- `tool_input.success == false`
- `tool_input.error` field exists
- `extra.success == false`
- `extra.error` field exists

## Event Mapping Reference

Complete mapping of Gemini native events to RuleZ event types:

| Gemini Native Event | RuleZ Event Type | Notes |
|---|---|---|
| `BeforeTool` | `PreToolUse` | Before a tool executes; can block or inject context |
| `AfterTool` | `PostToolUse` | After a tool executes |
| `AfterTool` (on fail) | `PostToolUseFailure` | Dual-fire when tool result indicates failure |
| `BeforeAgent` | `BeforeAgent` | Before agent processes prompt |
| `BeforeAgent` (dual) | `UserPromptSubmit` | Dual-fire; always fires alongside `BeforeAgent` |
| `AfterAgent` | `AfterAgent` | After agent completes |
| `BeforeModel` | `BeforeModel` | Before model inference (Gemini-only) |
| `AfterModel` | `AfterModel` | After model inference (Gemini-only) |
| `BeforeToolSelection` | `BeforeToolSelection` | Before tool selection (Gemini-only) |
| `SessionStart` | `SessionStart` | Session begins |
| `SessionEnd` | `SessionEnd` | Session ends |
| `Notification` | `Notification` | General notifications |
| `Notification` (ToolPermission) | `PermissionRequest` | Dual-fire when `notification_type` = `"ToolPermission"` |
| `PreCompact` | `PreCompact` | Before context compaction |

**Gemini-exclusive events:** `BeforeModel`, `AfterModel`, and `BeforeToolSelection` are only available on Gemini CLI. Rules using these events will not fire on other platforms.

## Configuration

RuleZ uses the same `hooks.yaml` configuration file across all platforms. Your existing rules will work with Gemini CLI automatically.

### Gemini-Tailored Example

```yaml
hooks:
  - name: inject-project-context
    event: BeforeAgent
    description: "Inject project context when agent starts"
    matchers: []
    action:
      type: inject
      inject_command: "cat .claude/context/project-overview.md"

  - name: block-force-push
    event: PreToolUse
    description: "Block force push to main"
    matchers:
      - tools: [Bash]
        command_pattern: "git push.*--force.*main"
    action:
      type: block
      message: "Force push to main is prohibited by policy"

  - name: audit-model-calls
    event: BeforeModel
    description: "Log all model inference calls (Gemini-only)"
    matchers: []
    action:
      type: audit
```

Note that `BeforeModel` rules will only fire on Gemini CLI since no other platform emits this event.

### Settings Scope Priority

Gemini CLI loads settings in precedence order:

1. **Project**: `.gemini/settings.json` (highest priority)
2. **User**: `~/.gemini/settings.json`
3. **System** (platform-dependent):
   - macOS: `/Library/Application Support/Gemini/settings.json` or `/etc/gemini/settings.json`
   - Linux: `/etc/gemini/settings.json`
   - Windows: `%ProgramData%\Gemini\settings.json`

Project-scope settings override user-scope, which override system-scope.

## Verifying Hooks Fire

### Doctor command

Check that hooks are installed correctly:

```bash
# Human-readable output
rulez gemini doctor

# Machine-readable JSON (for scripting)
rulez gemini doctor --json
```

Doctor checks each scope and reports one of:

| Status | Meaning |
|---|---|
| OK | RuleZ hook commands were found in this scope |
| MISSING | Settings file or hooks section not found |
| WARN | Hooks present but none reference `rulez` (likely misconfigured) |
| ERROR | File exists but could not be read or parsed |

### Debug command

Simulate events without waiting for real Gemini activity:

```bash
# Test a PreToolUse event
rulez debug PreToolUse --tool Write --path src/main.py -v

# Test a BeforeAgent event
rulez debug beforeagent -v

# JSON output for scripting
rulez debug pre --tool Bash --command "rm -rf /" --json
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

## Troubleshooting

### Hooks not firing

1. Run `rulez gemini doctor` to check installation status across all scopes
2. Verify you installed hooks in the correct scope:
   - Project scope: `.gemini/settings.json` must exist in your project root
   - User scope: `~/.gemini/settings.json` must exist
3. Ensure `rulez` is on your PATH: `which rulez`
4. Reinstall if needed: `rulez gemini install`

### Dual-fire confusion

If context appears duplicated or rules fire unexpectedly:

1. Check if you have rules on both `BeforeAgent` and `UserPromptSubmit` -- both fire on every `BeforeAgent` event from Gemini
2. Use `rulez debug beforeagent -v` to see which rules match
3. Consolidate to one event type, or add conditions to differentiate

### Settings scope priority issues

If hooks seem to behave inconsistently:

1. Run `rulez gemini doctor` -- it checks all scopes
2. Remember that project-scope settings take priority over user-scope
3. If you have conflicting settings at different scopes, the project scope wins

### Binary path issues

If Gemini cannot find the `rulez` binary:

```bash
# Specify the full path during install
rulez gemini install --binary /path/to/rulez
```

### Outdated binary

If hooks are not behaving as expected:

```bash
# Check for updates
rulez upgrade --check

# Install the latest version
rulez upgrade

# Reinstall hooks after upgrade
rulez gemini install
```

### Extension hook issues

If you use Gemini extensions with hooks:

1. Confirm the extension is installed under `~/.gemini/extensions`
2. Check for `hooks/hooks.json` inside the extension folder
3. For shared hooks, ensure `~/.gemini/hooks` contains valid JSON hook files
4. After updating hook files, re-run `rulez gemini doctor` to verify

## Further Reading

- [Platform Adapters Reference](../../mastering-hooks/references/platform-adapters.md) -- Cross-platform event mapping and dual-fire details
- [CLI Commands Reference](../../mastering-hooks/references/cli-commands.md) -- Complete command and flag reference
- [Hooks YAML Schema](../../mastering-hooks/references/hooks-yaml-schema.md) -- Configuration file format
- [Quick Reference](../../mastering-hooks/references/quick-reference.md) -- One-page cheat sheet
