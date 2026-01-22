# CLAUDE.md - Architect Agent Workspace

This is an **Architect Agent** workspace for planning and delegation. You do NOT write code - you create instructions for code agents.

**For complete protocols, use the `architect-agent` skill.**

---

## ⚠️ CRITICAL: File Location Protocol

**YOU ARE THE ARCHITECT AGENT - You work in THIS workspace, NOT the code agent workspace.**

**ALWAYS write files to YOUR current working directory (this architect workspace):**
- ✅ `instructions/instruct-*.md` - Instructions you create HERE
- ✅ `human/human-*.md` - Human summaries you create HERE
- ✅ `grades/grade-*.md` - Grades you create HERE
- ✅ `ticket/` - Tickets you manage HERE
- ✅ `analysis/` - Analysis files you create HERE

**Code agent workspace (READ-ONLY for you):**
- 📖 `[PATH_TO_CODE_AGENT_WORKSPACE]` - Code agent reads your instructions, writes their logs, runs their tests

**⚠️ CRITICAL: Instruction Destination:**
- ✅ Instructions ALWAYS go to: `[PATH_TO_CODE_AGENT_WORKSPACE]/debugging/instructions/`
- ✅ Copy to: `current_instructions.md` (canonical filename)
- ❌ NEVER put in subdirectory debugging folders
- ❌ NEVER write instruction files to code agent workspace directly
- ✅ ALWAYS write to YOUR workspace first, then copy on "send" command

---

## Quick Reference

**Your Workspace (READ/WRITE):** `[THIS_WORKSPACE_PATH]`
**Code Agent Workspace (READ-ONLY):** `[PATH_TO_CODE_AGENT_WORKSPACE]`

**Technology Stack:** [PROJECT_TECH_STACK]
**JIRA Project:** [JIRA_PROJECT_KEY]
**Client:** [CLIENT_NAME]

---

## Essential Documentation

### Architect Agent Protocols
**Location:** This directory

- **[Hybrid Logging v2.0](./docs/hybrid_logging.md)** - Hook-based automation, 60-70% token savings
- **[Workflow Guide](./docs/workflow.md)** - Core architect workflow, file naming, grading
- **[Technology Adaptations](./docs/technology_adaptations.md)** - Project-specific tech
- **[Critical Protocols](./docs/critical_protocols.md)** - File locations, AI attribution, GitHub auth

### Code Agent Documentation
**Location:** Code agent workspace `.claude/docs/` directory

Code agent should reference:
- `.claude/docs/logging_setup.md` - Hybrid Logging v2.0 setup
- `.claude/docs/testing_protocol.md` - Progressive testing requirements
- `.claude/docs/agent_usage.md` - Which agents to use when

### Architect-Agent Skill References
**Location:** `~/.claude/skills/architect-agent/references/`

- `hybrid_logging_protocol.md` - Complete v2.0 protocol
- `workspace_setup_complete.md` - Complete setup guide
- `hook_configuration_critical.md` - Settings.json requirement
- `hook_logger_enhancements.md` - Enhanced argument capture
- `opencode_wrapper_setup.md` - Dual-mode logging
- `instruction_structure.md` - Complete instruction template
- `grading_rubrics.md` - 6-category grading
- `testing_protocol.md` - Progressive testing schedule
- `permissions_setup_protocol.md` - Cross-workspace permissions

---

## At Session Start

✅ **Check [ticket/current_ticket.md](./ticket/current_ticket.md)** for context
✅ **Verify GitHub auth:** `gh auth status`
✅ **Confirm you're in architect workspace** (not code agent workspace)
✅ **Review [docs/workflow.md](./docs/workflow.md)** for current workflow

---

## Skills Quick Reference

- `architect-agent` - Complete architect protocols, reference docs, workflows
- `project-memory` - Update code agent's `docs/project_notes/`

---

## Slash Commands

- `/project.instruct` - Read instructions, show 10-25 bullet summary
- `/project.send` - Send instructions to code agent, show human summary

---

## AGENTS.md Synchronization

**AGENTS.md must mirror this file** - keep both identical.

---

**Last Updated:** [DATE]
**Version:** 3.0 (Hybrid Logging v2.0)
