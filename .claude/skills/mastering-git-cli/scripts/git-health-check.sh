#!/bin/bash
#
# git-health-check.sh
# Diagnose common repository issues and report status
#
# Usage:
#   ./git-health-check.sh [options]
#
# Options:
#   --verbose    Show detailed output
#   --fix        Attempt to fix simple issues (prune, gc)
#   --json       Output as JSON
#   --help       Show this help message
#
# Checks:
#   - Repository validity
#   - HEAD state (detached or not)
#   - Uncommitted changes
#   - Stashed changes
#   - Remote sync status (behind/ahead)
#   - Untracked files
#   - Submodule status
#   - Large files warning
#   - Worktree status
#

set -e

# Configuration
VERBOSE=false
FIX=false
JSON_OUTPUT=false

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Counters
ISSUES=0
WARNINGS=0

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose)
            VERBOSE=true
            shift
            ;;
        --fix)
            FIX=true
            shift
            ;;
        --json)
            JSON_OUTPUT=true
            shift
            ;;
        --help)
            head -28 "$0" | tail -24
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Helper functions
check_pass() {
    if ! $JSON_OUTPUT; then
        echo -e "${GREEN}✓${NC} $1"
    fi
}

check_warn() {
    ((WARNINGS++))
    if ! $JSON_OUTPUT; then
        echo -e "${YELLOW}⚠${NC} $1"
    fi
}

check_fail() {
    ((ISSUES++))
    if ! $JSON_OUTPUT; then
        echo -e "${RED}✗${NC} $1"
    fi
}

check_info() {
    if $VERBOSE && ! $JSON_OUTPUT; then
        echo -e "${BLUE}ℹ${NC} $1"
    fi
}

# Initialize JSON output
if $JSON_OUTPUT; then
    JSON_CHECKS="["
    add_json_check() {
        local name=$1
        local status=$2
        local message=$3
        local details=$4
        
        if [ "$JSON_CHECKS" != "[" ]; then
            JSON_CHECKS="$JSON_CHECKS,"
        fi
        JSON_CHECKS="$JSON_CHECKS{\"name\":\"$name\",\"status\":\"$status\",\"message\":\"$message\""
        if [ -n "$details" ]; then
            JSON_CHECKS="$JSON_CHECKS,\"details\":\"$details\""
        fi
        JSON_CHECKS="$JSON_CHECKS}"
    }
fi

if ! $JSON_OUTPUT; then
    echo ""
    echo -e "${BLUE}Git Repository Health Check${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
fi

# Check 1: Valid repository
if git rev-parse --git-dir > /dev/null 2>&1; then
    check_pass "Valid Git repository"
    $JSON_OUTPUT && add_json_check "valid_repo" "pass" "Valid Git repository"
else
    check_fail "Not a valid Git repository"
    $JSON_OUTPUT && add_json_check "valid_repo" "fail" "Not a valid Git repository"
    exit 1
fi

# Check 2: HEAD state
if git symbolic-ref HEAD > /dev/null 2>&1; then
    CURRENT_BRANCH=$(git symbolic-ref --short HEAD)
    check_pass "On branch: $CURRENT_BRANCH"
    $JSON_OUTPUT && add_json_check "head_state" "pass" "On branch: $CURRENT_BRANCH"
else
    CURRENT_SHA=$(git rev-parse --short HEAD)
    check_warn "DETACHED HEAD at $CURRENT_SHA"
    $JSON_OUTPUT && add_json_check "head_state" "warn" "Detached HEAD" "$CURRENT_SHA"
fi

# Check 3: Uncommitted changes
STAGED=$(git diff --cached --name-only 2>/dev/null | wc -l | tr -d ' ')
UNSTAGED=$(git diff --name-only 2>/dev/null | wc -l | tr -d ' ')

if [ "$STAGED" -eq 0 ] && [ "$UNSTAGED" -eq 0 ]; then
    check_pass "No uncommitted changes"
    $JSON_OUTPUT && add_json_check "uncommitted" "pass" "No uncommitted changes"
else
    MSG="Uncommitted changes: $STAGED staged, $UNSTAGED unstaged"
    check_warn "$MSG"
    $JSON_OUTPUT && add_json_check "uncommitted" "warn" "$MSG"
    if $VERBOSE; then
        git status -s | head -10
        TOTAL=$((STAGED + UNSTAGED))
        if [ $TOTAL -gt 10 ]; then
            echo "  ... and $((TOTAL - 10)) more"
        fi
    fi
fi

# Check 4: Stashed changes
STASH_COUNT=$(git stash list 2>/dev/null | wc -l | tr -d ' ')
if [ "$STASH_COUNT" -eq 0 ]; then
    check_pass "No stashed changes"
    $JSON_OUTPUT && add_json_check "stash" "pass" "No stashed changes"
else
    check_info "$STASH_COUNT stash(es) saved"
    $JSON_OUTPUT && add_json_check "stash" "info" "$STASH_COUNT stash(es)"
fi

# Check 5: Remote sync status
if git remote | grep -q .; then
    REMOTE=$(git remote | head -1)
    
    # Fetch latest (if fix mode)
    if $FIX; then
        check_info "Fetching from $REMOTE..."
        git fetch "$REMOTE" --quiet 2>/dev/null || true
    fi
    
    if git symbolic-ref HEAD > /dev/null 2>&1; then
        UPSTREAM=$(git rev-parse --abbrev-ref --symbolic-full-name @{u} 2>/dev/null || echo "")
        
        if [ -n "$UPSTREAM" ]; then
            AHEAD=$(git rev-list --count "$UPSTREAM..HEAD" 2>/dev/null || echo "0")
            BEHIND=$(git rev-list --count "HEAD..$UPSTREAM" 2>/dev/null || echo "0")
            
            if [ "$AHEAD" -eq 0 ] && [ "$BEHIND" -eq 0 ]; then
                check_pass "In sync with $UPSTREAM"
                $JSON_OUTPUT && add_json_check "remote_sync" "pass" "In sync with $UPSTREAM"
            elif [ "$BEHIND" -gt 0 ]; then
                check_warn "Behind $UPSTREAM by $BEHIND commit(s)"
                $JSON_OUTPUT && add_json_check "remote_sync" "warn" "Behind by $BEHIND"
            else
                check_info "Ahead of $UPSTREAM by $AHEAD commit(s)"
                $JSON_OUTPUT && add_json_check "remote_sync" "info" "Ahead by $AHEAD"
            fi
        else
            check_warn "No upstream branch configured"
            $JSON_OUTPUT && add_json_check "remote_sync" "warn" "No upstream configured"
        fi
    fi
else
    check_info "No remotes configured"
    $JSON_OUTPUT && add_json_check "remote_sync" "info" "No remotes"
fi

# Check 6: Untracked files
UNTRACKED=$(git ls-files --others --exclude-standard 2>/dev/null | wc -l | tr -d ' ')
if [ "$UNTRACKED" -eq 0 ]; then
    check_pass "No untracked files"
    $JSON_OUTPUT && add_json_check "untracked" "pass" "No untracked files"
elif [ "$UNTRACKED" -lt 10 ]; then
    check_info "$UNTRACKED untracked file(s)"
    $JSON_OUTPUT && add_json_check "untracked" "info" "$UNTRACKED untracked files"
else
    check_warn "$UNTRACKED untracked files (consider .gitignore)"
    $JSON_OUTPUT && add_json_check "untracked" "warn" "$UNTRACKED untracked files"
fi

# Check 7: Submodules
if [ -f ".gitmodules" ]; then
    SUBMODULE_STATUS=$(git submodule status 2>/dev/null || echo "")
    UNINIT=$(echo "$SUBMODULE_STATUS" | grep -c "^-" || echo "0")
    MODIFIED=$(echo "$SUBMODULE_STATUS" | grep -c "^+" || echo "0")
    
    if [ "$UNINIT" -gt 0 ]; then
        check_warn "$UNINIT submodule(s) not initialized"
        $JSON_OUTPUT && add_json_check "submodules" "warn" "$UNINIT not initialized"
    elif [ "$MODIFIED" -gt 0 ]; then
        check_info "$MODIFIED submodule(s) at different commit"
        $JSON_OUTPUT && add_json_check "submodules" "info" "$MODIFIED at different commit"
    else
        check_pass "All submodules in sync"
        $JSON_OUTPUT && add_json_check "submodules" "pass" "All in sync"
    fi
else
    check_info "No submodules"
    $JSON_OUTPUT && add_json_check "submodules" "info" "No submodules"
fi

# Check 8: Worktrees
WORKTREE_COUNT=$(git worktree list 2>/dev/null | wc -l | tr -d ' ')
if [ "$WORKTREE_COUNT" -gt 1 ]; then
    check_info "$WORKTREE_COUNT worktree(s) active"
    $JSON_OUTPUT && add_json_check "worktrees" "info" "$WORKTREE_COUNT worktrees"
    if $VERBOSE; then
        git worktree list
    fi
fi

# Check 9: Large files (>10MB)
if $VERBOSE; then
    LARGE_FILES=$(find . -type f -size +10M -not -path "./.git/*" 2>/dev/null | head -5)
    if [ -n "$LARGE_FILES" ]; then
        check_warn "Large files detected (>10MB)"
        echo "$LARGE_FILES" | while read -r f; do
            SIZE=$(du -h "$f" | cut -f1)
            echo "    $SIZE  $f"
        done
        $JSON_OUTPUT && add_json_check "large_files" "warn" "Large files detected"
    fi
fi

# Fix mode actions
if $FIX && ! $JSON_OUTPUT; then
    echo ""
    echo -e "${BLUE}Running fixes...${NC}"
    
    # Prune remote tracking branches
    git remote prune origin 2>/dev/null && echo "  Pruned stale remote branches" || true
    
    # Prune worktrees
    git worktree prune 2>/dev/null && echo "  Pruned stale worktrees" || true
    
    # Run garbage collection
    git gc --auto --quiet 2>/dev/null && echo "  Garbage collection complete" || true
fi

# Summary
if $JSON_OUTPUT; then
    echo "{"
    echo "  \"issues\": $ISSUES,"
    echo "  \"warnings\": $WARNINGS,"
    echo "  \"checks\": $JSON_CHECKS]"
    echo "}"
else
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    if [ $ISSUES -eq 0 ] && [ $WARNINGS -eq 0 ]; then
        echo -e "${GREEN}Repository is healthy!${NC}"
    elif [ $ISSUES -eq 0 ]; then
        echo -e "${YELLOW}$WARNINGS warning(s), no critical issues${NC}"
    else
        echo -e "${RED}$ISSUES issue(s), $WARNINGS warning(s)${NC}"
    fi
    echo ""
fi

exit $ISSUES
