---
last_modified: 2026-03-16
last_validated: 2026-03-16
---

# Agent Inline Hooks Reference

How to define, embed, and manage RuleZ hooks for Claude Code subagents.

## Overview

When Claude Code spawns a subagent via the Agent tool, hooks can be configured at multiple levels:

| Level | Scope | Lifetime | Defined In |
|-------|-------|----------|------------|
| **Global** | All sessions, all agents | Permanent | `~/.claude/hooks.yaml` |
| **Project** | Current project, all agents | Permanent | `.claude/hooks.yaml` |
| **Inline** | Single subagent invocation | Subagent lifetime only | Agent tool configuration |

Inline hooks let you apply ephemeral policies to a specific subagent without modifying your permanent hook configuration.

---

## Inline Hook Syntax

Inline hooks are specified as part of the Agent tool's configuration when spawning a subagent. They use the same YAML structure as standard hooks but are scoped to the subagent's execution context.

```yaml
# Agent tool invocation with inline hooks
agent:
  task: "Review the authentication module for security issues"
  hooks:
    PreToolUse:
      - matcher: "Bash"
        hooks:
          - type: command
            command: "rulez --inline-policy security-audit"
            timeout: 10
    BeforeAgent:
      - matcher: "*"
        hooks:
          - type: command
            command: "rulez --event BeforeAgent"
            timeout: 5
```

The `hooks` key within the agent configuration accepts the same event-keyed structure that `.claude/settings.json` uses.

---

## Merging Behavior

Inline hooks **extend** parent hooks -- they do not override them. When a subagent fires an event, the evaluation order is:

1. **Global hooks** fire first (from `~/.claude/hooks.yaml`)
2. **Project hooks** fire next (from `.claude/hooks.yaml`)
3. **Inline hooks** fire last (from the Agent tool configuration)

All matching hooks at every level execute. A block action at any level stops the tool invocation.

```
Event: PreToolUse (Write)
  |
  +--> Global hooks evaluate     --> inject: company-standards.md
  +--> Project hooks evaluate    --> inject: project-conventions.md
  +--> Inline hooks evaluate     --> block if path matches /etc/*
  |
  Result: All three fire. If the inline hook blocks, the Write is denied.
```

### Key Rules

- Inline hooks cannot disable or remove parent hooks
- Priority ordering works the same way: lower priority number = higher precedence
- If a parent hook blocks an action, inline hooks for that event do not fire
- If an inline hook blocks an action, the tool invocation is denied even though parent hooks allowed it

---

## Scope and Lifecycle

Inline hooks exist only for the duration of the subagent that defines them:

| Phase | Inline Hooks Active? |
|-------|---------------------|
| Parent conversation | No |
| Subagent starts (`BeforeAgent`) | Yes |
| Subagent tool calls (`PreToolUse`, `PostToolUse`) | Yes |
| Subagent completes (`AfterAgent`) | Yes (last event) |
| Parent conversation resumes | No |

After the subagent completes, inline hooks are discarded. They do not persist to disk, do not affect other subagents, and do not modify the parent session's hook configuration.

### Nested Subagents

If a subagent spawns its own subagent, inline hooks do **not** cascade down automatically. Each subagent level only sees:
- Global hooks
- Project hooks
- Its own inline hooks (if any)

---

## Examples

### Subagent with Additional Security Restrictions

Restrict a code-review subagent so it can only read files, not modify them.

```yaml
# Parent hooks.yaml (project-level)
hooks:
  - name: project-standards
    matchers:
      operations: [PreToolUse]
      tools: [Write, Edit]
    actions:
      inject: .claude/context/coding-standards.md

# Inline hooks for the security-review subagent
agent:
  task: "Audit src/auth/ for SQL injection vulnerabilities"
  hooks:
    PreToolUse:
      - matcher: "Write"
        hooks:
          - type: command
            command: |
              echo '{"continue": false, "reason": "Security review agent is read-only. Do not modify files."}'
            timeout: 5
      - matcher: "Edit"
        hooks:
          - type: command
            command: |
              echo '{"continue": false, "reason": "Security review agent is read-only. Do not modify files."}'
            timeout: 5
      - matcher: "Bash"
        hooks:
          - type: command
            command: |
              echo '{"continue": false, "reason": "Security review agent cannot run shell commands."}'
            timeout: 5
```

**What happens**: The project-level `project-standards` hook still fires for any `Write`/`Edit` attempts (injecting context), but the inline hooks then block the operation entirely. The subagent can only use `Read`, `Glob`, and `Grep`.

### Subagent with Domain-Specific Context Injection

Inject specialized context for a subagent working on database migrations.

```yaml
# Inline hooks for database migration subagent
agent:
  task: "Create a migration to add the user_preferences table"
  hooks:
    BeforeAgent:
      - matcher: "*"
        hooks:
          - type: command
            command: |
              cat <<'POLICY'
              {"continue": true, "context": "## Database Migration Policy\n\n- Always use reversible migrations (include up AND down)\n- Use explicit column types, never rely on ORM defaults\n- Add indexes for all foreign keys\n- Include data migration scripts if schema changes affect existing rows\n- Test migrations against a copy of production data\n- Maximum one table change per migration file"}
              POLICY
            timeout: 5
    PreToolUse:
      - matcher: "Write"
        hooks:
          - type: command
            command: |
              # Only allow writes to the migrations directory
              INPUT=$(cat -)
              PATH_VAL=$(echo "$INPUT" | jq -r '.tool_input.file_path // ""')
              if echo "$PATH_VAL" | grep -q "^migrations/\|^db/migrate/"; then
                echo '{"continue": true}'
              else
                echo '{"continue": false, "reason": "Migration agent can only write to migrations/ or db/migrate/ directories."}'
              fi
            timeout: 5
```

**What happens**: The `BeforeAgent` inline hook injects database migration policy when the subagent starts. The `PreToolUse` inline hook restricts file writes to migration directories only.

### Subagent with Audit Logging Hooks

Add detailed audit logging for a subagent performing sensitive operations.

```yaml
# Inline hooks for deployment subagent with full audit trail
agent:
  task: "Update the Kubernetes deployment manifest for v2.5.0"
  hooks:
    BeforeAgent:
      - matcher: "*"
        hooks:
          - type: command
            command: |
              # Log subagent start
              TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)
              echo "{\"event\": \"subagent_start\", \"task\": \"deployment\", \"timestamp\": \"$TIMESTAMP\"}" >> ~/.claude/logs/audit.jsonl
              echo '{"continue": true, "context": "Audit logging enabled for this deployment task."}'
            timeout: 5
    PreToolUse:
      - matcher: "*"
        hooks:
          - type: command
            command: |
              # Log every tool invocation
              INPUT=$(cat -)
              TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)
              TOOL=$(echo "$INPUT" | jq -r '.tool_name // "unknown"')
              echo "{\"event\": \"tool_use\", \"tool\": \"$TOOL\", \"timestamp\": \"$TIMESTAMP\", \"agent\": \"deployment\"}" >> ~/.claude/logs/audit.jsonl
              echo '{"continue": true}'
            timeout: 5
    AfterAgent:
      - matcher: "*"
        hooks:
          - type: command
            command: |
              # Log subagent completion with summary
              TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)
              TOOL_COUNT=$(grep -c '"agent": "deployment"' ~/.claude/logs/audit.jsonl 2>/dev/null || echo 0)
              echo "{\"event\": \"subagent_end\", \"task\": \"deployment\", \"timestamp\": \"$TIMESTAMP\", \"tool_invocations\": $TOOL_COUNT}" >> ~/.claude/logs/audit.jsonl
              echo '{"continue": true, "context": "Deployment audit complete. $TOOL_COUNT tool invocations logged."}'
            timeout: 5
```

**What happens**: Every tool call within the deployment subagent is logged to `~/.claude/logs/audit.jsonl` with timestamps. The `AfterAgent` hook writes a summary entry when the subagent completes.

---

## Best Practices

1. **Use inline hooks for ephemeral policies** -- If a restriction only applies to one specific task, use inline hooks rather than adding conditional logic to your permanent hooks.

2. **Keep inline hooks simple** -- Complex validation logic belongs in scripts referenced by your project hooks. Inline hooks work best for straightforward block/allow decisions and lightweight context injection.

3. **Combine with global hooks for defense in depth** -- Global hooks provide baseline security. Inline hooks add task-specific restrictions on top.

4. **Test with `rulez debug`** -- Before embedding inline hooks in an agent configuration, test the hook logic standalone:
   ```bash
   rulez debug PreToolUse --tool Write --path migrations/001.sql -v
   ```

5. **Log inline hook activity** -- Since inline hooks are ephemeral, add logging so you have a record of what policies were applied during subagent execution.

---

## Limitations

- Inline hooks cannot reference `inject: <file>` paths that don't exist in the project -- the subagent runs in the same working directory as the parent, but verify paths are accessible.
- Inline hooks are not persisted. If you need the same policy repeatedly, define it in your project hooks with an `enabled_when` condition.
- Not all platforms support inline hooks. This feature is specific to Claude Code's Agent tool. See [platform-adapters.md](platform-adapters.md) for cross-platform event support.
