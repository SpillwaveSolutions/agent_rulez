#!/usr/bin/env bash
# 01-install.sh — E2E scenario: Codex adapter detection and workspace setup
#
# Scenario function: scenario_install(workspace, rulez_binary)
# Does NOT invoke Codex CLI — validates adapter detection and workspace config only.
# Does NOT call `rulez codex install` (no such subcommand exists).

# Source Codex adapter for shared helpers (CODEX_CLI_NAME etc.)
# shellcheck source=../../lib/codex_adapter.sh
source "${E2E_ROOT}/lib/codex_adapter.sh"

# scenario_install workspace rulez_binary
# Checks codex adapter detection and runs setup_codex_hooks to create workspace config.
# Asserts .codex/config.toml exists and contains approval_policy.
# Returns 0 if all assertions pass, 1 if any fail.
scenario_install() {
  local workspace="$1"
  local rulez_binary="$2"

  local failures=0

  # Check if codex adapter can detect the CLI
  local check_output
  check_output="$(codex_adapter_check 2>&1)"
  local check_exit=$?

  echo "  [install] adapter check: ${check_output}"

  if [[ "${check_exit}" -ne 0 ]]; then
    echo "  [install] codex CLI not found — skipping install scenario" >&2
    return 77
  fi

  # Run setup_codex_hooks to create workspace config
  setup_codex_hooks "${workspace}" "${rulez_binary}"

  # Assert .codex/config.toml was created
  assert_file_exists "${workspace}/.codex/config.toml" ".codex/config.toml created" || failures=$((failures + 1))

  # Assert config.toml contains approval_policy
  assert_file_contains "${workspace}/.codex/config.toml" 'approval_policy' \
    "config.toml contains approval_policy" || failures=$((failures + 1))

  if [[ "${failures}" -eq 0 ]]; then
    return 0
  else
    return 1
  fi
}
