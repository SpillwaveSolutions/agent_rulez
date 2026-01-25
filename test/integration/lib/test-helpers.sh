#!/bin/bash
# Shared test helpers for CCH integration tests
#
# Usage: source this file from test scripts
#   source "$(dirname "$0")/../lib/test-helpers.sh"

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Paths - resolved relative to this script
LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INTEGRATION_DIR="$(cd "$LIB_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$INTEGRATION_DIR/../.." && pwd)"
CCH_CLI_DIR="$PROJECT_ROOT/cch_cli"
# Note: Cargo workspace builds to PROJECT_ROOT/target, not CCH_CLI_DIR/target
CCH_BINARY="$PROJECT_ROOT/target/release/cch"
CCH_LOG="$HOME/.claude/logs/cch.log"
RESULTS_DIR="$INTEGRATION_DIR/results"

# Test state
TEST_NAME=""
TEST_TEMP_DIR=""
TEST_START_TIME=""
ASSERTIONS_PASSED=0
ASSERTIONS_FAILED=0

# Strict mode - exit immediately on first assertion failure
# Enable with: STRICT_MODE=1 ./test.sh
STRICT_MODE="${STRICT_MODE:-0}"

# ============================================================================
# Strict Mode Support
# ============================================================================

# Handle assertion failure based on mode
# Usage: handle_assertion_failure <message> [details]
handle_assertion_failure() {
    local message="$1"
    local details="${2:-}"
    
    ASSERTIONS_FAILED=$((ASSERTIONS_FAILED + 1))
    echo -e "  ${RED}x${NC} FAIL - $message"
    
    if [ -n "$details" ]; then
        echo -e "      $details"
    fi
    
    if [ "$STRICT_MODE" = "1" ]; then
        echo ""
        echo -e "${RED}STRICT MODE: Exiting immediately on first failure${NC}"
        echo -e "  Test: $TEST_NAME"
        echo -e "  Failed assertion: $message"
        
        # Save result before exiting
        save_result "FAIL"
        
        # Cleanup if we have a temp dir
        cleanup_workspace 2>/dev/null || true
        
        exit 1
    fi
    
    return 1
}

# ============================================================================
# Prerequisites
# ============================================================================

check_prerequisites() {
    echo -e "${BLUE}Checking prerequisites...${NC}"
    
    # Check for Claude CLI
    if ! command -v claude &> /dev/null; then
        echo -e "${RED}ERROR - Claude CLI not found in PATH${NC}"
        echo "Install Claude CLI and ensure it's in your PATH"
        echo "See - https://docs.anthropic.com/en/docs/claude-code"
        exit 1
    fi
    echo -e "  ${GREEN}+${NC} Claude CLI found - $(which claude)"
    
    # Check/build CCH binary
    if [ ! -f "$CCH_BINARY" ]; then
        echo -e "  ${YELLOW}!${NC} CCH binary not found, building..."
        build_cch
    else
        echo -e "  ${GREEN}+${NC} CCH binary found - $CCH_BINARY"
    fi
    
    # Ensure log directory exists
    mkdir -p "$(dirname "$CCH_LOG")"
    echo -e "  ${GREEN}+${NC} Log directory ready - $(dirname "$CCH_LOG")"
    
    echo ""
}

build_cch() {
    echo -e "${BLUE}Building CCH binary...${NC}"
    # Build from project root (Cargo workspace)
    (cd "$PROJECT_ROOT" && cargo build --release)
    if [ ! -f "$CCH_BINARY" ]; then
        echo -e "${RED}ERROR - Failed to build CCH binary${NC}"
        exit 1
    fi
    echo -e "  ${GREEN}+${NC} CCH binary built successfully"
}

# ============================================================================
# Test Setup/Teardown
# ============================================================================

# Start a new test - call this at the beginning of each test script
# Usage: start_test "test-name"
start_test() {
    TEST_NAME="${1:-unnamed-test}"
    TEST_START_TIME=$(date +%s)
    ASSERTIONS_PASSED=0
    ASSERTIONS_FAILED=0
    
    echo ""
    echo -e "${BLUE}================================================================${NC}"
    echo -e "${BLUE}TEST - $TEST_NAME${NC}"
    if [ "$STRICT_MODE" = "1" ]; then
        echo -e "${YELLOW}MODE - STRICT (fail-fast enabled)${NC}"
    else
        echo -e "MODE - Normal (soft assertions)"
    fi
    echo -e "${BLUE}================================================================${NC}"
    echo ""
}

# Setup a test workspace by copying the use-case directory to a temp location
# Usage: setup_workspace "/path/to/use-case/dir"
# Returns: temp directory path (also stored in TEST_TEMP_DIR)
setup_workspace() {
    local use_case_dir="${1:-.}"
    
    TEST_TEMP_DIR=$(mktemp -d "/tmp/cch-integration-test-XXXXXX")
    
    # Copy use case files to temp dir
    cp -r "$use_case_dir"/* "$TEST_TEMP_DIR/" 2>/dev/null || true
    cp -r "$use_case_dir"/.[!.]* "$TEST_TEMP_DIR/" 2>/dev/null || true
    
    # Log to stderr so it doesn't corrupt function return value
    echo -e "  ${GREEN}+${NC} Created test workspace - $TEST_TEMP_DIR" >&2
    
    # Return the path on stdout
    echo "$TEST_TEMP_DIR"
}

# Install CCH in the test workspace
# Usage: install_cch [workspace_dir]
install_cch() {
    local workspace="${1:-$TEST_TEMP_DIR}"
    
    echo -e "  ${BLUE}*${NC} Installing CCH in workspace..."
    
    # Run cch install in the workspace
    (cd "$workspace" && "$CCH_BINARY" install --binary "$CCH_BINARY") > /dev/null 2>&1
    
    if [ $? -eq 0 ]; then
        echo -e "  ${GREEN}+${NC} CCH installed successfully"
    else
        echo -e "  ${YELLOW}!${NC} CCH install returned non-zero (may already be installed)"
    fi
}

# Clean up test workspace
# Usage: cleanup_workspace
cleanup_workspace() {
    if [ -n "$TEST_TEMP_DIR" ] && [ -d "$TEST_TEMP_DIR" ]; then
        rm -rf "$TEST_TEMP_DIR"
        echo -e "  ${GREEN}+${NC} Cleaned up test workspace"
    fi
}

# ============================================================================
# Log Management
# ============================================================================

# Clear CCH logs before a test
clear_cch_logs() {
    if [ -f "$CCH_LOG" ]; then
        : > "$CCH_LOG"
    fi
    echo -e "  ${GREEN}+${NC} Cleared CCH logs"
}

# Get the number of lines in the CCH log
get_log_line_count() {
    if [ -f "$CCH_LOG" ]; then
        wc -l < "$CCH_LOG" | tr -d ' '
    else
        echo "0"
    fi
}

# Get log entries added since a specific line number
# Usage: get_new_log_entries <start_line>
get_new_log_entries() {
    local start_line="${1:-0}"
    if [ -f "$CCH_LOG" ]; then
        tail -n "+$((start_line + 1))" "$CCH_LOG"
    fi
}

# Get recent log entries
# Usage: get_recent_logs [count]
get_recent_logs() {
    local count="${1:-10}"
    if [ -f "$CCH_LOG" ]; then
        tail -n "$count" "$CCH_LOG"
    fi
}

# Check if log contains a pattern
# Usage: log_contains "pattern"
log_contains() {
    local pattern="$1"
    grep -q "$pattern" "$CCH_LOG" 2>/dev/null
}

# Check if log contains pattern in entries after a specific line
# Usage: log_contains_since <start_line> "pattern"
log_contains_since() {
    local start_line="$1"
    local pattern="$2"
    get_new_log_entries "$start_line" | grep -q "$pattern" 2>/dev/null
}

# ============================================================================
# Claude CLI Invocation
# ============================================================================

# Default timeout for Claude CLI calls (in seconds)
# Override with CLAUDE_TIMEOUT environment variable
CLAUDE_TIMEOUT="${CLAUDE_TIMEOUT:-60}"

# Get the timeout command (gtimeout on macOS via coreutils, timeout on Linux)
get_timeout_cmd() {
    if command -v timeout &> /dev/null; then
        echo "timeout"
    elif command -v gtimeout &> /dev/null; then
        echo "gtimeout"
    else
        echo ""
    fi
}

# Run Claude CLI with a prompt in a specific directory
# Usage: run_claude <workspace> <prompt> [allowed_tools] [max_turns]
# Returns: exit code, stdout captured in CLAUDE_STDOUT, stderr in CLAUDE_STDERR
# Exit codes:
#   0   - Success
#   124 - Timeout (Claude did not respond in time)
#   Other - Claude CLI error
run_claude() {
    local workspace="$1"
    local prompt="$2"
    local allowed_tools="${3:-Bash Read Write Edit Glob}"
    local max_turns="${4:-3}"
    local timeout_seconds="${CLAUDE_TIMEOUT}"
    local timeout_cmd=$(get_timeout_cmd)
    
    echo -e "  ${BLUE}*${NC} Running Claude CLI..."
    echo -e "      Prompt - $prompt"
    echo -e "      Tools - $allowed_tools"
    echo -e "      Max turns - $max_turns"
    echo -e "      Timeout - ${timeout_seconds}s"
    
    # Capture stdout and stderr separately
    local stdout_file=$(mktemp)
    local stderr_file=$(mktemp)
    
    local exit_code=0
    
    if [ -n "$timeout_cmd" ]; then
        # Use timeout command if available
        (cd "$workspace" && "$timeout_cmd" "$timeout_seconds" claude -p "$prompt" \
            --allowedTools $allowed_tools \
            --max-turns "$max_turns" \
            --output-format text \
            > "$stdout_file" 2> "$stderr_file") || exit_code=$?
    else
        # Fallback: run without timeout (warn user)
        echo -e "  ${YELLOW}!${NC} Warning: timeout command not available, running without timeout"
        (cd "$workspace" && claude -p "$prompt" \
            --allowedTools $allowed_tools \
            --max-turns "$max_turns" \
            --output-format text \
            > "$stdout_file" 2> "$stderr_file") || exit_code=$?
    fi
    
    CLAUDE_STDOUT=$(cat "$stdout_file")
    CLAUDE_STDERR=$(cat "$stderr_file")
    
    rm -f "$stdout_file" "$stderr_file"
    
    if [ $exit_code -eq 0 ]; then
        echo -e "  ${GREEN}+${NC} Claude completed successfully"
    elif [ $exit_code -eq 124 ]; then
        echo -e "  ${RED}x${NC} TIMEOUT: Claude CLI did not respond within ${timeout_seconds}s"
        CLAUDE_STDERR="TIMEOUT: Claude CLI exceeded ${timeout_seconds}s timeout"
    else
        echo -e "  ${YELLOW}!${NC} Claude exited with code $exit_code"
    fi
    
    return $exit_code
}

# ============================================================================
# Assertions
# ============================================================================

# Assert that a condition is true
# Usage: assert_true <condition> <message>
assert_true() {
    local condition="$1"
    local message="${2:-Assertion failed}"
    
    if eval "$condition"; then
        ASSERTIONS_PASSED=$((ASSERTIONS_PASSED + 1))
        echo -e "  ${GREEN}+${NC} PASS - $message"
        return 0
    else
        handle_assertion_failure "$message"
        return 1
    fi
}

# Assert that log contains a pattern
# Usage: assert_log_contains "pattern" "message"
assert_log_contains() {
    local pattern="$1"
    local message="${2:-Log should contain - $pattern}"
    
    if log_contains "$pattern"; then
        ASSERTIONS_PASSED=$((ASSERTIONS_PASSED + 1))
        echo -e "  ${GREEN}+${NC} PASS - $message"
        return 0
    else
        handle_assertion_failure "$message" "Pattern not found - $pattern"
        return 1
    fi
}

# Assert that log contains pattern in recent entries
# Usage: assert_log_contains_since <start_line> "pattern" "message"
assert_log_contains_since() {
    local start_line="$1"
    local pattern="$2"
    local message="${3:-Log should contain - $pattern}"
    
    if log_contains_since "$start_line" "$pattern"; then
        ASSERTIONS_PASSED=$((ASSERTIONS_PASSED + 1))
        echo -e "  ${GREEN}+${NC} PASS - $message"
        return 0
    else
        handle_assertion_failure "$message" "Pattern not found in new log entries - $pattern"
        return 1
    fi
}

# Assert that Claude output contains a pattern
# Usage: assert_claude_output_contains "pattern" "message"
assert_claude_output_contains() {
    local pattern="$1"
    local message="${2:-Claude output should contain - $pattern}"
    
    if echo "$CLAUDE_STDOUT" | grep -q "$pattern"; then
        ASSERTIONS_PASSED=$((ASSERTIONS_PASSED + 1))
        echo -e "  ${GREEN}+${NC} PASS - $message"
        return 0
    else
        handle_assertion_failure "$message" "Pattern not found in Claude output"
        return 1
    fi
}

# Assert that Claude output does NOT contain a pattern
# Usage: assert_claude_output_not_contains "pattern" "message"
assert_claude_output_not_contains() {
    local pattern="$1"
    local message="${2:-Claude output should not contain - $pattern}"
    
    if ! echo "$CLAUDE_STDOUT" | grep -q "$pattern"; then
        ASSERTIONS_PASSED=$((ASSERTIONS_PASSED + 1))
        echo -e "  ${GREEN}+${NC} PASS - $message"
        return 0
    else
        handle_assertion_failure "$message" "Unexpected pattern found in Claude output"
        return 1
    fi
}

# Assert command succeeded
# Usage: assert_success <exit_code> "message"
assert_success() {
    local exit_code="$1"
    local message="${2:-Command should succeed}"
    
    if [ "$exit_code" -eq 0 ]; then
        ASSERTIONS_PASSED=$((ASSERTIONS_PASSED + 1))
        echo -e "  ${GREEN}+${NC} PASS - $message"
        return 0
    else
        handle_assertion_failure "$message" "Exit code was $exit_code, expected 0"
        return 1
    fi
}

# Assert file exists
# Usage: assert_file_exists "path" "message"
assert_file_exists() {
    local file_path="$1"
    local message="${2:-File should exist - $file_path}"
    
    if [ -f "$file_path" ]; then
        ASSERTIONS_PASSED=$((ASSERTIONS_PASSED + 1))
        echo -e "  ${GREEN}+${NC} PASS - $message"
        return 0
    else
        handle_assertion_failure "$message" "File not found - $file_path"
        return 1
    fi
}

# ============================================================================
# Test Results
# ============================================================================

# End a test and report results
# Usage: end_test
# Returns: 0 if all assertions passed, 1 otherwise
end_test() {
    local end_time=$(date +%s)
    local duration=$((end_time - TEST_START_TIME))
    local total=$((ASSERTIONS_PASSED + ASSERTIONS_FAILED))
    
    echo ""
    echo -e "${BLUE}----------------------------------------------------------------${NC}"
    
    if [ "$ASSERTIONS_FAILED" -eq 0 ]; then
        echo -e "${GREEN}PASSED${NC} - $TEST_NAME"
        echo -e "  $ASSERTIONS_PASSED/$total assertions passed in ${duration}s"
        save_result "PASS"
        return 0
    else
        echo -e "${RED}FAILED${NC} - $TEST_NAME"
        echo -e "  $ASSERTIONS_PASSED passed, $ASSERTIONS_FAILED failed in ${duration}s"
        save_result "FAIL"
        return 1
    fi
}

# Save test result to results directory
save_result() {
    local status="$1"
    local result_file="$RESULTS_DIR/${TEST_NAME}-$(date +%Y%m%d_%H%M%S).json"
    
    mkdir -p "$RESULTS_DIR"
    
    cat > "$result_file" << EOF
{
    "test_name": "$TEST_NAME",
    "status": "$status",
    "assertions_passed": $ASSERTIONS_PASSED,
    "assertions_failed": $ASSERTIONS_FAILED,
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "duration_seconds": $(($(date +%s) - TEST_START_TIME))
}
EOF
}

# ============================================================================
# Utility Functions
# ============================================================================

# Print a section header
section() {
    local title="$1"
    echo ""
    echo -e "${BLUE}--- $title ---${NC}"
}

# Print debug info (only if DEBUG is set)
debug() {
    if [ -n "${DEBUG:-}" ]; then
        echo -e "  ${YELLOW}[DEBUG]${NC} $*"
    fi
}

# Wait for a condition with timeout
# Usage: wait_for <condition> <timeout_seconds> <message>
wait_for() {
    local condition="$1"
    local timeout="${2:-30}"
    local message="${3:-Waiting for condition}"
    local elapsed=0
    
    echo -e "  ${BLUE}*${NC} $message (timeout ${timeout}s)..."
    
    while [ $elapsed -lt $timeout ]; do
        if eval "$condition"; then
            echo -e "  ${GREEN}+${NC} Condition met after ${elapsed}s"
            return 0
        fi
        sleep 1
        elapsed=$((elapsed + 1))
    done
    
    echo -e "  ${RED}x${NC} Timeout after ${timeout}s"
    return 1
}
