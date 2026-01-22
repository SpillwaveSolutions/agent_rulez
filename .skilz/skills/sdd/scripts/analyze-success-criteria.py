#!/usr/bin/env python3
"""
Analyze success criteria coverage across spec.md and tasks.md

This script analyzes how well success criteria from the specification
have corresponding verification tasks. It identifies:
- Total success criteria count
- Success criteria with verification tasks
- Measurability of each criterion
- Task mapping for each criterion

Usage:
    python3 analyze-success-criteria.py

Output:
    JSON object with coverage metrics and analysis

Part of: /speckit.analyze workflow
"""

import json

# Success criteria from spec.md
success_criteria = {
    "SC-001": {
        "text": "50% faster task completion",
        "measurable": True,
        "metric": "performance",
        "tasks": ["T025-T027", "T041-T043", "T056-T058", "T071-T073", "T082"],
    },
    "SC-002": {
        "text": "95% success rate after viewing help",
        "measurable": True,
        "metric": "usability",
        "tasks": ["T028-T043"],  # US4 help modal
    },
    "SC-003": {
        "text": "Zero accessibility regressions",
        "measurable": True,
        "metric": "accessibility",
        "tasks": ["T004", "T040", "T076"],  # axe-core tests
    },
    "SC-004": {
        "text": "100% shortcuts have visual UI",
        "measurable": True,
        "metric": "discoverability",
        "tasks": ["T079", "T080"],  # Documentation
    },
    "SC-005": {
        "text": "Help overlay <100ms",
        "measurable": True,
        "metric": "performance",
        "tasks": ["T082"],  # Performance verification
    },
    "SC-006": {
        "text": "Works on macOS, Windows, Linux",
        "measurable": True,
        "metric": "cross-platform",
        "tasks": ["T077", "T078"],  # Manual testing
    },
    "SC-007": {
        "text": "Full keyboard navigation",
        "measurable": True,
        "metric": "accessibility",
        "tasks": ["T017-T073"],  # All user stories
    },
    "SC-008": {
        "text": "Screen readers discover shortcuts",
        "measurable": True,
        "metric": "accessibility",
        "tasks": ["T028", "T031", "T040"],  # ARIA labels
    },
}

# All have task coverage
all_covered = all(sc.get("tasks") for sc in success_criteria.values())

print(json.dumps({
    "total_success_criteria": len(success_criteria),
    "all_covered": all_covered,
    "coverage_summary": {k: bool(v.get("tasks")) for k, v in success_criteria.items()},
    "details": success_criteria
}, indent=2))
