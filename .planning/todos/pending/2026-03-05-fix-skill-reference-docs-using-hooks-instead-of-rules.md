---
created: 2026-03-05T21:28:15.168Z
title: Fix mastering-hooks skill schema mismatches with RuleZ binary
area: docs
github_issue:
  - https://github.com/SpillwaveSolutions/agent_rulez/issues/103
  - https://github.com/SpillwaveSolutions/agent_rulez/issues/105
files:
  - mastering-hooks/references/hooks-yaml-schema.md
  - mastering-hooks/references/rule-patterns.md
  - mastering-hooks/assets/
---

## Problem

The mastering-hooks skill's reference documentation uses **multiple wrong field names** that don't match the RuleZ binary's actual YAML schema. Users following the skill's guidance create invalid configuration files.

Field name mismatches found:

| Skill docs (wrong) | RuleZ binary (correct) |
|---------------------|----------------------|
| `hooks:` (top-level) | `rules:` |
| `match:` | `matchers:` |
| `action:` | `actions:` |
| `type: inject` / `source: file` / `path:` | `inject: <path>` |
| `type: block` / `reason:` | `block: true` |
| `priority:` (top-level) | `metadata: { priority: }` |

Affected files:
- `mastering-hooks/references/hooks-yaml-schema.md` — schema definition uses wrong keys
- `mastering-hooks/references/rule-patterns.md` — all examples use wrong keys

## Solution

Audit all files under `mastering-hooks/references/` and `mastering-hooks/assets/` for each mismatch above. Update every YAML example and schema definition to match the actual binary format. Validate corrections against `rulez validate` to confirm.
