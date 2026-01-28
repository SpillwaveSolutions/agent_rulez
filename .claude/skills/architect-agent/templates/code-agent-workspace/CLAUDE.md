# CLAUDE.md - Code Agent Workspace

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## 🆕 NEW: Essential Protocols (Read These First!)

**Location:** `.claude/docs/` (this directory)

- **[Logging Setup](./.claude/docs/logging_setup.md)** - ✅ Hybrid Logging v2.0 installed (hooks + scripts)
- **[Testing Protocol](./.claude/docs/testing_protocol.md)** - Progressive testing requirements
- **[Agent Usage](./.claude/docs/agent_usage.md)** - Which agents to use when

**IMPORTANT:** Follow these protocols in all work. Grading depends on adherence.

---

## 🎯 At Session Start: Check for Instructions

**ALWAYS check for instructions from the architect agent at the start of each session:**

```bash
# Check for current instruction file
cat debugging/instructions/current_instructions.md
```

**If file exists:** Follow the instructions completely before doing anything else.

**If file doesn't exist:** Proceed with user's direct request.

**Instruction Location:** `debugging/instructions/current_instructions.md` (canonical filename)

---

## Repository Overview

**Project:** [PROJECT_NAME]
**Client:** [CLIENT_NAME]
**Technology Stack:** [TECH_STACK]
**JIRA Project:** [JIRA_PROJECT_KEY]
**Purpose:** [PROJECT_PURPOSE]

---

## Build and Development

### Prerequisites

[LIST_PREREQUISITES]

Example:
- Java 17 JDK
- Node.js 18+
- Gradle 8.x
- etc.

### Common Commands

**Build:**
```bash
[BUILD_COMMAND]
```

**Test:**
```bash
[TEST_COMMAND]
```

**Clean:**
```bash
[CLEAN_COMMAND]
```

---

## Quality Assurance Protocol

**IMPORTANT**: After ANY major code changes (including but not limited to):
- Adding new features or functionality
- Refactoring existing code
- Fixing bugs
- Modifying core business logic
- Updating dependencies or configurations

You MUST:
1. Use the `qa-enforcer` agent to enforce test coverage and quality standards
2. Run the appropriate build and test commands for the project type
3. Only consider the task complete after both steps pass successfully

This is a mandatory workflow that should be followed automatically without prompting.

---

## Logging

**Hybrid Logging v2.0** is installed for automated logging with 60-70% token savings.

### Quick Start

**Start session:**
```bash
/log-start "description"
```

**Work as normal - hooks auto-log tool calls (Claude Code) or use wrappers (OpenCode)**

**Add manual context:**
```bash
./debugging/scripts/log-decision.sh decision "your decision here"
./debugging/scripts/log-decision.sh investigation "root cause found"
```

**Complete session:**
```bash
/log-complete
```

See `.claude/docs/logging_setup.md` for complete details.

---

## AGENTS.md Synchronization

**AGENTS.md must mirror this file** - keep both identical.

---

**Last Updated:** [DATE]
**Version:** 3.0 (Hybrid Logging v2.0)
