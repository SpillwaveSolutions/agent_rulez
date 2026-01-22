# Claude Context Hooks (CCH)

## Command Line User Guide

The **CCH CLI** (`cch`) is the **deterministic runtime engine** for Claude Context Hooks. It is a high-performance Rust binary that evaluates hook events, matches rules, executes validators, injects context, and enforces policy.

> Think of it this way:
>
> * **CCH Skill** → *Policy author* (intelligent assistant)
> * **CCH CLI (`cch`)** → *Policy engine* (deterministic runtime)

The Skill helps you *generate* rules.
The CLI is what actually *runs them*.

---

## 1. Installation & Verification

Before using CCH, ensure the binary is available in your system.

### Check Version

```bash
cch --version
```

**Expected output:**

```
cch 1.0.0
```

For machine-readable metadata:

```bash
cch --version --json
```

Example:

```json
{
  "name": "cch",
  "version": "1.0.0",
  "api_version": 1,
  "config_schema_version": 1,
  "log_schema_version": 1
}
```

This is what the **CCH Skill** uses to verify compatibility.

---

### Installation Methods

If `cch` is not found:

#### Option 1 — Pre-built Binary (recommended)

Download from GitHub Releases for:

* macOS (Intel / Apple Silicon)
* Linux (x86_64 / ARM64)
* Windows (x86_64)

Place it in your PATH.

#### Option 2 — Cargo (developers)

```bash
cargo install --git https://github.com/.../cch
```

#### Option 3 — Via CCH Skill

Just ask Claude:

> *“Install CCH for this project.”*

The skill will:

* detect your OS/arch
* download the correct binary
* verify installation

---

## 2. Project Setup

These commands initialize and register CCH in a repository.

### Initialize Configuration

Scaffold a new configuration:

```bash
cch init
```

This creates:

```
.claude/
  hooks.yaml
  context/
  validators/
```

Including example rules and templates.

> This does **not** activate anything yet.
> It only creates files.

---

### Register with Claude Code

To activate CCH, it must be registered as a hook handler.

```bash
cch install --project
```

This modifies:

```
.claude/settings.json
```

And registers `cch` for lifecycle events.

#### Installation Modes

| Flag        | Scope                              |
| ----------- | ---------------------------------- |
| `--project` | Project only (recommended)         |
| `--user`    | Global (`~/.claude/settings.json`) |
| `--dry-run` | Preview changes                    |
| `--force`   | Overwrite existing hooks           |

---

## 3. Configuration Management

These commands operate on your **policy configuration**.

### Validate Configuration

```bash
cch validate
```

Checks:

* YAML syntax
* Schema validity
* Referenced files exist
* Regex/glob correctness
* Validator scripts exist

Exit codes:

* `0` → valid
* non-zero → errors found

Strict mode:

```bash
cch validate --strict
```

Treats warnings as errors.

---

### View Effective Configuration

```bash
cch config --show
```

Shows the **fully resolved configuration**, including:

* project vs user precedence
* default values
* normalized paths

This is extremely useful for debugging inheritance.

---

## 4. Observability & Policy Debugging

CCH is designed to be **explainable and auditable**.

### Explain a Rule

```bash
cch explain rule <rule-name>
```

Example:

```bash
cch explain rule no-console-log
```

Outputs:

* matchers
* actions
* mode (enforce / warn / audit)
* priority
* provenance metadata
* recent trigger stats

This is your primary **policy introspection tool**.

---

### Query Logs

All runtime decisions are logged to:

```
~/.claude/logs/cch.log
```

Query them:

```bash
cch logs
```

Logs are JSON Lines:

```json
{
  "timestamp": "2025-01-21T14:32:11Z",
  "event": "PreToolUse",
  "rule_name": "block-force-push",
  "mode": "enforce",
  "decision": "blocked",
  "tool": "Bash"
}
```

This enables:

* root cause analysis
* audit trails
* policy analytics
* compliance evidence

---

### Debug Mode

Verbose tracing:

```bash
cch debug pre-tool-use
```

Used with simulated events:

```bash
echo '{"tool_name":"Bash","tool_input":{"command":"git push --force"}}' | cch debug pre-tool-use
```

Outputs:

* which rules were evaluated
* why matchers passed/failed
* execution timing
* final decision

This is essentially **`--trace` for policy**.

---

## 5. Event Handlers (Advanced / Internal)

These commands are normally called by Claude Code itself.

They read **JSON from stdin** and return **JSON to stdout**.

### General Form

```bash
cat event.json | cch <event-command>
```

### Supported Event Commands

| Command         | Lifecycle Event           |
| --------------- | ------------------------- |
| `pre-tool-use`  | Before tool executes      |
| `post-tool-use` | After tool completes      |
| `permission`    | Permission request        |
| `user-prompt`   | User submits message      |
| `session-start` | Session begins            |
| `session-end`   | Session ends              |
| `pre-compact`   | Before context compaction |

---

### Example: Manual Testing

Create test input:

```json
{
  "tool_name": "Bash",
  "tool_input": {
    "command": "git push --force"
  }
}
```

Run:

```bash
cat test_event.json | cch pre-tool-use
```

Output:

```json
{
  "continue": false,
  "reason": "🚫 Force push is not allowed."
}
```

This is exactly what Claude Code consumes internally.

---

## 6. Uninstallation

Remove CCH from a project:

```bash
cch uninstall --project
```

Remove from global scope:

```bash
cch uninstall --user
```

This:

* removes only CCH hooks
* preserves other Claude hooks
* does not delete your config files

---

## 7. Exit Codes (Scripting & Automation)

When integrating CCH into scripts or CI:

| Code | Meaning                 |
| ---- | ----------------------- |
| 0    | Success                 |
| 1    | General error           |
| 2    | Configuration error     |
| 3    | Validation failure      |
| 4    | Validator script failed |
| 127  | Command not found       |

---

## 8. Mental Model (Important)

### What runs when?

| Component  | When it runs            |
| ---------- | ----------------------- |
| CCH Skill  | On demand, during setup |
| CCH CLI    | On *every* hook event   |
| hooks.yaml | Loaded every event      |
| validators | Executed conditionally  |
| context md | Injected dynamically    |

### Architectural Truth

> **CCH is a local AI policy engine.**
> Claude is subject to it.
> The Skill merely authors policies.

This is why CCH is:

* deterministic
* testable
* auditable
* safe for real governance

---

## One-Line Summary

The CCH CLI is not just a helper tool.

It is:

> **A deterministic, local-first policy enforcement engine for AI agents.**
