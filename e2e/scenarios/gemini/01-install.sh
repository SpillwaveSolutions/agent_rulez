#!/usr/bin/env bash
# 01-install.sh — E2E scenario: rulez gemini install produces a valid .gemini/settings.json
#
# Scenario function: scenario_install(workspace, rulez_binary)
# Does NOT invoke Gemini CLI — validates structural config only.

# Source Gemini adapter for any shared helpers (GEMINI_CLI_NAME etc.)
# shellcheck source=../../lib/gemini_adapter.sh
source "${E2E_ROOT}/lib/gemini_adapter.sh"

# scenario_install workspace rulez_binary
# Runs `rulez gemini install` in the workspace and asserts the resulting settings.json structure.
# Returns 0 if all assertions pass, 1 if any fail.
scenario_install() {
  local workspace="$1"
  local rulez_binary="$2"

  local failures=0

  # Run rulez gemini install with --scope project and --binary flag pointing at itself
  local install_output
  install_output="$(cd "${workspace}" && "${rulez_binary}" gemini install --scope project --binary "${rulez_binary}" 2>&1)"
  local install_exit=$?

  echo "  [install] output: ${install_output}"

  # Assert rulez gemini install exits 0
  assert_exit_code "${install_exit}" 0 "rulez gemini install exits 0" || failures=$((failures + 1))

  # Assert settings.json was created
  assert_file_exists "${workspace}/.gemini/settings.json" "settings.json created" || failures=$((failures + 1))

  # Assert settings.json contains BeforeTool hook entry (Gemini uses BeforeTool, not PreToolUse)
  assert_file_contains "${workspace}/.gemini/settings.json" '"BeforeTool"' \
    "settings.json contains BeforeTool hook" || failures=$((failures + 1))

  # Assert settings.json contains command entry
  assert_file_contains "${workspace}/.gemini/settings.json" '"command"' \
    "settings.json contains command entry" || failures=$((failures + 1))

  if [[ "${failures}" -eq 0 ]]; then
    return 0
  else
    return 1
  fi
}
