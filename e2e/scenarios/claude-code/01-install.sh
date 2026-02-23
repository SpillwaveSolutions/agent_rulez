#!/usr/bin/env bash
# 01-install.sh — E2E scenario: rulez install produces a valid .claude/settings.json
#
# Scenario function: scenario_install(workspace, rulez_binary)
# Does NOT invoke Claude CLI — validates structural config only.

# Source Claude adapter for any shared helpers (CLAUDE_CLI_NAME etc.)
# shellcheck source=../../lib/claude_adapter.sh
source "${E2E_ROOT}/lib/claude_adapter.sh"

# scenario_install workspace rulez_binary
# Runs `rulez install` in the workspace and asserts the resulting settings.json structure.
# Returns 0 if all assertions pass, 1 if any fail.
scenario_install() {
  local workspace="$1"
  local rulez_binary="$2"

  local failures=0

  # Run rulez install with --binary flag pointing at itself
  local install_output
  install_output="$(cd "${workspace}" && "${rulez_binary}" install --binary "${rulez_binary}" 2>&1)"
  local install_exit=$?

  echo "  [install] output: ${install_output}"

  # Assert rulez install exits 0
  assert_exit_code "${install_exit}" 0 "rulez install exits 0" || failures=$((failures + 1))

  # Assert settings.json was created
  assert_file_exists "${workspace}/.claude/settings.json" "settings.json created" || failures=$((failures + 1))

  # Assert settings.json contains PreToolUse hook entry
  assert_file_contains "${workspace}/.claude/settings.json" '"PreToolUse"' \
    "settings.json contains PreToolUse hook" || failures=$((failures + 1))

  # Assert settings.json contains command entry
  assert_file_contains "${workspace}/.claude/settings.json" '"command"' \
    "settings.json contains command entry" || failures=$((failures + 1))

  if [[ "${failures}" -eq 0 ]]; then
    return 0
  else
    return 1
  fi
}
