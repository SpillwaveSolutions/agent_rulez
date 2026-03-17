---
last_modified: 2026-03-16
last_validated: 2026-03-16
---

# BeforeAgent Event Guide: Subagent Governance with RuleZ

How to use BeforeAgent, AfterAgent, and PreToolUse events to govern subagent behavior across AI coding platforms.

---

## Event Scoping: PreToolUse vs BeforeAgent

Understanding when each event fires is critical to writing effective subagent policies.

### PreToolUse

`PreToolUse` fires **before each individual tool call** (Bash, Write, Read, Edit, Glob, Grep, etc.) -- whether in the main conversation or inside a subagent. This is the most common event for policy enforcement because it covers every tool invocation regardless of where it originates.

- Fires once per tool call
- Fires in both parent conversations and subagents
- Receives the tool name and tool input as context
- Can block, allow, or inject context per-call

### BeforeAgent

`BeforeAgent` fires **once** when a subagent is about to be spawned, before the Agent tool runs. It fires in the **parent context**, not the child. Use this to:

- **Inject policy context** that the subagent will see when it starts
- **Block certain agent tasks entirely** before any tools execute
- **Audit which agent tasks are being started** by logging the spawn event

Key distinction: `BeforeAgent` does not fire again for individual tool calls within the subagent. Once the subagent starts, tool-level governance is handled by `PreToolUse` and `PostToolUse`.

### AfterAgent

`AfterAgent` fires **once** when a subagent completes (or is terminated). Use this for:

- Post-completion audit logging
- Summarizing what the subagent did
- Triggering follow-up actions

---

## When to Use Each Event

| Use Case | Event | Why |
|----------|-------|-----|
| Block dangerous commands | `PreToolUse` | Fires for every tool, catches commands in subagents too |
| Inject coding standards | `PreToolUse` | Context needed per-tool-call |
| Restrict which agents can run | `BeforeAgent` | Only fires when an agent spawns |
| Audit agent activity | `AfterAgent` | Fires after agent completes |
| Block agents from certain tasks | `BeforeAgent` | Pre-empt before tools run |
| Restrict file writes to certain paths | `PreToolUse` | Evaluates each Write/Edit individually |
| Inject one-time context for a task | `BeforeAgent` | Fires once at spawn, avoids repeated injection |
| Log tool-level detail inside agents | `PreToolUse` + `PostToolUse` | Captures every operation |

---

## Examples

### Example 1: Block Subagents from Modifying Production Files

Use `BeforeAgent` to prevent subagents from starting if their task description mentions production deployments, and use `PreToolUse` as a safety net to block writes to production paths.

```yaml
version: "1"

rules:
  # Gate at agent spawn -- block tasks that target production
  - name: block-production-agents
    matchers:
      operations: [BeforeAgent]
      prompt_match: "(?i)(deploy|production|prod environment)"
    actions:
      block: true

  # Safety net -- block writes to production paths from any context
  - name: block-production-writes
    matchers:
      operations: [PreToolUse]
      tools: [Write, Edit, Bash]
      command_match: "(/opt/production/|/srv/prod/)"
    actions:
      block: true
```

**What happens**: If a subagent is spawned with a task like "deploy the new version to production", the `BeforeAgent` hook blocks it before any tools run. If a subagent with a different task description somehow tries to write to a production path, the `PreToolUse` hook catches it.

### Example 2: Inject Subagent-Specific Context

Use `BeforeAgent` with an inject action to give a subagent specialized policy context when it starts.

```yaml
version: "1"

rules:
  # Inject security review context when a review agent spawns
  - name: security-review-context
    matchers:
      operations: [BeforeAgent]
      prompt_match: "(?i)(security|vulnerability|audit|CVE)"
    actions:
      inject_inline: |
        ## Security Review Policy

        When performing security reviews:
        - Check all user inputs for injection vulnerabilities (SQL, XSS, command injection)
        - Verify authentication and authorization on every endpoint
        - Flag any hardcoded credentials, API keys, or secrets
        - Ensure cryptographic operations use current best practices
        - Report findings in SARIF format when possible
        - Do NOT modify code -- this is a read-only review

  # Enforce read-only for security review subagents
  - name: security-review-readonly
    matchers:
      operations: [PreToolUse]
      tools: [Write, Edit]
    actions:
      block: true
```

**What happens**: When a subagent is spawned with a security-related task, `BeforeAgent` injects the security review policy as context. The `PreToolUse` hook then enforces read-only access for the duration of the subagent.

### Example 3: Audit All Agent Spawns

Use `AfterAgent` with a script to log every subagent lifecycle event for compliance auditing.

```yaml
version: "1"

rules:
  # Log when any subagent starts
  - name: audit-agent-start
    matchers:
      operations: [BeforeAgent]
    actions:
      inline_script: |
        #!/bin/bash
        INPUT=$(cat -)
        TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)
        TASK=$(echo "$INPUT" | jq -r '.tool_input.task // "unknown"')
        echo "{\"event\":\"agent_start\",\"task\":\"$TASK\",\"timestamp\":\"$TIMESTAMP\"}" \
          >> ~/.claude/logs/agent-audit.jsonl

  # Log when any subagent completes
  - name: audit-agent-end
    matchers:
      operations: [AfterAgent]
    actions:
      inline_script: |
        #!/bin/bash
        INPUT=$(cat -)
        TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)
        TASK=$(echo "$INPUT" | jq -r '.tool_input.task // "unknown"')
        TOOL_COUNT=$(echo "$INPUT" | jq -r '.extra.tool_count // 0')
        echo "{\"event\":\"agent_end\",\"task\":\"$TASK\",\"tools_used\":$TOOL_COUNT,\"timestamp\":\"$TIMESTAMP\"}" \
          >> ~/.claude/logs/agent-audit.jsonl
```

**What happens**: Every subagent spawn and completion is recorded to `~/.claude/logs/agent-audit.jsonl` with timestamps and task descriptions. This creates a compliance-ready audit trail of all agent activity.

---

## Platform Support

Not all platforms support agent lifecycle events. Choose your hook strategy based on the platforms you target.

| Platform | BeforeAgent | AfterAgent | Notes |
|----------|-------------|------------|-------|
| **Claude Code** | Yes | Yes | Native support via `SubagentStart`/`SubagentStop` aliases |
| **Gemini CLI** | Yes (dual-fire) | Yes | `BeforeAgent` also fires `UserPromptSubmit` -- see [dual-fire events](../mastering-hooks/references/platform-adapters.md) |
| **GitHub Copilot** | No | No | Use `PreToolUse` for tool-level governance instead |
| **OpenCode** | No | No | Use `PreToolUse` for tool-level governance instead |
| **Codex CLI** | No hooks | No hooks | No hook support at all |

### Cross-Platform Strategy

If you need subagent governance across platforms:

1. **Always include `PreToolUse` rules** -- these work on every platform that supports hooks and catch tool calls inside subagents.
2. **Add `BeforeAgent`/`AfterAgent` rules as enhancements** -- these provide cleaner agent-level control on Claude Code and Gemini CLI.
3. **Use `rulez debug`** to verify behavior on each platform before deploying.

---

## Resolving Issue #107: Hooks Not Firing in Subagents

If your hooks work in the main conversation but not in subagents, the most common causes are:

1. **Wrong event type**: Use `PreToolUse` to catch tool calls inside subagents. `BeforeAgent` only fires once when the agent starts -- it does not fire for individual tool calls within the subagent.
2. **Scope mismatch**: Project-local hooks may not apply if the subagent changes working directory. Use `rulez install --global` for universal coverage.
3. **Missing registration**: The subagent runs a fresh process -- ensure RuleZ is registered in global settings, not just project settings.
4. **Platform limitation**: `BeforeAgent`/`AfterAgent` are only available on Claude Code and Gemini CLI. On other platforms, rely on `PreToolUse` exclusively.

### Diagnostic Steps

```bash
# Verify RuleZ is registered globally
cat ~/.claude/settings.json | grep -A10 hooks

# Test that your rule matches the expected event
rulez debug PreToolUse --tool Write --path src/main.rs -v

# Check BeforeAgent event specifically
rulez debug BeforeAgent -v

# Review audit logs for hook activity
rulez logs --limit 10
```

### See Also

- [Agent Inline Hooks Reference](../mastering-hooks/references/agent-inline-hooks.md) -- ephemeral hooks scoped to individual subagents
- [Troubleshooting Guide](../mastering-hooks/references/troubleshooting-guide.md) -- general hook debugging procedures
- [Platform Adapters Reference](../mastering-hooks/references/platform-adapters.md) -- cross-platform event mapping details
