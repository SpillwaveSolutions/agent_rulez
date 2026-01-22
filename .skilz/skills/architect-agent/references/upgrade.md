# Upgrade Guide - Architect Agent Skill v3.0

**From:** v1.0/v2.0 (Manual/Hybrid Logging)
**To:** v3.0 (Workspace Setup with Hooks Fix)
**Last Updated:** 2025-11-20

---

## What's Changed in v3.0

⚠️ **CRITICAL FIX:** Hooks MUST be in `.claude/settings.json` (NOT hooks.json!)

**New Features:**
- ✅ Automated workspace setup scripts
- ✅ Verification scripts
- ✅ Enhanced hook logger with full argument capture
- ✅ Complete workspace templates
- ✅ Improved documentation

---

## Migration Path

### Option 1: Fresh Start (Recommended)

Create new workspaces using templates and migrate content:

```bash
# 1. Create new code agent workspace
cd ~/.claude/skills/architect-agent/templates/
./setup-workspace.sh code-agent ~/projects/my-project-v3

# 2. Migrate content from old workspace
OLD_WORKSPACE=~/projects/my-project-old
NEW_WORKSPACE=~/projects/my-project-v3

# Copy source code and project files
cp -r $OLD_WORKSPACE/src $NEW_WORKSPACE/
cp -r $OLD_WORKSPACE/tests $NEW_WORKSPACE/
# ... copy other project files ...

# 3. Migrate logs (optional)
cp -r $OLD_WORKSPACE/debugging/logs/* $NEW_WORKSPACE/debugging/logs/

# 4. Verify new workspace
cd $NEW_WORKSPACE
../verify-workspace.sh
```

### Option 2: In-Place Upgrade

Upgrade existing workspace to v3.0 structure:

**⚠️ BACKUP FIRST:**
```bash
cp -r ~/projects/my-project ~/projects/my-project-backup
```

#### Code Agent Workspace Upgrade

```bash
cd ~/projects/my-project

# 1. CRITICAL FIX: Move hooks to settings.json
if [ -f .claude/hooks.json ]; then
    echo "Moving hooks from hooks.json to settings.json..."

    # Backup hooks.json
    mv .claude/hooks.json .claude/hooks.json.backup

    # Create settings.json with hooks
    cat > .claude/settings.json <<'EOF'
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
EOF

    echo "✅ Hooks moved to settings.json"
fi

# 2. Update hook-logger.py with enhanced version
cp ~/.claude/skills/architect-agent/templates/code-agent-workspace/.claude/hook-logger.py \
   .claude/hook-logger.py
chmod +x .claude/hook-logger.py

# 3. Install OpenCode wrapper support (if needed)
cp -r ~/.claude/skills/architect-agent/templates/code-agent-workspace/debugging/wrapper-scripts \
      debugging/
chmod +x debugging/wrapper-scripts/*.sh

cp -r ~/.claude/skills/architect-agent/templates/code-agent-workspace/.opencode \
      ./

# 4. Update documentation
cp -r ~/.claude/skills/architect-agent/templates/code-agent-workspace/.claude/docs \
      .claude/

# 5. Update slash commands
cp -r ~/.claude/skills/architect-agent/templates/code-agent-workspace/.claude/commands/*.md \
      .claude/commands/

# 6. Update CLAUDE.md sections (manual merge required)
echo "⚠️  Review and merge CLAUDE.md updates manually"
echo "    Template: ~/.claude/skills/architect-agent/templates/code-agent-workspace/CLAUDE.md"

# 7. Update AGENTS.md
cp CLAUDE.md AGENTS.md

# 8. Verify upgrade
~/.claude/skills/architect-agent/templates/verify-workspace.sh
```

#### Architect Workspace Upgrade

```bash
cd ~/projects/my-architect

# 1. Update documentation
cp -r ~/.claude/skills/architect-agent/templates/architect-workspace/docs/* docs/

# 2. Update slash commands
cp -r ~/.claude/skills/architect-agent/templates/architect-workspace/.claude/commands/*.md \
      .claude/commands/

# 3. Update CLAUDE.md sections (manual merge required)
echo "⚠️  Review and merge CLAUDE.md updates manually"
echo "    Template: ~/.claude/skills/architect-agent/templates/architect-workspace/CLAUDE.md"

# 4. Update AGENTS.md
cp CLAUDE.md AGENTS.md

# 5. Verify upgrade
~/.claude/skills/architect-agent/templates/verify-workspace.sh
```

---

## Critical Migration: hooks.json → settings.json

**⚠️ THIS IS THE MOST IMPORTANT CHANGE**

### Why This Matters

Claude Code ONLY reads hooks from `.claude/settings.json`. If your hooks are in `.claude/hooks.json`, they will NEVER execute.

### Migration Steps

```bash
cd ~/projects/my-code-agent

# 1. Check current configuration
if [ -f .claude/hooks.json ]; then
    echo "❌ Hooks in WRONG file: hooks.json"
    echo "   Must move to: settings.json"
fi

# 2. Archive old hooks.json
if [ -f .claude/hooks.json ]; then
    mv .claude/hooks.json .claude/hooks.json.backup
    echo "✅ Archived hooks.json"
fi

# 3. Create settings.json with hooks
cat > .claude/settings.json <<'EOF'
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
EOF

echo "✅ Created settings.json with hooks"

# 4. Verify
grep "PostToolUse" .claude/settings.json && echo "✅ Hooks configured in settings.json"
```

### Test Hooks After Migration

```bash
# Start session
./debugging/scripts/log-start.sh "migration-test"

# Run commands
ls -la
pwd
date

# Check for TOOL entries
grep "TOOL:" $(cat debugging/current_log_file.txt)

# Should see 3+ TOOL entries
# If not, hooks still not working - check troubleshooting

# Complete session
./debugging/scripts/log-complete.sh
```

---

## Enhanced Hook Logger Migration

The v3.0 hook logger captures more detailed arguments:

**What's New:**
- **Bash:** Command (200 chars) + description
- **Read:** File path + offset/limit
- **Write:** File path + content size
- **Edit:** File path + old/new strings + replace_all flag
- **Grep:** Pattern + path + glob + mode + case_insensitive
- **Glob:** Pattern + path
- **TodoWrite:** Count + status breakdown

**Upgrade:**
```bash
cp ~/.claude/skills/architect-agent/templates/code-agent-workspace/.claude/hook-logger.py \
   .claude/hook-logger.py
chmod +x .claude/hook-logger.py
```

---

## Verification After Upgrade

### Run Verification Script

```bash
cd ~/projects/my-workspace
~/.claude/skills/architect-agent/templates/verify-workspace.sh
```

### Critical Checks

```bash
# 1. Hooks in settings.json
grep "PostToolUse" .claude/settings.json || echo "❌ FAIL: No hooks in settings.json"

# 2. No hooks.json exists
ls .claude/hooks.json 2>&1 | grep "No such file" || echo "❌ FAIL: hooks.json still exists"

# 3. Hook logger executable
[ -x .claude/hook-logger.py ] && echo "✅ Hook logger executable" || echo "❌ FAIL: Not executable"

# 4. Valid JSON
python3 -m json.tool .claude/settings.json > /dev/null && echo "✅ Valid JSON" || echo "❌ FAIL: Invalid JSON"
```

---

## Rollback (If Needed)

If upgrade causes issues, rollback to backup:

```bash
# Restore from backup
rm -rf ~/projects/my-project
cp -r ~/projects/my-project-backup ~/projects/my-project

# Or restore just hooks configuration
cp ~/projects/my-project-backup/.claude/hooks.json ~/projects/my-project/.claude/
```

**Note:** v1.0/v2.0 hooks in hooks.json won't work. Only v3.0+ hooks in settings.json will work.

---

## Benefits After Upgrade

✅ **Hooks Actually Work** - Settings.json configuration is read by Claude Code
✅ **60-70% Token Savings** - Automated logging reduces conversation tokens
✅ **Better Audit Trail** - Enhanced argument capture provides more context
✅ **Dual-Mode Support** - Works in both Claude Code and OpenCode
✅ **Verification Tools** - Scripts to validate installation

---

## Troubleshooting

### Hooks Still Not Working

**Check everything:**
```bash
# 1. Hooks in settings.json
grep "PostToolUse" .claude/settings.json

# 2. No hooks.json
ls .claude/hooks.json 2>&1 | grep "No such file"

# 3. Hook logger path correct
grep "hook-logger.py" .claude/settings.json

# 4. Hook logger executable
ls -la .claude/hook-logger.py

# 5. Active log session
cat debugging/current_log_file.txt

# 6. Valid JSON
python3 -m json.tool .claude/settings.json
```

**See:** [hook_configuration_critical.md](./references/hook_configuration_critical.md)

### Merge Conflicts

If you have custom changes in CLAUDE.md:

```bash
# Compare template vs your version
diff -u ~/.claude/skills/architect-agent/templates/code-agent-workspace/CLAUDE.md \
         ~/projects/my-project/CLAUDE.md

# Manually merge changes
vim ~/projects/my-project/CLAUDE.md
```

---

## Support

**Documentation:**
- [Installation Guide](./INSTALLATION.md) - Fresh installation
- [Quick Start Guide](./references/quick_start.md) - Getting started
- [Hook Configuration](./references/hook_configuration_critical.md) - Critical fix details

**Get Help:**
- Review verification output for specific issues
- Check troubleshooting sections above
- Consult reference documentation

---

**Upgrade Complete!** 🎉

Your workspace now has:
- ✅ Working hooks (settings.json fix)
- ✅ Enhanced argument capture
- ✅ Dual-mode logging support
- ✅ Complete documentation
- ✅ Verification tools

---

**Last Updated:** 2025-11-20
**Version:** 3.0 (Hooks Fix + Automation)
