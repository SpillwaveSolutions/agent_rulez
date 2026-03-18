---
last_modified: 2026-03-16
last_validated: 2026-03-16
---

# RuleZ for Claude Code -- Usage Guide

A complete guide to installing, configuring, verifying, and troubleshooting RuleZ with Claude Code.

## Overview

RuleZ is a high-performance AI policy engine for development workflows. It intercepts Claude Code tool invocations via hooks and applies user-defined YAML rules for policy enforcement, context injection, and audit logging.

With RuleZ, you define declarative rules in a `hooks.yaml` file that control what Claude Code can and cannot do. Rules can block dangerous operations (like force-pushing to main), inject project standards into Claude's context (like Python coding guidelines when editing `.py` files), and log every tool invocation for audit purposes. RuleZ evaluates rules in microseconds, so there is no noticeable latency added to your Claude Code sessions.

## Prerequisites

Before you begin, ensure you have:

1. **Claude Code** installed and working ([claude.ai/code](https://claude.ai/code))
2. **RuleZ binary** on your PATH -- download from [GitHub Releases](https://github.com/SpillwaveSolutions/rulez/releases) or run `rulez upgrade` if you already have an older version

Verify your installation:

```bash
rulez --version
```

## Quick Start

Get RuleZ running with Claude Code in under 5 minutes.

### Step 1: Initialize configuration

Run `rulez init` inside your project directory. This creates a `.claude/hooks.yaml` file with example rules.

```bash
cd your-project/
rulez init
```

To include example context files and validators:

```bash
rulez init --with-examples
```

This creates:

```
.claude/
├── hooks.yaml           # Main configuration
└── context/
    └── .gitkeep         # Placeholder for context files
```

### Step 2: Install hooks

Register the RuleZ hook with Claude Code so it fires on every tool invocation.

```bash
# Project-local (recommended for team projects)
rulez install

# Global (applies to all projects)
rulez install --global
```

### Step 3: Verify installation

Confirm hooks are registered in Claude Code's settings:

```bash
cat .claude/settings.json | grep -A5 hooks
```

You should see hook entries pointing to the `rulez` binary.

### Step 4: Test a rule

Simulate a `PreToolUse` event to verify rules match correctly:

```bash
rulez debug PreToolUse --tool Write --path test.py -v
```

The `-v` (verbose) flag shows which rules matched and why. You should see output showing rule evaluation details.

## Configuration

RuleZ rules are defined in `.claude/hooks.yaml`. Each rule specifies an event to listen for, matchers to filter which invocations it applies to, and an action to take.

Here is a practical example with three rules:

```yaml
version: "1"

rules:
  - name: block-force-push
    description: "Prevent force push to main branch"
    matchers:
      operations: [PreToolUse]
      tools: [Bash]
      command_match: "git\\s+push\\s+--force.*main"
    actions:
      block: true

  - name: python-standards
    description: "Inject Python coding standards on .py file writes"
    matchers:
      operations: [PreToolUse]
      tools: [Write, Edit]
      extensions: [.py]
    actions:
      inject: .claude/context/python-standards.md

  - name: warn-large-files
    description: "Warn when editing files over 500 lines"
    matchers:
      operations: [PreToolUse]
      tools: [Write, Edit]
    actions:
      run: |
        LINE_COUNT=$(wc -l < "$TOOL_INPUT_FILE_PATH" 2>/dev/null || echo 0)
        if [ "$LINE_COUNT" -gt 500 ]; then
          echo '{"continue": true, "context": "WARNING: This file has '"$LINE_COUNT"' lines. Consider splitting it."}'
        else
          echo '{"continue": true}'
        fi
```

For the full schema reference, see [hooks-yaml-schema.md](../../mastering-hooks/references/hooks-yaml-schema.md).

### Matcher types

| Matcher | Purpose | Example |
|---------|---------|---------|
| `tools` | Match tool names | `[Write, Edit, Bash]` |
| `extensions` | Match file extensions | `[.py, .js, .ts]` |
| `directories` | Match path prefixes | `[src/, tests/]` |
| `operations` | Filter by event type | `[PreToolUse, PostToolUse]` |
| `command_match` | Regex on command text | `"rm -rf.*"` |
| `prompt_match` | Regex on user input | `"(?i)deploy"` |
| `enabled_when` | Conditional expression | `'env_CI == "true"'` |

### Action types

| Action | Purpose |
|--------|---------|
| `block` | Unconditionally block the tool invocation |
| `block_if_match` | Block if a regex matches in tool input |
| `inject` | Inject content from a file into Claude's context |
| `inject_inline` | Inject inline markdown into Claude's context |
| `inject_command` | Inject shell command output into context |
| `run` | Execute a script and use its JSON output |

## Verifying Hooks Fire

After configuring rules, verify they work correctly using these tools.

### Simulate events with debug

Use `rulez debug` to test rule matching without waiting for real Claude Code usage:

```bash
# Test a Write tool on a Python file
rulez debug PreToolUse --tool Write --path src/main.py -v

# Test a Bash command
rulez debug pre --tool Bash --command "git push --force origin main" -v

# Test a user prompt event
rulez debug prompt --prompt "Deploy to production" -v

# Get JSON output for scripting
rulez debug pre --tool Write --path test.py --json
```

The verbose output shows each rule, whether it matched, and why:

```
Debugging PreToolUse event
---
Simulated context:
  tool.name: Write
  tool.input.path: src/main.py

Rule matching:
  [SKIP] block-force-push
    - tools: [Bash] does not match Write
  [MATCH] python-standards
    - tools: [Write, Edit] matches Write
    - extensions: [.py] matches .py

Matched rules: 1
  1. python-standards (priority: 50)
     Action: inject from .claude/context/python-standards.md
```

### Check logs after real usage

After using Claude Code with RuleZ installed, check the audit log:

```bash
# Last 20 log entries
rulez logs --limit 20

# Only blocked invocations
rulez logs --decision blocked

# Logs since a specific time
rulez logs --since 2026-03-14T00:00:00Z
```

### Explain rule behavior

Understand how a specific rule is configured and when it fires:

```bash
# Explain a single rule
rulez explain rule python-standards

# List all configured rules
rulez explain rules
```

### Batch-test rules

Create a test file to validate multiple scenarios at once:

```yaml
# tests.yaml
tests:
  - name: "Block force push"
    event_type: PreToolUse
    tool: Bash
    command: "git push --force origin main"
    expected: block

  - name: "Inject Python standards"
    event_type: PreToolUse
    tool: Write
    path: "app.py"
    expected: inject

  - name: "Allow normal read"
    event_type: PreToolUse
    tool: Read
    path: "README.md"
    expected: allow
```

Run the tests:

```bash
rulez test tests.yaml

# With detailed output
rulez test tests.yaml --verbose
```

`rulez test` exits with code 1 if any test fails, making it suitable for CI pipelines.

## Uninstalling

Remove RuleZ hooks from Claude Code:

```bash
# Remove project-local hooks
rulez uninstall

# Remove global hooks
rulez uninstall --global
```

This removes hook entries from `settings.json` but leaves your `hooks.yaml` configuration intact.

## Troubleshooting

### Hooks not firing

1. **Check settings.json** -- Verify hooks are registered:
   ```bash
   cat .claude/settings.json | grep -A5 hooks
   ```
2. **Re-run install** -- If hooks are missing, re-register them:
   ```bash
   rulez install
   ```
3. **Check scope** -- If you installed globally but are checking project settings (or vice versa), the hooks may be in a different `settings.json`.

### Wrong rules matching (or not matching)

Use `rulez debug` with the `-v` flag to see exactly which rules match and why:

```bash
rulez debug PreToolUse --tool Write --path src/handler.py -v
```

Check the output for `[SKIP]` and `[MATCH]` indicators to understand the evaluation.

### Configuration validation errors

Validate your `hooks.yaml` for syntax and schema errors:

```bash
rulez validate
```

To validate a specific config file:

```bash
rulez validate --config /path/to/hooks.yaml
```

### Checking logs

View recent RuleZ execution logs to understand what happened:

```bash
# Last 20 entries
rulez logs --limit 20

# Filter by decision type
rulez logs --decision blocked
rulez logs --decision allowed

# Filter by mode
rulez logs --mode enforce
```

### Binary not found

If Claude Code reports that the rulez binary is not found:

1. **Verify PATH** -- Ensure `rulez` is on your system PATH:
   ```bash
   which rulez
   ```
2. **Specify binary path explicitly** -- If `rulez` is installed in a non-standard location:
   ```bash
   rulez install --binary /path/to/rulez
   ```

### Rule quality issues

Use `rulez lint` to check for common configuration problems:

```bash
rulez lint
```

This detects duplicate rule names, overlapping rules, dead (disabled) rules, missing descriptions, and other issues. Use `--verbose` for detailed analysis:

```bash
rulez lint --verbose
```

## Further Reading

- [CLI Commands Reference](../../mastering-hooks/references/cli-commands.md) -- Full reference for all RuleZ commands and flags
- [hooks-yaml-schema.md](../../mastering-hooks/references/hooks-yaml-schema.md) -- Complete configuration schema reference
- [Quick Reference](../../mastering-hooks/references/quick-reference.md) -- Cheat sheet for events, matchers, actions, and file locations
- [Rule Patterns](../../mastering-hooks/references/rule-patterns.md) -- Ready-to-use rule examples for common scenarios
- [Troubleshooting Guide](../../mastering-hooks/references/troubleshooting-guide.md) -- Advanced troubleshooting and diagnostics
