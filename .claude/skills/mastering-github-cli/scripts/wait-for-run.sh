#!/bin/bash
# wait-for-run.sh - Wait for a GitHub Actions workflow run to complete
#
# Usage: ./wait-for-run.sh <run-id> [timeout] [interval]
#   run-id   - The workflow run ID to watch
#   timeout  - Optional: timeout in seconds (default: 3600 = 1 hour)
#   interval - Optional: polling interval in seconds (default: 30)
#
# Exit codes:
#   0 - Run completed successfully
#   1 - Run failed or error occurred
#   2 - Run was cancelled
#   3 - Timeout reached
#
# Examples:
#   ./wait-for-run.sh 12345678
#   ./wait-for-run.sh 12345678 1800       # 30 minute timeout
#   ./wait-for-run.sh 12345678 3600 10    # 1 hour timeout, check every 10s

set -euo pipefail

# Dependency check
for cmd in gh jq; do
    command -v "$cmd" &>/dev/null || { echo "Error: $cmd required but not found" >&2; exit 1; }
done

# Verify gh authentication
gh auth status &>/dev/null || { echo "Error: gh not authenticated. Run: gh auth login" >&2; exit 1; }

# Arguments
RUN_ID=${1:?Usage: $0 <run-id> [timeout] [interval]}
TIMEOUT=${2:-3600}
INTERVAL=${3:-30}

# Validate run ID is numeric
if ! [[ "$RUN_ID" =~ ^[0-9]+$ ]]; then
    echo "Error: run-id must be numeric" >&2
    exit 1
fi

# Track start time
START_TIME=$(date +%s)

# Function to get run status
get_run_status() {
    gh run view "$RUN_ID" --json status,conclusion 2>/dev/null
}

# Function to check if timed out
check_timeout() {
    local elapsed=$(($(date +%s) - START_TIME))
    if [ "$elapsed" -ge "$TIMEOUT" ]; then
        return 0  # Timed out
    fi
    return 1  # Not timed out
}

# Function to format duration
format_duration() {
    local seconds=$1
    local minutes=$((seconds / 60))
    local remaining_seconds=$((seconds % 60))
    if [ "$minutes" -gt 0 ]; then
        echo "${minutes}m ${remaining_seconds}s"
    else
        echo "${remaining_seconds}s"
    fi
}

# Main wait loop
main() {
    echo "Waiting for run $RUN_ID to complete..."
    echo "Timeout: $(format_duration $TIMEOUT), Interval: ${INTERVAL}s"
    echo ""
    
    # Verify run exists
    if ! status_json=$(get_run_status); then
        echo "Error: Could not find run $RUN_ID" >&2
        exit 1
    fi
    
    while true; do
        # Check timeout
        if check_timeout; then
            local elapsed=$(($(date +%s) - START_TIME))
            echo ""
            echo "Timeout reached after $(format_duration $elapsed)"
            exit 3
        fi
        
        # Get current status
        if ! status_json=$(get_run_status); then
            echo "Error: Failed to get run status" >&2
            exit 1
        fi
        
        status=$(echo "$status_json" | jq -r '.status')
        conclusion=$(echo "$status_json" | jq -r '.conclusion')
        elapsed=$(($(date +%s) - START_TIME))
        
        # Print progress
        printf "\r[%s] Status: %-12s Elapsed: %s    " \
            "$(date +%H:%M:%S)" "$status" "$(format_duration $elapsed)"
        
        # Check if completed
        if [ "$status" = "completed" ]; then
            echo ""
            echo ""
            echo "Run completed with conclusion: $conclusion"
            
            case "$conclusion" in
                success)
                    echo "✓ Run succeeded"
                    exit 0
                    ;;
                failure)
                    echo "✗ Run failed"
                    echo ""
                    echo "Failed step logs:"
                    gh run view "$RUN_ID" --log-failed 2>/dev/null | tail -50 || true
                    exit 1
                    ;;
                cancelled)
                    echo "⊘ Run was cancelled"
                    exit 2
                    ;;
                skipped)
                    echo "⊘ Run was skipped"
                    exit 0
                    ;;
                *)
                    echo "? Unknown conclusion: $conclusion"
                    exit 1
                    ;;
            esac
        fi
        
        # Wait for next poll
        sleep "$INTERVAL"
    done
}

# Handle interrupt
trap 'echo ""; echo "Interrupted. Run $RUN_ID may still be in progress."; exit 130' INT TERM

main
