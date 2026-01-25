#!/bin/bash
# Run all CCH integration tests
#
# Usage:
#   ./run-all.sh              # Run all tests (soft assertions)
#   ./run-all.sh --strict     # Run with strict mode (fail-fast)
#   ./run-all.sh --quick      # Run only quick tests (skip slow ones)
#   DEBUG=1 ./run-all.sh      # Run with debug output
#   STRICT_MODE=1 ./run-all.sh  # Alternative strict mode via env var

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/test-helpers.sh"

# Parse arguments
QUICK_MODE=false
SPECIFIC_TEST=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --quick)
            QUICK_MODE=true
            shift
            ;;
        --strict)
            export STRICT_MODE=1
            shift
            ;;
        --test)
            SPECIFIC_TEST="$2"
            shift 2
            ;;
        *)
            echo "Unknown option - $1"
            echo "Usage - ./run-all.sh [--quick] [--strict] [--test <test-name>]"
            exit 1
            ;;
    esac
done

# Banner
echo ""
echo -e "${BLUE}+============================================================+${NC}"
echo -e "${BLUE}|       CCH Integration Test Suite                           |${NC}"
echo -e "${BLUE}+============================================================+${NC}"
if [ "${STRICT_MODE:-0}" = "1" ]; then
    echo -e "${YELLOW}|       MODE: STRICT (fail-fast on first assertion failure) |${NC}"
else
    echo -e "|       MODE: Normal (soft assertions)                       |"
fi
echo -e "${BLUE}+============================================================+${NC}"
echo ""

# Check prerequisites
check_prerequisites

# Track results
PASSED=0
FAILED=0
SKIPPED=0
TOTAL=0
FAILED_TESTS=""

# Run each use case
for test_dir in "$SCRIPT_DIR/use-cases"/*; do
    if [ -d "$test_dir" ] && [ -f "$test_dir/test.sh" ]; then
        test_name=$(basename "$test_dir")
        
        # Skip if specific test requested and this isn't it
        if [ -n "$SPECIFIC_TEST" ] && [ "$test_name" != "$SPECIFIC_TEST" ]; then
            continue
        fi
        
        # Check for slow test marker in quick mode
        if [ "$QUICK_MODE" = true ] && [ -f "$test_dir/.slow" ]; then
            echo -e "${YELLOW}SKIPPED${NC} - $test_name (slow test, use full mode)"
            SKIPPED=$((SKIPPED + 1))
            continue
        fi
        
        TOTAL=$((TOTAL + 1))
        
        # Run the test
        if (cd "$test_dir" && bash test.sh); then
            PASSED=$((PASSED + 1))
        else
            FAILED=$((FAILED + 1))
            FAILED_TESTS="$FAILED_TESTS\n  - $test_name"
        fi
        
        echo ""
    fi
done

# Summary
echo -e "${BLUE}+============================================================+${NC}"
echo -e "${BLUE}|                    TEST SUMMARY                            |${NC}"
echo -e "${BLUE}+============================================================+${NC}"
echo ""
echo -e "  Total    - $TOTAL"
echo -e "  ${GREEN}Passed${NC}   - $PASSED"
echo -e "  ${RED}Failed${NC}   - $FAILED"
if [ $SKIPPED -gt 0 ]; then
    echo -e "  ${YELLOW}Skipped${NC}  - $SKIPPED"
fi
echo ""

if [ "$FAILED" -gt 0 ]; then
    echo -e "${RED}Failed tests -${NC}"
    echo -e "$FAILED_TESTS"
    echo ""
    exit 1
else
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
fi
