# RuleZ Quick Reference

Fast lookup tables for events, matchers, actions, and file locations.

## Event Types

| Event | When Fired | Common Uses | Platforms |
|-------|------------|-------------|-----------|
| `PreToolUse` | Before any tool executes | Inject context, validate inputs | All |
| `PostToolUse` | After tool completes | Log actions, trigger follow-ups | All |
| `PostToolUseFailure` | After tool fails | Error logging, fallback actions | All |
| `PermissionRequest` | User asked to approve | Auto-approve/deny patterns | Claude Code, Gemini (dual) |
| `UserPromptSubmit` | User sends message | Inject session context | All |
| `BeforeAgent` | Agent/subagent launched | Track/control agent activity | Claude Code, Gemini (dual) |
| `AfterAgent` | Agent/subagent completed | Agent completion logging | Claude Code, Gemini |
| `BeforeModel` | Before model inference | Model-level policies | Gemini only |
| `AfterModel` | After model inference | Response monitoring | Gemini only |
| `BeforeToolSelection` | Before tool selection | Tool filtering | Gemini only |
| `SessionStart` | New session begins | Load project context | All |
| `SessionEnd` | Session terminates | Cleanup, logging | All |
| `PreCompact` | Before context compaction | Preserve critical info | All |
| `Stop` | Session stop event | Cleanup, final logging | Claude Code only |
| `Notification` | System notification | System event tracking | All (fallback) |
| `Setup` | Initial setup event | Configuration loading | Claude Code only |

**Deprecated aliases**: `SubagentStart` (use `BeforeAgent`), `SubagentStop` (use `AfterAgent`)

## Platform Support

| Platform | Adapter | Dual-Fire | Notes |
|----------|---------|-----------|-------|
| Claude Code | Native | N/A | Full event support (16 types) |
| Gemini CLI | `gemini.rs` | Yes | BeforeAgent+UserPromptSubmit, AfterTool+PostToolUseFailure, Notification+PermissionRequest |
| GitHub Copilot | `copilot.rs` | No | 7 event types supported |
| OpenCode | `opencode.rs` | Yes | tool.execute.after+PostToolUseFailure |

See [platform-adapters.md](platform-adapters.md) for full mapping table.

## Matcher Types

| Matcher | Applies To | Example |
|---------|-----------|---------|
| `tools` | Tool name | `[Write, Edit, Bash]` |
| `extensions` | File extension | `[.py, .js, .ts]` |
| `directories` | Path prefix | `[src/, tests/]` |
| `operations` | Bash operations | `[git, npm, docker]` |
| `command_match` | Regex on command | `"rm -rf.*"` |
| `prompt_match` | Regex on user input | `"(?i)deploy"` |
| `enabled_when` | Conditional expression | `"env.CI == 'true'"` |

## Action Types

| Action | Purpose | Key Fields |
|--------|---------|------------|
| `inject` | Inject file content into AI context | `path` |
| `inject_inline` | Inject inline markdown into AI context | `content` (string) |
| `inject_command` | Inject shell command output into context | `command` (string) |
| `run` | Execute script, use JSON output | `command`, `timeout` |
| `block` | Unconditionally block tool execution | `reason` |
| `block_if_match` | Block if regex matches in tool input | `pattern`, `reason` |

## Response Format (for scripts)

Scripts must output valid JSON:
```json
{"continue": true, "context": "Additional info for Claude", "reason": ""}
```

| Field | Type | Purpose |
|-------|------|---------|
| `continue` | bool | `true` to proceed, `false` to block |
| `context` | string | Markdown injected into AI assistant's context |
| `reason` | string | Explanation if blocked |

## File Locations

```
project/
├── .claude/
│   ├── hooks.yaml          # Primary RuleZ configuration
│   ├── settings.json       # Claude Code settings (hooks registered here)
│   ├── context/            # Markdown files for injection
│   │   ├── python-standards.md
│   │   └── security-checklist.md
│   ├── validators/         # Custom validation scripts
│   │   └── check-secrets.sh
│   └── rulez/
│       └── install.json    # RuleZ installation audit trail
```

## CLI Commands

| Command | Purpose |
|---------|---------|
| `rulez init` | Create .claude/hooks.yaml in current project |
| `rulez install --project` | Register RuleZ hook with Claude Code |
| `rulez uninstall` | Remove RuleZ hook from Claude Code |
| `rulez validate` | Validate hooks.yaml configuration |
| `rulez debug <event> --tool <name> -v` | Simulate event to test rule matching |
| `rulez repl` | Interactive debug mode (REPL) |
| `rulez logs --tail 20` | Query and display audit logs |
| `rulez explain rule <name>` | Analyze specific rule |
| `rulez explain config` | Overview all rules |
| `rulez test <file.yaml>` | Run batch test scenarios from YAML file |
| `rulez lint` | Analyze rule quality and detect issues |
| `rulez upgrade` | Check for and install newer binary releases |
| `rulez gemini install` | Install RuleZ for Gemini CLI |
| `rulez gemini hook` | Process Gemini CLI hook events |
| `rulez gemini doctor` | Diagnose Gemini CLI integration |
| `rulez copilot install` | Install RuleZ for GitHub Copilot |
| `rulez copilot hook` | Process Copilot hook events |
| `rulez copilot doctor` | Diagnose Copilot integration |
| `rulez opencode install` | Install RuleZ for OpenCode |
| `rulez opencode hook` | Process OpenCode hook events |
| `rulez opencode doctor` | Diagnose OpenCode integration |
| `rulez --version --json` | Check installation and API version |

### Global Options

| Option | Purpose |
|--------|---------|
| `--debug-logs` | Enable debug logging with full event and rule details |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Configuration error |
| 2 | Validation error |
| 3 | Runtime error |

## Debug Event Aliases

| Short Alias | Event Type |
|-------------|------------|
| `pre`, `pretooluse`, `pre-tool-use` | `PreToolUse` |
| `post`, `posttooluse`, `post-tool-use` | `PostToolUse` |
| `session`, `start`, `sessionstart` | `SessionStart` |
| `end`, `sessionend`, `session-end` | `SessionEnd` |
| `permission`, `perm`, `permissionrequest` | `PermissionRequest` |
| `prompt`, `user-prompt`, `userpromptsubmit` | `UserPromptSubmit` |
| `compact`, `precompact`, `pre-compact` | `PreCompact` |
| `subagent`, `beforeagent`, `before-agent`, `subagentstart` | `BeforeAgent` |
| `afteragent`, `after-agent`, `subagentstop` | `AfterAgent` |
| `idle`, `teammateidle` | `TeammateIdle` |
| `task`, `taskcompleted` | `TaskCompleted` |
