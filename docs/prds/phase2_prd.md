# Phase 2 Governance

Document: CCH Policy Governance Addendum
Version: 1.1
Status: Proposed (Backward Compatible)

⸻

CCH Policy Governance Addendum

Subtitle: Provenance, Priority, and Policy Modes

1. Purpose

This addendum introduces a policy governance layer to the Claude Context Hooks (CCH) system, enhancing:
	•	Explainability
	•	Auditability
	•	Gradual rollout
	•	Enterprise readiness
	•	Long-term maintainability

The goal is to evolve CCH from:

“A powerful local hook system”
into:
“A deterministic, auditable AI policy engine.”

This addendum is fully backward compatible with existing configurations.

⸻

2. New Concepts Introduced

2.1 Rule Metadata (Provenance)

Each rule MAY include a metadata block that captures provenance and intent.

metadata:
  author: "cch-skill"
  created_by: "aws-cdk-skill@1.2.0"
  reason: "Enforce infrastructure coding standards"
  confidence: high | medium | low
  last_reviewed: 2025-01-21
  ticket: "PLAT-3421"
  tags: [security, infra, compliance]

Semantics

Field	Meaning
author	Human or system that authored the rule
created_by	Skill, template, or tool
reason	Why this rule exists
confidence	How confident the recommender was
last_reviewed	Governance review date
ticket	Optional tracking ID
tags	Classification for search/filter

Runtime Behavior
	•	Ignored by matcher engine.
	•	Included in logs and debug output.
	•	Exposed via cch explain rule.

⸻

3. Policy Modes

Rules MAY specify an execution mode:

mode: enforce | warn | audit

3.1 Mode Semantics

Mode	Behavior
enforce (default)	Normal blocking behavior
warn	Never blocks, injects warning context
audit	No injection, no blocking, logs only

3.2 Use Cases

Scenario	Mode
Rollout new guardrails	audit
Soft cultural rules	warn
Security policy	enforce
Observability only	audit

This enables policy dry-runs and gradual enforcement.

⸻

4. Rule Priority

Rules MAY define a priority:

priority: 100

4.1 Priority Semantics
	•	Higher numbers run first.
	•	Default priority = 0.
	•	Rules sorted by:
	1.	priority (desc)
	2.	file order (stable)

4.2 Why This Matters

Prevents emergent policy bugs like:

“Why did my block rule never fire?”
“Why did inject happen after block?”

This gives explicit control over policy ordering.

⸻

5. Policy Conflict Model

5.1 Current Model (v1)
	•	All matching rules apply.
	•	Any block = global block.
	•	First block message wins.

5.2 Extended Model (v1.1)

With mode + priority:

Situation	Outcome
enforce + warn	enforce wins
audit + enforce	enforce wins
warn only	inject warning
audit only	log only
multiple enforce	highest priority wins

This introduces deterministic conflict resolution.

⸻

6. New CLI Surface

6.1 cch explain rule

cch explain rule no-console-log

Output (example)

Rule: no-console-log
Event: PreToolUse
Mode: enforce
Priority: 100

Matchers:
  tools: [Edit, Write]
  extensions: [.ts, .js]

Action:
  run: .claude/validators/no-console-log.py

Metadata:
  author: cch-skill
  created_by: react-skill@2.1.0
  reason: Enforce CLAUDE.md rule
  confidence: high
  last_reviewed: 2025-01-21

Recent Activity:
  Triggered 14 times
  Blocked 3 times
  Last trigger: 2025-01-20 14:32

This turns CCH into a self-explaining policy system.

⸻

7. Enhanced Logging Schema

7.1 Log Event Extension

Each log entry now includes:

{
  "rule_name": "no-console-log",
  "mode": "enforce",
  "priority": 100,
  "metadata": {
    "author": "cch-skill",
    "created_by": "react-skill@2.1.0",
    "reason": "CLAUDE.md enforcement"
  },
  "decision": "blocked",
  "event": "PreToolUse",
  "timestamp": "2025-01-21T14:32:11Z"
}

This enables:
	•	Governance dashboards
	•	SOC2 evidence
	•	Policy analytics
	•	Root cause analysis

⸻

8. Validator Trust Levels (Future-Proof Hook)

Introduce optional trust model:

run:
  script: .claude/validators/check.py
  trust: local | verified | untrusted

Semantics (v1 = informational only)

Trust	Meaning
local	User-authored
verified	Signed skill
untrusted	External source

No enforcement in v1, but logs surface trust level.

This sets up future:
	•	sandboxing
	•	signing
	•	supply chain policy

⸻

9. Policy Distribution Model (Roadmap)

This addendum formally introduces the concept of:

Policy Packs

A policy pack is:
	•	A directory or repo of rules
	•	With versioning
	•	With metadata
	•	Importable into projects

Example future syntax:

imports:
  - source: "github.com/org/cch-baselines"
    version: ">=1.2.0"

This is intentionally not implemented yet, but the governance layer makes it possible.

⸻

10. Backward Compatibility

All new features are:

Feature	Required?
metadata	Optional
mode	Optional
priority	Optional
explain rule	Additive
enhanced logs	Additive

Existing configs continue to work unchanged.

⸻

11. Strategic Positioning

With this addendum, CCH becomes:

Not just:

“Claude hook system”

But:

Local-first AI Policy Engine

Comparable to:
	•	OPA (but human-readable)
	•	Terraform Sentinel (but local)
	•	Kubernetes admission controllers (but for agents)

⸻

12. Design Philosophy (Explicit)

This addendum formalizes the core principle:

LLMs do not enforce policy.
LLMs are subject to policy.

CCH is the policy authority.
Skills are policy authors.
Claude is policy-constrained execution.

⸻

Final Outcome

After this addendum, CCH supports:

Capability	Status
Deterministic enforcement	✅
Explainable policy	✅
Provenance	✅
Gradual rollout	✅
Audit trails	✅
Enterprise governance	✅
Future policy distribution	🔜


⸻

One-Sentence Summary

This addendum upgrades CCH from:

“a powerful hook system”

to:

“a first-class, auditable, local AI policy engine suitable for real organizational governance.”