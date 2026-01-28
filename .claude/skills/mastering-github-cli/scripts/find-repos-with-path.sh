#!/bin/bash
# find-repos-with-path.sh - Find GitHub repos containing a specific directory/path
#
# Usage: ./find-repos-with-path.sh <path> [owner] [format]
#   path   - Path to search for (e.g., ".skilz", ".cursor", ".codex")
#   owner  - Optional: owner/org to scope search
#   format - Optional: output format (names|urls|json) - default: names
#
# Examples:
#   ./find-repos-with-path.sh .skilz
#   ./find-repos-with-path.sh .cursor myorg
#   ./find-repos-with-path.sh .codex myorg urls
#   ./find-repos-with-path.sh src/components "" json
#
# Exit codes:
#   0 - Success
#   1 - Error or no results

set -euo pipefail

# Dependency check
for cmd in gh jq; do
    command -v "$cmd" &>/dev/null || { echo "Error: $cmd required but not found" >&2; exit 1; }
done

# Verify gh authentication
gh auth status &>/dev/null || { echo "Error: gh not authenticated. Run: gh auth login" >&2; exit 1; }

# Arguments
PATH_QUERY=${1:?Usage: $0 <path> [owner] [format]}
OWNER=${2:-}
FORMAT=${3:-names}

# Validate format
case "$FORMAT" in
    names|urls|json) ;;
    *) echo "Error: format must be 'names', 'urls', or 'json'" >&2; exit 1 ;;
esac

# Build search command
SEARCH_ARGS=("search" "code" "path:$PATH_QUERY")

if [ -n "$OWNER" ]; then
    SEARCH_ARGS+=("--owner" "$OWNER")
fi

SEARCH_ARGS+=("--json" "repository" "--limit" "100")

# Function to deduplicate and format results
format_results() {
    case "$FORMAT" in
        names)
            jq -r '.[].repository.fullName' | sort -u
            ;;
        urls)
            jq -r '.[].repository.url' | sort -u
            ;;
        json)
            jq '[.[].repository | {name: .fullName, url: .url}] | unique_by(.name)'
            ;;
    esac
}

# Check rate limit before starting
check_rate_limit() {
    local remaining
    remaining=$(gh api rate_limit --jq '.resources.code_search.remaining' 2>/dev/null || echo "100")
    if [ "$remaining" -lt 5 ]; then
        echo "Warning: Code search rate limit low ($remaining remaining)" >&2
        local reset
        reset=$(gh api rate_limit --jq '.resources.code_search.reset')
        local wait_time=$((reset - $(date +%s)))
        if [ $wait_time -gt 0 ] && [ $wait_time -lt 120 ]; then
            echo "Waiting ${wait_time}s for rate limit reset..." >&2
            sleep "$wait_time"
        fi
    fi
}

# Main execution
main() {
    check_rate_limit
    
    echo "Searching for repos with path: $PATH_QUERY" >&2
    [ -n "$OWNER" ] && echo "Scoped to owner: $OWNER" >&2
    
    if ! results=$(gh "${SEARCH_ARGS[@]}" 2>&1); then
        echo "Error: Search failed - $results" >&2
        exit 1
    fi
    
    # Check for empty results
    count=$(echo "$results" | jq 'length')
    if [ "$count" -eq 0 ]; then
        echo "No repositories found with path: $PATH_QUERY" >&2
        exit 0
    fi
    
    echo "Found $count results (deduplicating...)" >&2
    
    # Format and output
    echo "$results" | format_results
    
    # Show unique count
    unique_count=$(echo "$results" | jq '[.[].repository.fullName] | unique | length')
    echo "Total unique repos: $unique_count" >&2
}

main
