#!/usr/bin/env bash
# reporting.sh — E2E test result tracking, JUnit XML generation, ASCII table, Markdown summary
#
# Usage: source this file after harness.sh, then call reporting_init() before running scenarios.
#
# Exported globals (set by reporting_init):
#   JUNIT_CASES_FILE — temp file accumulating JUnit XML testcase elements
#   JUNIT_TESTS      — total test count
#   JUNIT_FAILURES   — failure count
#   JUNIT_SKIPS      — skip count
#   RESULTS_FILE     — temp file storing results as "cli:scenario:status" per line
#
# Compatible with bash 3.2+ (macOS system bash).

set -euo pipefail

# ---------------------------------------------------------------------------
# Result Tracking
# ---------------------------------------------------------------------------

# reporting_init — initialize counters, temp file, and results file
reporting_init() {
  JUNIT_CASES_FILE="$(mktemp)"
  export JUNIT_CASES_FILE

  JUNIT_TESTS=0
  JUNIT_FAILURES=0
  JUNIT_SKIPS=0
  export JUNIT_TESTS JUNIT_FAILURES JUNIT_SKIPS

  RESULTS_FILE="$(mktemp)"
  export RESULTS_FILE
}

# _get_result cli scenario — lookup result from RESULTS_FILE, return "????" if not found
_get_result() {
  local cli="$1"
  local scenario="$2"
  local line
  line=$(grep "^${cli}:${scenario}:" "${RESULTS_FILE}" 2>/dev/null | tail -1) || true
  if [[ -n "${line}" ]]; then
    echo "${line##*:}"
  else
    echo "????"
  fi
}

# record_result cli_name scenario_name status elapsed_secs message
# status is "pass", "fail", or "skip"
# Increments counters, stores in RESULTS_FILE, appends JUnit XML testcase element.
record_result() {
  local cli_name="$1"
  local scenario_name="$2"
  local status="$3"
  local elapsed_secs="$4"
  local message="${5:-}"

  JUNIT_TESTS=$((JUNIT_TESTS + 1))

  # Store result in file
  echo "${cli_name}:${scenario_name}:${status}" >> "${RESULTS_FILE}"

  # Escape message for XML attribute (basic escaping)
  local safe_message
  safe_message="${message//&/&amp;}"
  safe_message="${safe_message//</&lt;}"
  safe_message="${safe_message//>/&gt;}"
  safe_message="${safe_message//\"/&quot;}"

  local classname="RuleZ.E2E.${cli_name//-/.}"
  local testname="${scenario_name}"

  case "${status}" in
    pass)
      cat >> "${JUNIT_CASES_FILE}" <<EOF
    <testcase classname="${classname}" name="${testname}" time="${elapsed_secs}"/>
EOF
      ;;
    fail)
      JUNIT_FAILURES=$((JUNIT_FAILURES + 1))
      cat >> "${JUNIT_CASES_FILE}" <<EOF
    <testcase classname="${classname}" name="${testname}" time="${elapsed_secs}">
      <failure type="E2EFailure" message="${safe_message}"><![CDATA[${message}]]></failure>
    </testcase>
EOF
      ;;
    skip)
      JUNIT_SKIPS=$((JUNIT_SKIPS + 1))
      cat >> "${JUNIT_CASES_FILE}" <<EOF
    <testcase classname="${classname}" name="${testname}" time="${elapsed_secs}">
      <skipped/>
    </testcase>
EOF
      ;;
    *)
      echo "WARNING: unknown status '${status}' for ${cli_name}:${scenario_name}" >&2
      ;;
  esac
}

# ---------------------------------------------------------------------------
# JUnit XML Output
# ---------------------------------------------------------------------------

# write_junit_xml output_path
# Wraps collected testcases in <testsuites><testsuite> and writes to output_path.
write_junit_xml() {
  local output_path="$1"

  local total_time=0

  cat > "${output_path}" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<testsuites>
  <testsuite name="RuleZ E2E" tests="${JUNIT_TESTS}" failures="${JUNIT_FAILURES}" errors="0" skipped="${JUNIT_SKIPS}" time="${total_time}">
$(cat "${JUNIT_CASES_FILE}")
  </testsuite>
</testsuites>
EOF

  rm -f "${JUNIT_CASES_FILE}"
  echo "JUnit XML written to: ${output_path}"
}

# ---------------------------------------------------------------------------
# ASCII Table
# ---------------------------------------------------------------------------

# print_results_table cli_list
# Takes a space-separated list of CLI names as arguments.
# Renders ASCII table: CLI x scenario matrix.
# Prints summary: "X passed, Y failed, Z skipped"
print_results_table() {
  local scenarios="install hook-fire deny inject"

  printf "\n"
  printf "%-22s" "CLI"
  for s in ${scenarios}; do
    printf " %-12s" "${s}"
  done
  printf "\n"

  printf "%-22s" "----------------------"
  for s in ${scenarios}; do
    printf " %-12s" "------------"
  done
  printf "\n"

  for cli in "$@"; do
    printf "%-22s" "${cli}"
    for s in ${scenarios}; do
      local result
      result="$(_get_result "${cli}" "${s}")"
      case "${result}" in
        pass) printf " %-12s" "PASS" ;;
        fail) printf " %-12s" "FAIL" ;;
        skip) printf " %-12s" "SKIP" ;;
        *)    printf " %-12s" "????" ;;
      esac
    done
    printf "\n"
  done

  printf "\n"
  printf "Summary: %d passed, %d failed, %d skipped\n" \
    "$((JUNIT_TESTS - JUNIT_FAILURES - JUNIT_SKIPS))" \
    "${JUNIT_FAILURES}" \
    "${JUNIT_SKIPS}"
  printf "\n"
}

# ---------------------------------------------------------------------------
# Markdown Summary
# ---------------------------------------------------------------------------

# write_markdown_summary output_path
# Generates Markdown table of results.
# If $GITHUB_STEP_SUMMARY is set, also appends to it.
write_markdown_summary() {
  local output_path="$1"

  local scenarios="install hook-fire deny inject"

  {
    echo "# RuleZ E2E Test Results"
    echo ""
    echo "**Run ID:** ${RUN_ID:-unknown}"
    echo ""
    printf "| %-22s" "CLI"
    for s in ${scenarios}; do
      printf " | %-12s" "${s}"
    done
    printf " |\n"

    printf "| %-22s" ":----------------------"
    for s in ${scenarios}; do
      printf " | %-12s" ":----------:"
    done
    printf " |\n"

    # Collect unique CLIs from results file
    local seen_clis=""
    while IFS=: read -r cli _scenario _status; do
      case " ${seen_clis} " in
        *" ${cli} "*) ;;  # already seen
        *) seen_clis="${seen_clis} ${cli}" ;;
      esac
    done < "${RESULTS_FILE}"

    for cli in ${seen_clis}; do
      printf "| %-22s" "${cli}"
      for s in ${scenarios}; do
        local result
        result="$(_get_result "${cli}" "${s}")"
        case "${result}" in
          pass) printf " | %-12s" "PASS" ;;
          fail) printf " | %-12s" "**FAIL**" ;;
          skip) printf " | %-12s" "SKIP" ;;
          *)    printf " | %-12s" "????" ;;
        esac
      done
      printf " |\n"
    done

    echo ""
    printf "**Summary:** %d passed, %d failed, %d skipped (total: %d)\n" \
      "$((JUNIT_TESTS - JUNIT_FAILURES - JUNIT_SKIPS))" \
      "${JUNIT_FAILURES}" \
      "${JUNIT_SKIPS}" \
      "${JUNIT_TESTS}"
  } > "${output_path}"

  # Also append to GitHub Actions step summary if available
  if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
    cat "${output_path}" >> "${GITHUB_STEP_SUMMARY}"
    echo "Markdown summary appended to GITHUB_STEP_SUMMARY"
  fi

  echo "Markdown summary written to: ${output_path}"
}
