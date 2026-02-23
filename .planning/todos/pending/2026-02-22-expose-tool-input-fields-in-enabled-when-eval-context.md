---
created: 2026-02-22T00:23:11.313Z
title: Expose tool_input fields in enabled_when eval context
area: tooling
files:
  - rulez/src/hooks.rs:556-584
  - rulez/src/models.rs:2590-2600
---

## Problem

The `build_eval_context()` function in `hooks.rs:556-584` only exposes `tool_name`, `event_type`, `prompt`, and `env_*` variables to `enabled_when` expressions. It does NOT expose `tool_input` fields like `source`, `reason`, `command`, `path`, etc.

This means rules cannot filter on payload values. For example, a `SessionStart` rule cannot distinguish between `source=="compact"` (post-compaction resume) and `source=="startup"` (fresh session). The `source` field IS already parsed into `EventDetails::Session` (`models.rs:2592-2599`) but never surfaced to the eval context.

Use case: Users want rules like "re-inject skill context only when session starts after compaction" — requires `enabled_when: 'source == "compact"'`.

## Solution

Expand `build_eval_context()` to iterate over `event.tool_input` (when it's a JSON object) and expose each top-level field as an eval variable. ~10 lines of code:

```rust
// In build_eval_context(), after existing variables:
if let Some(Value::Object(map)) = &event.tool_input {
    for (key, value) in map {
        if let Some(s) = value.as_str() {
            ctx.set_value(key.clone(), Value::String(s.to_string())).ok();
        } else if let Some(b) = value.as_bool() {
            ctx.set_value(key.clone(), Value::Boolean(b)).ok();
        } else if let Some(n) = value.as_i64() {
            ctx.set_value(key.clone(), Value::Int(n)).ok();
        }
    }
}
```

This enables: `enabled_when: 'source == "compact"'` on SessionStart rules, `enabled_when: 'command =~ "rm -rf"'` on PreToolUse rules, etc.

Phase 22.1 scope — small, self-contained change with high value.
