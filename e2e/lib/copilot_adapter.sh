#!/usr/bin/env bash
# copilot_adapter.sh — Copilot CLI headless invocation helper and workspace config generator
#
# Usage: source this file before calling adapter functions.
#
# Exported globals:
#   COPILOT_CLI_NAME — constant string "copilot" for reporting
#
# Functions:
#   copilot_adapter_check()                              — verify copilot is in PATH (OAuth login, no API key needed)
#   require_copilot_cli()                                — return 77 (skip) if copilot CLI not available
#   setup_copilot_hooks(workspace, rulez_binary)        — write .github/hooks/rulez.json with preToolUse hook
#   invoke_copilot_headless(workspace, prompt, timeout) — run copilot -p headlessly

# Constant for use in reporting/scenario names
COPILOT_CLI_NAME="copilot"
export COPILOT_CLI_NAME

# ---------------------------------------------------------------------------
# copilot_adapter_check
# Verifies `copilot` is in PATH. Copilot uses OAuth login — no API key check.
# Returns 1 with error message if not found.
# Prints copilot version on success.
# ---------------------------------------------------------------------------
copilot_adapter_check() {
  if ! command -v copilot > /dev/null 2>&1; then
    echo "ERROR: 'copilot' CLI not found in PATH." >&2
    echo "  Install GitHub Copilot CLI: npm install -g @githubnext/github-copilot-cli" >&2
    echo "  https://github.com/githubnext/github-copilot-cli" >&2
    return 1
  fi

  local version
  version="$(copilot --version 2>&1 || true)"
  echo "copilot_adapter: found copilot CLI: ${version}"
  return 0
}

# ---------------------------------------------------------------------------
# require_copilot_cli
# Returns 0 if copilot CLI is available for headless invocation, 77 (skip) otherwise.
# Scenarios that need copilot should call this at their start and return 77
# (triggering skip) if it fails.
# ---------------------------------------------------------------------------
require_copilot_cli() {
  if [[ "${COPILOT_CLI_AVAILABLE:-0}" -eq 1 ]]; then
    return 0
  fi
  echo "  [skip] copilot CLI not available for headless invocation" >&2
  return 77
}

# ---------------------------------------------------------------------------
# setup_copilot_hooks workspace rulez_binary
# Writes $workspace/.github/hooks/rulez.json with a preToolUse hook pointing
# at the given rulez binary (resolved to absolute path).
# Note: RuleZ hooks.yaml is always at .claude/hooks.yaml (even for Copilot tests).
# Copilot uses preToolUse (not BeforeTool), bash/powershell fields (not command),
# timeoutSec in seconds (not timeout in ms), and has version: 1 at top level.
# ---------------------------------------------------------------------------
setup_copilot_hooks() {
  local workspace="$1"
  local rulez_binary="$2"

  # Ensure workspace .github/hooks dir exists
  mkdir -p "${workspace}/.github/hooks"

  # Resolve rulez_binary to absolute path
  local abs_rulez
  abs_rulez="$(cd "$(dirname "${rulez_binary}")" && pwd)/$(basename "${rulez_binary}")"

  cat > "${workspace}/.github/hooks/rulez.json" <<EOF
{
  "version": 1,
  "hooks": {
    "preToolUse": [
      {
        "type": "command",
        "bash": "${abs_rulez} copilot hook",
        "powershell": "${abs_rulez} copilot hook",
        "timeoutSec": 10
      }
    ]
  }
}
EOF

  echo "copilot_adapter: wrote rulez.json to ${workspace}/.github/hooks/rulez.json"
}

# ---------------------------------------------------------------------------
# invoke_copilot_headless workspace prompt timeout_secs
# Runs Copilot CLI in headless (non-interactive) mode from the workspace dir.
# Captures stdout+stderr to $workspace/copilot-output.txt AND echoes it.
# Returns copilot's exit code.
# If exit code is 124 (timeout), returns 77 (skip) — timeout treated as skip.
# ---------------------------------------------------------------------------
invoke_copilot_headless() {
  local workspace="$1"
  local prompt="$2"
  local timeout_secs="${3:-120}"

  local output_file="${workspace}/copilot-output.txt"

  local exit_code=0

  # Run copilot from within the workspace so project-level .github/hooks/rulez.json is discovered
  (
    cd "${workspace}" && \
    NO_COLOR=true timeout "${timeout_secs}" copilot \
      -p "${prompt}" \
      --allow-all-tools 2>&1
  ) | tee "${output_file}" || exit_code="${PIPESTATUS[0]}"

  if [[ "${exit_code}" -eq 124 ]]; then
    echo "copilot_adapter: copilot timed out after ${timeout_secs}s — treating as skip" >&2
    return 77
  fi

  echo "copilot_adapter: copilot exited with code ${exit_code}"
  return "${exit_code}"
}
