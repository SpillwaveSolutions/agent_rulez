---
created: 2026-03-05T21:28:15.168Z
title: Fix skill reference docs using hooks instead of rules
area: docs
github_issue: https://github.com/SpillwaveSolutions/agent_rulez/issues/103
files:
  - mastering-hooks/references/hooks-yaml-schema.md:14-16
  - mastering-hooks/references/rule-patterns.md
---

## Problem

The mastering-hooks skill's reference documentation uses `hooks:` as the top-level YAML key in examples, but the actual RuleZ binary expects `rules:`. This causes users following the skill's guidance to create invalid configuration files that fail at runtime.

Affected files:
- `mastering-hooks/references/hooks-yaml-schema.md` (lines 14-16) — shows `hooks: []` as top-level key
- `mastering-hooks/references/rule-patterns.md` — uses `hooks:` in all examples

## Solution

Search all files under `mastering-hooks/references/` and `mastering-hooks/assets/` for `hooks:` used as the top-level YAML key and replace with `rules:`. Ensure the schema doc, rule patterns, and any template files are all consistent with the binary's expected format.
