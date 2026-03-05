# hooks.yaml Schema Reference

Complete reference for the RuleZ configuration file format.

## File Location

```
.claude/hooks.yaml    # Project-level (recommended)
~/.claude/hooks.yaml  # User-level (global)
```

## Top-Level Structure

```yaml
version: "1.0"                  # Required: Schema version (must be x.y format)
rules: []                       # Required: List of hook rules
```

## Rule Schema

```yaml
rules:
  - name: string                # Required: Unique kebab-case identifier
    description: string         # Optional: Human-readable explanation
    metadata:
      enabled: boolean          # Optional: Default true
    priority: integer           # Optional: Higher number = higher priority (default: 100)
    matchers:                   # Required: Conditions to match
      operations: [EventType]   # Filter by event type (e.g. [PreToolUse])
      tools: [ToolName]         # Filter by tool name
      command_match: "regex"    # Filter by Bash command
    actions:                    # Required: What to do when matched
      block: true
      reason: "explanation"
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

### Event Context Variables

Access in `enabled_when` expressions:

```yaml
# PreToolUse / PostToolUse
tool.name           # "Write", "Bash", "Read", etc.
tool.input.path     # File path for file operations
tool.input.command  # Command for Bash tool
tool.output         # Only in PostToolUse

# UserPromptSubmit
prompt.text         # Full user message

# SessionStart / SessionEnd
session.id          # Unique session identifier
session.project     # Project directory path

# Environment (all events)
env.CI              # "true" if in CI environment
env.USER            # Current username
env.HOME            # Home directory
```

---

## Matchers Configuration

All matchers are optional. Multiple matchers use AND logic (all must match).

```yaml
matchers:
  operations: [PreToolUse]     # Filter by event type
  tools: [Tool, ...]           # Match specific tool names
  extensions: [.ext, ...]      # Match file extensions
  directories: [path/, ...]    # Match directory prefixes
  command_match: "regex"       # Match Bash command
  prompt_match: "regex"        # Match user prompt
  enabled_when: "expression"   # Conditional expression
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

Array of file extensions. Matches `tool.input.path`.

```yaml
matchers:
  extensions: [.py, .pyi]      # Python files
  extensions: [.js, .ts, .jsx, .tsx]  # JavaScript/TypeScript
```

### directories

Array of directory prefixes. Uses forward slash.

```yaml
matchers:
  directories: [src/, lib/]    # Source directories
  directories: [tests/]        # Test directory only
```

### operations (Bash operations)

Array of Bash command prefixes. Extracts first word of command.

```yaml
matchers:
  operations: [git, npm, docker]  # Version control, package, container
  operations: [rm, mv, cp]        # File operations
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

Regex pattern matched against user prompt text.

```yaml
matchers:
  prompt_match: "(?i)deploy"             # Deploy requests
  prompt_match: "^/fix"                  # Slash commands
```

### enabled_when

Conditional expression for dynamic matching.

```yaml
matchers:
  enabled_when: "env.CI == 'true'"              # Only in CI
  enabled_when: "tool.input.path =~ '\\.test\\.'"  # Test files
  enabled_when: "session.project =~ 'backend'"  # Backend projects
```

**Operators**: `==`, `!=`, `=~` (regex match), `&&`, `||`, `!`

---

## Actions Configuration

### inject

Inject markdown content into the AI assistant's context.

```yaml
actions:
  inject: |
    ## Important Note
    Always follow these guidelines...
```

Or inject from a file:

```yaml
actions:
  inject_file: .claude/context/standards.md
```

Or inject from a command:

```yaml
actions:
  inject_command: cat VERSION
```

### run

Execute a script and use its output.

```yaml
actions:
  run: .claude/validators/check.sh
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
  reason: "This operation is not allowed in this project"
```

### block_if_match

Block if pattern matches in tool input.

```yaml
actions:
  block_if_match: "(?i)(password|secret|api_key)"
  reason: "Potential secret detected in file content"
```

---

## Complete Example

```yaml
version: "1.0"

rules:
  # High priority: Block dangerous operations first
  - name: block-force-push
    description: Prevent force push to protected branches
    priority: 10
    matchers:
      operations: [PreToolUse]
      tools: [Bash]
      command_match: "git push.*(--force|-f).*main"
    actions:
      block: true
      reason: "Force push to main is prohibited. Use a PR workflow."

  # Medium priority: Inject context for code changes
  - name: python-standards
    priority: 50
    matchers:
      operations: [PreToolUse]
      tools: [Write, Edit]
      extensions: [.py]
    actions:
      inject_file: .claude/context/python-standards.md

  # Conditional: Only in CI
  - name: ci-strict-mode
    matchers:
      operations: [PreToolUse]
      tools: [Bash]
      enabled_when: "env.CI == 'true'"
    actions:
      run: .claude/validators/ci-check.sh

  # Session start: Load project context
  - name: load-project-context
    matchers:
      operations: [SessionStart]
    actions:
      inject_file: .claude/context/project-overview.md

  # Agent lifecycle: Inject policy before agent runs
  - name: agent-policy
    description: Inject project conventions before agent tasks
    matchers:
      operations: [BeforeAgent]
    actions:
      inject_file: .claude/context/agent-policy.md
```

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
