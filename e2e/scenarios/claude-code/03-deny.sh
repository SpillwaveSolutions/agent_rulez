#!/usr/bin/env bash
# 03-deny.sh — E2E scenario: deny rule blocks a git force push tool call
#
# Scenario function: scenario_deny(workspace, rulez_binary)
# Invokes claude -p headlessly with a force push prompt; verifies via audit log.
#
# Note: Claude's exit code may be 0 even when a hook denies, because Claude handles
# hook denials internally. The proof of denial is in the audit log, not Claude's exit code.

# Source Claude adapter for invoke_claude_headless, setup_claude_hooks
# shellcheck source=../../lib/claude_adapter.sh
source "${E2E_ROOT}/lib/claude_adapter.sh"

# scenario_deny workspace rulez_binary
# Sets up workspace with deny fixture, invokes claude headlessly with force push prompt,
# asserts audit log contains deny rule name and block action.
# Returns 0 if all assertions pass, 1 if any fail.
scenario_deny() {
  local workspace="$1"
  local rulez_binary="$2"

  local failures=0

  # Write .claude/settings.json with PreToolUse hook pointing at rulez
  setup_claude_hooks "${workspace}" "${rulez_binary}"

  # Copy deny fixture into workspace as hooks.yaml
  cp "${E2E_ROOT}/fixtures/claude-code/hooks-deny.yaml" "${workspace}/.claude/hooks.yaml"

  # Snapshot the log before invocation
  local log_file="${HOME}/.claude/logs/rulez.log"
  if [[ -f "${log_file}" ]]; then
    WORKSPACE_LOG_SNAPSHOT="$(wc -l < "${log_file}")"
  else
    WORKSPACE_LOG_SNAPSHOT=0
  fi
  export WORKSPACE_LOG_SNAPSHOT

  # Invoke Claude headlessly — don't fail on non-zero exit (deny IS the expected outcome)
  invoke_claude_headless "${workspace}" \
    "Run this exact bash command: git push --force origin main" 120 || true

  # Assert audit log contains the deny rule name (proof rule was evaluated)
  assert_log_contains "e2e-deny-force-push" \
    "audit log contains deny rule name" || failures=$((failures + 1))

  # Assert audit log contains block action (proof the block was recorded)
  assert_log_contains "block" \
    "audit log contains block action" || failures=$((failures + 1))

  if [[ "${failures}" -eq 0 ]]; then
    return 0
  else
    return 1
  fi
}
