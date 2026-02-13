# Phase 20: Gemini CLI support and Gemini hooks support - Research

**Researched:** 2026-02-11
**Domain:** Gemini CLI hooks integration + RuleZ translation layer
**Confidence:** MEDIUM

## User Constraints (from CONTEXT.md)

No CONTEXT.md found for this phase.

## Summary

Gemini CLI hooks are scripts or programs executed synchronously with strict JSON I/O requirements. Hook events include lifecycle events (SessionStart/End, PreCompress, Notification), agent events (BeforeAgent/AfterAgent), model events (BeforeModel/AfterModel/BeforeToolSelection), and tool events (BeforeTool/AfterTool). Tool hooks use regex matchers against tool names (e.g., `write_file`, `replace`), while lifecycle hooks use exact string matchers. Output JSON supports `decision` (allow/deny), `reason`, `continue`, and hook-specific output fields; exit code `0` is preferred even for blocks, while exit code `2` is a system block that prevents the action and uses stderr as the reason.

RuleZ currently parses Claude Code hook events with a `hook_event_name` field and produces a Claude-style response (`continue`, `context`, `reason`). Planning for Gemini CLI support must account for different event names, different decision semantics, and a different output schema. A translation layer is required to map Gemini hook events to RuleZ EventType and to convert RuleZ responses into Gemini CLI JSON responses. The install path and configuration search for Gemini settings is distinct from the existing `.claude/hooks.yaml` flow; Gemini uses `.gemini/settings.json` (project), `~/.gemini/settings.json` (user), and system settings with explicit precedence. Extensions can bundle hooks in `hooks/hooks.json` under `~/.gemini/extensions/<extension>`, which must be merged with user and project settings; this is relevant to Gemini extension support requirements.

**Primary recommendation:** Implement a Gemini adapter that (1) parses Gemini hook input schema into a normalized internal event, (2) maps event types and tool names into RuleZ matchers, and (3) emits Gemini-compliant JSON output with correct semantics for `decision`, `reason`, `continue`, and hook-specific overrides.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Gemini CLI hooks schema | current docs | Input/output contracts for hooks | Official spec for event names, schemas, and semantics |
| `serde` / `serde_json` | existing repo | JSON parsing and serialization | Current implementation for hook event I/O |
| `regex` | existing repo | Matchers | Used for tool name filtering in RuleZ |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `chrono` | existing repo | Timestamp defaults | To align timestamps when Gemini input omits them |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Manual JSON parsing | `serde_json` | Manual parsing is error-prone and harder to keep aligned with schema changes |

**Installation:**
```bash
# No new dependencies expected beyond current Rust stack
```

## Architecture Patterns

### Recommended Project Structure
```
cch_cli/src/
├── adapters/            # Gemini + Claude event translation layers
├── hooks.rs             # Core RuleZ evaluation engine
├── models.rs            # Normalized event + response models
├── cli/                 # install/uninstall per CLI
└── config.rs            # Config loading (per CLI paths)
```

### Pattern 1: Event Normalization Adapter
**What:** Parse Gemini hook JSON into a GeminiEvent, normalize to RuleZ Event (tool_name/tool_input/event_type), then evaluate rules.
**When to use:** Any time the inbound schema differs from RuleZ/Claude input (Gemini hook schema does).
**Example:**
```rust
// Source: https://geminicli.com/docs/hooks/reference/
// Base Gemini input fields include session_id, transcript_path, cwd, hook_event_name, timestamp.
// Tool hooks add tool_name + tool_input, agent hooks add prompt/prompt_response.
// Adapter should map hook_event_name: "BeforeTool" -> EventType::PreToolUse
```

### Pattern 2: Response Translation Layer
**What:** Convert RuleZ Response into Gemini CLI output JSON with `decision`, `reason`, `continue`, and hookSpecificOutput.* fields.
**When to use:** Always for Gemini hook output; Claude-style `continue`/`context` are not sufficient.
**Example:**
```json
// Source: https://geminicli.com/docs/hooks/reference/
{
  "decision": "deny",
  "reason": "Security Policy: Potential secret detected in content",
  "systemMessage": "Security scanner blocked operation"
}
```

### Anti-Patterns to Avoid
- **Assuming Claude and Gemini semantics match:** Gemini uses `decision` allow/deny and `continue` for stopping the agent loop; Claude uses `continue` plus exit code 2 to block tools.
- **Writing logs to stdout:** Gemini requires stdout to be strictly JSON; logs must be stderr.
- **Ignoring config precedence:** Gemini merges project, user, system, and extensions; a single path lookup will miss hooks.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Gemini hook schemas | Custom ad-hoc schema | Official Gemini hook reference | Reduces mismatch risk as schemas evolve |
| Settings precedence | One-off path checks | Gemini settings layer rules | Prevents missed hooks or wrong overrides |

**Key insight:** Gemini CLI hook behavior is defined by strict schema and exit code semantics; any custom interpretation risks incorrect allow/deny behavior.

## Common Pitfalls

### Pitfall 1: Incorrect stdout behavior
**What goes wrong:** Hook outputs include logs or text outside JSON and Gemini falls back to allow.
**Why it happens:** Hooks are executed as command scripts; stdout pollution breaks parsing.
**How to avoid:** Ensure Gemini adapter prints only JSON; use stderr for logging.
**Warning signs:** Gemini reports a warning and allows blocked actions.

### Pitfall 2: Mapping `continue` incorrectly
**What goes wrong:** A block decision fails to stop a tool or stops the whole loop incorrectly.
**Why it happens:** Gemini `decision` (allow/deny) affects event-specific actions; `continue:false` halts the entire loop.
**How to avoid:** Use `decision: "deny"` for tool or turn blocks; reserve `continue:false` for stopping the agent loop.
**Warning signs:** Gemini stops the entire session when only a tool should be blocked.

### Pitfall 3: Ignoring hook matchers
**What goes wrong:** Hooks never fire or fire too often.
**Why it happens:** Tool hooks use regex against tool names; lifecycle hooks use exact strings; `*` or empty match all.
**How to avoid:** Respect matcher semantics per event type, and mirror Gemini tool names (`write_file`, `replace`).
**Warning signs:** No hook invocation even though settings are present.

### Pitfall 4: Missing extension hooks
**What goes wrong:** Extensions bundling hooks are ignored.
**Why it happens:** Hooks can be defined per extension in `hooks/hooks.json` under `~/.gemini/extensions/<name>` and merged at runtime.
**How to avoid:** Load extension hooks as an additional config layer.
**Warning signs:** Extension hooks show up in CLI but RuleZ does not run.

## Code Examples

Verified patterns from official sources:

### Gemini hook base input schema
```json
// Source: https://geminicli.com/docs/hooks/reference/
{
  "session_id": "...",
  "transcript_path": "/abs/path/to/transcript.json",
  "cwd": "/abs/path/to/project",
  "hook_event_name": "BeforeTool",
  "timestamp": "2026-02-11T12:34:56Z"
}
```

### BeforeTool input/output shape
```json
// Source: https://geminicli.com/docs/hooks/reference/
{
  "tool_name": "write_file",
  "tool_input": {
    "file_path": "/abs/path/file.txt",
    "content": "..."
  }
}
```

```json
// Source: https://geminicli.com/docs/hooks/reference/
{
  "decision": "deny",
  "reason": "Security Policy: Potential secret detected"
}
```

### AfterAgent retry semantics
```json
// Source: https://geminicli.com/docs/hooks/reference/
{
  "decision": "deny",
  "reason": "Please retry with a summary section"
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Ad-hoc hook config | Structured `settings.json` with hooks | 2025-09 (config format update) | Must parse new settings schema and layer precedence |
| Tool blocks via exit codes only | Structured JSON with `decision` | Current hooks spec | Must emit JSON even for deny decisions |

**Deprecated/outdated:**
- `settings.json` v1 format (pre-09/17/25) should not be assumed; new format is official.

## Open Questions

1. **Which Gemini hook events should map to existing RuleZ EventType vs new EventType values?**
   - What we know: Gemini uses BeforeTool/AfterTool/BeforeAgent/AfterAgent/etc.
   - What's unclear: Whether RuleZ should extend EventType to support Gemini-specific events (BeforeModel, BeforeToolSelection, AfterModel).
   - Recommendation: Add explicit Gemini event mapping table; extend EventType if needed rather than overloading existing values.

2. **Where should RuleZ read Gemini hook configuration and bundled hooks?**
   - What we know: Gemini uses `.gemini/settings.json` (project) and `~/.gemini/settings.json` (user); extensions bundle hooks at `~/.gemini/extensions/<name>/hooks/hooks.json`.
   - What's unclear: Requirement mentions `~/.gemini/hooks/` for extension hooks, which is not documented in official extensions reference.
   - Recommendation: Validate with Gemini CLI source or release notes; consider supporting both `~/.gemini/hooks/` and extension `hooks/hooks.json` as fallbacks.

3. **Decision semantics mapping for RuleZ modes (block/inject) to Gemini `decision`/`continue`/hookSpecificOutput fields**
   - What we know: Gemini expects `decision: allow/deny` and optional `hookSpecificOutput` for overrides.
   - What's unclear: How to represent RuleZ inject context for BeforeTool/AfterTool/BeforeAgent in Gemini (likely `hookSpecificOutput.additionalContext` or tool_input override).
   - Recommendation: Define a deterministic mapping per event type in the adapter and document it.

## Sources

### Primary (HIGH confidence)
- https://geminicli.com/docs/hooks/ - Hook overview and event list
- https://geminicli.com/docs/hooks/reference/ - Hook input/output schemas and semantics
- https://geminicli.com/docs/tools/file-system/ - Tool names and parameters (write_file, replace)
- https://geminicli.com/docs/extensions/reference/ - Extension hook location and format
- https://geminicli.com/docs/get-started/configuration/ - Settings.json paths and precedence

### Secondary (MEDIUM confidence)
- https://developers.googleblog.com/tailor-gemini-cli-to-your-workflow-with-hooks/ - Release context and usage patterns

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Gemini docs and existing Rust stack are explicit
- Architecture: MEDIUM - Adapter design inferred from current CCH CLI structure
- Pitfalls: HIGH - Hook semantics and stdout rules are explicit in Gemini docs

**Research date:** 2026-02-11
**Valid until:** 2026-03-13
