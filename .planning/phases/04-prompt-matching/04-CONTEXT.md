# Phase 4: Prompt Matching - Context

**Gathered:** 2026-02-08
**Status:** Ready for planning

<domain>
## Phase Boundary

Users can route rules based on prompt text patterns, enabling intent-based policy enforcement. This phase adds `prompt_match` as a new matcher type alongside existing `tool_name`, `command_match`, and `path` matchers.

Requirements: PROMPT-01 through PROMPT-05 (5 requirements)

</domain>

<decisions>
## Implementation Decisions

### Pattern Syntax
- Case-sensitive by default (consistent with regex defaults), opt-in for case-insensitive via flag
- Support both regex anchors (^, $) AND a convenience `anchor` field (start | end | contains)
- Add shorthands for common patterns to improve readability:
  - `contains_word: 'delete'` expands to `\bdelete\b`
  - Full regex still available for power users

### Multiple Pattern Logic
- Default to ANY (OR) logic — rule matches if any pattern matches
- Support both syntaxes:
  - Array syntax for simple case: `prompt_match: ['pattern1', 'pattern2']`
  - Object syntax when mode needed: `prompt_match: { patterns: [...], mode: all }`
- Support negation with `not:` prefix: `not: 'pattern'` to exclude matches

### Match Target
- Match against full prompt text (not just first line)
- No normalization — match raw prompt text as-is
- Missing prompt field = rule doesn't match (safe default)
- Works on all event types that have a prompt field

### Script Matching Behavior
- Full event context available: prompt, tool_name, event_type, env vars
- Configurable error handling with fail-closed as default (error = no match, log warning)

### Claude's Discretion
- Pattern type choice (regex-only vs literal+regex flag)
- Nested group support for complex logic (keep it simple if complexity outweighs benefit)
- Script helper functions (contains_word, line_count, etc.)
- YAML field naming (prompt_script vs inside prompt_match object)

</decisions>

<specifics>
## Specific Ideas

- Shorthands should make rules more readable for common cases like "block if prompt contains word 'delete'"
- Power users should still have full regex access
- Follow existing RuleZ patterns for consistency (command_match is a good reference)

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 04-prompt-matching*
*Context gathered: 2026-02-08*
