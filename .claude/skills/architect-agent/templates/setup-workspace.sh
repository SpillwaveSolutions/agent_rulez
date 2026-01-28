#!/bin/bash
# setup-workspace.sh - Set up or update architect/code agent workspaces
# Version: 3.0 (Hybrid Logging v2.0)

set -e  # Exit on error

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Workspace Setup Script v3.0${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Check arguments
if [ $# -lt 2 ]; then
    echo -e "${RED}Usage: $0 <workspace-type> <workspace-path> [options]${NC}"
    echo ""
    echo "workspace-type:"
    echo "  architect    - Set up architect agent workspace"
    echo "  code-agent   - Set up code agent workspace"
    echo ""
    echo "Options:"
    echo "  --code-agent-path <path>    - Path to code agent workspace (required for architect)"
    echo "  --skip-git                  - Skip git initialization"
    echo "  --force                     - Overwrite existing files"
    echo ""
    echo "Examples:"
    echo "  $0 architect ~/projects/my-architect --code-agent-path ~/projects/my-code-agent"
    echo "  $0 code-agent ~/projects/my-code-agent"
    exit 1
fi

WORKSPACE_TYPE="$1"
WORKSPACE_PATH="$2"
shift 2

# Parse options
CODE_AGENT_PATH=""
SKIP_GIT=false
FORCE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --code-agent-path)
            CODE_AGENT_PATH="$2"
            shift 2
            ;;
        --skip-git)
            SKIP_GIT=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Validate workspace type
if [ "$WORKSPACE_TYPE" != "architect" ] && [ "$WORKSPACE_TYPE" != "code-agent" ]; then
    echo -e "${RED}Error: workspace-type must be 'architect' or 'code-agent'${NC}"
    exit 1
fi

# Validate architect requires code-agent-path
if [ "$WORKSPACE_TYPE" = "architect" ] && [ -z "$CODE_AGENT_PATH" ]; then
    echo -e "${RED}Error: architect workspace requires --code-agent-path${NC}"
    exit 1
fi

# Expand paths
WORKSPACE_PATH=$(cd "$(dirname "$WORKSPACE_PATH")" 2>/dev/null && pwd)/$(basename "$WORKSPACE_PATH") || WORKSPACE_PATH="$WORKSPACE_PATH"
if [ -n "$CODE_AGENT_PATH" ]; then
    CODE_AGENT_PATH=$(cd "$(dirname "$CODE_AGENT_PATH")" 2>/dev/null && pwd)/$(basename "$CODE_AGENT_PATH") || CODE_AGENT_PATH="$CODE_AGENT_PATH"
fi

echo -e "${YELLOW}Workspace Type:${NC} $WORKSPACE_TYPE"
echo -e "${YELLOW}Workspace Path:${NC} $WORKSPACE_PATH"
if [ -n "$CODE_AGENT_PATH" ]; then
    echo -e "${YELLOW}Code Agent Path:${NC} $CODE_AGENT_PATH"
fi
echo ""

# Get template directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ "$WORKSPACE_TYPE" = "architect" ]; then
    TEMPLATE_DIR="$SCRIPT_DIR/architect-workspace"
else
    TEMPLATE_DIR="$SCRIPT_DIR/code-agent-workspace"
fi

if [ ! -d "$TEMPLATE_DIR" ]; then
    echo -e "${RED}Error: Template directory not found: $TEMPLATE_DIR${NC}"
    exit 1
fi

echo -e "${GREEN}Using template:${NC} $TEMPLATE_DIR"
echo ""

# Create workspace directory
mkdir -p "$WORKSPACE_PATH"
cd "$WORKSPACE_PATH"

echo -e "${GREEN}Setting up $WORKSPACE_TYPE workspace...${NC}"
echo ""

# Copy template files
echo "📁 Copying template files..."
rsync -av --exclude='.git' "$TEMPLATE_DIR/" "$WORKSPACE_PATH/" | while read line; do
    if [[ ! "$line" =~ ^(sending|sent|total) ]]; then
        echo "  ✅ $line"
    fi
done
echo ""

# Make scripts executable
echo "🔧 Setting permissions..."
if [ "$WORKSPACE_TYPE" = "code-agent" ]; then
    chmod +x .claude/hook-logger.py
    chmod +x debugging/scripts/*.sh
    chmod +x debugging/wrapper-scripts/*.sh
    echo "  ✅ Made scripts executable"
fi
echo ""

# Replace placeholders in files
echo "📝 Configuring workspace..."

if [ "$WORKSPACE_TYPE" = "architect" ]; then
    # Replace placeholders in architect workspace
    find "$WORKSPACE_PATH" -type f \( -name "*.md" -o -name "*.json" -o -name "*.sh" \) -exec sed -i.bak \
        -e "s|\[THIS_WORKSPACE_PATH\]|$WORKSPACE_PATH|g" \
        -e "s|\[PATH_TO_CODE_AGENT_WORKSPACE\]|$CODE_AGENT_PATH|g" \
        -e "s|\[DATE\]|$(date +%Y-%m-%d)|g" \
        {} \;

    # Remove backup files
    find "$WORKSPACE_PATH" -name "*.bak" -delete

    echo "  ✅ Configured architect workspace paths"

    # Update .claude/settings.json with correct paths
    cat > .claude/settings.json <<EOF
{
  "allowedDirectories": [
    "$WORKSPACE_PATH",
    "$CODE_AGENT_PATH/debugging/instructions"
  ]
}
EOF
    echo "  ✅ Updated .claude/settings.json permissions"

else
    # Replace placeholders in code agent workspace
    find "$WORKSPACE_PATH" -type f \( -name "*.md" -o -name "*.json" -o -name "*.sh" \) -exec sed -i.bak \
        -e "s|\[DATE\]|$(date +%Y-%m-%d)|g" \
        {} \;

    # Remove backup files
    find "$WORKSPACE_PATH" -name "*.bak" -delete

    echo "  ✅ Configured code agent workspace"
fi

echo ""

# Git initialization
if [ "$SKIP_GIT" = false ]; then
    if [ ! -d ".git" ]; then
        echo "🔧 Initializing git repository..."
        git init
        echo "  ✅ Git repository initialized"
        echo ""
    else
        echo "ℹ️  Git repository already exists"
        echo ""
    fi
fi

# Summary
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}✅ Workspace setup complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo ""

if [ "$WORKSPACE_TYPE" = "architect" ]; then
    echo "1. Review and customize CLAUDE.md"
    echo "2. Update docs/technology_adaptations.md with project details"
    echo "3. Create your first ticket in ticket/current_ticket.md"
    echo "4. Verify GitHub auth: gh auth status"
    echo "5. Start using: cd $WORKSPACE_PATH"
else
    echo "1. Review and customize CLAUDE.md"
    echo "2. Update CLAUDE.md with project-specific info:"
    echo "   - Project name and purpose"
    echo "   - Technology stack"
    echo "   - Build and test commands"
    echo "3. Test hooks: ./debugging/scripts/log-start.sh 'test' && ls && cat debugging/current_log_file.txt"
    echo "4. Verify hooks working: grep 'TOOL:' \$(cat debugging/current_log_file.txt)"
    echo "5. Start using: cd $WORKSPACE_PATH"
fi

echo ""
echo -e "${GREEN}Workspace ready at: $WORKSPACE_PATH${NC}"
