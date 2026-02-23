#!/usr/bin/env bash
# 03-deny.sh — E2E scenario: deny rule blocks a git force push tool call
#
# Scenario function: scenario_deny(workspace, rulez_binary)
# Invokes gemini -p headlessly with a force push prompt; verifies via audit log.
#
# Note: Gemini's exit code may be 0 even when a hook denies, because Gemini handles
# hook denials internally. The proof of denial is in the audit log, not Gemini's exit code.

# Source Gemini adapter for invoke_gemini_headless, setup_gemini_hooks
# shellcheck source=../../lib/gemini_adapter.sh
source "${E2E_ROOT}/lib/gemini_adapter.sh"

# scenario_deny workspace rulez_binary
# Sets up workspace with deny fixture, invokes gemini headlessly with force push prompt,
# asserts audit log contains deny rule name and block action.
# Returns 0 if all assertions pass, 1 if any fail.
scenario_deny() {
  local workspace="$1"
  local rulez_binary="$2"

  # This scenario requires a live gemini CLI (returns 77 = skip)
  require_gemini_cli || return $?

  local failures=0

  # Write .gemini/settings.json with BeforeTool hook pointing at rulez
  setup_gemini_hooks "${workspace}" "${rulez_binary}"

  # Copy deny fixture into workspace as hooks.yaml
  # NOTE: RuleZ config (hooks.yaml) always lives at .claude/hooks.yaml even for gemini tests.
  # The .gemini/settings.json tells gemini CLI to call `rulez gemini hook`,
  # and rulez reads its policy config from .claude/hooks.yaml.
  mkdir -p "${workspace}/.claude"
  cp "${E2E_ROOT}/fixtures/gemini/hooks-deny.yaml" "${workspace}/.claude/hooks.yaml"

  # Snapshot the log before invocation
  local log_file="${HOME}/.claude/logs/rulez.log"
  if [[ -f "${log_file}" ]]; then
    WORKSPACE_LOG_SNAPSHOT="$(wc -l < "${log_file}")"
  else
    WORKSPACE_LOG_SNAPSHOT=0
  fi
  export WORKSPACE_LOG_SNAPSHOT

  # Invoke Gemini headlessly — don't fail on non-zero exit (deny IS the expected outcome)
  invoke_gemini_headless "${workspace}" \
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
