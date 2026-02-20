# Platform Adapters Reference

RuleZ supports multiple AI coding assistant platforms through adapter-based event translation. Write rules using RuleZ event types and adapters handle the translation automatically.

## Supported Platforms

| Platform | Adapter | Status | Dual-Fire Support |
|----------|---------|--------|-------------------|
| **Claude Code** | Native | Full support (16 event types) | N/A |
| **Gemini CLI** | `gemini.rs` | Full support | Yes (3 scenarios) |
| **GitHub Copilot** | `copilot.rs` | Supported | No |
| **OpenCode** | `opencode.rs` | Supported | Yes (1 scenario) |

## Cross-Platform Event Mapping

This table shows how each platform's native events map to RuleZ unified event types.

| RuleZ EventType | Claude Code | Gemini CLI | Copilot | OpenCode |
|-----------------|-------------|------------|---------|----------|
| `PreToolUse` | `PreToolUse` | `BeforeTool` | `preToolUse` | `tool.execute.before` |
| `PostToolUse` | `PostToolUse` | `AfterTool` | `postToolUse` | `tool.execute.after` |
| `PostToolUseFailure` | `PostToolUseFailure` | `AfterTool` (on fail) | `errorOccurred` | `tool.execute.after` (on fail) |
| `PermissionRequest` | `PermissionRequest` | `Notification` (ToolPermission) | -- | -- |
| `UserPromptSubmit` | `UserPromptSubmit` | `BeforeAgent` (dual) | `promptSubmit` | `session.updated` |
| `BeforeAgent` | `SubagentStart` (alias) | `BeforeAgent` (dual) | -- | -- |
| `AfterAgent` | `SubagentStop` (alias) | `AfterAgent` | -- | -- |
| `BeforeModel` | -- | `BeforeModel` | -- | -- |
| `AfterModel` | -- | `AfterModel` | -- | -- |
| `BeforeToolSelection` | -- | `BeforeToolSelection` | -- | -- |
| `SessionStart` | `SessionStart` | `SessionStart` | `sessionStart` | `session.created` |
| `SessionEnd` | `SessionEnd` | `SessionEnd` | `sessionEnd` | `session.deleted` |
| `PreCompact` | `PreCompact` | `PreCompact` | `preCompact` | `session.compacted` |
| `Stop` | `Stop` | -- | -- | -- |
| `Notification` | `Notification` | fallback | fallback | fallback |
| `Setup` | `Setup` | -- | -- | -- |

**Legend**: `--` = not available on this platform; `(dual)` = dual-fire event; `(alias)` = backward-compatible alias

## Dual-Fire Events

Some platform events map to **multiple** RuleZ event types. When this happens, rules for all mapped types are evaluated. If any evaluation blocks, the event is blocked.

| Platform Event | Primary EventType | Also Fires | Condition |
|----------------|-------------------|------------|-----------|
| Gemini `BeforeAgent` | `BeforeAgent` | `UserPromptSubmit` | Always (payload has `prompt`) |
| Gemini `AfterTool` | `PostToolUse` | `PostToolUseFailure` | When result indicates failure |
| Gemini `Notification` | `Notification` | `PermissionRequest` | When `notification_type` = `"ToolPermission"` |
| OpenCode `tool.execute.after` | `PostToolUse` | `PostToolUseFailure` | When payload has `error` or `success: false` |

### What Triggers Failure Detection?

Both Gemini and OpenCode detect tool failures the same way:
- `tool_input.success == false`
- `tool_input.error` field exists
- `extra.success == false`
- `extra.error` field exists

### Dual-Fire Implications for Rule Writing

1. **Gemini `BeforeAgent`**: If you have rules on both `BeforeAgent` and `UserPromptSubmit`, both will trigger when Gemini fires `BeforeAgent`. Design rules accordingly to avoid duplicate context injection.

2. **Tool failure**: If you have rules on both `PostToolUse` and `PostToolUseFailure`, both will trigger when a tool fails on Gemini or OpenCode. This is useful for having general post-tool logic plus specific failure handling.

3. **Permission requests on Gemini**: The `PermissionRequest` event is only available on Gemini via dual-fire from `Notification` when `notification_type` is `"ToolPermission"`.

## Platform-Specific Response Handling

Each adapter translates RuleZ's standard response format into the platform's expected format:

### Claude Code (Native)
Standard RuleZ response: `{"continue": bool, "reason": string, "context": string}`

### Gemini CLI
Translated to `GeminiHookResponse`:
- `decision`: `Allow` | `Deny`
- `reason`: Optional string
- `system_message`: Context injected as system message (non-tool events)
- `tool_input`: JSON object override (tool events)

### GitHub Copilot
Translated to `CopilotHookResponse`:
- `permission_decision`: `Allow` | `Deny`
- `permission_decision_reason`: Optional string (only when denied)
- `tool_input`: JSON object override (tool events)

### OpenCode
Translated to JSON:
- `continue`: boolean
- `reason`: Optional string
- `context`: Optional string
- `tools`: Array of available RuleZ tools (`rulez.check`, `rulez.explain`)

## Universal Events

These events are available on **all** platforms and are safe to use in cross-platform configurations:

- `PreToolUse`
- `PostToolUse`
- `UserPromptSubmit`
- `SessionStart`
- `SessionEnd`
- `PreCompact`

## Cross-Platform Rule Writing Tips

1. **Stick to universal events** when writing rules meant for all platforms
2. **Use `BeforeAgent`/`AfterAgent`** only if targeting Claude Code and Gemini
3. **Use `BeforeModel`/`AfterModel`/`BeforeToolSelection`** only if targeting Gemini
4. **Test with `rulez debug`** to verify matching before deploying
5. **Be aware of dual-fire** on Gemini to avoid duplicate context injection

## Ground Truth Reference

The authoritative event mapping is maintained in `docs/EVENT-MAPPING.md` in the RuleZ repository. Adapter source code is in `rulez/src/adapters/`.
