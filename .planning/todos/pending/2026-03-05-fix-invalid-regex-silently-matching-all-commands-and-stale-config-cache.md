---
created: 2026-03-05T21:28:15.168Z
title: Fix invalid regex silently matching all commands and stale config cache
area: tooling
github_issue: https://github.com/SpillwaveSolutions/agent_rulez/issues/101
files:
  - rulez/src/hooks.rs
  - rulez/src/config.rs
---

## Problem

Two related bugs in the rule evaluation engine:

### 1. Invalid regex silently matches everything
In `hooks.rs`, when `command_match` contains a pattern the `regex` crate can't compile (e.g., negative lookahead `(?!...)`), `Regex::new()` returns `Err`. The `if let Ok(regex)` silently skips the check, and the rule falls through as a **match** — meaning an invalid regex in a `block` rule blocks ALL commands.

### 2. Stale config cache not invalidated
After removing a rule from `hooks.yaml`, the binary continues to block with the removed rule's name. The regex/config cache is not invalidated when the YAML file changes between invocations. A CRC or timestamp check on the YAML file is needed to invalidate the cache.

Comment from @RichardHightower: "There is a cache for grep regex... We should probably get a CRC of the YAML file and invalidate cache if the file's CRC changes. Or we can use timestamp."

## Solution

### Fix 1: Treat regex compile failure as non-match
```rust
match Regex::new(pattern) {
    Ok(regex) => {
        if !regex.is_match(command) {
            return false;
        }
    }
    Err(_) => return false,  // invalid regex = safe non-match
}
```

Also add `command_match` to `Config::validate()` regex compilation checks so bad patterns are caught at startup with a clear error message.

### Fix 2: Cache invalidation
Add file timestamp or CRC check on `hooks.yaml` before using cached config/regex. Invalidate cache when file has changed since last read. Consider using `std::fs::metadata().modified()` for simplicity.

### Fix 3: Honor `metadata.enabled: false`
Ensure `enabled: false` unconditionally skips the rule before any matcher evaluation.
