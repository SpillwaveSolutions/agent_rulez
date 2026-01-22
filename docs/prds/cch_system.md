# CCH Unified Reference PRD

**Document:** Claude Context Hooks - Unified System Reference  
**Version:** 1.0  
**Date:** January 21, 2025  

---

## Document Map

This document provides a unified view of the Claude Context Hooks (CCH) system and references the detailed specifications in companion documents.

| Document | Purpose | Scope |
|----------|---------|-------|
| **CCH Binary PRD** | Runtime hook handler | Rust binary specification |
| **CCH Binary PRD Addendum** | Logging, CLI contract, distribution | Binary observability & integration |
| **CCH Skill PRD** | Intelligent installer & configurator | Agentic skill specification |
| **CCH Skill PRD Addendum** | Binary integration, telemetry, provenance | Skill-binary coordination |
| **CCH Unified Reference PRD** (this doc) | System overview, contracts, flows | Cross-cutting concerns |

---

## 1. System Overview

### 1.1 What is CCH?

Claude Context Hooks (CCH) is a two-part system for controlling and customizing Claude Code's behavior:

```
┌─────────────────────────────────────────────────────────────────┐
│                        CCH System                                │
│                                                                  │
│  ┌─────────────────────┐         ┌─────────────────────┐       │
│  │     CCH Binary      │         │     CCH Skill       │       │
│  │       (Rust)        │         │     (Agentic)       │       │
│  ├─────────────────────┤         ├─────────────────────┤       │
│  │ • Runtime execution │         │ • Installation      │       │
│  │ • Hook handling     │◄───────►│ • Configuration     │       │
│  │ • Rule matching     │         │ • Analysis          │       │
│  │ • Context injection │         │ • Recommendations   │       │
│  │ • Logging           │         │ • Troubleshooting   │       │
│  └─────────────────────┘         └─────────────────────┘       │
│           │                                │                    │
│           ▼                                ▼                    │
│  ┌─────────────────────┐         ┌─────────────────────┐       │
│  │  .claude/hooks.yaml │         │  .claude/cch/       │       │
│  │  .claude/context/   │         │    install.json     │       │
│  │  .claude/validators/│         │    config.json      │       │
│  └─────────────────────┘         └─────────────────────┘       │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 Design Philosophy

| Principle | Implementation |
|-----------|----------------|
| **Separation of concerns** | Binary handles runtime, Skill handles intelligence |
| **Deterministic enforcement** | Binary executes rules exactly as specified |
| **Intelligent assistance** | Skill analyzes, recommends, explains |
| **Local-first** | No network dependencies, all data stays local |
| **Auditable** | Every decision logged, every rule traced to source |
| **Extensible** | Custom validators in any language |

### 1.3 Analogous Systems

| CCH Component | Analogous To |
|---------------|--------------|
| CCH Binary | Terraform CLI, Git, OPA Engine |
| CCH Skill | Terraform Cloud, GitHub, Policy Authoring |
| hooks.yaml | terraform.tf, .rego files |
| Validators | Sentinel policies, Rego rules |

---

## 2. Component Responsibilities

### 2.1 CCH Binary

**Role:** Deterministic runtime hook handler

**Responsibilities:**

| Responsibility | Description |
|----------------|-------------|
| Event handling | Receive hook events from Claude Code |
| Config loading | Parse and validate hooks.yaml |
| Rule matching | Evaluate rules against events |
| Action execution | Inject context, block operations, run scripts |
| Logging | Record all decisions and outcomes |
| Output | Return JSON response to Claude Code |

**Does NOT:**
- Make recommendations
- Analyze codebases
- Generate configurations
- Require user interaction

**Reference:** CCH Binary PRD, CCH Binary PRD Addendum

### 2.2 CCH Skill

**Role:** Intelligent installation and configuration assistant

**Responsibilities:**

| Responsibility | Description |
|----------------|-------------|
| Binary management | Install, update, verify CCH binary |
| Project analysis | Discover skills, parse CLAUDE.md, analyze codebase |
| Recommendations | Generate rule suggestions with explanations |
| Configuration | Create hooks.yaml, context files, validators |
| Troubleshooting | Analyze logs, explain rule behavior |
| Audit | Track provenance, maintain installation records |

**Does NOT:**
- Handle runtime hook events
- Execute during normal Claude Code operation
- Run continuously in background

**Reference:** CCH Skill PRD, CCH Skill PRD Addendum

---

## 3. Integration Contracts

### 3.1 Binary ↔ Skill Contract

The skill depends on the binary implementing specific interfaces.

#### 3.1.1 Required CLI Commands

| Command | Purpose | Contract |
|---------|---------|----------|
| `cch --version --json` | Version info | Returns JSON with version, api_version |
| `cch init` | Create config | Creates .claude/hooks.yaml |
| `cch validate` | Check config | Exit 0 if valid, non-zero with errors |
| `cch install --project` | Install hooks | Modifies .claude/settings.json |
| `cch install --user` | Install hooks | Modifies ~/.claude/settings.json |
| `cch uninstall` | Remove hooks | Removes CCH from settings |
| `cch logs` | Query logs | Returns log entries |
| `cch config --show` | Show config | Returns configuration values |
| `cch debug <event>` | Debug matching | Verbose trace output |

#### 3.1.2 Version Contract

```json
// cch --version --json
{
  "name": "cch",
  "version": "1.2.3",
  "api_version": 1,
  "log_schema_version": 1,
  "config_schema_version": 1
}
```

| Field | Purpose |
|-------|---------|
| `version` | Binary version (semver) |
| `api_version` | CLI contract version (skill checks this) |
| `log_schema_version` | Log format version |
| `config_schema_version` | YAML schema version |

#### 3.1.3 Compatibility Matrix

| Skill Version | Minimum Binary API | Minimum Binary Version |
|---------------|-------------------|------------------------|
| 1.x | 1 | 1.0.0 |

### 3.2 Binary ↔ Claude Code Contract

The binary integrates with Claude Code's hook system.

#### 3.2.1 Event Input (stdin)

```json
{
  "tool_name": "Bash",
  "tool_input": {
    "command": "git commit -m 'feat: thing'",
    "description": "Commit changes"
  },
  "session_id": "abc123"
}
```

#### 3.2.2 Event Output (stdout)

**Continue:**
```json
{
  "continue": true,
  "context": "# Injected context\n..."
}
```

**Block:**
```json
{
  "continue": false,
  "reason": "Operation blocked: reason"
}
```

#### 3.2.3 Hook Registration

The binary registers itself in Claude Code settings:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "*",
        "hooks": [{"type": "command", "command": "cch pre-tool-use"}]
      }
    ]
  }
}
```

### 3.3 Validator Script Contract

Scripts executed by the binary must follow this interface.

#### 3.3.1 Input

| Source | Content |
|--------|---------|
| stdin | Full event JSON |
| `$CCH_EVENT` | Event type |
| `$CCH_TOOL` | Tool name |
| `$CCH_FILE` | File path (if applicable) |
| `$CCH_COMMAND` | Command (if Bash) |
| `$CCH_CONFIG_DIR` | Config directory |
| `$CCH_PROJECT_DIR` | Project root |

#### 3.3.2 Output

| Exit Code | Meaning |
|-----------|---------|
| 0 | Continue (allow) |
| 1-255 | Block (deny) |

| Stream | Usage |
|--------|-------|
| stdout | Additional context to inject |
| stderr | Block message (if exit non-zero) |

#### 3.3.3 Interpreter Detection

| Extension | Interpreter |
|-----------|-------------|
| `.py` | `python3` |
| `.ts` | `bun` |
| `.js` | `bun` |
| `.sh` | `bash` |
| (executable) | Direct |

---

## 4. Data Flow

### 4.1 Setup Flow (Skill)

```
┌─────────────────────────────────────────────────────────────────┐
│                     Setup Flow (Skill)                           │
└─────────────────────────────────────────────────────────────────┘

User: "Set up hooks for this project"
                │
                ▼
┌───────────────────────────────────┐
│ 1. Environment Check              │
│    • Detect OS/arch               │
│    • Check CCH binary             │
│    • Verify version               │
│    • Install if missing           │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 2. Project Analysis               │
│    • Scan .claude/skills/         │
│    • Parse CLAUDE.md              │
│    • Analyze codebase             │
│    • Check existing config        │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 3. Generate Recommendations       │
│    • Skill trigger rules          │
│    • Rule enforcement guards      │
│    • Safety defaults              │
│    • Confidence classification    │
│    • Conflict detection           │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 4. User Review                    │
│    • Present recommendations      │
│    • Explain each rule            │
│    • Get approval/modifications   │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 5. Generate Configuration         │
│    • Create hooks.yaml            │
│    • Create context files         │
│    • Create validators            │
│    • Record provenance            │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 6. Install                        │
│    • cch validate                 │
│    • cch install --project        │
│    • Update audit trail           │
└───────────────────────────────────┘
                │
                ▼
            Complete
```

### 4.2 Runtime Flow (Binary)

```
┌─────────────────────────────────────────────────────────────────┐
│                    Runtime Flow (Binary)                         │
└─────────────────────────────────────────────────────────────────┘

Claude Code: PreToolUse event
                │
                ▼
┌───────────────────────────────────┐
│ 1. Receive Event                  │
│    • Parse JSON from stdin        │
│    • Extract tool, file, command  │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 2. Load Configuration             │
│    • Find hooks.yaml              │
│    • Parse YAML                   │
│    • Validate schema              │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 3. Match Rules                    │
│    • Filter by event type         │
│    • Evaluate each rule           │
│    • Check all matchers           │
│    • Collect matching rules       │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 4. Execute Actions                │
│    For each matching rule:        │
│    • inject → load markdown       │
│    • run → execute script         │
│    • block_if_match → check       │
│    • action:block → set flag      │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 5. Log Decision                   │
│    • Write to log file            │
│    • Include all details          │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 6. Return Response                │
│    • Aggregate context            │
│    • Determine continue/block     │
│    • Output JSON to stdout        │
└───────────────────────────────────┘
                │
                ▼
Claude Code: Process response
```

### 4.3 Troubleshooting Flow (Skill)

```
┌─────────────────────────────────────────────────────────────────┐
│                 Troubleshooting Flow (Skill)                     │
└─────────────────────────────────────────────────────────────────┘

User: "Why didn't my rule block that commit?"
                │
                ▼
┌───────────────────────────────────┐
│ 1. Identify Issue                 │
│    • Parse user question          │
│    • Determine rule/event         │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 2. Discover Log Location          │
│    • cch config --show log-path   │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 3. Query Logs                     │
│    • Find relevant events         │
│    • Parse rule matches           │
│    • Check outcomes               │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 4. Analyze                        │
│    • Did rule match? Why/why not? │
│    • What was the action?         │
│    • What was the outcome?        │
└───────────────────────────────────┘
                │
                ▼
┌───────────────────────────────────┐
│ 5. Explain                        │
│    • Show relevant log entry      │
│    • Explain matching logic       │
│    • Identify issue               │
│    • Suggest fix                  │
└───────────────────────────────────┘
                │
                ▼
            Resolution
```

---

## 5. File System Layout

### 5.1 Project Structure

```
project/
├── .claude/
│   ├── hooks.yaml              # Hook configuration (primary config)
│   ├── settings.json           # Claude Code settings (hooks registered here)
│   ├── bin/
│   │   └── cch                 # Project-pinned binary (optional)
│   ├── cch/
│   │   ├── install.json        # Installation audit trail
│   │   └── config.json         # CCH configuration overrides (optional)
│   ├── context/
│   │   ├── explain-command.md  # Injection templates
│   │   ├── commit-guidelines.md
│   │   └── ...
│   ├── validators/
│   │   ├── no-console-log.py   # Custom validators
│   │   ├── aws-safety.ts
│   │   └── ...
│   └── skills/
│       ├── aws-cdk/
│       │   └── SKILL.md
│       └── react/
│           └── SKILL.md
├── CLAUDE.md                    # Project rules and conventions
└── ...
```

### 5.2 User-Level Structure

```
~/.claude/
├── settings.json               # User-level Claude Code settings
├── hooks.yaml                  # User-level hooks (lower priority)
├── logs/
│   └── cch.log                 # CCH execution logs
└── cch/
    └── config.json             # User-level CCH config
```

### 5.3 Configuration Precedence

| Priority | Location | Scope |
|----------|----------|-------|
| 1 (highest) | `.claude/hooks.yaml` | Project-specific |
| 2 | `~/.claude/hooks.yaml` | User defaults |

Rules from both files are merged. Project rules take precedence for conflicts.

---

## 6. Schema Reference

### 6.1 hooks.yaml Schema

```yaml
version: "1"                    # Required, schema version

# Optional metadata (for tooling)
_metadata:
  generated_by: string
  generated_at: datetime
  sources: array

# Rules organized by event
PreToolUse:
  - name: string                # Required
    enabled_when:               # Optional
      env: string
      equals|matches|exists: value
    tools: [string]             # Optional, default "*"
    extensions: [string]        # Optional
    directories: [string]       # Optional
    operations: [string]        # Optional
    command_match: string       # Optional, regex
    inject: string              # Optional, file path
    run: string|object          # Optional, script path
    action: "block"             # Optional
    message: string             # Optional
    block_if_match:             # Optional
      - pattern: string
        message: string

PostToolUse:
  # Same structure as PreToolUse minus blocking

PermissionRequest:
  # Same as PreToolUse plus:
  - require_fields:
      - name: string
        description: string

UserPromptSubmit:
  - name: string
    prompt_match: string        # Regex
    inject: string
    run: string

SessionStart:
SessionEnd:
Setup:
Notification:
Stop:
SubagentStop:
PreCompact:
  - name: string
    inject: string
    run: string
```

### 6.2 Log Entry Schema

```json
{
  "v": 1,
  "ts": "ISO8601",
  "level": "info|warn|error|debug",
  "event": "PreToolUse|PostToolUse|...",
  "session_id": "string",
  "data": {
    "tool": "string|null",
    "file": "string|null",
    "command": "string|null",
    "rules_evaluated": "number",
    "rules_matched": [
      {
        "name": "string",
        "action": "string",
        "source": "string"
      }
    ],
    "scripts_executed": [
      {
        "script": "string",
        "exit_code": "number",
        "duration_ms": "number"
      }
    ],
    "outcome": "continue|block",
    "context_injected": "boolean",
    "blocked": "boolean",
    "block_reason": "string|null"
  },
  "timing": {
    "total_ms": "number",
    "config_load_ms": "number",
    "matching_ms": "number",
    "actions_ms": "number"
  }
}
```

### 6.3 install.json Schema

```json
{
  "schema_version": 1,
  "binary": {
    "version": "string",
    "api_version": "number",
    "installed_at": "datetime",
    "installed_by": "string",
    "install_mode": "project|user",
    "source": {
      "type": "github_release",
      "url": "string",
      "sha256": "string"
    },
    "location": "string",
    "platform": {
      "os": "string",
      "arch": "string"
    }
  },
  "config": {
    "generated_at": "datetime",
    "generated_by": "string",
    "sources_analyzed": ["array"],
    "rules_generated": "number"
  },
  "history": [
    {
      "action": "string",
      "timestamp": "datetime",
      "details": "object"
    }
  ]
}
```

---

## 7. Security Model

### 7.1 Trust Boundaries

```
┌─────────────────────────────────────────────────────────────────┐
│                      Trust Boundaries                            │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│           Trusted (User-Controlled)      │
│                                          │
│  • hooks.yaml (user writes)             │
│  • Validator scripts (user writes)      │
│  • Context markdown (user writes)       │
│  • CLAUDE.md (user writes)              │
│  • CCH binary (verified checksum)       │
└─────────────────────────────────────────┘
              │
              │ Input
              ▼
┌─────────────────────────────────────────┐
│           CCH Binary (Sandbox)           │
│                                          │
│  • Parses trusted config                │
│  • Executes trusted scripts             │
│  • No network access                    │
│  • Limited file access                  │
│  • Logs all decisions                   │
└─────────────────────────────────────────┘
              │
              │ Output
              ▼
┌─────────────────────────────────────────┐
│           Claude Code                    │
│                                          │
│  • Receives JSON response               │
│  • Applies continue/block               │
│  • Injects context                      │
└─────────────────────────────────────────┘
```

### 7.2 Security Properties

| Property | Guarantee |
|----------|-----------|
| No network | Binary makes no network requests |
| Local execution | All processing on local machine |
| User-controlled | Only user-written configs/scripts execute |
| Logged | Every decision recorded |
| Reversible | Uninstall returns to pre-CCH state |
| Verified | Binary checksum verified on install |

### 7.3 Secret Handling

| Data | Handling |
|------|----------|
| Environment variables | Sensitive patterns redacted in logs |
| File contents | Never logged (paths only) |
| Commands | Logged, with sensitive args redacted |

---

## 8. Performance Requirements

### 8.1 Binary Performance

| Operation | Target | Maximum |
|-----------|--------|---------|
| Cold start | < 5ms | 10ms |
| Config parse | < 2ms | 5ms |
| Rule matching | < 1ms | 3ms |
| Total (no scripts) | < 10ms | 20ms |

### 8.2 Skill Performance

No strict requirements (interactive use), but:

| Operation | Expected |
|-----------|----------|
| Version check | < 1s |
| Project analysis | < 10s |
| Config generation | < 5s |
| Installation | < 5s |

---

## 9. Versioning Strategy

### 9.1 Version Types

| Version | Controls | Location |
|---------|----------|----------|
| Binary version | Feature availability | `cch --version` |
| API version | CLI contract | `cch --version --json` |
| Config schema | YAML format | `hooks.yaml: version` |
| Log schema | Log format | Log entry `v` field |
| Skill version | Skill capabilities | SKILL.md metadata |

### 9.2 Compatibility Rules

| When API changes... | Binary version | Migration |
|---------------------|----------------|-----------|
| Backward compatible additions | MINOR bump | None required |
| Breaking changes | MAJOR bump | Migration guide |

| When config schema changes... | Config version | Migration |
|-------------------------------|----------------|-----------|
| New optional fields | Same version | None required |
| Breaking changes | New version | Converter provided |

---

## 10. Glossary

| Term | Definition |
|------|------------|
| **CCH** | Claude Context Hooks - the overall system |
| **CCH Binary** | The Rust executable that handles hook events |
| **CCH Skill** | The agentic skill that helps configure CCH |
| **Hook** | A Claude Code lifecycle event |
| **Rule** | A configuration entry that matches events and specifies actions |
| **Matcher** | A condition that determines if a rule applies |
| **Action** | What happens when a rule matches (inject, block, run) |
| **Validator** | A script that performs custom validation |
| **Context** | Markdown content injected into Claude's context |
| **Provenance** | The source and history of a rule |

---

## 11. Document References

### 11.1 Primary Documents

| Document | Content |
|----------|---------|
| CCH Binary PRD | Full binary specification |
| CCH Skill PRD | Full skill specification |

### 11.2 Addenda

| Document | Content |
|----------|---------|
| CCH Binary PRD Addendum | Logging, CLI contract, distribution |
| CCH Skill PRD Addendum | Binary integration, telemetry, provenance |

### 11.3 Cross-Reference Index

| Topic | Primary Doc | Addendum |
|-------|-------------|----------|
| YAML schema | Binary PRD §7 | - |
| Rule matching | Binary PRD §3.3 | - |
| Event handling | Binary PRD §3.2 | - |
| Script execution | Binary PRD §8 | Binary Addendum §3 |
| Logging | - | Binary Addendum §1 |
| CLI contract | - | Binary Addendum §2 |
| Exit codes | - | Binary Addendum §3 |
| Binary distribution | Binary PRD §15 | Binary Addendum §5 |
| Skill triggers | Skill PRD §2 | - |
| Project analysis | Skill PRD §5 | - |
| Recommendations | Skill PRD §6 | - |
| Binary resolution | - | Skill Addendum §1 |
| Log analysis | - | Skill Addendum §2 |
| Rule data model | - | Skill Addendum §3 |
| Confidence handling | - | Skill Addendum §4 |
| Conflict detection | - | Skill Addendum §5 |
| Audit trail | - | Skill Addendum §6 |

---

*End of CCH Unified Reference PRD*