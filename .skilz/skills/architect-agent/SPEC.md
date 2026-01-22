# Architect Agent Skill - Specification

**Version:** 4.1
**Last Updated:** 2025-01-21
**Status:** Living Document

---

## 1. Purpose & Vision

### What This Skill Does

The **architect-agent skill** transforms AI agents (Claude Code, OpenCode, GEMINI, GitHub Copilot, etc.) into specialized **architect agents** that:

1. **Plan** complex implementation work with architectural guidance
2. **Delegate** work to code agents via detailed, structured instructions
3. **Grade** completed implementations against objective rubrics
4. **Iterate** with code agents until quality thresholds are met (≥95%)
5. **Learn** from outcomes by updating code agent memory (CLAUDE.md/AGENTS.md)

### Core Value Proposition

**Before this skill:**
- AI agents implement work directly without clear success criteria
- No structured grading or quality assurance
- No iterative improvement cycle
- No accumulated learning from past work

**After this skill:**
- Architect agent plans with architectural thinking
- Code agent executes with clear requirements
- Objective grading ensures quality (≥95% target)
- Iterative cycles improve until standards met
- Institutional knowledge accumulates in code agent workspace

---

## 2. Scope & Boundaries

### In Scope

**Core Workflows:**
- ✅ Creating delegation instructions (human summary + technical details)
- ✅ Workspace initialization (architect + code agent directories)
- ✅ Grading completed work (6-category rubric, 100 points)
- ✅ Iterative improvement cycles (until ≥95%)
- ✅ Cross-workspace collaboration (architect ↔ code agent)
- ✅ Memory management (updating code agent CLAUDE.md/AGENTS.md with learnings)
- ✅ OpenCode integration (uses plugins for most things, and calls log script for decisions, etc.)
- ✅ Claude Code integration (uses hooks for most things, and calls log script for decisions, etc.)
- ✅ Gemini CLI integration, Github Copilot CLI (calls log scripts for everything)
- ✅ Template-based automation (setup-workspace.sh, verify-workspace.sh)

**Documentation & Protocols:**
- ✅ Logging protocols (real-time, hybrid, compatible-mode for code command line assistance that do not support hooks or plugins that allow responding to events)
- ✅ Testing protocols (progressive, coverage ≥60%)
- ✅ Resilience protocols (error recovery, verification)
- ✅ Agent specialization (right agent for the job)
   - This should be configurable with known defaults for developers that have their own coding subagents
- ✅ Git/PR management (commit format, PR structure)
- ✅ Permissions setup (cross-workspace file operations)

### Out of Scope

**Not Included:**
- ❌ Actual code implementation (that's the code agent's job)
- ❌ Domain-specific business logic (architect provides architectural guidance only)
- ❌ Project management tools integration (Jira, Linear, etc.)
- ❌ CI/CD pipeline execution (code agent runs builds/tests)
- ❌ Code review for style (focus is on completeness, testing, verification)
- ❌ Production deployment (architect delegates, doesn't deploy)

**Why These Boundaries:**
- Architect agent **plans and evaluates**, code agent **implements**
- Separation of concerns maintains clear responsibilities
- Prevents scope creep into tool-specific implementations

---

## 2.5 File Naming and Correlation Specification

### Universal Pattern

All architect agent and code agent files follow a consistent naming pattern:

```
<type>-<date>-<time>-<ticket_id>_<phase>_<description>.md
```

**Component Specifications:**

| Component | Format | Example | Notes |
|-----------|--------|---------|-------|
| **type** | Text | `instruct`, `grade`, `human`, `log`, `analysis` | Identifies file category |
| **date** | `YYYY_MM_DD` | `2025_01_21` | Underscores (NOT hyphens) for readability |
| **time** | `HH_MM` | `14_30` | 24-hour format, underscores |
| **ticket_id** | Alphanumeric | `tkt123`, `proj456` | Jira/ticket system ID |
| **phase** | Text | `phase5`, `phase5b`, `step3` | Multi-phase work identifier |
| **description** | snake_case | `infrastructure_deployment` | Brief descriptive words |

**Date/Time Rationale:**
- Human-readable format (underscores) enables quick visual parsing
- Consistent with ISO 8601 but filesystem-friendly
- Sorts chronologically in file listings
- Distinguishes from version numbers (which use hyphens)

### Correlation Rules

**CRITICAL**: Instructions, grades, logs, and human summaries for the same task MUST share:

1. Same `ticket_id`
2. Same `phase` identifier (if multi-phase)
3. Same `description` words
4. **Different** timestamps (reflect actual creation time)

**Example - Correlated Files:**
```
# Architect workspace
instructions/instruct-2025_01_21-14_30-tkt123_phase5_infrastructure_deployment.md
human/human-2025_01_21-14_30-tkt123_phase5_infrastructure_deployment.md
grades/grade-2025_01_21-18_45-tkt123_phase5_infrastructure_deployment.md

# Code agent workspace
debugging/logs/log-2025_01_21-14_35-tkt123_phase5_infrastructure_deployment.md
```

**Note**: Grade and log timestamps differ because they're created after work completes.

**Example - Improvement Iteration:**
```
# First attempt (Score: 82%)
instructions/instruct-2025_01_21-14_30-tkt123_phase5_infrastructure_deployment.md
grades/grade-2025_01_21-18_45-tkt123_phase5_infrastructure_deployment.md

# Second attempt (Score: 97%)
instructions/instruct-2025_01_21-20_15-tkt123_phase5b_fix_deployment_permissions.md
grades/grade-2025_01_21-23_00-tkt123_phase5b_fix_deployment_permissions.md
```

**Sub-phase Naming**: Use suffixes like `phase5b`, `phase5c` for iterative improvements.

### Directory-Specific Patterns

#### Architect Agent Workspace Files

**instructions/**
```
instruct-YYYY_MM_DD-HH_MM-ticket_id_phase_description.md
```
- Full technical instructions for code agent
- Includes logging requirements, testing protocol, success criteria
- Archived after completion (never deleted from architect workspace)

**grades/**
```
grade-YYYY_MM_DD-HH_MM-ticket_id_phase_description.md
```
- Grading report with rubric breakdown (100-point scale)
- Must correlate to instruction file (same ticket_id_phase_description)
- One grade per instruction attempt

**human/**
```
human-YYYY_MM_DD-HH_MM-ticket_id_phase_description.md
```
- Concise summary for humans (not code agents)
- Correlates to instruction file
- Optional but recommended for complex tasks

**analysis/**
```
analysis-YYYY_MM_DD-HH_MM-description.md
```
- Investigation and research documents
- May omit ticket_id for general analysis
- Used for brainstorming and architectural exploration

#### Code Agent Workspace Files

**debugging/logs/**
```
log-YYYY_MM_DD-HH_MM-description.md
```
- Description portion MUST match corresponding instruction
- Created by code agent at session start
- Contains execution timeline with tool calls, decisions, verifications
- Never deleted (permanent audit trail)

**Example Correlation (Architect → Code Agent):**
```
# Architect creates instruction
instructions/instruct-2025_01_21-14_30-tkt123_phase5_infrastructure_deployment.md

# Code agent creates matching log
debugging/logs/log-2025_01_21-14_35-tkt123_phase5_infrastructure_deployment.md
```

### Date Format Usage Contexts

**Four different date/time formats used across the system:**

| Context | Format | Example | Why This Format? |
|---------|--------|---------|------------------|
| **File names** | `YYYY_MM_DD-HH_MM` | `2025_01_21-14_30` | Filesystem-safe, sorts chronologically |
| **Log entries** | `[HH:MM:SS]` | `[14:30:45]` | Compact, relative to session start |
| **Human dates** | `YYYY-MM-DD` | `2025-01-21` | ISO 8601 standard, widely recognized |
| **Timestamps** | `YYYY-MM-DDTHH:MM:SSZ` | `2025-01-21T14:30:45Z` | ISO 8601 with timezone (JSON/APIs) |

**Consistency Rule**: Each context uses its specified format consistently throughout the system.

### Scenario Examples

**Single-Phase Ticket Work:**
```
# Architect workspace
instructions/instruct-2025_01_04-14_30-proj123_implement_contact_api.md
human/human-2025_01_04-14_30-proj123_implement_contact_api.md
grades/grade-2025_01_04-18_45-proj123_implement_contact_api.md

# Code agent workspace
debugging/logs/log-2025_01_04-14_35-proj123_implement_contact_api.md
```

**Multi-Phase with Improvement Iteration:**
```
# Phase 5 - Initial attempt (82%)
instructions/instruct-2025_10_20-22_35-tkt121_phase5_infrastructure_deployment.md
grades/grade-2025_10_20-23_00-tkt121_phase5_infrastructure_deployment.md
debugging/logs/log-2025_10_20-22_37-tkt121_phase5_infrastructure_deployment.md

# Phase 5c - Fix after failures (97%)
instructions/instruct-2025_10_21-22_56-tkt121_phase5c_fix_scheduler_https.md
grades/grade-2025_10_21-23_43-tkt121_phase5c_fix_scheduler_https.md
debugging/logs/log-2025_10_21-22_58-tkt121_phase5c_fix_scheduler_https.md
```

**General Analysis (No Ticket):**
```
analysis/analysis-2025_01_04-14_00-database_schema_design.md
analysis/analysis-2025_01_04-16_30-api_performance_investigation.md
```

**Related References:**
- `references/file_naming.md` - Complete naming convention details
- `references/logging_protocol.md` - Log file requirements
- `references/instruction_grading_workflow.md` - File lifecycle and correlation

---

## 3. Architecture

### Three-Level Progressive Disclosure

**Level 1: Metadata (Always in Context)**
- SKILL.md YAML frontmatter (~100 words)
- Name, description, triggers
- Loaded automatically when Claude Code starts

**Level 2: SKILL.md Body (When Skill Triggers)**
- Core workflows and protocols (<5k words)
- Quick reference checklists
- Links to references for details
- Loaded when user triggers skill

**Level 3: References (As Needed)**
- Detailed protocols in `references/` (unlimited size)
- Loaded only when architect needs specific guidance
- Keeps context window lean

**Design Principle:** Minimize context usage while maintaining comprehensive guidance.

---

### Directory Structure

```
architect-agent/
├── SKILL.md                    # Core skill (metadata + workflows)
├── SPEC.md                     # THIS FILE - project specification
├── README.md                   # Brief overview + quick start
├── CLAUDE.md                   # Skill workspace configuration
├── AGENTS.md                   # Git workflow requirements
├── CONTRIBUTING.md             # Contribution guidelines
│
├── references/                 # Detailed protocols (26+ files)
│   ├── README.md               # Reference index
│   ├── installation.md         # Setup guide
│   ├── upgrade.md              # Migration guide
│   ├── quick_start.md          # Getting started
│   ├── [Core Workflows]/       # logging, testing, grading, etc.
│   ├── [OpenCode Suite]/       # opencode_*, claude_vs_opencode_*
│   ├── [Templates]/            # code_agent_claude_template.md, etc.
│   └── [Advanced]/             # instruction_grading_workflow.md, etc.
│
├── templates/                  # Workspace templates
│   ├── README.md               # Template documentation
│   ├── setup-workspace.sh      # Automated setup
│   ├── verify-workspace.sh     # Validation script
│   ├── architect-workspace/    # Architect workspace template
│   └── code-agent-workspace/   # Code agent workspace template with samples for opencode and claude code
│
└── docs/                       # Project documentation
    └── testing/                # Test cases and expected outputs
```

**File Organization Standards:**
- **Root level**: Only essential files (SKILL.md, README.md, SPEC.md, CLAUDE.md, AGENTS.md)
- **references/**: ALL detailed protocols and guides
- **templates/**: Complete, ready-to-deploy workspace templates
- **docs/**: Project-level documentation (not protocol docs)

---

### Multi-Workspace Architecture

**Architect Agent Workspace:**
```
~/projects/architect-agent-workspace/
├── instructions/           # Instructions for code agent (YOU write here)
├── human/                  # Human summaries (YOU write here)
├── grades/                 # Grading reports (YOU write here)
├── ticket/                 # Current ticket tracking
├── analysis/               # Investigation documents
└── CLAUDE.md               # Code agent workspace path configured here
```

**Code Agent Workspace:**
```
~/projects/code-agent-workspace/
├── src/                    # Source code (THEY write here)
├── tests/                  # Tests (THEY write here)
├── debugging/
│   ├── logs/               # Execution logs (THEY write here)
│   └── instructions/       # Temporary instructions (YOU copy here)
├── CLAUDE.md               # Memory + architect learnings
└── AGENTS.md               # Agent collaboration protocol
```

**Critical Principle:** Architect and code agent work in **separate directories**. Instructions flow from architect → code agent via file copy.

### 3.2.1 Code Agent .claude/ Directory (Critical Configuration)

**Complete Directory Structure:**
```
code-agent-workspace/
└── .claude/
    ├── settings.json          # ✅ CRITICAL: Hook configuration goes HERE
    ├── hook-logger.py         # Hook script for automated logging
    ├── docs/                  # Protocol documentation (optional)
    │   ├── logging_setup.md
    │   ├── testing_protocol.md
    │   └── agent_usage.md
    └── hooks.json.backup      # ❌ OLD: Do not use (archived)
```

#### settings.json Configuration (CRITICAL)

**⚠️ CRITICAL DISCOVERY**: Claude Code reads hooks from `.claude/settings.json`, NOT from `.claude/hooks.json`.

**Basic Configuration (PostToolUse only):**
```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ]
  }
}
```

**Enhanced Configuration (v4.1 - Recommended):**
The template includes 9 event types: PostToolUse, PreToolUse, UserPromptSubmit, PermissionRequest, Notification, SessionStart, PreCompact, Stop, SubagentStop. See Section 6.1 "Complete Hook Configuration (v4.1)" for full configuration.

**Key Settings:**
- **Event Types**: 9 total (3 graded, 6 non-graded extras)
  - **Graded**: PostToolUse, SessionStart (required for passing grades)
  - **Non-Graded**: PreToolUse, UserPromptSubmit, PermissionRequest, Notification, PreCompact, Stop, SubagentStop (optional extras)
- **Matcher**: `"*"` - Captures all tools/events
- **Command**: Same hook script handles all event types
- **Timeout**: 5 seconds per event

**Impact**: Moving to settings.json with enhanced events achieved:
- ✅ 60-70% token savings (automated vs manual logging)
- ✅ Zero-prompt logging workflow
- ✅ Automatic capture of all tool calls + user prompts + permissions
- ✅ ~2,200 tokens saved per 30-command session
- ✅ Permission optimization data for faster execution

#### hook-logger.py Script (v4.1)

**Purpose**: Unified hook script that handles 9 event types, writes formatted entries to active log file.

**Supported Event Types (v4.1):**
- **PostToolUse**: Tool execution results (Bash, Read, Write, Edit, Grep, Glob, TodoWrite)
- **PreToolUse**: Before tool execution
- **UserPromptSubmit**: User prompts before processing
- **PermissionRequest**: Permission requests (CRITICAL for optimization)
- **Notification**: System notifications
- **SessionStart**: Session initialization
- **PreCompact**: Before compaction
- **Stop**: Agent completion
- **SubagentStop**: Subagent completion

**Tool-Specific Details (PostToolUse):**
- **Bash**: command + description
- **Read**: file_path + offset/limit
- **Write**: file_path + content size
- **Edit**: file_path + old/new strings + replace_all flag
- **Grep**: pattern + path + glob + output_mode + case_insensitive
- **Glob**: pattern + path
- **TodoWrite**: todo count + status summary

**Key Features:**
- Single script handles all 9 event types (detects event_type from JSON)
- Silent failure (doesn't break hooks on error)
- Checks for active log session via `debugging/current_log_file.txt`
- Only logs when session is active
- Formats entries with timestamps: `[HH:MM:SS]`
- Emoji indicators for different event types

**Example Output:**
```
---
[12:34:56] TOOL: Bash
COMMAND: ls -la
DESC: List files
---
[12:35:10] 💬 USER_PROMPT (chat)
TEXT: Add JWT authentication
---
[12:35:20] 🔒 PERMISSION: Bash(git push)
TOOL: Bash
RESOURCE: git push origin main
RESPONSE: approved
---
```

#### .claude/docs/ Protocol Documentation

**Optional but Recommended**: Local copies of key protocols for quick reference.

**Typical Contents:**
- `logging_setup.md` - Session management, /log-start, /log-complete
- `testing_protocol.md` - Progressive testing requirements
- `agent_usage.md` - Which agents to use when

**Purpose**: Code agent can reference protocols without reading architect skill references.

**Note**: These are copies/subsets of architect skill references, tailored to code agent needs.

#### Installation Verification

**Check settings.json location:**
```bash
ls -la .claude/settings.json  # MUST exist
ls .claude/hooks.json 2>&1 | grep "No such file"  # Should NOT exist
```

**Verify hook script:**
```bash
ls -la .claude/hook-logger.py  # MUST be executable
[ -x .claude/hook-logger.py ] && echo "✅ Executable" || echo "❌ Need chmod +x"
```

**Test hook configuration:**
```bash
python3 -m json.tool .claude/settings.json > /dev/null && echo "✅ Valid JSON" || echo "❌ Invalid JSON"
grep "PostToolUse" .claude/settings.json && echo "✅ Hooks configured"
```

**Related References:**
- `references/hook_configuration_critical.md` - Complete settings.json guide
- `references/hybrid_logging_protocol.md` - Hybrid logging v2.0 protocol
- `references/hook_logger_enhancements.md` - Enhanced argument capture

### 3.3 Debugging Directory Structure (Code Agent)

**Complete Structure:**
```
code-agent-workspace/
└── debugging/
    ├── logs/                        # Execution logs (timestamped sessions)
    │   ├── log-2025_01_21-14_30-task_description.md
    │   ├── log-2025_01_21-18_45-another_task.md
    │   └── session_20250121_143052.log
    │
    ├── instructions/                # Temporary instruction workspace
    │   └── current_instructions.md  # Single active instruction (0-1 files)
    │
    ├── scripts/                     # Session management scripts
    │   ├── log-start.sh            # Start logging session
    │   ├── log-complete.sh         # Complete logging session
    │   ├── log-decision.sh         # Log decisions/rationale
    │   └── get-unstuck.sh          # Resilience helper
    │
    ├── wrapper-scripts/            # OpenCode wrappers (optional)
    │   ├── run-with-logging.sh    # Main wrapper
    │   ├── log-tool-call.sh       # Pre-execution logging
    │   └── log-tool-result.sh     # Post-execution logging
    │
    └── current_log_file.txt        # Tracks active log file path
```

#### debugging/logs/ Directory

**Purpose**: Stores all execution logs created by code agent.

**File Naming Pattern:**
```
log-YYYY_MM_DD-HH_MM-description.md
```

**Requirements:**
- Description MUST match corresponding instruction file
- One log per session/instruction execution
- Never deleted (permanent record)

**Log File Structure:**
```markdown
# [TASK TITLE] - Execution Log
**Date:** 2025-01-21
**Start Time:** 14:30:00
**Agent:** Claude Code
**Task:** Implement authentication feature

---

## Execution Timeline

[14:30:15] TOOL: Bash
COMMAND: ls -la
DESC: List directory contents
---

[14:30:20] 🎯 DECISION: Use JWT for token generation
💭 RATIONALE: Matches existing auth patterns in codebase
---

[14:32:45] TOOL: Write
FILE: src/auth/jwt.py
SIZE: 1234 chars
---

[14:45:00] 🏁 Final Summary
**Status:** ✅ COMPLETE
---
```

**Log Correlation Example:**
```
# Architect instruction
instructions/instruct-2025_01_21-14_30-tkt123_implement_auth.md

# Code agent log (matching description)
debugging/logs/log-2025_01_21-14_35-tkt123_implement_auth.md
```

#### debugging/instructions/ Directory

**Purpose**: Temporary workspace for current instruction from architect.

**File Pattern:**
```
current_instructions.md    # Always ONE file maximum
```

**Lifecycle:**
1. **Created**: Architect copies instruction to this location
2. **Active**: Code agent reads and executes
3. **Score ≥95%**: Architect deletes file (success!)
4. **Score <95%**: Architect REPLACES file with improvement instructions
5. **Final**: Deleted after ≥95% achieved

**File Count Rules:**
- **0 files**: No active work from architect
- **1 file**: Current task in progress
- **Never >1 file**: Only one active instruction at a time

**Related Workflow**: See Section 4.4 (Iterative Improvement Workflow) for complete lifecycle.

#### debugging/scripts/ Directory

**Purpose**: Session management and logging helper scripts.

**log-start.sh** - Start logging session
```bash
# Usage
./debugging/scripts/log-start.sh "task-description"

# Creates
debugging/logs/log-YYYY_MM_DD-HH_MM-task_description.md
debugging/current_log_file.txt (points to active log)
```

**log-complete.sh** - Complete logging session
```bash
# Usage
./debugging/scripts/log-complete.sh

# Actions
- Finalizes active log with summary
- Removes current_log_file.txt
```

**log-decision.sh** - Log decisions and rationale
```bash
# Usage
./debugging/scripts/log-decision.sh <type> <message>

# Types
decision, rationale, investigation, verification, deviation, milestone

# Example
./debugging/scripts/log-decision.sh decision "Using JWT for auth tokens"
./debugging/scripts/log-decision.sh rationale "Matches existing patterns"
```

**get-unstuck.sh** - Resilience helper
```bash
# Usage
./debugging/scripts/get-unstuck.sh

# Purpose
- Provides troubleshooting steps when stuck
- Suggests MCP tools (Perplexity, Context7)
- Offers decision-making framework
```

#### debugging/wrapper-scripts/ Directory (Optional)

**Purpose**: OpenCode wrapper scripts for non-hook-enabled tools.

**When Needed:**
- OpenCode doesn't support hooks/plugins
- GEMINI, GitHub Copilot CLI, or other tools
- Compatible-mode architecture

**run-with-logging.sh** - Main wrapper
```bash
# Usage
./debugging/wrapper-scripts/run-with-logging.sh <command>

# Example
./debugging/wrapper-scripts/run-with-logging.sh "ls -la"

# Actions
1. Logs pre-execution (tool call)
2. Executes command
3. Logs post-execution (result)
```

**log-tool-call.sh** - Pre-execution logging
- Logs tool name and parameters
- Timestamp before execution

**log-tool-result.sh** - Post-execution logging
- Logs exit code
- Captures output
- Timestamp after execution

**See Section 6.2** for complete OpenCode wrapper architecture.

#### debugging/current_log_file.txt

**Purpose**: Tracks the active log file path.

**Format:**
```
debugging/logs/log-2025_01_21-14_30-task_description.md
```

**Usage:**
- Created by `log-start.sh`
- Read by `hook-logger.py` and wrapper scripts
- Deleted by `log-complete.sh`
- Enables hooks/wrappers to know where to write

**Lifecycle:**
```bash
# Session start
./debugging/scripts/log-start.sh "auth-feature"
# Creates: debugging/current_log_file.txt → debugging/logs/log-2025_01_21-14_30-auth_feature.md

# During work
# hook-logger.py reads current_log_file.txt to find active log

# Session end
./debugging/scripts/log-complete.sh
# Deletes: debugging/current_log_file.txt
```

**Related References:**
- `references/logging_protocol.md` - Complete logging requirements
- `references/instruction_grading_workflow.md` - File lifecycle
- `references/opencode_wrapper_setup.md` - Wrapper script details

---

### 3.4 Code Agent Workspace Template

**Purpose**: The `templates/code-agent-workspace/` directory provides a complete, ready-to-deploy workspace template for code agents with example files for testing and validation. Supports both Claude Code (hooks) and OpenCode (plugin) automatic logging.

**Complete Template Structure:**
```
templates/code-agent-workspace/
├── .claude/
│   ├── settings.json          # Hook configuration (Claude Code)
│   └── settings.local.json    # Cross-workspace permissions
│
├── .opencode/
│   ├── opencode.json          # OpenCode configuration (v1.0.85+)
│   ├── package.json           # Plugin dependencies
│   └── plugin/
│       └── logger.js          # Automatic logging plugin (13 event handlers)
│
├── debugging/
│   ├── logs/                  # Execution logs (empty initially)
│   ├── instructions/          # Temporary instruction workspace (empty initially)
│   ├── scripts/               # Session management scripts
│   │   ├── log-start.sh      # Start logging session
│   │   ├── log-complete.sh   # Complete logging session
│   │   ├── log-decision.sh   # Log decisions/rationale
│   │   └── get-unstuck.sh    # Resilience helper
│   ├── wrapper-scripts/       # OpenCode wrappers (optional, empty initially)
│   └── EVENT-REFERENCE.md     # OpenCode plugin event documentation
│
├── src/                       # Example source files for testing
│   └── example.ts            # Sample TypeScript file with functions/interfaces
│
├── CLAUDE.md                  # Code agent memory and protocols
└── AGENTS.md                  # Multi-agent collaboration guide
```

#### src/example.ts - Testing Template

**Purpose**: Provides a sample TypeScript file that code agents can modify, test, and extend during instruction execution.

**Use Cases:**
1. **Testing logging protocol**: Modify example.ts and verify logs capture changes
2. **Validating testing workflow**: Run tests on example functions
3. **Demonstrating agent capabilities**: Show code modification in action
4. **Template for new projects**: Starting point for TypeScript projects

**File Contents:**
```typescript
/**
 * Example TypeScript file for testing code agent execution
 *
 * Includes:
 * - Basic functions (add, multiply, greet)
 * - Interface definition (User)
 * - User management functions
 * - Email validation
 * - Main function for testing
 */
```

**How to Use:**
```bash
# From code agent workspace
cd src/

# 1. View example file
cat example.ts

# 2. Test execution (if Node.js installed)
npx ts-node example.ts

# 3. Modify during instruction execution
# Code agent receives instruction: "Add a subtract function to example.ts"
# Agent modifies, tests, logs changes
```

**Related Files:**
- `debugging/scripts/log-start.sh` - Start logging before modifications
- `debugging/scripts/log-decision.sh` - Log why changes were made
- `debugging/scripts/log-complete.sh` - Finalize log after testing

#### .opencode/ - OpenCode Plugin Configuration

**Purpose**: Provides automatic logging for OpenCode AI code agent with 13 event handlers.

**Structure:**
```
.opencode/
├── opencode.json       # OpenCode configuration (v1.0.85+ schema)
├── package.json        # Plugin dependencies (@opencode-ai/plugin)
└── plugin/
    └── logger.js       # Automatic logging plugin (SPEC.md v4.1 compliant)
```

**Plugin Features:**
- ✅ **13 event handlers** - Tools, Sessions, Files, Commands, Permissions, TUI, TODOs
- ✅ **Zero token cost** - All logging happens in background hooks
- ✅ **60-70% token savings** - Compared to manual logging
- ✅ **SPEC.md compliant** - File naming follows Section 2.5 (`log-YYYY_MM_DD-HH_MM-description.md`)
- ✅ **Same log format** - Identical to Claude Code hook output (enables identical grading)

**Event Categories:**
1. Tool Events (2) - `tool.execute.before`, `tool.execute.after`
2. Session Events (6) - `created`, `updated`, `deleted`, `status`, `error`, `idle`
3. File Events (2) - `file.edited`, `file.watcher.updated`
4. Command Events (1) - `command.executed`
5. Permission Events (2) - `permission.replied`, `permission.updated`
6. TUI Events (3) - `tui.prompt.append`, `tui.command.execute`, `tui.toast.show`
7. TODO Events (1) - `todo.updated`

**Configuration:**
```bash
# Set log level
export OPENCODE_LOG_LEVEL=VERBOSE  # or ESSENTIAL

# Disable specific events
export OPENCODE_LOG_TUI=false              # Disable TUI events
export OPENCODE_LOG_FILE_WATCHER=false     # Disable file watcher
```

**How It Works:**
1. Plugin loads automatically when OpenCode starts
2. Reads active log file from `debugging/current_log_file.txt`
3. Appends formatted entries to active log in background
4. Same format as Claude Code hooks (enables identical grading)

**Documentation:** See `debugging/EVENT-REFERENCE.md` for complete event reference.

**Template Philosophy:**
- **Self-contained**: All scripts and examples work out of the box
- **SPEC-compliant**: File naming follows Section 2.5 conventions
- **Educational**: Demonstrates proper logging and testing workflows
- **Extensible**: Easy to add more examples (Python, Java, etc.)
- **Dual-mode**: Supports both Claude Code (hooks) and OpenCode (plugin) automatic logging

---

## 4. Key Workflows

### 4.1 Instruction Creation Workflow

**Trigger:** User says "write instructions for code agent"

**Steps:**
1. **Prerequisite check**: Verify architect agent directories exist
2. **Receive user request**: Clarify if brainstorming or implementation
3. **Architecture planning**: Research, provide alternatives, get approval
4. **Create TWO files**:
   - `human/human-YYYY_MM_DD-HH_MM-ticket_phase_description.md` (concise summary)
   - `instructions/instruct-YYYY_MM_DD-HH_MM-ticket_phase_description.md` (detailed technical)
5. **Mandatory sections** in instructions:
   - Logging requirements (ALWAYS first)
   - Testing protocol (progressive, coverage ≥60%)
   - Required agents (qa-enforcer MANDATORY)
   - Success criteria (10-15 checkboxes)
   - Resilience requirements
   - Exact commands with flags

**Quality Standards:**
- Instructions must be **self-contained** (code agent doesn't need to ask questions)
- Success criteria must be **measurable** (not subjective)
- Logging MUST use `tee` for real-time capture
- Testing MUST be progressive (after every 10-50 lines, not just at end)

---

### 4.2 Workspace Initialization Workflow

**Trigger:** User says "this is a new architect agent, help me set it up"

**Steps:**
1. **Prerequisite check**: Verify directories do NOT already exist
2. **Confirm code agent location**: Ask user for path
3. **Create directories**: instructions/, human/, grades/, analysis/, ticket/
4. **Create CLAUDE.md**: Include code agent workspace path
5. **Confirm setup complete**: Display structure, remind user of next steps

**Automated Alternative:**
- Use `templates/setup-workspace.sh` for instant setup
- Includes all protocols, scripts, configurations
- Recommended for most users

---

### 4.3 Grading Workflow

**Trigger:** User says "grade the code agent's work"

**Steps:**
1. **Prerequisite check**: Verify architect agent directories exist
2. **Read logs**: Review code agent's `debugging/logs/` directory
3. **Apply rubric**: Score against 6 categories (100 points total)
4. **Check requirements**:
   - Every action has verification?
   - Tests run progressively (not just at end)?
   - Coverage ≥60%?
   - Correct agents used?
   - CI/CD changes tested?
5. **Create grade file**: `grades/grade-YYYY_MM_DD-HH_MM-same_description.md`
6. **Determine outcome**:
   - Score ≥95%: Success, delete instruction
   - Score <95%: Create improvement instruction

**Grading Rubric (100 Points):**
| Category | Points | Key Criteria |
|----------|--------|--------------|
| Completeness | 25 | All requirements met, success criteria checked |
| Code Quality | 20 | Best practices, maintainability, correctness |
| Testing & Verification | 20 | Coverage ≥60%, all actions verified |
| Documentation | 15 | Complete logs, change docs, inline comments |
| Resilience & Adaptability | 10 | Recovery from errors, smart workarounds |
| Logging & Traceability | 10 | Real-time logs with tee, timestamps |

**Automatic Grade Caps:**
- No unit tests run: Max D (65%)
- Tests fail: F (50%) - UNACCEPTABLE
- Coverage <60%: Max C- (70%)
- No logs: Max C+ (78%)
- CI/CD not tested: Max C+ (78%)

---

### 4.4 Iterative Improvement Workflow (NEW in v3.0)

**Trigger:** User says "send instructions to code agent" (after creating instructions)

**Steps:**
1. **Copy instruction**: `instructions/instruct-*.md` → code agent's `debugging/instructions/<uuid>-YYYYMMDD-HHMM.md`
2. **Generate UUID**: 6-character hex (e.g., `a1b2c3`)
3. **Create summary**: Adaptive 3-15 point summary
4. **Code agent executes**: Implements work, creates logs
5. **Grade work**: Apply rubric
6. **Outcome A (≥95%)**: Delete instruction, update CLAUDE.md with success patterns
7. **Outcome B (<95%)**:
   - Rename old instruction: `<uuid>-...-graded-<score>.md`
   - Create improvement instruction: new UUID, targeted fixes
   - Update CLAUDE.md with failure patterns to avoid
8. **Repeat**: Until ≥95% achieved

**Cleanup:**
- Score ≥95%: Instruction deleted immediately
- Score <95%: Old graded files deleted on next grading cycle
- Maximum 0-2 files in debugging/instructions/ at any time

---

### 4.5 Memory Management Workflow

**When code agent completes work:**
- Architect updates code agent's CLAUDE.md with learnings
- Architect updates code agent's AGENTS.md if workflow patterns emerged

**What to document:**

**After failures (score <95% multiple times):**
```markdown
### Error Handling Patterns (Auto-Updated)

**Added: 2025-01-21 - Auth feature (82% → 96%)**
- ❌ Don't: Skip integration test error cases
- ✅ Do: Test both success and failure scenarios
- 💡 Learning: Always test invalid credentials, expired tokens, missing headers
```

**After successes (score ≥98%):**
```markdown
### Testing Requirements (Auto-Updated)

**Added: 2025-01-21 - Database refactor (98%)**
- Pattern: TDD with comprehensive edge cases
- Approach: Write tests first, then implementation
- Reuse: Apply to all database-touching features
```

---

## 5. Quality Standards

### Instruction Quality

**MUST HAVE:**
- Logging requirements (FIRST section, uses `tee`)
- Testing protocol (progressive schedule, coverage target)
- Required agents (qa-enforcer MANDATORY)
- Success criteria (10-15 measurable items)
- Resilience instructions (error recovery patterns)
- Exact commands (no "run appropriate commands")

**MUST NOT HAVE:**
- AI attribution (no mentions of Claude, AI, automation)
- Vague requirements ("ensure good quality")
- Subjective criteria ("make it look nice")
- Batch logging (MUST be real-time with `tee`)

---

### Grading Quality

**MUST BE:**
- **Objective**: Based on measurable criteria
- **Evidence-based**: Cite logs, code, test results
- **Consistent**: Apply same rubric to all work
- **Constructive**: Explain what's missing, not just score

**MUST HAVE:**
- Breakdown by all 6 categories
- Specific evidence from logs/code
- Clear point deductions with reasoning
- Improvement guidance if <95%

---

### Code Agent Workspace Standards

**MUST HAVE:**
- `CLAUDE.md` with architect learnings section
- `AGENTS.md` with delegation protocol
- `debugging/logs/` directory for execution logs
- `debugging/instructions/` directory (temporary, 0-2 files max)
- `.claude/settings.json` with hooks (if using hooks)

**MUST NOT HAVE:**
- Architect agent files (instructions/, grades/, human/)
- Duplicate protocol documentation (reference architect skill instead)

---

## 6. Technology Integration

### OpenCode Support (v3.0+)

**What it provides:**
- Dual-mode logging (works in Claude Code AND OpenCode)
- Wrapper scripts for OpenCode compatibility
- Shell initialization functions

**How it works:**
- Hooks write to logs in Claude Code
- Wrapper scripts write to logs in OpenCode
- Same log format regardless of tool

**See:** references/opencode_integration_quickstart.md

---

### 6.1 Hook Event Types and Lifecycle

**Claude Code Hook System**: Event-driven architecture for automated logging and monitoring.

#### Available Hook Events

**Comprehensive Event Catalog:**

| Event Type | Timing | Use Case | Grading Impact | Status |
|------------|--------|----------|----------------|--------|
| **PostToolUse** | After tool execution | Automated logging (current) | **GRADED** (10 pts) | ✅ Implemented v3.0 |
| **SessionStart** | Session initialization | Session tracking | **GRADED** (2 pts) | ✅ Implemented v4.1 |
| **SessionEnd** | Session termination | Session analytics | **GRADED** (implicit) | ✅ Implemented (implicit) |
| **UserPromptSubmit** | Before Claude processes user input | Log user requests, audit trail | NON-GRADED (audit) | ✅ Implemented v4.1 |
| **PermissionRequest** | When permission needed | Permission audit, optimization data | NON-GRADED (optimization) | ✅ Implemented v4.1 |
| **PreToolUse** | Before tool execution | Pre-execution logging | NON-GRADED | ✅ Implemented v4.1 |
| **Notification** | System notifications | All notification types | NON-GRADED | ✅ Implemented v4.1 |
| **Stop** | Agent completion | Agent lifecycle tracking | NON-GRADED | ✅ Implemented v4.1 |
| **SubagentStop** | Subagent completion | Subagent lifecycle tracking | NON-GRADED | ✅ Implemented v4.1 |
| **PreCompact** | Before compact | Compact lifecycle tracking | NON-GRADED | ✅ Implemented v4.1 |

**Grading vs Non-Graded Events:**
- **GRADED Events** (PostToolUse, SessionStart/End): Required for passing grades, directly impact scores
- **NON-GRADED Events** (all others): Optional extras that provide audit trail and optimization data but don't affect grading

**Critical Optimization Feature**: PermissionRequest logging enables identifying which permissions should be pre-approved in `.claude/settings.local.json` to eliminate permission prompts and speed up code agent execution.

#### Current Implementation (v4.1)

**Core Graded Events (Required for Passing Grades):**
- ✅ **PostToolUse** - Fully implemented in hook-logger.py (GRADED: 10 points)
- ✅ **SessionStart** - Explicit event hook in settings.json (GRADED: 2 points)
- ✅ **SessionEnd** (implicit) - Via /log-complete or log-complete.sh (GRADED)

**Enhanced Non-Graded Events (Optional Extras):**
- ✅ **UserPromptSubmit** - Captures all user prompts (NON-GRADED: audit trail)
- ✅ **PermissionRequest** - Tracks permission requests (NON-GRADED: **CRITICAL for optimization**)
- ✅ **PreToolUse** - Before tool execution (NON-GRADED: pre-execution logging)
- ✅ **Notification** - All notification types (NON-GRADED: system events)
- ✅ **Stop** - Agent completion events (NON-GRADED: lifecycle tracking)
- ✅ **SubagentStop** - Subagent completion events (NON-GRADED: lifecycle tracking)
- ✅ **PreCompact** - Before compaction (NON-GRADED: lifecycle tracking)

**Configuration**: All 9 event types configured in `.claude/settings.json`, processed by single `hook-logger.py` script.

**Future Enhancements:**
- ⏳ **ErrorOccurred** - Comprehensive error logging (Planned v5.0)
- ⏳ **ToolError** - Detailed tool failure tracking (Planned v5.0)

#### Enhanced Event Details (v4.1)

##### UserPromptSubmit Event (NON-GRADED)

**Status**: ✅ Implemented v4.1

**User Requirement**: "LOG ALL TYPES OF NOTIFICATION... UserPromptSubmit - Runs when the user submits a prompt, before Claude processes it."

**Purpose**: Log every user prompt BEFORE Claude processes it.

**Use Cases:**
1. **Audit Trail**: Complete record of user interactions
2. **Context Analysis**: Understand what prompts lead to success/failure
3. **Workflow Tracking**: Map user requests to instruction creation
4. **Grading Context**: Know what user originally requested (NON-GRADED)

**Log Format:**
```markdown
---
[14:30:45] 💬 USER_PROMPT (chat)
TEXT: Please implement JWT authentication
---
```

##### PermissionRequest Event (NON-GRADED - CRITICAL for Optimization)

**Status**: ✅ Implemented v4.1

**Purpose**: Track all permission requests to identify which permissions should be pre-approved.

**Critical Use Case**: By analyzing permission logs, architects can identify frequently-requested permissions and add them to `.claude/settings.local.json` to eliminate permission prompts and significantly speed up code agent execution.

**Log Format:**
```markdown
---
[14:32:10] 🔒 PERMISSION: Bash(git push)
TOOL: Bash
RESOURCE: git push origin main
RESPONSE: approved
---
```

**Optimization Workflow:**
1. Run code agent on several tasks with permission logging enabled
2. Review logs to identify frequently-approved permissions
3. Add patterns to `.claude/settings.local.json` for auto-approval
4. Code agent runs faster without permission prompts

**Example settings.local.json Entry:**
```json
{
  "permissions": {
    "Bash(git push:*)": "always_allow",
    "Bash(git add:*)": "always_allow",
    "Bash(npm install:*)": "always_allow"
  }
}
```

#### Complete Hook Configuration (v4.1)

**Status**: ✅ Production-ready, 60-70% token savings achieved

**Full Configuration** (.claude/settings.json with all 9 event types):
```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ],
    "PreToolUse": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ],
    "UserPromptSubmit": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
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
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ],
    "Notification": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ],
    "SessionStart": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ],
    "PreCompact": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ],
    "Stop": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ],
    "SubagentStop": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hook-logger.py",
            "timeout": 5
          }
        ]
      }
    ]
  }
}
```

**Event Categories:**
- **Graded Events**: PostToolUse, SessionStart (required for passing grades)
- **Non-Graded Events**: PreToolUse, UserPromptSubmit, PermissionRequest, Notification, PreCompact, Stop, SubagentStop (optional extras)

**Note**: All 9 event types are processed by the same `hook-logger.py` script. The script detects the event type from the JSON payload and formats the log entry accordingly.

**Supported Tools:**
- Bash (command + description)
- Read (file_path + offset/limit)
- Write (file_path + content size)
- Edit (file_path + old/new strings)
- Grep (pattern + path + glob + mode)
- Glob (pattern + path)
- TodoWrite (todo count + status summary)

**Token Savings Analysis:**
- **Manual logging**: ~100 tokens per tool
- **Automated logging**: ~25 tokens per tool
- **Savings**: ~75 tokens per tool call (75%)
- **Per session (30 tools)**: ~2,250 tokens saved

#### Hook Configuration Best Practices

**DO:**
- ✅ Configure hooks in `.claude/settings.json` (NOT hooks.json)
- ✅ Use `"matcher": "*"` to capture all tools
- ✅ Set reasonable timeout (5-10 seconds)
- ✅ Make hook scripts executable (`chmod +x`)
- ✅ Implement silent failure in hook scripts
- ✅ Check for active log session before writing
- ✅ Log UserPromptSubmit for audit trail (v4.1+)

**DON'T:**
- ❌ Put hooks in `.claude/hooks.json` (not read by Claude Code)
- ❌ Use long timeouts (>15 seconds) - slows tool execution
- ❌ Allow hook failures to break tool execution
- ❌ Log when no active session (creates orphan logs)
- ❌ Throw away wrapper knowledge (compatible-mode needed)

#### Future Event Expansions (v5.0+)

**Planned Event Types:**

1. **ErrorOccurred** - Comprehensive error logging
   - Use case: Root cause analysis, error pattern detection
   - Status: Planned for v5.0

2. **ToolError** - Detailed tool failure tracking
   - Use case: Tool reliability metrics, failure patterns
   - Status: Planned for v5.0

**Note**: PreToolUse, UserPromptSubmit, PermissionRequest, Notification, Stop, SubagentStop, and PreCompact events requested by user have all been implemented in v4.1 as non-graded extras.

**Related References:**
- `references/hook_configuration_critical.md` - Complete settings.json guide
- `references/hook_logger_enhancements.md` - Enhanced argument capture
- `references/hybrid_logging_protocol.md` - Hybrid logging v2.0

### 6.2  Wrapper Architecture (Compatible-Mode)

Provide this as an option, include the script under debugging, but do not force it. 
By default, tools that do not support plugin or hooks will just use the scripts. 

**Purpose**: Enable automated logging for AI tools that don't support hooks/plugins (OpenCode, GEMINI, GitHub Copilot CLI).

**Architecture**: Bash wrapper scripts that intercept tool calls and log pre/post execution.

#### Compatible-Mode Design Principles

**Problem**: Not all AI coding assistants support hook systems like Claude Code.
- OpenCode: May or may not support plugins (version-dependent)
- GEMINI: No documented hook system
- GitHub Copilot CLI: Limited extensibility
- Other tools: Unknown plugin architectures

**Solution**: Bash wrapper scripts that work with ANY command-line tool.

**Design Goals:**
1. **Universal compatibility** - Works with any CLI tool
2. **Same log format** - Identical to Claude Code hook output
3. **Minimal overhead** - <100ms per command
4. **Zero config** - No tool-specific setup
5. **Graceful degradation** - Falls back to direct execution on error

#### Wrapper Script Architecture

**Directory Structure:**
```
debugging/wrapper-scripts/
├── run-with-logging.sh        # Main wrapper (orchestrates logging)
├── log-tool-call.sh           # Pre-execution logger
└── log-tool-result.sh         # Post-execution logger
```

**Execution Flow:**
```
User/AI → run-with-logging.sh
   ↓
   1. log-tool-call.sh (PRE)
   2. Execute command
   3. log-tool-result.sh (POST)
   ↓
Output → User/AI
```

#### run-with-logging.sh (Main Wrapper)

**Purpose**: Orchestrate command execution with pre/post logging.

**Key Implementation:**
```bash
#!/bin/bash
# Check for active log session
if [ ! -f debugging/current_log_file.txt ]; then
    echo "❌ Error: No active logging session." >&2
    exit 1
fi

# Log the tool call (pre-execution)
./debugging/wrapper-scripts/log-tool-call.sh "Bash" "$COMMAND"

# Execute command and capture output
TEMP_OUTPUT=$(mktemp)
eval "command $COMMAND" > "$TEMP_OUTPUT" 2>&1
EXIT_CODE=$?

# Log result (post-execution)
./debugging/wrapper-scripts/log-tool-result.sh "$EXIT_CODE" "$TEMP_OUTPUT"

# Display output and exit with original code
cat "$TEMP_OUTPUT"
rm -f "$TEMP_OUTPUT"
exit $EXIT_CODE
```

**Key Features:**
- ✅ Checks for active session
- ✅ Captures stdout + stderr
- ✅ Preserves exit codes
- ✅ Uses temporary files (no race conditions)
- ✅ Cleans up after itself
- ✅ `command` prefix prevents recursion

#### Recursion Prevention

**Critical Pattern**: Use `command` builtin to bypass shell functions/aliases.

**Problem Without `command`:**
```bash
# If ls is aliased to wrapper
alias ls='./wrapper-scripts/run-with-logging.sh ls'

# This causes infinite recursion
ls  →  wrapper  →  ls  →  wrapper  →  ls  →  ∞
```

**Solution With `command`:**
```bash
# In wrapper script
command ls  # Bypasses aliases, runs actual ls command
```

**Why This Works**: `command` builtin forces shell to use the actual binary, not aliases or functions.

#### Dual-Mode Architecture (Hooks + Wrappers)

**Hybrid Workspace**: Support BOTH Claude Code AND OpenCode in same workspace.

**Directory Structure:**
```
code-agent-workspace/
├── .claude/                      # Claude Code hooks
│   ├── settings.json
│   └── hook-logger.py
├── debugging/
│   ├── scripts/                  # Universal session management
│   │   ├── log-start.sh
│   │   └── log-complete.sh
│   └── wrapper-scripts/          # OpenCode/compatible-mode
│       └── run-with-logging.sh
```

**Usage:**
- **Claude Code**: Uses hooks automatically (via settings.json)
- **OpenCode**: Uses wrapper scripts (via run-with-logging.sh)
- **Both**: Same log format, same log files, same grading

#### GEMINI and GitHub Copilot CLI Support

**GEMINI Compatible-Mode:**
```bash
# GEMINI doesn't support hooks
# Use wrappers for all commands

./debugging/scripts/log-start.sh "gemini-task"
./debugging/wrapper-scripts/run-with-logging.sh "pytest tests/"
./debugging/wrapper-scripts/run-with-logging.sh "git commit -m 'fix'"
./debugging/scripts/log-complete.sh
```

**GitHub Copilot CLI Compatible-Mode:**
```bash
# Copilot CLI has limited extensibility
# Use wrappers for automated logging

./debugging/scripts/log-start.sh "copilot-task"
./debugging/wrapper-scripts/run-with-logging.sh "gh copilot suggest 'list files'"
./debugging/wrapper-scripts/run-with-logging.sh "git status"
./debugging/scripts/log-complete.sh
```

**Grading Note**: Logs from GEMINI/Copilot CLI are graded identically to Claude Code logs. Same rubric, same standards.

#### Comparison: Hooks vs Wrappers

| Feature | Claude Code Hooks | OpenCode Wrappers |
|---------|------------------|-------------------|
| **Setup** | settings.json configuration | Shell scripts |
| **Transparency** | Fully automatic | Requires wrapper call |
| **Tool Support** | Claude Code tools only | ANY CLI command |
| **Performance** | ~5ms overhead | ~50-100ms overhead |
| **Compatibility** | Claude Code only | Universal (works with any tool) |
| **Maintenance** | Update hook script | Update wrapper scripts |
| **Log Format** | Identical | Identical |
| **Token Savings** | 60-70% | 60-70% (same) |

#### Best Practices

**DO:**
- ✅ Use wrappers for non-hook-enabled tools
- ✅ Check for active session before logging
- ✅ Preserve exit codes
- ✅ Capture both stdout and stderr
- ✅ Clean up temporary files
- ✅ Use same log format as hooks
- ✅ Document wrapper usage in CLAUDE.md
- ✅ Use `command` builtin to prevent recursion

**DON'T:**
- ❌ Throw away wrapper knowledge (user emphasized this)
- ❌ Assume all tools support hooks
- ❌ Modify wrapper scripts without testing
- ❌ Let wrapper failures break commands
- ❌ Create different log formats for wrappers vs hooks
- ❌ Forget recursion prevention with `command`

**Related References:**
- `references/opencode_wrapper_setup.md` - Complete wrapper setup
- `references/opencode_integration_quickstart.md` - Quick migration guide
- `references/opencode_logging_protocol.md` - OpenCode-specific protocol
- `references/claude_vs_opencode_comparison.md` - Feature comparison

### 6.3 Event Logging Matrix

**Purpose**: Comprehensive reference for what events to log, their priorities, and impact on grading/tokens.

#### Event Priority Matrix

| Event Type | Priority | Log Always? | Token Impact | Grading Impact | Implementation |
|------------|----------|-------------|--------------|----------------|----------------|
| **UserPromptSubmit** | CRITICAL | ✅ Yes | ~10 tokens | +2 pts (Completeness) | ⏳ v4.1 |
| **PostToolUse** | CRITICAL | ✅ Yes | ~5 tokens (auto) | +8 pts (Logging & Traceability) | ✅ v3.0 |
| **ErrorOccurred** | CRITICAL | ✅ Yes | ~15 tokens | +2 pts (Resilience) | ⏳ v4.0 |
| **SessionStart** | HIGH | ✅ Yes | ~10 tokens | +1 pt (Logging) | ✅ v3.0 |
| **SessionEnd** | HIGH | ✅ Yes | ~20 tokens | +1 pt (Logging) | ✅ v3.0 |
| **ToolError** | HIGH | ✅ Yes | ~20 tokens | +2 pts (Resilience) | ⏳ v4.0 |
| **Decision** | HIGH | ✅ Yes | ~15 tokens (manual) | +3 pts (Resilience) | ✅ v2.0 |
| **Verification** | MEDIUM | ✅ Yes | ~15 tokens (manual) | +5 pts (Testing & Verification) | ✅ v2.0 |
| **Investigation** | MEDIUM | ✅ Yes | ~20 tokens (manual) | +2 pts (Resilience) | ✅ v2.0 |
| **Milestone** | MEDIUM | ⚠️ Optional | ~10 tokens (manual) | +1 pt (Completeness) | ✅ v2.0 |
| **Notification** | HIGH | ✅ Yes | ~10 tokens | Varies | ⏳ v4.0 |

**Key Insight**: Automated logging (PostToolUse) provides 60-70% token savings while comprehensive event coverage ensures maximum grading scores.

#### Token Impact Analysis

**Per-Session Token Costs (30-command session):**

**Without Logging (Baseline):**
- Agent work: ~5,000 tokens
- Total: ~5,000 tokens

**With Manual Logging (v1.0):**
- Agent work: ~5,000 tokens
- Manual logging overhead: ~3,000 tokens
- Total: ~8,000 tokens (+60% overhead)

**With Automated Logging (v3.0 - Current):**
- Agent work: ~5,000 tokens
- Automated logging overhead: ~750 tokens
- Total: ~5,750 tokens (+15% overhead)
- **Savings: 2,250 tokens per session (75% reduction in logging overhead)**

**Projected with All Events (v4.0+):**
- Agent work: ~5,000 tokens
- Automated logging (PostToolUse, ToolError, PreToolUse): ~750 tokens
- UserPromptSubmit (5 prompts): ~50 tokens
- ErrorOccurred (2 errors): ~30 tokens
- Notifications: ~20 tokens
- Total: ~5,850 tokens (+17% overhead)
- **Additional overhead: ~100 tokens for comprehensive event logging (2%)**

#### Grading Impact by Event Type

**How Event Logging Affects Scores:**

| Event Type | Grading Category | Points Impact | Required? |
|------------|------------------|---------------|-----------|
| **PostToolUse** | Logging & Traceability (10 pts) | +8 points if present | ✅ Required |
| **Decision** | Resilience & Adaptability (10 pts) | +3 points per decision | ✅ Required |
| **Verification** | Testing & Verification (20 pts) | +5 points per verification | ✅ Required |
| **Investigation** | Resilience & Adaptability (10 pts) | +2 points per investigation | ⚠️ If errors occur |
| **UserPromptSubmit** | Completeness (25 pts) | +2 points (requirements clarity) | ⚠️ Optional (v4.1) |
| **ToolError** | Resilience & Adaptability (10 pts) | +2 points per recovery | ⚠️ If errors occur |
| **Milestone** | Completeness (25 pts) | +1 point per milestone | ⚠️ Optional |

**Scoring Examples:**

**Grade: 97% (Excellent Logging)**
- All tool calls logged via PostToolUse ✅ (+8 pts)
- Decisions logged with rationale ✅ (+3 pts)
- Errors investigated and resolved ✅ (+2 pts)
- All actions verified ✅ (+5 pts)
- Milestones documented ✅ (+1 pt)

**Grade: 78% (Poor Logging - Missing Events)**
- Some tool calls logged ⚠️ (-2 points)
- No decision logging ❌ (-3 points)
- No verification logs ❌ (-5 points)
- Errors occurred but not investigated ❌ (-2 points)

#### Manual vs Automated Event Types

**Automated (Hooks/Wrappers):**
- PostToolUse ✅
- PreToolUse (v4.0) ⏳
- ToolError (v4.0) ⏳
- ErrorOccurred (v4.0) ⏳
- UserPromptSubmit (v4.1) ⏳

**Manual (Scripts):**
- Decision - `./debugging/scripts/log-decision.sh decision "..."`
- Rationale - `./debugging/scripts/log-decision.sh rationale "..."`
- Investigation - `./debugging/scripts/log-decision.sh investigation "..."`
- Verification - `./debugging/scripts/log-decision.sh verification "..."`
- Milestone - `./debugging/scripts/log-decision.sh milestone "..."`
- Deviation - `./debugging/scripts/log-decision.sh deviation "..."`

**Why Manual?**
- Require human judgment (what decision was made?)
- Context-dependent (why this approach?)
- Analysis-driven (what investigation revealed?)
- **Automated logging can't replace human reasoning documentation**

#### UserPromptSubmit Detailed Specification

**User Emphasis**: "LOG ALL TYPES OF NOTIFICATION... UserPromptSubmit"

**Purpose**: Capture every user request BEFORE Claude processes it.

**What to Capture:**
```json
{
  "event": "UserPromptSubmit",
  "timestamp": "2025-01-21T14:30:45Z",
  "session_id": "abc123",
  "prompt_type": "chat" | "slash_command" | "file_analysis" | "skill_trigger",
  "prompt": "write instructions for code agent",
  "context": {
    "workspace": "/path/to/workspace",
    "active_files": ["CLAUDE.md", "README.md"],
    "previous_prompts_count": 5
  }
}
```

**Log Format:**
```markdown
---
[14:30:45] USER_PROMPT
TYPE: chat
PROMPT: "write instructions for code agent"
SESSION: abc123
CONTEXT: workspace=/path/to/workspace, files=2
---
```

**Grading Benefit:**
- Architect can see original user intent
- Verify code agent understood requirements
- Track prompt → instruction → implementation flow
- Complete audit trail for quality assurance

#### Event Logging Best Practices

**DO:**
- ✅ Log ALL critical events (UserPromptSubmit, PostToolUse, ErrorOccurred)
- ✅ Log HIGH priority events (SessionStart/End, ToolError, Decisions)
- ✅ Use automated logging when possible (hooks/wrappers)
- ✅ Include timestamps on EVERY entry
- ✅ Capture exit codes and error messages
- ✅ Document decisions and rationale
- ✅ Verify actions and log results
- ✅ Log investigations when troubleshooting

**DON'T:**
- ❌ Skip logging critical events
- ❌ Batch logs at end of session
- ❌ Log LOW priority events by default
- ❌ Forget to log UserPromptSubmit (v4.1+)
- ❌ Log without timestamps
- ❌ Assume logging overhead is too high (it's only 2%)

**Related References:**
- `references/logging_protocol.md` - Complete logging requirements
- `references/hook_configuration_critical.md` - Hook setup
- `references/grading_rubrics.md` - How logging affects grades

---

## 7. Evolution & Versioning

### Version History

**v1.0 (Initial):** Manual logging, basic workflows
**v2.0 (Hybrid):** Hybrid logging, improved protocols
**v3.0 (Hooks Fix):** Settings.json hooks, automated setup, templates
**v4.0 (Reorganization):** Comprehensive reorganization, all references linked, seamless UX
**v4.1 (Drift Elimination):** Complete file naming/correlation specs, .claude/ directory documentation, debugging/ structure, comprehensive hook event types (10+), OpenCode wrapper architecture, event logging matrix - eliminates all drift between spec and implementation

### Future Considerations

**Potential additions:**
- Multi-code-agent orchestration (one architect → many code agents)
- Ticket integration (Jira, Linear, GitHub Issues)
- Metrics dashboard (grade trends, time to ≥95%, common failures)
- AI model variations (different models for architect vs code agent)

**NOT planned:**
- IDE integration (this is a CLI skill)
- Language-specific implementations (architecture patterns are language-agnostic)
- Production deployment tooling (out of scope)

---

## 8. Decision Framework

### When to Add a New Feature

**Ask:**
1. Does it align with core purpose (plan, delegate, grade, iterate, learn)?
2. Does it fit within scope boundaries?
3. Does it respect the architecture (3-level progressive disclosure)?
4. Does it maintain quality standards?
5. Is there a clear protocol to document it?

**If yes to all:** Proceed
**If no to any:** Reconsider or adjust scope

---

### When to Update This Spec

**Update SPEC.md when:**
- Core purpose or scope changes
- New major workflow added
- Architecture significantly revised
- Quality standards updated
- Version increment

**Do NOT update for:**
- Minor protocol tweaks (update reference files instead)
- Bug fixes (update relevant docs)
- Template changes (update templates/README.md)

---

## 9. Success Criteria

### For the Skill

**Skill is successful when:**
- ✅ Users can trigger all workflows without manual file navigation
- ✅ Progressive disclosure works (context stays lean)
- ✅ All reference files are discoverable from SKILL.md
- ✅ Template automation is easy to find and use
- ✅ Grading is objective and consistent
- ✅ Code agents reach ≥95% on iterations (not stuck at 70-80%)
- ✅ Documentation is clear, complete, and cross-referenced

**Metrics:**
- 0% orphaned reference files (all linked)
- <5k words in SKILL.md body (stay lean)
- All 7 triggers work seamlessly
- Setup time <5 minutes (using templates)

---

### For Users

**Users are successful when:**
- ✅ Can set up architect + code agent workspaces in <5 minutes
- ✅ Create instructions that code agents understand first time
- ✅ Grade objectively without subjective bias
- ✅ Iterate to ≥95% within 1-3 cycles
- ✅ Accumulate learnings in code agent memory
- ✅ Use appropriate agents for each task
- ✅ Don't have to "coax" the skill to work

---

## 10. Maintenance Guidelines

### Regular Reviews

**Monthly:**
- Check for orphaned reference files
- Validate cross-references still work
- Update SKILL.md reference section if new files added

**Quarterly:**
- Review grading rubric for relevance
- Update version history
- Check SPEC.md alignment with actual implementation

**Annually:**
- Major version increment review
- Architecture reevaluation
- Quality standards refresh

---

### Adding New References

**When adding a new reference file:**
1. Place in `references/` directory
2. Add to `references/README.md` index
3. Add to SKILL.md "Reference Documents" section (correct category)
4. Cross-reference from related files
5. Update SPEC.md if it represents new capability

---

### Deprecating Features

**When deprecating:**
1. Mark as deprecated in SKILL.md with date
2. Provide migration path to replacement
3. Keep deprecated docs for 2 versions
4. Remove after 2 versions
5. Update SPEC.md version history

---

## 11. Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for:
- Git workflow (branch → issue → PR)
- Commit message format
- PR requirements
- Review process

---

## Appendix: Key Principles

1. **Separation of Concerns**: Architect plans/evaluates, code agent implements
2. **Objective Grading**: Measurable criteria, evidence-based scoring
3. **Iterative Improvement**: Work continues until ≥95% threshold
4. **Progressive Disclosure**: Minimize context, load as needed
5. **Real-Time Logging**: Use `tee`, never batch logs at end
6. **Progressive Testing**: After every 10-50 lines, not just at end
7. **Right Agent for Job**: qa-enforcer validates, doesn't create docs
8. **Template Automation**: Make setup effortless with templates
9. **Memory Accumulation**: Code agent learns from outcomes
10. **Seamless UX**: Users shouldn't have to "coax" it to work

---

**End of Specification**
**Last Updated:** 2025-01-21
**Version:** 4.1
