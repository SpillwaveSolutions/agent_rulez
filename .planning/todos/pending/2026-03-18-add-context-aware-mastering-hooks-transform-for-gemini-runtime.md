---
created: 2026-03-18T05:01:11.076Z
title: Add context-aware mastering-hooks transform for Gemini runtime
area: cli
files:
  - rulez/src/skills/transform.rs
  - rulez/src/skills/transforms/
  - rulez/src/cli/skills.rs:200-212
---

## Problem

Requirement CONFIG-04 from v2.3.0 (Multi-Runtime Skill Portability) is only partially implemented. The mastering-hooks skill is discovered and installed but passes through the **same generic `TransformPipeline`** as all other skills — no special handling exists.

`mastering-hooks` describes Claude Code-specific hook integration (hooks.yaml, rule evaluation, PreToolUse/PostToolUse events). When installed for Gemini, the generic `PathRefTransform` rewrites `.claude/` → `.gemini/` mechanically, but the conceptual content (hook mechanisms, rule matching) has no Gemini equivalent. The result is a misleading Gemini skill that references Claude Code concepts verbatim.

Specific gap found during v2.3.0 milestone audit:
- `discover_inventory()` in `cli/skills.rs:200-212` adds mastering-hooks as an extra skill ✓
- No downstream code in `transform.rs` or any `transforms/` file checks `skill.name == "mastering-hooks"` ✗
- CONFIG-04 requirement clause "context-aware handling" is unimplemented

## Solution

In `rulez/src/skills/transform.rs` (or a new `transforms/mastering_hooks.rs`), add a `MasteringHooksTransform` that fires only when `skill.source_path` contains `mastering-hooks`:

- For `Runtime::Gemini`: either (a) skip install entirely with a clear warning ("mastering-hooks is Claude Code-specific, skipping for Gemini"), or (b) inject a preamble note explaining the skill describes Claude Code hooks and may not apply
- For `Runtime::OpenCode` / `Runtime::Codex`: same consideration — hooks model differs
- For `Runtime::Claude`: pass through unchanged (canonical source)

Option (a) is simpler and more honest. Option (b) is better UX if users want a reference copy.

Wire into `TransformPipeline::for_runtime()` so it runs before the generic path-ref transform.
