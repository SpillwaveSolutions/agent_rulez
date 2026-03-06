#!/usr/bin/env bash
# 01-install.sh — E2E scenario: rulez copilot install produces a valid .github/hooks/rulez.json
#
# Scenario function: scenario_install(workspace, rulez_binary)
# Does NOT invoke Copilot CLI — validates structural config only.

# Source Copilot adapter for any shared helpers (COPILOT_CLI_NAME etc.)
# shellcheck source=../../lib/copilot_adapter.sh
source "${E2E_ROOT}/lib/copilot_adapter.sh"

# scenario_install workspace rulez_binary
# Runs `rulez copilot install` in the workspace and asserts the resulting rulez.json structure.
# Returns 0 if all assertions pass, 1 if any fail.
scenario_install() {
  local workspace="$1"
  local rulez_binary="$2"

  local failures=0

  # Run rulez copilot install with --binary flag pointing at itself
  # Note: Copilot install has no --scope flag (unlike Gemini which uses --scope project)
  local install_output
  install_output="$(cd "${workspace}" && "${rulez_binary}" copilot install --binary "${rulez_binary}" 2>&1)"
  local install_exit=$?

  echo "  [install] output: ${install_output}"

  # Assert rulez copilot install exits 0
  assert_exit_code "${install_exit}" 0 "rulez copilot install exits 0" || failures=$((failures + 1))

  # Assert rulez.json was created at .github/hooks/rulez.json
  assert_file_exists "${workspace}/.github/hooks/rulez.json" ".github/hooks/rulez.json created" || failures=$((failures + 1))

  # Assert rulez.json contains preToolUse hook entry (Copilot uses preToolUse, not BeforeTool)
  assert_file_contains "${workspace}/.github/hooks/rulez.json" '"preToolUse"' \
    "rulez.json contains preToolUse hook" || failures=$((failures + 1))

  # Assert rulez.json contains copilot hook command entry
  # The bash/powershell values are absolute paths ending in "copilot hook", e.g.
  # "/path/to/rulez copilot hook" — match without surrounding quotes since no literal
  # JSON string value is exactly "copilot hook" (it has a path prefix).
  assert_file_contains "${workspace}/.github/hooks/rulez.json" 'copilot hook' \
    "rulez.json contains copilot hook entry" || failures=$((failures + 1))

  if [[ "${failures}" -eq 0 ]]; then
    return 0
  else
    return 1
  fi
}
