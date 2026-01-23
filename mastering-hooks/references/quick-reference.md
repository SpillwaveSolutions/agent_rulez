# CCH Quick Reference

Fast lookup tables for events, matchers, actions, and file locations.

## Event Types

| Event | When Fired | Common Uses |
|-------|------------|-------------|
| `PreToolUse` | Before any tool executes | Inject context, validate inputs |
| `PostToolUse` | After tool completes | Log actions, trigger follow-ups |
| `PermissionRequest` | User asked to approve | Auto-approve/deny patterns |
| `UserPromptSubmit` | User sends message | Inject session context |
| `SessionStart` | New session begins | Load project context |
| `SessionEnd` | Session terminates | Cleanup, logging |
| `PreCompact` | Before context compaction | Preserve critical info |

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
| `inject` | Add context to Claude | `source`, `path`/`content` |
| `run` | Execute script | `command`, `timeout` |
| `block` | Prevent tool execution | `reason` |
| `block_if_match` | Conditional block | `pattern`, `reason` |
| `require_fields` | Validate inputs | `fields`, `message` |

## Response Format (for scripts)

Scripts must output valid JSON:
```json
{"continue": true, "context": "Additional info for Claude", "reason": ""}
```

| Field | Type | Purpose |
|-------|------|---------|
| `continue` | bool | `true` to proceed, `false` to block |
| `context` | string | Markdown injected into Claude's context |
| `reason` | string | Explanation if blocked |

## File Locations

```
project/
├── .claude/
│   ├── hooks.yaml          # Primary CCH configuration
│   ├── settings.json       # Claude Code settings (hooks registered here)
│   ├── context/            # Markdown files for injection
│   │   ├── python-standards.md
│   │   └── security-checklist.md
│   ├── validators/         # Custom validation scripts
│   │   └── check-secrets.sh
│   └── cch/
│       └── install.json    # CCH installation audit trail
```

## Common Commands

| Command | Purpose |
|---------|---------|
| `cch --version --json` | Check installation and API version |
| `cch init` | Create .claude/hooks.yaml |
| `cch validate` | Validate configuration |
| `cch install --project` | Register with Claude Code |
| `cch debug <event> --tool <name> -v` | Debug hook matching |
| `cch logs --tail 20` | View recent hook executions |
| `cch explain rule <name>` | Analyze specific rule |
| `cch explain config` | Overview all rules |
