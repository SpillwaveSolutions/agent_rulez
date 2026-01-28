#!/bin/bash
#
# verify-release.sh
# Verify release workflow completed successfully
#
# Usage: ./verify-release.sh [version]
#
# Checks:
# - Tag exists locally and on remote
# - GitHub release exists
# - Release assets are uploaded
# - Workflow status
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Get version from argument or read from Cargo.toml
if [ -n "$1" ]; then
    VERSION="$1"
else
    VERSION=$("$SCRIPT_DIR/read-version.sh")
fi

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo ""
echo -e "${BLUE}CCH Release Verification: v${VERSION}${NC}"
echo "======================================"
echo ""

ERRORS=0

# Check 1: Local tag exists
echo -e "${BLUE}[1/5]${NC} Checking local tag..."
if git rev-parse "v${VERSION}" >/dev/null 2>&1; then
    TAG_SHA=$(git rev-parse --short "v${VERSION}")
    echo -e "${GREEN}[PASS]${NC} Tag v${VERSION} exists locally (${TAG_SHA})"
else
    echo -e "${RED}[FAIL]${NC} Tag v${VERSION} not found locally"
    echo "  Create with: git tag v${VERSION}"
    ((ERRORS++)) || true
fi

# Check 2: Remote tag exists
echo -e "${BLUE}[2/5]${NC} Checking remote tag..."
if git ls-remote --tags origin 2>/dev/null | grep -q "refs/tags/v${VERSION}$"; then
    echo -e "${GREEN}[PASS]${NC} Tag v${VERSION} pushed to origin"
else
    echo -e "${RED}[FAIL]${NC} Tag v${VERSION} not on remote"
    echo "  Push with: git push origin v${VERSION}"
    ((ERRORS++)) || true
fi

# Check 3: GitHub release exists
echo -e "${BLUE}[3/5]${NC} Checking GitHub release..."
if gh release view "v${VERSION}" > /dev/null 2>&1; then
    echo -e "${GREEN}[PASS]${NC} GitHub release v${VERSION} exists"
    RELEASE_URL=$(gh release view "v${VERSION}" --json url --jq '.url')
    echo "  URL: ${RELEASE_URL}"
else
    echo -e "${YELLOW}[WAIT]${NC} GitHub release not found yet"
    echo "  Workflow may still be running..."
fi

# Check 4: Release assets
echo -e "${BLUE}[4/5]${NC} Checking release assets..."
ASSETS=$(gh release view "v${VERSION}" --json assets --jq '.assets[].name' 2>/dev/null || echo "")
if [ -n "$ASSETS" ]; then
    ASSET_COUNT=$(echo "$ASSETS" | wc -l | tr -d ' ')
    echo -e "${GREEN}[PASS]${NC} Found ${ASSET_COUNT} release assets:"
    echo "$ASSETS" | while read -r asset; do
        echo "  - $asset"
    done

    # Verify expected assets
    EXPECTED_ASSETS=(
        "cch-linux-x86_64.tar.gz"
        "cch-linux-aarch64.tar.gz"
        "cch-macos-x86_64.tar.gz"
        "cch-macos-aarch64.tar.gz"
        "cch-windows-x86_64.exe.zip"
        "checksums.txt"
    )

    MISSING=0
    for expected in "${EXPECTED_ASSETS[@]}"; do
        if ! echo "$ASSETS" | grep -q "$expected"; then
            echo -e "${YELLOW}  Missing: $expected${NC}"
            ((MISSING++)) || true
        fi
    done

    if [ $MISSING -gt 0 ]; then
        echo -e "${YELLOW}[WARN]${NC} $MISSING expected asset(s) missing"
    fi
else
    echo -e "${YELLOW}[WAIT]${NC} No assets found yet"
fi

# Check 5: Workflow status
echo -e "${BLUE}[5/5]${NC} Checking workflow status..."
echo ""
echo "Recent workflow runs:"
gh run list --limit 5 2>/dev/null | head -6 || echo "  Could not fetch workflow runs"

# Summary
echo ""
echo "======================================"
if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}Release verification complete!${NC}"
    echo ""
    echo "Release URL:"
    echo "  https://github.com/SpillwaveSolutions/code_agent_context_hooks/releases/tag/v${VERSION}"
else
    echo -e "${RED}$ERRORS verification error(s)${NC}"
    echo ""
    echo "If workflow is still running, wait and re-run this script."
fi
echo ""
