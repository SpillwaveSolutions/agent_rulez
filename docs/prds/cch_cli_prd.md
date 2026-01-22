# Claude Context Hooks (CCH)

## Product Requirements Document & Implementation Plan

**Version:** 1.0
**Last Updated:** January 21, 2025

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [User Stories](#2-user-stories)
3. [Functional Requirements](#3-functional-requirements)
4. [Non-Functional Requirements](#4-non-functional-requirements)
5. [Technical Architecture](#5-technical-architecture)
6. [CLI Interface](#6-cli-interface)
7. [Configuration Reference](#7-configuration-reference)
8. [Script Validators](#8-script-validators)
9. [Claude Code Integration](#9-claude-code-integration)
10. [Example Configurations](#10-example-configurations)
11. [Example Markdown Templates](#11-example-markdown-templates)
12. [Example Validator Scripts](#12-example-validator-scripts)
13. [Implementation Plan](#13-implementation-plan)
14. [Testing Strategy](#14-testing-strategy)
15. [Distribution](#15-distribution)
16. [Success Metrics](#16-success-metrics)
17. [Future Enhancements](#17-future-enhancements)
18. [Open Questions](#18-open-questions)
19. [Appendix](#19-appendix)

---

## 1. Executive Summary

### 1.1 Product Name

**Claude Context Hooks** (CLI binary: `cch`)

### 1.2 Problem Statement

Claude Code's hook system requires writing raw JSON configuration and shell commands. This creates several pain points:

- **Poor readability:** JSON with escaped commands is hard to read and maintain
- **No conditional logic:** Can't easily inject context based on file types, directories, or operations
- **No reusability:** Hook patterns can't be easily shared across projects
- **No guardrails:** Difficult to enforce agent roles or block dangerous operations
- **Opaque commands:** Claude asks permission for complex commands without explaining them

### 1.3 Solution

A Rust-based CLI tool that provides:

- **Human-friendly YAML configuration** organized by hook event
- **Flexible rule matching** based on extensions, directories, tools, commands, and patterns
- **Markdown-based context injection** for readable, maintainable prompts
- **External script validators** for custom validation logic in any language
- **Built-in blocking and guards** for dangerous operations
- **Structured explanations** forcing Claude to explain commands before asking permission
- **Direct installation** into Claude Code's settings (no manual JSON editing)

### 1.4 Target Users

| User Type | Use Case |
|-----------|----------|
| Individual developers | Personal productivity, command explanations |
| Team leads | Enforce coding standards via AI agents |
| Platform engineers | Multi-agent workflows (architect + developer) |
| Security-conscious users | Block dangerous operations, audit commands |

### 1.5 Key Benefits

| Benefit | Description |
|---------|-------------|
| **Readable configs** | YAML with comments vs cryptic JSON |
| **Maintainable** | Markdown files for context, scripts for logic |
| **Shareable** | Copy configs between projects or publish |
| **Safe** | Block dangerous operations, require explanations |
| **Fast** | Rust binary with <10ms execution time |
| **Extensible** | Custom validators in Python, TypeScript, Bash |

---

## 2. User Stories

### 2.1 Skill Enforcement

> As a developer, I want Claude to automatically reference the CDK skill documentation when editing infrastructure code, so it follows our established patterns.

**Acceptance Criteria:**
- When Claude edits `.ts` files in `cdk/` or `infra/` directories, CDK guidelines are injected
- Guidelines come from a markdown file I can easily edit
- Multiple skills can apply to the same file (additive)

### 2.2 Agent Role Guardrails

> As a team lead, I want to restrict the architect agent to only edit markdown files, so it doesn't accidentally modify production code.

**Acceptance Criteria:**
- When `CLAUDE_ROLE=architect`, code file edits are blocked
- Clear error message explains why and what to do instead
- Markdown, YAML, and JSON files are allowed
- Role is determined by environment variable

### 2.3 Git Commit Guards

> As a developer, I want to block commits containing "WIP", "fixup", or merge conflict markers, and remind Claude of our commit message conventions.

**Acceptance Criteria:**
- Commits with forbidden patterns are blocked with clear message
- Commit guidelines are injected before any commit
- Force push is blocked entirely
- Custom patterns can be added via config

### 2.4 Command Explanations

> As a user, I want Claude to explain what a bash command does in plain English before asking me to approve it, especially for complex commands with environment variables.

**Acceptance Criteria:**
- All bash commands get a plain-English explanation
- Complex commands are broken down part by part
- Destructive commands require structured explanation (what, why, risk, undo)
- Explanation appears BEFORE the yes/no prompt

### 2.5 Post-Edit Reminders

> As a developer, I want Claude to be reminded to run linters and tests after editing code files.

**Acceptance Criteria:**
- After editing `.py`, `.ts`, `.js` files, lint reminder is injected
- After editing files in `src/`, test reminder is injected
- Reminders are unobtrusive suggestions, not blocks

### 2.6 Project Context Loading

> As a developer, I want project-specific context loaded automatically when I start a Claude Code session.

**Acceptance Criteria:**
- Project overview loaded on session start
- Different contexts for different projects (per-project config)
- Can include architecture decisions, conventions, team info

### 2.7 Custom Validation

> As a platform engineer, I want to run custom validation scripts before certain operations, blocking them if validation fails.

**Acceptance Criteria:**
- Can specify `.py`, `.ts`, `.js`, or `.sh` scripts as validators
- Script receives event data via stdin
- Exit 0 = allow, exit non-zero = block
- stderr becomes the block message
- Scripts can inject additional context via stdout

---

## 3. Functional Requirements

### 3.1 Configuration Format

Configuration is YAML, organized by hook event type.

**File locations (searched in order):**
1. `$CLAUDE_PROJECT_DIR/.claude/hooks.yaml`
2. `~/.claude/hooks.yaml`
3. Path specified via `--config` flag

**Basic structure:**

```yaml
version: "1"

SessionStart:
  - name: rule-name
    # ... matchers and actions

PreToolUse:
  - name: rule-name
    # ... matchers and actions

PostToolUse:
  - name: rule-name
    # ... matchers and actions

# ... other events
```

### 3.2 Supported Events

| Event | When It Fires | Available Actions |
|-------|---------------|-------------------|
| `SessionStart` | Claude Code launches or resumes | inject, run |
| `SessionEnd` | User exits Claude Code | inject, run |
| `Setup` | `claude --init` or `--maintenance` | inject, run |
| `UserPromptSubmit` | User sends message (before processing) | inject, run |
| `PreToolUse` | Before tool executes | inject, run, block, block_if_match |
| `PostToolUse` | After tool completes | inject, run |
| `PermissionRequest` | Claude asks for permission (yes/no) | inject, run, require_fields |
| `Notification` | Claude sends notification | inject, run |
| `Stop` | Claude finishes responding | inject, run |
| `SubagentStop` | Subagent task completes | inject, run |
| `PreCompact` | Before context compaction | inject, run |

### 3.3 Rule Matching

Rules are evaluated **top-to-bottom** within each event. Matching is **additive** — all matching rules apply their actions.

**Matcher fields:**

| Matcher | Type | Description | Events |
|---------|------|-------------|--------|
| `tools` | `string[]` | Tool names. Supports `*` wildcard. | PreToolUse, PostToolUse, PermissionRequest |
| `extensions` | `string[]` | File extensions (e.g., `.py`, `ts`). Auto-normalizes. | PreToolUse, PostToolUse, PermissionRequest |
| `directories` | `string[]` | Glob patterns (e.g., `src/**`, `**/test/**`) | PreToolUse, PostToolUse, PermissionRequest |
| `operations` | `string[]` | Operation types: `create`, `edit`, `read`, `delete` | PreToolUse, PostToolUse, PermissionRequest |
| `command_match` | `string` | Regex for Bash commands | PreToolUse, PostToolUse, PermissionRequest |
| `prompt_match` | `string` | Regex for user prompts | UserPromptSubmit |
| `enabled_when` | `object` | Conditional on environment variable | All events |

**Matcher logic:**
- Unspecified matchers match everything (implicit wildcard)
- All specified matchers must match (AND logic)
- Within an array, any item matching is sufficient (OR logic)

**Examples:**

```yaml
# Matches Edit or Write tool on .ts or .tsx files in src/ directory
- tools: [Edit, Write]
  extensions: [.ts, .tsx]
  directories: [src/**]

# Matches any Bash command containing "git commit" or "git push"
- tools: [Bash]
  command_match: "git (commit|push)"

# Matches only when CLAUDE_ROLE=architect
- enabled_when:
    env: CLAUDE_ROLE
    equals: architect
```

### 3.4 Actions

| Action | Type | Description | Events |
|--------|------|-------------|--------|
| `inject` | `string` | Path to markdown file to inject into context | All |
| `run` | `string` or `object` | Path to validator script | All |
| `action` | `"block"` | Unconditionally block the operation | PreToolUse, PermissionRequest |
| `message` | `string` | Custom message when blocking | PreToolUse, PermissionRequest |
| `block_if_match` | `array` | Conditional blocking based on patterns | PreToolUse, PermissionRequest |
| `require_fields` | `array` | Require structured explanation | PermissionRequest |

**Execution order within a rule:**
1. `inject` — always runs if rule matches
2. `run` — executes validator script
3. `block_if_match` — checks patterns
4. `action: block` — unconditional block

**Multiple rules:**
- All matching rules execute in order
- All injections concatenate
- Any block = overall block (first block message wins)

### 3.5 Conditional Execution

Rules can be conditionally enabled based on environment variables:

```yaml
- name: architect-only-rule
  enabled_when:
    env: CLAUDE_ROLE
    equals: architect
  # ... rest of rule

- name: production-warning
  enabled_when:
    env: ENVIRONMENT
    matches: "prod|production"  # regex
  # ... rest of rule

- name: ci-mode
  enabled_when:
    env: CI
    exists: true  # just check if set
  # ... rest of rule
```

### 3.6 Output Format

CCH outputs JSON that Claude Code hooks consume:

**Continue (with optional context injection):**
```json
{
  "continue": true,
  "context": "# Injected Context\n\nThis is additional context..."
}
```

**Block:**
```json
{
  "continue": false,
  "reason": "🚫 Operation blocked: reason here"
}
```

**Empty response (no matching rules):**
```json
{
  "continue": true
}
```

---

## 4. Non-Functional Requirements

### 4.1 Performance

| Metric | Target |
|--------|--------|
| Cold start | < 5ms |
| Config parsing (10KB YAML) | < 2ms |
| Rule matching (50 rules) | < 1ms |
| Script execution overhead | < 5ms |
| Total execution (no scripts) | < 10ms |
| Total execution (with scripts) | < script time + 10ms |

### 4.2 Reliability

- Graceful handling of missing config files (use defaults)
- Graceful handling of missing markdown files (warn to stderr, continue)
- Graceful handling of script errors (treat as block, report error)
- Valid JSON output even on internal errors
- No panics in release builds
- Script timeout protection (default 30s)

### 4.3 Compatibility

| Platform | Architecture | Support |
|----------|--------------|---------|
| macOS | Intel (x86_64) | ✅ Full |
| macOS | Apple Silicon (aarch64) | ✅ Full |
| Linux | x86_64 | ✅ Full |
| Linux | ARM64 | ✅ Full |
| Windows | x86_64 | ✅ Full |

**Runtime dependencies:**
- None for core functionality
- Python 3 for `.py` validators
- Bun for `.ts` and `.js` validators
- Bash for `.sh` validators

### 4.4 Security

- Scripts run with user's permissions (no elevation)
- Scripts receive only event data (no secrets)
- Config files are user-owned and user-readable only
- No network access from CCH itself
- No arbitrary code execution except explicitly configured scripts

### 4.5 Usability

- Clear error messages with file:line references
- `--validate` flag for config checking
- `--debug` flag for troubleshooting
- `--init` to scaffold example configuration
- Colored terminal output
- Helpful `--help` text with examples

---

## 5. Technical Architecture

### 5.1 System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        Claude Code                               │
│                                                                  │
│  ~/.claude/settings.json (or .claude/settings.json)             │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │ "hooks": {                                                 │ │
│  │   "PreToolUse": [{                                         │ │
│  │     "matcher": "*",                                        │ │
│  │     "hooks": [{ "type": "command", "command": "cch ..." }] │ │
│  │   }],                                                      │ │
│  │   ...                                                      │ │
│  │ }                                                          │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ Invokes on hook event
                              │ stdin: JSON event data
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                         cch (Rust)                               │
│                                                                  │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌──────────────┐ │
│  │    CLI    │─▶│  Config   │─▶│  Matcher  │─▶│   Actions    │ │
│  │  Parser   │  │  Loader   │  │  Engine   │  │   Executor   │ │
│  └───────────┘  └───────────┘  └───────────┘  └──────────────┘ │
│                       │                              │          │
│                       ▼                              ▼          │
│               .claude/hooks.yaml              .claude/context/  │
│                                               .claude/validators│
└─────────────────────────────────────────────────────────────────┘
                              │
                              │ stdout: JSON response
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Claude Code                               │
│                  (processes response, continues)                 │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Component Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                           cch                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────┐       │
│  │     CLI     │     │   Config    │     │   Events    │       │
│  │             │     │             │     │             │       │
│  │ - Clap args │     │ - Load YAML │     │ - Parse JSON│       │
│  │ - Subcommands│    │ - Resolve   │     │ - Event types│      │
│  │ - Help text │     │   paths     │     │ - Validation│       │
│  └──────┬──────┘     │ - Validate  │     └──────┬──────┘       │
│         │            └──────┬──────┘            │              │
│         │                   │                   │              │
│         ▼                   ▼                   ▼              │
│  ┌─────────────────────────────────────────────────────┐       │
│  │                    Matcher Engine                    │       │
│  │                                                      │       │
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐         │       │
│  │  │   Tools   │ │ Extensions│ │Directories│         │       │
│  │  │  Matcher  │ │  Matcher  │ │  Matcher  │         │       │
│  │  └───────────┘ └───────────┘ └───────────┘         │       │
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐         │       │
│  │  │ Commands  │ │ Prompts   │ │Conditions │         │       │
│  │  │  Matcher  │ │  Matcher  │ │  Matcher  │         │       │
│  │  └───────────┘ └───────────┘ └───────────┘         │       │
│  └─────────────────────────┬───────────────────────────┘       │
│                            │                                    │
│                            ▼                                    │
│  ┌─────────────────────────────────────────────────────┐       │
│  │                   Action Executor                    │       │
│  │                                                      │       │
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐         │       │
│  │  │  Inject   │ │   Block   │ │   Run     │         │       │
│  │  │  (load MD)│ │  (check)  │ │ (scripts) │         │       │
│  │  └───────────┘ └───────────┘ └───────────┘         │       │
│  │  ┌───────────┐                                      │       │
│  │  │  Require  │                                      │       │
│  │  │  Fields   │                                      │       │
│  │  └───────────┘                                      │       │
│  └─────────────────────────┬───────────────────────────┘       │
│                            │                                    │
│                            ▼                                    │
│  ┌─────────────────────────────────────────────────────┐       │
│  │                   Output Builder                     │       │
│  │                                                      │       │
│  │  - Aggregate injections                             │       │
│  │  - Determine continue/block                         │       │
│  │  - Format JSON response                             │       │
│  └─────────────────────────────────────────────────────┘       │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 5.3 Module Structure

```
cch/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── .github/
│   └── workflows/
│       └── release.yml          # cargo-dist release workflow
├── src/
│   ├── main.rs                  # Entry point
│   ├── lib.rs                   # Library exports
│   ├── cli/
│   │   ├── mod.rs               # CLI module exports
│   │   ├── args.rs              # Clap argument definitions
│   │   ├── commands/
│   │   │   ├── mod.rs           # Command module exports
│   │   │   ├── events.rs        # Event handler commands
│   │   │   ├── init.rs          # cch init
│   │   │   ├── validate.rs      # cch validate
│   │   │   ├── install.rs       # cch install
│   │   │   └── debug.rs         # cch debug
│   │   └── output.rs            # Terminal output helpers
│   ├── config/
│   │   ├── mod.rs               # Config module exports
│   │   ├── loader.rs            # YAML loading, path resolution
│   │   ├── types.rs             # Config struct definitions
│   │   └── validation.rs        # Config validation
│   ├── events/
│   │   ├── mod.rs               # Events module exports
│   │   ├── types.rs             # Event enum and structs
│   │   └── parser.rs            # JSON stdin parsing
│   ├── matcher/
│   │   ├── mod.rs               # Matcher module exports
│   │   ├── engine.rs            # Main matching logic
│   │   ├── tools.rs             # Tool name matching
│   │   ├── extensions.rs        # File extension matching
│   │   ├── directories.rs       # Glob pattern matching
│   │   ├── operations.rs        # Operation type detection
│   │   ├── commands.rs          # Regex command matching
│   │   ├── prompts.rs           # Regex prompt matching
│   │   └── conditions.rs        # enabled_when evaluation
│   ├── actions/
│   │   ├── mod.rs               # Actions module exports
│   │   ├── executor.rs          # Action execution orchestration
│   │   ├── inject.rs            # Markdown loading
│   │   ├── block.rs             # Blocking logic
│   │   ├── block_if_match.rs    # Pattern-based blocking
│   │   ├── run.rs               # Script execution
│   │   └── require_fields.rs    # Structured explanations
│   ├── output/
│   │   ├── mod.rs               # Output module exports
│   │   └── response.rs          # JSON response building
│   └── utils/
│       ├── mod.rs               # Utils module exports
│       ├── paths.rs             # Path resolution helpers
│       └── errors.rs            # Error types
└── tests/
    ├── integration/
    │   ├── mod.rs
    │   ├── pre_tool_use.rs
    │   ├── post_tool_use.rs
    │   ├── permission_request.rs
    │   └── scripts.rs
    └── fixtures/
        ├── configs/
        │   ├── basic.yaml
        │   ├── complex.yaml
        │   └── invalid.yaml
        ├── context/
        │   └── test.md
        └── validators/
            ├── pass.sh
            └── fail.py
```

### 5.4 Data Flow

**Event handling flow:**

```
1. Claude Code triggers hook event
2. Invokes: cch pre-tool-use (or other subcommand)
3. Event JSON piped to cch stdin
4. CLI parses subcommand → determines event type
5. Config loader:
   a. Finds hooks.yaml (project → user → default)
   b. Parses YAML into config structs
   c. Validates configuration
6. Event parser:
   a. Reads JSON from stdin
   b. Parses into event struct
   c. Extracts tool_name, tool_input, file_path, etc.
7. Matcher engine:
   a. Gets rules for current event type
   b. For each rule, evaluates all matchers
   c. Collects list of matching rules
8. Action executor (for each matching rule, in order):
   a. If `inject`: load markdown file, append to context
   b. If `run`: execute script, check exit code
      - Exit 0: continue, capture stdout as additional context
      - Exit non-zero: block, capture stderr as message
   c. If `block_if_match`: check patterns against command/input
      - Match found: block with configured message
   d. If `action: block`: set blocked flag with message
9. Output builder:
   a. Aggregate all injected context
   b. Determine overall continue/block status
   c. Build JSON response
10. Response written to stdout
11. Claude Code processes response
```

---

## 6. CLI Interface

### 6.1 Command Overview

```
cch - Claude Context Hooks

USAGE:
    cch <COMMAND> [OPTIONS]

COMMANDS:
    Event Handlers:
        session-start     Handle SessionStart event
        session-end       Handle SessionEnd event
        setup             Handle Setup event
        user-prompt       Handle UserPromptSubmit event
        pre-tool-use      Handle PreToolUse event
        post-tool-use     Handle PostToolUse event
        permission        Handle PermissionRequest event
        notification      Handle Notification event
        stop              Handle Stop event
        subagent-stop     Handle SubagentStop event
        pre-compact       Handle PreCompact event

    Utilities:
        init              Create example configuration
        validate          Validate configuration file
        install           Install hooks into Claude Code settings
        debug             Run event handler with verbose output

    Help:
        help              Print help for a command

OPTIONS:
    -c, --config <PATH>    Path to hooks.yaml (overrides search)
    -v, --verbose          Verbose output to stderr
    -q, --quiet            Suppress warnings
    -h, --help             Print help
    -V, --version          Print version

EXAMPLES:
    # Handle a PreToolUse event (stdin receives JSON)
    echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | cch pre-tool-use

    # Initialize configuration in current project
    cch init

    # Initialize with specific template
    cch init --template architect

    # Validate configuration
    cch validate

    # Validate specific file
    cch validate --config ./my-hooks.yaml

    # Install to user settings
    cch install --user

    # Install to project settings
    cch install --project

    # Debug a specific event
    echo '{"tool_name":"Bash",...}' | cch debug pre-tool-use
```

### 6.2 Event Handler Commands

All event handler commands:
- Read JSON from stdin
- Load configuration
- Match rules and execute actions
- Output JSON to stdout

```bash
# PreToolUse - most common
echo '{"tool_name":"Edit","tool_input":{"file_path":"src/main.ts"}}' | cch pre-tool-use

# PermissionRequest - for explain-yourself feature
echo '{"tool_name":"Bash","tool_input":{"command":"rm -rf /tmp/test"}}' | cch permission

# UserPromptSubmit - inject context based on user's question
echo '{"prompt":"How do I set up the database?"}' | cch user-prompt
```

### 6.3 Utility Commands

**`cch init`**

Creates example configuration:

```bash
# Create .claude/hooks.yaml with examples
cch init

# Create with specific template
cch init --template minimal      # Bare minimum
cch init --template developer    # Standard dev setup
cch init --template architect    # Architect agent setup
cch init --template strict       # Maximum guardrails

# Create in specific directory
cch init --path /path/to/project
```

**`cch validate`**

Validates configuration without running:

```bash
# Validate configuration
cch validate

# Output:
# ✓ Configuration valid
# ✓ 12 rules defined
# ✓ All referenced files exist
# ⚠ Warning: validators/old.py not referenced

# Validate specific file
cch validate --config ./hooks.yaml

# Strict mode (warnings are errors)
cch validate --strict
```

**`cch install`**

Installs CCH as Claude Code's hook handler:

```bash
# Install to user settings (~/.claude/settings.json)
cch install --user

# Install to project settings (.claude/settings.json)
cch install --project

# Preview changes without writing
cch install --dry-run

# Force overwrite existing hooks
cch install --force
```

**`cch debug`**

Run with verbose output for troubleshooting:

```bash
echo '{"tool_name":"Bash","tool_input":{"command":"git commit"}}' | cch debug pre-tool-use

# Output to stderr:
# [DEBUG] Loading config from .claude/hooks.yaml
# [DEBUG] Parsed 12 rules for PreToolUse
# [DEBUG] Evaluating rule: git-commit-guidelines
# [DEBUG]   tools: [Bash] ✓
# [DEBUG]   command_match: "git commit" ✓
# [DEBUG]   Rule matched!
# [DEBUG] Executing inject: .claude/context/commit-guidelines.md
# [DEBUG] Evaluating rule: block-wip-commits
# [DEBUG]   tools: [Bash] ✓
# [DEBUG]   command_match: "git commit" ✓
# [DEBUG]   block_if_match: checking patterns...
# [DEBUG]   Pattern "WIP" not found
# [DEBUG] Final: continue=true, context_length=1234
```

---

## 7. Configuration Reference

### 7.1 Full Schema

```yaml
# Version of the configuration format (required)
version: "1"

# Rules organized by event type
# Each event contains an array of rules

SessionStart:
  - name: string              # Rule identifier (required)
    enabled_when:             # Conditional execution (optional)
      env: string             # Environment variable name
      equals: string          # Exact match
      matches: string         # Regex match
      exists: boolean         # Just check if set
    inject: string            # Path to markdown file (optional)
    run: string | RunConfig   # Path to validator script (optional)

UserPromptSubmit:
  - name: string
    enabled_when: ...
    prompt_match: string      # Regex to match user's prompt (optional)
    inject: string
    run: string | RunConfig

PreToolUse:
  - name: string
    enabled_when: ...
    tools: string[]           # Tool names to match (optional, default: *)
    extensions: string[]      # File extensions (optional)
    directories: string[]     # Glob patterns (optional)
    operations: string[]      # create, edit, read, delete (optional)
    command_match: string     # Regex for Bash commands (optional)
    inject: string
    run: string | RunConfig
    action: "block"           # Unconditional block (optional)
    message: string           # Block message (optional)
    block_if_match:           # Conditional blocking (optional)
      - pattern: string       # Regex pattern
        message: string       # Message if matched

PostToolUse:
  - name: string
    enabled_when: ...
    tools: string[]
    extensions: string[]
    directories: string[]
    operations: string[]
    command_match: string
    inject: string
    run: string | RunConfig

PermissionRequest:
  - name: string
    enabled_when: ...
    tools: string[]
    extensions: string[]
    directories: string[]
    operations: string[]
    command_match: string
    inject: string
    run: string | RunConfig
    action: "block"
    message: string
    block_if_match: ...
    require_fields:           # Structured explanation (optional)
      - name: string          # Field identifier
        description: string   # What to explain

Notification:
  - name: string
    enabled_when: ...
    inject: string
    run: string | RunConfig

Stop:
  - name: string
    enabled_when: ...
    inject: string
    run: string | RunConfig

SubagentStop:
  - name: string
    enabled_when: ...
    inject: string
    run: string | RunConfig

PreCompact:
  - name: string
    enabled_when: ...
    inject: string
    run: string | RunConfig

Setup:
  - name: string
    enabled_when: ...
    inject: string
    run: string | RunConfig

SessionEnd:
  - name: string
    enabled_when: ...
    inject: string
    run: string | RunConfig

# RunConfig object (alternative to string path)
# run:
#   script: string            # Path to script
#   interpreter: string       # Override interpreter (optional)
#   timeout: number           # Timeout in seconds (optional, default: 30)
#   env:                      # Additional environment variables (optional)
#     KEY: value
```

### 7.2 Matcher Reference

**`tools`**

Matches tool names. Case-sensitive.

```yaml
tools: [Bash]                    # Exact match
tools: [Edit, Write]             # Multiple (OR)
tools: ["*"]                     # All tools (default)
tools: [Bash, Edit, Write, Read, Glob, Grep, LS]
```

**`extensions`**

Matches file extensions. Case-insensitive. Auto-normalizes dots.

```yaml
extensions: [.py]                # Single extension
extensions: [.ts, .tsx, .js]     # Multiple (OR)
extensions: [py, ts, js]         # Without dots (auto-normalized)
```

**`directories`**

Matches file paths against glob patterns.

```yaml
directories: [src/**]            # Anything under src/
directories: [**/test/**]        # Any test directory
directories: [cdk/**, infra/**]  # Multiple patterns (OR)
directories: [src/components/*]  # Direct children only
```

**`operations`**

Matches operation type inferred from tool and input.

```yaml
operations: [create]             # New files
operations: [edit]               # Modify existing
operations: [delete]             # Remove files
operations: [read]               # Read files
operations: [create, edit]       # Multiple (OR)
```

Operation detection:
- `Write` tool → `create` (new file) or `edit` (existing)
- `Edit` tool → `edit`
- `Read` tool → `read`
- `Bash` with `rm` → `delete`
- `Bash` with `touch`, `mkdir`, `>` → `create`

**`command_match`**

Regex pattern matched against Bash command.

```yaml
command_match: "git commit"              # Simple match
command_match: "git (commit|push)"       # Alternation
command_match: "rm\\s+-rf"               # With flags
command_match: "(?i)drop\\s+table"       # Case-insensitive
```

**`prompt_match`**

Regex pattern matched against user's prompt (UserPromptSubmit only).

```yaml
prompt_match: "database|sql|migration"
prompt_match: "(?i)how do i"
prompt_match: "^/deploy"                 # Slash commands
```

**`enabled_when`**

Conditional execution based on environment variables.

```yaml
# Exact match
enabled_when:
  env: CLAUDE_ROLE
  equals: architect

# Regex match
enabled_when:
  env: ENVIRONMENT
  matches: "prod|staging"

# Existence check
enabled_when:
  env: CI
  exists: true

# Negation
enabled_when:
  env: DISABLE_GUARDS
  exists: false
```

### 7.3 Action Reference

**`inject`**

Loads and injects markdown file content.

```yaml
inject: .claude/context/guidelines.md    # Relative to project root
inject: ./context/local.md               # Relative to config file
inject: ~/shared/context.md              # Home directory
inject: /absolute/path/file.md           # Absolute path
```

**`run`**

Executes validator script.

```yaml
# Simple form
run: .claude/validators/check.py

# Full form
run:
  script: .claude/validators/check.py
  interpreter: python3.11                # Override interpreter
  timeout: 60                            # Timeout in seconds
  env:                                   # Additional env vars
    STRICT_MODE: "true"
```

**`action`**

Unconditional block.

```yaml
action: block
message: |
  🚫 This operation is not allowed.
  
  Please contact the platform team for assistance.
```

**`block_if_match`**

Conditional blocking based on patterns.

```yaml
block_if_match:
  - pattern: "WIP"
    message: "WIP commits not allowed"
  - pattern: "(?i)fixup|squash"
    message: "Rebase fixup commits before PR"
  - pattern: "<<<<|>>>>|===="
    message: "Merge conflict markers detected"
```

**`require_fields`**

Require structured explanation (PermissionRequest only).

```yaml
require_fields:
  - name: what
    description: "What this command does in plain English"
  - name: why
    description: "Why this action is necessary"
  - name: risk
    description: "Risk level and what could go wrong"
  - name: undo
    description: "How to reverse if needed"
```

This generates injected context:

```markdown
Before requesting approval, provide the following information:

## What
[What this command does in plain English]

## Why
[Why this action is necessary]

## Risk
[Risk level and what could go wrong]

## Undo
[How to reverse if needed]
```

---

## 8. Script Validators

### 8.1 Overview

Scripts provide custom validation logic beyond pattern matching. Any executable script can be a validator.

### 8.2 Script Interface

**Input:**

| Source | Content |
|--------|---------|
| stdin | Full event JSON |
| `$CCH_EVENT` | Event type (e.g., `PreToolUse`) |
| `$CCH_TOOL` | Tool name (e.g., `Bash`) |
| `$CCH_FILE` | File path if applicable |
| `$CCH_COMMAND` | Command if Bash tool |
| `$CCH_CONFIG_DIR` | Directory containing hooks.yaml |
| `$CCH_PROJECT_DIR` | Project root directory |

**Output:**

| Exit Code | Meaning |
|-----------|---------|
| 0 | Continue (allow the operation) |
| 1-255 | Block (deny the operation) |

| Stream | Usage |
|--------|-------|
| stdout | Additional context to inject (optional) |
| stderr | Block message (used if exit non-zero) |

### 8.3 Interpreter Detection

| Extension | Interpreter |
|-----------|-------------|
| `.py` | `python3` |
| `.ts` | `bun` |
| `.js` | `bun` |
| `.sh` | `bash` |
| `.rb` | `ruby` |
| `.pl` | `perl` |
| (executable) | Direct execution |

### 8.4 Script Examples

**Python validator (sql-safety.py):**

```python
#!/usr/bin/env python3
"""Validate SQL commands for safety."""
import sys
import json
import re

def main():
    event = json.load(sys.stdin)
    command = event.get("tool_input", {}).get("command", "")
    
    # Block DROP TABLE without confirmation
    if re.search(r"DROP\s+TABLE", command, re.IGNORECASE):
        if "--confirm" not in command:
            print("DROP TABLE requires --confirm flag", file=sys.stderr)
            return 1
    
    # Block DELETE without WHERE
    if re.search(r"DELETE\s+FROM", command, re.IGNORECASE):
        if not re.search(r"WHERE", command, re.IGNORECASE):
            print("DELETE without WHERE clause is dangerous", file=sys.stderr)
            return 1
    
    # Block TRUNCATE entirely
    if re.search(r"TRUNCATE", command, re.IGNORECASE):
        print("TRUNCATE is not allowed - use DELETE with WHERE", file=sys.stderr)
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
```

**TypeScript validator (aws-safety.ts):**

```typescript
#!/usr/bin/env bun
/**
 * Validate AWS CLI commands for safety.
 */

interface Event {
  tool_name: string;
  tool_input: {
    command?: string;
  };
}

const BLOCKED_ACCOUNTS = ["111111111111", "prod", "production"];
const DANGEROUS_OPERATIONS = [
  /aws\s+rds.*delete/i,
  /aws\s+ec2.*terminate/i,
  /aws\s+s3.*rb\s/i,
  /aws\s+iam.*delete/i,
];

async function main(): Promise<number> {
  const input = await Bun.stdin.text();
  const event: Event = JSON.parse(input);
  const command = event.tool_input?.command || "";

  // Block production account access
  for (const account of BLOCKED_ACCOUNTS) {
    if (command.includes(account)) {
      console.error(`Access to ${account} account is blocked`);
      return 1;
    }
  }

  // Block dangerous operations
  for (const pattern of DANGEROUS_OPERATIONS) {
    if (pattern.test(command)) {
      console.error("Destructive AWS operation requires manual approval");
      return 1;
    }
  }

  return 0;
}

main().then(process.exit);
```

**Bash validator (pre-commit.sh):**

```bash
#!/usr/bin/env bash
# Validate git commits before they happen.
set -euo pipefail

# Read event from stdin
EVENT=$(cat)
COMMAND=$(echo "$EVENT" | jq -r '.tool_input.command // ""')
MESSAGE=$(echo "$COMMAND" | grep -oP '(?<=-m\s")[^"]+' || echo "")

# Check for forbidden patterns
if echo "$MESSAGE" | grep -qiE "(WIP|FIXME|TODO|HACK)"; then
    echo "Commit message contains forbidden patterns" >&2
    exit 1
fi

# Check for merge conflict markers in staged files
if git diff --cached --name-only | xargs grep -l "<<<<<<" 2>/dev/null; then
    echo "Staged files contain merge conflict markers" >&2
    exit 1
fi

# Inject reminder as additional context
cat << 'EOF'
Remember our commit message format:
- Start with type: feat|fix|docs|refactor|test|chore
- Keep first line under 72 characters
- Reference issue number if applicable
EOF

exit 0
```

### 8.5 Script Best Practices

1. **Always handle missing fields gracefully**
   ```python
   command = event.get("tool_input", {}).get("command", "")
   ```

2. **Use stderr for block messages, stdout for context**
   ```bash
   echo "Error message" >&2  # Block message
   echo "Context info"       # Injected context
   ```

3. **Exit quickly on success**
   ```python
   if not requires_validation(command):
       sys.exit(0)
   ```

4. **Provide helpful error messages**
   ```typescript
   console.error(`Cannot access ${account}: production accounts are blocked.
   If you need production access, use the AWS console directly.`);
   ```

5. **Consider timeout constraints** (default 30s)
   ```yaml
   run:
     script: ./slow-validator.py
     timeout: 120
   ```

---

## 9. Claude Code Integration

### 9.1 How It Works

Claude Code's native hooks system calls external commands at specific lifecycle points. CCH registers itself as the handler for all events, then routes to your YAML-based configuration.

### 9.2 Installation Process

```bash
# 1. Install CCH binary
cargo install --git https://github.com/user/cch

# 2. Create configuration
cd your-project
cch init

# 3. Edit hooks.yaml to your needs
vim .claude/hooks.yaml

# 4. Install into Claude Code
cch install --project   # For this project only
# OR
cch install --user      # For all projects
```

### 9.3 Generated Settings

Running `cch install` adds to Claude Code's settings:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "cch pre-tool-use"
          }
        ]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "cch post-tool-use"
          }
        ]
      }
    ],
    "PermissionRequest": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "cch permission"
          }
        ]
      }
    ],
    "UserPromptSubmit": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "cch user-prompt"
          }
        ]
      }
    ],
    "SessionStart": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "cch session-start"
          }
        ]
      }
    ],
    "SessionEnd": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "cch session-end"
          }
        ]
      }
    ],
    "Stop": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "cch stop"
          }
        ]
      }
    ],
    "Notification": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "cch notification"
          }
        ]
      }
    ],
    "SubagentStop": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "cch subagent-stop"
          }
        ]
      }
    ],
    "PreCompact": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "cch pre-compact"
          }
        ]
      }
    ],
    "Setup": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "cch setup"
          }
        ]
      }
    ]
  }
}
```

### 9.4 Uninstallation

```bash
# Remove CCH hooks from settings
cch uninstall --project
cch uninstall --user

# Remove CCH binary
cargo uninstall cch
```

---

## 10. Example Configurations

### 10.1 Minimal (Getting Started)

```yaml
version: "1"

PreToolUse:
  - name: explain-bash
    tools: [Bash]
    inject: .claude/context/explain-commands.md
```

### 10.2 Standard Developer Setup

```yaml
version: "1"

SessionStart:
  - name: project-context
    inject: .claude/context/project-overview.md

UserPromptSubmit:
  - name: architecture-context
    prompt_match: "architect|design|structure|pattern"
    inject: .claude/context/architecture.md

  - name: database-context
    prompt_match: "database|sql|migration|schema"
    inject: .claude/context/database.md

PreToolUse:
  # Skill injection
  - name: typescript-skill
    tools: [Edit, Write]
    extensions: [.ts, .tsx]
    inject: .claude/context/skills/typescript.md

  - name: python-skill
    tools: [Edit, Write]
    extensions: [.py]
    inject: .claude/context/skills/python.md

  # Git guards
  - name: commit-guidelines
    tools: [Bash]
    command_match: "git commit"
    inject: .claude/context/commit-guidelines.md

  - name: block-bad-commits
    tools: [Bash]
    command_match: "git commit"
    block_if_match:
      - pattern: "WIP"
        message: "WIP commits not allowed"
      - pattern: "<<<<|>>>>"
        message: "Merge conflict markers detected"

  - name: block-force-push
    tools: [Bash]
    command_match: "git push.*(--force|-f)"
    action: block
    message: "Force push is not allowed. Use --force-with-lease if necessary."

PostToolUse:
  - name: lint-reminder
    tools: [Edit, Write]
    extensions: [.ts, .tsx, .py, .js]
    inject: .claude/context/lint-reminder.md

PermissionRequest:
  - name: explain-commands
    tools: [Bash]
    inject: .claude/context/templates/explain-command.md

  - name: explain-destructive
    tools: [Bash]
    command_match: "rm -rf|git reset --hard|drop table"
    require_fields:
      - name: what
        description: "What this does"
      - name: why
        description: "Why it's needed"
      - name: risk
        description: "What could go wrong"

Stop:
  - name: completion-check
    inject: .claude/context/completion-checklist.md
```

### 10.3 Architect Agent Setup

```yaml
version: "1"

SessionStart:
  - name: architect-role
    inject: .claude/context/roles/architect.md

PreToolUse:
  # Allow only documentation files
  - name: allow-markdown
    tools: [Edit, Write]
    extensions: [.md, .mdx]
    inject: .claude/context/markdown-guidelines.md

  - name: allow-yaml
    tools: [Edit, Write]
    extensions: [.yaml, .yml]
    inject: .claude/context/yaml-guidelines.md

  - name: allow-json
    tools: [Edit, Write]
    extensions: [.json]
    inject: .claude/context/json-guidelines.md

  # Block all code files
  - name: block-code-files
    tools: [Edit, Write, str_replace]
    extensions: [.py, .ts, .tsx, .js, .jsx, .go, .rs, .java, .rb, .sh, .bash]
    action: block
    message: |
      🚫 Architect agent cannot edit code files.
      
      You may only edit:
      - Markdown files (.md, .mdx)
      - YAML files (.yaml, .yml)
      - JSON files (.json)
      
      To make code changes:
      1. Document the required changes in specifications
      2. Create instructions for the developer agent
      3. Place instructions in the handoff directory

  # Block direct command execution
  - name: block-commands
    tools: [Bash]
    command_match: "^(?!cat|ls|head|tail|grep|find|tree)"
    action: block
    message: |
      🚫 Architect agent can only run read-only commands.
      
      Allowed: cat, ls, head, tail, grep, find, tree
      
      For other operations, create instructions for the developer agent.

Stop:
  - name: architect-checklist
    inject: .claude/context/architect-checklist.md
```

### 10.4 Maximum Security Setup

```yaml
version: "1"

PreToolUse:
  # Validate all bash commands with script
  - name: validate-all-commands
    tools: [Bash]
    run: .claude/validators/command-validator.py

  # Block production access
  - name: block-production
    tools: [Bash]
    command_match: "prod|production|PROD"
    action: block
    message: "Production access is blocked. Use staging environment."

  # Block dangerous commands
  - name: block-dangerous
    tools: [Bash]
    block_if_match:
      - pattern: "rm\\s+-rf\\s+/"
        message: "Cannot delete root directories"
      - pattern: "chmod\\s+777"
        message: "777 permissions not allowed"
      - pattern: "curl.*\\|.*sh"
        message: "Piping curl to shell is not allowed"
      - pattern: "> /dev/"
        message: "Cannot write to device files"

  # Block sensitive file access
  - name: block-sensitive-files
    tools: [Edit, Write, Read]
    directories: [".env*", "**/.env*", "**/secrets/**", "**/.aws/**"]
    action: block
    message: "Cannot access sensitive configuration files"

  # Require validation for all file edits
  - name: validate-file-edits
    tools: [Edit, Write]
    run: .claude/validators/file-validator.py

PermissionRequest:
  # Require full explanation for everything
  - name: explain-everything
    tools: ["*"]
    inject: .claude/context/templates/full-explanation.md
    require_fields:
      - name: action
        description: "Exactly what will happen"
      - name: purpose
        description: "Why this is needed for the task"
      - name: scope
        description: "What files/systems are affected"
      - name: risks
        description: "What could go wrong"
      - name: reversibility
        description: "How to undo if needed"
      - name: alternatives
        description: "Other ways to achieve this"
```

### 10.5 Infrastructure (CDK/Terraform) Setup

```yaml
version: "1"

PreToolUse:
  # CDK skill
  - name: cdk-skill
    tools: [Edit, Write]
    extensions: [.ts, .js]
    directories: [cdk/**, infra/cdk/**, **/cdk/**]
    inject: .claude/context/skills/cdk.md

  # Terraform skill
  - name: terraform-skill
    tools: [Edit, Write]
    extensions: [.tf, .tfvars]
    inject: .claude/context/skills/terraform.md

  # Validate infrastructure changes
  - name: validate-infra
    tools: [Edit, Write]
    directories: [cdk/**, terraform/**, infra/**]
    run: .claude/validators/infra-validator.ts

  # Block direct apply commands
  - name: block-direct-apply
    tools: [Bash]
    command_match: "cdk deploy|terraform apply"
    block_if_match:
      - pattern: "--require-approval never"
        message: "Cannot skip approval for infrastructure changes"
      - pattern: "-auto-approve"
        message: "Cannot auto-approve infrastructure changes"

  # Warn about production deployments
  - name: production-deploy-warning
    tools: [Bash]
    command_match: "(cdk|terraform).*(deploy|apply).*prod"
    inject: .claude/context/production-deploy-warning.md

PermissionRequest:
  - name: explain-infra-commands
    tools: [Bash]
    command_match: "cdk|terraform|pulumi|aws|gcloud|az"
    inject: .claude/context/templates/explain-infra.md
    require_fields:
      - name: resources
        description: "What resources will be created/modified/deleted"
      - name: cost
        description: "Estimated cost impact"
      - name: downtime
        description: "Expected downtime or service disruption"
      - name: rollback
        description: "Rollback procedure if something goes wrong"
```

---

## 11. Example Markdown Templates

### 11.1 explain-command.md

```markdown
# Command Explanation Required

Before asking for permission, explain this command clearly:

## What does this do?
Provide a clear, jargon-free explanation of what this command accomplishes.

## Why is this needed?
Connect this action to the user's goal. Why is this step necessary?

## Command breakdown
If the command has multiple parts, environment variables, or pipes:
- Explain each component
- Decode any environment variables
- Clarify what each flag does

## Risk assessment
- **Risk level:** [Low / Medium / High]
- **Reversible:** [Yes / No / Partially]
- **Side effects:** List any side effects
```

### 11.2 explain-destructive.md

```markdown
# ⚠️ Destructive Operation - Full Explanation Required

This command performs a potentially destructive action. You MUST provide complete information.

## What
Explain exactly what this command does. No jargon. No abbreviations.

## Why
Explain why this destructive action is necessary. What goal does it serve?

## Risk
- **Risk Level:** [Low / Medium / High / Critical]
- **What could go wrong:**
- **Blast radius:** What systems/data could be affected?

## Undo
- Can this be undone? [Yes / No / Partially]
- If yes, how?
- If no, what's the recovery procedure?

## Confirmation
State: "I understand this is destructive and have verified the target is correct."

---
Only after providing ALL sections above, ask for permission.
```

### 11.3 project-overview.md

```markdown
# Project Context

## Overview
[Brief description of what this project does]

## Technology Stack
- **Language:** TypeScript
- **Runtime:** Node.js 20 / Bun
- **Framework:** [e.g., Express, Fastify]
- **Database:** [e.g., PostgreSQL, MongoDB]
- **Infrastructure:** [e.g., AWS CDK, Terraform]

## Directory Structure
```
src/           # Application source code
tests/         # Test files
cdk/           # Infrastructure as code
scripts/       # Build and utility scripts
docs/          # Documentation
```

## Key Conventions
- Use absolute imports from `src/`
- Tests go in `tests/` mirroring `src/` structure
- All PRs require passing tests and linting

## Important Files
- `src/config.ts` - Application configuration
- `src/db/schema.ts` - Database schema definitions
- `cdk/app.ts` - Infrastructure entry point
```

### 11.4 architect-role.md

```markdown
# Architect Agent Role

You are operating as the **Architect Agent**. Your responsibilities:

## What You Do
- Create technical specifications and design documents
- Define architecture decisions and patterns
- Write instructions for the developer agent
- Review and provide feedback on approaches

## What You Don't Do
- ❌ Edit source code files (.ts, .js, .py, etc.)
- ❌ Run build or deployment commands
- ❌ Make direct changes to infrastructure
- ❌ Execute tests

## Workflow
1. Analyze requirements
2. Design solution in markdown specifications
3. Create detailed instructions for developer agent
4. Place instructions in `instructions/` directory
5. Hand off to developer agent for implementation

## Output Locations
- Specifications: `docs/specs/`
- Architecture decisions: `docs/adr/`
- Developer instructions: `instructions/`
```

### 11.5 commit-guidelines.md

```markdown
# Git Commit Guidelines

## Commit Message Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

## Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `test`: Adding or correcting tests
- `chore`: Build process or auxiliary tool changes

## Rules
- Subject line max 72 characters
- Use imperative mood ("Add feature" not "Added feature")
- Reference issues in footer: `Closes #123`

## Examples
```
feat(auth): add OAuth2 login support

Implement Google and GitHub OAuth2 providers.
Users can now link social accounts.

Closes #456
```

## Forbidden
- No "WIP" commits on main/develop branches
- No fixup/squash commits in PRs (rebase first)
- No merge conflict markers
```

### 11.6 completion-checklist.md

```markdown
# Before Saying "Done"

Verify the following:

## Code Quality
- [ ] Code follows project style guidelines
- [ ] No console.log or debug statements left behind
- [ ] No commented-out code
- [ ] No TODO/FIXME without issue references

## Testing
- [ ] New code has tests
- [ ] All tests pass
- [ ] No skipped tests without explanation

## Documentation
- [ ] Public APIs are documented
- [ ] Complex logic has comments
- [ ] README updated if needed

## Final Checks
- [ ] Changes are committed
- [ ] Branch is up to date with main
- [ ] No untracked files that should be committed
```

---

## 12. Example Validator Scripts

### 12.1 command-validator.py

```python
#!/usr/bin/env python3
"""
Comprehensive command validator.
Blocks dangerous commands and requires confirmation for risky ones.
"""
import sys
import json
import re
import os

# Commands that are always blocked
BLOCKED_PATTERNS = [
    (r"rm\s+-rf\s+/(?!\w)", "Cannot delete root directories"),
    (r"rm\s+-rf\s+~", "Cannot delete home directory"),
    (r"chmod\s+777", "777 permissions are insecure"),
    (r"curl.*\|\s*(ba)?sh", "Piping curl to shell is dangerous"),
    (r"wget.*\|\s*(ba)?sh", "Piping wget to shell is dangerous"),
    (r">\s*/dev/sd", "Cannot write directly to disk devices"),
    (r"dd\s+if=.*of=/dev/sd", "Cannot dd to disk devices"),
    (r"mkfs\.", "Cannot format filesystems"),
    (r":(){ :|:& };:", "Fork bomb detected"),
    (r">\s*/etc/passwd", "Cannot modify passwd file"),
    (r">\s*/etc/shadow", "Cannot modify shadow file"),
]

# Commands that require explicit confirmation
RISKY_PATTERNS = [
    (r"rm\s+-rf", "Recursive force delete"),
    (r"git\s+reset\s+--hard", "Hard reset discards changes"),
    (r"git\s+clean\s+-fd", "Clean removes untracked files"),
    (r"DROP\s+TABLE", "Dropping database table"),
    (r"DROP\s+DATABASE", "Dropping entire database"),
    (r"TRUNCATE", "Truncating table"),
    (r"DELETE\s+FROM\s+\w+\s*$", "DELETE without WHERE clause"),
]


def main():
    try:
        event = json.load(sys.stdin)
    except json.JSONDecodeError:
        print("Invalid JSON input", file=sys.stderr)
        return 1
    
    command = event.get("tool_input", {}).get("command", "")
    
    if not command:
        return 0
    
    # Check blocked patterns
    for pattern, message in BLOCKED_PATTERNS:
        if re.search(pattern, command, re.IGNORECASE):
            print(f"🚫 BLOCKED: {message}", file=sys.stderr)
            print(f"Command: {command}", file=sys.stderr)
            return 1
    
    # Check risky patterns (warn but allow)
    warnings = []
    for pattern, message in RISKY_PATTERNS:
        if re.search(pattern, command, re.IGNORECASE):
            warnings.append(message)
    
    if warnings:
        # Output warning context to stdout (will be injected)
        print("⚠️ **Risky Operation Detected**")
        print()
        for warning in warnings:
            print(f"- {warning}")
        print()
        print("Proceed with caution. Ensure you have backups if needed.")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())
```

### 12.2 aws-safety.ts

```typescript
#!/usr/bin/env bun
/**
 * AWS CLI command validator.
 * Blocks production access and dangerous operations.
 */

interface Event {
  tool_name: string;
  tool_input: {
    command?: string;
  };
}

// Production account IDs and profile names to block
const BLOCKED_IDENTIFIERS = [
  "111111111111",  // Production account ID
  "222222222222",  // Production account ID
  "--profile prod",
  "--profile production",
  "AWS_PROFILE=prod",
];

// Dangerous operation patterns
const DANGEROUS_PATTERNS = [
  { pattern: /aws\s+rds.*delete-db-instance/i, message: "Deleting RDS instance" },
  { pattern: /aws\s+rds.*delete-db-cluster/i, message: "Deleting RDS cluster" },
  { pattern: /aws\s+ec2.*terminate-instances/i, message: "Terminating EC2 instances" },
  { pattern: /aws\s+s3.*rb\s/i, message: "Removing S3 bucket" },
  { pattern: /aws\s+s3\s+rm.*--recursive/i, message: "Recursive S3 delete" },
  { pattern: /aws\s+iam.*delete-user/i, message: "Deleting IAM user" },
  { pattern: /aws\s+iam.*delete-role/i, message: "Deleting IAM role" },
  { pattern: /aws\s+lambda.*delete-function/i, message: "Deleting Lambda function" },
  { pattern: /aws\s+cloudformation.*delete-stack/i, message: "Deleting CloudFormation stack" },
];

async function main(): Promise<number> {
  let event: Event;
  
  try {
    const input = await Bun.stdin.text();
    event = JSON.parse(input);
  } catch {
    console.error("Invalid JSON input");
    return 1;
  }
  
  const command = event.tool_input?.command || "";
  
  if (!command.includes("aws ")) {
    return 0;
  }
  
  // Check for production identifiers
  for (const identifier of BLOCKED_IDENTIFIERS) {
    if (command.includes(identifier)) {
      console.error(`🚫 BLOCKED: Production account access detected`);
      console.error(`Identifier: ${identifier}`);
      console.error(`Use staging or development environment for testing.`);
      return 1;
    }
  }
  
  // Check for dangerous operations
  const dangers: string[] = [];
  for (const { pattern, message } of DANGEROUS_PATTERNS) {
    if (pattern.test(command)) {
      dangers.push(message);
    }
  }
  
  if (dangers.length > 0) {
    // Output warning context (stdout = injected)
    console.log("⚠️ **Dangerous AWS Operation**\n");
    console.log("This command will perform:");
    for (const danger of dangers) {
      console.log(`- ${danger}`);
    }
    console.log("\nEnsure you have:");
    console.log("- Verified the target resources");
    console.log("- Recent backups if applicable");
    console.log("- Approval for destructive changes");
  }
  
  return 0;
}

main().then((code) => process.exit(code));
```

### 12.3 pre-commit.sh

```bash
#!/usr/bin/env bash
# Pre-commit validation script.
# Ensures commits meet quality standards.
set -euo pipefail

# Read event from stdin
EVENT=$(cat)
COMMAND=$(echo "$EVENT" | jq -r '.tool_input.command // ""')

# Extract commit message if present
MESSAGE=""
if [[ "$COMMAND" =~ -m[[:space:]]+[\"\']([^\"\']+)[\"\'] ]]; then
    MESSAGE="${BASH_REMATCH[1]}"
elif [[ "$COMMAND" =~ -m[[:space:]]+([^[:space:]]+) ]]; then
    MESSAGE="${BASH_REMATCH[1]}"
fi

# Check for forbidden patterns in commit message
FORBIDDEN_PATTERNS=("WIP" "wip" "FIXME" "fixme" "TODO" "todo" "HACK" "hack" "XXX")
for pattern in "${FORBIDDEN_PATTERNS[@]}"; do
    if [[ "$MESSAGE" == *"$pattern"* ]]; then
        echo "🚫 Commit message contains forbidden pattern: $pattern" >&2
        echo "Please use a descriptive commit message." >&2
        exit 1
    fi
done

# Check commit message format (conventional commits)
if [[ -n "$MESSAGE" ]]; then
    if ! [[ "$MESSAGE" =~ ^(feat|fix|docs|style|refactor|test|chore|ci|perf|build|revert)(\([a-z0-9-]+\))?: ]]; then
        echo "⚠️ Commit message doesn't follow conventional commits format." >&2
        echo "Expected: type(scope): description" >&2
        echo "Types: feat, fix, docs, style, refactor, test, chore, ci, perf, build, revert" >&2
        # Warning only, don't block
    fi
fi

# Check for staged files with issues (if in a git repo)
if git rev-parse --git-dir > /dev/null 2>&1; then
    # Check for conflict markers in staged files
    if git diff --cached --name-only | xargs -I {} sh -c 'grep -l "<<<<<<" "{}" 2>/dev/null' | grep -q .; then
        echo "🚫 Staged files contain merge conflict markers" >&2
        exit 1
    fi
    
    # Check for debug statements in staged files
    DEBUG_FILES=$(git diff --cached --name-only | xargs grep -l "console\.log\|debugger\|binding\.pry\|import pdb" 2>/dev/null || true)
    if [[ -n "$DEBUG_FILES" ]]; then
        echo "⚠️ Debug statements found in:" >&2
        echo "$DEBUG_FILES" >&2
        echo "Consider removing before committing." >&2
        # Warning only
    fi
fi

# Success - inject reminder context
cat << 'EOF'
**Commit Checklist:**
- ✅ Tests pass
- ✅ Linting passes  
- ✅ No debug statements
- ✅ Descriptive commit message
EOF

exit 0
```

### 12.4 file-validator.py

```python
#!/usr/bin/env python3
"""
File edit validator.
Enforces file-level rules and restrictions.
"""
import sys
import json
import os
import re
from pathlib import Path

# Files that should never be edited
PROTECTED_FILES = [
    ".env",
    ".env.local",
    ".env.production",
    "package-lock.json",
    "yarn.lock", 
    "pnpm-lock.yaml",
    "Cargo.lock",
    "poetry.lock",
]

# Directories that require extra caution
SENSITIVE_DIRS = [
    "config/",
    "secrets/",
    ".github/workflows/",
    "infrastructure/",
    "prod/",
    "production/",
]

# File patterns that shouldn't be created
FORBIDDEN_PATTERNS = [
    r"\.env\.\w+\.local$",  # .env.*.local files
    r"\.bak$",              # Backup files
    r"\.tmp$",              # Temp files
    r"~$",                  # Editor backup files
]


def main():
    try:
        event = json.load(sys.stdin)
    except json.JSONDecodeError:
        print("Invalid JSON input", file=sys.stderr)
        return 1
    
    tool_input = event.get("tool_input", {})
    file_path = tool_input.get("file_path") or tool_input.get("path", "")
    
    if not file_path:
        return 0
    
    file_path = Path(file_path)
    file_name = file_path.name
    
    # Check protected files
    if file_name in PROTECTED_FILES:
        print(f"🚫 BLOCKED: {file_name} is a protected file", file=sys.stderr)
        print("This file should not be edited directly.", file=sys.stderr)
        return 1
    
    # Check forbidden patterns
    for pattern in FORBIDDEN_PATTERNS:
        if re.search(pattern, str(file_path)):
            print(f"🚫 BLOCKED: File matches forbidden pattern", file=sys.stderr)
            print(f"Pattern: {pattern}", file=sys.stderr)
            return 1
    
    # Check sensitive directories (warn but allow)
    for sensitive_dir in SENSITIVE_DIRS:
        if sensitive_dir in str(file_path):
            print(f"⚠️ **Sensitive Directory Edit**")
            print(f"Editing file in: {sensitive_dir}")
            print("Please ensure this change is intentional and reviewed.")
            break
    
    return 0


if __name__ == "__main__":
    sys.exit(main())
```

---

## 13. Implementation Plan

### Phase 1: Foundation (Days 1-2)

**Goal:** Basic CLI that can parse config and handle events

**Tasks:**
- [ ] Initialize Rust project with Cargo
- [ ] Set up directory structure
- [ ] Add dependencies to Cargo.toml
- [ ] Implement CLI argument parsing with Clap
  - [ ] Define all subcommands
  - [ ] Implement --config, --verbose, --quiet flags
  - [ ] Add help text and examples
- [ ] Implement config loader
  - [ ] Config file path resolution
  - [ ] YAML parsing with serde_yaml
  - [ ] Config struct definitions
- [ ] Implement event parser
  - [ ] JSON stdin reading
  - [ ] Event struct definitions
- [ ] Implement basic output
  - [ ] JSON response struct
  - [ ] Serialization to stdout
- [ ] Add basic error handling
- [ ] Write unit tests for config loading

**Deliverable:** `cch pre-tool-use` reads config, parses stdin, outputs `{"continue": true}`

### Phase 2: Matching Engine (Days 3-4)

**Goal:** Full rule matching capabilities

**Tasks:**
- [ ] Implement matcher engine framework
  - [ ] Rule evaluation loop
  - [ ] Match result collection
- [ ] Implement tool name matcher
  - [ ] Exact match
  - [ ] Wildcard support
- [ ] Implement extension matcher
  - [ ] Dot normalization
  - [ ] Case-insensitive matching
- [ ] Implement directory matcher
  - [ ] Glob pattern support (glob crate)
  - [ ] Multiple pattern OR logic
- [ ] Implement operation type detection
  - [ ] Infer from tool name
  - [ ] Infer from tool input
- [ ] Implement command regex matcher
  - [ ] Regex compilation and caching
  - [ ] Pattern matching
- [ ] Implement prompt regex matcher
- [ ] Implement enabled_when conditions
  - [ ] env + equals
  - [ ] env + matches
  - [ ] env + exists
- [ ] Comprehensive unit tests for all matchers

**Deliverable:** Rules correctly match based on all criteria

### Phase 3: Core Actions (Days 5-6)

**Goal:** Injection and blocking working

**Tasks:**
- [ ] Implement inject action
  - [ ] Markdown file loading
  - [ ] Path resolution (relative, absolute, home)
  - [ ] Missing file handling (warn, continue)
  - [ ] Context aggregation (multiple rules)
- [ ] Implement block action
  - [ ] Unconditional blocking
  - [ ] Custom message support
  - [ ] First block wins (message)
- [ ] Implement block_if_match
  - [ ] Pattern array evaluation
  - [ ] Per-pattern messages
- [ ] Implement require_fields
  - [ ] Field definition parsing
  - [ ] Template generation
- [ ] Action execution orchestration
  - [ ] Correct execution order
  - [ ] Result aggregation
- [ ] Unit tests for all actions

**Deliverable:** Injection, blocking, and require_fields working

### Phase 4: Script Execution (Days 7-8)

**Goal:** External validator scripts working

**Tasks:**
- [ ] Implement script path resolution
- [ ] Implement interpreter detection
  - [ ] Extension-based detection
  - [ ] Explicit interpreter override
- [ ] Implement script execution
  - [ ] Spawn process
  - [ ] Pipe stdin (event JSON)
  - [ ] Set environment variables
  - [ ] Capture stdout/stderr
  - [ ] Handle exit code
- [ ] Implement timeout handling
  - [ ] Default 30s timeout
  - [ ] Configurable per-script
  - [ ] Graceful termination
- [ ] Implement error handling
  - [ ] Script not found
  - [ ] Permission denied
  - [ ] Execution failure
  - [ ] Timeout exceeded
- [ ] Integration tests with real scripts
  - [ ] Python validator
  - [ ] TypeScript validator (bun)
  - [ ] Bash validator

**Deliverable:** Script validators working with all supported languages

### Phase 5: All Events (Day 9)

**Goal:** Support all hook events

**Tasks:**
- [ ] Implement SessionStart handler
- [ ] Implement SessionEnd handler
- [ ] Implement Setup handler
- [ ] Implement UserPromptSubmit handler
  - [ ] prompt_match support
- [ ] Implement PreToolUse handler (already done)
- [ ] Implement PostToolUse handler
- [ ] Implement PermissionRequest handler
  - [ ] require_fields support
- [ ] Implement Notification handler
- [ ] Implement Stop handler
- [ ] Implement SubagentStop handler
- [ ] Implement PreCompact handler
- [ ] Validate actions per event type
- [ ] Integration tests for each event

**Deliverable:** All 11 events fully supported

### Phase 6: Utility Commands (Day 10)

**Goal:** Developer experience tooling

**Tasks:**
- [ ] Implement `cch init`
  - [ ] Create .claude directory
  - [ ] Generate example hooks.yaml
  - [ ] Generate example context files
  - [ ] Template options (minimal, standard, architect, strict)
- [ ] Implement `cch validate`
  - [ ] Config syntax validation
  - [ ] Reference file existence check
  - [ ] Matcher pattern validation
  - [ ] Warning for unused files
  - [ ] Strict mode option
- [ ] Implement `cch install`
  - [ ] Read existing settings.json
  - [ ] Merge CCH hooks
  - [ ] Write updated settings
  - [ ] --user and --project options
  - [ ] --dry-run preview
  - [ ] --force overwrite
- [ ] Implement `cch uninstall`
  - [ ] Remove CCH hooks from settings
  - [ ] Preserve other hooks
- [ ] Implement `cch debug`
  - [ ] Verbose matcher output
  - [ ] Action execution trace
  - [ ] Script output display
- [ ] Colored terminal output
- [ ] Helpful error messages with file:line

**Deliverable:** Complete developer tooling

### Phase 7: Testing & Polish (Days 11-12)

**Goal:** Production-ready quality

**Tasks:**
- [ ] Comprehensive integration tests
  - [ ] End-to-end event handling
  - [ ] Complex config scenarios
  - [ ] Error conditions
- [ ] Performance testing
  - [ ] Cold start benchmarks
  - [ ] Large config benchmarks
- [ ] Error message review
  - [ ] Clear, actionable messages
  - [ ] Consistent formatting
- [ ] Documentation review
  - [ ] README accuracy
  - [ ] Example accuracy
- [ ] Edge case handling
  - [ ] Empty config
  - [ ] No matching rules
  - [ ] Invalid JSON input
  - [ ] Missing files
- [ ] Code cleanup
  - [ ] Remove debug code
  - [ ] Consistent style
  - [ ] Documentation comments

**Deliverable:** Production-ready binary

### Phase 8: Distribution (Days 13-14)

**Goal:** Easy installation for users

**Tasks:**
- [ ] Set up GitHub repository
  - [ ] README.md
  - [ ] LICENSE (MIT)
  - [ ] CONTRIBUTING.md
  - [ ] Issue templates
- [ ] Configure cargo-dist
  - [ ] `cargo dist init`
  - [ ] Configure target platforms
  - [ ] Configure release workflow
- [ ] Create GitHub Actions workflows
  - [ ] CI (test on all platforms)
  - [ ] Release (build and upload)
- [ ] Create install.sh script
  - [ ] Platform detection
  - [ ] Binary download
  - [ ] PATH installation
- [ ] Test installation methods
  - [ ] `cargo install --git`
  - [ ] Pre-built binaries
  - [ ] install.sh
- [ ] Write installation documentation
- [ ] Optional: Publish to crates.io

**Deliverable:** Users can install via cargo or pre-built binaries

---

## 14. Testing Strategy

### 14.1 Unit Tests

| Module | Coverage Focus |
|--------|----------------|
| config/loader | Path resolution, YAML parsing, defaults |
| config/validation | Schema validation, error messages |
| events/parser | JSON parsing, field extraction |
| matcher/* | Each matcher type independently |
| actions/inject | File loading, path resolution |
| actions/block | Block logic, message handling |
| actions/run | (mock execution) |
| output/response | JSON serialization |

### 14.2 Integration Tests

| Scenario | Description |
|----------|-------------|
| Basic injection | Single rule matches, injects markdown |
| Multiple rules | Multiple rules match, contexts aggregate |
| Blocking | Rule blocks, returns correct response |
| Script pass | Script exits 0, continues |
| Script fail | Script exits 1, blocks with message |
| Script timeout | Script exceeds timeout, blocks |
| No config | Graceful handling, returns continue |
| No matches | No rules match, returns continue |
| Invalid input | Bad JSON, returns error |

### 14.3 End-to-End Tests

| Test | Description |
|------|-------------|
| Full workflow | init → validate → install → event handling |
| Real scripts | Python, TypeScript, Bash validators |
| Claude Code simulation | Simulate actual hook invocations |

### 14.4 Test Fixtures

```
tests/fixtures/
├── configs/
│   ├── minimal.yaml
│   ├── standard.yaml
│   ├── complex.yaml
│   ├── invalid-syntax.yaml
│   └── invalid-schema.yaml
├── context/
│   ├── simple.md
│   └── with-variables.md
├── validators/
│   ├── always-pass.sh
│   ├── always-fail.py
│   ├── conditional.ts
│   └── slow.sh
└── events/
    ├── pre-tool-use-bash.json
    ├── pre-tool-use-edit.json
    ├── permission-request.json
    └── user-prompt.json
```

---

## 15. Distribution

### 15.1 Installation Methods

**Method 1: Cargo (requires Rust)**

```bash
# From crates.io (if published)
cargo install cch

# From GitHub
cargo install --git https://github.com/user/cch
```

**Method 2: Pre-built binaries**

```bash
# Using install script
curl -fsSL https://raw.githubusercontent.com/user/cch/main/install.sh | sh

# Manual download
# Go to https://github.com/user/cch/releases/latest
# Download binary for your platform
# Move to PATH
```

**Method 3: Homebrew (future)**

```bash
brew install user/tap/cch
```

### 15.2 Release Artifacts

Each release includes:

| Artifact | Platform |
|----------|----------|
| `cch-aarch64-apple-darwin.tar.gz` | macOS Apple Silicon |
| `cch-x86_64-apple-darwin.tar.gz` | macOS Intel |
| `cch-x86_64-unknown-linux-gnu.tar.gz` | Linux x86_64 |
| `cch-aarch64-unknown-linux-gnu.tar.gz` | Linux ARM64 |
| `cch-x86_64-pc-windows-msvc.zip` | Windows x86_64 |
| `checksums.txt` | SHA256 checksums |
| `install.sh` | Unix install script |
| `install.ps1` | Windows install script |

### 15.3 Release Process

```bash
# 1. Update version in Cargo.toml
# 2. Update CHANGELOG.md
# 3. Commit changes
git commit -am "Release v1.0.0"

# 4. Create and push tag
git tag v1.0.0
git push origin v1.0.0

# 5. GitHub Actions automatically:
#    - Runs tests
#    - Builds for all platforms
#    - Creates GitHub Release
#    - Uploads artifacts
```

---

## 16. Success Metrics

### 16.1 Performance

| Metric | Target | Measurement |
|--------|--------|-------------|
| Cold start | < 5ms | `time cch --version` |
| Config parse (10KB) | < 2ms | Benchmark test |
| Rule matching (50 rules) | < 1ms | Benchmark test |
| Total (no scripts) | < 10ms | End-to-end test |

### 16.2 Reliability

| Metric | Target |
|--------|--------|
| Test coverage | > 80% |
| CI pass rate | > 99% |
| Crash rate | 0 in production |

### 16.3 Adoption (6 months post-launch)

| Metric | Target |
|--------|--------|
| GitHub stars | 100+ |
| Cargo downloads | 500+ |
| Active users | 50+ |
| Community configs shared | 10+ |

---

## 17. Future Enhancements

### Post-v1.0 Roadmap

| Version | Feature | Description |
|---------|---------|-------------|
| v1.1 | Config inheritance | Project extends user config |
| v1.1 | Variable substitution | `{file_path}`, `{tool_name}` in markdown |
| v1.2 | Remote configs | Pull configs from URLs |
| v1.2 | Config sharing | `cch pull user/config-name` |
| v1.3 | Conditional markdown | `{% if extension == '.py' %}` |
| v1.3 | State tracking | `after_count: 3` for "remind after N edits" |
| v2.0 | Plugin system | Custom matchers and actions in Rust/WASM |
| v2.0 | Web UI | Visual config editor |
| v2.1 | Telemetry | Anonymous usage stats (opt-in) |
| v2.1 | Marketplace | Share and discover configs |

### Integration Ideas

| Integration | Description |
|-------------|-------------|
| VS Code extension | Edit configs with IntelliSense |
| GitHub Action | Validate configs in CI |
| Pre-commit hook | Validate before commit |
| Claude Code plugin | First-party integration |

---

## 18. Open Questions

### Resolved

| Question | Decision |
|----------|----------|
| Language | Rust |
| Config format | YAML |
| Script interpreters | Bun for .ts/.js, python3 for .py, bash for .sh |
| Distribution | GitHub Releases + cargo install |

### Open

| Question | Options | Notes |
|----------|---------|-------|
| Project name | `cch`, `claude-hooks`, `context-hooks` | Need to check availability |
| License | MIT, Apache 2.0 | Leaning MIT |
| Config file name | `hooks.yaml`, `context.yaml`, `cch.yaml` | Need consistency |
| crates.io | Publish or GitHub-only? | Can add later |
| Homebrew | Create tap? | Can add later |

---

## 19. Appendix

### 19.1 Rust Dependencies

```toml
[package]
name = "cch"
version = "0.1.0"
edition = "2021"
description = "Context injection and validation for Claude Code hooks"
license = "MIT"
repository = "https://github.com/user/cch"
readme = "README.md"
keywords = ["claude", "hooks", "ai", "developer-tools"]
categories = ["command-line-utilities", "development-tools"]

[dependencies]
# CLI
clap = { version = "4", features = ["derive", "env"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"

# Patterns
glob = "0.3"
regex = "1"

# Error handling
thiserror = "1"
anyhow = "1"

# File system
dirs = "5"

# Output
colored = "2"

# Process execution
which = "6"

[dev-dependencies]
tempfile = "3"
assert_cmd = "2"
predicates = "3"
assert_fs = "1"

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### 19.2 Event JSON Schemas

**PreToolUse / PostToolUse / PermissionRequest:**
```json
{
  "tool_name": "Bash",
  "tool_input": {
    "command": "git commit -m 'feat: add feature'",
    "description": "Commit changes"
  },
  "session_id": "abc123"
}
```

**UserPromptSubmit:**
```json
{
  "prompt": "How do I set up the database?",
  "session_id": "abc123"
}
```

**SessionStart / SessionEnd / Setup:**
```json
{
  "session_id": "abc123"
}
```

**Stop:**
```json
{
  "session_id": "abc123",
  "stop_reason": "end_turn"
}
```

**SubagentStop:**
```json
{
  "session_id": "abc123",
  "subagent_id": "subagent-456",
  "result": "success"
}
```

**Notification:**
```json
{
  "session_id": "abc123",
  "message": "Task completed",
  "type": "info"
}
```

### 19.3 Response JSON Schema

```json
{
  "continue": true,
  "context": "Optional injected context...",
  "reason": "Optional block reason (when continue=false)"
}
```

### 19.4 References

- [Claude Code Hooks Documentation](https://docs.anthropic.com/en/docs/claude-code/hooks)
- [Clap Documentation](https://docs.rs/clap)
- [Serde YAML Documentation](https://docs.rs/serde_yaml)
- [cargo-dist Documentation](https://opensource.axo.dev/cargo-dist/)

---

---

# CCH Binary PRD - Addendum

**Document:** Addendum to Claude Context Hooks (CCH) Binary PRD  
**Version:** 1.0  
**Date:** January 21, 2025  
**Related Documents:**
- CCH Binary PRD (main document)
- CCH Skill PRD
- CCH Unified Reference PRD

---

## Overview

This addendum extends the CCH Binary PRD with specifications for:

1. Logging and Telemetry
2. CLI Contract (for skill integration)
3. Version Compatibility
4. Exit Codes and Error Handling
5. Binary Distribution and Integrity

These specifications ensure reliable integration between the CCH binary and the CCH skill, and provide the foundation for observability and auditability.

---

## 1. Logging and Telemetry

### 1.1 Log Levels

| Level | Content | Use Case |
|-------|---------|----------|
| `error` | Failures, exceptions, invalid configs | Production monitoring |
| `warn` | Blocked actions, validation warnings | Troubleshooting |
| `info` | All hook events with outcomes | Default operation |
| `debug` | Full matcher tracing, timing details | Development/debugging |
| `trace` | Everything including stdin/stdout | Deep debugging |

Default level: `info`

### 1.2 Log Locations

| Source | Location |
|--------|----------|
| Default | `~/.claude/logs/cch.log` |
| Environment override | `CCH_LOG_PATH` |
| CLI override | `--log-path <path>` |
| Disable logging | `--no-log` or `CCH_LOG=false` |

**Resolution order:**
1. `--log-path` (highest priority)
2. `CCH_LOG_PATH` environment variable
3. Default location

**Directory creation:**
- CCH creates log directory if it doesn't exist
- Fails gracefully if unable to create (logs to stderr only)

### 1.3 Log Rotation

| Setting | Default | Override |
|---------|---------|----------|
| Rotation | Daily | `CCH_LOG_ROTATE=hourly\|daily\|size:10MB` |
| Retention | 7 days | `CCH_LOG_RETAIN=<days>` |
| Max size | 100MB | `CCH_LOG_MAX_SIZE=<bytes>` |

### 1.4 Log Format

**Default:** JSON Lines (one JSON object per line)

**Alternative:** `--log-format=pretty` for human-readable output

#### JSON Lines Schema (v1)

```json
{
  "v": 1,
  "ts": "2025-01-21T10:30:00.123Z",
  "level": "info",
  "event": "PreToolUse",
  "session_id": "abc123",
  "data": {
    "tool": "Bash",
    "file": null,
    "command": "git commit -m 'feat: add feature'",
    "rules_evaluated": 12,
    "rules_matched": [
      {
        "name": "git-commit-guidelines",
        "action": "inject",
        "source": ".claude/hooks.yaml:23"
      },
      {
        "name": "block-wip",
        "action": "continue",
        "source": ".claude/hooks.yaml:45"
      }
    ],
    "scripts_executed": [],
    "outcome": "continue",
    "context_injected": true,
    "context_length": 1234,
    "blocked": false,
    "block_reason": null
  },
  "timing": {
    "total_ms": 3,
    "config_load_ms": 1,
    "matching_ms": 1,
    "actions_ms": 1
  }
}
```

#### Schema Fields

| Field | Type | Description |
|-------|------|-------------|
| `v` | `number` | Log schema version (currently 1) |
| `ts` | `string` | ISO8601 timestamp with milliseconds |
| `level` | `string` | Log level: error, warn, info, debug, trace |
| `event` | `string` | Hook event type |
| `session_id` | `string` | Claude Code session identifier |
| `data` | `object` | Event-specific data (see below) |
| `timing` | `object` | Performance metrics |

#### Event Data Fields

| Field | Type | Description |
|-------|------|-------------|
| `tool` | `string\|null` | Tool name (if applicable) |
| `file` | `string\|null` | File path (if applicable) |
| `command` | `string\|null` | Command string (if Bash) |
| `rules_evaluated` | `number` | Total rules checked |
| `rules_matched` | `array` | Rules that matched |
| `scripts_executed` | `array` | Validator scripts run |
| `outcome` | `string` | "continue" or "block" |
| `context_injected` | `boolean` | Whether context was added |
| `context_length` | `number` | Bytes of context injected |
| `blocked` | `boolean` | Whether action was blocked |
| `block_reason` | `string\|null` | Block message if blocked |

#### Script Execution Log Entry

```json
{
  "script": ".claude/validators/check.py",
  "interpreter": "python3",
  "exit_code": 0,
  "duration_ms": 45,
  "stdout_length": 0,
  "stderr_length": 0,
  "timed_out": false
}
```

### 1.5 Log Query Support

CCH provides a built-in log query command:

```bash
# Show recent events
cch logs

# Show last N events
cch logs --last 10

# Filter by event type
cch logs --event PreToolUse

# Filter by outcome
cch logs --blocked

# Filter by time range
cch logs --since "1 hour ago"
cch logs --since 2025-01-21T10:00:00

# Filter by rule
cch logs --rule block-wip

# Output as JSON (for piping to jq)
cch logs --json

# Follow mode (like tail -f)
cch logs --follow
```

### 1.6 Privacy and Security

| Principle | Implementation |
|-----------|----------------|
| Local only | Logs never transmitted externally |
| No secrets | Sensitive env vars redacted in logs |
| User control | User can disable logging entirely |
| Readable | Logs owned by user, standard permissions |

**Redaction rules:**
- Environment variables matching `*SECRET*`, `*PASSWORD*`, `*TOKEN*`, `*KEY*`, `*CREDENTIAL*` are logged as `[REDACTED]`
- File contents are never logged (only paths)
- Command arguments matching secret patterns are redacted

---

## 2. CLI Contract

This section defines the CLI interface that the CCH Skill (and other integrations) can rely on.

### 2.1 Stability Guarantees

| Category | Guarantee |
|----------|-----------|
| **Stable** | Will not change without major version bump |
| **Beta** | May change in minor versions with deprecation warning |
| **Experimental** | May change at any time |

### 2.2 Stable Commands

These commands are part of the stable API contract:

| Command | Purpose | Stability |
|---------|---------|-----------|
| `cch --version` | Print version | Stable |
| `cch --help` | Print help | Stable |
| `cch init` | Initialize configuration | Stable |
| `cch validate` | Validate configuration | Stable |
| `cch install --project` | Install to project settings | Stable |
| `cch install --user` | Install to user settings | Stable |
| `cch uninstall --project` | Remove from project settings | Stable |
| `cch uninstall --user` | Remove from user settings | Stable |
| `cch pre-tool-use` | Handle PreToolUse event | Stable |
| `cch post-tool-use` | Handle PostToolUse event | Stable |
| `cch permission` | Handle PermissionRequest event | Stable |
| `cch user-prompt` | Handle UserPromptSubmit event | Stable |
| `cch session-start` | Handle SessionStart event | Stable |
| `cch session-end` | Handle SessionEnd event | Stable |
| `cch stop` | Handle Stop event | Stable |
| `cch notification` | Handle Notification event | Stable |
| `cch subagent-stop` | Handle SubagentStop event | Stable |
| `cch pre-compact` | Handle PreCompact event | Stable |
| `cch setup` | Handle Setup event | Stable |
| `cch logs` | Query execution logs | Stable |

### 2.3 Beta Commands

| Command | Purpose | Stability |
|---------|---------|-----------|
| `cch debug <event>` | Debug event handling with verbose trace | Beta |
| `cch config --show` | Show resolved configuration | Beta |
| `cch config --path` | Show config file path | Beta |

### 2.4 Version Output Format

```bash
$ cch --version
cch 1.2.3
```

For machine parsing:

```bash
$ cch --version --json
{
  "name": "cch",
  "version": "1.2.3",
  "api_version": 1,
  "log_schema_version": 1,
  "config_schema_version": 1
}
```

| Field | Description |
|-------|-------------|
| `name` | Binary name |
| `version` | Semantic version (X.Y.Z) |
| `api_version` | CLI contract version |
| `log_schema_version` | Log format version |
| `config_schema_version` | YAML config schema version |

### 2.5 Configuration Query

The skill needs to discover CCH configuration:

```bash
# Show all config values
$ cch config --show
config_path: /Users/me/project/.claude/hooks.yaml
log_path: /Users/me/.claude/logs/cch.log
log_level: info
...

# Show as JSON
$ cch config --show --json
{
  "config_path": "/Users/me/project/.claude/hooks.yaml",
  "log_path": "/Users/me/.claude/logs/cch.log",
  "log_level": "info",
  ...
}

# Show specific value
$ cch config --show log-path
/Users/me/.claude/logs/cch.log
```

---

## 3. Exit Codes

### 3.1 Standard Exit Codes

| Code | Meaning | Usage |
|------|---------|-------|
| 0 | Success | Normal operation |
| 1 | General error | Unspecified failure |
| 2 | Configuration error | Invalid YAML, missing file |
| 3 | Validation error | Config validation failed |
| 4 | Script error | Validator script failed to execute |
| 5 | Timeout | Script exceeded timeout |
| 10 | Installation error | Failed to modify settings |
| 126 | Command not executable | Script permission denied |
| 127 | Command not found | Script or interpreter not found |

### 3.2 Event Handler Exit Codes

For event handler commands (`pre-tool-use`, etc.), exit codes are NOT used to signal block/continue. The JSON response determines that.

Exit codes only indicate operational errors:

| Code | Meaning |
|------|---------|
| 0 | Handler completed (check JSON for outcome) |
| 1+ | Handler failed (error in CCH itself) |

---

## 4. Version Compatibility

### 4.1 Semantic Versioning

CCH follows semantic versioning (MAJOR.MINOR.PATCH):

| Change Type | Version Bump | Example |
|-------------|--------------|---------|
| Breaking CLI changes | MAJOR | 1.0.0 → 2.0.0 |
| New features, backward compatible | MINOR | 1.0.0 → 1.1.0 |
| Bug fixes | PATCH | 1.0.0 → 1.0.1 |

### 4.2 API Version

Separate from the binary version, the API version tracks the CLI contract:

| API Version | Binary Versions | Notes |
|-------------|-----------------|-------|
| 1 | 1.0.0 - 1.x.x | Initial stable API |
| 2 | 2.0.0+ | Breaking changes |

The skill checks API version, not binary version:

```bash
$ cch --version --json | jq '.api_version'
1
```

### 4.3 Configuration Schema Version

The `version` field in `hooks.yaml`:

```yaml
version: "1"
```

| Config Version | CCH Versions | Notes |
|----------------|--------------|-------|
| "1" | 1.0.0+ | Initial schema |

CCH validates config version and reports clear errors for incompatible configs.

### 4.4 Deprecation Policy

- Deprecated features emit warnings for at least 2 minor versions
- Deprecated features removed only in major versions
- Warnings include migration guidance

---

## 5. Binary Distribution and Integrity

### 5.1 Release Artifacts

Each release includes:

| Artifact | Description |
|----------|-------------|
| `cch-aarch64-apple-darwin.tar.gz` | macOS Apple Silicon |
| `cch-x86_64-apple-darwin.tar.gz` | macOS Intel |
| `cch-x86_64-unknown-linux-gnu.tar.gz` | Linux x86_64 (glibc) |
| `cch-x86_64-unknown-linux-musl.tar.gz` | Linux x86_64 (musl/Alpine) |
| `cch-aarch64-unknown-linux-gnu.tar.gz` | Linux ARM64 |
| `cch-x86_64-pc-windows-msvc.zip` | Windows x86_64 |
| `checksums.sha256` | SHA256 checksums for all artifacts |
| `checksums.sha256.sig` | GPG signature of checksums (optional) |

### 5.2 Checksum Verification

```bash
# Download checksums
curl -LO https://github.com/user/cch/releases/download/v1.2.3/checksums.sha256

# Verify artifact
sha256sum -c checksums.sha256 --ignore-missing
```

### 5.3 Installation Audit Trail

When installing CCH, the skill should create an audit record:

**Location:** `.claude/cch/install.json`

```json
{
  "version": "1.2.3",
  "api_version": 1,
  "installed_at": "2025-01-21T10:30:00Z",
  "installed_by": "cch-skill",
  "source": {
    "type": "github_release",
    "url": "https://github.com/user/cch/releases/download/v1.2.3/cch-aarch64-apple-darwin.tar.gz",
    "sha256": "abc123..."
  },
  "location": ".claude/bin/cch",
  "platform": {
    "os": "darwin",
    "arch": "aarch64"
  }
}
```

### 5.4 Binary Resolution Order

When the skill executes CCH commands, it resolves the binary in this order:

1. **Project-pinned binary** (highest priority)
   - `.claude/bin/cch`
   - Ensures reproducible behavior per project

2. **User PATH**
   - `which cch` / `where cch`
   - Standard installation

3. **Configured override**
   - `.claude/cch/config.json` → `cchBinaryPath`
   - Escape hatch for non-standard setups

If none found, the skill offers to install.

---

## 6. Error Messages

### 6.1 Error Message Format

All errors follow a consistent format:

```
error: <brief description>

<detailed explanation>

help: <suggested fix>
```

Example:

```
error: configuration file not found

Looked for hooks.yaml in:
  - /Users/me/project/.claude/hooks.yaml
  - /Users/me/.claude/hooks.yaml

help: Run 'cch init' to create a configuration file
```

### 6.2 Error Codes

Errors include machine-readable codes:

```bash
$ cch validate 2>&1
error[E002]: invalid YAML syntax

  --> .claude/hooks.yaml:15:3
   |
15 |   - name missing-colon
   |     ^^^^^^^^^^^^^^^^^^
   |
   = help: Add a colon after 'name'

$ echo $?
2
```

Error code ranges:

| Range | Category |
|-------|----------|
| E001-E099 | Configuration errors |
| E100-E199 | Validation errors |
| E200-E299 | Runtime errors |
| E300-E399 | Script errors |

---

## 7. Performance Requirements

### 7.1 Benchmarks

| Operation | Target | Maximum |
|-----------|--------|---------|
| Cold start | < 5ms | 10ms |
| Config parse (10KB) | < 2ms | 5ms |
| Rule matching (50 rules) | < 1ms | 3ms |
| Total (no scripts) | < 10ms | 20ms |
| Log write | < 1ms | 2ms |

### 7.2 Measurement

CCH includes timing in debug output:

```bash
$ echo '{}' | cch debug pre-tool-use
[DEBUG] Timing:
  config_load: 1.2ms
  rule_match: 0.8ms
  actions: 0.5ms
  log_write: 0.3ms
  total: 2.8ms
```

---

## 8. Security Considerations

### 8.1 Script Execution

| Control | Implementation |
|---------|----------------|
| No shell expansion | Scripts invoked directly, not via shell |
| Timeout | Default 30s, configurable |
| Working directory | Set to project root |
| Environment | Inherited, with CCH_* additions |

### 8.2 File Access

| Access | Scope |
|--------|-------|
| Read config | Project `.claude/` and user `~/.claude/` |
| Read context | Paths specified in config |
| Read validators | Paths specified in config |
| Write logs | `~/.claude/logs/` |
| Write settings | `.claude/settings.json` or `~/.claude/settings.json` |

### 8.3 No Network Access

CCH binary makes no network requests. All operations are local.

---

## Summary

This addendum specifies:

1. **Logging:** JSON Lines format with configurable levels, rotation, and query support
2. **CLI Contract:** Stable commands the skill can rely on
3. **Exit Codes:** Consistent error signaling
4. **Version Compatibility:** Semantic versioning with API version tracking
5. **Distribution:** Checksums, signatures, audit trails
6. **Performance:** Benchmarks and measurement
7. **Security:** Script sandboxing, file access controls

These specifications enable:

- Reliable skill integration
- Troubleshooting and debugging
- Audit and compliance
- Long-term maintainability

---

*End of CCH Binary PRD Addendum*