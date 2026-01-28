#!/bin/bash
# verify-workspace.sh - Verify architect/code agent workspace setup
# Version: 3.0 (Hybrid Logging v2.0)

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

ERRORS=0
WARNINGS=0

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Workspace Verification Script v3.0${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Detect workspace type
if [ -f "CLAUDE.md" ]; then
    if grep -q "Architect Agent" CLAUDE.md; then
        WORKSPACE_TYPE="architect"
    elif grep -q "Code Agent" CLAUDE.md; then
        WORKSPACE_TYPE="code-agent"
    else
        echo -e "${RED}❌ Cannot determine workspace type from CLAUDE.md${NC}"
        exit 1
    fi
else
    echo -e "${RED}❌ CLAUDE.md not found - not a valid workspace${NC}"
    exit 1
fi

echo -e "${YELLOW}Workspace Type:${NC} $WORKSPACE_TYPE"
echo -e "${YELLOW}Workspace Path:${NC} $(pwd)"
echo ""

check_file() {
    local file="$1"
    local description="$2"

    if [ -f "$file" ]; then
        echo -e "  ✅ $description"
    else
        echo -e "  ${RED}❌ Missing: $description${NC}"
        ((ERRORS++))
    fi
}

check_dir() {
    local dir="$1"
    local description="$2"

    if [ -d "$dir" ]; then
        echo -e "  ✅ $description"
    else
        echo -e "  ${RED}❌ Missing: $description${NC}"
        ((ERRORS++))
    fi
}

check_executable() {
    local file="$1"
    local description="$2"

    if [ -f "$file" ]; then
        if [ -x "$file" ]; then
            echo -e "  ✅ $description (executable)"
        else
            echo -e "  ${YELLOW}⚠️  $description (not executable)${NC}"
            ((WARNINGS++))
        fi
    else
        echo -e "  ${RED}❌ Missing: $description${NC}"
        ((ERRORS++))
    fi
}

# Common checks for both types
echo -e "${GREEN}📋 Core Configuration:${NC}"
check_file "CLAUDE.md" "CLAUDE.md"
check_file "AGENTS.md" "AGENTS.md"
check_file ".claude/settings.json" ".claude/settings.json"
echo ""

# Verify CLAUDE.md and AGENTS.md are identical
echo -e "${GREEN}🔍 Configuration Sync:${NC}"
if [ -f "CLAUDE.md" ] && [ -f "AGENTS.md" ]; then
    if diff -q CLAUDE.md AGENTS.md > /dev/null; then
        echo -e "  ✅ CLAUDE.md and AGENTS.md are identical"
    else
        echo -e "  ${YELLOW}⚠️  CLAUDE.md and AGENTS.md differ${NC}"
        ((WARNINGS++))
    fi
fi
echo ""

# Verify settings.json is valid JSON
echo -e "${GREEN}🔍 JSON Validation:${NC}"
if [ -f ".claude/settings.json" ]; then
    if python3 -m json.tool .claude/settings.json > /dev/null 2>&1; then
        echo -e "  ✅ .claude/settings.json is valid JSON"
    else
        echo -e "  ${RED}❌ .claude/settings.json has invalid JSON${NC}"
        ((ERRORS++))
    fi
fi
echo ""

# Type-specific checks
if [ "$WORKSPACE_TYPE" = "architect" ]; then
    echo -e "${GREEN}📋 Architect Workspace Structure:${NC}"
    check_dir "instructions" "instructions/ directory"
    check_dir "human" "human/ directory"
    check_dir "grades" "grades/ directory"
    check_dir "ticket" "ticket/ directory"
    check_dir "analysis" "analysis/ directory"
    check_dir "docs" "docs/ directory"
    check_dir ".claude/commands" ".claude/commands/ directory"
    echo ""

    echo -e "${GREEN}📋 Architect Documentation:${NC}"
    check_file "docs/hybrid_logging.md" "docs/hybrid_logging.md"
    check_file "docs/workflow.md" "docs/workflow.md"
    check_file "docs/technology_adaptations.md" "docs/technology_adaptations.md"
    check_file "docs/critical_protocols.md" "docs/critical_protocols.md"
    echo ""

    echo -e "${GREEN}📋 Slash Commands:${NC}"
    check_file ".claude/commands/project.instruct.md" "/project.instruct"
    check_file ".claude/commands/project.send.md" "/project.send"
    echo ""

    echo -e "${GREEN}🔍 Permissions:${NC}"
    if [ -f ".claude/settings.json" ]; then
        if grep -q "allowedDirectories" .claude/settings.json; then
            echo -e "  ✅ allowedDirectories configured"

            # Check if code agent path is configured
            if grep -q "debugging/instructions" .claude/settings.json; then
                echo -e "  ✅ Code agent instructions path configured"
            else
                echo -e "  ${YELLOW}⚠️  Code agent instructions path not configured${NC}"
                ((WARNINGS++))
            fi
        else
            echo -e "  ${RED}❌ No allowedDirectories in settings.json${NC}"
            ((ERRORS++))
        fi
    fi
    echo ""

elif [ "$WORKSPACE_TYPE" = "code-agent" ]; then
    echo -e "${GREEN}📋 Code Agent Workspace Structure:${NC}"
    check_dir ".claude/commands" ".claude/commands/ directory"
    check_dir ".claude/docs" ".claude/docs/ directory"
    check_dir "debugging" "debugging/ directory"
    check_dir "debugging/logs" "debugging/logs/ directory"
    check_dir "debugging/scripts" "debugging/scripts/ directory"
    check_dir "debugging/wrapper-scripts" "debugging/wrapper-scripts/ directory"
    check_dir "debugging/instructions" "debugging/instructions/ directory"
    check_dir ".opencode" ".opencode/ directory"
    echo ""

    echo -e "${GREEN}📋 Hook System:${NC}"
    check_executable ".claude/hook-logger.py" ".claude/hook-logger.py"

    if [ -f ".claude/hooks.json" ]; then
        echo -e "  ${RED}❌ .claude/hooks.json exists (should be in settings.json!)${NC}"
        ((ERRORS++))
    else
        echo -e "  ✅ No .claude/hooks.json (correct)"
    fi

    if [ -f ".claude/settings.json" ]; then
        if grep -q "PostToolUse" .claude/settings.json; then
            echo -e "  ✅ PostToolUse hooks configured in settings.json"

            if grep -q "hook-logger.py" .claude/settings.json; then
                echo -e "  ✅ hook-logger.py referenced in hooks"
            else
                echo -e "  ${RED}❌ hook-logger.py not referenced in hooks${NC}"
                ((ERRORS++))
            fi
        else
            echo -e "  ${RED}❌ No PostToolUse hooks in settings.json${NC}"
            ((ERRORS++))
        fi
    fi
    echo ""

    echo -e "${GREEN}📋 Logging Scripts:${NC}"
    check_executable "debugging/scripts/log-start.sh" "log-start.sh"
    check_executable "debugging/scripts/log-complete.sh" "log-complete.sh"
    check_executable "debugging/scripts/log-decision.sh" "log-decision.sh"
    echo ""

    echo -e "${GREEN}📋 Wrapper Scripts:${NC}"
    check_executable "debugging/wrapper-scripts/run-with-logging.sh" "run-with-logging.sh"
    check_executable "debugging/wrapper-scripts/log-tool-call.sh" "log-tool-call.sh"
    check_executable "debugging/wrapper-scripts/log-tool-result.sh" "log-tool-result.sh"
    echo ""

    echo -e "${GREEN}📋 OpenCode Support:${NC}"
    check_file ".opencode/shell-init.sh" ".opencode/shell-init.sh"

    if [ -f ".opencode/shell-init.sh" ]; then
        if grep -q "export -f" .opencode/shell-init.sh; then
            echo -e "  ✅ Shell functions exported"
        else
            echo -e "  ${YELLOW}⚠️  No exported functions in shell-init.sh${NC}"
            ((WARNINGS++))
        fi
    fi
    echo ""

    echo -e "${GREEN}📋 Documentation:${NC}"
    check_file ".claude/docs/logging_setup.md" "logging_setup.md"
    check_file ".claude/docs/testing_protocol.md" "testing_protocol.md"
    check_file ".claude/docs/agent_usage.md" "agent_usage.md"
    echo ""

    echo -e "${GREEN}📋 Slash Commands:${NC}"
    check_file ".claude/commands/log-start.md" "/log-start"
    check_file ".claude/commands/log-checkpoint.md" "/log-checkpoint"
    check_file ".claude/commands/log-complete.md" "/log-complete"
    check_file ".claude/commands/instruct.md" "/instruct"
    echo ""

    # Check for recursion prevention in wrapper
    echo -e "${GREEN}🔍 Critical Checks:${NC}"
    if [ -f "debugging/wrapper-scripts/run-with-logging.sh" ]; then
        if grep -q 'command \$COMMAND' debugging/wrapper-scripts/run-with-logging.sh; then
            echo -e "  ✅ Recursion prevention in place (command prefix)"
        else
            echo -e "  ${RED}❌ Missing recursion prevention in run-with-logging.sh${NC}"
            ((ERRORS++))
        fi
    fi
    echo ""
fi

# Summary
echo -e "${GREEN}========================================${NC}"
if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✅ Verification PASSED${NC}"
    echo -e "${GREEN}All checks passed successfully!${NC}"
    exit 0
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠️  Verification PASSED with warnings${NC}"
    echo -e "${YELLOW}Warnings: $WARNINGS${NC}"
    echo -e "${YELLOW}Review warnings above and fix if needed.${NC}"
    exit 0
else
    echo -e "${RED}❌ Verification FAILED${NC}"
    echo -e "${RED}Errors: $ERRORS${NC}"
    echo -e "${YELLOW}Warnings: $WARNINGS${NC}"
    echo ""
    echo "Please fix the errors above before using this workspace."
    exit 1
fi
