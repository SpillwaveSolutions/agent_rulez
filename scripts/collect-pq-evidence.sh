#!/bin/bash
# Collect Performance Qualification (PQ) Evidence
#
# This script runs PQ tests and collects evidence for validation reports.
# Output is stored in docs/validation/pq/<date>/
#
# Usage:
#   ./scripts/collect-pq-evidence.sh [--release]
#
# Options:
#   --release    Build and test in release mode (REQUIRED for accurate PQ metrics)

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

# Warn if not release mode
if [[ "$BUILD_MODE" != "release" ]]; then
    echo -e "${YELLOW}WARNING: Running PQ tests in debug mode.${NC}"
    echo -e "${YELLOW}For accurate performance metrics, use --release${NC}"
    echo ""
fi

# Setup
DATE=$(date +%Y-%m-%d)
TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)
EVIDENCE_DIR="$PROJECT_ROOT/docs/validation/pq/$DATE"
VERSION=$(grep '^version' "$PROJECT_ROOT/cch_cli/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}CCH PQ Evidence Collection${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Version: $VERSION"
echo "Date: $TIMESTAMP"
echo "Mode: $BUILD_MODE"
echo ""

# Create evidence directory
mkdir -p "$EVIDENCE_DIR"

# Build CCH
echo -e "${BLUE}Building CCH ($BUILD_MODE)...${NC}"
cd "$PROJECT_ROOT"
cargo build $CARGO_FLAGS 2>&1 | tee "$EVIDENCE_DIR/build.log"

# Set binary path
if [[ "$BUILD_MODE" == "release" ]]; then
    BINARY="$PROJECT_ROOT/target/release/cch"
else
    BINARY="$PROJECT_ROOT/target/debug/rulez"
fi

# Collect binary info
echo -e "${BLUE}Collecting binary information...${NC}"
{
    echo "# Binary Information"
    echo ""
    echo "## Size"
    echo '```'
    ls -lh "$BINARY"
    echo '```'
    echo ""
    echo "## File Type"
    echo '```'
    file "$BINARY"
    echo '```'
    echo ""
    if command -v size &> /dev/null; then
        echo "## Sections"
        echo '```'
        size "$BINARY" 2>/dev/null || echo "size command not available"
        echo '```'
    fi
} > "$EVIDENCE_DIR/binary-info.md"

# Run PQ performance tests
echo -e "${BLUE}Running PQ performance tests...${NC}"
cargo test $CARGO_FLAGS pq_performance -- --nocapture 2>&1 | tee "$EVIDENCE_DIR/pq-performance.log"
PERF_RESULT=${PIPESTATUS[0]}

# Run PQ memory tests
echo -e "${BLUE}Running PQ memory tests...${NC}"
cargo test $CARGO_FLAGS pq_memory -- --nocapture 2>&1 | tee "$EVIDENCE_DIR/pq-memory.log"
MEMORY_RESULT=${PIPESTATUS[0]}

# Cold start benchmark
echo -e "${BLUE}Running cold start benchmarks...${NC}"
{
    echo "# Cold Start Benchmarks"
    echo ""
    echo "## --version"
    echo '```'
    for i in {1..10}; do
        echo -n "Run $i: "
        { time "$BINARY" --version > /dev/null; } 2>&1 | grep real
    done
    echo '```'
    echo ""
    echo "## --help"
    echo '```'
    for i in {1..10}; do
        echo -n "Run $i: "
        { time "$BINARY" --help > /dev/null; } 2>&1 | grep real
    done
    echo '```'
} > "$EVIDENCE_DIR/cold-start-benchmark.md"

# Determine overall result
if [ $PERF_RESULT -eq 0 ] && [ $MEMORY_RESULT -eq 0 ]; then
    OVERALL_RESULT=0
else
    OVERALL_RESULT=1
fi

# Collect environment info
{
    echo "# Environment Information"
    echo ""
    echo "## System"
    echo '```'
    uname -a
    echo '```'
    echo ""
    echo "## CPU"
    echo '```'
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sysctl -n machdep.cpu.brand_string 2>/dev/null || echo "Apple Silicon"
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        cat /proc/cpuinfo | grep "model name" | head -1 || echo "Unknown"
    fi
    echo '```'
    echo ""
    echo "## Memory"
    echo '```'
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sysctl -n hw.memsize | awk '{print $1/1024/1024/1024 " GB"}'
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        free -h | head -2
    fi
    echo '```'
    echo ""
    echo "## Rust Toolchain"
    echo '```'
    rustc --version
    cargo --version
    echo '```'
} > "$EVIDENCE_DIR/environment.md"

# Generate report
echo -e "${BLUE}Generating PQ report...${NC}"
{
    echo "# PQ Evidence Report"
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
        echo "All PQ tests passed successfully."
    else
        echo "**Status:** ❌ FAIL"
        echo ""
        echo "One or more PQ tests failed. See test logs for details."
    fi
    echo ""
    echo "---"
    echo ""
    echo "## Performance Targets"
    echo ""
    echo "| Metric | Target | Mode |"
    echo "|--------|--------|------|"
    echo "| Cold Start | <15ms | Release |"
    echo "| Event Processing | <50ms | Release |"
    echo "| Memory Baseline | <10MB RSS | Release |"
    echo "| Binary Size | <10MB | Release |"
    echo ""
    echo "---"
    echo ""
    echo "## Test Results"
    echo ""
    echo "| Test Suite | Status |"
    echo "|------------|--------|"
    if [ $PERF_RESULT -eq 0 ]; then
        echo "| Performance Tests | ✅ Pass |"
    else
        echo "| Performance Tests | ❌ Fail |"
    fi
    if [ $MEMORY_RESULT -eq 0 ]; then
        echo "| Memory Tests | ✅ Pass |"
    else
        echo "| Memory Tests | ❌ Fail |"
    fi
    echo ""
    echo "---"
    echo ""
    echo "## Evidence Files"
    echo ""
    echo "- [Environment](environment.md)"
    echo "- [Binary Info](binary-info.md)"
    echo "- [Performance Tests](pq-performance.log)"
    echo "- [Memory Tests](pq-memory.log)"
    echo "- [Cold Start Benchmarks](cold-start-benchmark.md)"
} > "$EVIDENCE_DIR/report.md"

# Summary
echo ""
echo -e "${BLUE}========================================${NC}"
if [ $OVERALL_RESULT -eq 0 ]; then
    echo -e "${GREEN}PQ Evidence Collection Complete - PASS${NC}"
else
    echo -e "${RED}PQ Evidence Collection Complete - FAIL${NC}"
fi
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Evidence saved to: $EVIDENCE_DIR"
echo ""
ls -la "$EVIDENCE_DIR"

exit $OVERALL_RESULT
