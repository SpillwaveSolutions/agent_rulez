#!/usr/bin/env bash
# claude_adapter.sh — Claude Code CLI headless invocation helper and workspace config generator
#
# Usage: source this file before calling adapter functions.
#
# Exported globals:
#   CLAUDE_CLI_NAME — constant string "claude-code" for reporting
#
# Functions:
#   claude_adapter_check()                              — verify claude is in PATH
#   setup_claude_hooks(workspace, rulez_binary)        — write .claude/settings.json with PreToolUse hook
#   invoke_claude_headless(workspace, prompt, timeout) — run claude -p headlessly

# Constant for use in reporting/scenario names
CLAUDE_CLI_NAME="claude-code"
export CLAUDE_CLI_NAME

# ---------------------------------------------------------------------------
# claude_adapter_check
# Verifies `claude` is in PATH. Returns 1 with error message if not found.
# Prints claude version on success.
# ---------------------------------------------------------------------------
claude_adapter_check() {
  if ! command -v claude > /dev/null 2>&1; then
    echo "ERROR: 'claude' CLI not found in PATH." >&2
    echo "  Install Claude Code CLI and ensure it is in your PATH." >&2
    echo "  https://docs.anthropic.com/en/docs/claude-code" >&2
    return 1
  fi

  local version
  version="$(claude --version 2>&1 || true)"
  echo "claude_adapter: found claude CLI: ${version}"
  return 0
}

# ---------------------------------------------------------------------------
# setup_claude_hooks workspace rulez_binary
# Writes $workspace/.claude/settings.json with a PreToolUse hook pointing
# at the given rulez binary (resolved to absolute path).
# ---------------------------------------------------------------------------
setup_claude_hooks() {
  local workspace="$1"
  local rulez_binary="$2"

  # Ensure workspace .claude dir exists
  mkdir -p "${workspace}/.claude"

  # Resolve rulez_binary to absolute path
  local abs_rulez
  abs_rulez="$(cd "$(dirname "${rulez_binary}")" && pwd)/$(basename "${rulez_binary}")"

  cat > "${workspace}/.claude/settings.json" <<EOF
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "${abs_rulez}",
            "timeout": 10
          }
        ]
      }
    ]
  }
}
EOF

  echo "claude_adapter: wrote settings.json to ${workspace}/.claude/settings.json"
}

# ---------------------------------------------------------------------------
# invoke_claude_headless workspace prompt timeout_secs
# Runs Claude Code in headless (non-interactive) mode from the workspace dir.
# Captures stdout+stderr to $workspace/claude-output.txt AND echoes it.
# Returns claude's exit code.
# ---------------------------------------------------------------------------
invoke_claude_headless() {
  local workspace="$1"
  local prompt="$2"
  local timeout_secs="${3:-120}"

  local output_file="${workspace}/claude-output.txt"

  local exit_code=0

  # Run claude from within the workspace so project-level settings.json is discovered
  (
    cd "${workspace}" && \
    timeout "${timeout_secs}" claude \
      -p "${prompt}" \
      --dangerously-skip-permissions \
      --output-format json \
      --max-turns 1 \
      --allowedTools "Bash" \
      --no-session-persistence \
      --model "claude-haiku-3-5" 2>&1
  ) | tee "${output_file}" || exit_code="${PIPESTATUS[0]}"

  echo "claude_adapter: claude exited with code ${exit_code}"
  return "${exit_code}"
}
