#!/bin/bash
#
# preflight-check.sh
# Pre-release verification checks for CCH
#
# Usage: ./preflight-check.sh [--json]
#
# Checks:
# - Working directory status
# - Current branch (main or release/*)
# - cargo fmt --check
# - cargo clippy (no warnings)
# - cargo test (all pass)
#
# Exit codes:
# - 0: All checks pass
# - 1: One or more checks failed
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
JSON_OUTPUT=false

if [ "$1" = "--json" ]; then
    JSON_OUTPUT=true
fi

# Colors (disabled for JSON output)
if $JSON_OUTPUT; then
    RED=""
    GREEN=""
    YELLOW=""
    BLUE=""
    NC=""
else
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[0;33m'
    BLUE='\033[0;34m'
    NC='\033[0m'
fi

ERRORS=0
WARNINGS=0

check_pass() {
    $JSON_OUTPUT || echo -e "${GREEN}[PASS]${NC} $1"
}

check_fail() {
    ((ERRORS++)) || true
    $JSON_OUTPUT || echo -e "${RED}[FAIL]${NC} $1"
}

check_warn() {
    ((WARNINGS++)) || true
    $JSON_OUTPUT || echo -e "${YELLOW}[WARN]${NC} $1"
}

check_info() {
    $JSON_OUTPUT || echo -e "${BLUE}[INFO]${NC} $1"
}

# Header
$JSON_OUTPUT || echo ""
$JSON_OUTPUT || echo -e "${BLUE}CCH Release Pre-flight Checks${NC}"
$JSON_OUTPUT || echo "=============================="
$JSON_OUTPUT || echo ""

cd "$REPO_ROOT"

# Check 1: Working directory status
check_info "Checking working directory..."
if [ -z "$(git status --porcelain)" ]; then
    check_pass "Working directory is clean"
else
    MODIFIED_COUNT=$(git status --porcelain | wc -l | tr -d ' ')
    check_warn "Uncommitted changes detected ($MODIFIED_COUNT files)"
    $JSON_OUTPUT || git status --porcelain | head -5
    $JSON_OUTPUT || [ "$MODIFIED_COUNT" -gt 5 ] && echo "  ... and more"
fi

# Check 2: Current branch
check_info "Checking branch..."
BRANCH=$(git branch --show-current)
if [[ "$BRANCH" == "main" || "$BRANCH" == release/* || "$BRANCH" == hotfix/* ]]; then
    check_pass "On branch: $BRANCH"
else
    check_fail "Not on main, release/*, or hotfix/* branch (currently: $BRANCH)"
fi

# Check 3: Format check
check_info "Running cargo fmt --check..."
cd "$REPO_ROOT/cch_cli"
if cargo fmt --check > /dev/null 2>&1; then
    check_pass "cargo fmt --check passes"
else
    check_fail "cargo fmt --check failed - run 'cd cch_cli && cargo fmt'"
fi

# Check 4: Clippy
check_info "Running cargo clippy..."
if cargo clippy --all-targets --all-features -- -D warnings > /dev/null 2>&1; then
    check_pass "cargo clippy passes (no warnings)"
else
    check_fail "cargo clippy has warnings/errors"
    $JSON_OUTPUT || echo "  Run: cd cch_cli && cargo clippy --all-targets --all-features -- -D warnings"
fi

# Check 5: Unit Tests
check_info "Running cargo test..."
TEST_OUTPUT=$(cargo test 2>&1)
if echo "$TEST_OUTPUT" | grep -q "test result: ok"; then
    TEST_SUMMARY=$(echo "$TEST_OUTPUT" | grep "test result:" | head -1)
    check_pass "All unit tests pass: $TEST_SUMMARY"
else
    check_fail "Unit tests failed"
    $JSON_OUTPUT || echo "  Run: cd cch_cli && cargo test"
fi

# Check 5b: Integration Tests
check_info "Running integration tests..."
cd "$REPO_ROOT"
if [ -x "$REPO_ROOT/test/integration/run-all.sh" ]; then
    # Check if Claude CLI is available
    if command -v claude &> /dev/null; then
        INTEGRATION_OUTPUT=$("$REPO_ROOT/test/integration/run-all.sh" 2>&1) || true
        if echo "$INTEGRATION_OUTPUT" | grep -q "All tests passed"; then
            PASSED_COUNT=$(echo "$INTEGRATION_OUTPUT" | grep -o "Passed.*[0-9]" | grep -o "[0-9]*" | head -1)
            check_pass "All integration tests pass (${PASSED_COUNT:-all} passed)"
        elif echo "$INTEGRATION_OUTPUT" | grep -q "PASSED"; then
            check_pass "Integration tests pass"
        else
            check_fail "Integration tests failed"
            $JSON_OUTPUT || echo "  Run: ./test/integration/run-all.sh"
            $JSON_OUTPUT || echo "  Or: task integration-test"
        fi
    else
        check_warn "Claude CLI not available - skipping integration tests"
        $JSON_OUTPUT || echo "  Integration tests require Claude CLI to be installed"
        $JSON_OUTPUT || echo "  Install: https://docs.anthropic.com/en/docs/claude-code"
    fi
else
    check_fail "Integration test runner not found at test/integration/run-all.sh"
fi
cd "$REPO_ROOT/cch_cli"

# Check 6: Version in Cargo.toml
check_info "Checking version..."
cd "$REPO_ROOT"
VERSION=$("$SCRIPT_DIR/read-version.sh" 2>/dev/null || echo "")
if [ -n "$VERSION" ]; then
    check_pass "Version: $VERSION"
else
    check_fail "Could not read version from Cargo.toml"
fi

# Check 7: CHANGELOG.md exists
if [ -f "$REPO_ROOT/CHANGELOG.md" ]; then
    check_pass "CHANGELOG.md exists"
else
    check_warn "CHANGELOG.md not found - create it before release"
fi

# Summary
$JSON_OUTPUT || echo ""
$JSON_OUTPUT || echo "=============================="

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    $JSON_OUTPUT || echo -e "${GREEN}All pre-flight checks passed!${NC}"
    $JSON_OUTPUT && echo "{\"status\": \"pass\", \"errors\": 0, \"warnings\": 0, \"version\": \"$VERSION\"}"
    exit 0
elif [ $ERRORS -eq 0 ]; then
    $JSON_OUTPUT || echo -e "${YELLOW}$WARNINGS warning(s), no critical errors${NC}"
    $JSON_OUTPUT && echo "{\"status\": \"warn\", \"errors\": 0, \"warnings\": $WARNINGS, \"version\": \"$VERSION\"}"
    exit 0
else
    $JSON_OUTPUT || echo -e "${RED}$ERRORS error(s), $WARNINGS warning(s)${NC}"
    $JSON_OUTPUT || echo ""
    $JSON_OUTPUT || echo "Fix errors before proceeding with release."
    $JSON_OUTPUT && echo "{\"status\": \"fail\", \"errors\": $ERRORS, \"warnings\": $WARNINGS, \"version\": \"$VERSION\"}"
    exit 1
fi
