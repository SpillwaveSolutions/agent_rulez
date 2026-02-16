# RuleZ Manual Testing Runbooks

Hands-on runbooks for manually testing RuleZ across multiple AI coding tool platforms.

## Overview

These runbooks guide you through testing three core RuleZ capabilities:

1. **Block Force Push** - Security rule that blocks dangerous git operations
2. **Context Injection** - Automatically inject guidance when editing specific files
3. **Debug Dry-Run** - Use `rulez debug` to simulate events before going live

Each runbook targets a specific AI coding tool platform and walks through the same use cases so you can verify consistent behavior across tools.

## Runbooks

| Runbook | Platform | Status |
|---------|----------|--------|
| [01-claude-code.md](01-claude-code.md) | Claude Code | Ready |
| [02-opencode.md](02-opencode.md) | OpenCode | Ready |
| [03-gemini-cli.md](03-gemini-cli.md) | Gemini CLI | Ready |
| 04-copilot-cli.md | GitHub Copilot CLI | Planned |

## Prerequisites

### Build RuleZ

From the repository root:

```bash
cargo build --release
```

The binary will be at `target/release/rulez`.

### Verify the binary works

```bash
./target/release/rulez --version
./target/release/rulez --help
```

### Platform-specific requirements

| Platform | Requirement | Install Check |
|----------|-------------|---------------|
| Claude Code | Claude Code CLI installed | `claude --version` |
| OpenCode | OpenCode CLI installed | `opencode --version` |
| Gemini CLI | Gemini CLI installed | `gemini --version` |

### Test project directory

All runbooks use a **separate temporary test project** (not this repository). Each runbook includes instructions to create a fresh test directory, which keeps your repo clean and lets you experiment freely.

## Approach

Each use case follows the same pattern:

```
Setup --> Configure hooks.yaml --> Install RuleZ --> Dry-Run with `rulez debug` --> Live Test --> Verify --> Cleanup
```

1. **Setup** - Create a temp project directory with sample files
2. **Configure** - Write a `hooks.yaml` with rules for the use case
3. **Install** - Run `rulez install` (or platform-specific `rulez <platform> install`)
4. **Dry-Run** - Use `rulez debug` to simulate events and verify rules match
5. **Live Test** - Run the actual AI coding tool and trigger the rule
6. **Verify** - Check logs and tool behavior for expected outcomes
7. **Cleanup** - Remove the temp project

## hooks.yaml Schema Reference

Quick reference for writing test rules.

### Rule structure

```yaml
version: "1.0"

rules:
  - name: rule-name              # Unique identifier
    description: "What it does"  # Human-readable
    matchers:                    # When to trigger
      tools: ["Bash", "Edit"]   # Tool names
      command_match: "regex"     # Regex on command string
      directories: ["src/**"]   # Glob patterns on file paths
      extensions: [".rs"]       # File extension filter
      operations: ["write"]     # Operation types
      prompt_match:              # Match on user prompt text
        pattern: "regex"
      require_fields: ["field"]  # Required JSON fields in tool_input
      field_types:               # Type checks on tool_input fields
        field_name: "string"
    actions:                     # What to do
      block: true                # Block the operation
      inject: "path/to/file.md" # Inject file contents as context
      inject_inline: "markdown"  # Inject inline markdown
      inject_command: "cmd"      # Run command, inject stdout
      run:                       # Run a validator script
        command: "script.sh"
      block_if_match: "regex"    # Block only if regex matches
      validate_expr: "expr"      # evalexpr boolean expression
      inline_script: "script"    # Inline shell script validation

settings:
  log_level: "debug"             # info, debug, trace
  debug_logs: true               # Enable detailed logging
  fail_open: false               # Allow on error (default: false)
```

### Event types (Claude Code)

| Event | When | Common Matchers |
|-------|------|-----------------|
| PreToolUse | Before tool runs | `tools`, `command_match`, `directories` |
| PostToolUse | After tool runs | `tools` |
| SessionStart | Session begins | (none needed) |
| SessionEnd | Session ends | (none needed) |
| PermissionRequest | Tool needs permission | `tools` |
| UserPromptSubmit | User sends prompt | `prompt_match` |
| PreCompact | Before context compaction | (none needed) |

## Log file locations

- **Claude Code**: `~/.claude/logs/rulez.log`
- **OpenCode**: `~/.opencode/logs/rulez.log`
- **Gemini CLI**: `~/.gemini/logs/rulez.log`

## Related

- [Integration Test Suite](../test/integration/README.md) - Automated integration tests
- [RuleZ CLI Help](../rulez/README.md) - CLI reference
