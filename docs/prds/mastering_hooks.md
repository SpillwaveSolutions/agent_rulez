# CCH Skill - Product Requirements Document

## Claude Context Hooks Skill (cch-skill)

**Version:** 1.0
**Last Updated:** January 21, 2025

---

## 1. Overview

### 1.1 Purpose

The CCH Skill is an agentic skill that serves as an intelligent installer, configurator, and manager for the Claude Context Hooks (CCH) system. It bridges the gap between the CCH binary and the user by analyzing projects, discovering existing skills and rules, and generating optimal hook configurations.

### 1.2 Relationship to CCH

| Component | Role |
|-----------|------|
| **CCH Binary (Rust)** | Runtime hook handler - executes on every hook event |
| **CCH Skill (This)** | Setup & configuration assistant - runs on-demand to install and configure |

The skill is NOT the hook handler. It helps you set up the hook handler.

### 1.3 Key Value Propositions

| Value | Description |
|-------|-------------|
| **Zero-config start** | Analyzes project and generates sensible defaults |
| **Skill auto-wiring** | Discovers `.claude/skills/` and creates triggers automatically |
| **Rule enforcement** | Converts CLAUDE.md prose into reliable hook guards |
| **Cross-platform install** | Handles CCH binary installation on any OS |
| **Guided configuration** | Interactive recommendations with explanations |

---

## 2. Triggers

### 2.1 Explicit Triggers

The skill activates when the user explicitly requests hook-related help:

| Trigger Phrase | Intent |
|----------------|--------|
| "set up hooks" | Full project setup |
| "install cch" | Binary installation only |
| "configure hooks" | Configuration management |
| "create hook for..." | Single rule creation |
| "add hook to..." | Add rule to existing config |
| "trigger skill with hook" | Wire skill to hook |
| "enforce rule with hook" | Convert rule to guard |
| "hook configuration" | General hook help |
| "debug hook" | Troubleshooting |
| "why isn't hook working" | Troubleshooting |
| "update hooks" | Modify existing config |
| "remove hook" | Delete rule |
| "show hooks" | Display current config |
| "validate hooks" | Check configuration |
| "test hook" | Simulate event |

### 2.2 Contextual Triggers

The skill MAY be suggested (not auto-activated) when:

| Context | Suggestion |
|---------|------------|
| User creates a new skill | "Want to create a hook to trigger this skill?" |
| User edits CLAUDE.md with MUST/MUST NOT | "Want to enforce this rule with a hook?" |
| User complains about Claude not following rules | "Hooks can enforce rules more reliably than prompts" |
| User asks about blocking operations | "CCH can block operations before they happen" |
| Project has skills but no hooks.yaml | "Found skills without hook triggers. Want to set them up?" |

### 2.3 Non-Triggers

The skill should NOT activate for:

- General coding questions
- Questions about Claude Code itself (not hooks)
- Runtime hook issues (that's CCH binary's job)
- Unrelated configuration files

---

## 3. Capabilities

### 3.1 Installation & Environment

| Capability | Description |
|------------|-------------|
| **Detect OS** | Identify macOS, Linux, Windows |
| **Detect architecture** | Identify x86_64, ARM64/aarch64 |
| **Check CCH installed** | Verify binary exists and is in PATH |
| **Check CCH version** | Get current version, compare to latest |
| **Install CCH** | Download and install appropriate binary |
| **Update CCH** | Upgrade to latest version |
| **Verify installation** | Confirm CCH works correctly |

### 3.2 Project Analysis

| Capability | Description |
|------------|-------------|
| **Discover skills** | Scan `.claude/skills/` recursively for SKILL.md files |
| **Parse skill metadata** | Extract name, description, file patterns, tools |
| **Parse CLAUDE.md** | Extract rules, conventions, restrictions |
| **Analyze codebase** | Identify languages, frameworks, directory structure |
| **Find sensitive files** | Locate .env, secrets, production configs |
| **Detect CI/CD** | Find GitHub Actions, deployment workflows |
| **Check existing hooks** | Read current `.claude/hooks.yaml` if present |
| **Identify conflicts** | Find rules that might conflict |

### 3.3 Recommendation Engine

| Capability | Description |
|------------|-------------|
| **Skill triggers** | Generate PreToolUse rules to inject skill docs |
| **Rule guards** | Convert MUST/MUST NOT to block rules |
| **Rule reminders** | Convert SHOULD/SHOULD NOT to inject rules |
| **Safety guards** | Recommend standard safety rules |
| **Explanation rules** | Suggest PermissionRequest templates |
| **Post-edit hooks** | Recommend linting/testing reminders |
| **Prioritize rules** | Order recommendations by impact |
| **Explain reasoning** | Tell user WHY each rule is recommended |

### 3.4 Configuration Generation

| Capability | Description |
|------------|-------------|
| **Generate hooks.yaml** | Create complete YAML configuration |
| **Generate context markdown** | Create injection templates |
| **Generate validators** | Create Python validation scripts |
| **Merge configurations** | Add to existing hooks.yaml without breaking it |
| **Template selection** | Apply pre-built templates (minimal, standard, strict) |
| **Custom rules** | Create rules from user descriptions |

### 3.5 Validation & Testing

| Capability | Description |
|------------|-------------|
| **Validate config** | Run `cch validate`, report errors |
| **Validate references** | Check all referenced files exist |
| **Validate validators** | Check Python scripts are syntactically valid |
| **Simulate events** | Test what rules match for sample events |
| **Test validators** | Run validator scripts with sample input |
| **Dry-run install** | Preview what installation would do |

### 3.6 Installation & Management

| Capability | Description |
|------------|-------------|
| **Install to project** | Run `cch install --project` |
| **Install to user** | Run `cch install --user` |
| **Uninstall** | Run `cch uninstall` |
| **Backup config** | Save existing config before changes |
| **Rollback** | Restore previous configuration |
| **Show status** | Display current hook configuration |

### 3.7 Troubleshooting

| Capability | Description |
|------------|-------------|
| **Debug matching** | Explain why rule did/didn't match |
| **Debug validators** | Run validator with verbose output |
| **Check installation** | Verify hooks are properly installed |
| **Common issues** | Diagnose frequent problems |
| **Suggest fixes** | Provide actionable solutions |

---

## 4. Workflows

### 4.1 Initial Project Setup

**Trigger:** "Set up hooks for this project"

**Flow:**

```
1. Environment Check
   ├─ Detect OS and architecture
   ├─ Check if CCH binary installed
   │   ├─ If not: offer to install
   │   └─ If outdated: offer to update
   └─ Verify CCH working

2. Project Analysis
   ├─ Scan .claude/skills/ for skill definitions
   ├─ Parse CLAUDE.md for rules
   ├─ Analyze directory structure
   ├─ Identify languages and frameworks
   └─ Check for existing hooks.yaml

3. Generate Recommendations
   ├─ Skill trigger rules (for each discovered skill)
   ├─ Rule enforcement (from CLAUDE.md)
   ├─ Standard safety guards
   └─ Explanation templates

4. Present to User
   ├─ Show recommended hooks.yaml
   ├─ Explain each rule and why
   ├─ Show files to be created
   └─ Ask for approval/modifications

5. Generate Files
   ├─ Create .claude/hooks.yaml
   ├─ Create .claude/context/*.md
   ├─ Create .claude/validators/*.py
   └─ Set executable permissions

6. Validate
   ├─ Run cch validate
   ├─ Report any errors
   └─ Fix if needed

7. Install
   ├─ Run cch install --project
   └─ Verify installation

8. Confirm
   └─ Show summary of what was set up
```

### 4.2 Add Skill Trigger

**Trigger:** "Create a hook to trigger the CDK skill when editing infrastructure files"

**Flow:**

```
1. Parse Request
   ├─ Identify skill: CDK
   ├─ Identify target: infrastructure files
   └─ Identify action: trigger (inject)

2. Locate Skill
   ├─ Find .claude/skills/*/SKILL.md matching "CDK"
   └─ If not found: ask user for path or clarification

3. Analyze Skill
   ├─ Read SKILL.md
   ├─ Extract relevant extensions (.ts, .js)
   └─ Extract relevant directories (cdk/**, infra/**)

4. Generate Rule
   └─ Create PreToolUse rule with:
       - name: cdk-skill-trigger
       - tools: [Edit, Write, str_replace]
       - extensions: [.ts, .js]
       - directories: [cdk/**, infra/**]
       - inject: {path to skill}

5. Present to User
   ├─ Show proposed rule
   ├─ Explain what it does
   └─ Ask for approval

6. Update Config
   ├─ Read existing hooks.yaml (or create new)
   ├─ Add rule to PreToolUse section
   └─ Write updated file

7. Validate & Install
   ├─ Run cch validate
   └─ Run cch install --project
```

### 4.3 Enforce CLAUDE.md Rule

**Trigger:** "Help me enforce the 'no console.log' rule with hooks"

**Flow:**

```
1. Parse CLAUDE.md
   ├─ Find rule about console.log
   └─ Extract: "MUST NOT leave console.log statements in production code"

2. Determine Enforcement Strategy
   ├─ Rule type: MUST NOT → block or warn
   ├─ Scope: TypeScript/JavaScript files
   └─ Method: validator script (pattern too complex for regex)

3. Generate Validator
   └─ Create Python script that:
       - Reads file content from event
       - Checks for console.log
       - Returns exit 1 with message if found

4. Generate Rule
   └─ Create PreToolUse rule with:
       - name: no-console-log
       - tools: [Edit, Write]
       - extensions: [.ts, .tsx, .js, .jsx]
       - run: .claude/validators/no-console-log.py

5. Present to User
   ├─ Show proposed validator script
   ├─ Show proposed rule
   └─ Ask: block (exit 1) or warn (exit 0 + message)?

6. Generate Files
   ├─ Create .claude/validators/no-console-log.py
   └─ Update hooks.yaml

7. Validate & Install
```

### 4.4 Troubleshoot Hook

**Trigger:** "Why isn't my commit hook blocking WIP commits?"

**Flow:**

```
1. Identify Issue
   └─ User expects WIP commits to be blocked, but they're not

2. Load Configuration
   ├─ Read .claude/hooks.yaml
   └─ Find git commit related rules

3. Analyze Rules
   ├─ Find: block_if_match with pattern "WIP"
   └─ Check matcher: command_match: "git commit"

4. Simulate Event
   └─ Create test event:
       {
         "tool_name": "Bash",
         "tool_input": {
           "command": "git commit -m 'WIP: work in progress'"
         }
       }

5. Trace Matching
   ├─ tools: [Bash] ✓ matches
   ├─ command_match: "git commit" ✓ matches
   └─ block_if_match: "WIP" → checking against what?

6. Identify Problem
   └─ Pattern "WIP" is checked against command string
       but user expects it to check commit MESSAGE
       The message is inside -m '...' quotes

7. Explain to User
   ├─ Show the issue clearly
   └─ Explain: block_if_match checks the whole command,
       including the -m flag, so "WIP" SHOULD match

8. Debug Further
   ├─ Run cch debug pre-tool-use with the event
   └─ Check if hooks.yaml is actually installed

9. Suggest Fixes
   ├─ Option A: Check if hooks are installed correctly
   ├─ Option B: Use validator script for more control
   └─ Option C: Adjust pattern to be more specific
```

### 4.5 Interactive Configuration

**Trigger:** "Help me configure hooks"

**Flow:**

```
1. Check Current State
   ├─ CCH installed? Version?
   ├─ hooks.yaml exists?
   └─ How many rules defined?

2. Present Options
   ├─ "Set up from scratch"
   ├─ "Add a new rule"
   ├─ "Modify existing rule"
   ├─ "Remove a rule"
   ├─ "View current configuration"
   └─ "Validate and reinstall"

3. Based on Selection
   └─ Branch to appropriate workflow

4. For "Add a new rule"
   ├─ "What should this rule do?"
   │   ├─ "Inject context when editing certain files"
   │   ├─ "Block certain operations"
   │   ├─ "Require explanation before commands"
   │   ├─ "Remind after editing files"
   │   └─ "Run custom validation"
   └─ Continue with guided questions
```

---

## 5. Project Analysis Details

### 5.1 Skill Discovery

**Location:** `.claude/skills/` (recursive)

**Process:**

```
1. Find all SKILL.md files
2. For each SKILL.md:
   a. Parse YAML frontmatter (if present)
   b. Extract:
      - name
      - description
      - triggers (keywords)
      - file patterns (from examples, triggers)
      - tools used
   c. Infer applicable:
      - extensions (from file patterns)
      - directories (from file patterns)
```

**Output per skill:**

```yaml
skill:
  name: aws-cdk
  path: .claude/skills/aws-cdk/SKILL.md
  description: "AWS CDK infrastructure as code"
  extensions: [.ts, .js]
  directories: [cdk/**, infra/**, **/cdk/**]
  tools: [Edit, Write, Bash]
```

### 5.2 CLAUDE.md Parsing

**Rules to extract:**

| Pattern | Rule Type | Action |
|---------|-----------|--------|
| MUST / ALWAYS | Hard requirement | Block violations |
| MUST NOT / NEVER | Hard prohibition | Block violations |
| SHOULD / PREFER | Soft requirement | Warn/remind |
| SHOULD NOT / AVOID | Soft prohibition | Warn/remind |
| DO NOT | Prohibition | Block or warn |

**Process:**

```
1. Read CLAUDE.md
2. Find rule statements (MUST, SHOULD, etc.)
3. For each rule:
   a. Extract the rule text
   b. Classify as hard/soft
   c. Identify scope (files, commands, patterns)
   d. Determine enforcement method:
      - Simple pattern → block_if_match
      - Complex logic → validator script
      - Context-dependent → inject warning
```

**Output per rule:**

```yaml
rule:
  text: "MUST NOT leave console.log statements"
  type: hard_prohibition
  scope:
    extensions: [.ts, .tsx, .js, .jsx]
  enforcement:
    method: validator
    action: block
```

### 5.3 Codebase Analysis

**Signals to detect:**

| Signal | Detection Method | Use |
|--------|------------------|-----|
| Languages | File extensions | Extension matchers |
| Frameworks | package.json, requirements.txt | Skill suggestions |
| Structure | Directory names | Directory matchers |
| Sensitive files | .env*, secrets/, etc. | Safety guards |
| CI/CD | .github/workflows/ | Deployment guards |
| Infrastructure | cdk/, terraform/, infra/ | IaC skill triggers |

---

## 6. Recommendation Types

### 6.1 Skill Triggers

**Purpose:** Automatically inject skill documentation when editing relevant files

**Template:**

```yaml
PreToolUse:
  - name: {skill-name}-trigger
    tools: [Edit, Write, str_replace]
    extensions: [{from skill analysis}]
    directories: [{from skill analysis}]
    inject: {skill SKILL.md path}
```

### 6.2 Rule Guards (Hard)

**Purpose:** Block violations of MUST/MUST NOT rules

**Template (pattern-based):**

```yaml
PreToolUse:
  - name: enforce-{rule-id}
    tools: [{relevant tools}]
    extensions: [{relevant extensions}]
    block_if_match:
      - pattern: "{violation pattern}"
        message: "{rule text}"
```

**Template (validator-based):**

```yaml
PreToolUse:
  - name: enforce-{rule-id}
    tools: [{relevant tools}]
    extensions: [{relevant extensions}]
    run: .claude/validators/{rule-id}.py
```

### 6.3 Rule Reminders (Soft)

**Purpose:** Remind about SHOULD/SHOULD NOT without blocking

**Template:**

```yaml
PreToolUse:
  - name: remind-{rule-id}
    tools: [{relevant tools}]
    extensions: [{relevant extensions}]
    inject: .claude/context/reminders/{rule-id}.md
```

### 6.4 Safety Guards

**Purpose:** Block dangerous operations

**Standard recommendations:**

| Guard | Trigger | Action |
|-------|---------|--------|
| Block force push | `git push.*(--force\|-f)` | block |
| Block rm -rf / | `rm\s+-rf\s+/` | block |
| Block prod access | `prod\|production` in sensitive commands | block |
| Block credential exposure | Patterns matching secrets | block |
| Warn destructive commands | `rm -rf`, `DROP TABLE`, etc. | inject warning |

### 6.5 Explanation Templates

**Purpose:** Require Claude to explain commands before asking permission

**Standard recommendations:**

| Template | Trigger | Fields |
|----------|---------|--------|
| explain-command | All Bash | what, why |
| explain-destructive | Destructive commands | what, why, risk, undo |
| explain-deployment | Deploy commands | what, changes, rollback |

### 6.6 Post-Edit Reminders

**Purpose:** Remind to run linters/tests after editing

**Standard recommendations:**

| Reminder | Trigger | Content |
|----------|---------|---------|
| Run linter | Edit .ts/.py/.js | "Remember to run linter" |
| Run tests | Edit src/** | "Consider running tests" |
| Update docs | Edit public API | "Update documentation if needed" |

---

## 7. Generated File Formats

### 7.1 hooks.yaml

```yaml
# Generated by CCH Skill
# Last updated: {timestamp}
version: "1"

# Skill Triggers
# Automatically inject skill documentation when editing relevant files
PreToolUse:
  - name: aws-cdk-trigger
    tools: [Edit, Write, str_replace]
    extensions: [.ts, .js]
    directories: [cdk/**, infra/**]
    inject: .claude/skills/aws-cdk/SKILL.md

# Rule Enforcement
# Guards based on CLAUDE.md rules
  - name: no-console-log
    tools: [Edit, Write]
    extensions: [.ts, .tsx, .js, .jsx]
    run: .claude/validators/no-console-log.py

# Safety Guards
  - name: block-force-push
    tools: [Bash]
    command_match: "git push.*(--force|-f)"
    action: block
    message: "Force push is not allowed. Use --force-with-lease if necessary."

# Explanation Requirements
PermissionRequest:
  - name: explain-bash-commands
    tools: [Bash]
    inject: .claude/context/explain-command.md

# Post-Edit Reminders
PostToolUse:
  - name: lint-reminder
    tools: [Edit, Write]
    extensions: [.ts, .tsx, .py]
    inject: .claude/context/lint-reminder.md
```

### 7.2 Context Markdown

```markdown
<!-- .claude/context/explain-command.md -->
<!-- Generated by CCH Skill -->

# Command Explanation Required

Before asking for permission, explain this command:

## What does this do?
[Plain English explanation]

## Why is this needed?
[Connection to user's goal]

## Risk level
[Low / Medium / High]
```

### 7.3 Validator Scripts

```python
#!/usr/bin/env python3
"""
Validator: no-console-log
Generated by CCH Skill

Enforces CLAUDE.md rule: "MUST NOT leave console.log statements"
"""
import sys
import json
import re


def main():
    try:
        event = json.load(sys.stdin)
    except json.JSONDecodeError:
        return 0  # Can't validate, allow
    
    content = event.get("tool_input", {}).get("content", "")
    
    # Check for console.log (but not commented out)
    lines = content.split("\n")
    for i, line in enumerate(lines, 1):
        stripped = line.lstrip()
        if stripped.startswith("//") or stripped.startswith("*"):
            continue
        if "console.log" in line:
            print(f"❌ console.log found on line {i}", file=sys.stderr)
            print("Per CLAUDE.md: MUST NOT leave console.log statements", file=sys.stderr)
            return 1
    
    return 0


if __name__ == "__main__":
    sys.exit(main())
```

---

## 8. User Interaction Patterns

### 8.1 Recommendation Presentation

When presenting recommendations, the skill should:

```
1. Group by category (Skill Triggers, Rule Guards, Safety, etc.)
2. For each recommendation:
   a. Show the rule (YAML)
   b. Explain WHY this is recommended
   c. Show what it will do
   d. Note any files that will be created
3. Ask for approval:
   a. "Accept all"
   b. "Accept with modifications"
   c. "Select which to include"
   d. "Reject all"
```

### 8.2 Approval Flow

```
Skill: "I recommend 12 hooks for this project:
        - 3 skill triggers
        - 5 rule guards
        - 4 safety guards
        
        Would you like to:
        1. Accept all recommendations
        2. Review each one individually
        3. See the full hooks.yaml first
        4. Cancel"

User: "2"

Skill: "Rule 1: aws-cdk-trigger
        
        This will inject the AWS CDK skill documentation whenever you
        edit .ts or .js files in the cdk/ or infra/ directories.
        
        [Shows YAML]
        
        Accept this rule? (y/n/modify)"
```

### 8.3 Error Handling

When errors occur:

```
1. Explain what went wrong (clearly)
2. Explain WHY it went wrong
3. Suggest how to fix it
4. Offer to help fix it
```

Example:

```
Skill: "❌ Validation failed: .claude/validators/check.py not found

        The hooks.yaml references a validator script that doesn't exist.
        
        Options:
        1. Create the missing script (I'll generate it)
        2. Remove the rule that uses it
        3. Update the path to an existing script"
```

---

## 9. Constraints & Limitations

### 9.1 What the Skill Does NOT Do

| Not Supported | Reason |
|---------------|--------|
| Handle runtime hook execution | That's CCH binary's job |
| Modify Claude Code internals | No access |
| Execute arbitrary code | Security |
| Access network | Not needed for setup |
| Remember across sessions | Stateless skill |

### 9.2 Dependencies

| Dependency | Required For |
|------------|--------------|
| CCH binary | All operations except analysis |
| Bash | Installation, validation |
| Python 3 | Validator scripts |
| Bun | TS/JS validators (if user chooses) |

### 9.3 File System Access

| Access | Scope |
|--------|-------|
| Read | Anywhere in project |
| Write | `.claude/` directory only |
| Execute | CCH binary, validator scripts |

---

## 10. Success Criteria

### 10.1 Functional Success

| Criteria | Metric |
|----------|--------|
| Install CCH successfully | Works on macOS, Linux, Windows |
| Discover all project skills | 100% of .claude/skills/*/SKILL.md |
| Parse CLAUDE.md | Extract all MUST/SHOULD rules |
| Generate valid config | `cch validate` passes |
| Install hooks | `cch install` succeeds |
| Validators work | Scripts execute correctly |

### 10.2 User Experience Success

| Criteria | Metric |
|----------|--------|
| Clear recommendations | User understands each suggestion |
| Actionable errors | User can fix issues from messages |
| Non-destructive | Existing config backed up before changes |
| Reversible | Can uninstall/rollback |

---

## 11. Future Considerations

### 11.1 Potential Enhancements

| Enhancement | Description |
|-------------|-------------|
| Config templates library | Pre-built configs for common stacks |
| Remote skill triggers | Fetch skill docs from URLs |
| Validator library | Pre-built validators for common rules |
| Config sharing | Export/import configurations |
| Hook analytics | Track which rules fire most often |

### 11.2 Plugin Path

When CCH becomes a Claude Code plugin:

| Current (Skill) | Future (Plugin) |
|-----------------|-----------------|
| Manual trigger | Automatic suggestions |
| File-based config | UI-based config |
| CLI installation | Integrated installation |
| Text output | Rich UI output |

---

## 12. Open Questions

| Question | Options | Notes |
|----------|---------|-------|
| Skill name | `cch`, `cch-skill`, `hooks-manager` | Needs to match CCH branding |
| Where to store validators | `.claude/validators/` vs `.claude/hooks/validators/` | Consistency with CCH |
| Backup strategy | `.bak` files vs `backups/` directory | Space vs. organization |
| Template source | Embedded vs. fetched | Offline capability |

---

*End of CCH Skill PRD*

---

# CCH Skill PRD - Addendum

**Document:** Addendum to Claude Context Hooks (CCH) Skill PRD  
**Version:** 1.0  
**Date:** January 21, 2025  
**Related Documents:**
- CCH Skill PRD (main document)
- CCH Binary PRD
- CCH Binary PRD Addendum
- CCH Unified Reference PRD

---

## Overview

This addendum extends the CCH Skill PRD with specifications for:

1. CCH Binary Integration
2. Telemetry and Log Analysis
3. Rule Data Model and Provenance
4. Confidence and Ambiguity Handling
5. Conflict Detection
6. Installation Audit Trail

These specifications address gaps identified in architectural review and ensure the skill provides proper observability, auditability, and intelligent recommendations.

---

## 1. CCH Binary Integration

### 1.1 Binary Resolution Order

When executing CCH commands, the skill resolves the binary in this order:

| Priority | Location | Rationale |
|----------|----------|-----------|
| 1 (highest) | `.claude/bin/cch` | Project-pinned, reproducible |
| 2 | `which cch` (PATH) | User-installed |
| 3 | Configured override | Escape hatch |

**Configured override location:** `.claude/cch/config.json`

```json
{
  "cchBinaryPath": "/custom/path/to/cch"
}
```

**Resolution algorithm:**

```python
def resolve_cch_binary():
    # 1. Project-pinned
    project_bin = Path(".claude/bin/cch")
    if project_bin.exists() and is_executable(project_bin):
        return project_bin
    
    # 2. User PATH
    path_bin = shutil.which("cch")
    if path_bin:
        return Path(path_bin)
    
    # 3. Configured override
    config = load_cch_config()
    if config and config.get("cchBinaryPath"):
        override = Path(config["cchBinaryPath"])
        if override.exists() and is_executable(override):
            return override
    
    # Not found
    return None
```

### 1.2 Supported Platforms and Architectures

| OS | Architecture | Binary Artifact |
|----|--------------|-----------------|
| macOS | Apple Silicon (arm64) | `cch-aarch64-apple-darwin.tar.gz` |
| macOS | Intel (x86_64) | `cch-x86_64-apple-darwin.tar.gz` |
| Linux | x86_64 (glibc) | `cch-x86_64-unknown-linux-gnu.tar.gz` |
| Linux | x86_64 (musl) | `cch-x86_64-unknown-linux-musl.tar.gz` |
| Linux | ARM64 | `cch-aarch64-unknown-linux-gnu.tar.gz` |
| Windows | x86_64 | `cch-x86_64-pc-windows-msvc.zip` |

### 1.3 Installation Modes

| Mode | Location | Use Case |
|------|----------|----------|
| Project (recommended) | `.claude/bin/cch` | Reproducible per-repo |
| User | Platform-dependent (see below) | Personal machine, all projects |

**User install locations:**

| Platform | Location |
|----------|----------|
| macOS | `~/.local/bin/cch` |
| Linux | `~/.local/bin/cch` |
| Windows | `%USERPROFILE%\.local\bin\cch.exe` |

### 1.4 Required CCH Commands

The skill depends on these CCH CLI commands:

| Operation | Command | Required |
|-----------|---------|----------|
| Version check | `cch --version` | Yes |
| Version check (JSON) | `cch --version --json` | Yes |
| Initialize structure | `cch init` | Yes |
| Validate config | `cch validate` | Yes |
| Install hooks (project) | `cch install --project` | Yes |
| Install hooks (user) | `cch install --user` | Yes |
| Uninstall hooks | `cch uninstall --project\|--user` | Yes |
| Debug matching | `cch debug <event>` | Yes |
| Query logs | `cch logs` | Yes |
| Show config | `cch config --show` | Yes |
| Show log path | `cch config --show log-path` | Yes |

If a command is missing or fails unexpectedly, the skill treats this as an incompatible CCH version.

### 1.5 Version Compatibility

**Minimum supported CCH version:** 1.0.0  
**Minimum API version:** 1

**Version check process:**

```python
def check_cch_version():
    result = run(["cch", "--version", "--json"])
    info = json.loads(result.stdout)
    
    if info["api_version"] < MINIMUM_API_VERSION:
        return VersionResult.INCOMPATIBLE
    
    if parse_version(info["version"]) < MINIMUM_VERSION:
        return VersionResult.OUTDATED
    
    return VersionResult.OK
```

**Behavior on version issues:**

| Result | Skill Action |
|--------|--------------|
| `INCOMPATIBLE` | Block operations, require upgrade |
| `OUTDATED` | Warn, recommend upgrade, allow continue |
| `OK` | Proceed normally |

### 1.6 Trust and Integrity

When installing CCH binary, the skill:

1. **Downloads from official source:** GitHub Releases only
2. **Verifies checksum:** Downloads `checksums.sha256`, verifies artifact
3. **Records installation:** Creates audit trail (see Section 6)

**Checksum verification:**

```python
def verify_download(artifact_path, checksums_path):
    expected = parse_checksums(checksums_path)
    actual = sha256_file(artifact_path)
    
    if actual != expected[artifact_path.name]:
        raise IntegrityError(f"Checksum mismatch for {artifact_path.name}")
```

---

## 2. Telemetry and Log Analysis

### 2.1 Log Discovery

The skill discovers CCH log location via:

```bash
cch config --show log-path
```

Fallback if command unavailable:
- `~/.claude/logs/cch.log`

### 2.2 Log Parsing

The skill parses JSON Lines format logs:

```python
def parse_logs(log_path, filters=None):
    events = []
    with open(log_path) as f:
        for line in f:
            try:
                entry = json.loads(line)
                if matches_filters(entry, filters):
                    events.append(entry)
            except json.JSONDecodeError:
                continue  # Skip malformed lines
    return events
```

### 2.3 Troubleshooting Capabilities

| Capability | Implementation |
|------------|----------------|
| "Why didn't my rule match?" | Parse recent logs, find event, show matcher trace |
| "Why was this blocked?" | Find blocking event, show rule and reason |
| "What rules are firing?" | Aggregate `rules_matched` from recent logs |
| "Is CCH working?" | Check for recent log entries |

**Troubleshooting workflow:**

```
User: "Why didn't my commit hook block WIP commits?"

Skill:
1. cch config --show log-path
2. Parse recent PreToolUse events for "git commit"
3. Check if block-wip rule is in rules_matched
4. If not matched: explain why (matcher didn't match)
5. If matched but not blocked: explain action was "continue"
6. Show relevant log entry
7. Suggest fix
```

### 2.4 Analytics Capabilities

| Capability | Use |
|------------|-----|
| Rule firing frequency | Identify hot rules, unused rules |
| Block frequency | Understand enforcement patterns |
| Script performance | Find slow validators |
| Common violations | Prioritize rule improvements |

**Example analytics query:**

```python
def rule_frequency(logs, days=7):
    cutoff = datetime.now() - timedelta(days=days)
    counts = Counter()
    
    for entry in logs:
        if parse_timestamp(entry["ts"]) > cutoff:
            for rule in entry["data"]["rules_matched"]:
                counts[rule["name"]] += 1
    
    return counts.most_common()
```

### 2.5 Privacy Considerations

| Principle | Implementation |
|-----------|----------------|
| Local only | Skill reads logs locally, never transmits |
| User consent | Analytics shown only when requested |
| No PII extraction | Skill doesn't extract/store sensitive data from logs |

---

## 3. Rule Data Model and Provenance

### 3.1 Internal Rule Model

The skill maintains an internal representation of rules that includes provenance:

```python
@dataclass
class Rule:
    # Identity
    id: str                          # e.g., "no-console-log"
    name: str                        # Human-readable name
    
    # Source provenance
    source: RuleSource               # Where this rule came from
    original_text: Optional[str]     # Original text (e.g., from CLAUDE.md)
    generated_at: datetime           # When the rule was generated
    generated_by: str                # "cch-skill" or "manual"
    
    # Classification
    classification: RuleClassification  # hard_requirement, soft_recommendation, etc.
    confidence: Confidence           # high, medium, low
    
    # Scope
    scope: RuleScope                 # extensions, directories, tools, etc.
    
    # Enforcement
    enforcement: RuleEnforcement     # type, action, validator path
    
    # Metadata
    description: Optional[str]       # Explanation for humans
    rationale: Optional[str]         # Why this rule exists


@dataclass
class RuleSource:
    type: str                        # "claude_md", "skill", "manual", "safety_default"
    file: Optional[str]              # Source file path
    line: Optional[int]              # Line number in source
    skill_name: Optional[str]        # If from a skill


@dataclass
class RuleClassification:
    type: str                        # "hard_prohibition", "hard_requirement", 
                                     # "soft_prohibition", "soft_recommendation"
    keywords: List[str]              # MUST, MUST NOT, SHOULD, etc.


class Confidence(Enum):
    HIGH = "high"                    # Mechanical, clear rule
    MEDIUM = "medium"                # Some ambiguity
    LOW = "low"                      # Vague, stylistic
```

### 3.2 Source Types

| Source Type | Description | Example |
|-------------|-------------|---------|
| `claude_md` | Extracted from CLAUDE.md | "MUST NOT use console.log" |
| `skill` | Generated from skill analysis | Trigger for aws-cdk skill |
| `manual` | User-defined | Custom block rule |
| `safety_default` | Standard safety recommendation | Block force push |
| `template` | From a template | Strict template rules |

### 3.3 Provenance in Generated Config

The skill embeds provenance as comments and metadata:

```yaml
# Generated by cch-skill at 2025-01-21T10:30:00Z
# Source: Project analysis + CLAUDE.md
version: "1"

# Metadata (optional, for tooling)
_metadata:
  generated_by: cch-skill
  generated_at: "2025-01-21T10:30:00Z"
  cch_skill_version: "1.0.0"
  sources:
    - type: claude_md
      file: CLAUDE.md
      rules_extracted: 5
    - type: skill
      skills_found: 3
    - type: safety_default
      rules_added: 4

PreToolUse:
  # Source: CLAUDE.md line 12
  # Original: "MUST NOT leave console.log statements in production code"
  # Confidence: high
  - name: no-console-log
    tools: [Edit, Write]
    extensions: [.ts, .tsx, .js, .jsx]
    run: .claude/validators/no-console-log.py

  # Source: Skill discovery (.claude/skills/aws-cdk/SKILL.md)
  # Confidence: high
  - name: aws-cdk-trigger
    tools: [Edit, Write, str_replace]
    extensions: [.ts, .js]
    directories: [cdk/**, infra/**]
    inject: .claude/skills/aws-cdk/SKILL.md

  # Source: Safety default
  # Confidence: high
  - name: block-force-push
    tools: [Bash]
    command_match: "git push.*(--force|-f)"
    action: block
    message: "Force push is not allowed"
```

### 3.4 Provenance Query

The skill can explain any rule:

```
User: "Why does the block-force-push rule exist?"

Skill:
"The 'block-force-push' rule is a safety default that I recommend for all 
projects. Force pushing can overwrite team members' work and is generally 
dangerous on shared branches.

Source: Safety default (not from your CLAUDE.md or skills)
Confidence: High
Generated: 2025-01-21

If you want to allow force push, you can remove this rule from .claude/hooks.yaml"
```

---

## 4. Confidence and Ambiguity Handling

### 4.1 Confidence Levels

| Level | Criteria | Example |
|-------|----------|---------|
| **High** | Mechanical, unambiguous, enforceable | "MUST NOT use console.log" |
| **Medium** | Clear intent but some interpretation needed | "SHOULD keep functions under 50 lines" |
| **Low** | Vague, stylistic, subjective | "Prefer functional style" |

### 4.2 Confidence Determination

```python
def determine_confidence(rule_text: str, context: AnalysisContext) -> Confidence:
    # High confidence indicators
    if any(kw in rule_text.upper() for kw in ["MUST NOT", "MUST", "NEVER", "ALWAYS"]):
        if is_mechanically_enforceable(rule_text):
            return Confidence.HIGH
    
    # Medium confidence indicators
    if any(kw in rule_text.upper() for kw in ["SHOULD NOT", "SHOULD", "PREFER NOT"]):
        return Confidence.MEDIUM
    
    # Low confidence indicators
    if any(kw in rule_text.lower() for kw in ["consider", "prefer", "try to", "when possible"]):
        return Confidence.LOW
    
    # Default to medium if unclear
    return Confidence.MEDIUM


def is_mechanically_enforceable(rule_text: str) -> bool:
    """Check if rule can be enforced with pattern matching or static analysis."""
    # Examples of enforceable rules:
    # - "no console.log" → grep for console.log
    # - "no TODO without issue" → regex match
    # - "files must have header" → check first lines
    
    enforceable_patterns = [
        r"console\.log",
        r"TODO|FIXME|HACK",
        r"debugger",
        r"\.env",
        # ... more patterns
    ]
    
    return any(re.search(p, rule_text, re.I) for p in enforceable_patterns)
```

### 4.3 Behavior by Confidence

| Confidence | Skill Behavior |
|------------|----------------|
| **High** | Auto-recommend, include in default setup |
| **Medium** | Present to user with explanation, ask for confirmation |
| **Low** | Suggest only, don't include by default, explain limitations |

### 4.4 Presenting Low-Confidence Rules

```
Skill: "I found a rule in CLAUDE.md that I'm not confident I can enforce reliably:

  'Prefer functional programming style over imperative'

This is subjective and can't be reliably detected by pattern matching. 

Options:
1. Skip this rule (recommended)
2. Add as a reminder injection (won't block, just reminds)
3. Create a custom validator (requires manual implementation)

What would you like to do?"
```

---

## 5. Conflict Detection

### 5.1 Conflict Types

| Type | Example | Severity |
|------|---------|----------|
| **Direct contradiction** | "Use OOP" vs "Use functional style" | High |
| **Scope overlap** | Two rules matching same files with different actions | Medium |
| **Implicit tension** | "Keep functions small" vs "Avoid too many functions" | Low |

### 5.2 Conflict Detection Process

```python
def detect_conflicts(rules: List[Rule]) -> List[Conflict]:
    conflicts = []
    
    for i, rule_a in enumerate(rules):
        for rule_b in rules[i+1:]:
            # Check scope overlap
            if scopes_overlap(rule_a.scope, rule_b.scope):
                # Check action conflict
                if actions_conflict(rule_a.enforcement, rule_b.enforcement):
                    conflicts.append(Conflict(
                        type="action_conflict",
                        rules=[rule_a, rule_b],
                        severity="high",
                        description=f"Rules '{rule_a.name}' and '{rule_b.name}' "
                                   f"match the same files but have conflicting actions"
                    ))
                
                # Check semantic conflict
                semantic = check_semantic_conflict(rule_a, rule_b)
                if semantic:
                    conflicts.append(semantic)
    
    return conflicts
```

### 5.3 Conflict Resolution

When conflicts are detected, the skill presents them:

```
Skill: "I found a conflict in your rules:

  Rule 1 (from CLAUDE.md:15): 'Always use functional style'
  Rule 2 (from CLAUDE.md:42): 'Prefer OOP for service classes'

These rules may conflict when working on service files.

Options:
1. Keep both (may cause confusion)
2. Prioritize Rule 1 (functional everywhere)
3. Prioritize Rule 2 (OOP for services)
4. Scope Rule 1 to exclude services
5. Skip both

Which would you prefer?"
```

### 5.4 Conflict Report

For complex projects, generate a conflict report:

```markdown
# Rule Conflict Report

Generated: 2025-01-21T10:30:00Z

## High Severity Conflicts

### Conflict 1: Style contradiction
- **Rule A:** functional-style (CLAUDE.md:15)
- **Rule B:** oop-services (CLAUDE.md:42)
- **Overlap:** Both match .ts files in src/services/
- **Recommendation:** Scope functional-style to exclude src/services/

## Medium Severity Conflicts

### Conflict 2: Overlapping triggers
- **Rule A:** typescript-skill (extensions: .ts, .tsx)
- **Rule B:** react-skill (extensions: .tsx)
- **Overlap:** .tsx files trigger both
- **Recommendation:** OK if intentional (additive injection)

## Summary
- High severity: 1
- Medium severity: 1
- Low severity: 0
```

---

## 6. Installation Audit Trail

### 6.1 Audit File Location

`.claude/cch/install.json`

### 6.2 Audit File Schema

```json
{
  "schema_version": 1,
  "binary": {
    "version": "1.2.3",
    "api_version": 1,
    "installed_at": "2025-01-21T10:30:00Z",
    "installed_by": "cch-skill",
    "install_mode": "project",
    "source": {
      "type": "github_release",
      "url": "https://github.com/user/cch/releases/download/v1.2.3/cch-aarch64-apple-darwin.tar.gz",
      "sha256": "abc123def456..."
    },
    "location": ".claude/bin/cch",
    "platform": {
      "os": "darwin",
      "arch": "aarch64"
    }
  },
  "config": {
    "generated_at": "2025-01-21T10:35:00Z",
    "generated_by": "cch-skill",
    "skill_version": "1.0.0",
    "sources_analyzed": [
      {
        "type": "claude_md",
        "file": "CLAUDE.md",
        "hash": "sha256:abc...",
        "rules_extracted": 5
      },
      {
        "type": "skill_discovery",
        "directory": ".claude/skills",
        "skills_found": ["aws-cdk", "react", "python"]
      }
    ],
    "rules_generated": 12,
    "conflicts_detected": 1,
    "conflicts_resolved": 1
  },
  "history": [
    {
      "action": "install_binary",
      "timestamp": "2025-01-21T10:30:00Z",
      "version": "1.2.3"
    },
    {
      "action": "generate_config",
      "timestamp": "2025-01-21T10:35:00Z",
      "rules_count": 12
    },
    {
      "action": "install_hooks",
      "timestamp": "2025-01-21T10:36:00Z",
      "target": "project"
    }
  ]
}
```

### 6.3 Audit Operations

| Operation | Recorded Data |
|-----------|---------------|
| Binary install | Version, source, checksum, location |
| Config generation | Sources analyzed, rules generated, conflicts |
| Hook installation | Target (project/user), timestamp |
| Config update | What changed, why |
| Uninstall | What was removed |

### 6.4 Audit Query

The skill can answer provenance questions:

```
User: "When was CCH installed and what version?"

Skill:
"CCH was installed on January 21, 2025 at 10:30 AM.

Binary: v1.2.3 (API v1)
Location: .claude/bin/cch
Source: GitHub Release
Checksum: verified ✓

Configuration was generated at 10:35 AM with:
- 5 rules from CLAUDE.md
- 3 skill triggers
- 4 safety defaults

Last modified: January 21, 2025 at 10:36 AM"
```

---

## 7. Error Handling

### 7.1 Binary Errors

| Error | Skill Response |
|-------|----------------|
| Binary not found | Offer to install |
| Version incompatible | Explain and offer upgrade |
| Command failed | Show error, suggest debug steps |
| Timeout | Report timeout, suggest increasing limit |

### 7.2 Configuration Errors

| Error | Skill Response |
|-------|----------------|
| Invalid YAML | Show error location, offer to fix |
| Missing referenced file | Identify file, offer to create or remove reference |
| Invalid pattern | Explain issue, suggest correction |

### 7.3 Analysis Errors

| Error | Skill Response |
|-------|----------------|
| CLAUDE.md not found | Continue without, note limitation |
| Skills directory empty | Continue without skill triggers |
| Ambiguous rule | Ask for clarification |

---

## 8. Future Considerations

### 8.1 State Management (Post-v1)

While v1 is stateless, future versions may want:

| State | Purpose |
|-------|---------|
| Rule firing counts | Analytics |
| User preferences | Remember choices |
| Conflict resolutions | Don't re-ask |

Storage location: `.claude/cch/state.json`

### 8.2 Remote Configuration (Post-v1)

| Feature | Description |
|---------|-------------|
| Shared templates | Pull from team repository |
| Central policy | Enterprise policy distribution |
| Update notifications | Check for skill/binary updates |

### 8.3 IDE Integration (Post-v1)

| Feature | Description |
|---------|-------------|
| VS Code extension | IntelliSense for hooks.yaml |
| Config preview | See what rules match current file |
| Inline diagnostics | Show conflicts in editor |

---

## Summary

This addendum specifies:

1. **Binary Integration:** Resolution order, version checks, integrity verification
2. **Telemetry:** Log parsing, troubleshooting, analytics
3. **Rule Data Model:** Internal representation with provenance
4. **Confidence Handling:** Different behavior for high/medium/low confidence rules
5. **Conflict Detection:** Finding and resolving rule conflicts
6. **Audit Trail:** Recording all installation and configuration actions

These specifications transform the skill from "setup assistant" to "governance tool" by adding:

- **Explainability:** Every rule can be traced to its source
- **Reliability:** Version compatibility and integrity checks
- **Observability:** Log analysis and troubleshooting
- **Trust:** Confidence levels prevent overreach

---

*End of CCH Skill PRD Addendum*