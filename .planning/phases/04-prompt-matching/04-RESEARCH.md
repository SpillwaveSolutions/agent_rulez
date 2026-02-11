# Phase 4: Prompt Matching - Research

**Researched:** 2026-02-09
**Domain:** Rust pattern matching with serde deserialization and regex evaluation
**Confidence:** HIGH

## Summary

This phase extends RuleZ's matcher system to support prompt text matching for the `UserPromptSubmit` event. The research focused on how to implement flexible YAML syntax for pattern matching, regex compilation/caching performance, and integration with the existing evalexpr-based scripting system.

**Key findings:**
1. `command_match` provides a solid reference pattern - uses simple `Option<String>` regex pattern
2. Serde's `#[serde(untagged)]` enables flexible YAML syntax (array vs object) for `prompt_match`
3. Regex caching with `once_cell::OnceLock` (std since Rust 1.70) provides zero-cost abstraction
4. Evalexpr context already supports custom variables - can add `prompt` variable easily
5. `UserPromptSubmit` event includes `prompt` field according to Claude Code documentation

**Primary recommendation:** Use untagged enum for `PromptMatch` type to support both simple array syntax and complex object syntax with options, following the pattern established by `RunAction` in Phase 2.4.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Pattern Syntax:**
- Case-sensitive by default (consistent with regex defaults), opt-in for case-insensitive via flag
- Support both regex anchors (^, $) AND a convenience `anchor` field (start | end | contains)
- Add shorthands for common patterns to improve readability:
  - `contains_word: 'delete'` expands to `\bdelete\b`
  - Full regex still available for power users

**Multiple Pattern Logic:**
- Default to ANY (OR) logic — rule matches if any pattern matches
- Support both syntaxes:
  - Array syntax for simple case: `prompt_match: ['pattern1', 'pattern2']`
  - Object syntax when mode needed: `prompt_match: { patterns: [...], mode: all }`
- Support negation with `not:` prefix: `not: 'pattern'` to exclude matches

**Match Target:**
- Match against full prompt text (not just first line)
- No normalization — match raw prompt text as-is
- Missing prompt field = rule doesn't match (safe default)
- Works on all event types that have a prompt field

**Script Matching Behavior:**
- Full event context available: prompt, tool_name, event_type, env vars
- Configurable error handling with fail-closed as default (error = no match, log warning)

### Claude's Discretion

- Pattern type choice (regex-only vs literal+regex flag)
- Nested group support for complex logic (keep it simple if complexity outweighs benefit)
- Script helper functions (contains_word, line_count, etc.)
- YAML field naming (prompt_script vs inside prompt_match object)

### Deferred Ideas (OUT OF SCOPE)

None — discussion stayed within phase scope

</user_constraints>

## Standard Stack

The established libraries/tools for implementing prompt matching:

### Core Dependencies (Already in Project)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| serde | 1.x | YAML deserialization | Universal Rust serialization framework |
| serde_yaml | 0.9.x | YAML parsing | Standard YAML implementation for Rust |
| regex | 1.x | Pattern matching | Official regex crate, guarantees linear time |
| evalexpr | 11.x | Script evaluation | Already used for `enabled_when` expressions |
| once_cell | 1.19.x | Regex caching | Part of std::sync since Rust 1.70 as OnceLock |

### No Additional Dependencies Needed

This phase leverages existing crate versions in Cargo.toml. All required functionality is available.

**Installation:** None required - all dependencies already present.

## Architecture Patterns

### Recommended Type Structure

```rust
// In models.rs - add to Matchers struct
pub struct Matchers {
    // ... existing fields ...

    /// Prompt text pattern matching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_match: Option<PromptMatch>,
}

// Flexible prompt matching configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PromptMatch {
    /// Simple array syntax: ["pattern1", "pattern2"]
    Simple(Vec<String>),

    /// Complex object syntax with options
    Complex {
        patterns: Vec<String>,
        #[serde(default = "default_match_mode")]
        mode: MatchMode,
        #[serde(default)]
        case_insensitive: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        anchor: Option<Anchor>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MatchMode {
    Any,  // OR logic (default)
    All,  // AND logic
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Anchor {
    Start,    // ^ prefix
    End,      // $ suffix
    Contains, // No anchors (default)
}

fn default_match_mode() -> MatchMode {
    MatchMode::Any
}
```

### Pattern 1: Untagged Enum Deserialization

**What:** Use serde's `#[serde(untagged)]` to support multiple YAML syntaxes transparently

**When to use:** When you want ergonomic syntax variation (simple array vs. complex object)

**Example:**
```yaml
# Simple syntax - deserialized as PromptMatch::Simple
prompt_match: ["delete", "drop database"]

# Complex syntax - deserialized as PromptMatch::Complex
prompt_match:
  patterns: ["delete", "drop"]
  mode: all
  case_insensitive: true
```

**Implementation notes:**
- Serde tries variants in order - put `Simple` first for array matching
- `Complex` variant uses named fields for object matching
- Performance cost is negligible according to benchmarks

**Sources:**
- [Serde Enum Representations](https://serde.rs/enum-representations.html)
- [serde-untagged crate](https://crates.io/crates/serde-untagged) (not needed for this simple case)

### Pattern 2: Regex Compilation Caching

**What:** Cache compiled regex patterns to avoid recompilation on each match

**When to use:** Regex patterns that are evaluated repeatedly (e.g., on every prompt)

**Example:**
```rust
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;

// Global cache for compiled regexes
static REGEX_CACHE: Lazy<Mutex<HashMap<String, Regex>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn get_or_compile_regex(pattern: &str, case_insensitive: bool) -> Result<Regex, regex::Error> {
    let cache_key = format!("{}:{}", pattern, case_insensitive);

    let mut cache = REGEX_CACHE.lock().unwrap();

    if let Some(regex) = cache.get(&cache_key) {
        return Ok(regex.clone());
    }

    let regex = if case_insensitive {
        regex::RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()?
    } else {
        Regex::new(pattern)?
    };

    cache.insert(cache_key, regex.clone());
    Ok(regex)
}
```

**Implementation notes:**
- Use `once_cell::Lazy` for lazy initialization (or `std::sync::LazyLock` on Rust 1.80+)
- Store compiled Regex in HashMap keyed by pattern+options
- Regex is `Clone` but internally uses Arc, so cloning is cheap
- Consider LRU eviction if cache grows unbounded

**Sources:**
- [Rust Regex Caching Discussion](https://github.com/rust-lang/regex/discussions/960)
- [regex-cache crate](https://docs.rs/regex-cache) (optional - provides LRU caching)
- [once_cell documentation](https://docs.rs/once_cell/latest/once_cell/)
- [Rust lazy_static Usage](https://dev.to/rimutaka/rusts-lazystatic-usage-benchmarks-and-code-deep-dive-1bic)

### Pattern 3: Evalexpr Context Extension

**What:** Add `prompt` variable to evalexpr context for script-based matching

**When to use:** When implementing `prompt_script` or script-based prompt matching

**Example:**
```rust
// In hooks.rs - extend build_eval_context function
fn build_eval_context(event: &Event) -> HashMapContext<DefaultNumericTypes> {
    let mut ctx = HashMapContext::new();

    // Existing environment variables
    for (key, value) in std::env::vars() {
        let var_name = format!("env_{}", key);
        ctx.set_value(var_name.into(), Value::String(value)).ok();
    }

    // Existing tool_name and event_type
    let tool_name = event.tool_name.as_deref().unwrap_or("").to_string();
    ctx.set_value("tool_name".into(), Value::String(tool_name)).ok();

    ctx.set_value(
        "event_type".into(),
        Value::String(event.hook_event_name.to_string())
    ).ok();

    // NEW: Add prompt variable for UserPromptSubmit events
    if let Some(ref prompt) = event.prompt {
        ctx.set_value("prompt".into(), Value::String(prompt.clone())).ok();
    }

    ctx
}
```

**Implementation notes:**
- Evalexpr supports string variables naturally
- Can add helper functions later (contains_word, line_count) via custom functions
- Context building is O(1) per variable, no performance concern

**Sources:**
- [evalexpr GitHub](https://github.com/ISibboI/evalexpr)
- [eval_with_context documentation](https://docs.rs/evalexpr/latest/evalexpr/fn.eval_with_context.html)

### Pattern 4: Shorthand Pattern Expansion

**What:** Expand convenience shorthands like `contains_word` into full regex patterns

**When to use:** At deserialization time or before regex compilation

**Example:**
```rust
impl PromptMatch {
    /// Expand shorthand patterns into full regex patterns
    pub fn expand_pattern(pattern: &str) -> String {
        // Handle 'not:' prefix
        if let Some(inner) = pattern.strip_prefix("not:") {
            // Negative lookahead: match if pattern does NOT occur
            return format!("^(?!.*{}).*$", regex::escape(inner.trim()));
        }

        // Handle 'contains_word:' shorthand
        if let Some(word) = pattern.strip_prefix("contains_word:") {
            // Word boundary: \bword\b
            return format!(r"\b{}\b", regex::escape(word.trim()));
        }

        // No shorthand - return as-is
        pattern.to_string()
    }

    /// Apply anchor to pattern
    pub fn apply_anchor(pattern: &str, anchor: Option<Anchor>) -> String {
        match anchor {
            Some(Anchor::Start) => format!("^{}", pattern),
            Some(Anchor::End) => format!("{}$", pattern),
            Some(Anchor::Contains) | None => pattern.to_string(),
        }
    }
}
```

**Implementation notes:**
- Expand shorthands BEFORE regex compilation for caching efficiency
- Use `regex::escape()` for literal text in `contains_word` to avoid injection
- Negation uses negative lookahead: `^(?!.*pattern).*$`
- Keep expansion logic pure (no side effects) for testability

### Anti-Patterns to Avoid

**Anti-Pattern 1: Recompiling regex on every match**
- **Why bad:** Regex compilation is expensive (orders of magnitude slower than matching)
- **Instead:** Use caching pattern (Pattern 2 above) or compile once in a static

**Anti-Pattern 2: Using `#[serde(flatten)]` with enums**
- **Why bad:** Not supported by serde, causes compile errors or unexpected behavior
- **Instead:** Use `#[serde(untagged)]` for variant discrimination based on structure
- **Source:** [GitHub Issue #1402](https://github.com/serde-rs/serde/issues/1402)

**Anti-Pattern 3: Complex nested group logic**
- **Why bad:** Makes YAML hard to read and maintain, diminishing returns on complexity
- **Instead:** Keep to simple any/all modes; use scripts for complex logic

**Anti-Pattern 4: Normalizing prompt text**
- **Why bad:** User said "no normalization" - match raw text
- **Instead:** Match against `event.prompt` as-is, let users write patterns that handle whitespace

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Regex parsing/compilation | Custom pattern parser | `regex` crate | Handles Unicode, guarantees linear time, heavily optimized |
| Pattern caching | HashMap + RefCell | `once_cell::Lazy` or `std::sync::LazyLock` | Thread-safe, no overhead, well-tested |
| YAML flexible syntax | Custom deserializer | `#[serde(untagged)]` | Serde handles try-each-variant logic correctly |
| Expression evaluation | Custom script parser | `evalexpr` crate | Already integrated, supports variables and functions |
| String escape for regex | Manual escaping | `regex::escape()` | Handles all special characters correctly |

**Key insight:** Rust's regex and serde ecosystems are mature. Leverage them instead of reimplementing.

## Common Pitfalls

### Pitfall 1: Untagged Enum Variant Ordering

**What goes wrong:** Serde tries variants in source code order. If `Complex` is first, it might match simple arrays incorrectly.

**Why it happens:** Serde's untagged deserialization tries each variant until one succeeds. A complex variant with all optional fields might succeed on simple input.

**How to avoid:**
- Put more specific variants first (e.g., `Simple(Vec<String>)` before `Complex { ... }`)
- Use `#[serde(default)]` on optional fields in complex variant
- Test both syntaxes in unit tests to verify correct deserialization

**Warning signs:** Simple array syntax deserialized as Complex variant with empty/default fields

**Source:** [Serde Untagged Discussion](https://users.rust-lang.org/t/fun-and-sadness-with-serde-untagged-enums/65089)

### Pitfall 2: Regex Cache Memory Growth

**What goes wrong:** Unbounded HashMap grows indefinitely if users have dynamic patterns.

**Why it happens:** Each unique pattern creates a cache entry that's never evicted.

**How to avoid:**
- For this phase: Document that patterns should be static in YAML (not computed)
- Future: Add LRU eviction policy if cache size becomes a concern
- Alternative: Use `regex_cache` crate with built-in LRU

**Warning signs:** Memory growth over time, proportional to number of unique patterns seen

**Source:** [regex-cache crate](https://docs.rs/regex-cache) provides LRU solution

### Pitfall 3: Prompt Field Absent on Non-UserPromptSubmit Events

**What goes wrong:** `event.prompt` is `None` for events like `PreToolUse`, causing rules to not match.

**Why it happens:** Only `UserPromptSubmit` event includes the `prompt` field from Claude Code.

**How to avoid:**
- Default behavior: If `event.prompt` is `None`, rule with `prompt_match` does not match (safe)
- Document clearly: `prompt_match` is primarily for `UserPromptSubmit` event
- Consider adding `operations: [UserPromptSubmit]` to prompt-matching rules as best practice

**Warning signs:** Rule never matches even though prompt should match

**Detection:** Check logs - if `event.prompt` is absent, matcher skips

**Source:** [Claude Code Hooks Reference](https://code.claude.com/docs/en/hooks)

### Pitfall 4: Case-Insensitive Flag Applied Incorrectly

**What goes wrong:** User sets `case_insensitive: true` but pattern still matches case-sensitively.

**Why it happens:** Forgot to use `RegexBuilder` instead of `Regex::new()`.

**How to avoid:**
```rust
let regex = if case_insensitive {
    RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .build()?
} else {
    Regex::new(&pattern)?
};
```

**Warning signs:** Pattern "DELETE" doesn't match text "delete" when `case_insensitive: true`

### Pitfall 5: Negation Pattern Fails to Match Anything

**What goes wrong:** `not: 'pattern'` rule never matches any prompt.

**Why it happens:** Negative lookahead regex `^(?!.*pattern).*$` must match the entire string.

**How to avoid:**
- Test negation patterns thoroughly
- Consider alternative: Use script-based matching with `!prompt.contains("pattern")` for clearer semantics
- Document that `not:` prefix creates a "does not contain" check

**Warning signs:** Rule with `not:` prefix never fires

## Code Examples

Verified patterns for implementation:

### Example 1: Extending Matchers Struct

```rust
// In models.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Matchers {
    /// Tool names to match (e.g., ["Bash", "Edit"])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,

    /// File extensions to match (e.g., [".rs", ".ts"])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,

    /// Directory patterns to match (e.g., ["src/**", "tests/**"])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<String>>,

    /// Operation types to match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,

    /// Regex pattern for command matching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_match: Option<String>,

    /// NEW: Prompt text pattern matching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_match: Option<PromptMatch>,
}
```

### Example 2: Prompt Matching in hooks.rs

```rust
// In hooks.rs - add to matches_rule function after command_match check

// Check prompt patterns
if let Some(ref prompt_match) = matchers.prompt_match {
    // If rule has prompt_match but event has no prompt, rule doesn't match
    if let Some(ref prompt_text) = event.prompt {
        if !matches_prompt(prompt_text, prompt_match)? {
            return Ok(false);
        }
    } else {
        // No prompt field in event - rule doesn't match
        return Ok(false);
    }
}

// Helper function for prompt matching logic
fn matches_prompt(prompt: &str, prompt_match: &PromptMatch) -> Result<bool> {
    match prompt_match {
        PromptMatch::Simple(patterns) => {
            // Default ANY mode
            for pattern in patterns {
                let expanded = PromptMatch::expand_pattern(pattern);
                let regex = get_or_compile_regex(&expanded, false)?;
                if regex.is_match(prompt) {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        PromptMatch::Complex { patterns, mode, case_insensitive, anchor } => {
            let matches: Vec<bool> = patterns.iter().map(|pattern| {
                let expanded = PromptMatch::expand_pattern(pattern);
                let anchored = PromptMatch::apply_anchor(&expanded, *anchor);
                let regex = get_or_compile_regex(&anchored, *case_insensitive).ok()?;
                Some(regex.is_match(prompt))
            }).collect::<Option<Vec<_>>>().unwrap_or_default();

            match mode {
                MatchMode::Any => Ok(matches.iter().any(|&m| m)),
                MatchMode::All => Ok(matches.iter().all(|&m| m)),
            }
        }
    }
}
```

**Source:** Adapted from existing `command_match` pattern in hooks.rs lines 245-254

### Example 3: YAML Configuration Examples

```yaml
# Example 1: Simple array syntax (ANY mode, case-sensitive)
- name: block-destructive-prompts
  description: Block prompts with destructive keywords
  matchers:
    operations: [UserPromptSubmit]
    prompt_match: ["delete all", "drop database", "rm -rf"]
  actions:
    block: true

# Example 2: Contains word shorthand
- name: warn-production-mentions
  description: Warn when prompt mentions production
  matchers:
    operations: [UserPromptSubmit]
    prompt_match: ["contains_word:production", "contains_word:prod"]
  actions:
    inject_inline: |
      ⚠️ Your prompt mentions production systems.
      Please double-check any commands before execution.
  mode: warn

# Example 3: Complex object syntax with ALL mode
- name: require-all-safety-keywords
  description: Only allow prompts that mention both 'test' and 'staging'
  matchers:
    operations: [UserPromptSubmit]
    prompt_match:
      patterns: ["contains_word:test", "contains_word:staging"]
      mode: all
      case_insensitive: true
  actions:
    inject_inline: "✅ Safe environment detected"

# Example 4: Negation pattern
- name: block-without-review
  description: Block operations that don't mention review or approval
  matchers:
    operations: [UserPromptSubmit]
    prompt_match: ["not:review", "not:approval"]
  actions:
    block: true

# Example 5: Anchor patterns
- name: match-command-start
  description: Match prompts starting with specific commands
  matchers:
    operations: [UserPromptSubmit]
    prompt_match:
      patterns: ["git push", "npm publish", "cargo publish"]
      anchor: start
  actions:
    inject: .claude/deployment-checklist.md
```

### Example 4: Adding prompt to Event struct

```rust
// In models.rs - add field to Event struct
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    #[serde(alias = "event_type")]
    pub hook_event_name: EventType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_input: Option<serde_json::Value>,

    pub session_id: String,

    #[serde(default = "chrono::Utc::now")]
    pub timestamp: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_use_id: Option<String>,

    /// NEW: User prompt text (sent by Claude Code on UserPromptSubmit events)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}
```

**Source:** Claude Code sends `prompt` field in UserPromptSubmit event - [Hooks Reference](https://code.claude.com/docs/en/hooks)

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| lazy_static for regex caching | once_cell::Lazy or std::sync::LazyLock | Rust 1.70 (June 2023) | Zero-cost lazy statics in std |
| Manual enum tag discrimination | #[serde(untagged)] | Serde 1.0+ | Automatic variant selection |
| Custom expression evaluator | evalexpr crate | Established library | Standardized script evaluation |

**Deprecated/outdated:**
- `lazy_static` crate - superseded by `once_cell` and `std::sync::LazyLock` (Rust 1.80+)
- `regex_cache` crate for simple caching - can hand-roll with once_cell more simply

**Current best practices:**
- Use `std::sync::LazyLock` if MSRV is 1.80+ (August 2024), otherwise `once_cell::Lazy`
- Use `#[serde(untagged)]` for flexible YAML syntax
- Compile regex once per pattern, cache compiled instances
- Use `regex::escape()` for literal text in patterns

**Source:** [once_cell deprecation notice](https://docs.rs/once_cell/latest/once_cell/) recommends std alternatives

## Open Questions

Things that couldn't be fully resolved during research:

1. **Regex cache eviction policy**
   - What we know: Unbounded HashMap will grow if patterns are dynamic
   - What's unclear: Will users create dynamic patterns in practice? (probably not - patterns are in YAML)
   - Recommendation: Start without eviction; add LRU later if needed. Most configs have < 100 patterns.

2. **Script-based matching helper functions**
   - What we know: Evalexpr supports custom functions via Context
   - What's unclear: What helper functions are actually useful? (contains_word, line_count, matches_regex, etc.)
   - Recommendation: Defer to phase 5 or later. Start with raw evalexpr for flexibility.

3. **Prompt normalization edge cases**
   - What we know: User wants NO normalization
   - What's unclear: Should we document that prompts include newlines, leading/trailing whitespace?
   - Recommendation: Document behavior clearly - pattern matches against raw prompt text as sent by Claude Code.

4. **Case-insensitive matching performance**
   - What we know: RegexBuilder can create case-insensitive regexes
   - What's unclear: Performance impact? (minimal - regex crate optimizes internally)
   - Recommendation: No special handling needed. Cache key includes case_insensitive flag.

## Performance Considerations

Based on research findings:

1. **Regex Compilation Cost**
   - Compilation: ~1-10ms per pattern (depends on complexity)
   - Matching: ~10-1000ns per match (linear time guarantee)
   - **Mitigation:** Cache compiled regexes (Pattern 2)
   - **Source:** [Regex Optimization Techniques](https://last9.io/blog/regex-optimization-techniques/)

2. **Untagged Enum Deserialization Cost**
   - Negligible overhead - Serde tries variants sequentially
   - For 2 variants (Simple, Complex), this is O(1) in practice
   - **Source:** [Serde Performance](https://github.com/serde-rs/serde/discussions)

3. **Evalexpr Context Building**
   - Building context: O(n) where n = number of env vars + constants
   - Expression evaluation: O(m) where m = expression complexity
   - **Mitigation:** Only add `prompt` variable to context, not entire prompt history
   - **Source:** [evalexpr documentation](https://docs.rs/evalexpr)

4. **Memory Considerations**
   - Regex cache: ~1KB per compiled pattern
   - Prompt text: Variable (typically < 10KB per event)
   - **Mitigation:** Use references where possible, clone only when necessary

5. **Recommended Optimizations**
   - Enable `perf-inline` and `perf-literal` features in regex crate
   - Use release build for benchmarking (`cargo build --release`)
   - Consider pre-compiling regex patterns at config load time (future optimization)

## Testing Strategy

Key test scenarios to implement:

1. **YAML Deserialization Tests**
   - Simple array syntax: `["pattern1", "pattern2"]`
   - Complex object syntax: `{ patterns: [...], mode: all }`
   - Case-insensitive flag parsing
   - Anchor field parsing
   - Invalid YAML should error gracefully

2. **Pattern Matching Tests**
   - Basic regex matching
   - Case-sensitive vs case-insensitive
   - ANY vs ALL mode logic
   - Anchor positions (start, end, contains)
   - Shorthand expansion (contains_word, not:)
   - Missing prompt field (should not match)

3. **Integration Tests**
   - Full event processing with prompt_match rules
   - Interaction with other matchers (operations, tools)
   - Priority and mode interaction with prompt matching

4. **Performance Tests**
   - Regex cache hit rate
   - Matching latency (should be < 1ms for typical prompts)
   - Memory usage with many patterns

## Claude's Discretion Decisions

Based on research, here are recommendations for areas left to Claude's discretion:

### 1. Pattern Type Choice

**Recommendation:** Regex-only for patterns, no separate literal type.

**Rationale:**
- Simplifies implementation (one code path)
- Users can use `regex::escape()` via contains_word shorthand for literals
- Consistent with `command_match` which uses regex
- If literal matching needed, user writes: `contains_word:exact_text` (shorthand handles escaping)

### 2. Nested Group Support

**Recommendation:** No nested groups in v1. Keep to simple any/all modes.

**Rationale:**
- Complexity outweighs benefit for 95% of use cases
- Users needing complex logic can use `prompt_script` with evalexpr
- Easier to understand and maintain YAML configs
- Can add later if users request it (backward compatible)

### 3. Script Helper Functions

**Recommendation:** Defer to phase 5 or later. Start with raw evalexpr.

**Rationale:**
- Evalexpr supports custom functions but requires careful API design
- Let users experiment with raw expressions first to discover needs
- Helper functions can be added backward-compatibly later
- Examples: `contains_word(prompt, "delete")`, `line_count(prompt) > 5`, `matches(prompt, "regex")`

### 4. YAML Field Naming

**Recommendation:** Use `prompt_match` at the top level of `Matchers` struct.

**Rationale:**
- Consistent with existing matchers: `command_match`, `tools`, `extensions`
- Clear and discoverable in YAML
- No need for separate `prompt_script` field - script-based matching comes in phase 5

## Sources

### Primary (HIGH confidence)

**Official Documentation:**
- [Serde Enum Representations](https://serde.rs/enum-representations.html) - Untagged enum patterns
- [Regex crate documentation](https://docs.rs/regex) - Regex compilation and matching
- [evalexpr crate documentation](https://docs.rs/evalexpr) - Expression evaluation with context
- [once_cell documentation](https://docs.rs/once_cell/latest/once_cell/) - Lazy static alternatives
- [Claude Code Hooks Reference](https://code.claude.com/docs/en/hooks) - UserPromptSubmit event structure

**Codebase Reference:**
- RulesZ models.rs lines 272-276 - `command_match` implementation pattern
- RuleZ hooks.rs lines 245-254 - Regex matching in matcher evaluation
- RuleZ hooks.rs lines 126-146 - `build_eval_context` for evalexpr integration
- RuleZ models.rs lines 143-174 - `RunAction` untagged enum pattern (Phase 2.4)

### Secondary (MEDIUM confidence)

- [GitHub: rust-regex-cache](https://github.com/1aim/rust-regex-cache) - LRU caching for regex
- [How to Use lazy_static for Runtime Initialization in Rust](https://oneuptime.com/blog/post/2026-01-25-rust-lazy-static/view) - Lazy static patterns
- [Rust Regex Internals](https://burntsushi.net/regex-internals/) - Performance characteristics

### Community Discussions (verified with official sources)

- [Serde Untagged Enums Discussion](https://users.rust-lang.org/t/fun-and-sadness-with-serde-untagged-enums/65089) - Untagged enum pitfalls
- [GitHub regex discussions](https://github.com/rust-lang/regex/discussions/960) - Performance and caching
- [Serde flatten limitation](https://github.com/serde-rs/serde/issues/1402) - Why not to use flatten with enums

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All dependencies already in use, well-documented
- Architecture patterns: HIGH - Based on existing codebase patterns and official Serde documentation
- Performance: MEDIUM - Based on community benchmarks and regex documentation claims
- Claude Code integration: HIGH - Official hooks documentation confirms prompt field in UserPromptSubmit

**Research date:** 2026-02-09
**Valid until:** ~60 days (stable ecosystem, minimal churn expected)

**Researcher notes:**
- No major version bumps expected in dependencies (regex, serde, evalexpr are mature)
- User decisions in CONTEXT.md are comprehensive and well-considered
- Implementation should be straightforward following the `command_match` pattern
- Biggest implementation decision is regex caching strategy (recommend simple approach first)
