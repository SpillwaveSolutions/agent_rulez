#!/bin/bash
# batch-search.sh - Search GitHub with >1000 results using date partitioning
#
# GitHub Search API limits results to 1000 per query. This script works around
# that limitation by splitting searches into date ranges and combining results.
#
# Usage: ./batch-search.sh <query> <start-date> <end-date> [partition-days] [type]
#   query          - Search query (without date qualifiers)
#   start-date     - Start date (YYYY-MM-DD)
#   end-date       - End date (YYYY-MM-DD)
#   partition-days - Optional: days per partition (default: 30)
#   type           - Optional: 'repos' or 'code' (default: repos)
#
# Examples:
#   ./batch-search.sh "language:python topic:cli" 2024-01-01 2024-12-31
#   ./batch-search.sh "language:go stars:>100" 2023-01-01 2024-12-31 60
#   ./batch-search.sh "filename:SKILL.md" 2024-01-01 2024-06-30 30 code
#
# Output: JSON array of all unique results
#
# Exit codes:
#   0 - Success
#   1 - Error

set -euo pipefail

# Dependency check
for cmd in gh jq date; do
    command -v "$cmd" &>/dev/null || { echo "Error: $cmd required but not found" >&2; exit 1; }
done

# Verify gh authentication
gh auth status &>/dev/null || { echo "Error: gh not authenticated. Run: gh auth login" >&2; exit 1; }

# Arguments
QUERY=${1:?Usage: $0 <query> <start-date> <end-date> [partition-days] [type]}
START_DATE=${2:?Usage: $0 <query> <start-date> <end-date> [partition-days] [type]}
END_DATE=${3:?Usage: $0 <query> <start-date> <end-date> [partition-days] [type]}
PARTITION_DAYS=${4:-30}
SEARCH_TYPE=${5:-repos}

# Validate search type
case "$SEARCH_TYPE" in
    repos|code) ;;
    *) echo "Error: type must be 'repos' or 'code'" >&2; exit 1 ;;
esac

# Validate dates
validate_date() {
    if ! date -d "$1" &>/dev/null; then
        echo "Error: Invalid date format: $1 (use YYYY-MM-DD)" >&2
        exit 1
    fi
}
validate_date "$START_DATE"
validate_date "$END_DATE"

# Convert dates to seconds for comparison
start_seconds=$(date -d "$START_DATE" +%s)
end_seconds=$(date -d "$END_DATE" +%s)

if [ "$start_seconds" -ge "$end_seconds" ]; then
    echo "Error: start-date must be before end-date" >&2
    exit 1
fi

# Calculate partition size in seconds
partition_seconds=$((PARTITION_DAYS * 86400))

# Temp file for collecting results
RESULTS_FILE=$(mktemp)
trap 'rm -f "$RESULTS_FILE"' EXIT

# Rate limit delay based on search type
if [ "$SEARCH_TYPE" = "code" ]; then
    DELAY=6  # Code search: 10/minute
else
    DELAY=2  # Repo search: 30/minute
fi

# Function to search a date range
search_range() {
    local range_start=$1
    local range_end=$2
    local range_start_fmt=$(date -d "@$range_start" +%Y-%m-%d)
    local range_end_fmt=$(date -d "@$range_end" +%Y-%m-%d)
    
    local date_qualifier
    if [ "$SEARCH_TYPE" = "repos" ]; then
        date_qualifier="created:${range_start_fmt}..${range_end_fmt}"
    else
        # Code search uses pushed date
        date_qualifier="pushed:${range_start_fmt}..${range_end_fmt}"
    fi
    
    local full_query="$QUERY $date_qualifier"
    
    echo "  Searching: $range_start_fmt to $range_end_fmt" >&2
    
    local result
    if [ "$SEARCH_TYPE" = "repos" ]; then
        result=$(gh search repos "$full_query" --limit 1000 --json fullName,url,createdAt 2>/dev/null) || result="[]"
    else
        result=$(gh search code "$full_query" --limit 100 --json repository,path 2>/dev/null) || result="[]"
    fi
    
    local count=$(echo "$result" | jq 'length')
    echo "    Found: $count results" >&2
    
    # Warn if hitting limit
    if [ "$count" -ge 1000 ] && [ "$SEARCH_TYPE" = "repos" ]; then
        echo "    Warning: Hit 1000 result limit. Consider smaller partitions." >&2
    elif [ "$count" -ge 100 ] && [ "$SEARCH_TYPE" = "code" ]; then
        echo "    Warning: Hit 100 result limit for code search." >&2
    fi
    
    echo "$result"
}

# Main execution
main() {
    echo "Batch search starting..." >&2
    echo "Query: $QUERY" >&2
    echo "Range: $START_DATE to $END_DATE" >&2
    echo "Partition: ${PARTITION_DAYS} days" >&2
    echo "Type: $SEARCH_TYPE" >&2
    echo "" >&2
    
    # Initialize results array
    echo "[]" > "$RESULTS_FILE"
    
    # Iterate through date partitions
    current=$start_seconds
    partition_count=0
    
    while [ "$current" -lt "$end_seconds" ]; do
        partition_count=$((partition_count + 1))
        
        # Calculate partition end
        partition_end=$((current + partition_seconds))
        if [ "$partition_end" -gt "$end_seconds" ]; then
            partition_end=$end_seconds
        fi
        
        # Search this partition
        partition_results=$(search_range "$current" "$partition_end")
        
        # Merge results
        jq -s 'add' "$RESULTS_FILE" <(echo "$partition_results") > "${RESULTS_FILE}.tmp"
        mv "${RESULTS_FILE}.tmp" "$RESULTS_FILE"
        
        # Move to next partition
        current=$((partition_end + 86400))  # +1 day to avoid overlap
        
        # Rate limit delay (skip after last partition)
        if [ "$current" -lt "$end_seconds" ]; then
            sleep "$DELAY"
        fi
    done
    
    echo "" >&2
    echo "Searched $partition_count partitions" >&2
    
    # Deduplicate and output
    if [ "$SEARCH_TYPE" = "repos" ]; then
        # Dedupe by fullName for repos
        jq 'unique_by(.fullName) | sort_by(.fullName)' "$RESULTS_FILE"
        final_count=$(jq 'unique_by(.fullName) | length' "$RESULTS_FILE")
    else
        # Dedupe by repo+path for code
        jq 'unique_by(.repository.fullName + .path) | sort_by(.repository.fullName)' "$RESULTS_FILE"
        final_count=$(jq 'unique_by(.repository.fullName + .path) | length' "$RESULTS_FILE")
    fi
    
    echo "" >&2
    echo "Total unique results: $final_count" >&2
}

main
