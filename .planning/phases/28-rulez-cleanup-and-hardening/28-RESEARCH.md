# Phase 28: RuleZ Cleanup and Hardening - Research

**Researched:** 2026-03-05
**Domain:** Rust policy engine — bug fixes, performance hardening, docs alignment, CLI extension
**Confidence:** HIGH (all findings from direct codebase inspection)

## Summary

Phase 28 addresses 9 accumulated todos across the rulez binary, mastering-hooks skill docs, and rulez-ui log store. All source files referenced in the todos are confirmed to exist at their current paths (old `cch_cli/` paths are stale — code lives in `rulez/src/`). The binary version is 1.8.0.

The two CRITICAL bugs (todos 1 and 2) are confirmed: `command_match` uses bare `Regex::new()` with `if let Ok(...)` which silently discards invalid patterns and falls through without blocking. The `debug` command (`cli/debug.rs`) has its own local `rule_matches_event()` function that does NOT call `execute_validator_script()` or `execute_inline_script()`, so `run:` script actions are never tested via `rulez debug`.

Config caching (todo 5) is partially addressed — there is an LRU regex cache (`REGEX_CACHE`) from Phase 8, but `Config::load()` reads from disk on every `process_event()` call with no mtime check or in-process cache. The config and the `command_match` regex compile paths are separate concerns that partially overlap.

The mastering-hooks docs are confirmed misaligned: they use `hooks:` (actual YAML key is `rules:`), `match:` (actual is `matchers:`), and `action:` (actual is `actions:`). All examples in the skill docs will silently fail validation.

**Primary recommendation:** Fix the two CRITICAL bugs (todos 1 and 2) in a single PR, then address the remaining 7 todos independently — they have no blocking dependencies on each other.

## Standard Stack

### Core (Already in Cargo.toml)
| Library | Version | Purpose | Notes |
|---------|---------|---------|-------|
| `regex` | 1.10 | Pattern matching | Workspace dep |
| `lru` | 0.12 | LRU cache (regex cache) | Already used for `REGEX_CACHE` |
| `tokio` | 1.0 | Async runtime | Already used |
| `serde_json` | 1.0 | JSON serialization | Already used |

### Proposed Additions
| Library | Version | Purpose | When to Add |
|---------|---------|---------|-------------|
| `globset` | 0.4 | Proper glob matching | Todo 4 — replace `contains()` hack |
| `rayon` | 1.10 | Data parallelism | Todo 6 — parallel rule evaluation (evaluate before adding) |
| `self_update` | 0.40 | Binary self-upgrade | Todo 9 — `rulez upgrade` command |

**Note on `rayon` vs `tokio::spawn`:** The `evaluate_rules()` function is `async` and already uses tokio. For parallel rule evaluation, `tokio::task::spawn` with `join_all` from `futures` is the correct approach since the rule executor calls async `execute_validator_script()`. `rayon` is for CPU-bound sync work and would fight the tokio runtime.

**Installation (proposed additions):**
```bash
cargo add globset
cargo add self_update --features archive-tar,archive-zip,compression-flate2,compression-zip-deflate
```

## Architecture Patterns

### Current Project Structure (confirmed)
```
rulez/src/
├── main.rs              # CLI entry, hook stdin processing
├── hooks.rs             # 5297 lines: matchers, eval, scripts, tests
├── config.rs            # 1728 lines: Config::load(), validate()
├── models.rs            # 3150 lines: Rule, Actions, RunAction, Event, etc.
├── logging.rs           # Audit trail
├── cli/
│   ├── debug.rs         # 554 lines: SEPARATE rule_matches_event() — source of todo 2 bug
│   ├── init.rs
│   ├── install.rs
│   ├── validate.rs
│   ├── logs.rs
│   ├── explain.rs
│   ├── gemini_*.rs
│   ├── copilot_*.rs
│   └── opencode_*.rs
```

### Pattern: LRU Regex Cache (already present)
```rust
// Source: rulez/src/hooks.rs lines 33-73
pub static REGEX_CACHE: LazyLock<Mutex<LruCache<String, Regex>>> = LazyLock::new(|| {
    Mutex::new(LruCache::new(NonZeroUsize::new(MAX_CACHED_PATTERNS).unwrap()))
});

fn get_or_compile_regex(pattern: &str, case_insensitive: bool) -> Result<Regex> {
    let cache_key = format!("{}:{}", if case_insensitive { "ci" } else { "cs" }, pattern);
    // ... LRU lookup then compile
}
```

**Problem:** `get_or_compile_regex()` is used by `matches_prompt()` but NOT by `command_match` evaluation at lines 687, 787, 1048, 1331. Those 4 call sites all use bare `Regex::new()`.

### Pattern: Config Loading (per-event, no cache)
```rust
// Source: rulez/src/hooks.rs line 453
let config = Config::load(event.cwd.as_ref().map(|p| Path::new(p.as_str())))?;
```
`Config::load()` reads the YAML from disk on every hook event. There is no mtime check, no in-process `Arc<Config>` cache, no file watcher.

### Pattern: Rule Action Execution (hooks.rs vs debug.rs divergence)
- **`hooks.rs`** `execute_rule_actions()` (line 974): handles `validate_expr`, `inline_script`, `block`, `block_if_match`, `inject_inline`, `inject_command`, `inject`, and `run` (via `actions.script_path()` → `execute_validator_script()`)
- **`debug.rs`** `rule_matches_event()` (line 255): only checks matchers (`operations`, `tools`, `command_match`, `extensions`, `directories`) — never executes any action, never calls `execute_validator_script()`

This means `rulez debug` tests matcher logic but does NOT test the run script action, making it useless for validating rules with `run:` scripts.

### Pattern: build_eval_context (missing tool_input fields)
```rust
// Source: rulez/src/hooks.rs lines 556-584
fn build_eval_context(event: &Event) -> HashMapContext<DefaultNumericTypes> {
    // Adds: env_* vars, tool_name, event_type, prompt
    // MISSING: tool_input fields (command, filePath, content, etc.)
}
```
The `enabled_when` expression system exposes `tool_name`, `event_type`, `prompt`, and all env vars. It does NOT expose individual `tool_input` fields like `command`, `filePath`, `content`. Users wanting to write `enabled_when: "tool_input_command =~ 'git'"` cannot do so.

Note: `build_eval_context_with_custom_functions()` (line 275) adds `get_field()` and `has_field()` custom functions for `validate_expr`/`validate_context` — but these are NOT used by `is_rule_enabled()` which calls the simpler `build_eval_context()`.

## Todo-by-Todo Analysis

### Todo 1: Fix invalid regex silently matching all commands (CRITICAL)
**Confidence:** HIGH

**Bug location:**
- `hooks.rs` line 687: `matches_rule()` — `command_match` in non-debug path
- `hooks.rs` line 787: `matches_rule_with_debug()` — `command_match` in debug path
- `hooks.rs` line 1048: `execute_rule_actions()` — `block_if_match` pattern
- `hooks.rs` line 1331: (another block_if_match in different context branch)
- `debug.rs` line 284: `rule_matches_event()` — debug CLI path

**The bug:** All 5 call sites use `if let Ok(regex) = Regex::new(pattern)`. When the pattern is invalid, the `Err` variant is silently discarded and execution falls through. In `matches_rule()`, falling through the `if let Ok` block means the `command_match` guard doesn't reject the event — so the rule matches all commands.

**Fix:** Replace with `get_or_compile_regex()` (already exists) and use fail-closed error handling:
```rust
// Instead of:
if let Ok(regex) = Regex::new(pattern) {
    if !regex.is_match(command) { return false; }
}
// Use:
match get_or_compile_regex(pattern, false) {
    Ok(regex) => { if !regex.is_match(command) { return false; } }
    Err(_) => { return false; } // Fail-closed: invalid regex blocks
}
```

**Stale cache note (from todo description):** `Config::load()` re-reads disk on every event. If YAML changes between events, the new config is picked up automatically — there is no stale cache on the config side. However, the `REGEX_CACHE` holds compiled patterns. If a user fixes an invalid regex, the old invalid pattern may not be in cache (it failed to compile), so this is not the stale cache problem. The stale cache problem is more likely about config loading being expensive, which overlaps with Todo 5.

**Complexity:** SMALL (5 call sites to fix, pattern is straightforward)

### Todo 2: rulez debug does not exercise run action scripts (CRITICAL)
**Confidence:** HIGH

**Bug location:** `rulez/src/cli/debug.rs`

**Root cause:** `debug.rs` has its own local `rule_matches_event()` function (lines 255-337) that only checks matchers. When the run mode proceeds to `run_json_mode()` or the human-readable path, it calls `hooks::process_event()` which DOES execute run scripts. But the `rule_matches_event()` local function used for per-rule tracing never executes scripts.

More specifically: in `run()` (line 91) and `run_json_mode()` (line 192), the actual `hooks::process_event()` IS called and DOES execute run scripts. The bug is that `debug.rs` builds its per-rule trace using `rule_matches_event()` which doesn't show script execution results. The issue description says "doesn't pipe event JSON to run scripts or handle their output" which is about the trace output, not about whether execution happens.

**Verification needed:** Test manually with a config containing a `run:` script. If `hooks::process_event()` is called, the script WILL execute — but the per-rule trace won't show it.

**Fix approach:** The `run_json_mode()` function needs to also call `execute_rule_actions_with_mode()` (or similar) for matched rules to capture script output in the JSON trace.

**Complexity:** MEDIUM (requires refactoring debug.rs to show run script results in trace)

### Todo 3: Expose tool_input fields in enabled_when eval context (HIGH VALUE)
**Confidence:** HIGH

**Bug location:** `hooks.rs` line 556-583, `build_eval_context()` function

**Current context variables:** `env_*` (all env vars), `tool_name`, `event_type`, `prompt`

**Missing:** `tool_input` fields. The `Event` struct has `tool_input: Option<serde_json::Value>`.

**Fix:** Iterate `tool_input` fields, add them with `tool_input_` prefix (or flat if Value is an object):
```rust
// Add after line 581 (after prompt block):
if let Some(ref tool_input) = event.tool_input {
    if let Some(obj) = tool_input.as_object() {
        for (key, val) in obj {
            let var_name = format!("tool_input_{}", key);
            // Convert JSON value to evalexpr Value
            match val {
                serde_json::Value::String(s) => ctx.set_value(var_name, Value::String(s.clone())).ok(),
                serde_json::Value::Bool(b) => ctx.set_value(var_name, Value::Boolean(*b)).ok(),
                serde_json::Value::Number(n) => {
                    if let Some(f) = n.as_f64() {
                        ctx.set_value(var_name, Value::Float(f)).ok()
                    } else { None }
                }
                _ => None,
            };
        }
    }
}
```

**Complexity:** SMALL (~15 lines)

**Note:** The `build_eval_context_with_custom_functions()` function already handles `tool_input` via `get_field()` / `has_field()` custom functions. These are used by `validate_expr` but NOT by `enabled_when`. After this fix, users can write: `enabled_when: "tool_input_command =~ 'git push'"`.

### Todo 4: Replace Naive Matchers with globset
**Confidence:** HIGH

**Bug location:** `hooks.rs` lines 723-725, 835-837 (directory matching); `debug.rs` lines 326-328

**Current code:**
```rust
// "Simple glob matching - in production, use a proper glob library"  (comment from line 723)
path_str.contains(dir.trim_end_matches("/**"))
    || path_str.contains(dir.trim_end_matches("/*"))
```

**Problem:** This `contains()` check is wrong. `src/` would match `/other/src/foo.rs` (false positive). It strips trailing `/**` and `/*` but doesn't handle `?`, `[a-z]`, `**` as path separator, or negation patterns.

**Fix:** Add `globset` crate. Replace the `contains()` block with `GlobSet` matching:
```rust
use globset::{Glob, GlobSetBuilder};
// Build globset from directories list, then:
let glob_set = GlobSetBuilder::new()
    .add(Glob::new(dir)?)
    .build()?;
glob_set.is_match(file_path)
```

**Complexity:** MEDIUM (add dependency, replace matching logic in 2 locations: `hooks.rs` and `debug.rs`)

### Todo 5: Implement Regex and Config Caching
**Confidence:** HIGH (partially done for regex, not done for config)

**Regex caching:** The `REGEX_CACHE` + `get_or_compile_regex()` already exists (Phase 8). The fix here is to USE it at all call sites (same fix as Todo 1). This means Todos 1 and 5 are the same work for regex caching.

**Config caching:** `Config::load()` hits disk on every hook event. Fix requires an in-process cache:
- Option A: `LazyLock<Mutex<Option<(Config, SystemTime)>>>` — cache with mtime check
- Option B: Pass `Config` through `process_event()` instead of re-loading (breaks public API)
- Option C: Use `notify` crate for file watching (adds complexity)

**Recommended:** Option A (mtime-based cache). Check `fs::metadata(path).modified()` before re-reading.

**Dependency:** This overlaps Todo 1 on the regex side — do them together.

**Complexity:** SMALL for regex (same fix as Todo 1), MEDIUM for config mtime cache

### Todo 6: Parallel Rule Evaluation
**Confidence:** MEDIUM

**Current:** `evaluate_rules()` is a sequential `for rule in config.enabled_rules()` loop.

**Concern:** Rules currently short-circuit — a blocking rule stops evaluation. If parallelized, all rules run and results must be merged. This changes semantics.

**Correct approach:** Use `tokio::task::JoinSet` or `futures::future::join_all` with `tokio::task::spawn` since `execute_rule_actions_with_mode()` is async. `rayon` would be wrong here (async context).

**Risk:** The current sequential design has important semantics: first matching rule with a block wins. Parallelizing requires collecting all results and applying merge logic post-hoc. The `merge_responses_with_mode()` function already exists for this.

**Recommendation:** Only parallelize the matching phase (all rules evaluated in parallel), then process responses in priority order. Skip if rules count is typically small (<50) — overhead may exceed benefit.

**Complexity:** LARGE (semantics change, requires careful testing)

### Todo 7: Offload Log Filtering to Web Worker or Rust
**Confidence:** HIGH

**Current implementation:** `rulez-ui/src/stores/logStore.ts` — `applyClientFilters()` function runs synchronously on the React main thread. It iterates all `entries` array checking 8 string fields per entry.

**Performance concern:** At 100K entries this will block the UI thread for ~100-500ms per filter change.

**Fix options:**
1. Web Worker: Move `applyClientFilters()` to a worker, post messages for filter changes
2. Rust/Tauri: Move filtering to the Tauri backend command, pass filter params to `readLogs()`
3. Debounce: Add 200ms debounce to text filter input to reduce frequency (cheapest fix)

**Recommended:** Start with debounce (free), then move heavy filtering to Tauri command if needed. The current `readLogs()` already accepts `since`/`until` params — extend with `text` and `severity` params.

**Complexity:** SMALL (debounce), MEDIUM (Tauri command extension)

### Todo 8: Fix mastering-hooks skill schema mismatches
**Confidence:** HIGH (confirmed by direct file inspection)

**File:** `mastering-hooks/references/hooks-yaml-schema.md`

**Confirmed mismatches:**

| Doc uses | Actual YAML key | Location |
|----------|----------------|----------|
| `hooks:` (top-level list) | `rules:` | Schema doc line 16, 22 |
| `match:` | `matchers:` | Schema doc line 29, 101+ |
| `action:` | `actions:` | Schema doc line 29, 196+ |
| `event:` (per-rule) | `matchers.operations:` (list) | Schema doc line 25 |
| `enabled:` (per-rule) | `metadata.enabled:` (nested) | Schema doc line 27 |
| `priority:` (per-rule, lower=higher) | `priority:` (per-rule, higher=higher) | Schema doc line 28 |
| `version: "1"` | `version: "1.0"` | Schema doc line 15, config.rs validate() |

**File:** `mastering-hooks/references/rule-patterns.md`

Uses `match:` and `action:` throughout all examples (confirmed lines 28-235). All examples in this file use the wrong field names.

**Actual YAML structure (from models.rs inspection):**
```yaml
version: "1.0"     # NOT "1"
rules:             # NOT "hooks:"
  - name: string
    matchers:      # NOT "match:"
      tools: [...]
      command_match: "..."
    actions:       # NOT "action:"
      block: true
      inject: "..."
      run: "path"
```

**Complexity:** SMALL (text replacements across 2 files)

### Todo 9: Auto-check and upgrade RuleZ binary
**Confidence:** MEDIUM

**Current state:** No `upgrade` subcommand exists in `main.rs`. `main.rs` has `Init`, `Install`, `Uninstall`, `Debug`, `Repl`, `Validate`, `Logs`, `Explain`, `Gemini`, `Copilot`, `OpenCode` — no `Upgrade`.

**Implementation approach:**
- Add `rulez upgrade` subcommand to `main.rs`
- Create `rulez/src/cli/upgrade.rs`
- Use `self_update` crate (used by many Rust CLIs) — handles GitHub releases API, binary replacement, platform detection
- Or implement manually: `reqwest` to GitHub releases API, compare semver, download, replace binary

**Self_update crate approach:**
```rust
// self_update handles GitHub releases, semver comparison, binary download
let status = self_update::backends::github::Update::configure()
    .repo_owner("owner")
    .repo_name("rulez")
    .bin_name("rulez")
    .current_version(env!("CARGO_PKG_VERSION"))
    .build()?
    .update()?;
```

**Alternatives:**
- `cargo-update` pattern: just print the latest version and instructions
- Shell script approach: `rulez upgrade` generates a curl command for the user

**Complexity:** MEDIUM (new subcommand + dependency on `self_update` or `reqwest`)

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Glob pattern matching | `contains()` strip hacks | `globset` crate | Handles `**`, `?`, `[ranges]`, proper path semantics |
| Binary self-update | HTTP download + replace logic | `self_update` crate | Handles atomic replace, rollback, GitHub releases API, semver |
| Parallel async tasks | Manual `Vec<JoinHandle>` | `futures::future::join_all` or `tokio::task::JoinSet` | Correct cancellation, error propagation |

**Key insight:** The `contains()` directory matching is already commented as "in production, use a proper glob library" — the codebase already knows this is temporary. Do not extend or improve the current `contains()` approach; replace it.

## Common Pitfalls

### Pitfall 1: Fixing the Regex Bug Introduces a Config Validation Gap
**What goes wrong:** After fixing `command_match` to fail-closed on invalid regex, users who have invalid regex in their config will have rules that never match. This is correct behavior but unexpected.
**How to avoid:** Add validation of `command_match` regex in `Config::validate()` (currently only validates `prompt_match` regex, not `command_match`).
**Warning signs:** Tests pass but config with invalid `command_match` silently blocks nothing.

### Pitfall 2: Debug CLI's Duplicate rule_matches_event()
**What goes wrong:** Fixing `command_match` regex handling in `hooks.rs` `matches_rule()` without also fixing `debug.rs` `rule_matches_event()` leaves the debug CLI with the old broken behavior.
**How to avoid:** Both locations must be fixed together. The 5 call sites are: hooks.rs lines 687, 787, 1048, 1331 AND debug.rs line 284.
**Warning signs:** `rulez debug` reports a rule matches when `hooks::process_event()` would not (or vice versa).

### Pitfall 3: Schema Doc `version: "1"` vs `version: "1.0"`
**What goes wrong:** Docs say `version: "1"` but `config.rs` validate() uses regex `r"^\d+\.\d+$"` which requires `x.y` format. A user following the docs will get a validation error.
**How to avoid:** Fix docs to show `version: "1.0"`.

### Pitfall 4: Parallel Rule Evaluation Breaks Short-Circuit Semantics
**What goes wrong:** Current sequential evaluation stops at first block. Parallelizing all rules means a `block` rule and an `inject` rule could both match; merge logic must still produce `block`.
**How to avoid:** The existing `merge_responses_with_mode()` function handles this — but it must be applied after collecting ALL parallel results, sorted by priority.

### Pitfall 5: globset Compilation is Expensive per Event
**What goes wrong:** Building a `GlobSet` for each rule on each event would be slower than the current `contains()` hack.
**How to avoid:** Cache compiled `GlobSet` objects alongside the `REGEX_CACHE`, or build them at config load time.

## Code Examples

### Current Broken Pattern (to fix — Todo 1)
```rust
// Source: rulez/src/hooks.rs line 687
// BUG: if Regex::new fails, falls through with no rejection = silent match-all
if let Ok(regex) = Regex::new(pattern) {
    if !regex.is_match(command) {
        return false;
    }
}
// ^^^ If pattern is "[invalid(", regex compile fails, this whole block is skipped,
// and the function continues past command_match check — rule matches any command.
```

### Fixed Pattern (fail-closed)
```rust
// Use existing get_or_compile_regex() with fail-closed error handling
match get_or_compile_regex(pattern, false) {
    Ok(regex) => {
        if !regex.is_match(command) {
            return false;
        }
    }
    Err(_) => {
        tracing::warn!("Invalid command_match regex '{}' in rule '{}' - failing closed", pattern, rule_name);
        return false; // Fail-closed: invalid regex = no match
    }
}
```

### build_eval_context() Fix (Todo 3)
```rust
// Source: rulez/src/hooks.rs after line 581 (end of prompt block, before closing brace)
// Expose tool_input fields with tool_input_ prefix
if let Some(ref tool_input) = event.tool_input {
    if let Some(obj) = tool_input.as_object() {
        for (key, val) in obj {
            let var_name = format!("tool_input_{}", key);
            match val {
                serde_json::Value::String(s) => {
                    ctx.set_value(var_name, Value::String(s.clone())).ok();
                }
                serde_json::Value::Bool(b) => {
                    ctx.set_value(var_name, Value::Boolean(*b)).ok();
                }
                serde_json::Value::Number(n) => {
                    if let Some(f) = n.as_f64() {
                        ctx.set_value(var_name, Value::Float(f)).ok();
                    }
                }
                _ => {} // Arrays, objects, null — skip (not supported by evalexpr)
            }
        }
    }
}
```

### Config Validation Addition (Todo 1 + 5)
```rust
// Add to Config::validate() in config.rs after prompt_match validation:
// Validate command_match regex compiles
if let Some(ref pattern) = rule.matchers.command_match {
    if let Err(e) = regex::Regex::new(pattern) {
        return Err(anyhow::anyhow!(
            "Invalid command_match regex '{}' in rule '{}': {}",
            pattern, rule.name, e
        ));
    }
}
```

## State of the Art

| Old Approach | Current Approach | Notes |
|--------------|------------------|-------|
| `cch_cli/src/hooks.rs` | `rulez/src/hooks.rs` | Renamed in Phase 11; all old paths stale |
| No regex cache | LRU cache via `get_or_compile_regex()` | Added Phase 8; NOT used at all call sites |
| `contains()` for glob matching | `contains()` (still) | Planned for replacement; comment says so |
| Config read on disk every call | Config read on disk every call | No change since original; todo 5 to address |

**Deprecated/outdated in skill docs:**
- `hooks:` field name: replaced by `rules:` in actual YAML
- `match:` field name: replaced by `matchers:` in actual YAML
- `action:` field name: replaced by `actions:` in actual YAML
- `event:` per-rule field: not a field; use `matchers.operations: [EventName]`
- `version: "1"`: must be `version: "1.0"` (enforced by validate())

## Open Questions

1. **What is the "stale config cache" in Todo 1 specifically?**
   - What we know: `Config::load()` re-reads disk on every event (no cache)
   - What's unclear: The todo description links invalid regex and stale cache — these may be two separate bugs or one compound symptom
   - Recommendation: Treat as two separate fixes: (a) fail-closed regex, (b) mtime-based config cache

2. **Should `rulez debug` execute run scripts or only show matcher results?**
   - What we know: Debug currently doesn't show run script output in the trace
   - What's unclear: Is the intent to actually run the scripts (which could have side effects) or just report what would happen?
   - Recommendation: Execute run scripts in debug mode but prefix output with `[DEBUG]` to distinguish

3. **What is the GitHub repo name/owner for the `rulez upgrade` command?**
   - What we know: The SKILL.md mentions `api_version: "1.8.0"` but no public repo URL
   - What's unclear: Is `rulez upgrade` meant to upgrade from GitHub releases or a different mechanism?
   - Recommendation: Clarify with project owner; implement a `--check` flag first that just prints available version

4. **Should `globset` compilation be cached per-config-load or per-rule?**
   - What we know: Building GlobSet is non-trivial; doing it per event is wasteful
   - What's unclear: Whether to cache in-process alongside REGEX_CACHE or at Config load time
   - Recommendation: Build GlobSet at Config::from_file() time, store in Config struct alongside rules

## Dependencies Between Todos

| Todo | Depends On | Notes |
|------|-----------|-------|
| 1 (regex fail-closed) | — | Independent |
| 2 (debug run scripts) | — | Independent |
| 3 (tool_input in enabled_when) | — | Independent |
| 4 (globset) | — | Independent, but see pitfall 5 (cache GlobSet with config) |
| 5 (caching) | 1 (regex caching IS the same fix) | The regex cache fix in Todo 1 satisfies the regex half of Todo 5 |
| 6 (parallel eval) | — | Independent; evaluate complexity before committing |
| 7 (UI log filter) | — | Separate codebase (rulez-ui) |
| 8 (docs fix) | — | Independent |
| 9 (upgrade command) | — | Independent |

**Recommended implementation order:**
1. Todo 1 + 5 (regex) together — same 5 call sites
2. Todo 8 (docs) — fast, no code risk
3. Todo 3 (tool_input context) — small, high value
4. Todo 2 (debug run scripts) — medium complexity
5. Todo 4 (globset) — add dependency + refactor matching
6. Todo 9 (upgrade command) — new subcommand
7. Todo 7 (UI filtering) — separate codebase
8. Todo 6 (parallel eval) — largest, most risk, evaluate necessity

## Sources

### Primary (HIGH confidence)
- Direct inspection: `rulez/src/hooks.rs` (5297 lines) — regex patterns, caching, build_eval_context, matching
- Direct inspection: `rulez/src/config.rs` (1728 lines) — Config::load(), Config::validate()
- Direct inspection: `rulez/src/cli/debug.rs` (554 lines) — debug command, local rule_matches_event()
- Direct inspection: `rulez/src/main.rs` — all subcommands, no upgrade command present
- Direct inspection: `rulez/Cargo.toml` + workspace `Cargo.toml` — current dependencies
- Direct inspection: `mastering-hooks/references/hooks-yaml-schema.md` — confirmed field name mismatches
- Direct inspection: `mastering-hooks/references/rule-patterns.md` — confirmed field name mismatches
- Direct inspection: `rulez-ui/src/stores/logStore.ts` — applyClientFilters() on main thread

### Secondary (MEDIUM confidence)
- `globset` crate (docs.rs) — standard Rust glob library, used by ripgrep, cargo
- `self_update` crate — standard approach for Rust binary self-upgrade from GitHub releases
- `tokio::task::JoinSet` — correct async parallel pattern for tokio runtime

## Metadata

**Confidence breakdown:**
- Bug locations (todos 1, 2, 3, 4): HIGH — exact line numbers confirmed by code inspection
- Config caching gap (todo 5): HIGH — Config::load() code is clear
- Schema doc mismatches (todo 8): HIGH — both files inspected directly
- logStore.ts main-thread filtering (todo 7): HIGH — small file, clear issue
- Parallel evaluation approach (todo 6): MEDIUM — needs profiling to validate necessity
- Upgrade command approach (todo 9): MEDIUM — crate exists, repo URL unknown

**Research date:** 2026-03-05
**Valid until:** 2026-04-05 (stable codebase, no fast-moving dependencies)
