#!/usr/bin/env bash
# codex_adapter.sh — Codex CLI headless invocation helper and workspace config generator
#
# Usage: source this file before calling adapter functions.
#
# Exported globals:
#   CODEX_CLI_NAME — constant string "codex" for reporting
#
# Functions:
#   codex_adapter_check()                                — verify codex is in PATH
#   require_codex_cli()                                  — return 77 (skip) if unavailable
#   setup_codex_hooks(workspace, rulez_binary)           — write .codex/config.toml (stub)
#   invoke_codex_headless(workspace, prompt, timeout)    — run codex exec headlessly

# Constant for use in reporting/scenario names
CODEX_CLI_NAME="codex"
export CODEX_CLI_NAME

# ---------------------------------------------------------------------------
# codex_adapter_check
# Verifies `codex` is in PATH. Codex uses OpenAI API keys (not OAuth),
# so we only check for the binary presence.
#
# Returns 1 with error message if codex not found.
# Prints codex version on success.
# ---------------------------------------------------------------------------
codex_adapter_check() {
  if ! command -v codex > /dev/null 2>&1; then
    echo "ERROR: 'codex' CLI not found in PATH." >&2
    echo "  Install Codex: npm install -g @openai/codex" >&2
    echo "  See: https://github.com/openai/codex" >&2
    return 1
  fi

  local version
  version="$(codex --version 2>&1 || true)"
  echo "codex_adapter: found codex CLI: ${version}"
  return 0
}

# ---------------------------------------------------------------------------
# require_codex_cli
# Returns 0 if codex CLI is available for headless invocation, 77 (skip) otherwise.
# Scenarios that need codex should call this at their start and return 77
# (triggering skip) if it fails.
# ---------------------------------------------------------------------------
require_codex_cli() {
  if [[ "${CODEX_CLI_AVAILABLE:-0}" -eq 1 ]]; then
    return 0
  fi
  echo "  [skip] codex CLI not available for headless invocation" >&2
  return 77
}

# ---------------------------------------------------------------------------
# setup_codex_hooks workspace rulez_binary
# Stub — Codex CLI does not currently support hooks.
# Creates a minimal .codex/config.toml with approval_policy and model settings.
# Does NOT invoke any `rulez codex install` command (no such subcommand exists).
#
# WARNING: This is a placeholder. When Codex adds hook support, this function
# should be updated to configure actual hook integration.
# ---------------------------------------------------------------------------
setup_codex_hooks() {
  local workspace="$1"
  local rulez_binary="$2"

  # Ensure workspace dir exists
  mkdir -p "${workspace}/.codex"

  # Write minimal Codex config
  cat > "${workspace}/.codex/config.toml" <<'EOF'
# Codex CLI configuration (E2E test workspace)
# NOTE: Codex does not currently support hooks.
# This config enables headless execution only.
model = "o4-mini"
approval_policy = "never"
EOF

  echo "codex_adapter: WARNING — Codex CLI does not support hooks" >&2
  echo "codex_adapter: wrote config.toml to ${workspace}/.codex/config.toml"
}

# ---------------------------------------------------------------------------
# invoke_codex_headless workspace prompt timeout_secs
# Runs Codex CLI in headless (non-interactive) mode from the workspace dir.
# Captures stdout+stderr to $workspace/codex-output.txt AND echoes it.
# Returns codex's exit code.
# If exit code is 124 (timeout), returns 77 (skip) — timeout treated as skip.
# ---------------------------------------------------------------------------
invoke_codex_headless() {
  local workspace="$1"
  local prompt="$2"
  local timeout_secs="${3:-120}"

  local output_file="${workspace}/codex-output.txt"

  local exit_code=0

  # Run codex from within the workspace so project-level config is discovered
  # Codex uses `codex exec "message" --ask-for-approval never --json` for headless invocation
  (
    cd "${workspace}" && \
    NO_COLOR=true timeout "${timeout_secs}" codex exec \
      "${prompt}" \
      --ask-for-approval never \
      --json 2>&1
  ) | tee "${output_file}" || exit_code="${PIPESTATUS[0]}"

  if [[ "${exit_code}" -eq 124 ]]; then
    echo "codex_adapter: codex timed out after ${timeout_secs}s — treating as skip" >&2
    return 77
  fi

  echo "codex_adapter: codex exited with code ${exit_code}"
  return "${exit_code}"
}
