# RuleZ Tool Name Mapping

Canonical tool name mappings across all supported platforms. Each adapter normalizes platform-specific tool names to Claude Code's PascalCase canonical names at ingestion time, so rules with `tools:` matchers work identically across all platforms.

## Tool Names

| Canonical (Claude Code) | Gemini CLI | Copilot | OpenCode |
|-------------------------|------------|---------|----------|
| `Bash` | `run_shell_command`, `execute_code` | `shell` | `bash` |
| `Write` | `write_file` | `write` | `write` |
| `Edit` | `replace` | `edit` | `edit` |
| `Read` | `read_file` | `read` | `read` |
| `Glob` | `glob` | `glob` | `glob` |
| `Grep` | `search_file_content`, `grep_search` | `grep` | `grep` |
| `WebFetch` | `web_fetch` | `fetch` | `webfetch`, `fetch` |
| `Task` | (pass-through) | `task` | `task` |
| `TodoRead` | — | `TodoRead` (pass-through) | — |
| `TodoWrite` | — | `TodoWrite` (pass-through) | — |

## How It Works

Each adapter has a `map_tool_name()` function that translates platform-native tool names to canonical PascalCase names at parse time. This mirrors the existing `map_event_type()` pattern used for event type canonicalization.

The matcher in `hooks.rs` uses exact case-sensitive matching on canonical names:

```yaml
# This rule works on ALL platforms — Gemini's run_shell_command,
# Copilot's shell, and OpenCode's bash all resolve to "Bash"
rules:
  - name: block-dangerous-commands
    event: PreToolUse
    tools: [Bash]
    action: deny
```

## Pass-Through Behavior

Unknown tool names (MCP tools, platform-specific tools without a canonical equivalent) pass through unchanged. No `platform_tool_name` field is injected for pass-through tools.

For example, `mcp__github__create_issue` on any platform remains `mcp__github__create_issue` in the event — rules can match it directly.

## Platform Tool Name Preservation

When a tool name IS mapped to a canonical name, the original platform name is preserved in `tool_input.platform_tool_name`. This allows rules and scripts to inspect which platform issued the tool call.

```json
{
  "tool_name": "Bash",
  "tool_input": {
    "command": "ls -la",
    "platform_tool_name": "run_shell_command"
  }
}
```

## Platform-Specific Tools (Pass-Through)

These platform tools have no Claude Code equivalent and pass through unchanged:

**Gemini CLI:** `list_directory`, `read_many_files`, `google_web_search`, `ask_user`, `save_memory`, `write_todos`, `activate_skill`

**OpenCode:** `list`, `lsp`, `patch`, `skill`, `todowrite`, `todoread`, `websearch`, `question`

**Copilot:** `TodoRead`, `TodoWrite` (already PascalCase, pass through as-is)

## Confidence Notes

| Platform | Confidence | Source |
|----------|-----------|--------|
| Gemini CLI | MEDIUM | Verified from geminicli.com/docs/tools/ (2026-02). `grep_search` and `search_file_content` both mapped as aliases. |
| OpenCode | HIGH | Verified from opencode.ai/docs/tools/. Official tool names match mappings. |
| Copilot | LOW | No official tool name documentation found. Mappings are best-effort inference from hook format specs. |

## Related

- [EVENT-MAPPING.md](EVENT-MAPPING.md) — Event type mappings across platforms
- Source: `rulez/src/adapters/gemini.rs`, `copilot.rs`, `opencode.rs` — `map_tool_name()` functions
