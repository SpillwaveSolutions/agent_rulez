#!/bin/bash
# Collect Installation Qualification (IQ) Evidence
#
# This script runs IQ tests and collects evidence for validation reports.
# Output is stored in docs/validation/iq/<date>/
#
# Usage:
#   ./scripts/collect-iq-evidence.sh [--release]
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
YELLOW='\033[1;33m'
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
EVIDENCE_DIR="$PROJECT_ROOT/docs/validation/iq/$DATE"
<<<<<<< HEAD
VERSION=$(grep '^version' "$PROJECT_ROOT/rulez/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
=======
VERSION=$(grep '^version' "$PROJECT_ROOT/cch_cli/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
>>>>>>> bc6e5da510358ec5b9a121c5b4c5e210630f4952

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}CCH IQ Evidence Collection${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Version: $VERSION"
echo "Date: $TIMESTAMP"
echo "Mode: $BUILD_MODE"
echo ""

# Create evidence directory
mkdir -p "$EVIDENCE_DIR"

# Collect environment info
echo -e "${BLUE}Collecting environment information...${NC}"
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
    echo ""
    echo "## Platform"
    echo '```'
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sw_vers 2>/dev/null || echo "macOS"
        sysctl -n machdep.cpu.brand_string 2>/dev/null || echo "Apple Silicon"
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        cat /etc/os-release 2>/dev/null || echo "Linux"
        uname -m
    fi
    echo '```'
} > "$EVIDENCE_DIR/environment.md"

# Build CCH
echo -e "${BLUE}Building CCH ($BUILD_MODE)...${NC}"
cd "$PROJECT_ROOT"
cargo build $CARGO_FLAGS 2>&1 | tee "$EVIDENCE_DIR/build.log"

# Run IQ tests
echo -e "${BLUE}Running IQ tests...${NC}"
cargo test $CARGO_FLAGS iq_ -- --nocapture 2>&1 | tee "$EVIDENCE_DIR/iq-tests.log"
IQ_RESULT=${PIPESTATUS[0]}

# Check binary functionality
echo -e "${BLUE}Verifying binary functionality...${NC}"
{
    echo "# Binary Verification"
    echo ""
    echo "## Version"
    echo '```'
    if [[ "$BUILD_MODE" == "release" ]]; then
<<<<<<< HEAD
        ./target/release/rulez --version
    else
        ./target/debug/rulez --version
=======
        ./target/release/cch --version
    else
        ./target/debug/cch --version
>>>>>>> bc6e5da510358ec5b9a121c5b4c5e210630f4952
    fi
    echo '```'
    echo ""
    echo "## Help"
    echo '```'
    if [[ "$BUILD_MODE" == "release" ]]; then
<<<<<<< HEAD
        ./target/release/rulez --help
    else
        ./target/debug/rulez --help
=======
        ./target/release/cch --help
    else
        ./target/debug/cch --help
>>>>>>> bc6e5da510358ec5b9a121c5b4c5e210630f4952
    fi
    echo '```'
    echo ""
    echo "## Validate (no config)"
    echo '```'
    if [[ "$BUILD_MODE" == "release" ]]; then
<<<<<<< HEAD
        ./target/release/rulez validate 2>&1 || true
    else
        ./target/debug/rulez validate 2>&1 || true
=======
        ./target/release/cch validate 2>&1 || true
    else
        ./target/debug/cch validate 2>&1 || true
>>>>>>> bc6e5da510358ec5b9a121c5b4c5e210630f4952
    fi
    echo '```'
} > "$EVIDENCE_DIR/binary-verification.md"

# Generate report
echo -e "${BLUE}Generating IQ report...${NC}"
{
    echo "# IQ Evidence Report"
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
    if [ $IQ_RESULT -eq 0 ]; then
        echo "**Status:** ✅ PASS"
        echo ""
        echo "All IQ tests passed successfully."
    else
        echo "**Status:** ❌ FAIL"
        echo ""
        echo "One or more IQ tests failed. See iq-tests.log for details."
    fi
    echo ""
    echo "---"
    echo ""
    echo "## Evidence Files"
    echo ""
    echo "- [Environment](environment.md)"
    echo "- [Build Log](build.log)"
    echo "- [IQ Test Results](iq-tests.log)"
    echo "- [Binary Verification](binary-verification.md)"
    echo ""
    echo "---"
    echo ""
    echo "## Test Output"
    echo ""
    echo '```'
    tail -50 "$EVIDENCE_DIR/iq-tests.log"
    echo '```'
} > "$EVIDENCE_DIR/report.md"

# Summary
echo ""
echo -e "${BLUE}========================================${NC}"
if [ $IQ_RESULT -eq 0 ]; then
    echo -e "${GREEN}IQ Evidence Collection Complete - PASS${NC}"
else
    echo -e "${RED}IQ Evidence Collection Complete - FAIL${NC}"
fi
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Evidence saved to: $EVIDENCE_DIR"
echo ""
ls -la "$EVIDENCE_DIR"

exit $IQ_RESULT
