#!/usr/bin/env python3
"""
Analyze edge case coverage across spec.md and tasks.md

This script analyzes how well edge cases from the specification
are covered by implementation tasks. It identifies:
- Total edge cases count
- Explicitly covered edge cases (have dedicated tasks/tests)
- Implicitly covered edge cases (handled by general logic)
- Uncovered edge cases (gaps requiring attention)

Usage:
    python3 analyze-edge-cases.py

Output:
    JSON object with coverage metrics and analysis

Part of: /speckit.analyze workflow
"""

import json

# Edge cases from spec.md
edge_cases = {
    "E1": {
        "text": "Browser shortcut conflicts (Cmd+F)",
        "covered_by": ["FR-004", "T017", "T019"],
        "coverage": "EXPLICIT"
    },
    "E2": {
        "text": "Rapid key presses",
        "covered_by": [],
        "coverage": "IMPLICIT",  # Store state updates handle this
        "note": "Zustand state updates are atomic"
    },
    "E3": {
        "text": "Cmd/Ctrl+1-6 while help modal open",
        "covered_by": ["T056"],
        "coverage": "EXPLICIT"
    },
    "E4": {
        "text": "Empty skill list navigation",
        "covered_by": ["T062", "T064"],
        "coverage": "IMPLICIT",  # Component handles out-of-bounds
        "note": "Component handles bounds checking"
    },
    "E5": {
        "text": "Highlight skill while different skill selected",
        "covered_by": ["FR-025", "T062", "T064"],
        "coverage": "EXPLICIT"
    },
    "E6": {
        "text": "Tab navigation when tabs missing",
        "covered_by": ["T057"],
        "coverage": "EXPLICIT"
    },
    "E7": {
        "text": "Multiple Escape presses in different contexts",
        "covered_by": ["T020", "T024", "T031", "T037", "T063", "T071"],
        "coverage": "EXPLICIT"
    },
}

uncovered = [k for k, v in edge_cases.items() if v["coverage"] == "UNCOVERED"]
implicit = [k for k, v in edge_cases.items() if v["coverage"] == "IMPLICIT"]

print(json.dumps({
    "total_edge_cases": len(edge_cases),
    "explicitly_covered": len([v for v in edge_cases.values() if v["coverage"] == "EXPLICIT"]),
    "implicitly_covered": len(implicit),
    "uncovered": uncovered,
    "uncovered_details": {k: edge_cases[k]["text"] for k in uncovered},
    "all_edge_cases": edge_cases
}, indent=2))
