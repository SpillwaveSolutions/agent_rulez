#!/usr/bin/env bash
#
# tail-code-agent-logs.sh - Watch code agent logs in real-time
#
# Usage:
#   ./tail-code-agent-logs.sh           # Tail active log (from current_log_file.txt)
#   ./tail-code-agent-logs.sh --latest  # Tail most recently modified log
#   ./tail-code-agent-logs.sh --all     # Tail all logs (newest first)
#   ./tail-code-agent-logs.sh --list    # List available logs
#   ./tail-code-agent-logs.sh <file>    # Tail specific log file
#

set -euo pipefail

# Configuration
CODE_AGENT_DIR="${CODE_AGENT_DIR:-[PATH_TO_CODE_AGENT_WORKSPACE]}"
LOGS_DIR="$CODE_AGENT_DIR/debugging/logs"
CURRENT_LOG_PTR="$CODE_AGENT_DIR/debugging/current_log_file.txt"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color
BOLD='\033[1m'

print_header() {
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}  ${BOLD}Code Agent Log Viewer${NC}                                     ${BLUE}║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_usage() {
    echo -e "${BOLD}Usage:${NC}"
    echo "  $0              # Tail active log (from current_log_file.txt)"
    echo "  $0 --latest     # Tail most recently modified log"
    echo "  $0 --all        # Tail all logs (newest first)"
    echo "  $0 --list       # List available logs"
    echo "  $0 <file>       # Tail specific log file"
    echo ""
    echo -e "${BOLD}Environment:${NC}"
    echo "  CODE_AGENT_DIR  # Override code agent directory"
}

list_logs() {
    print_header
    echo -e "${BOLD}Available Logs:${NC}"
    echo ""

    if [[ ! -d "$LOGS_DIR" ]]; then
        echo -e "${RED}Error: Logs directory not found: $LOGS_DIR${NC}"
        exit 1
    fi

    local active_log=""
    if [[ -f "$CURRENT_LOG_PTR" ]]; then
        active_log=$(cat "$CURRENT_LOG_PTR" 2>/dev/null || echo "")
    fi

    # List logs sorted by modification time (newest first)
    while IFS= read -r file; do
        local basename=$(basename "$file")
        local mod_time=$(stat -f "%Sm" -t "%Y-%m-%d %H:%M" "$file" 2>/dev/null || stat -c "%y" "$file" 2>/dev/null | cut -d'.' -f1)
        local size=$(du -h "$file" 2>/dev/null | cut -f1)

        if [[ "$file" == *"$active_log"* ]] || [[ "$active_log" == *"$basename"* ]]; then
            echo -e "  ${GREEN}▶${NC} ${BOLD}$basename${NC} ${GREEN}(ACTIVE)${NC}"
        else
            echo -e "    $basename"
        fi
        echo -e "      ${CYAN}Modified:${NC} $mod_time  ${CYAN}Size:${NC} $size"
    done < <(find "$LOGS_DIR" -name "*.md" -type f -exec ls -t {} + 2>/dev/null)

    echo ""
}

get_active_log() {
    if [[ -f "$CURRENT_LOG_PTR" ]]; then
        local ptr=$(cat "$CURRENT_LOG_PTR" 2>/dev/null || echo "")
        if [[ -n "$ptr" ]]; then
            # Handle both relative and absolute paths
            if [[ "$ptr" == /* ]]; then
                echo "$ptr"
            else
                echo "$CODE_AGENT_DIR/$ptr"
            fi
            return 0
        fi
    fi
    return 1
}

get_latest_log() {
    find "$LOGS_DIR" -name "*.md" -type f -exec ls -t {} + 2>/dev/null | head -1
}

tail_log() {
    local log_file="$1"
    local basename=$(basename "$log_file")

    print_header
    echo -e "${BOLD}Tailing:${NC} ${CYAN}$basename${NC}"
    echo -e "${BOLD}Full Path:${NC} $log_file"
    echo -e "${YELLOW}Press Ctrl+C to stop${NC}"
    echo ""
    echo -e "${BLUE}────────────────────────────────────────────────────────────────${NC}"
    echo ""

    # Use tail -f with some context
    tail -n 50 -f "$log_file" 2>/dev/null || {
        echo -e "${RED}Error: Cannot tail file: $log_file${NC}"
        exit 1
    }
}

tail_all_logs() {
    print_header
    echo -e "${BOLD}Tailing all logs (newest activity first)${NC}"
    echo -e "${YELLOW}Press Ctrl+C to stop${NC}"
    echo ""
    echo -e "${BLUE}────────────────────────────────────────────────────────────────${NC}"
    echo ""

    # Use tail -f on all markdown log files
    local files=$(find "$LOGS_DIR" -name "*.md" -type f 2>/dev/null)
    if [[ -z "$files" ]]; then
        echo -e "${RED}No log files found in $LOGS_DIR${NC}"
        exit 1
    fi

    # tail -f with file headers
    tail -n 20 -f $files 2>/dev/null || {
        echo -e "${RED}Error tailing log files${NC}"
        exit 1
    }
}

# Main
main() {
    # Check if logs directory exists
    if [[ ! -d "$LOGS_DIR" ]]; then
        echo -e "${RED}Error: Code agent logs directory not found${NC}"
        echo -e "Expected: $LOGS_DIR"
        echo ""
        echo -e "${YELLOW}Make sure CODE_AGENT_DIR is set correctly or the code agent has started logging.${NC}"
        exit 1
    fi

    case "${1:-}" in
        --help|-h)
            print_header
            print_usage
            ;;
        --list|-l)
            list_logs
            ;;
        --all|-a)
            tail_all_logs
            ;;
        --latest)
            local latest=$(get_latest_log)
            if [[ -z "$latest" ]]; then
                echo -e "${RED}No log files found in $LOGS_DIR${NC}"
                exit 1
            fi
            tail_log "$latest"
            ;;
        "")
            # Default: tail active log or latest
            local active=$(get_active_log || echo "")
            if [[ -n "$active" ]] && [[ -f "$active" ]]; then
                echo -e "${GREEN}Found active log session${NC}"
                tail_log "$active"
            else
                echo -e "${YELLOW}No active log session found, using latest log${NC}"
                local latest=$(get_latest_log)
                if [[ -z "$latest" ]]; then
                    echo -e "${RED}No log files found in $LOGS_DIR${NC}"
                    exit 1
                fi
                tail_log "$latest"
            fi
            ;;
        *)
            # Specific file
            local file="$1"
            if [[ ! -f "$file" ]]; then
                # Try in logs directory
                file="$LOGS_DIR/$1"
            fi
            if [[ ! -f "$file" ]]; then
                echo -e "${RED}Error: Log file not found: $1${NC}"
                list_logs
                exit 1
            fi
            tail_log "$file"
            ;;
    esac
}

main "$@"
