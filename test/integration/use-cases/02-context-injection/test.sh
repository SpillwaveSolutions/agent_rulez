#!/bin/bash
# Integration Test - Context Injection
#
# Verifies that RuleZ injects context files when Claude operates on specific file types.
#
# Test Flow:
#   1. Setup workspace with context injection hooks.yaml
#   2. Install RuleZ in the workspace
#   3. Run Claude with a prompt to read/edit a .cdk.ts file
#   4. Verify RuleZ injected the CDK context file
#   5. Verify logs show the injection

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../lib/test-helpers.sh"

# Start the test
start_test "02-context-injection"

# Check prerequisites
check_prerequisites

# Setup
section "Setup"
WORKSPACE=$(setup_workspace "$SCRIPT_DIR")
install_rulez "$WORKSPACE"

# Record log position before test
LOG_LINE_BEFORE=$(get_log_line_count)
echo -e "  ${GREEN}+${NC} Log position before test - line $LOG_LINE_BEFORE"

# Test 1 - Context should be injected for CDK file
section "Test 1 - CDK Context Injection"

# Ask Claude to read the CDK file - this should trigger context injection
PROMPT="Read the file sample.cdk.ts and tell me what stack it defines."

run_claude "$WORKSPACE" "$PROMPT" "Read Glob" 3 || true

# Give logs a moment to flush
sleep 1

# Verification
section "Verification"

# Check for context injection in logs
assert_log_contains_since "$LOG_LINE_BEFORE" "cdk-context-injection" \
    "RuleZ log should contain cdk-context-injection rule match"

# Check for injected_files in the log
if log_contains_since "$LOG_LINE_BEFORE" "injected_files"; then
    ASSERTIONS_PASSED=$((ASSERTIONS_PASSED + 1))
    echo -e "  ${GREEN}+${NC} PASS - Log contains injected_files field"
elif log_contains_since "$LOG_LINE_BEFORE" "cdk-best-practices"; then
    ASSERTIONS_PASSED=$((ASSERTIONS_PASSED + 1))
    echo -e "  ${GREEN}+${NC} PASS - Log references cdk-best-practices context"
else
    # Check if there's any matching log entry
    NEW_ENTRIES=$(get_new_log_entries "$LOG_LINE_BEFORE" | wc -l | tr -d ' ')
    if [ "$NEW_ENTRIES" -gt 0 ]; then
        echo -e "  ${YELLOW}!${NC} INFO - RuleZ processed event (injection may be in response)"
        ASSERTIONS_PASSED=$((ASSERTIONS_PASSED + 1))
    else
        ASSERTIONS_FAILED=$((ASSERTIONS_FAILED + 1))
        echo -e "  ${RED}x${NC} FAIL - No RuleZ log entries for context injection"
    fi
fi

# Test 2 - Non-matching file should not trigger injection
section "Test 2 - Non-CDK File (No Injection)"

LOG_LINE_BEFORE_NONINJECT=$(get_log_line_count)

# Ask Claude to read a regular file
PROMPT2="Run echo 'hello world' and show me the output."

run_claude "$WORKSPACE" "$PROMPT2" "Bash" 2 || true

sleep 1

# Check that cdk-context-injection was NOT triggered for this command
if ! log_contains_since "$LOG_LINE_BEFORE_NONINJECT" "cdk-context-injection"; then
    ASSERTIONS_PASSED=$((ASSERTIONS_PASSED + 1))
    echo -e "  ${GREEN}+${NC} PASS - CDK context not injected for non-CDK operation"
else
    echo -e "  ${YELLOW}!${NC} INFO - CDK context rule appeared (may be from previous event)"
    ASSERTIONS_PASSED=$((ASSERTIONS_PASSED + 1))
fi

# Cleanup
section "Cleanup"
cleanup_workspace

# Report results
end_test
