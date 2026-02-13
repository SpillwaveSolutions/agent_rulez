# Phase 5: Field Validation - Research

**Researched:** 2026-02-09
**Domain:** JSON field validation and type checking in Rust
**Confidence:** HIGH

## Summary

Field validation for tool inputs in RuleZ requires validating that specific JSON fields exist and match expected types, supporting nested field access via dot notation. The research confirms that **custom validation using serde_json is the optimal approach** rather than external validation libraries like jsonschema. This decision is based on three factors: (1) RuleZ's requirements are simpler than full JSON Schema validation, (2) serde_json provides all necessary primitives for dot-notation field traversal and type checking, and (3) consistency with existing Phase 4 patterns (prompt_match validation, fail-closed philosophy).

The implementation follows established RuleZ patterns: validation-at-load for configuration (Phase 4), fail-closed blocking on validation failure (Phase 3), and integration with existing matcher architecture. The `pointer()` method in serde_json provides efficient nested field access via JSON Pointer (RFC 6901) format, which can be adapted for dot notation.

**Primary recommendation:** Implement custom field validation using serde_json's Value type with pointer-based nested field access, validated at config load time, integrated as a matcher condition in the existing rule evaluation pipeline.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Validation Syntax:**
- `require_fields` is a simple list of field name strings: `require_fields: ["file_path", "content"]`
- `field_types` is a separate optional companion key as a YAML map: `field_types: {file_path: string, line: number}`
- Dot notation for nested fields with unlimited depth: `require_fields: ["input.user.address.city"]`
- No wildcard (`items.*.name`) or array index (`items[0].id`) notation — keep it simple

**Failure Behavior:**
- Report ALL missing/invalid fields in a single error, not just the first
- Block on type mismatch — wrong type is as bad as missing field (fail-closed)
- Error messages show types only, not actual values — avoids leaking sensitive data (e.g., "field 'count' expected number, got string")

**Type Checking Depth:**
- Strict JSON types only — no coercion ("42" is a string, not a number)
- Supported types: string, number, boolean, array, object, any
- Array type checks only "is it an array" — no element type validation
- Object type checks only "is it an object" — use dot notation for inner field requirements
- `any` type supported — field must exist but can be any JSON type

**Edge Cases & Defaults:**
- Null JSON values count as missing (null = absent for require_fields)
- Validate `require_fields` and `field_types` at config load time (consistent with prompt_match validation pattern)
- Missing/invalid `tool_input` causes all require_fields checks to fail (fail-closed, block)

### Claude's Discretion

- Whether `require_fields` acts as a matching condition or post-match validation (architectural decision for hooks.rs)
- Whether empty strings and empty arrays count as "present" (choose based on JSON semantics)
- Error message formatting and structure
- How field_types interacts with require_fields when both specify the same field

### Deferred Ideas (OUT OF SCOPE)

- Array element type validation (`array<string>`) — future enhancement
- Wildcard/glob field paths (`items.*.name`) — future enhancement
- Nested object schema validation (JSON Schema-like) — future enhancement

</user_constraints>

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| serde_json | 1.x (already in use) | JSON manipulation, field access, type checking | Already a dependency, provides Value type with pointer() method for RFC 6901 path access, no additional dependencies needed |
| regex | 1.x (already in use) | Pattern validation for field names | Already used for prompt_match, consistent dependency |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| N/A | - | - | Custom validation is sufficient for this phase's requirements |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Custom validation | jsonschema 0.41 | jsonschema provides full JSON Schema Draft 7 validation but adds 20+ transitive dependencies (regex-syntax, fancy-regex, url, percent-encoding, etc.) for features we don't need. Custom validation with serde_json is lighter, more maintainable, and follows RuleZ patterns. |
| Custom validation | serde_valid | Designed for struct-level validation via derive macros, not dynamic JSON Value validation. Doesn't fit RuleZ's dynamic rule engine architecture. |
| Custom validation | validator crate | Similar to serde_valid, targets typed structs not dynamic JSON. Requires defining Rust types for all possible tool_input shapes. |
| Dot notation (`user.name`) | JSON Pointer (`/user/name`) | JSON Pointer (RFC 6901) is the underlying standard. Convert dot notation to JSON Pointer format internally for consistency with serde_json's pointer() API. |

**Installation:**

No additional dependencies required. serde_json and regex are already in Cargo.toml.

## Architecture Patterns

### Recommended Project Structure

Field validation logic integrates into existing structure:

```
rulez/src/
├── models.rs           # Add FieldValidation types alongside PromptMatch
├── config.rs           # Add validation logic for require_fields/field_types at load
├── hooks.rs            # Add field validation matching logic in evaluate_rules
└── tests/
    ├── field_validation_unit.rs        # Unit tests for field path parsing
    └── field_validation_integration.rs # E2E tests using full stack
```

### Pattern 1: Dot Notation to JSON Pointer Conversion

**What:** Convert user-friendly dot notation (`user.name`) to JSON Pointer format (`/user/name`) for serde_json compatibility

**When to use:** During field path validation and at runtime field lookup

**Example:**
```rust
// Source: Custom implementation based on RFC 6901 and serde_json docs
fn dot_to_pointer(field_path: &str) -> String {
    // Convert "user.name.first" to "/user/name/first"
    // Escape special characters per RFC 6901: ~ becomes ~0, / becomes ~1
    field_path
        .split('.')
        .map(|segment| {
            segment
                .replace("~", "~0")
                .replace("/", "~1")
        })
        .collect::<Vec<_>>()
        .join("/")
        .insert_str(0, "/")
}
```

### Pattern 2: Field Existence and Type Validation

**What:** Check if a field exists at a given path and optionally validate its JSON type

**When to use:** During rule evaluation when require_fields or field_types is specified

**Example:**
```rust
// Source: Adapted from https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html
use serde_json::Value;

fn validate_field(
    tool_input: &Value,
    field_path: &str,
    expected_type: Option<&str>
) -> Result<(), String> {
    let pointer = dot_to_pointer(field_path);

    match tool_input.pointer(&pointer) {
        None => Err(format!("field '{}' is missing", field_path)),
        Some(Value::Null) => Err(format!("field '{}' is null (treated as missing)", field_path)),
        Some(value) => {
            if let Some(expected) = expected_type {
                validate_type(value, expected, field_path)
            } else {
                Ok(()) // Field exists, type check not required
            }
        }
    }
}

fn validate_type(value: &Value, expected: &str, field_path: &str) -> Result<(), String> {
    let matches = match expected {
        "string" => value.is_string(),
        "number" => value.is_number(),
        "boolean" => value.is_boolean(),
        "array" => value.is_array(),
        "object" => value.is_object(),
        "any" => true, // any type accepted
        _ => return Err(format!("unsupported type specifier: {}", expected)),
    };

    if matches {
        Ok(())
    } else {
        let actual = match value {
            Value::String(_) => "string",
            Value::Number(_) => "number",
            Value::Bool(_) => "boolean",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Null => "null",
        };
        Err(format!(
            "field '{}' expected {}, got {}",
            field_path, expected, actual
        ))
    }
}
```

### Pattern 3: Configuration-Time Validation

**What:** Validate field paths and type specifiers when loading hooks.yaml, similar to prompt_match validation

**When to use:** In Config::validate() method

**Example:**
```rust
// Source: Pattern established in rulez/src/config.rs:152-187 (Phase 4)
impl Config {
    pub fn validate(&self) -> Result<()> {
        for rule in &self.rules {
            // Validate require_fields
            if let Some(ref fields) = rule.matchers.require_fields {
                if fields.is_empty() {
                    return Err(anyhow::anyhow!(
                        "Empty require_fields array in rule '{}'",
                        rule.name
                    ));
                }

                for field_path in fields {
                    // Validate dot notation syntax
                    if field_path.is_empty() || field_path.starts_with('.') || field_path.ends_with('.') {
                        return Err(anyhow::anyhow!(
                            "Invalid field path '{}' in rule '{}'",
                            field_path, rule.name
                        ));
                    }
                }
            }

            // Validate field_types
            if let Some(ref types) = rule.matchers.field_types {
                for (field_path, type_spec) in types {
                    // Validate type specifier
                    if !["string", "number", "boolean", "array", "object", "any"].contains(&type_spec.as_str()) {
                        return Err(anyhow::anyhow!(
                            "Invalid type '{}' for field '{}' in rule '{}'",
                            type_spec, field_path, rule.name
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
```

### Pattern 4: Integration with Matcher Pipeline

**What:** Field validation as a matcher condition, evaluated alongside tools, extensions, prompt_match

**When to use:** In evaluate_rules() when rule has require_fields or field_types

**Architectural recommendation:** Treat as a **pre-match validation** (like enabled_when) rather than a post-match validation. Rationale:
1. **Fail-fast**: Invalid tool_input should prevent rule matching entirely
2. **Consistency**: Mirrors enabled_when pattern (condition fails → rule doesn't match)
3. **Clarity**: Matchers section logically groups all conditions that must be true for rule to apply

**Example:**
```rust
// Source: Pattern from rulez/src/hooks.rs:292-300 (evaluate_rules)
async fn evaluate_rules<'a>(
    event: &'a Event,
    config: &'a Config,
    debug_config: &DebugConfig,
) -> Result<(Vec<&'a Rule>, Response, Vec<RuleEvaluation>)> {
    for rule in config.enabled_rules() {
        // Check field validation BEFORE other matchers
        if !validate_required_fields(rule, event)? {
            // Log debug info if enabled
            continue; // Rule doesn't match
        }

        // Then check other matchers (tools, extensions, prompt_match, etc.)
        // ...
    }
}
```

### Anti-Patterns to Avoid

- **Don't leak field values in error messages:** Show only types, never actual values (security/privacy)
- **Don't short-circuit on first error:** Collect all field validation errors and report together (better UX)
- **Don't conflate null with present:** Null JSON values are treated as missing per user constraints
- **Don't use array/object for nested validation:** Use dot notation instead (simpler, explicit)

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JSON Pointer parsing | Custom path parser with `/` splitting and escaping | serde_json::Value::pointer() | Handles RFC 6901 escaping (~0, ~1), edge cases, already tested |
| Type coercion ("42" → 42) | String-to-number conversion logic | Strict type checking with Value::is_*() | User constraint: no coercion, strict JSON types |
| Full JSON Schema validation | Subset of JSON Schema Draft 7 | Keep it simple with require_fields + field_types | Phase boundary excludes JSON Schema features |

**Key insight:** The temptation to use jsonschema is strong (it's well-tested, feature-complete), but it's overkill for this phase. RuleZ only needs field existence + type checks, not conditionals, allOf, anyOf, pattern properties, etc. Custom validation is 100 lines vs 20+ dependencies.

## Common Pitfalls

### Pitfall 1: Treating Empty Strings/Arrays as Missing

**What goes wrong:** Developer might treat `{"name": ""}` or `{"items": []}` as "field is missing"

**Why it happens:** Confusion between "field doesn't exist," "field is null," and "field is empty value"

**How to avoid:** Follow JSON semantics strictly:
- Missing field: `{"user": {}}` where `user.name` is not in the object
- Null field: `{"user": {"name": null}}`
- Empty string: `{"user": {"name": ""}}`
- Empty array: `{"items": []}`

Per serde_json documentation (https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html):
- Empty strings (`""`) and empty arrays (`[]`) are **present** values
- Only `null` and missing keys are treated as absent
- `Value::pointer()` returns `Some(Value::Null)` for null, `None` for missing

**Recommendation:** Empty strings and empty arrays count as **present**. This aligns with JSON semantics and serde_json behavior.

**Warning signs:** Tests showing `{"name": ""}` failing require_fields check

### Pitfall 2: Dot Notation Edge Cases

**What goes wrong:** Field paths like `.name`, `name.`, `name..field`, or `name.` cause runtime panics or incorrect validation

**Why it happens:** Inadequate input validation at config load time

**How to avoid:** Validate field paths in Config::validate():
- Reject paths starting or ending with `.`
- Reject paths with consecutive dots (`..`)
- Reject empty segments between dots
- Reject empty field paths

**Warning signs:** Panics in pointer() calls, unexpected validation failures

### Pitfall 3: Type Mismatches Between require_fields and field_types

**What goes wrong:** User specifies `require_fields: ["count"]` and `field_types: {total: number}` — different field sets, unclear intent

**Why it happens:** Two separate keys that can overlap or diverge

**How to avoid:**
- When field appears in both: field_types wins (it's more specific)
- When field only in require_fields: existence check only
- When field only in field_types: existence + type check (implicit requirement)

**Recommendation:** field_types **implies** require_fields. If `field_types: {count: number}`, the field must exist AND be a number. Users don't need to duplicate `count` in require_fields.

**Warning signs:** User confusion about why field_types doesn't enforce existence

### Pitfall 4: Missing tool_input Handling

**What goes wrong:** Rule with require_fields processes event where tool_input is None, causing unclear error or panic

**Why it happens:** Not all events have tool_input (UserPromptSubmit, SessionStart, etc.)

**How to avoid:** Fail-closed behavior:
- If tool_input is None → all require_fields checks fail
- If tool_input is not an object (array, string, number) → all checks fail
- Log clear error: "tool_input missing or invalid, field validation failed"

**Warning signs:** Panics or confusing errors when applying field rules to prompt-only events

## Code Examples

Verified patterns from serde_json official sources:

### Nested Field Access with pointer()

```rust
// Source: https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html
use serde_json::{json, Value};

fn main() {
    let data = json!({
        "user": {
            "name": "Alice",
            "address": {
                "city": "London"
            }
        },
        "count": 42
    });

    // Access nested fields using JSON Pointer format
    assert_eq!(data.pointer("/user/name"), Some(&Value::String("Alice".to_string())));
    assert_eq!(data.pointer("/user/address/city"), Some(&Value::String("London".to_string())));
    assert_eq!(data.pointer("/count"), Some(&Value::Number(42.into())));

    // Missing field returns None
    assert_eq!(data.pointer("/user/email"), None);

    // Null field returns Some(Value::Null)
    let data_with_null = json!({"user": {"name": null}});
    assert_eq!(data_with_null.pointer("/user/name"), Some(&Value::Null));
}
```

### Type Checking with Value Methods

```rust
// Source: https://context7.com/serde-rs/json/llms.txt
use serde_json::{json, Value};

fn main() {
    let data = json!({
        "name": "John",
        "age": 43,
        "active": true,
        "tags": ["rust", "json"],
        "metadata": {"version": "1.0"}
    });

    // Type checking methods
    if data["name"].is_string() {
        println!("name is a string");
    }

    if data["age"].is_number() {
        println!("age is a number");
    }

    if data["active"].is_boolean() {
        println!("active is a boolean");
    }

    if data["tags"].is_array() {
        println!("tags is an array");
    }

    if data["metadata"].is_object() {
        println!("metadata is an object");
    }

    // Missing keys return Value::Null, which is none of the above types
    assert!(!data["nonexistent"].is_string());
    assert!(data["nonexistent"].is_null());
}
```

### Safe Field Extraction with get()

```rust
// Source: https://context7.com/serde-rs/json/llms.txt
use serde_json::{json, Value};

fn main() {
    let obj = json!({
        "name": "Alice",
        "age": 30
    });

    // Safe access with get (returns Option<&Value>)
    if let Some(name) = obj.get("name") {
        if let Some(s) = name.as_str() {
            println!("Name string: {}", s);
        }
    }

    // Check key existence
    if obj.get("email").is_none() {
        println!("Email not found");
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| JSON Schema libraries for all validation | Lightweight custom validation for simple cases | 2024+ | Modern Rust projects increasingly use custom validation for domain-specific needs, reserving jsonschema for API contracts and complex schemas. RuleZ falls into "simple case" category. |
| String-based field paths | JSON Pointer (RFC 6901) | Standard since 2013 | serde_json's pointer() method provides robust, tested implementation. No need to reinvent. |
| Type coercion by default | Strict type checking | serde_json always strict | JSON distinguishes "42" (string) from 42 (number). RuleZ follows this, no coercion needed. |

**Deprecated/outdated:**
- **json_dotpath crate**: While it exists (https://lib.rs/crates/json_dotpath), it's unnecessary when serde_json's pointer() handles the core use case. Converting dot notation to JSON Pointer format is trivial.

## Open Questions

1. **Should field_types automatically imply require_fields?**
   - What we know: Two separate YAML keys, can specify independently
   - What's unclear: User intent when field appears in field_types but not require_fields
   - Recommendation: **field_types implies existence requirement** (simpler mental model, reduces duplication)

2. **Should we support escaped dots in field names?**
   - What we know: JSON allows keys with literal dots: `{"user.name": "value"}`
   - What's unclear: Whether to support `user\.name` to match such keys
   - Recommendation: **Not in this phase** — deferred to future enhancement. JSON Pointer escaping (~0, ~1) handles special chars.

3. **Should validation errors accumulate or short-circuit?**
   - What we know: User constraint says "report ALL missing/invalid fields"
   - What's unclear: Performance impact of checking all fields vs stopping at first error
   - Recommendation: **Accumulate all errors** per user constraint, performance impact negligible for typical rule configs (<50 fields)

## Sources

### Primary (HIGH confidence)

- [serde_json Value documentation](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html) - pointer() method, type checking
- [Context7 serde-rs/json](https://context7.com/serde-rs/json/llms.txt) - code examples for JSON manipulation
- RFC 6901 JSON Pointer - standard for path-based field access
- RuleZ codebase: rulez/src/config.rs (validation patterns), rulez/src/hooks.rs (matcher integration)

### Secondary (MEDIUM confidence)

- [LogRocket: JSON input validation in Rust](https://blog.logrocket.com/json-input-validation-in-rust-web-services/) - best practices for custom validation
- [Vinted Engineering: Validating JSON in Rust](https://vinted.engineering/2021/02/15/validating-json-input-in-rust-web-services/) - validator crate patterns
- [jsonschema crate](https://docs.rs/jsonschema) - evaluated for comparison, not recommended for this use case
- [Rust users forum: JSON type validation](https://users.rust-lang.org/t/json-type-validation/95905) - community discussion on approaches

### Tertiary (LOW confidence)

- json_dotpath crate - exists but not needed, pointer() is sufficient

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - serde_json already in use, proven for this use case
- Architecture: HIGH - follows established RuleZ patterns from Phase 4
- Pitfalls: MEDIUM - identified from first principles and serde_json docs, not battle-tested in RuleZ context yet

**Research date:** 2026-02-09
**Valid until:** 90 days (stable domain, serde_json is mature)
