# RuleZ Event Type Mapping

Universal event type mappings across all supported platforms.

## Event Types

| RuleZ EventType       | Claude Code            | Gemini CLI             | Copilot                | OpenCode               |
|-----------------------|------------------------|------------------------|------------------------|------------------------|
| `PreToolUse`          | `PreToolUse`           | `BeforeTool`           | `preToolUse`           | `tool.execute.before`  |
| `PostToolUse`         | `PostToolUse`          | `AfterTool`            | `postToolUse`          | `tool.execute.after`   |
| `PostToolUseFailure`  | `PostToolUseFailure`   | `AfterTool` (on fail)  | `errorOccurred`        | `tool.execute.after` (on fail) |
| `PermissionRequest`   | `PermissionRequest`    | `Notification` (ToolPermission) | —             | —                      |
| `UserPromptSubmit`    | `UserPromptSubmit`     | `BeforeAgent` (dual)   | `promptSubmit`         | `session.updated`      |
| `SessionStart`        | `SessionStart`         | `SessionStart`         | `sessionStart`         | `session.created`      |
| `SessionEnd`          | `SessionEnd`           | `SessionEnd`           | `sessionEnd`           | `session.deleted`      |
| `Stop`                | `Stop`                 | —                      | —                      | —                      |
| `Notification`        | `Notification`         | fallback               | fallback               | fallback               |
| `BeforeAgent`         | `SubagentStart` (alias)| `BeforeAgent` (dual)   | —                      | —                      |
| `AfterAgent`          | `SubagentStop` (alias) | `AfterAgent`           | —                      | —                      |
| `PreCompact`          | `PreCompact`           | `PreCompact`           | `preCompact`           | `session.compacted`    |
| `Setup`               | `Setup`                | —                      | —                      | —                      |
| `TeammateIdle`        | `TeammateIdle`         | —                      | —                      | —                      |
| `TaskCompleted`       | `TaskCompleted`        | —                      | —                      | —                      |
| `BeforeModel`         | —                      | `BeforeModel`          | —                      | —                      |
| `AfterModel`          | —                      | `AfterModel`           | —                      | —                      |
| `BeforeToolSelection` | —                      | `BeforeToolSelection`  | —                      | —                      |

## Dual-Fire Events

Some platform events map to multiple RuleZ event types. When this happens, rules for all mapped types are evaluated. If any evaluation blocks, the event is blocked.

| Platform Event            | Primary EventType  | Also Fires            | Condition                     |
|---------------------------|--------------------|-----------------------|-------------------------------|
| Gemini `BeforeAgent`      | `BeforeAgent`      | `UserPromptSubmit`    | Always (payload has `prompt`) |
| Gemini `AfterTool`        | `PostToolUse`      | `PostToolUseFailure`  | When result indicates failure |
| Gemini `Notification`     | `Notification`     | `PermissionRequest`   | When `notification_type` = `"ToolPermission"` |
| OpenCode `tool.execute.after` | `PostToolUse`  | `PostToolUseFailure`  | When payload has `error` or `success: false` |

## Serde Aliases

The `EventType` enum supports backward-compatible aliases via `#[serde(alias)]`:

| Alias            | Resolves To   |
|------------------|---------------|
| `SubagentStart`  | `BeforeAgent` |
| `SubagentStop`   | `AfterAgent`  |

## Debug CLI Aliases

The `rulez debug` command accepts these aliases:

| Input                              | Event Type          |
|------------------------------------|---------------------|
| `pretooluse`, `pre`, `pre-tool-use`| `PreToolUse`        |
| `posttooluse`, `post`              | `PostToolUse`       |
| `beforeagent`, `subagentstart`, `subagent` | `BeforeAgent` |
| `afteragent`, `subagentstop`       | `AfterAgent`        |
| `sessionstart`, `session`, `start` | `SessionStart`      |
| `sessionend`, `end`                | `SessionEnd`        |
| `permissionrequest`, `permission`  | `PermissionRequest` |
| `userpromptsubmit`, `prompt`       | `UserPromptSubmit`  |
| `precompact`, `compact`            | `PreCompact`        |
| `teammateidle`, `idle`             | `TeammateIdle`      |
| `taskcompleted`, `task`            | `TaskCompleted`     |
