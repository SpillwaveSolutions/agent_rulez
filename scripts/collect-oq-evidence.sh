#!/bin/bash
# Collect Operational Qualification (OQ) Evidence
#
# This script runs OQ tests and collects evidence for validation reports.
# Output is stored in docs/validation/oq/<date>/
#
# Usage:
#   ./scripts/collect-oq-evidence.sh [--release]
#
# Options:
#   --release    Build and test in release mode (recommended for formal validation)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

# Parse arguments
BUILD_MODE="debug"
CARGO_FLAGS=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            BUILD_MODE="release"
            CARGO_FLAGS="--release"
            shift
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Setup
DATE=$(date +%Y-%m-%d)
TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)
EVIDENCE_DIR="$PROJECT_ROOT/docs/validation/oq/$DATE"
<<<<<<< HEAD
VERSION=$(grep '^version' "$PROJECT_ROOT/rulez/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
=======
VERSION=$(grep '^version' "$PROJECT_ROOT/cch_cli/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
>>>>>>> bc6e5da510358ec5b9a121c5b4c5e210630f4952

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}CCH OQ Evidence Collection${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Version: $VERSION"
echo "Date: $TIMESTAMP"
echo "Mode: $BUILD_MODE"
echo ""

# Create evidence directory
mkdir -p "$EVIDENCE_DIR/test-cases"

# Build CCH
echo -e "${BLUE}Building CCH ($BUILD_MODE)...${NC}"
cd "$PROJECT_ROOT"
cargo build $CARGO_FLAGS 2>&1 | tee "$EVIDENCE_DIR/build.log"

# Track overall status
OVERALL_RESULT=0

# Run OQ test suites
declare -a TEST_SUITES=(
    "oq_us1_blocking:US1 - Blocking Dangerous Commands"
    "oq_us2_injection:US2 - Context Injection"
    "oq_us3_validators:US3 - External Validators"
    "oq_us4_permissions:US4 - Permission Enforcement"
    "oq_us5_logging:US5 - Audit Logging"
)

for suite in "${TEST_SUITES[@]}"; do
    IFS=':' read -r test_name description <<< "$suite"
    
    echo -e "${BLUE}Running $description...${NC}"
    
    if cargo test $CARGO_FLAGS $test_name -- --nocapture 2>&1 | tee "$EVIDENCE_DIR/test-cases/$test_name.log"; then
        echo -e "${GREEN}  ✅ $test_name passed${NC}"
    else
        echo -e "${RED}  ❌ $test_name failed${NC}"
        OVERALL_RESULT=1
    fi
done

# Collect environment info
{
    echo "# Environment Information"
    echo ""
    echo "## System"
    echo '```'
    uname -a
    echo '```'
    echo ""
    echo "## Rust Toolchain"
    echo '```'
    rustc --version
    cargo --version
    echo '```'
} > "$EVIDENCE_DIR/environment.md"

# Generate report
echo -e "${BLUE}Generating OQ report...${NC}"
{
    echo "# OQ Evidence Report"
    echo ""
    echo "**Product:** Claude Context Hooks (CCH)"
    echo "**Version:** $VERSION"
    echo "**Date:** $TIMESTAMP"
    echo "**Build Mode:** $BUILD_MODE"
    echo ""
    echo "---"
    echo ""
    echo "## Summary"
    echo ""
    if [ $OVERALL_RESULT -eq 0 ]; then
        echo "**Status:** ✅ PASS"
        echo ""
        echo "All OQ tests passed successfully."
    else
        echo "**Status:** ❌ FAIL"
        echo ""
        echo "One or more OQ tests failed. See test logs for details."
    fi
    echo ""
    echo "---"
    echo ""
    echo "## Test Results"
    echo ""
    echo "| Test Suite | Description | Status |"
    echo "|------------|-------------|--------|"
    
    for suite in "${TEST_SUITES[@]}"; do
        IFS=':' read -r test_name description <<< "$suite"
        log_file="$EVIDENCE_DIR/test-cases/$test_name.log"
        
        if grep -q "test result: ok" "$log_file" 2>/dev/null; then
            echo "| $test_name | $description | ✅ Pass |"
        else
            echo "| $test_name | $description | ❌ Fail |"
        fi
    done
    
    echo ""
    echo "---"
    echo ""
    echo "## Evidence Files"
    echo ""
    echo "- [Environment](environment.md)"
    echo "- [Build Log](build.log)"
    echo "- Test Cases:"
    for suite in "${TEST_SUITES[@]}"; do
        IFS=':' read -r test_name description <<< "$suite"
        echo "  - [$test_name](test-cases/$test_name.log)"
    done
} > "$EVIDENCE_DIR/report.md"

# Summary
echo ""
echo -e "${BLUE}========================================${NC}"
if [ $OVERALL_RESULT -eq 0 ]; then
    echo -e "${GREEN}OQ Evidence Collection Complete - PASS${NC}"
else
    echo -e "${RED}OQ Evidence Collection Complete - FAIL${NC}"
fi
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Evidence saved to: $EVIDENCE_DIR"
echo ""
ls -la "$EVIDENCE_DIR"

exit $OVERALL_RESULT
