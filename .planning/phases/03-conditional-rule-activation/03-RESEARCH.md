# Phase 3: Conditional Rule Activation - Research

**Researched:** 2026-02-07
**Domain:** Rust expression evaluation, conditional rule logic
**Confidence:** HIGH

## Summary

This phase adds an `enabled_when` field to rules that allows conditional activation based on runtime context. When the expression evaluates to false, the rule is skipped entirely (not matched). This enables use cases like CI-only rules, environment-specific behaviors, and tool-aware rule logic.

The implementation requires:
1. Adding `enabled_when: Option<String>` to the `Rule` struct in `models.rs`
2. Integrating the `evalexpr` crate for expression evaluation
3. Building a context with `env.*`, `tool.name`, and `event.type` variables
4. Evaluating `enabled_when` before `matches_rule()` in the rule evaluation loop
5. Adding expression validation to `rulez validate` command

**evalexpr** is the recommended library because:
- Lightweight with no dependencies (aligns with RuleZ's design philosophy)
- Supports boolean expressions with comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`)
- Supports string comparison and logical operators (`&&`, `||`, `!`)
- Provides clear error types for validation (`EvalexprError`)
- Allows pre-compilation for performance (`build_operator_tree`)
- Current version: 13.1.0

**Primary recommendation:** Use `evalexpr` crate with `HashMapContext` for variable injection. Evaluate `enabled_when` expression before `matches_rule()` check. Pre-compile expressions during config loading for performance.

## Standard Stack

The established libraries/tools for this domain:

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| evalexpr | 13.1.0 | Expression parsing and evaluation | Lightweight, no deps, boolean support, Rust ecosystem standard for simple expressions |
| serde | 1.0 | Serialization | Already used in codebase |
| std::env | stdlib | Environment variable access | Standard library, no additional deps |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| anyhow | 1.0 | Error handling | Already used, wrap evalexpr errors |
| tracing | 0.1 | Logging | Debug expression evaluation |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| evalexpr | rhai | Rhai is full scripting language - overkill, larger binary, more complex |
| evalexpr | meval-rs | meval only supports math expressions, not boolean/string |
| evalexpr | custom parser | More work, less robust, no community testing |
| evalexpr | fasteval | Focused on numeric performance, less string support |

**Installation:**
```toml
# Add to rulez/Cargo.toml [dependencies]
evalexpr = "13.1"
```

## Architecture Patterns

### Recommended Project Structure
```
src/
├── models.rs        # Add enabled_when field to Rule struct
├── hooks.rs         # Evaluate enabled_when before matches_rule()
├── config.rs        # Validate expressions during config loading
└── expression.rs    # NEW: Expression evaluation context builder (optional)
```

### Pattern 1: Early Exit for Disabled Rules
**What:** Check `enabled_when` before other matchers to skip rules entirely
**When to use:** In the rule evaluation loop
**Example:**
```rust
// In evaluate_rules() in hooks.rs
for rule in config.enabled_rules() {
    // NEW: Check enabled_when first
    if !is_rule_enabled_for_context(rule, event)? {
        continue; // Skip rule entirely, don't even check matchers
    }

    let (matched, matcher_results) = if debug_config.enabled {
        matches_rule_with_debug(event, rule)
    } else {
        (matches_rule(event, rule), None)
    };
    // ... rest of evaluation
}
```

### Pattern 2: Context Variable Namespace
**What:** Use dot-notation for variable namespaces (`env.CI`, `tool.name`, `event.type`)
**When to use:** Building the evaluation context
**Example:**
```rust
use evalexpr::*;

fn build_expression_context(event: &Event) -> HashMapContext<DefaultNumericTypes> {
    let mut context = HashMapContext::new();

    // Environment variables as env.VAR_NAME
    for (key, value) in std::env::vars() {
        context.set_value(
            format!("env_{}", key).into(),
            Value::String(value)
        ).ok();
    }

    // Tool information
    if let Some(ref tool_name) = event.tool_name {
        context.set_value("tool_name".into(), Value::String(tool_name.clone())).ok();
    }

    // Event type
    context.set_value(
        "event_type".into(),
        Value::String(event.hook_event_name.to_string())
    ).ok();

    context
}
```

### Pattern 3: Pre-compiled Expressions
**What:** Parse expressions once during config load, store as `Node` for fast evaluation
**When to use:** Performance optimization when same rule evaluated many times
**Example:**
```rust
use evalexpr::{build_operator_tree, Node};

// During config loading
let compiled: Node = build_operator_tree("env_CI == \"true\"")?;

// During evaluation (fast)
let result: bool = compiled.eval_boolean_with_context(&context)?;
```

### Pattern 4: Expression Validation in Config
**What:** Validate all `enabled_when` expressions parse correctly during `rulez validate`
**When to use:** Config validation command
**Example:**
```rust
// In config.rs or validation module
fn validate_expressions(&self) -> Result<()> {
    for rule in &self.rules {
        if let Some(ref expr) = rule.enabled_when {
            build_operator_tree(expr).with_context(|| {
                format!("Invalid enabled_when expression in rule '{}': {}", rule.name, expr)
            })?;
        }
    }
    Ok(())
}
```

### Anti-Patterns to Avoid
- **Evaluating after matchers:** Wasted work - check `enabled_when` first
- **Blocking on expression errors:** Use fail-open like other RuleZ features
- **Direct env var names:** Use namespace (`env.CI` not `CI`) to avoid collisions
- **Storing raw strings:** Pre-compile to `Node` for performance if rules are evaluated frequently

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Expression parsing | Custom tokenizer | evalexpr | Handles precedence, types, errors |
| Boolean evaluation | `==` string matching | evalexpr | Supports `&&`, `||`, `!`, comparisons |
| Error handling | String errors | EvalexprError | Typed errors, good messages |
| Variable binding | Manual HashMap | HashMapContext | Type-safe, built-in |

**Key insight:** Expression parsing looks simple but has many edge cases (operator precedence, escaping, type coercion). evalexpr handles these correctly and is well-tested.

## Common Pitfalls

### Pitfall 1: Environment Variable Names with Special Characters
**What goes wrong:** `env.CI-CD` fails because `-` is an operator
**Why it happens:** evalexpr interprets `-` as subtraction
**How to avoid:** Convert special chars to underscores: `env_CI_CD` or use accessor function
**Warning signs:** Expression parses but gives wrong result or error

### Pitfall 2: String Comparison Without Quotes
**What goes wrong:** `env.CI == true` compares string to boolean
**Why it happens:** `true` without quotes is boolean literal
**How to avoid:** Document that env vars need quoted comparison: `env.CI == "true"`
**Warning signs:** Expression returns false when expected true

### Pitfall 3: Missing Variables Return Error
**What goes wrong:** `env.NONEXISTENT == "true"` fails with VariableIdentifierNotFound
**Why it happens:** evalexpr requires all referenced variables to exist
**How to avoid:** Pre-populate all referenced env vars, or use custom function `env("CI", "default")`
**Warning signs:** Rules fail in some environments but not others

### Pitfall 4: Expression Evaluated on Every Event
**What goes wrong:** Slow performance with many rules
**Why it happens:** Re-parsing expression string each time
**How to avoid:** Pre-compile to `Node` during config load
**Warning signs:** High CPU during heavy tool use

### Pitfall 5: evalexpr Uses Underscores in Identifiers
**What goes wrong:** `env.CI` fails because `.` is not valid in identifiers
**Why it happens:** evalexpr uses `_` not `.` in variable names
**How to avoid:** Transform `env.CI` to `env_CI` before evaluation, or document underscore syntax
**Warning signs:** Parse errors on valid-looking expressions

## Code Examples

Verified patterns for conditional rule activation:

### Adding enabled_when to Rule struct
```rust
// Source: models.rs, following existing Optional field pattern
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rule {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Condition expression that must be true for rule to be active
    /// Evaluated against context variables: env_*, tool_name, event_type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_when: Option<String>,

    pub matchers: Matchers,
    pub actions: Actions,
    // ... other fields
}
```

### Building Expression Context
```rust
// Source: Based on evalexpr docs and RuleZ Event structure
use evalexpr::{HashMapContext, Value, DefaultNumericTypes};

fn build_eval_context(event: &Event) -> HashMapContext<DefaultNumericTypes> {
    let mut ctx = HashMapContext::new();

    // Add environment variables with env_ prefix
    for (key, value) in std::env::vars() {
        let var_name = format!("env_{}", key);
        ctx.set_value(var_name.into(), Value::String(value)).ok();
    }

    // Add tool name
    if let Some(ref tool_name) = event.tool_name {
        ctx.set_value("tool_name".into(), Value::String(tool_name.clone())).ok();
    } else {
        ctx.set_value("tool_name".into(), Value::String("".to_string())).ok();
    }

    // Add event type
    ctx.set_value(
        "event_type".into(),
        Value::String(event.hook_event_name.to_string())
    ).ok();

    ctx
}
```

### Evaluating enabled_when
```rust
// Source: Based on evalexpr eval_boolean_with_context
use evalexpr::{eval_boolean_with_context, build_operator_tree};

fn is_rule_enabled(rule: &Rule, event: &Event) -> bool {
    match &rule.enabled_when {
        None => true, // No condition = always enabled
        Some(expr) => {
            let ctx = build_eval_context(event);
            match eval_boolean_with_context(expr, &ctx) {
                Ok(result) => result,
                Err(e) => {
                    tracing::warn!(
                        "enabled_when expression failed for rule '{}': {} - treating as disabled",
                        rule.name, e
                    );
                    false // Fail-closed: invalid expression disables rule
                }
            }
        }
    }
}
```

### Validating Expressions in Config
```rust
// Source: Based on config.rs validate() pattern
use evalexpr::build_operator_tree;

impl Config {
    pub fn validate(&self) -> Result<()> {
        // ... existing validation ...

        // Validate enabled_when expressions parse correctly
        for rule in &self.rules {
            if let Some(ref expr) = rule.enabled_when {
                build_operator_tree(expr).with_context(|| {
                    format!(
                        "Invalid enabled_when expression '{}' in rule '{}': syntax error",
                        expr, rule.name
                    )
                })?;
            }
        }

        Ok(())
    }
}
```

### Integration in evaluate_rules
```rust
// Source: hooks.rs evaluate_rules(), adding enabled_when check
async fn evaluate_rules<'a>(
    event: &'a Event,
    config: &'a Config,
    debug_config: &DebugConfig,
) -> Result<(Vec<&'a Rule>, Response, Vec<RuleEvaluation>)> {
    let mut matched_rules = Vec::new();
    let mut response = Response::allow();
    let mut rule_evaluations = Vec::new();

    for rule in config.enabled_rules() {
        // NEW: Check enabled_when before matchers
        if !is_rule_enabled(rule, event) {
            if debug_config.enabled {
                rule_evaluations.push(RuleEvaluation {
                    rule_name: rule.name.clone(),
                    matched: false,
                    matcher_results: None, // Skipped due to enabled_when
                });
            }
            continue;
        }

        // Existing matcher logic...
        let (matched, matcher_results) = if debug_config.enabled {
            matches_rule_with_debug(event, rule)
        } else {
            (matches_rule(event, rule), None)
        };
        // ... rest unchanged
    }

    Ok((matched_rules, response, rule_evaluations))
}
```

### YAML Configuration Example
```yaml
# Example hooks.yaml with enabled_when
version: "1.0"
rules:
  - name: ci-strict-mode
    description: "Only active in CI environments"
    enabled_when: 'env_CI == "true"'
    matchers:
      tools: [Bash]
      command_match: "git push"
    actions:
      block: true

  - name: dev-helper
    description: "Only for local development"
    enabled_when: 'env_CI != "true" && tool_name == "Bash"'
    matchers:
      tools: [Bash]
    actions:
      inject_inline: "Remember to run tests before pushing!"
```

### Unit Test Examples
```rust
#[test]
fn test_enabled_when_yaml_parsing() {
    let yaml = r#"
name: ci-only
enabled_when: 'env_CI == "true"'
matchers:
  tools: [Bash]
actions:
  block: true
"#;
    let rule: Rule = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(rule.enabled_when, Some(r#"env_CI == "true""#.to_string()));
}

#[test]
fn test_expression_evaluation_true() {
    let mut ctx = HashMapContext::<DefaultNumericTypes>::new();
    ctx.set_value("env_CI".into(), Value::String("true".to_string())).unwrap();

    let result = eval_boolean_with_context(r#"env_CI == "true""#, &ctx).unwrap();
    assert!(result);
}

#[test]
fn test_expression_evaluation_false() {
    let mut ctx = HashMapContext::<DefaultNumericTypes>::new();
    ctx.set_value("env_CI".into(), Value::String("false".to_string())).unwrap();

    let result = eval_boolean_with_context(r#"env_CI == "true""#, &ctx).unwrap();
    assert!(!result);
}

#[test]
fn test_expression_with_logical_operators() {
    let mut ctx = HashMapContext::<DefaultNumericTypes>::new();
    ctx.set_value("env_CI".into(), Value::String("true".to_string())).unwrap();
    ctx.set_value("tool_name".into(), Value::String("Bash".to_string())).unwrap();

    let result = eval_boolean_with_context(
        r#"env_CI == "true" && tool_name == "Bash""#,
        &ctx
    ).unwrap();
    assert!(result);
}

#[test]
fn test_invalid_expression_validation() {
    let result = build_operator_tree("env_CI ==");  // Missing right operand
    assert!(result.is_err());
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Static rule activation | Conditional `enabled_when` | This phase | Rules can adapt to environment |
| Manual CI detection | Expression evaluation | This phase | Declarative, user-configurable |

**Deprecated/outdated:**
- None - this is new functionality

## Open Questions

Things that couldn't be fully resolved:

1. **Expression Syntax: Dots vs Underscores**
   - What we know: evalexpr uses underscores in identifiers, not dots
   - What's unclear: Should we transform user-facing `env.CI` to internal `env_CI`?
   - Recommendation: Use underscore syntax directly (`env_CI`) for simplicity, document clearly

2. **Fail-Open vs Fail-Closed for Expression Errors**
   - What we know: Other RuleZ features use fail-open (allow on error)
   - What's unclear: Should invalid `enabled_when` disable rule (fail-closed) or skip condition (fail-open)?
   - Recommendation: Fail-closed (disable rule) - safer to not apply rule than apply incorrectly

3. **Performance: Pre-compile Expressions?**
   - What we know: `build_operator_tree` can pre-compile to `Node`
   - What's unclear: Is runtime overhead significant enough to warrant caching?
   - Recommendation: Start simple (eval each time), add caching if performance issues arise

4. **What Variables Should Be Available?**
   - What we know: Phase requirements specify `env.*`, `tool.name`, `event.type`
   - What's unclear: Should we expose more (session_id, cwd, etc.)?
   - Recommendation: Start with required variables, expand based on user feedback

## Sources

### Primary (HIGH confidence)
- [evalexpr GitHub](https://github.com/ISibboI/evalexpr) - Library source, examples
- [evalexpr docs.rs](https://docs.rs/evalexpr) - API documentation, v13.1.0
- [EvalexprError enum](https://docs.rs/evalexpr/latest/evalexpr/error/enum.EvalexprError.html) - Error types
- `rulez/src/models.rs` lines 213-246 - Existing Rule struct pattern
- `rulez/src/hooks.rs` lines 121-158 - Existing evaluate_rules pattern

### Secondary (MEDIUM confidence)
- [Rhai expressions docs](https://rhai.rs/book/engine/expressions.html) - Alternative library comparison
- [evalexpr Node docs](https://docs.rs/evalexpr/5.0.5/evalexpr/struct.Node.html) - Pre-compilation pattern

### Tertiary (LOW confidence)
- None - evalexpr is well-documented and actively maintained

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - evalexpr is established, well-documented, no deps
- Architecture: HIGH - Clear integration point before matches_rule()
- Pitfalls: MEDIUM - Based on evalexpr docs, some edge cases need testing

**Research date:** 2026-02-07
**Valid until:** 60 days (evalexpr is stable, no breaking changes expected)

---

## Implementation Checklist Summary

For the planner:

1. **Cargo.toml** - Add `evalexpr = "13.1"` dependency
2. **models.rs** - Add `enabled_when: Option<String>` to `Rule` struct
3. **hooks.rs** - Add `build_eval_context()` function
4. **hooks.rs** - Add `is_rule_enabled()` function
5. **hooks.rs** - Check `enabled_when` before `matches_rule()` in evaluate_rules loop
6. **config.rs** - Add expression validation in `validate()` method
7. **cli/validate.rs** - Report expression syntax errors
8. **Unit tests** - Test expression parsing, evaluation, error cases
9. **Integration tests** - Test full rule evaluation with enabled_when
