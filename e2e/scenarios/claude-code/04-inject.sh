#!/usr/bin/env bash
# 04-inject.sh — E2E scenario: inject rule executes inject_command writing a marker file
#
# Scenario function: scenario_inject(workspace, rulez_binary)
# Invokes claude -p headlessly; verifies inject_command created the marker file
# and audit log contains the inject rule name.

# Source Claude adapter for invoke_claude_headless, setup_claude_hooks
# shellcheck source=../../lib/claude_adapter.sh
source "${E2E_ROOT}/lib/claude_adapter.sh"

# scenario_inject workspace rulez_binary
# Sets up workspace with inject template (substituting __WORKSPACE__), invokes claude
# headlessly, asserts marker file exists and audit log contains inject rule name.
# Returns 0 if all assertions pass, 1 if any fail.
scenario_inject() {
  local workspace="$1"
  local rulez_binary="$2"

  # This scenario requires a live claude CLI (returns 77 = skip)
  require_claude_cli || return $?

  local failures=0

  # Write .claude/settings.json with PreToolUse hook pointing at rulez
  setup_claude_hooks "${workspace}" "${rulez_binary}"

  # Generate hooks.yaml from template: replace __WORKSPACE__ with absolute workspace path
  sed "s|__WORKSPACE__|${workspace}|g" \
    "${E2E_ROOT}/fixtures/claude-code/hooks-inject.yaml.template" \
    > "${workspace}/.claude/hooks.yaml"

  # Snapshot the log before invocation
  local log_file="${HOME}/.claude/logs/rulez.log"
  if [[ -f "${log_file}" ]]; then
    WORKSPACE_LOG_SNAPSHOT="$(wc -l < "${log_file}")"
  else
    WORKSPACE_LOG_SNAPSHOT=0
  fi
  export WORKSPACE_LOG_SNAPSHOT

  # Invoke Claude headlessly to trigger the inject rule
  invoke_claude_headless "${workspace}" "Run this bash command: echo hello-e2e-inject" 120 || true

  # Assert marker file was created by inject_command
  assert_file_exists "${workspace}/e2e-inject-fired.marker" \
    "inject marker file created" || failures=$((failures + 1))

  # Assert audit log contains inject rule name (proof rule was evaluated)
  assert_log_contains "e2e-inject-marker" \
    "audit log contains inject rule name" || failures=$((failures + 1))

  if [[ "${failures}" -eq 0 ]]; then
    return 0
  else
    return 1
  fi
}
