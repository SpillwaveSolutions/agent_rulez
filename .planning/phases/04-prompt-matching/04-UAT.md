---
status: testing
phase: 04-prompt-matching
source: [04-01-SUMMARY.md, 04-02-SUMMARY.md, 04-03-SUMMARY.md, 04-04-SUMMARY.md]
started: 2026-02-09T22:00:00Z
updated: 2026-02-09T22:05:00Z
---

## Current Test

number: 6
name: Contains Word Shorthand
expected: |
  Create a rule with `prompt_match: ["contains_word:delete"]`.
  A prompt with "delete" as a word should match, but "deleting" should NOT match (word boundary).
awaiting: user response

## Tests

### 1. Simple Array Prompt Matching
expected: Create a rule with `prompt_match: ["delete", "drop"]` and run debug with a prompt containing "delete all files". The rule should match and block/inject as configured.
result: pass
notes: Verified via piped event JSON - exits code 2 with "Blocked by rule 'block-destructive'"

### 2. Complex Object Prompt Matching
expected: Create a rule with `prompt_match: { patterns: ["test", "staging"], mode: all }` requiring BOTH patterns. A prompt with only "test" should NOT match; a prompt with both "test" and "staging" should match.
result: pass

### 3. Case-Insensitive Matching
expected: Create a rule with `prompt_match: { patterns: ["DELETE"], case_insensitive: true }`. A prompt containing "delete" (lowercase) should match.
result: pass

### 4. Anchor: Start Position
expected: Create a rule with `prompt_match: { patterns: ["start"], anchor: start }`. A prompt starting with "start here" should match, but "not start here" should NOT match.
result: pass

### 5. Anchor: End Position
expected: Create a rule with `prompt_match: { patterns: ["done"], anchor: end }`. A prompt ending with "all done" should match, but "done already" should NOT match.
result: pass

### 6. Contains Word Shorthand
expected: Create a rule with `prompt_match: ["contains_word:delete"]`. A prompt with "delete" as a word should match, but "deleting" should NOT match (word boundary).
result: [pending]

### 7. Negation Pattern
expected: Create a rule with `prompt_match: ["not:production"]`. A prompt WITHOUT "production" should match; a prompt WITH "production" should NOT match.
result: [pending]

### 8. Prompt Variable in evalexpr
expected: Create a rule with `enabled_when: 'prompt == "enable me"'`. Only when the prompt text is exactly "enable me" should the rule activate.
result: [pending]

### 9. Config Validation: Invalid Regex Rejected
expected: Run `rulez validate` on a config with `prompt_match: ["[invalid(regex"]`. The validation should fail with a clear error message mentioning the invalid pattern.
result: [pending]

### 10. Config Validation: Empty Patterns Rejected
expected: Run `rulez validate` on a config with `prompt_match: []`. The validation should fail with an error about empty patterns.
result: [pending]

## Summary

total: 10
passed: 5
issues: 0
pending: 5
skipped: 0

## Gaps

[none yet]
