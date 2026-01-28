#!/usr/bin/env python3
"""
Analyze requirement coverage across spec.md and tasks.md

This script analyzes how well functional requirements from the specification
are covered by implementation tasks. It identifies:
- Total requirements count
- Requirements with task coverage
- Uncovered requirements (gaps)
- Vague requirements lacking measurable criteria

Usage:
    python3 analyze-requirements.py

Output:
    JSON object with coverage metrics and analysis

Part of: /speckit.analyze workflow
"""

import json

# Requirements inventory from spec.md
requirements = {
    # Search Access (US1)
    "FR-001": {"story": "US1", "text": "detect Cmd/Ctrl+F", "measurable": True},
    "FR-002": {"story": "US1", "text": "focus search input", "measurable": True},
    "FR-003": {"story": "US1", "text": "select existing text", "measurable": True},
    "FR-004": {"story": "US1", "text": "prevent default Cmd/Ctrl+F", "measurable": True},
    "FR-005": {"story": "US1", "text": "return to list from detail", "measurable": True},
    "FR-006": {"story": "US1", "text": "clear search on Escape", "measurable": True},

    # Tab Navigation (US2)
    "FR-007": {"story": "US2", "text": "detect Cmd/Ctrl+1-6", "measurable": True},
    "FR-008": {"story": "US2", "text": "switch to Overview tab", "measurable": True},
    "FR-009": {"story": "US2", "text": "switch to Content tab", "measurable": True},
    "FR-010": {"story": "US2", "text": "switch to Triggers tab", "measurable": True},
    "FR-011": {"story": "US2", "text": "switch to Diagram tab", "measurable": True},
    "FR-012": {"story": "US2", "text": "switch to References tab", "measurable": True},
    "FR-013": {"story": "US2", "text": "switch to Scripts tab", "measurable": True},
    "FR-014": {"story": "US2", "text": "visual indication of active tab", "measurable": True},
    "FR-015": {"story": "US2", "text": "ignore when no skill selected", "measurable": True},

    # List Navigation (US3)
    "FR-016": {"story": "US3", "text": "detect arrow keys", "measurable": True},
    "FR-017": {"story": "US3", "text": "highlight first on Down", "measurable": True},
    "FR-018": {"story": "US3", "text": "move to next on Down", "measurable": True},
    "FR-019": {"story": "US3", "text": "move to previous on Up", "measurable": True},
    "FR-020": {"story": "US3", "text": "wrap to first from last", "measurable": True},
    "FR-021": {"story": "US3", "text": "wrap to last from first", "measurable": True},
    "FR-022": {"story": "US3", "text": "select on Enter", "measurable": True},
    "FR-023": {"story": "US3", "text": "display details on Enter", "measurable": True},
    "FR-024": {"story": "US3", "text": "deselect on Escape", "measurable": True},
    "FR-025": {"story": "US3", "text": "distinct visual indicators", "measurable": True},

    # Help Overlay (US4)
    "FR-026": {"story": "US4", "text": "detect ? key", "measurable": True},
    "FR-027": {"story": "US4", "text": "display modal", "measurable": True},
    "FR-028": {"story": "US4", "text": "group shortcuts by context", "measurable": True},
    "FR-029": {"story": "US4", "text": "show key and description", "measurable": True},
    "FR-030": {"story": "US4", "text": "platform-appropriate keys", "measurable": True},
    "FR-031": {"story": "US4", "text": "close on Escape", "measurable": True},
    "FR-032": {"story": "US4", "text": "close on click outside", "measurable": True},
    "FR-033": {"story": "US4", "text": "trap focus", "measurable": True},
    "FR-034": {"story": "US4", "text": "ARIA labels", "measurable": True},

    # Cross-Platform
    "FR-035": {"story": "Foundation", "text": "detect OS", "measurable": True},
    "FR-036": {"story": "Foundation", "text": "use Cmd on macOS", "measurable": True},
    "FR-037": {"story": "Foundation", "text": "use Ctrl on Windows/Linux", "measurable": True},
    "FR-038": {"story": "US4", "text": "display correct key name", "measurable": True},

    # Accessibility
    "FR-039": {"story": "All", "text": "visible UI alternatives", "measurable": False, "vague": "visible"},
    "FR-040": {"story": "All", "text": "manage focus for screen readers", "measurable": False, "vague": "appropriately"},
    "FR-041": {"story": "All", "text": "prevent keyboard traps", "measurable": True},
    "FR-042": {"story": "All", "text": "announce state changes", "measurable": False, "vague": "state changes"},
}

# Task mapping (simplified - real analysis would parse tasks.md)
task_coverage = {
    "FR-001": ["T017", "T019", "T021"],
    "FR-002": ["T018", "T020", "T021"],
    "FR-003": ["T018", "T020", "T022"],
    "FR-004": ["T017", "T019"],
    "FR-005": ["T018", "T020", "T023"],
    "FR-006": ["T018", "T020", "T024"],
    "FR-007": ["T045", "T046", "T048-T053"],
    "FR-008": ["T044", "T047", "T048"],
    "FR-009": ["T044", "T047", "T049"],
    "FR-010": ["T044", "T047", "T050"],
    "FR-011": ["T044", "T047", "T051"],
    "FR-012": ["T044", "T047", "T052"],
    "FR-013": ["T044", "T047", "T053"],
    "FR-014": ["T044", "T047", "T054"],
    "FR-015": ["T044", "T045", "T055"],
    "FR-016": ["T059", "T061", "T063"],
    "FR-017": ["T059", "T061", "T063"],
    "FR-018": ["T059", "T061", "T064"],
    "FR-019": ["T059", "T061", "T065"],
    "FR-020": ["T059", "T061", "T066"],
    "FR-021": ["T059", "T061", "T067"],
    "FR-022": ["T059", "T061", "T068"],
    "FR-023": ["T059", "T061", "T068"],
    "FR-024": ["T059", "T061", "T069"],
    "FR-025": ["T060", "T062", "T070"],
    "FR-026": ["T030", "T032", "T034"],
    "FR-027": ["T028", "T031", "T034"],
    "FR-028": ["T028", "T031", "T035"],
    "FR-029": ["T028", "T031", "T036"],
    "FR-030": ["T028", "T031", "T036"],
    "FR-031": ["T028", "T031", "T037"],
    "FR-032": ["T028", "T031", "T038"],
    "FR-033": ["T029", "T031", "T039"],
    "FR-034": ["T028", "T031", "T040"],
    "FR-035": ["T012", "T014"],
    "FR-036": ["T012", "T014"],
    "FR-037": ["T012", "T014"],
    "FR-038": ["T028", "T031"],
    "FR-039": ["T079", "T080"],  # Documentation
    "FR-040": ["T076", "T040"],  # Accessibility testing
    "FR-041": ["T029", "T031", "T039"],
    "FR-042": ["T060", "T062"],  # ARIA in components
}

# Calculate coverage
total_reqs = len(requirements)
covered_reqs = len([r for r in requirements if r in task_coverage])
coverage_pct = (covered_reqs / total_reqs) * 100

print(json.dumps({
    "total_requirements": total_reqs,
    "covered_requirements": covered_reqs,
    "coverage_percentage": round(coverage_pct, 1),
    "uncovered": [r for r in requirements if r not in task_coverage],
    "vague_requirements": [r for r, data in requirements.items() if data.get("vague")],
}, indent=2))
