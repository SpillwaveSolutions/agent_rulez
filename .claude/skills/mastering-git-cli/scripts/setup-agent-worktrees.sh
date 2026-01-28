#!/bin/bash
#
# setup-agent-worktrees.sh
# Create isolated worktrees for parallel agent development
#
# Usage:
#   ./setup-agent-worktrees.sh [num_agents] [base_branch] [prefix]
#
# Arguments:
#   num_agents   - Number of agent worktrees to create (default: 3)
#   base_branch  - Branch to base worktrees on (default: main)
#   prefix       - Worktree path prefix (default: ../agent-)
#
# Examples:
#   ./setup-agent-worktrees.sh                    # 3 agents from main
#   ./setup-agent-worktrees.sh 5                  # 5 agents from main
#   ./setup-agent-worktrees.sh 3 develop          # 3 agents from develop
#   ./setup-agent-worktrees.sh 3 main ../work-    # Custom prefix
#
# Output:
#   Creates worktrees at:
#     ../agent-1, ../agent-2, ... ../agent-N
#     ../integration
#
#   Each agent worktree is on branch: agent-<N>-work
#   Integration worktree is on branch: integration
#

set -e

# Configuration
NUM_AGENTS=${1:-3}
BASE_BRANCH=${2:-main}
PREFIX=${3:-../agent-}

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

# Validate we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    error "Not in a git repository"
fi

# Validate base branch exists
if ! git rev-parse --verify "$BASE_BRANCH" > /dev/null 2>&1; then
    error "Branch '$BASE_BRANCH' does not exist"
fi

# Get repository root
REPO_ROOT=$(git rev-parse --show-toplevel)
info "Repository root: $REPO_ROOT"
info "Creating $NUM_AGENTS agent worktrees from '$BASE_BRANCH'"

# Create agent worktrees
for i in $(seq 1 $NUM_AGENTS); do
    BRANCH_NAME="agent-$i-work"
    WORKTREE_PATH="${PREFIX}${i}"
    
    # Check if worktree already exists
    if git worktree list | grep -q "$WORKTREE_PATH"; then
        warn "Worktree already exists: $WORKTREE_PATH (skipping)"
        continue
    fi
    
    # Check if branch already exists
    if git rev-parse --verify "$BRANCH_NAME" > /dev/null 2>&1; then
        info "Branch exists, creating worktree: $WORKTREE_PATH"
        git worktree add "$WORKTREE_PATH" "$BRANCH_NAME"
    else
        info "Creating new branch and worktree: $WORKTREE_PATH"
        git worktree add -b "$BRANCH_NAME" "$WORKTREE_PATH" "$BASE_BRANCH"
    fi
    
    success "Created: $WORKTREE_PATH on branch $BRANCH_NAME"
done

# Create integration worktree
INTEGRATION_PATH="../integration"
if git worktree list | grep -q "$INTEGRATION_PATH"; then
    warn "Integration worktree already exists (skipping)"
else
    if git rev-parse --verify "integration" > /dev/null 2>&1; then
        git worktree add "$INTEGRATION_PATH" "integration"
    else
        git worktree add -b "integration" "$INTEGRATION_PATH" "$BASE_BRANCH"
    fi
    success "Created: $INTEGRATION_PATH on branch integration"
fi

# Summary
echo ""
info "Worktree setup complete!"
echo ""
git worktree list
echo ""
info "Agent coordination patterns:"
echo "  Independent work:  Each agent works in their worktree, merge to integration"
echo "  Sequential:        Agent 2 can: git fetch origin agent-1-work && git merge origin/agent-1-work"
echo "  Competitive:       Try different approaches, cherry-pick winner to main"
echo ""
info "To sync an agent with main:"
echo "  cd ${PREFIX}1 && git fetch origin $BASE_BRANCH && git rebase origin/$BASE_BRANCH"
echo ""
info "To cleanup when done:"
echo "  ./cleanup-agent-worktrees.sh"
