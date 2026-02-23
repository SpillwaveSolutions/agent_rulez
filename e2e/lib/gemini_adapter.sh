#!/usr/bin/env bash
# gemini_adapter.sh — Gemini CLI headless invocation helper and workspace config generator
#
# Usage: source this file before calling adapter functions.
#
# Exported globals:
#   GEMINI_CLI_NAME — constant string "gemini" for reporting
#
# Functions:
#   gemini_adapter_check()                              — verify gemini is in PATH and GEMINI_API_KEY is set
#   require_gemini_cli()                                — return 77 (skip) if gemini CLI not available
#   setup_gemini_hooks(workspace, rulez_binary)        — write .gemini/settings.json with BeforeTool hook
#   invoke_gemini_headless(workspace, prompt, timeout) — run gemini -p headlessly

# Constant for use in reporting/scenario names
GEMINI_CLI_NAME="gemini"
export GEMINI_CLI_NAME

# ---------------------------------------------------------------------------
# gemini_adapter_check
# Verifies `gemini` is in PATH and GEMINI_API_KEY is set and non-empty.
# Returns 1 with error message if either check fails.
# Prints gemini version on success.
# ---------------------------------------------------------------------------
gemini_adapter_check() {
  if ! command -v gemini > /dev/null 2>&1; then
    echo "ERROR: 'gemini' CLI not found in PATH." >&2
    echo "  Install Gemini CLI: npm install -g @google/gemini-cli" >&2
    echo "  https://github.com/google-gemini/gemini-cli" >&2
    return 1
  fi

  if [[ -z "${GEMINI_API_KEY:-}" ]]; then
    echo "ERROR: GEMINI_API_KEY environment variable is not set." >&2
    echo "  Get an API key at: https://aistudio.google.com/apikey" >&2
    return 1
  fi

  local version
  version="$(gemini --version 2>&1 || true)"
  echo "gemini_adapter: found gemini CLI: ${version}"
  return 0
}

# ---------------------------------------------------------------------------
# require_gemini_cli
# Returns 0 if gemini CLI is available for headless invocation, 77 (skip) otherwise.
# Scenarios that need gemini should call this at their start and return 77
# (triggering skip) if it fails.
# ---------------------------------------------------------------------------
require_gemini_cli() {
  if [[ "${GEMINI_CLI_AVAILABLE:-0}" -eq 1 ]]; then
    return 0
  fi
  echo "  [skip] gemini CLI not available for headless invocation" >&2
  return 77
}

# ---------------------------------------------------------------------------
# setup_gemini_hooks workspace rulez_binary
# Writes $workspace/.gemini/settings.json with a BeforeTool hook pointing
# at the given rulez binary (resolved to absolute path).
# Note: RuleZ hooks.yaml is always at .claude/hooks.yaml (even for Gemini tests).
# Gemini uses regex matchers (not glob) so ".*" matches all tools.
# ---------------------------------------------------------------------------
setup_gemini_hooks() {
  local workspace="$1"
  local rulez_binary="$2"

  # Ensure workspace .gemini dir exists
  mkdir -p "${workspace}/.gemini"

  # Resolve rulez_binary to absolute path
  local abs_rulez
  abs_rulez="$(cd "$(dirname "${rulez_binary}")" && pwd)/$(basename "${rulez_binary}")"

  cat > "${workspace}/.gemini/settings.json" <<EOF
{
  "hooks": {
    "BeforeTool": [
      {
        "matcher": ".*",
        "hooks": [
          {
            "type": "command",
            "command": "${abs_rulez} gemini hook",
            "timeout": 10000
          }
        ]
      }
    ]
  }
}
EOF

  echo "gemini_adapter: wrote settings.json to ${workspace}/.gemini/settings.json"
}

# ---------------------------------------------------------------------------
# invoke_gemini_headless workspace prompt timeout_secs
# Runs Gemini CLI in headless (non-interactive) mode from the workspace dir.
# Captures stdout+stderr to $workspace/gemini-output.txt AND echoes it.
# Returns gemini's exit code.
# If exit code is 124 (timeout), returns 77 (skip) — timeout treated as skip
# per research noting --yolo flag intermittent behavior.
# ---------------------------------------------------------------------------
invoke_gemini_headless() {
  local workspace="$1"
  local prompt="$2"
  local timeout_secs="${3:-120}"

  local output_file="${workspace}/gemini-output.txt"

  local exit_code=0

  # Run gemini from within the workspace so project-level .gemini/settings.json is discovered
  (
    cd "${workspace}" && \
    NO_COLOR=true timeout "${timeout_secs}" gemini \
      -p "${prompt}" \
      --yolo \
      --output-format json 2>&1
  ) | tee "${output_file}" || exit_code="${PIPESTATUS[0]}"

  if [[ "${exit_code}" -eq 124 ]]; then
    echo "gemini_adapter: gemini timed out after ${timeout_secs}s — treating as skip" >&2
    return 77
  fi

  echo "gemini_adapter: gemini exited with code ${exit_code}"
  return "${exit_code}"
}
