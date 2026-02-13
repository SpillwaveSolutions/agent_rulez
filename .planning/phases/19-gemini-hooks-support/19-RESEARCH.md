# Phase 19: Gemini Hooks Support - Research

**Researched:** 2026-02-11
**Domain:** Gemini CLI hooks integration and RuleZ translation layer
**Confidence:** MEDIUM

## Summary

Gemini CLI hooks are synchronous, JSON-driven event handlers configured in `settings.json` across project, user, system, and extension scopes. Hooks execute external commands and communicate only via `stdin` (JSON input), `stdout` (JSON output), and `stderr` (logs/errors). The CLI enforces strict JSON output: any non-JSON on `stdout` causes parsing failure and defaults to allow, so hook runners must be strict about output hygiene. Hook events span the agent lifecycle (`BeforeAgent`, `AfterAgent`), model lifecycle (`BeforeModel`, `AfterModel`, `BeforeToolSelection`), tool lifecycle (`BeforeTool`, `AfterTool`), and system lifecycle (`SessionStart`, `SessionEnd`, `Notification`, `PreCompress`). Tool matchers use regexes; lifecycle matchers use exact strings.

To integrate RuleZ, the critical work is to map Gemini hook events and payloads into RuleZ event types (e.g., `BeforeTool` -> RuleZ PreToolUse equivalents, `AfterTool` -> PostToolUse), then translate RuleZ decisions back into Gemini’s structured JSON response schema. Output semantics are subtle: `decision: "deny"` blocks an action but differs by event; exit code 2 triggers a critical block with specific behavior differences between tool and agent/model events. For extension support, Gemini CLI loads extension hooks from `~/.gemini/extensions/<ext>/hooks/hooks.json`, with extension configs merged after system/user/project (lowest precedence). Hook security is an explicit concern: project hooks are fingerprinted and warned on first execution, and environment variable redaction is available but off by default.

**Primary recommendation:** Implement a dedicated Gemini hook adapter that (1) parses Gemini hook input schemas per event, (2) maps to RuleZ event types + payloads, (3) emits strict JSON output compatible with Gemini hook response semantics (including allow/deny/continue/reason), and (4) respects settings precedence and extension hook sources.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust 2024 + `serde_json` | current in repo | Parse/emit Gemini hook JSON | Canonical JSON handling in RuleZ stack |
| `regex` (Rust) | current in repo | Match Gemini tool hook regex matchers | Required for `matcher` behavior |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `tokio` | current in repo | Async execution of RuleZ policies and IO | If hook adapter or RuleZ pipeline is async |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Rust + `serde_json` | JS/Node hook runner | Inconsistent with RuleZ core; harder to share policy logic |

**Installation:**
```bash
# No new dependencies beyond existing RuleZ stack
```

## Architecture Patterns

### Recommended Project Structure
```
src/
├── adapters/            # Gemini/other host adapters
├── domain/              # RuleZ core policy/event types
├── io/                  # JSON parsing/serialization helpers
└── cli/                 # RuleZ CLI entry points
```

### Pattern 1: Adapter Translation Layer
**What:** Normalize Gemini hook inputs into RuleZ event payloads, then translate RuleZ decisions to Gemini hook responses.
**When to use:** Always for Gemini hook integration to isolate host-specific schemas.
**Example:**
```json
// Source: https://geminicli.com/docs/hooks/reference/
{
  "decision": "deny",
  "reason": "Security Policy: Potential secret detected in content.",
  "systemMessage": "Security scanner blocked operation"
}
```

### Pattern 2: Settings Precedence Handling
**What:** Merge configuration in priority order: project > user > system > extensions.
**When to use:** Loading hooks or building a consolidated hook plan from multiple scopes.
**Example:**
```json
// Source: https://geminicli.com/docs/hooks/
{
  "hooks": {
    "BeforeTool": [
      {
        "matcher": "write_file|replace",
        "hooks": [
          { "name": "security-check", "type": "command", "command": ".gemini/hooks/security.sh" }
        ]
      }
    ]
  }
}
```

### Anti-Patterns to Avoid
- **Printing to stdout before JSON:** Gemini treats non-JSON output as parsing failure and defaults to allow. Always log to `stderr`.
- **Hooking with `*` everywhere:** Excessive execution slows the agent loop; use specific matchers.
- **Using exit code 2 as default:** Prefer structured JSON with exit 0 for clean policy decisions.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JSON parsing | Manual string parsing | `serde_json` | Hook payloads are structured; manual parsing is brittle |
| Matcher evaluation | Custom ad-hoc matcher | Standard regex engine | Gemini matchers are regex for tool events |

**Key insight:** Gemini hook protocols are strict JSON contracts; correctness beats convenience.

## Common Pitfalls

### Pitfall 1: Polluted stdout
**What goes wrong:** Any non-JSON output causes parsing failure and defaults to allow.
**Why it happens:** Debug `print` statements in hook output.
**How to avoid:** Write all logs to `stderr` only; validate JSON before emitting.
**Warning signs:** Hooks appear to be ignored; CLI shows system messages but policy not enforced.

### Pitfall 2: Exit code semantics mismatch
**What goes wrong:** Using exit code 2 in the wrong hook type causes unexpected blocking or retries.
**Why it happens:** Exit code 2 has different effects for tool events vs agent/model events.
**How to avoid:** Prefer `decision: "deny"` in JSON with exit 0 for normal policy blocks; reserve exit 2 for critical blocks.
**Warning signs:** Tool actions blocked but agent continues unexpectedly, or retries triggered unintentionally.

### Pitfall 3: Incorrect matcher scope
**What goes wrong:** Lifecycle hooks never fire when matcher is treated as regex.
**Why it happens:** Lifecycle matchers are exact strings; tool matchers are regex.
**How to avoid:** Use exact strings for lifecycle events and regex for tool events per docs.
**Warning signs:** SessionStart/SessionEnd hooks never execute.

## Code Examples

Verified patterns from official sources:

### BeforeTool hook: block secret writes
```bash
#!/usr/bin/env bash
# Source: https://geminicli.com/docs/hooks/writing-hooks/
input=$(cat)
content=$(echo "$input" | jq -r '.tool_input.content // .tool_input.new_string // ""')
if echo "$content" | grep -qE 'api[_-]?key|password|secret'; then
  echo "Blocked potential secret" >&2
  cat <<EOF
{"decision":"deny","reason":"Security Policy: Potential secret detected in content.","systemMessage":"Security scanner blocked operation"}
EOF
  exit 0
fi
echo '{"decision":"allow"}'
```

### AfterAgent hook: retry on invalid output
```json
// Source: https://geminicli.com/docs/hooks/reference/
{
  "decision": "deny",
  "reason": "Response is missing required section. Please add it.",
  "systemMessage": "Requesting missing section"
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Ad-hoc scripts without standard schemas | Strict JSON hook I/O schemas + event-specific inputs | Gemini CLI hooks system | Enables reliable policy enforcement and consistent tooling |

**Deprecated/outdated:**
- Writing to stdout for logs: breaks JSON parsing; use stderr only.

## Open Questions

1. **Exact tool name list for Gemini CLI**
   - What we know: Tool matchers are regex over tool names; examples include `write_file` and `replace`.
   - What’s unclear: Full tool name list to map or document for RuleZ users.
   - Recommendation: Reference Gemini CLI tools list from `/docs/tools` during implementation and add to adapter tests.

2. **RuleZ event taxonomy alignment**
   - What we know: Gemini provides `BeforeTool`, `AfterTool`, `BeforeAgent`, `AfterAgent`, `BeforeModel`, `AfterModel`, `BeforeToolSelection`, `SessionStart`, `SessionEnd`, `Notification`, `PreCompress`.
   - What’s unclear: Which RuleZ event types map cleanly to each Gemini event (beyond tool lifecycle).
   - Recommendation: Define a formal mapping table early in the plan and gate on integration tests per event.

## Sources

### Primary (HIGH confidence)
- https://geminicli.com/docs/hooks/reference/ - Hook schemas, event inputs/outputs, exit codes
- https://geminicli.com/docs/hooks/ - Hook events, configuration precedence, environment vars
- https://geminicli.com/docs/hooks/writing-hooks/ - Practical examples, exit strategies
- https://geminicli.com/docs/hooks/best-practices/ - Performance, debugging, security guidance
- https://geminicli.com/docs/extensions/reference/ - Extension hook location and format

## Metadata

**Confidence breakdown:**
- Standard stack: MEDIUM - based on current repo stack and Gemini hook JSON requirements
- Architecture: MEDIUM - adapter pattern is inferred from integration requirements + Gemini schemas
- Pitfalls: HIGH - directly from official docs

**Research date:** 2026-02-11
**Valid until:** 2026-03-11
