# hooks.yaml Schema Reference

Complete reference for the CCH configuration file format.

## File Location

```
.claude/hooks.yaml    # Project-level (recommended)
~/.claude/hooks.yaml  # User-level (global)
```

## Top-Level Structure

```yaml
version: "1"                    # Required: Schema version
hooks: []                       # Required: List of hook rules
```

## Hook Rule Schema

```yaml
hooks:
  - name: string                # Required: Unique kebab-case identifier
    event: EventType            # Required: When to trigger
    description: string         # Optional: Human-readable explanation
    enabled: boolean            # Optional: Default true
    priority: integer           # Optional: Lower = higher priority (default: 100)
    match: MatchConfig          # Required: Conditions to match
    action: ActionConfig        # Required: What to do when matched
```

---

## Event Types

| Event | Description | Available Context |
|-------|-------------|-------------------|
| `PreToolUse` | Before tool executes | tool_name, tool_input, file_path |
| `PostToolUse` | After tool completes | tool_name, tool_input, tool_output, file_path |
| `PermissionRequest` | User approval requested | tool_name, permission_type |
| `UserPromptSubmit` | User sends message | prompt_text |
| `SessionStart` | New session begins | session_id, project_path |
| `SessionEnd` | Session terminates | session_id, duration |
| `PreCompact` | Before context compaction | current_tokens, max_tokens |

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

## Match Configuration

All matchers are optional. Multiple matchers use AND logic (all must match).

```yaml
match:
  tools: [Tool, ...]           # Match specific tool names
  extensions: [.ext, ...]      # Match file extensions
  directories: [path/, ...]    # Match directory prefixes
  operations: [op, ...]        # Match Bash operations
  command_match: "regex"       # Match Bash command
  prompt_match: "regex"        # Match user prompt
  enabled_when: "expression"   # Conditional expression
```

### tools

Array of tool names to match.

```yaml
match:
  tools: [Write, Edit, Read]   # Exact tool names
  tools: [Bash]                # Just Bash tool
```

**Valid tool names**: `Read`, `Write`, `Edit`, `Bash`, `Glob`, `Grep`, `Task`, `WebFetch`, `TodoRead`, `TodoWrite`

### extensions

Array of file extensions. Matches `tool.input.path`.

```yaml
match:
  extensions: [.py, .pyi]      # Python files
  extensions: [.js, .ts, .jsx, .tsx]  # JavaScript/TypeScript
```

### directories

Array of directory prefixes. Uses forward slash.

```yaml
match:
  directories: [src/, lib/]    # Source directories
  directories: [tests/]        # Test directory only
```

### operations

Array of Bash command prefixes. Extracts first word of command.

```yaml
match:
  operations: [git, npm, docker]  # Version control, package, container
  operations: [rm, mv, cp]        # File operations
```

### command_match

Regex pattern matched against full Bash command.

```yaml
match:
  command_match: "git push.*--force"     # Force push
  command_match: "rm -rf /"              # Dangerous delete
  command_match: "(?i)password"          # Case-insensitive
```

**Regex flavor**: Rust regex (similar to PCRE, no lookbehind)

### prompt_match

Regex pattern matched against user prompt text.

```yaml
match:
  prompt_match: "(?i)deploy"             # Deploy requests
  prompt_match: "^/fix"                  # Slash commands
```

### enabled_when

Conditional expression for dynamic matching.

```yaml
match:
  enabled_when: "env.CI == 'true'"              # Only in CI
  enabled_when: "tool.input.path =~ '\\.test\\.'"  # Test files
  enabled_when: "session.project =~ 'backend'"  # Backend projects
```

**Operators**: `==`, `!=`, `=~` (regex match), `&&`, `||`, `!`

---

## Action Configuration

### inject

Inject markdown content into Claude's context.

```yaml
action:
  type: inject
  source: file | inline | command
  
  # For source: file
  path: .claude/context/standards.md
  
  # For source: inline
  content: |
    ## Important Note
    Always follow these guidelines...
  
  # For source: command
  command: cat VERSION
  timeout: 10                    # Optional: seconds (default: 30)
```

### run

Execute a script and use its output.

```yaml
action:
  type: run
  command: .claude/validators/check.sh
  timeout: 30                    # Optional: seconds
  env:                           # Optional: environment variables
    STRICT_MODE: "true"
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
action:
  type: block
  reason: "This operation is not allowed in this project"
```

### block_if_match

Block if pattern matches in tool input.

```yaml
action:
  type: block_if_match
  pattern: "(?i)(password|secret|api_key)"
  field: content                 # Field to check in tool input
  reason: "Potential secret detected in file content"
```

### require_fields

Validate that required fields exist in tool input.

```yaml
action:
  type: require_fields
  fields: [path, content]
  message: "Write tool requires both path and content"
```

---

## Complete Example

```yaml
version: "1"

hooks:
  # High priority: Block dangerous operations first
  - name: block-force-push
    event: PreToolUse
    priority: 10
    description: Prevent force push to protected branches
    match:
      tools: [Bash]
      command_match: "git push.*(--force|-f).*main"
    action:
      type: block
      reason: "Force push to main is prohibited. Use a PR workflow."

  # Medium priority: Inject context for code changes
  - name: python-standards
    event: PreToolUse
    priority: 50
    match:
      tools: [Write, Edit]
      extensions: [.py]
    action:
      type: inject
      source: file
      path: .claude/context/python-standards.md

  # Conditional: Only in CI
  - name: ci-strict-mode
    event: PreToolUse
    match:
      tools: [Bash]
      enabled_when: "env.CI == 'true'"
    action:
      type: run
      command: .claude/validators/ci-check.sh
      timeout: 60

  # Session start: Load project context
  - name: load-project-context
    event: SessionStart
    match: {}                    # Match all session starts
    action:
      type: inject
      source: file
      path: .claude/context/project-overview.md
```

---

## Validation

Validate your configuration:

```bash
cch validate
```

Common validation errors:

| Error | Cause | Fix |
|-------|-------|-----|
| `unknown field` | Typo in field name | Check spelling |
| `invalid event type` | Wrong event name | Use exact event names |
| `file not found` | Bad path in action | Verify file exists |
| `invalid regex` | Bad regex syntax | Test regex separately |
| `duplicate rule name` | Same name used twice | Use unique names |
