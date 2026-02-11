#!/bin/bash
# Generate Combined IQ/OQ/PQ Validation Report
#
# This script generates a comprehensive validation report by aggregating
# evidence from IQ, OQ, and PQ phases.
#
# Usage:
#   ./scripts/generate-validation-report.sh [--date YYYY-MM-DD]
#
# Options:
#   --date    Specify date to aggregate (default: today)

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
DATE=$(date +%Y-%m-%d)

while [[ $# -gt 0 ]]; do
    case $1 in
        --date)
            DATE="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Setup
TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)
VALIDATION_DIR="$PROJECT_ROOT/docs/validation"
REPORT_DIR="$VALIDATION_DIR/sign-off"
VERSION=$(grep '^version' "$PROJECT_ROOT/cch_cli/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}CCH Validation Report Generator${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Version: $VERSION"
echo "Date: $DATE"
echo ""

# Check for evidence directories
IQ_DIR="$VALIDATION_DIR/iq/$DATE"
OQ_DIR="$VALIDATION_DIR/oq/$DATE"
PQ_DIR="$VALIDATION_DIR/pq/$DATE"

IQ_STATUS="❌ Not Found"
OQ_STATUS="❌ Not Found"
PQ_STATUS="❌ Not Found"

if [ -f "$IQ_DIR/report.md" ]; then
    if grep -q "✅ PASS" "$IQ_DIR/report.md"; then
        IQ_STATUS="✅ Pass"
    else
        IQ_STATUS="❌ Fail"
    fi
fi

if [ -f "$OQ_DIR/report.md" ]; then
    if grep -q "✅ PASS" "$OQ_DIR/report.md"; then
        OQ_STATUS="✅ Pass"
    else
        OQ_STATUS="❌ Fail"
    fi
fi

if [ -f "$PQ_DIR/report.md" ]; then
    if grep -q "✅ PASS" "$PQ_DIR/report.md"; then
        PQ_STATUS="✅ Pass"
    else
        PQ_STATUS="❌ Fail"
    fi
fi

# Determine overall status
if [[ "$IQ_STATUS" == "✅ Pass" ]] && [[ "$OQ_STATUS" == "✅ Pass" ]] && [[ "$PQ_STATUS" == "✅ Pass" ]]; then
    OVERALL_STATUS="✅ PASSED"
    CONCLUSION="All validation phases completed successfully. CCH v$VERSION is qualified for release."
else
    OVERALL_STATUS="❌ FAILED"
    CONCLUSION="One or more validation phases did not pass. Review evidence before release."
fi

# Create report
mkdir -p "$REPORT_DIR"
REPORT_FILE="$REPORT_DIR/validation-report-$DATE.md"

{
    cat << 'HEADER'
# CCH Validation Report

## Document Control

| Field | Value |
|-------|-------|
HEADER
    echo "| **Product** | Claude Context Hooks (CCH) |"
    echo "| **Version** | $VERSION |"
    echo "| **Validation Date** | $DATE |"
    echo "| **Report Generated** | $TIMESTAMP |"
    echo "| **Overall Status** | $OVERALL_STATUS |"
    echo ""
    echo "---"
    echo ""
    echo "## Executive Summary"
    echo ""
    echo "$CONCLUSION"
    echo ""
    echo "---"
    echo ""
    echo "## Validation Results"
    echo ""
    echo "### Phase Summary"
    echo ""
    echo "| Phase | Description | Status |"
    echo "|-------|-------------|--------|"
    echo "| IQ | Installation Qualification | $IQ_STATUS |"
    echo "| OQ | Operational Qualification | $OQ_STATUS |"
    echo "| PQ | Performance Qualification | $PQ_STATUS |"
    echo ""
    echo "---"
    echo ""
    echo "## Installation Qualification (IQ)"
    echo ""
    echo "**Purpose:** Verify CCH installs correctly on all supported platforms."
    echo ""
    if [ -f "$IQ_DIR/report.md" ]; then
        echo "**Status:** $IQ_STATUS"
        echo ""
        echo "**Evidence Location:** \`docs/validation/iq/$DATE/\`"
        echo ""
        echo "### IQ Evidence Files"
        echo ""
        ls -1 "$IQ_DIR" 2>/dev/null | while read file; do
            echo "- [$file](../iq/$DATE/$file)"
        done
    else
        echo "**Status:** Evidence not collected for this date."
        echo ""
        echo "Run: \`./scripts/collect-iq-evidence.sh --release\`"
    fi
    echo ""
    echo "---"
    echo ""
    echo "## Operational Qualification (OQ)"
    echo ""
    echo "**Purpose:** Verify CCH operates correctly under normal conditions."
    echo ""
    if [ -f "$OQ_DIR/report.md" ]; then
        echo "**Status:** $OQ_STATUS"
        echo ""
        echo "**Evidence Location:** \`docs/validation/oq/$DATE/\`"
        echo ""
        echo "### OQ Test Suites"
        echo ""
        echo "| Suite | Description |"
        echo "|-------|-------------|"
        echo "| US1 | Blocking Dangerous Commands |"
        echo "| US2 | Context Injection |"
        echo "| US3 | External Validators |"
        echo "| US4 | Permission Enforcement |"
        echo "| US5 | Audit Logging |"
    else
        echo "**Status:** Evidence not collected for this date."
        echo ""
        echo "Run: \`./scripts/collect-oq-evidence.sh --release\`"
    fi
    echo ""
    echo "---"
    echo ""
    echo "## Performance Qualification (PQ)"
    echo ""
    echo "**Purpose:** Verify CCH meets performance requirements."
    echo ""
    if [ -f "$PQ_DIR/report.md" ]; then
        echo "**Status:** $PQ_STATUS"
        echo ""
        echo "**Evidence Location:** \`docs/validation/pq/$DATE/\`"
        echo ""
        echo "### Performance Targets"
        echo ""
        echo "| Metric | Target |"
        echo "|--------|--------|"
        echo "| Cold Start | <15ms |"
        echo "| Event Processing | <50ms |"
        echo "| Memory (RSS) | <10MB |"
        echo "| Binary Size | <10MB |"
    else
        echo "**Status:** Evidence not collected for this date."
        echo ""
        echo "Run: \`./scripts/collect-pq-evidence.sh --release\`"
    fi
    echo ""
    echo "---"
    echo ""
    echo "## Sign-Off"
    echo ""
    echo "| Role | Name | Signature | Date |"
    echo "|------|------|-----------|------|"
    echo "| QA Lead | | | |"
    echo "| Dev Lead | | | |"
    echo "| Product Owner | | | |"
    echo ""
    echo "---"
    echo ""
    echo "*This report was generated by \`generate-validation-report.sh\`*"
} > "$REPORT_FILE"

# Summary
echo -e "${BLUE}========================================${NC}"
echo -e "Validation Report: $OVERALL_STATUS"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "IQ: $IQ_STATUS"
echo "OQ: $OQ_STATUS"
echo "PQ: $PQ_STATUS"
echo ""
echo "Report saved to: $REPORT_FILE"
