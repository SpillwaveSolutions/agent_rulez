# RuleZ Event Schema

This document describes the JSON event format that AI coding assistants send to RuleZ on stdin, and the JSON response format that RuleZ writes to stdout.

## Event JSON Schema

When a hook fires, the AI coding assistant sends a JSON object on stdin. RuleZ deserializes this into the `Event` struct defined in `rulez/src/models.rs`.

### Required fields

| Field | Type | Description |
|-------|------|-------------|
| `hook_event_name` | string | Event type in PascalCase (see [Event Types](#event-types)). Also accepted as `event_type` (alias). |
| `session_id` | string | Unique identifier for the current session. |

### Optional fields

| Field | Type | Description |
|-------|------|-------------|
| `tool_name` | string | Name of the tool being invoked (e.g., `"Bash"`, `"Write"`, `"Edit"`, `"Read"`). |
| `tool_input` | object | Tool parameters. Contents vary by tool -- see [Tool Input Examples](#tool-input-examples). |
| `cwd` | string | Current working directory. Used to locate project-level `.claude/hooks.yaml`. |
| `timestamp` | string | ISO 8601 timestamp. Defaults to current time if omitted. |
| `user_id` | string | User identifier (if available). |
| `transcript_path` | string | Path to session transcript file. |
| `permission_mode` | string | Claude Code permission mode (e.g., `"default"`, `"plan"`). |
| `tool_use_id` | string | Unique identifier for this specific tool invocation. |
| `prompt` | string | User prompt text. Populated for `UserPromptSubmit` events. |

### Full example

```json
{
  "hook_event_name": "PreToolUse",
  "session_id": "abc-123-def-456",
  "tool_name": "Bash",
  "tool_input": {
    "command": "git push --force origin main",
    "description": "Force push to main"
  },
  "cwd": "/home/user/my-project",
  "timestamp": "2026-03-11T14:30:00.000Z",
  "user_id": "user-789",
  "transcript_path": "/home/user/.claude/sessions/abc-123.jsonl",
  "permission_mode": "default",
  "tool_use_id": "toolu_01ABC"
}
```

### Minimal example

Only the two required fields:

```json
{
  "hook_event_name": "SessionStart",
  "session_id": "session-001"
}
```

## Event Types

RuleZ defines the following event types in the `EventType` enum. These are universal across all supported platforms -- platform adapters translate platform-specific names into these types.

### Tool lifecycle events

| Event Type | When it fires | Typical use |
|------------|---------------|-------------|
| `PreToolUse` | Before a tool is executed. | Block dangerous commands, inject coding standards, run validators. |
| `PostToolUse` | After a tool completes successfully. | Log results, inject follow-up reminders. |
| `PostToolUseFailure` | After a tool execution fails. | Log failures, inject recovery guidance. |
| `PermissionRequest` | When the assistant requests user permission. | Auto-approve safe operations, inject permission context. |

### Session lifecycle events

| Event Type | When it fires | Typical use |
|------------|---------------|-------------|
| `SessionStart` | When a new session begins. | Inject project context, load conventions. |
| `SessionEnd` | When a session ends. | Final audit logging. |

### User interaction events

| Event Type | When it fires | Typical use |
|------------|---------------|-------------|
| `UserPromptSubmit` | When the user submits a prompt. | Match prompt patterns, inject context based on user intent. |

### Agent lifecycle events

| Event Type | When it fires | Typical use |
|------------|---------------|-------------|
| `BeforeAgent` | Before a sub-agent starts. Also accepts alias `SubagentStart`. | Inject agent-specific policies. |
| `AfterAgent` | After a sub-agent completes. Also accepts alias `SubagentStop`. | Log agent results. |

### Model lifecycle events

| Event Type | When it fires | Typical use |
|------------|---------------|-------------|
| `BeforeModel` | Before a model inference call. | Inject system context. |
| `AfterModel` | After a model inference call completes. | Log model usage. |
| `BeforeToolSelection` | Before the model selects which tool to use. | Influence tool selection. |

### Other events

| Event Type | When it fires | Typical use |
|------------|---------------|-------------|
| `PreCompact` | Before conversation compaction. | Inject must-retain context. |
| `Stop` | When the assistant decides to stop. | Final logging. |
| `Notification` | When a notification is emitted. | Audit notifications. |
| `Setup` | During initial setup. | One-time initialization. |

## Tool Input Examples

The `tool_input` field varies by tool. Common shapes:

### Bash tool

```json
{
  "command": "git status",
  "description": "Check git status"
}
```

### Write tool

```json
{
  "file_path": "/home/user/project/src/main.rs",
  "content": "fn main() { ... }"
}
```

### Edit tool

```json
{
  "file_path": "/home/user/project/src/lib.rs",
  "old_string": "fn old_name()",
  "new_string": "fn new_name()"
}
```

### Read tool

```json
{
  "file_path": "/home/user/project/README.md"
}
```

## Response Format

RuleZ writes a JSON response to stdout. The AI coding assistant reads this to determine whether to proceed.

### Response fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `continue` | boolean | Yes | `true` to allow the operation, `false` to block it. |
| `reason` | string | No | Human-readable explanation for blocking or context injection. |
| `context` | string | No | Markdown content to inject into the assistant's context window. |
| `timing` | object | No | Performance metrics (`processing_ms`, `rules_evaluated`). |

### Allow response (no rules matched)

```json
{
  "continue": true
}
```

### Block response

```json
{
  "continue": false,
  "reason": "Force push to main/master is prohibited. Use a PR workflow."
}
```

### Context injection response

```json
{
  "continue": true,
  "context": "## Python Standards\n\n- Use type hints on all functions\n- Follow PEP 8 naming conventions\n"
}
```

### Response with timing

```json
{
  "continue": true,
  "context": "Injected coding standards.",
  "timing": {
    "processing_ms": 2,
    "rules_evaluated": 5
  }
}
```

## Exit Code Convention

RuleZ uses exit codes to signal the outcome to the AI coding assistant:

| Exit Code | Meaning | Effect |
|-----------|---------|--------|
| 0 | Success / allow | The operation proceeds. The assistant reads stdout for optional context. |
| 2 | Validation error / block | The operation is blocked. The assistant reads the `reason` field from stdout. |
| 1 | Configuration error | RuleZ could not load or parse `hooks.yaml`. Behavior depends on `fail_open` setting. |
| 3 | Runtime error | An unexpected error occurred during rule evaluation. |

## Schema Validation

RuleZ performs JSON Schema validation on incoming events using a schema auto-generated from the `Event` struct (via the `schemars` crate, JSON Schema draft 2020-12).

Schema validation uses **fail-open semantics**: extra fields, wrong optional types, or missing optional fields produce log warnings but do not block processing. Only missing *required* fields (`hook_event_name`, `session_id`) cause a fatal deserialization error.

You can export the generated schema with:

```bash
rulez schema --export
```

## Platform Adapters

Different AI coding assistants use different event formats. RuleZ includes platform adapters that translate platform-specific events into the canonical format described above:

| Platform | Adapter | Notes |
|----------|---------|-------|
| Claude Code | Native | Events match the schema directly. |
| Gemini CLI | `adapters/gemini.rs` | Translates Gemini event names and response format. |
| GitHub Copilot | `adapters/copilot.rs` | Translates Copilot event names and response format. |
| OpenCode | `adapters/opencode.rs` | Translates OpenCode event names and response format. |
| Codex | N/A | No hooks support as of v2.1.0. |
