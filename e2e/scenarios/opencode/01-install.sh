#!/usr/bin/env bash
# 01-install.sh — E2E scenario: rulez opencode install produces a valid .opencode/settings.json
#
# Scenario function: scenario_install(workspace, rulez_binary)
# Does NOT invoke OpenCode CLI — validates structural config only.

# Source OpenCode adapter for any shared helpers (OPENCODE_CLI_NAME etc.)
# shellcheck source=../../lib/opencode_adapter.sh
source "${E2E_ROOT}/lib/opencode_adapter.sh"

# scenario_install workspace rulez_binary
# Runs `rulez opencode install` in the workspace and asserts the resulting settings.json structure.
# Returns 0 if all assertions pass, 1 if any fail.
scenario_install() {
  local workspace="$1"
  local rulez_binary="$2"

  local failures=0

  # Run rulez opencode install with --binary flag and --scope project
  local install_output
  install_output="$(cd "${workspace}" && "${rulez_binary}" opencode install --scope project --binary "${rulez_binary}" 2>&1)"
  local install_exit=$?

  echo "  [install] output: ${install_output}"

  # Assert rulez opencode install exits 0
  assert_exit_code "${install_exit}" 0 "rulez opencode install exits 0" || failures=$((failures + 1))

  # Assert settings.json was created at .opencode/settings.json
  assert_file_exists "${workspace}/.opencode/settings.json" ".opencode/settings.json created" || failures=$((failures + 1))

  # Assert settings.json contains tool.execute.before hook entry
  assert_file_contains "${workspace}/.opencode/settings.json" '"tool.execute.before"' \
    "settings.json contains tool.execute.before hook" || failures=$((failures + 1))

  # Assert settings.json contains opencode hook command string
  assert_file_contains "${workspace}/.opencode/settings.json" 'opencode hook' \
    "settings.json contains opencode hook entry" || failures=$((failures + 1))

  if [[ "${failures}" -eq 0 ]]; then
    return 0
  else
    return 1
  fi
}
