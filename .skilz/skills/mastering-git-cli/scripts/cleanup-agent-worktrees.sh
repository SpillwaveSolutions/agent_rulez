#!/bin/bash
#
# cleanup-agent-worktrees.sh
# Remove all agent worktrees and optionally delete branches
#
# Usage:
#   ./cleanup-agent-worktrees.sh [options]
#
# Options:
#   --delete-branches    Also delete the agent-*-work branches
#   --force              Force removal even with uncommitted changes
#   --dry-run            Show what would be done without doing it
#   --help               Show this help message
#
# Examples:
#   ./cleanup-agent-worktrees.sh                    # Remove worktrees only
#   ./cleanup-agent-worktrees.sh --delete-branches  # Remove worktrees and branches
#   ./cleanup-agent-worktrees.sh --force            # Force remove with changes
#   ./cleanup-agent-worktrees.sh --dry-run          # Preview actions
#

set -e

# Configuration
DELETE_BRANCHES=false
FORCE=false
DRY_RUN=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
info() { echo -e "${BLUE}[INFO]${NC} $1"; }
success() { echo -e "${GREEN}[OK]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }
dry() { echo -e "${YELLOW}[DRY-RUN]${NC} Would: $1"; }

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --delete-branches)
            DELETE_BRANCHES=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --help)
            head -30 "$0" | tail -25
            exit 0
            ;;
        *)
            error "Unknown option: $1. Use --help for usage."
            ;;
    esac
done

# Validate we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    error "Not in a git repository"
fi

info "Cleaning up agent worktrees..."
if $DRY_RUN; then
    warn "DRY-RUN mode: No changes will be made"
fi

# Build force flag
FORCE_FLAG=""
if $FORCE; then
    FORCE_FLAG="--force"
fi

# Find and remove agent worktrees
WORKTREES_REMOVED=0
while IFS= read -r line; do
    if [[ -z "$line" ]]; then
        continue
    fi
    
    # Extract path from porcelain output
    WORKTREE_PATH=$(echo "$line" | sed 's/^worktree //')
    
    # Check if this is an agent worktree
    if [[ "$WORKTREE_PATH" =~ agent-[0-9]+ ]] || [[ "$WORKTREE_PATH" =~ /agent-[0-9]+$ ]]; then
        if $DRY_RUN; then
            dry "git worktree remove $FORCE_FLAG $WORKTREE_PATH"
        else
            info "Removing worktree: $WORKTREE_PATH"
            if git worktree remove $FORCE_FLAG "$WORKTREE_PATH" 2>/dev/null; then
                success "Removed: $WORKTREE_PATH"
                ((WORKTREES_REMOVED++))
            else
                warn "Failed to remove: $WORKTREE_PATH (may have uncommitted changes, use --force)"
            fi
        fi
    fi
done < <(git worktree list --porcelain | grep "^worktree ")

# Remove integration worktree
INTEGRATION_PATH="../integration"
if git worktree list | grep -q "$INTEGRATION_PATH"; then
    if $DRY_RUN; then
        dry "git worktree remove $FORCE_FLAG $INTEGRATION_PATH"
    else
        info "Removing integration worktree"
        if git worktree remove $FORCE_FLAG "$INTEGRATION_PATH" 2>/dev/null; then
            success "Removed: $INTEGRATION_PATH"
            ((WORKTREES_REMOVED++))
        else
            warn "Failed to remove integration worktree"
        fi
    fi
fi

# Prune stale entries
if $DRY_RUN; then
    dry "git worktree prune"
else
    git worktree prune
    info "Pruned stale worktree entries"
fi

# Delete branches if requested
BRANCHES_DELETED=0
if $DELETE_BRANCHES; then
    info "Deleting agent branches..."
    
    # Delete agent-*-work branches
    for branch in $(git branch --list "agent-*-work" | tr -d ' '); do
        if $DRY_RUN; then
            dry "git branch -D $branch"
        else
            if git branch -D "$branch" 2>/dev/null; then
                success "Deleted branch: $branch"
                ((BRANCHES_DELETED++))
            else
                warn "Failed to delete: $branch"
            fi
        fi
    done
    
    # Delete integration branch
    if git rev-parse --verify "integration" > /dev/null 2>&1; then
        if $DRY_RUN; then
            dry "git branch -D integration"
        else
            if git branch -D "integration" 2>/dev/null; then
                success "Deleted branch: integration"
                ((BRANCHES_DELETED++))
            else
                warn "Failed to delete integration branch"
            fi
        fi
    fi
fi

# Summary
echo ""
if $DRY_RUN; then
    info "DRY-RUN complete. No changes were made."
else
    info "Cleanup complete!"
    echo "  Worktrees removed: $WORKTREES_REMOVED"
    if $DELETE_BRANCHES; then
        echo "  Branches deleted: $BRANCHES_DELETED"
    fi
fi
echo ""
info "Remaining worktrees:"
git worktree list
