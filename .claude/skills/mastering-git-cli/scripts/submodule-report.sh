#!/bin/bash
#
# submodule-report.sh
# Generate human-readable status report for all submodules
#
# Usage:
#   ./submodule-report.sh [options]
#
# Options:
#   --verbose    Show full commit hashes and additional details
#   --json       Output as JSON (for programmatic use)
#   --help       Show this help message
#
# Output includes:
#   - Submodule name and path
#   - Current vs recorded commit
#   - Branch tracking status
#   - Uncommitted changes indicator
#   - Behind/ahead of remote
#

set -e

# Configuration
VERBOSE=false
JSON_OUTPUT=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose)
            VERBOSE=true
            shift
            ;;
        --json)
            JSON_OUTPUT=true
            shift
            ;;
        --help)
            head -22 "$0" | tail -18
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Validate we're in a git repository with submodules
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "Error: Not in a git repository"
    exit 1
fi

if [ ! -f ".gitmodules" ]; then
    echo "No submodules found in this repository"
    exit 0
fi

# JSON output start
if $JSON_OUTPUT; then
    echo "{"
    echo '  "submodules": ['
fi

# Get list of submodules
SUBMODULES=$(git config --file .gitmodules --get-regexp path | awk '{ print $2 }')
FIRST=true
COUNT=0
TOTAL=$(echo "$SUBMODULES" | wc -l)

for SUBMODULE_PATH in $SUBMODULES; do
    ((COUNT++))
    
    # Get submodule name
    SUBMODULE_NAME=$(git config --file .gitmodules --get-regexp "submodule\..*\.path" | grep "$SUBMODULE_PATH" | sed 's/.*submodule\.\(.*\)\.path.*/\1/')
    
    # Get URL
    SUBMODULE_URL=$(git config --file .gitmodules --get "submodule.$SUBMODULE_NAME.url" 2>/dev/null || echo "unknown")
    
    # Get tracked branch
    TRACKED_BRANCH=$(git config --file .gitmodules --get "submodule.$SUBMODULE_NAME.branch" 2>/dev/null || echo "none")
    
    # Get recorded commit (what parent expects)
    RECORDED_COMMIT=$(git ls-tree HEAD "$SUBMODULE_PATH" 2>/dev/null | awk '{ print $3 }')
    if [ -z "$RECORDED_COMMIT" ]; then
        RECORDED_COMMIT="not-initialized"
    fi
    
    # Check if submodule is initialized
    if [ ! -d "$SUBMODULE_PATH/.git" ] && [ ! -f "$SUBMODULE_PATH/.git" ]; then
        STATUS="not-initialized"
        CURRENT_COMMIT="N/A"
        CURRENT_BRANCH="N/A"
        HAS_CHANGES="N/A"
        AHEAD_BEHIND="N/A"
    else
        STATUS="initialized"
        
        # Get current commit in submodule
        CURRENT_COMMIT=$(git -C "$SUBMODULE_PATH" rev-parse HEAD 2>/dev/null || echo "error")
        
        # Check if on a branch or detached
        CURRENT_BRANCH=$(git -C "$SUBMODULE_PATH" symbolic-ref --short HEAD 2>/dev/null || echo "DETACHED")
        
        # Check for uncommitted changes
        if git -C "$SUBMODULE_PATH" diff --quiet 2>/dev/null && git -C "$SUBMODULE_PATH" diff --cached --quiet 2>/dev/null; then
            HAS_CHANGES="clean"
        else
            HAS_CHANGES="dirty"
        fi
        
        # Check ahead/behind
        if [ "$CURRENT_BRANCH" != "DETACHED" ]; then
            AHEAD_BEHIND=$(git -C "$SUBMODULE_PATH" rev-list --left-right --count "origin/$CURRENT_BRANCH...$CURRENT_BRANCH" 2>/dev/null || echo "? ?")
            BEHIND=$(echo "$AHEAD_BEHIND" | awk '{ print $1 }')
            AHEAD=$(echo "$AHEAD_BEHIND" | awk '{ print $2 }')
            AHEAD_BEHIND="ahead:$AHEAD behind:$BEHIND"
        else
            AHEAD_BEHIND="N/A (detached)"
        fi
        
        # Check if at recorded commit
        if [ "$CURRENT_COMMIT" = "$RECORDED_COMMIT" ]; then
            COMMIT_STATUS="at-recorded"
        else
            COMMIT_STATUS="different"
        fi
    fi
    
    # Output
    if $JSON_OUTPUT; then
        if ! $FIRST; then
            echo ","
        fi
        FIRST=false
        
        SHORT_RECORDED=$(echo "$RECORDED_COMMIT" | cut -c1-7)
        SHORT_CURRENT=$(echo "$CURRENT_COMMIT" | cut -c1-7)
        
        cat <<EOF
    {
      "name": "$SUBMODULE_NAME",
      "path": "$SUBMODULE_PATH",
      "url": "$SUBMODULE_URL",
      "tracked_branch": "$TRACKED_BRANCH",
      "status": "$STATUS",
      "recorded_commit": "$SHORT_RECORDED",
      "current_commit": "$SHORT_CURRENT",
      "current_branch": "$CURRENT_BRANCH",
      "has_changes": "$HAS_CHANGES",
      "commit_status": "${COMMIT_STATUS:-N/A}"
    }
EOF
    else
        # Human-readable output
        echo ""
        echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${BLUE}Submodule:${NC} $SUBMODULE_NAME ($COUNT/$TOTAL)"
        echo -e "${BLUE}Path:${NC}      $SUBMODULE_PATH"
        
        if $VERBOSE; then
            echo -e "${BLUE}URL:${NC}       $SUBMODULE_URL"
        fi
        
        # Status indicator
        if [ "$STATUS" = "not-initialized" ]; then
            echo -e "${BLUE}Status:${NC}    ${RED}NOT INITIALIZED${NC}"
            echo -e "           Run: git submodule update --init $SUBMODULE_PATH"
        else
            echo -e "${BLUE}Status:${NC}    ${GREEN}Initialized${NC}"
            
            # Branch info
            if [ "$CURRENT_BRANCH" = "DETACHED" ]; then
                echo -e "${BLUE}Branch:${NC}    ${YELLOW}DETACHED HEAD${NC}"
            else
                echo -e "${BLUE}Branch:${NC}    $CURRENT_BRANCH"
            fi
            
            if [ "$TRACKED_BRANCH" != "none" ]; then
                echo -e "${BLUE}Tracking:${NC}  $TRACKED_BRANCH"
            fi
            
            # Commit comparison
            SHORT_RECORDED=$(echo "$RECORDED_COMMIT" | cut -c1-7)
            SHORT_CURRENT=$(echo "$CURRENT_COMMIT" | cut -c1-7)
            
            if [ "$CURRENT_COMMIT" = "$RECORDED_COMMIT" ]; then
                echo -e "${BLUE}Commit:${NC}    ${GREEN}$SHORT_CURRENT (at recorded)${NC}"
            else
                echo -e "${BLUE}Commit:${NC}    ${YELLOW}$SHORT_CURRENT (recorded: $SHORT_RECORDED)${NC}"
            fi
            
            if $VERBOSE; then
                echo -e "${BLUE}Full SHA:${NC}  $CURRENT_COMMIT"
            fi
            
            # Changes
            if [ "$HAS_CHANGES" = "dirty" ]; then
                echo -e "${BLUE}Changes:${NC}   ${RED}Uncommitted changes${NC}"
            else
                echo -e "${BLUE}Changes:${NC}   ${GREEN}Clean${NC}"
            fi
            
            # Ahead/behind
            if [ "$CURRENT_BRANCH" != "DETACHED" ]; then
                echo -e "${BLUE}Sync:${NC}      $AHEAD_BEHIND"
            fi
        fi
    fi
done

# JSON output end
if $JSON_OUTPUT; then
    echo ""
    echo "  ]"
    echo "}"
else
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo -e "${BLUE}Summary:${NC} $TOTAL submodule(s) found"
    echo ""
    echo "Commands:"
    echo "  Initialize all:  git submodule update --init --recursive"
    echo "  Update all:      git submodule update --remote"
    echo "  Foreach command: git submodule foreach 'git status'"
fi
