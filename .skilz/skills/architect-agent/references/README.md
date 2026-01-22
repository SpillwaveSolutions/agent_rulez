# Reference Documentation Index

This directory contains all detailed protocols, guides, and templates for the architect-agent skill. Use this index to quickly find the documentation you need.

---

## Quick Navigation

- **Getting Started?** → [quick_start.md](#setup--installation)
- **Installing?** → [installation.md](#setup--installation)
- **Upgrading?** → [upgrade.md](#setup--installation)
- **Creating Instructions?** → [instruction_structure.md](#core-workflows)
- **Grading Work?** → [grading_rubrics.md](#core-workflows)
- **Setting Up OpenCode?** → [opencode_integration_quickstart.md](#opencode-integration-suite)
- **Debugging Issues?** → [get_unstuck_protocol.md](#advanced-workflows)

---

## Core Workflows (9 files)

Essential protocols that govern day-to-day architect agent operations.

### 1. logging_protocol.md
**What**: Real-time logging with `tee`, timestamps, and proper markdown formatting
**When**: Reference when creating instructions that require logging
**Key Topics**: tee usage, log file naming, real-time vs batch logging, verification logging

### 2. testing_protocol.md
**What**: Progressive testing schedule and coverage requirements
**When**: Reference when creating instructions for code implementation
**Key Topics**: Test after every 10-50 lines, coverage ≥60%, milestone testing, CI/CD validation

### 3. grading_rubrics.md
**What**: 6-category grading system with automatic grade caps
**When**: Reference when grading completed work
**Key Topics**: 100-point rubric, grade scale, automatic caps, deduction rules

### 4. agent_specialization.md
**What**: Agent categories, proper usage, and common mistakes
**When**: Reference when deciding which agents to require in instructions
**Key Topics**: qa-enforcer, change-explainer, docs-sync-editor, right agent for the job

### 5. resilience_protocol.md
**What**: Error recovery patterns and verification requirements
**When**: Reference when instructing code agent to handle errors
**Key Topics**: Retry logic, perplexity/context7 research, verification checkpoints

### 6. file_naming.md
**What**: Naming patterns for instructions, grades, and logs
**When**: Reference when creating any architect agent files
**Key Topics**: Date format, ticket ID, phase naming, matching rules

### 7. git_pr_management.md
**What**: Commit message format and PR creation templates
**When**: Reference when instructing code agent on git operations
**Key Topics**: Commit format, PR structure, no AI attribution

### 8. instruction_structure.md
**What**: Complete template for delegation instructions
**When**: Use as starting point when creating new instructions
**Key Topics**: All required sections, logging first, testing requirements, success criteria

### 9. ticket_tracking_pr_management.md
**What**: Ticket lifecycle and PR description generation
**When**: Reference when managing multi-phase tickets
**Key Topics**: Ticket structure, PR from ticket, project-memory integration

---

## Setup & Installation (3 files)

Guides for setting up and maintaining architect/code agent workspaces.

### 10. installation.md
**What**: Complete installation guide for v3.0+ (automated + manual setup)
**When**: First-time setup of architect or code agent workspace
**Key Topics**: Automated templates, manual directory creation, hooks configuration, verification

### 11. upgrade.md
**What**: Migration guide from v1.0/v2.0 to v3.0+ (critical hooks fix)
**When**: Upgrading existing workspace to latest protocols
**Key Topics**: Fresh start vs in-place upgrade, hooks.json → settings.json, verification steps

### 12. quick_start.md
**What**: 5-minute getting started guide
**When**: New users want fastest path to working system
**Key Topics**: Template setup, first instruction, first grading cycle

---

## Templates & Configuration (3 files)

Templates for configuring code agent workspaces and cross-workspace collaboration.

### 13. code_agent_claude_template.md
**What**: Template sections to add to code agent's CLAUDE.md
**When**: Setting up new code agent or adding delegation protocol
**Key Topics**: Instruction reception, grading protocol, memory updates, file lifecycle

### 14. code_agent_agents_template.md
**What**: Template sections to add to code agent's AGENTS.md
**When**: Setting up new code agent or adding delegation protocol
**Key Topics**: Multi-agent collaboration, commands reference, troubleshooting

### 15. permissions_setup_protocol.md
**What**: Cross-workspace permissions configuration (detailed examples)
**When**: Setting up architect-code agent collaboration to avoid permission prompts
**Key Topics**: settings.local.json, absolute paths, script-based protocols, wildcards

---

## OpenCode Integration Suite (5 files)

Complete guide to dual-mode operation (Claude Code + OpenCode).

### 16. opencode_integration_quickstart.md
**What**: Quick start for migrating code agent to OpenCode support
**When**: Want code agent to work in both Claude Code and OpenCode
**Key Topics**: Non-destructive migration, wrapper scripts, shell init functions

### 17. opencode_logging_protocol.md
**What**: Complete OpenCode logging protocol
**When**: Need detailed OpenCode-specific logging instructions
**Key Topics**: Wrapper script usage, shell functions, dual-mode compatibility

### 18. opencode_setup_guide.md
**What**: Detailed OpenCode workspace setup
**When**: Need step-by-step OpenCode configuration
**Key Topics**: Directory structure, shell-init.sh, wrapper scripts

### 19. opencode_migration_guide.md
**What**: Full migration guide with troubleshooting
**When**: Encountering issues during OpenCode migration
**Key Topics**: Migration steps, common errors, verification, rollback

### 20. claude_vs_opencode_comparison.md
**What**: Feature comparison and decision framework
**When**: Deciding whether to use Claude Code, OpenCode, or both
**Key Topics**: Feature matrix, use cases, recommendations

---

## Logging & Hooks (v3.0 Hybrid) (4 files)

v3.0+ logging system with automated hooks.

### 21. hybrid_logging_protocol.md
**What**: Hybrid logging approach (automated hooks + manual logging)
**When**: Understanding v3.0 logging architecture
**Key Topics**: Hook-based automation, manual fallback, 60-70% token savings

### 22. hybrid_logging_migration_guide.md
**What**: Migration from manual to hybrid logging
**When**: Upgrading from v1.0/v2.0 manual logging
**Key Topics**: Installation steps, hook configuration, verification

### 23. hook_configuration_critical.md
**What**: **CRITICAL FIX** - Hooks must be in settings.json (not hooks.json)
**When**: Hooks not working or setting up new workspace
**Key Topics**: Why hooks.json doesn't work, correct settings.json format, troubleshooting

### 24. hook_logger_enhancements.md
**What**: Enhanced hook logger with full argument capture
**When**: Understanding what information hooks automatically capture
**Key Topics**: Tool-specific extractors, argument limits, timestamp handling

---

## Advanced Workflows (3 files)

Advanced patterns for iterative improvement and troubleshooting.

### 25. instruction_grading_workflow.md
**What**: Iterative instruction-grading cycle (NEW in v3.0)
**When**: Want structured iteration until ≥95% quality threshold
**Key Topics**: UUID naming, temporary workspace, automatic cleanup, memory updates

### 26. get_unstuck_protocol.md
**What**: Troubleshooting guide when code agent gets stuck
**When**: Code agent stops making progress or needs intervention
**Key Topics**: Common blockers, research patterns, escalation protocol

### 27. workspace_setup_complete.md
**What**: Complete workspace setup checklist
**When**: Validating workspace is correctly configured
**Key Topics**: Required files, directory structure, permissions, verification commands

---

## Additional Resources (2 files)

Supplementary configuration and setup guides.

### 28. opencode_wrapper_setup.md
**What**: OpenCode wrapper script configuration details
**When**: Need to customize OpenCode wrapper behavior
**Key Topics**: Wrapper script anatomy, customization options, troubleshooting

### 29. permissions_setup_protocol.md
**What**: Detailed permissions examples and patterns
**When**: Setting up complex multi-project permissions
**Key Topics**: Pattern matching, security considerations, common configurations

---

## File Organization

```
references/
├── README.md (this file)
│
├── Core Workflows/
│   ├── logging_protocol.md
│   ├── testing_protocol.md
│   ├── grading_rubrics.md
│   ├── agent_specialization.md
│   ├── resilience_protocol.md
│   ├── file_naming.md
│   ├── git_pr_management.md
│   ├── instruction_structure.md
│   └── ticket_tracking_pr_management.md
│
├── Setup & Installation/
│   ├── installation.md
│   ├── upgrade.md
│   └── quick_start.md
│
├── Templates/
│   ├── code_agent_claude_template.md
│   ├── code_agent_agents_template.md
│   └── permissions_setup_protocol.md
│
├── OpenCode Suite/
│   ├── opencode_integration_quickstart.md
│   ├── opencode_logging_protocol.md
│   ├── opencode_setup_guide.md
│   ├── opencode_migration_guide.md
│   └── claude_vs_opencode_comparison.md
│
├── Logging & Hooks/
│   ├── hybrid_logging_protocol.md
│   ├── hybrid_logging_migration_guide.md
│   ├── hook_configuration_critical.md
│   └── hook_logger_enhancements.md
│
└── Advanced/
    ├── instruction_grading_workflow.md
    ├── get_unstuck_protocol.md
    ├── workspace_setup_complete.md
    └── opencode_wrapper_setup.md
```

---

## How to Use This Index

1. **Finding a topic**: Use Quick Navigation or category sections
2. **Reading a file**: All files are markdown, readable in any editor
3. **Cross-references**: Files reference each other - follow links as needed
4. **Progressive disclosure**: Start with SKILL.md, dive into references as needed

---

## Contributing

When adding new reference files:
1. Place file in `references/` directory
2. Update this README.md with entry
3. Update SKILL.md Reference Documents section
4. Add cross-references from related files
5. Update SPEC.md if new capability

---

**Total: 29 reference files**
**Last Updated:** 2025-01-21
**Version:** 4.0 (PDA Optimized)
**Token Budget:** ~1,800 tokens (orchestrator)
