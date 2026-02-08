# Phase 1: Inline Content Injection - Research

**Researched:** 2026-02-06
**Domain:** Rust/serde_yaml YAML parsing, RuleZ rule engine extension
**Confidence:** HIGH

## Summary

This phase adds a new `inject_inline` action field to the RuleZ rule engine, allowing users to embed markdown content directly in their YAML configuration instead of referencing external files. This is a straightforward extension of the existing `inject` action pattern.

The implementation requires:
1. Adding `inject_inline: Option<String>` to the `Actions` struct in `models.rs`
2. Handling the new field in `hooks.rs` alongside existing `inject` handling
3. No validation changes needed in `config.rs` (serde handles YAML parsing automatically)

The existing serde_yaml 0.9 library already supports YAML multiline string syntax (`|` for literal blocks, `>` for folded blocks) natively. No additional parsing logic is required - serde deserializes multiline YAML strings into Rust `String` values automatically.

**Primary recommendation:** Add `inject_inline: Option<String>` to `Actions` struct, handle it in `execute_rule_actions()` with simple string injection (no file I/O needed).

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| serde | 1.0 | Serialization framework | Rust ecosystem standard |
| serde_yaml | 0.9 | YAML parsing | Already used in codebase |
| tokio | 1.0 | Async runtime | Already used for file I/O |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| anyhow | 1.0 | Error handling | Already used for Result types |
| tracing | 0.1 | Logging | Already used for warnings |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| serde_yaml | yaml-rust2 | yaml-rust2 is newer but serde_yaml already in use |
| String field | Custom newtype | Unnecessary complexity for simple string |

**No new dependencies required.**

## Architecture Patterns

### Recommended Project Structure
```
src/
├── models.rs        # Add inject_inline field to Actions struct
├── hooks.rs         # Handle inject_inline in execute_rule_actions()
└── config.rs        # No changes needed (serde handles YAML)
```

### Pattern 1: Parallel Action Fields
**What:** Add new action fields alongside existing ones with same Optional pattern
**When to use:** When extending rule actions with new capabilities
**Example:**
```rust
// Source: Existing Actions struct pattern in models.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Actions {
    /// Path to context file to inject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject: Option<String>,

    /// Inline markdown content to inject (new)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject_inline: Option<String>,

    // ... other fields
}
```

### Pattern 2: Action Priority Handling
**What:** When both `inject` and `inject_inline` are present, define clear precedence
**When to use:** When multiple related action fields could conflict
**Example:**
```rust
// In execute_rule_actions():
// Priority: inject_inline > inject (inline is more explicit)
if let Some(ref inline_content) = actions.inject_inline {
    return Ok(Response::inject(inline_content.clone()));
}
if let Some(ref inject_path) = actions.inject {
    // existing file injection logic
}
```

### Anti-Patterns to Avoid
- **Nested structs for simple additions:** Don't create `InjectAction { path: Option<String>, inline: Option<String> }` - adds unnecessary complexity
- **Merging inject types:** Don't try to combine file content and inline content - users should choose one per rule
- **Validation of content format:** Don't validate that inline content is valid markdown - RuleZ is format-agnostic

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| YAML multiline parsing | Custom string handling | serde_yaml native support | Already handles `\|` and `>` syntax |
| Optional field serialization | Manual JSON building | `#[serde(skip_serializing_if = "Option::is_none")]` | Pattern already used |
| Response creation | Manual struct building | `Response::inject()` | Existing helper method |

**Key insight:** serde_yaml 0.9 automatically handles YAML multiline string syntax. The `|` (literal) and `>` (folded) indicators are YAML features parsed at the library level - no special Rust code needed.

## Common Pitfalls

### Pitfall 1: Forgetting skip_serializing_if
**What goes wrong:** Empty `inject_inline` fields appear in serialized YAML output
**Why it happens:** Default serde behavior includes None as `null`
**How to avoid:** Always add `#[serde(skip_serializing_if = "Option::is_none")]`
**Warning signs:** YAML output shows `inject_inline: null` or `inject_inline: ~`

### Pitfall 2: Clone vs Reference in Response
**What goes wrong:** Unnecessary string allocations
**Why it happens:** Using `inline_content.clone()` when not needed
**How to avoid:** Use `.clone()` - Response::inject takes `impl Into<String>` which handles both
**Warning signs:** N/A - minor performance, acceptable pattern

### Pitfall 3: Missing Test for Multiline Content
**What goes wrong:** Multiline YAML strings parse differently than expected
**Why it happens:** `|` preserves newlines, `>` folds them
**How to avoid:** Test both `|` and `>` syntax in unit tests
**Warning signs:** Injected content has wrong whitespace

### Pitfall 4: Conflict with Existing inject Field
**What goes wrong:** Both `inject` and `inject_inline` specified, unclear behavior
**Why it happens:** YAML allows both fields simultaneously
**How to avoid:** Define clear precedence (inject_inline wins) and optionally log warning
**Warning signs:** User confusion about which takes effect

## Code Examples

Verified patterns from the existing codebase:

### Adding Optional Field to Actions
```rust
// Source: models.rs line 274-301, existing pattern
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Actions {
    /// Path to context file to inject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject: Option<String>,

    /// Inline markdown content to inject directly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject_inline: Option<String>,

    // ... existing fields unchanged
}
```

### Handling New Action in hooks.rs
```rust
// Source: hooks.rs line 371-382, pattern for inject handling
// Add BEFORE existing inject handling:
if let Some(ref inline_content) = actions.inject_inline {
    // No file I/O needed - content is already in memory
    return Ok(Response::inject(inline_content.clone()));
}

// Existing inject file handling follows
if let Some(ref inject_path) = actions.inject {
    match read_context_file(inject_path).await {
        Ok(context) => {
            return Ok(Response::inject(context));
        }
        Err(e) => {
            tracing::warn!("Failed to read context file '{}': {}", inject_path, e);
        }
    }
}
```

### YAML Multiline String Test
```rust
// Test both YAML multiline syntaxes
#[test]
fn test_inject_inline_literal_block() {
    let yaml = r#"
inject_inline: |
  ## Production Warning
  You are editing production files.
  Be extra careful.
"#;
    let actions: Actions = serde_yaml::from_str(yaml).unwrap();
    let content = actions.inject_inline.unwrap();
    assert!(content.contains("## Production Warning"));
    assert!(content.contains("\n")); // Literal preserves newlines
}

#[test]
fn test_inject_inline_folded_block() {
    let yaml = r#"
inject_inline: >
  This is a long paragraph that
  will be folded into a single line
  with spaces instead of newlines.
"#;
    let actions: Actions = serde_yaml::from_str(yaml).unwrap();
    let content = actions.inject_inline.unwrap();
    // Folded collapses newlines to spaces
    assert!(content.contains("This is a long paragraph"));
}

#[test]
fn test_inject_inline_simple_string() {
    let yaml = r#"
inject_inline: "Single line warning message"
"#;
    let actions: Actions = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(actions.inject_inline.unwrap(), "Single line warning message");
}
```

### Integration Test Pattern
```rust
// Following test pattern from oq_us2_injection.rs
#[test]
fn test_inject_inline_in_rule() {
    let yaml = r#"
version: "1.0"
rules:
  - name: warn-production
    matchers:
      directories: ["/prod/"]
    actions:
      inject_inline: |
        ## Production Warning
        You are editing production files. Be extra careful.
"#;
    let config: Config = serde_yaml::from_str(yaml).unwrap();
    let rule = &config.rules[0];
    assert!(rule.actions.inject_inline.is_some());
    assert!(rule.actions.inject_inline.as_ref().unwrap().contains("Production Warning"));
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| External files only (`inject:`) | Inline content also (`inject_inline:`) | This phase | Simpler single-file configs |

**Deprecated/outdated:**
- None - this is additive, not replacing existing functionality

## Open Questions

Things that couldn't be fully resolved:

1. **Should warn mode apply to inject_inline?**
   - What we know: Warn mode already handles file-based inject normally
   - What's unclear: Same treatment for inline makes sense, but worth confirming
   - Recommendation: Treat identically - inject_inline is just simpler inject

2. **Maximum inline content size?**
   - What we know: `settings.max_context_size` exists (default 1MB)
   - What's unclear: Should inline content be subject to same limit?
   - Recommendation: Apply same limit for consistency (optional enhancement, not P1)

## Sources

### Primary (HIGH confidence)
- `models.rs` lines 272-313 - Existing Actions struct and patterns
- `hooks.rs` lines 371-382 - Existing inject handling pattern
- `Cargo.toml` - serde_yaml 0.9 in use

### Secondary (MEDIUM confidence)
- serde_yaml 0.9 documentation - Multiline YAML string handling is automatic

### Tertiary (LOW confidence)
- None - this is well-established Rust/serde territory

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - No new dependencies, existing patterns
- Architecture: HIGH - Simple field addition following established pattern
- Pitfalls: HIGH - Based on direct codebase analysis

**Research date:** 2026-02-06
**Valid until:** 60 days (stable Rust patterns, no external API dependencies)

---

## Implementation Checklist Summary

For the planner:

1. **models.rs** - Add `inject_inline: Option<String>` to `Actions` struct
2. **hooks.rs** - Handle `inject_inline` in `execute_rule_actions()` before `inject`
3. **hooks.rs** - Handle `inject_inline` in `execute_rule_actions_warn_mode()` similarly
4. **Unit tests** - Test YAML parsing for literal (`|`), folded (`>`), and quoted strings
5. **Integration test** - Test full rule evaluation with inject_inline action
6. **No changes needed** - config.rs, Response type, or other files
