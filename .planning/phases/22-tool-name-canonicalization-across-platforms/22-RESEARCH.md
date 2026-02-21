# Phase 22: Tool Name Canonicalization Across Platforms - Research

**Researched:** 2026-02-20
**Domain:** Rust adapter pattern, cross-platform tool name normalization
**Confidence:** HIGH (for Rust patterns and current code state), MEDIUM (for platform tool name lists)

---

## Summary

Phase 22 adds `map_tool_name()` to the three non-Claude-Code adapters (Gemini, Copilot, OpenCode) so that a rule like `tools: [Bash]` fires correctly regardless of which platform sends the hook. The architectural decision (Option A: adapter-level mapping) is already made and the functions have already been added to all three adapter files. However, the work is **incomplete and currently broken**:

1. The codebase **does not compile** due to a Rust ownership/move bug in the `if let` destructuring pattern used in `gemini.rs` and `copilot.rs` — the `opencode.rs` adapter uses a different (working) pattern.
2. Adapter tests still **assert the pre-canonicalization (platform-native) tool names** — they need to be updated to assert canonical names and check `platform_tool_name` in `tool_input`.
3. The verified Gemini CLI tool names differ from what the current `gemini.rs` mapping contains — several key mappings use incorrect platform-side names.
4. `TOOL-MAPPING.md` has not been created yet.

**Primary recommendation:** Fix the compile bug first (applies the OpenCode pattern to Gemini/Copilot), correct the Gemini tool name mappings against official docs, update all three test files to assert canonical names, and create `TOOL-MAPPING.md`.

---

## Current State of Work

### What Is Already Done

All three adapter files have `map_tool_name()` functions and the call-site logic to canonicalize:

- `/rulez/src/adapters/gemini.rs` — `map_tool_name()` present; call site uses `if let` destructuring (broken)
- `/rulez/src/adapters/copilot.rs` — `map_tool_name()` present; call site uses `if let` destructuring (broken)
- `/rulez/src/adapters/opencode.rs` — `map_tool_name()` present; call site correctly mutates `tool_input` Map directly (working)

### What Is Broken

**Compile error (HIGH confidence — verified by `cargo build`):**

Both `gemini.rs` and `copilot.rs` use this pattern:

```rust
// gemini.rs lines 75-85, copilot.rs lines 62-72
let tool_input = if let (Some(ref orig), Some(Value::Object(mut map))) =
    (original_tool_name, tool_input)   // <-- tool_input is MOVED into the tuple here
{
    map.insert("platform_tool_name".to_string(), Value::String(orig.clone()));
    Some(Value::Object(map))
} else {
    tool_input   // <-- ERROR: tool_input already moved above
};
```

The Rust borrow checker rejects this because `tool_input` is moved into the `if let` destructuring pattern, then referenced again in the `else` branch.

**The fix is the OpenCode pattern** (lines 73-79 of `opencode.rs`), which avoids the `if let` entirely by operating directly on the `Map` struct that was already unwrapped:

```rust
// opencode.rs — WORKING pattern
let (canonical_tool_name, original_tool_name) = match input.tool_name {
    Some(ref name) => { ... }
    None => (None, None),
};

if let Some(ref orig) = original_tool_name {
    tool_input.insert(           // tool_input is a Map<String, Value>, not Option<Value>
        "platform_tool_name".to_string(),
        Value::String(orig.clone()),
    );
}
```

For `gemini.rs` and `copilot.rs`, the fix requires either:
- (a) Converting the `Option<Value>` `tool_input` into a `Map<String, Value>` before the canonicalization block (as OpenCode does), or
- (b) Replacing the `if let` with an explicit match that handles the `None` case without referencing the moved value.

### What Tests Assert (Stale — Need Updating)

| Test file | Line | Current assertion | Required assertion |
|-----------|------|-------------------|--------------------|
| `gemini_adapter.rs` | 20 | `Some("write_file")` | `Some("Write")` |
| `gemini_adapter.rs` | (none yet) | missing `platform_tool_name` check | add `platform_tool_name == "write_file"` |
| `copilot_adapter.rs` | 19 | `Some("shell")` | `Some("Bash")` |
| `copilot_adapter.rs` | (none yet) | missing `platform_tool_name` check | add `platform_tool_name == "shell"` |
| `opencode_payload_tests.rs` | 20 | `Some("bash")` | `Some("Bash")` |
| `opencode_payload_tests.rs` | (none yet) | missing `platform_tool_name` check | add `platform_tool_name == "bash"` |

---

## Standard Stack

This phase is pure Rust — no new dependencies are needed.

### Core
| Component | Version | Purpose |
|-----------|---------|---------|
| `serde_json::Map<String, Value>` | 1.x (existing) | Tool input mutation |
| `serde_json::Value` | 1.x (existing) | JSON event payloads |
| Rust pattern matching | stable | Ownership-safe conditional mutation |

### No New Dependencies
This phase uses only what is already in `Cargo.toml`. No `cargo add` steps required.

---

## Architecture Patterns

### Recommended: Follow the OpenCode Pattern

The `opencode.rs` adapter correctly solved the same problem. The planner should specify that `gemini.rs` and `copilot.rs` refactor to match it.

**OpenCode pattern (working):**

```rust
// Step 1: Build tool_input as a flat Map first (not Option<Value>)
let mut tool_input: Map<String, Value> = match input.tool_input {
    Some(Value::Object(map)) => map,
    Some(v) => { let mut m = Map::new(); m.insert("tool_input".to_string(), v); m }
    None => Map::new(),
};

// Merge extras into the map
for (k, v) in input.extra {
    tool_input.entry(k).or_insert(v);
}

// Step 2: Canonicalize tool name
let (canonical_tool_name, original_tool_name) = match input.tool_name {
    Some(ref name) => {
        let canonical = map_tool_name(name);
        let original = if canonical != *name { Some(name.clone()) } else { None };
        (Some(canonical), original)
    }
    None => (None, None),
};

// Step 3: Inject platform_tool_name — no borrow issue because tool_input is Map, not Option
if let Some(ref orig) = original_tool_name {
    tool_input.insert(
        "platform_tool_name".to_string(),
        Value::String(orig.clone()),
    );
}

// Step 4: Wrap in Option for the Event struct
let event = Event {
    tool_input: if tool_input.is_empty() { None } else { Some(Value::Object(tool_input)) },
    ...
};
```

**Note for Gemini:** `merge_tool_input()` is a helper that returns `Option<Value>`. The refactor can either inline this logic (as OpenCode does) or change `merge_tool_input` to return `Map<String, Value>` directly.

### Anti-Patterns to Avoid

- **Moving an Option into an `if let` then using it in the `else` branch.** The Rust borrow checker will reject this. Never do:
  ```rust
  let tool_input = if let (_, Some(Value::Object(mut map))) = (x, tool_input) {
      // map is here
  } else {
      tool_input  // ERROR: already moved
  };
  ```
- **Adding the `map_tool_name` call after `merge_tool_input` with an `Option`-based approach** without an explicit rebinding. Always unwrap to `Map` first.

---

## Verified Tool Name Mappings

### Gemini CLI — CORRECTIONS NEEDED (MEDIUM confidence)

The **official Gemini CLI documentation** (fetched from `geminicli.com/docs/tools` and `github.com/google-gemini/gemini-cli/blob/main/docs/tools/`) lists these tool names:

| Canonical (Claude Code) | Correct Gemini Name | Current gemini.rs mapping | Verdict |
|------------------------|---------------------|---------------------------|---------|
| `Bash` | `run_shell_command` | `"execute_code"` | **WRONG — needs fix** |
| `Write` | `write_file` | `"write_file"` | Correct |
| `Edit` | `replace` | `"replace"` | Correct |
| `Read` | `read_file` | `"read_file"` | Correct |
| `Glob` | `glob` | `"list_files"` | **WRONG — needs fix** |
| `Grep` | `search_file_content` or `grep_search` | `"search_files"` | **WRONG — needs fix** |
| `Task` | unknown/not confirmed | `"run_agent"` | **UNVERIFIED** |
| `WebFetch` | `web_fetch` | `"web_fetch"` | Correct |

**Additional Gemini tools without Claude Code equivalents:**
- `list_directory` — directory listing (no Claude Code analog, pass through)
- `read_many_files` — multi-file read (no analog, pass through)
- `google_web_search` — web search (no analog, pass through)
- `ask_user` — user interaction (no analog, pass through)
- `save_memory` — memory storage (no analog, pass through)
- `write_todos` — todo management (no analog, pass through)
- `activate_skill` — skill loading (no analog, pass through)

**Confidence:** MEDIUM. Tool names confirmed via official Gemini CLI docs pages at `geminicli.com` and the GitHub docs directory. The `grep_search` vs `search_file_content` discrepancy needs a single authoritative source — both were found in different docs sections. The `run_agent` mapping for `Task` was not confirmed in any official source.

**Action required:** Before finalizing `gemini.rs`, verify `grep_search` vs `search_file_content` against the Gemini CLI source at `packages/core/src/tools/grep.ts` and confirm whether a `run_agent` or subagent-invocation tool exists.

### OpenCode — CONFIRMED (HIGH confidence)

The **official OpenCode documentation** at `opencode.ai/docs/tools/` lists these tool names used in `tool.execute.before` events:

| Canonical (Claude Code) | OpenCode Tool Name | Current opencode.rs mapping | Verdict |
|------------------------|--------------------|-----------------------------|---------|
| `Bash` | `bash` | `"bash"` | Correct |
| `Write` | `write` | `"write"` | Correct |
| `Edit` | `edit` | `"edit"` | Correct |
| `Read` | `read` | `"read"` | Correct |
| `Glob` | `glob` | `"glob"` | Correct |
| `Grep` | `grep` | `"grep"` | Correct |
| `Task` | `task` | `"task"` | Correct |
| `WebFetch` | `webfetch` | `"fetch"` | **POSSIBLE ISSUE — verify** |

**Additional OpenCode tools without Claude Code equivalents (pass through unchanged):**
- `list` — list directory contents
- `lsp` — LSP server interactions
- `patch` — apply patch files
- `skill` — load skill files
- `todowrite` — create/update task lists
- `todoread` — read task lists
- `websearch` — web search
- `question` — ask users questions

**Confidence:** HIGH for the core 8 tools. The `webfetch` vs `fetch` mapping should be verified — the OpenCode docs list `webfetch` as the tool name, but the current `opencode.rs` maps `"fetch"` → `"WebFetch"`. The correct mapping may be `"webfetch"` → `"WebFetch"`.

### Copilot — LOW confidence (insufficient official docs)

GitHub Copilot's official hooks documentation does not enumerate the tool names that appear in `preToolUse` / `postToolUse` payloads. The current `copilot.rs` mapping is:

| Canonical (Claude Code) | Copilot Tool Name | Current copilot.rs mapping |
|------------------------|-------------------|---------------------------|
| `Bash` | `shell` (or `Bash`?) | `"shell"` → `"Bash"` |
| `Write` | `write` | `"write"` → `"Write"` |
| `Edit` | `edit` | `"edit"` → `"Edit"` |
| `Read` | `read` | `"read"` → `"Read"` |
| `Glob` | `glob` | `"glob"` → `"Glob"` |
| `Grep` | `grep` | `"grep"` → `"Grep"` |
| `Task` | `task` | `"task"` → `"Task"` |
| `WebFetch` | `fetch` | `"fetch"` → `"WebFetch"` |

**TodoRead / TodoWrite** are listed in the plan document as Copilot-only tools with no mapping needed (they pass through as-is).

**Note from plan doc:** Copilot PascalCase names (`Bash`, `Write`, `Edit`, etc.) match the canonical names and would pass through unchanged — the lowercase variants (`shell`, `write`, `edit`) are what need mapping.

**Confidence:** LOW. No official docs confirmed these names. The Phase 21 decisions mention Copilot format details (permissionDecision, .github/hooks/*.json format) but not tool names. Treat as hypothesis requiring validation if the Copilot adapter is to be production-ready. The plan document itself notes this needs verification.

---

## Tool Matching Logic in hooks.rs

The current matcher at `hooks.rs:673-681` is case-sensitive exact match:

```rust
if let Some(ref tools) = matchers.tools {
    if let Some(ref tool_name) = event.tool_name {
        if !tools.contains(tool_name) {
            return false;
        }
    } else {
        return false;
    }
}
```

**This does NOT need to change** if all adapters canonicalize correctly at ingestion time. The plan document (Option A, already selected) explicitly avoids changing the matcher.

**Case-insensitive fallback (open question from plan doc):** The plan asks whether the matcher should also support case-insensitive matching as a fallback. Research finding: this adds complexity and breaks the clean adapter boundary. Since unknown tool names pass through unchanged, adding case-insensitive matching in hooks.rs would only help if an adapter fails to map a known tool — which means the adapter mapping is wrong. Better to fix the mapping. **Recommendation: no case-insensitive fallback in hooks.rs.**

---

## Common Pitfalls

### Pitfall 1: Rust Move Semantics in if-let Destructuring
**What goes wrong:** The `if let (x, option_value) = (a, option)` pattern moves `option` into the `if let` scope. The `else` branch cannot reference `option` again.
**Why it happens:** This is exactly the current bug in `gemini.rs` and `copilot.rs`.
**How to avoid:** Unwrap `Option<Value>` to `Map<String, Value>` *before* the canonicalization block (OpenCode pattern). Then there is no Option left to move.
**Warning signs:** `error[E0382]: use of moved value: tool_input` from `cargo build`.

### Pitfall 2: Wrong Gemini Tool Names
**What goes wrong:** Tests and rules use names like `execute_code` or `list_files` that Gemini CLI does not actually emit — the real names are `run_shell_command`, `glob`, `search_file_content`.
**Why it happens:** The original plan document notes these were "based on test files and common patterns" and explicitly flags them as needing verification.
**How to avoid:** Use the verified names from `geminicli.com/docs/tools/` (fetched 2026-02-20). Update `gemini.rs` accordingly.
**Warning signs:** Integration tests against real Gemini CLI hooks fail with no-match for tools rules.

### Pitfall 3: Stale Test Assertions After Canonicalization
**What goes wrong:** Tests assert `tool_name == "bash"` (platform name) instead of `tool_name == "Bash"` (canonical name). They pass before canonicalization is active but fail after.
**Why it happens:** The three adapter test files were written when tool names passed through unchanged. The canonicalization was added to the adapters but tests were not updated.
**How to avoid:** After each adapter fix, update the corresponding test to:
  1. Assert `tool_name == "Bash"` (canonical)
  2. Assert `tool_input["platform_tool_name"] == "bash"` (original preserved)
  3. Also add a test for a tool name that doesn't map (passes through unchanged, no `platform_tool_name` injected)

### Pitfall 4: Missing Test for Unknown Tool Names
**What goes wrong:** No test verifies that unknown tools pass through unchanged without `platform_tool_name` being injected.
**Why it happens:** Easy to forget edge case. The `_ => platform_name.to_string()` branch in `map_tool_name()` needs test coverage.
**How to avoid:** Add a test for each adapter where `tool_name` is `"some_custom_mcp_tool"` and verify both `tool_name == "some_custom_mcp_tool"` and no `platform_tool_name` key in `tool_input`.

### Pitfall 5: OpenCode `webfetch` vs `fetch`
**What goes wrong:** The current `opencode.rs` maps `"fetch"` → `"WebFetch"`, but OpenCode's official docs list the tool as `webfetch`.
**Why it happens:** Mismatch between what the code assumed and what the platform actually uses.
**How to avoid:** Test with `tool_name: "webfetch"` and verify it maps to `"WebFetch"`. If OpenCode uses `webfetch`, add `"webfetch"` → `"WebFetch"` to the mapping (keeping `"fetch"` as a fallback or removing it).

---

## Code Examples

### Correct Fix Pattern for gemini.rs and copilot.rs

Apply the OpenCode approach — flatten to `Map` before canonicalization:

```rust
// Source: rulez/src/adapters/opencode.rs (existing working code)

pub fn parse_event(value: Value) -> Result<GeminiEvent> {
    let input: GeminiHookInput = serde_json::from_value(value)?;
    let mappings = map_event_type(&input.hook_event_name, input.tool_input.as_ref(), &input.extra);
    let (primary_event_type, is_tool_event) = mappings[0];
    let additional_event_types: Vec<EventType> =
        mappings.iter().skip(1).map(|(et, _)| *et).collect();

    // Flatten tool_input + extra into a single Map first
    let mut tool_input_map: Map<String, Value> = match input.tool_input {
        Some(Value::Object(map)) => map,
        Some(v) => { let mut m = Map::new(); m.insert("tool_input".to_string(), v); m }
        None => Map::new(),
    };
    for (k, v) in input.extra {
        tool_input_map.entry(k).or_insert(v);
    }

    // Preserve event name if mapping changed it
    let preserve_name = input.hook_event_name != primary_event_type.to_string();
    if preserve_name {
        tool_input_map
            .entry("gemini_hook_event_name".to_string())
            .or_insert(Value::String(input.hook_event_name.clone()));
    }

    // Canonicalize tool name — no borrow issue since tool_input_map is already a Map
    let (canonical_tool_name, original_tool_name) = match input.tool_name {
        Some(ref name) => {
            let canonical = map_tool_name(name);
            let original = if canonical != *name { Some(name.clone()) } else { None };
            (Some(canonical), original)
        }
        None => (None, None),
    };

    if let Some(ref orig) = original_tool_name {
        tool_input_map.insert(
            "platform_tool_name".to_string(),
            Value::String(orig.clone()),
        );
    }

    let tool_input = if tool_input_map.is_empty() { None } else { Some(Value::Object(tool_input_map)) };

    let event = Event {
        hook_event_name: primary_event_type,
        tool_name: canonical_tool_name,
        tool_input,
        ...
    };
    Ok(GeminiEvent { hook_event_name: input.hook_event_name, event, is_tool_event, additional_event_types })
}
```

Note: This refactor inlines `merge_tool_input()`. The helper function can be removed or kept for other uses.

### Test Pattern for Canonicalization

Each adapter test should now cover three cases:

```rust
// Source: pattern from existing adapter tests + new requirements

// Case 1: Tool name IS mapped — assert canonical name + platform_tool_name preserved
#[test]
fn test_tool_name_canonicalized() {
    let input = json!({
        "session_id": "s1",
        "hook_event_name": "BeforeTool",  // or "preToolUse", "tool.execute.before"
        "tool_name": "write_file",        // platform name
        "tool_input": { "file_path": "/tmp/test.txt", "content": "hi" }
    });
    let parsed = parse_event(input).unwrap();
    assert_eq!(parsed.event.tool_name.as_deref(), Some("Write"));  // canonical
    let map = parsed.event.tool_input.unwrap();
    assert_eq!(map["platform_tool_name"].as_str(), Some("write_file"));
}

// Case 2: Tool name is NOT mapped — passes through, no platform_tool_name injected
#[test]
fn test_unknown_tool_name_passes_through() {
    let input = json!({
        "session_id": "s1",
        "hook_event_name": "BeforeTool",
        "tool_name": "mcp__server__custom_tool",
        "tool_input": { "arg": "val" }
    });
    let parsed = parse_event(input).unwrap();
    assert_eq!(parsed.event.tool_name.as_deref(), Some("mcp__server__custom_tool"));
    let map = parsed.event.tool_input.unwrap();
    assert!(map.get("platform_tool_name").is_none());
}

// Case 3: No tool_name field — tool_name is None, no platform_tool_name
#[test]
fn test_event_with_no_tool_name() {
    let input = json!({
        "session_id": "s1",
        "hook_event_name": "BeforeAgent"
    });
    let parsed = parse_event(input).unwrap();
    assert!(parsed.event.tool_name.is_none());
}
```

### Corrected Gemini map_tool_name

```rust
// Based on verified names from geminicli.com/docs/tools/ (2026-02-20)
fn map_tool_name(platform_name: &str) -> String {
    match platform_name {
        "run_shell_command" => "Bash".to_string(),    // was "execute_code" — WRONG
        "write_file" => "Write".to_string(),
        "replace" => "Edit".to_string(),
        "read_file" => "Read".to_string(),
        "glob" => "Glob".to_string(),                  // was "list_files" — WRONG
        "search_file_content" | "grep_search" => "Grep".to_string(),  // was "search_files"
        "web_fetch" => "WebFetch".to_string(),
        // "run_agent" mapping for Task is UNVERIFIED — omit until confirmed
        _ => platform_name.to_string(),
    }
}
```

### Corrected OpenCode map_tool_name

```rust
// Based on opencode.ai/docs/tools/ (2026-02-20)
fn map_tool_name(platform_name: &str) -> String {
    match platform_name {
        "bash" => "Bash".to_string(),
        "write" => "Write".to_string(),
        "edit" => "Edit".to_string(),
        "read" => "Read".to_string(),
        "glob" => "Glob".to_string(),
        "grep" => "Grep".to_string(),
        "task" => "Task".to_string(),
        "webfetch" | "fetch" => "WebFetch".to_string(),  // "webfetch" is official name
        _ => platform_name.to_string(),
    }
}
```

---

## Files to Modify

| File | Change | Priority |
|------|--------|----------|
| `rulez/src/adapters/gemini.rs` | Fix compile error (refactor parse_event to use Map-first pattern); correct tool name mappings | P0 — blocks everything |
| `rulez/src/adapters/copilot.rs` | Fix compile error (same refactor) | P0 — blocks everything |
| `rulez/src/adapters/opencode.rs` | Add `"webfetch"` mapping, verify `"task"` mapping | P1 |
| `rulez/tests/gemini_adapter.rs` | Update stale assertions; add canonicalization tests | P1 |
| `rulez/tests/copilot_adapter.rs` | Update stale assertions; add canonicalization tests | P1 |
| `rulez/tests/opencode_payload_tests.rs` | Update stale assertions; add canonicalization tests | P1 |
| `docs/TOOL-MAPPING.md` | New doc — mirrors `docs/EVENT-MAPPING.md` format | P2 |

**No changes needed:**
- `rulez/src/adapters/mod.rs` — pub mod declarations are fine as-is
- `rulez/src/hooks.rs` — matcher logic stays unchanged (exact match on canonical names)
- `rulez/src/models.rs` — no new types needed

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead |
|---------|-------------|-------------|
| Bidirectional tool name lookup | Custom reverse-lookup HashMap | Not needed — this phase is one-way (platform → canonical) only |
| Tool alias config in hooks.yaml | Config-driven alias resolution | Not needed — adapter-level mapping handles this cleanly per Option A |
| Case-insensitive matching in hooks.rs | Expanding matcher logic | Not needed — fix the adapter mapping instead |

---

## Open Questions

1. **Correct Gemini CLI name for shell execution: `run_shell_command` or `execute_code`?**
   - What we know: Official Gemini CLI docs at `geminicli.com` list `run_shell_command` as the tool name for shell execution. The hooks guide also shows `run_shell_command` in examples.
   - What's unclear: Whether the hooks payload uses the same name as the tool API, or whether there's a distinct `execute_code` for code blocks.
   - Recommendation: Use `run_shell_command`. If in doubt, add both as aliases: `"run_shell_command" | "execute_code" => "Bash"`.

2. **Gemini CLI grep/search tool: `search_file_content` or `grep_search`?**
   - What we know: The tools index page lists `search_file_content`; the file-system docs page lists `grep_search`.
   - What's unclear: Which is the actual hook payload field name.
   - Recommendation: Map both: `"search_file_content" | "grep_search" => "Grep"`.

3. **Gemini CLI `glob` vs `list_files` for Glob tool?**
   - What we know: Official docs list `glob` as a tool. `list_directory` also exists but is a directory lister, not a glob matcher.
   - Recommendation: Map `"glob"` → `"Glob"`. Remove the `"list_files"` mapping (no evidence that name exists).

4. **Does Gemini CLI have a subagent/Task tool?**
   - What we know: The current mapping has `"run_agent"` → `"Task"` but this is unverified. No official docs mention `run_agent`.
   - Recommendation: Remove `"run_agent"` from the mapping (let it pass through) until a Gemini user reports it.

5. **OpenCode `webfetch` vs `fetch`?**
   - What we know: Official OpenCode docs list `webfetch` as the tool name.
   - What's unclear: The current `opencode.rs` maps `"fetch"` but the official name is `"webfetch"`.
   - Recommendation: Map `"webfetch" | "fetch"` → `"WebFetch"` to handle both spellings safely.

6. **Copilot tool names — no official source found**
   - What we know: The GitHub Copilot hooks documentation does not enumerate tool names. The Phase 21 prior decisions don't include tool names. The current `copilot.rs` mapping is based on inference.
   - Recommendation: Keep the current Copilot mapping as a best-effort guess. Add a comment noting LOW confidence. Do not block the phase on resolving this.

---

## State of the Art

| Old Approach | Current Approach | Status |
|--------------|------------------|--------|
| Tool names pass through raw (no mapping) | Adapter-level canonicalization via `map_tool_name()` | Code written but broken (compile error) |
| Tests assert platform names | Tests should assert canonical names + check `platform_tool_name` | Not yet updated |
| No TOOL-MAPPING.md | `docs/TOOL-MAPPING.md` mirrors EVENT-MAPPING.md | Not yet created |

---

## Sources

### Primary (HIGH confidence)
- `opencode.ai/docs/tools/` — Confirmed OpenCode tool names (bash, write, edit, read, glob, grep, task, webfetch, list, etc.)
- `geminicli.com/docs/tools/` and `github.com/google-gemini/gemini-cli/blob/main/docs/tools/index.md` — Confirmed Gemini CLI tool names (run_shell_command, write_file, replace, read_file, glob, search_file_content/grep_search, web_fetch)
- `cargo build` output — Confirmed compile errors in gemini.rs and copilot.rs
- Direct code inspection of `rulez/src/adapters/` and `rulez/tests/` — Confirmed stale test assertions

### Secondary (MEDIUM confidence)
- `geminicli.com/docs/hooks/writing-hooks/` — Confirmed `tool_name` field exists in hook payloads; mentions `write_file`, `read_file`, `list_directory`, `run_shell_command` in context
- `github.com/google-gemini/gemini-cli/blob/main/docs/tools/file-system.md` — Lists `glob`, `grep_search`, `replace`, `read_file`, `write_file`, `list_directory`
- `github.com/anomalyco/opencode/issues/12472` — Confirms OpenCode uses lowercase names (bash, edit) vs Claude Code PascalCase (Bash, Edit)

### Tertiary (LOW confidence)
- GitHub Copilot hooks docs (`docs.github.com/en/copilot/reference/hooks-configuration`) — Does not enumerate tool names; inaccessible during research
- Copilot tool name mappings in `copilot.rs` — Based on inference, not official docs

---

## Metadata

**Confidence breakdown:**
- Compile fix (Rust move semantics): HIGH — error is unambiguous, OpenCode pattern is proven working
- Gemini tool names: MEDIUM — verified from official docs but `grep` and `task` tool names have uncertainty
- OpenCode tool names: HIGH — official docs confirm the list clearly
- Copilot tool names: LOW — no official docs found; current mapping is hypothesis
- Test update requirements: HIGH — stale assertions are visible in code and expected behavior is clear
- hooks.rs stays unchanged: HIGH — the plan document's Option A decision is sound

**Research date:** 2026-02-20
**Valid until:** 2026-03-20 (platform docs can change, verify Gemini tool names if > 30 days old)
