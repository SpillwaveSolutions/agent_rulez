# RuleZ Lint -- Rule Quality Analysis

Static analysis for your hooks configuration. Catch misconfigurations, dead rules, and quality issues before they cause runtime surprises.

## Overview

`rulez lint` performs static analysis on your `hooks.yaml` configuration file. It checks for common mistakes like duplicate rule names, rules with no matchers (which would match every event), conflicting actions, and other quality issues. Each diagnostic has a severity level -- ERRORs cause a non-zero exit code so you can gate deployments, while WARNINGs and INFOs are advisory. Think of it as ESLint for your RuleZ configuration.

## Prerequisites

- **RuleZ v2.2+** installed and on your PATH (`rulez --version`)
- A `hooks.yaml` configuration file (run `rulez init` to create one)

## Quick Start: Lint Your First Config

### Run lint on your project

From your project root (where `.claude/hooks.yaml` lives):

```bash
rulez lint
```

If your configuration is clean, you will see:

```
rulez lint -- Rule Quality Analysis
==================================

Loaded 5 rules from .claude/hooks.yaml

No issues found. Configuration looks good!
```

### Example output with issues

If your configuration has problems, lint reports each one:

```
rulez lint -- Rule Quality Analysis
==================================

Loaded 8 rules from .claude/hooks.yaml

[ERROR] duplicate-rule-name: Rules at positions 2 and 5 both have the name 'deny-rm-rf'
[WARN]  dead-rule: Rule 'old-audit-logger' is disabled (metadata.enabled: false) -- consider removing it
[WARN]  no-description: Rule 'block-force-push' has no description

Summary: 1 error, 2 warnings, 0 info
```

### Understanding severity levels

| Severity | Exit Code | Meaning |
|----------|-----------|---------|
| **ERROR** | 1 | Critical misconfiguration. Must fix before rules work correctly. |
| **WARNING** | 0 | Potential issue. Review and fix or acknowledge. |
| **INFO** | 0 | Suggestion for improvement. Only shown with `--verbose`. |

ERRORs cause `rulez lint` to exit with code 1, making it suitable for CI pipelines and pre-commit hooks.

## CLI Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--config <path>` | `-c` | Path to hooks.yaml file (default: `.claude/hooks.yaml`) |
| `--verbose` | `-v` | Show INFO-level diagnostics (e.g., `glob-consolidation`) |

**Exit codes:**
- `0` -- No errors found (warnings and info are allowed)
- `1` -- One or more ERROR-severity diagnostics found

**Examples:**

```bash
# Lint the default config
rulez lint

# Lint a specific file
rulez lint --config path/to/hooks.yaml

# Show all diagnostics including INFO
rulez lint --verbose
```

## Rule Reference

Each lint rule is documented below with its severity, what it detects, why it matters, and before/after examples.

---

### `duplicate-rule-name`

**Severity:** ERROR

**What It Detects:** Two or more rules with the same `name` field.

**Why It Matters:** Rule names are used for identification in logs, `rulez explain` output, and `rulez test` results. Duplicate names make it impossible to tell which rule fired and can cause unexpected evaluation behavior.

**Bad Example:**

```yaml
rules:
  - name: deny-rm-rf
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf"
    actions:
      block: true

  - name: deny-rm-rf          # Duplicate!
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf\\s+/"
    actions:
      block: true
```

**Fixed Example:**

```yaml
rules:
  - name: deny-rm-rf
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf"
    actions:
      block: true

  - name: deny-rm-rf-root     # Unique name
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf\\s+/"
    actions:
      block: true
```

---

### `no-matchers`

**Severity:** ERROR

**What It Detects:** A rule with no matcher fields defined (no `tools`, `extensions`, `directories`, `operations`, `command_match`, `prompt_match`, `require_fields`, or `field_types`).

**Why It Matters:** A rule with no matchers matches every event. This is almost never intentional and will either block all operations or inject context into every response, causing significant disruption.

**Bad Example:**

```yaml
rules:
  - name: audit-everything
    description: "Log all events"
    matchers: {}               # No matchers -- matches ALL events!
    actions:
      run: "logger 'event fired'"
```

**Fixed Example:**

```yaml
rules:
  - name: audit-tool-executions
    description: "Log tool execution events"
    matchers:
      operations: [tool_execute]  # Only match tool executions
    actions:
      run: "logger 'tool executed'"
```

---

### `conflicting-actions`

**Severity:** ERROR

**What It Detects:** A rule that has both `block: true` and an inject action (`inject`, `inject_inline`, or `inject_command`).

**Why It Matters:** When a rule blocks an operation, the tool invocation is denied entirely. Injecting context into a blocked operation has no effect -- the context is discarded because the operation never proceeds. This indicates a logic error in the rule definition.

**Bad Example:**

```yaml
rules:
  - name: block-and-inject
    description: "Block dangerous ops and inject warning"
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf"
    actions:
      block: true
      inject_inline: "This is dangerous!"   # Conflict!
```

**Fixed Example (block):**

```yaml
rules:
  - name: deny-rm-rf
    description: "Block dangerous rm -rf commands"
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf"
    actions:
      block: true
      message: "Blocked: rm -rf commands are not allowed"
```

**Fixed Example (inject):**

```yaml
rules:
  - name: warn-rm-rf
    description: "Inject warning for rm -rf commands"
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf"
    actions:
      inject_inline: "WARNING: rm -rf is dangerous. Use with caution."
```

---

### `overlapping-rules`

**Severity:** WARNING

**What It Detects:** Two enabled rules with the same `operations`, `tools`, and `command_match` matchers.

**Why It Matters:** Overlapping rules can produce unexpected interactions -- both rules fire on the same event, which may cause conflicting actions or duplicate log entries. If the overlap is intentional, differentiate the rules by adding distinct matchers (e.g., different `extensions` or `directories`).

**Bad Example:**

```yaml
rules:
  - name: deny-rm-rf-v1
    matchers:
      operations: [tool_execute]
      tools: [Bash]
      command_match: "rm\\s+-rf"
    actions:
      block: true

  - name: deny-rm-rf-v2       # Overlaps with v1!
    matchers:
      operations: [tool_execute]
      tools: [Bash]
      command_match: "rm\\s+-rf"
    actions:
      block: true
      message: "Blocked for safety"
```

**Fixed Example:**

```yaml
rules:
  - name: deny-rm-rf
    description: "Block all rm -rf commands"
    matchers:
      operations: [tool_execute]
      tools: [Bash]
      command_match: "rm\\s+-rf"
    actions:
      block: true
      message: "Blocked for safety"
```

---

### `dead-rule`

**Severity:** WARNING

**What It Detects:** A rule with `metadata.enabled: false`.

**Why It Matters:** Disabled rules add clutter to your configuration without providing any value. If you disabled a rule temporarily, consider re-enabling it or removing it entirely to keep your config clean and maintainable.

**Bad Example:**

```yaml
rules:
  - name: old-audit-logger
    description: "Legacy audit logging"
    matchers:
      operations: [tool_execute]
    actions:
      run: "logger 'tool executed'"
    metadata:
      enabled: false           # Dead rule!
```

**Fixed Example:**

```yaml
# Either remove the rule entirely, or re-enable it:
rules:
  - name: audit-logger
    description: "Audit logging for tool executions"
    matchers:
      operations: [tool_execute]
    actions:
      run: "logger 'tool executed'"
    metadata:
      enabled: true
```

---

### `no-description`

**Severity:** WARNING

**What It Detects:** A rule with no `description` field (or an empty description).

**Why It Matters:** Descriptions appear in `rulez explain` output and audit logs. Without a description, it is harder to understand why a rule exists, making maintenance and debugging more difficult over time.

**Bad Example:**

```yaml
rules:
  - name: block-force-push
    matchers:                   # No description!
      tools: [Bash]
      command_match: "git\\s+push.*--force"
    actions:
      block: true
```

**Fixed Example:**

```yaml
rules:
  - name: block-force-push
    description: "Prevent force-pushing to any remote branch"
    matchers:
      tools: [Bash]
      command_match: "git\\s+push.*--force"
    actions:
      block: true
```

---

### `invalid-regex`

**Severity:** WARNING

**What It Detects:** A `command_match` field containing an invalid regular expression.

**Why It Matters:** If the regex cannot be compiled, the rule's `command_match` matcher will never match any event, effectively making the rule useless. This is usually a typo or a missing escape character.

**Bad Example:**

```yaml
rules:
  - name: deny-dangerous-commands
    description: "Block dangerous shell commands"
    matchers:
      tools: [Bash]
      command_match: "rm -rf [("     # Invalid regex: unclosed bracket
    actions:
      block: true
```

**Fixed Example:**

```yaml
rules:
  - name: deny-dangerous-commands
    description: "Block dangerous shell commands"
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf"     # Valid regex
    actions:
      block: true
```

---

### `glob-consolidation`

**Severity:** INFO (only shown with `--verbose`)

**What It Detects:** Multiple rules with the same action but different `extensions` matchers.

**Why It Matters:** Instead of having separate rules for `.py`, `.js`, and `.ts` files that all inject the same coding standards file, you can consolidate them into a single rule with multiple extensions. This reduces config size and makes maintenance easier.

**Bad Example:**

```yaml
rules:
  - name: inject-standards-py
    matchers:
      extensions: [py]
    actions:
      inject: ".claude/context/coding-standards.md"

  - name: inject-standards-js
    matchers:
      extensions: [js]
    actions:
      inject: ".claude/context/coding-standards.md"
```

**Fixed Example:**

```yaml
rules:
  - name: inject-coding-standards
    description: "Inject coding standards for Python and JavaScript files"
    matchers:
      extensions: [py, js]
    actions:
      inject: ".claude/context/coding-standards.md"
```

---

### `missing-priority`

**Severity:** INFO

**What It Detects:** A rule with no explicit `priority` field (defaults to 0).

**Why It Matters:** When multiple rules match the same event, priority determines evaluation order. If all rules use the default priority of 0, evaluation order is based on position in the file. Setting explicit priorities makes the evaluation order deterministic and easier to reason about, especially as your configuration grows.

**Bad Example:**

```yaml
rules:
  - name: inject-python-standards
    matchers:
      extensions: [py]
    actions:
      inject: ".claude/context/python-standards.md"
    # No priority -- defaults to 0
```

**Fixed Example:**

```yaml
rules:
  - name: inject-python-standards
    matchers:
      extensions: [py]
    actions:
      inject: ".claude/context/python-standards.md"
    priority: 10
```

---

## Full Example

Here is a complete hooks.yaml with multiple lint issues, followed by the lint output and the corrected version.

### Configuration with issues

```yaml
rules:
  - name: deny-rm-rf
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf"
    actions:
      block: true

  - name: deny-rm-rf
    description: "Block rm -rf on root"
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf\\s+/"
    actions:
      block: true

  - name: block-and-inject
    matchers:
      tools: [Write]
      extensions: [env, pem, key]
    actions:
      block: true
      inject_inline: "Do not write secrets"

  - name: catch-all
    matchers: {}
    actions:
      run: "echo event"

  - name: old-policy
    matchers:
      tools: [Bash]
    actions:
      block: true
    metadata:
      enabled: false

  - name: audit-writes
    matchers:
      tools: [Write]
    actions:
      run: "logger 'file written'"
```

### Lint output

```bash
$ rulez lint
rulez lint -- Rule Quality Analysis
==================================

Loaded 6 rules from .claude/hooks.yaml

[ERROR] duplicate-rule-name: Rules at positions 1 and 2 both have the name 'deny-rm-rf'
[ERROR] conflicting-actions: Rule 'block-and-inject' has both block and inject actions -- blocked operations cannot inject context
[ERROR] no-matchers: Rule 'catch-all' has no matchers -- it will match all events
[WARN]  dead-rule: Rule 'old-policy' is disabled (metadata.enabled: false) -- consider removing it
[WARN]  no-description: Rule 'deny-rm-rf' has no description
[WARN]  no-description: Rule 'audit-writes' has no description

Summary: 3 errors, 3 warnings, 0 info
```

### Corrected configuration

```yaml
rules:
  - name: deny-rm-rf
    description: "Block rm -rf commands"
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf"
    actions:
      block: true
      message: "Blocked: rm -rf commands are not allowed"
    priority: 100

  - name: deny-rm-rf-root
    description: "Block rm -rf specifically targeting root"
    matchers:
      tools: [Bash]
      command_match: "rm\\s+-rf\\s+/"
    actions:
      block: true
      message: "Blocked: rm -rf / is never allowed"
    priority: 101

  - name: block-secret-writes
    description: "Prevent writing secret files"
    matchers:
      tools: [Write]
      extensions: [env, pem, key]
    actions:
      block: true
      message: "Blocked: cannot write secret files"

  - name: audit-writes
    description: "Log file write events"
    matchers:
      tools: [Write]
    actions:
      run: "logger 'file written'"
    priority: 1
```

## Troubleshooting

### "Failed to load configuration for linting"

The configuration file could not be found or parsed.

- Check that the file exists: `ls .claude/hooks.yaml`
- Validate YAML syntax: `rulez validate`
- Specify the correct path: `rulez lint --config path/to/hooks.yaml`

### Many overlapping-rules warnings

If you intentionally have rules that match similar events (e.g., one blocks and one logs), differentiate their matchers to silence the warning. Add a `command_match`, `extensions`, or `directories` matcher that makes them distinct.

### glob-consolidation diagnostics not showing

The `glob-consolidation` rule has INFO severity and is only shown with the `--verbose` flag:

```bash
rulez lint --verbose
```

### Lint reports errors but config works fine at runtime

Lint performs static analysis and may flag issues that do not manifest at runtime in your specific workflow. However, fixing lint errors is strongly recommended -- the issues it detects (like `no-matchers` matching all events) can cause subtle problems that are hard to debug.

## Further Reading

- [CLI Commands Reference](../../mastering-hooks/references/cli-commands.md) -- Full `rulez lint` flag reference
- [Hooks YAML Schema](../../mastering-hooks/references/hooks-yaml-schema.md) -- Complete rule syntax and field definitions
- [Rule Patterns](../../mastering-hooks/references/rule-patterns.md) -- Common rule patterns and best practices
