# Hybrid Logging v2.0 Overview

**Version:** 2.0
**Status:** ✅ Production Ready
**Token Savings:** 60-70% (~2,200 tokens per 30-command session)

---

## What is Hybrid Logging?

**Dual-mode automated logging system** that works in both Claude Code and OpenCode environments:

- **Claude Code:** PostToolUse hooks automatically capture tool calls
- **OpenCode:** Shell wrapper scripts intercept and log commands

**Both achieve the same result:** Automatic logging with 60-70% token savings.

---

## How It Works

### Claude Code (Hooks)
1. Hook configuration in `.claude/settings.json` (NOT hooks.json!)
2. PostToolUse event fires after each tool execution
3. Hook logger script (`.claude/hook-logger.py`) captures tool data
4. TOOL entries written to active log file

### OpenCode (Wrappers)
1. Shell functions exported in `.opencode/shell-init.sh`
2. Functions intercept commands (gradle, task, git, etc.)
3. Wrapper script logs command before + after execution
4. TOOL entries written to active log file

---

## For Architect Agents

### Your Responsibility
- **Instruct code agent to use logging system**
- Include logging commands in instructions
- Verify hooks/wrappers working after delegation
- Grade based on logging quality (10 points)

### Instructions Should Include
```markdown
## Logging

1. Start session: `/log-start "description"`
2. Use hooks (Claude Code) or wrappers (OpenCode) for auto-logging
3. Add manual context with ./debugging/scripts/log-decision.sh
4. Complete session: `/log-complete`
```

### Grading Logging (10 points)
- **10:** Perfect - hooks capturing all commands, manual context logged
- **8:** Good - hooks working, most manual context present
- **6:** Adequate - hooks operational, some manual entries missing
- **4:** Poor - hooks not working OR many manual entries missing
- **0-3:** Unacceptable - no automated logging or batch logs at end

---

## Token Efficiency

**Without Hybrid Logging (Manual):**
```
I'm going to run this command: ls -la
<output>
The output shows...
I'll now run: pwd
<output>
This tells me...
```
**Cost:** ~100 tokens per tool call

**With Hybrid Logging (Automated):**
```
<just the analysis and decisions>
```
**Cost:** ~25 tokens per tool call
**Savings:** 75% per tool call

---

## References

**Code Agent Setup:**
- Code agent workspace: `.claude/docs/logging_setup.md`
- Hook configuration: `.claude/settings.json`
- Hook logger: `.claude/hook-logger.py`

**Architect Agent References:**
- `~/.claude/skills/architect-agent/references/hybrid_logging_protocol.md`
- `~/.claude/skills/architect-agent/references/hook_configuration_critical.md`
- `~/.claude/skills/architect-agent/references/opencode_wrapper_setup.md`

---

**Last Updated:** [DATE]
