# Quick Start Guide - Architect Agent Skill

**Version:** 3.0 (Workspace Setup with Hooks Fix)
**Last Updated:** 2025-11-20

---

## 5-Minute Setup: Create New Workspace

### Prerequisites

- Git installed
- Claude Code CLI installed (`claude`)
- Python 3 (for hook logger)

### Option 1: Code Agent Workspace Only

```bash
# 1. Navigate to skill templates
cd ~/.claude/skills/architect-agent/templates/

# 2. Run setup script
./setup-workspace.sh code-agent ~/projects/my-code-agent

# 3. Customize CLAUDE.md
cd ~/projects/my-code-agent
vim CLAUDE.md
# Update: [PROJECT_NAME], [TECH_STACK], [BUILD_COMMAND], [TEST_COMMAND]

# 4. Verify installation
../verify-workspace.sh

# 5. Test hooks
./debugging/scripts/log-start.sh "test"
ls -la
grep "TOOL:" $(cat debugging/current_log_file.txt)
./debugging/scripts/log-complete.sh
```

**Done!** Hooks are working if you see TOOL entries.

---

### Option 2: Architect + Code Agent Workspaces

```bash
# 1. Navigate to skill templates
cd ~/.claude/skills/architect-agent/templates/

# 2. Create code agent workspace first
./setup-workspace.sh code-agent ~/projects/my-code-agent

# 3. Create architect workspace
./setup-workspace.sh architect ~/projects/my-architect \
    --code-agent-path ~/projects/my-code-agent

# 4. Verify both
cd ~/projects/my-code-agent && ../verify-workspace.sh
cd ~/projects/my-architect && ../verify-workspace.sh
```

**Done!** Both workspaces are ready.

---

## 10-Minute Setup: Full Configuration

### 1. Create Workspaces (5 minutes)

Follow "Option 2" above to create both workspaces.

### 2. Configure Project Details (3 minutes)

**Code Agent CLAUDE.md:**
```markdown
**Project:** YourProject
**Technology Stack:** Java 17, Gradle 8.x, Spring Boot 3.x
**Build:** ./gradlew clean build
**Test:** ./gradlew test
```

**Architect docs/technology_adaptations.md:**
- Update build commands
- Update test commands
- Update JIRA project key
- Update GitHub repository

### 3. Test End-to-End (2 minutes)

**Create test instruction:**
```bash
cd ~/projects/my-architect

cat > instructions/instruct-test.md <<'EOF'
# Test Instruction

Run: ls -la
Run: pwd
Run: echo "Test complete"

Report: Did commands execute?
EOF

# Create human summary
cat > human/human-test.md <<'EOF'
# Test Summary
- List directory contents
- Print working directory
- Echo test message
EOF

# Send to code agent
cp instructions/instruct-test.md \
   ~/projects/my-code-agent/debugging/instructions/current_instructions.md
```

**Execute in code agent:**
```bash
cd ~/projects/my-code-agent
claude -p "Read debugging/instructions/current_instructions.md and execute"
```

**Verify hooks captured:**
```bash
grep "TOOL:" debugging/logs/session_*.log
```

---

## Troubleshooting

### Hooks Not Working

**Check:**
```bash
# 1. Hooks in settings.json (NOT hooks.json)
grep "PostToolUse" .claude/settings.json

# 2. No hooks.json exists
ls .claude/hooks.json 2>&1 | grep "No such file"

# 3. Hook logger executable
ls -la .claude/hook-logger.py

# 4. Valid JSON
python3 -m json.tool .claude/settings.json
```

**Fix:**
```bash
# If hooks in wrong file
mv .claude/hooks.json .claude/hooks.json.backup

# Hooks MUST be in settings.json
grep "PostToolUse" .claude/settings.json || echo "Add hooks to settings.json!"
```

**See:** [hook_configuration_critical.md](./hook_configuration_critical.md)

---

### Permission Denied

**Architect can't write to code agent instructions:**

```bash
# Check architect's settings.json
cat ~/projects/my-architect/.claude/settings.json

# Should include:
# "allowedDirectories": [
#   "/full/path/to/architect",
#   "/full/path/to/code-agent/debugging/instructions"
# ]
```

**Fix:**
```bash
# Update architect's settings.json with absolute paths
vim ~/projects/my-architect/.claude/settings.json
```

---

### Scripts Not Executable

```bash
# Code agent workspace
cd ~/projects/my-code-agent
chmod +x .claude/hook-logger.py
chmod +x debugging/scripts/*.sh
chmod +x debugging/wrapper-scripts/*.sh
```

---

## Next Steps

### For Code Agent Users

1. **Customize CLAUDE.md** with project specifics
2. **Test hooks** - Verify TOOL entries appear in logs
3. **Configure GitHub** - Run `gh auth status`
4. **Start using** - Begin normal development with automatic logging

### For Architect Users

1. **Create first ticket** in `ticket/current_ticket.md`
2. **Write instructions** for code agent
3. **Send instructions** using `/project.send`
4. **Monitor execution** via code agent logs
5. **Grade work** after completion

---

## Common Commands

### Code Agent

```bash
# Start logging
/log-start "description"

# Work normally - hooks auto-log

# Add manual context
./debugging/scripts/log-decision.sh decision "why I chose X"

# Complete session
/log-complete
```

### Architect

```bash
# Check current ticket
cat ticket/current_ticket.md

# Create instruction
vim instructions/instruct-$(date +%Y%m%d_%H%M%S)-description.md

# Create human summary
vim human/human-$(date +%Y%m%d_%H%M%S)-description.md

# Send to code agent
/project.send
```

---

## Key Files

### Code Agent

- `.claude/settings.json` - **Hook configuration (CRITICAL!)**
- `.claude/hook-logger.py` - Hook logger script
- `debugging/logs/` - Session log files
- `debugging/instructions/current_instructions.md` - Current instructions

### Architect

- `instructions/` - Instruction files
- `human/` - Human summaries (10-25 bullets)
- `grades/` - Grading reports
- `ticket/current_ticket.md` - Current work
- `.claude/settings.json` - Permissions

---

## References

**Complete Guides:**
- [workspace_setup_complete.md](./workspace_setup_complete.md) - Full setup checklist
- [hook_configuration_critical.md](./hook_configuration_critical.md) - THE critical fix
- [hybrid_logging_protocol.md](./hybrid_logging_protocol.md) - Complete protocol

**Installation:**
- `templates/setup-workspace.sh` - Automated setup script
- `templates/verify-workspace.sh` - Verification script

**Templates:**
- `templates/architect-workspace/` - All architect files
- `templates/code-agent-workspace/` - All code agent files

---

**Last Updated:** 2025-11-20
**Version:** 3.0 (Hooks Fix + Automation)
