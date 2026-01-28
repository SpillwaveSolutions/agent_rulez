# Spec-Driven Development (SDD) Skill

A comprehensive Claude Code skill for guiding users through GitHub's Spec-Kit and the Spec-Driven Development methodology.

## What is Spec-Driven Development?

Spec-Driven Development flips traditional software development on its head. Instead of treating specifications as temporary scaffolding, SDD makes them **executable** - they directly generate working implementations rather than just guiding them.

## What's New in v2.1.0

### 🎯 Enhanced Explanations & Summaries
- **10-Point Summary Template**: Structured summaries after every SDD command showing key decisions, what was generated, items to review, watch-outs, and next steps
- **Rationale for Decisions**: Every major decision includes "why" it was made
- **No More Black Boxes**: Clear explanations eliminate the mystery of what was just generated

### 📊 Feature Status Tracking
- **Automatic Progress Tracking**: See exactly where each feature is (Specified, Planned, Tasked, In Progress, Complete)
- **Hybrid Status Display**: Brief status line in every summary + detailed dashboard on demand
- **Dependency Visualization**: See what features depend on what, and what's blocking progress
- **Progress Percentages**: Automatic calculation based on workflow completion (20%, 40%, 60%, 80%, 100%)

### 💬 Natural Language Feature Management
- **Add Features**: Just say "Add a feature for email notifications"
- **Reorder Features**: "Move user-notifications before profile-management"
- **Remove Features**: "We don't need reporting anymore"
- **Show Status**: "Show me all features" or "What's blocking admin-dashboard?"
- **Check Dependencies**: "Can we start profile-management yet?"

### 📁 Brownfield Support
Comprehensive support for **existing codebases**! Reverse-engineer existing projects into SDD format, generate constitutions from existing code, and integrate new features into legacy systems.

## Quick Start

To use this skill, simply mention any of these trigger words in your conversation with Claude:

**General SDD:**
- "spec-driven development", "sdd", "speckit"
- "specify cli", "/speckit"
- "executable specifications"

**For New Projects (Greenfield):**
- "new project", "from scratch"
- "specify init"

**For Existing Projects (Brownfield):**
- "brownfield", "existing codebase"
- "legacy code", "modernization"
- "reverse engineer", "codebase analysis"

**For Feature Management:**
- "feature status", "track features"
- "add feature", "move feature"
- "show features", "feature progress"

## What This Skill Provides

### 1. Installation Guidance
- Persistent installation (recommended) using `uv tool install`
- One-time usage with `uvx`
- Installation verification with `specify check`
- Troubleshooting for common issues

### 2. Complete Workflow Support

**Greenfield Workflow** (New Projects - 6 steps):
1. **Initialize Project**: `specify init`
2. **Establish Principles**: `/speckit.constitution`
3. **Create Specification**: `/speckit.specify`
4. **Technical Planning**: `/speckit.plan`
5. **Task Breakdown**: `/speckit.tasks`
6. **Implementation**: `/speckit.implement`

**Brownfield Workflow** (Existing Projects - 7 steps):
1. **Analyze Codebase**: `/speckit.brownfield`
2. **Initialize SDD**: `specify init --here --force`
3. **Generate Constitution**: `/speckit.analyze-codebase`
4. **Choose Strategy**: `/speckit.reverse-engineer`
5. **Document Features**: (optional, based on strategy)
6. **Specify New Feature**: `/speckit.specify`
7. **Integration Planning**: `/speckit.integration-plan`

### 3. Feature Status Tracking & Management

- **Automatic Progress Tracking**: Tracks each feature through Specified (20%) → Planned (40%) → Tasked (60%) → In Progress (80%) → Complete (100%)
- **Natural Language Commands**: "Add feature for X", "Move feature Y before Z", "Show feature status"
- **Dependency Management**: Automatic dependency tracking and blocking detection
- **Status Dashboard**: Brief status in every summary + detailed dashboard on demand (`/speckit.status` or option [D])

### 4. Optional Enhancement Commands

- `/speckit.clarify` - Clarify underspecified areas
- `/speckit.analyze` - Consistency & coverage analysis
- `/speckit.checklist` - Generate quality validation checklists

### 5. Validation Commands (Brownfield)

- `/speckit.validate-reverse-engineering` - Verify reverse-engineering accuracy
- `/speckit.coverage-check` - Check documentation coverage
- `/speckit.validate-constitution` - Validate constitution consistency
- `/speckit.trace [feature]` - Spec-to-code traceability

### 5. Development Phase Support

- **Greenfield (0-to-1)**: Build new projects from scratch
- **Brownfield (Existing)**: Reverse-engineer and enhance existing codebases
- **Creative Exploration**: Try multiple tech stacks in parallel
- **Legacy Modernization**: Systematically upgrade old systems

### 6. Best Practices

- For new users: Start small, follow the sequence
- For experienced users: Parallel exploration, custom checklists
- For enterprise teams: Establish constitution early, version control everything

## Artifacts Generated

After running SDD commands, expect these artifacts:

```
project-name/
├── .speckit/
│   ├── constitution.md      # Project principles
│   ├── features/
│   │   └── 001-feature-name/
│   │       ├── specify.md    # Requirements
│   │       ├── plan.md       # Technical plan
│   │       ├── tasks.md      # Task breakdown
│   │       └── checklist.md  # Quality gates
│   └── .claude/
│       └── commands/         # Slash commands
└── [application code]
```

## Supported AI Agents

Works with Claude Code, GitHub Copilot, Cursor, Windsurf, Gemini CLI, and many others (see skill.md for full list).

## Prerequisites

Users need:
- `uv` for package management
- Python 3.11+
- Git
- A supported AI coding agent

## Example Usage

```bash
# Install specify-cli
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git

# Initialize project
specify init my-app --ai claude

# Then in the AI agent:
/speckit.constitution Create principles for production-ready web apps...
/speckit.specify Build a task management app with drag-and-drop...
/speckit.plan Use React with TypeScript, Vite, and Tailwind CSS...
/speckit.tasks
/speckit.implement
```

## Files in This Skill

- **SKILL.md**: Modern skill with YAML frontmatter and lean orchestration (v2.1.0)
- **README.md**: This file - quick reference
- **CLAUDE.md**: Repository guidance for AI agents
- **references/**: Detailed workflow documentation
  - **sdd_install.md**: Installation and setup guide
  - **greenfield.md**: Complete greenfield workflow with enhanced summaries (6 steps)
  - **brownfield.md**: Complete brownfield workflow with enhanced summaries (7 steps)
  - **feature_management.md**: Comprehensive feature management guide (NEW in v2.1.0)

## Resources

- GitHub Spec-Kit: https://github.com/github/spec-kit
- Issues/Support: https://github.com/github/spec-kit/issues

## Philosophy

Remember: This is **AI-native development**. Specifications aren't just documentation - they're executable artifacts that directly drive implementation. The AI agent uses them to generate working code that matches the intent defined in the specs.

---

**Maintained by**: Based on GitHub Spec-Kit by Den Delimarsky (@localden) and John Lam (@jflam)
**License**: MIT
