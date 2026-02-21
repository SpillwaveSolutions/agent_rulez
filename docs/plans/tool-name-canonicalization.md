# Plan: Tool Name Canonicalization Across Platforms

## Context

RuleZ already standardizes **event types** across platforms (Claude Code, Gemini CLI, Copilot, OpenCode) via `map_event_type()` functions in each adapter. However, **tool names** pass through as-is with no normalization. This means a rule like `tools: [Bash]` works on Claude Code but silently does nothing on Gemini CLI (where the tool is called `execute_code`) or OpenCode (where it's `bash`).

This plan proposes adding tool name canonicalization — mapping platform-specific tool names to a universal set, following the same pattern already established for event types.

## Current State

### What IS standardized: Event Types
Each adapter has a `map_event_type()` function that translates platform-native event names to canonical `EventType` enum values. This works well.

### What is NOT standardized: Tool Names
Tool names pass through untouched in every adapter:
```rust
// gemini.rs:62, copilot.rs:49, opencode.rs:61
let event = Event {
    tool_name: input.tool_name,  // direct pass-through, no mapping
    ...
};
```

The matcher in `hooks.rs:673-681` does case-sensitive exact matching:
```rust
if !tools.contains(tool_name) {
    return false;
}
```

### Known Tool Name Differences

| Canonical (Claude Code) | Gemini CLI | OpenCode | Copilot |
|------------------------|------------|----------|---------|
| `Bash`                 | `execute_code` | `bash` | `Bash` / `shell` |
| `Write`                | `write_file`   | `write` | `Write` |
| `Edit`                 | `replace`      | `edit`  | `Edit` |
| `Read`                 | `read_file`    | `read`  | `Read` |
| `Glob`                 | `list_files`   | `glob`  | `Glob` |
| `Grep`                 | `search_files` | `grep`  | `Grep` |
| `Task`                 | `run_agent`    | `task`  | `Task` |
| `WebFetch`             | `web_fetch`    | `fetch` | `WebFetch` |
| `TodoRead`             | —              | —       | `TodoRead` |
| `TodoWrite`            | —              | —       | `TodoWrite` |

> **Note:** Gemini and OpenCode tool names need verification against actual platform docs. The table above is based on test files and common patterns. A research spike should confirm exact names before implementation.

## Proposed Approach

### Option A: Adapter-Level Mapping (Recommended)

Add a `map_tool_name()` function to each adapter, mirroring the existing `map_event_type()` pattern. This normalizes tool names to Claude Code's PascalCase canonical names at ingestion time.

**Pros:**
- Follows existing pattern (`map_event_type` already works this way)
- Rules always use canonical names — simple for users
- Zero changes needed in matching logic (`hooks.rs`)
- `EventDetails::extract()` already uses canonical names internally

**Cons:**
- Must maintain mapping tables per adapter
- Users lose visibility into the original platform tool name (mitigated by preserving original in `tool_input`)

#### Implementation sketch

Each adapter gets a function like:
```rust
fn map_tool_name(platform_name: &str) -> String {
    match platform_name {
        "execute_code" => "Bash".to_string(),
        "write_file" => "Write".to_string(),
        "replace" => "Edit".to_string(),
        "read_file" => "Read".to_string(),
        "list_files" => "Glob".to_string(),
        "search_files" => "Grep".to_string(),
        _ => platform_name.to_string(), // pass through unknown tools
    }
}
```

And the event construction changes from:
```rust
tool_name: input.tool_name,
```
to:
```rust
tool_name: input.tool_name.map(|n| map_tool_name(&n)),
```

Optionally preserve the original name:
```rust
// Add original_tool_name to tool_input for debugging/logging
if preserve_name { tool_input["platform_tool_name"] = original_name; }
```

### Option B: Matcher-Level Normalization (Alternative)

Instead of mapping at ingestion, expand the matcher to check aliases. The `tools` matcher would accept canonical names and internally check a lookup table.

**Pros:**
- No adapter changes needed
- Original tool names preserved on the Event

**Cons:**
- Matching logic becomes more complex
- Every rule evaluation pays the lookup cost
- `EventDetails::extract()` still needs canonical names (so you'd need mapping there too)
- Breaks the clean separation between adapters and core

### Option C: Case-Insensitive + Alias Config (Hybrid)

Make tool matching case-insensitive and allow users to define tool aliases in `hooks.yaml`:
```yaml
tool_aliases:
  Bash: [execute_code, shell, bash]
  Write: [write_file, write]
```

**Pros:**
- User-configurable, handles unknown platforms
- Case-insensitive helps with trivial differences

**Cons:**
- Adds config complexity
- Users must know platform-specific names
- Doesn't solve the problem for casual users

## Recommended: Option A (Adapter-Level Mapping)

### Files to Modify

| File | Change |
|------|--------|
| `rulez/src/adapters/gemini.rs` | Add `map_tool_name()`, apply in `parse_event()` |
| `rulez/src/adapters/copilot.rs` | Add `map_tool_name()`, apply in `parse_event()` |
| `rulez/src/adapters/opencode.rs` | Add `map_tool_name()`, apply in `parse_event()` |
| `rulez/src/adapters/mod.rs` | Optional: shared `CanonicalTool` enum or common mapping util |
| `docs/TOOL-MAPPING.md` | New doc (mirrors EVENT-MAPPING.md format) |
| `rulez/tests/gemini_adapter.rs` | Update tests to verify tool name normalization |
| `rulez/tests/copilot_adapter.rs` | Update tests to verify tool name normalization |
| `rulez/tests/opencode_payload_tests.rs` | Update tests to verify tool name normalization |

### Implementation Steps

1. **Research spike**: Confirm exact tool names for each platform from their official docs
2. **Add `map_tool_name()` to each adapter** with platform-specific mappings
3. **Preserve original tool name** in `tool_input` as `platform_tool_name` field
4. **Add `TOOL-MAPPING.md`** documenting all mappings (like EVENT-MAPPING.md)
5. **Update adapter tests** to verify normalization
6. **Update Rule Explorer UI** to show canonical tool names (already done — uses Claude Code names)
7. **Add `rulez explain` output** showing tool name mapping when relevant

### Verification

- Existing tests pass (canonical names already used in test events)
- New adapter tests confirm `write_file` → `Write`, `execute_code` → `Bash`, etc.
- `rulez debug --adapter gemini PreToolUse --tool write_file` matches rules with `tools: [Write]`
- Unknown tool names pass through unchanged
- Original platform tool name preserved in `tool_input.platform_tool_name`

## Open Questions

1. **Should unknown tool names pass through as-is or be normalized to PascalCase?** (Recommendation: pass through as-is to avoid breaking user expectations for platform-specific tools)
2. **Should we support reverse mapping?** (e.g., in `rulez explain` showing "Gemini calls this `write_file`")
3. **Exact Gemini/OpenCode/Copilot tool names need verification** against current platform docs — test files may not cover all tools
4. **Should the Rule Explorer UI show platform-specific names?** (e.g., a dropdown to see "On Gemini, this tool is called `write_file`")
