# AGENTS.md

**NOTE:** All project specifications, implementation plans, and feature tracking are now consolidated in the `.speckit/` directory. The legacy `specs/` and `.specify/` directories have been removed.

## Git Workflow Requirements

**CRITICAL: Always use feature branches for all work.**

### Branching Model

```
main (protected)          <- Production-ready, fully validated
  ^
  |
develop (default)         <- Integration branch, fast CI (~2-3 min)
  ^
  |
feature/* | fix/*         <- Short-lived working branches
```

### Branch Rules
- **NEVER commit directly to `main` or `develop`**
- Create feature branches from `develop`: `git checkout develop && git checkout -b feature/<name>`
- PRs to `develop` run Fast CI (~2-3 min)
- PRs to `main` run Full Validation (~10-15 min) - use for releases only

### Branch Naming Convention
- Features: `feature/<feature-name>` (e.g., `feature/add-debug-command`)
- Bugfixes: `fix/<bug-description>` (e.g., `fix/config-parsing-error`)
- Documentation: `docs/<doc-topic>` (e.g., `docs/update-readme`)
- Hotfixes: `hotfix/<issue>` (for emergency fixes to main)

### Daily Development Workflow
```bash
# 1. Start from develop
git checkout develop && git pull origin develop

# 2. Create feature branch
git checkout -b feature/<name>

# 3. Make changes, run pre-commit checks
cargo fmt --all --check && cargo clippy --all-targets --all-features --workspace -- -D warnings && cargo test --tests --all-features --workspace

# 4. Push and create PR targeting develop
git push -u origin feature/<name>
gh pr create --base develop

# 5. After merge, clean up
git checkout develop && git pull && git branch -d feature/<name>
```

### Release Workflow (to main)
```bash
# Create PR from develop to main
gh pr create --base main --head develop --title "Release: v1.x.x"
# Wait for Full Validation (~10-15 min)
# Merge after all IQ/OQ/PQ tests pass
```

### Hotfix Workflow
```bash
# Create hotfix from main
git checkout main && git checkout -b hotfix/<issue>
# Fix, PR to main, then backport to develop
```

### Pre-Commit Checks (MANDATORY)
```bash
cargo fmt --all --check && cargo clippy --all-targets --all-features --workspace -- -D warnings && cargo test --tests --all-features --workspace
```

**NEVER commit if any check fails.** Fix all issues first.

### Pre-Push Checks (MANDATORY — before push or PR)
```bash
# Code coverage runs ALL tests including e2e with instrumentation.
# This catches pipe, process, and concurrency bugs that cargo test alone misses.
cargo llvm-cov --all-features --workspace --no-report
```

**NEVER push if code coverage tests fail.** The CI code coverage step WILL fail on the same bugs.

### CI Tiers
| Target | CI Level | Time | What Runs |
|--------|----------|------|-----------|
| PR to `develop` | Fast CI | ~2-3 min | fmt, clippy, unit tests, Linux IQ |
| PR to `main` | Full Validation | ~10-15 min | Fast CI + IQ (4 platforms) + OQ + PQ |

Reference: [docs/devops/BRANCHING.md](docs/devops/BRANCHING.md) | [docs/devops/CI_TIERS.md](docs/devops/CI_TIERS.md)

<skills_system priority="1">

## Available Skills

<!-- SKILLS_TABLE_START -->
<usage>
When users ask you to perform tasks, check if any of the available skills
below can help complete the task more effectively.

How to use skills:
- Invoke: Bash("skilz read <skill-name> --agent opencode")
- The skill content will load with detailed instructions
- Base directory provided in output for resolving bundled resources

Step-by-step process:
1. Identify a skill from <available_skills> that matches the user's request
2. Run the command above to load the skill's SKILL.md content
3. Follow the instructions in the loaded skill content
4. Skills may include bundled scripts, templates, and references
</usage>

<available_skills>

<skill>
<name>architect-agent</name>
<description>Coordinates planning, delegation, and evaluation across architect and code agent workspaces. Use when asked to "write instructions for code agent", "initialize architect workspace", "grade code agent work", "send instructions", or "verify code agent setup".</description>
<location>.opencode/skill/architect-agent/SKILL.md</location>
</skill>

<skill>
<name>design-doc-mermaid</name>
<description>Create Mermaid diagrams (activity, deployment, sequence, architecture) from text descriptions or source code. Use when asked to "create a diagram", "generate mermaid", "document architecture", "code to diagram", "create design doc", or "convert code to diagram". Supports hierarchical on-demand guide loading, Unicode semantic symbols, and Python utilities for diagram extraction and image conversion.</description>
<location>.opencode/skill/design-doc-mermaid/SKILL.md</location>
</skill>

<skill>
<name>documentation-specialist</name>
<description>|</description>
<location>.opencode/skill/documentation-specialist/SKILL.md</location>
</skill>

<skill>
<name>mastering-git-cli</name>
<description>Git CLI operations, workflows, and automation for modern development (2025). Use when working with repositories, commits, branches, merging, rebasing, worktrees, submodules, or multi-repo architectures. Includes parallel agent workflow patterns, merge strategies, conflict resolution, and large repo optimization. Triggers on git commands, version control, merge conflicts, worktree setup, submodule management, repository troubleshooting, branch strategy, rebase operations, cherry-pick decisions, and CI/CD git integration.</description>
<location>.opencode/skill/mastering-git-cli/SKILL.md</location>
</skill>

<skill>
<name>mastering-github-cli</name>
<description>|</description>
<location>.opencode/skill/mastering-github-cli/SKILL.md</location>
</skill>

<skill>
<name>mastering-python-skill</name>
<description>Modern Python coaching covering language foundations through advanced production patterns. Use when asked to "write Python code", "explain Python concepts", "set up a Python project", "configure Poetry or PDM", "write pytest tests", "create a FastAPI endpoint", "run uvicorn server", "configure alembic migrations", "set up logging", "process data with pandas", or "debug Python errors". Triggers on "Python best practices", "type hints", "async Python", "packaging", "virtual environments", "Pydantic validation", "dependency injection", "SQLAlchemy models".</description>
<location>.opencode/skill/mastering-python-skill/SKILL.md</location>
</skill>

<skill>
<name>mastering-typescript</name>
<description>|</description>
<location>.opencode/skill/mastering-typescript/SKILL.md</location>
</skill>

<skill>
<name>plantuml</name>
<description>Generate PlantUML diagrams from text descriptions and convert them to PNG/SVG images. Use when asked to "create a diagram", "generate PlantUML", "convert puml to image", "extract diagrams from markdown", or "prepare markdown for Confluence". Supports all PlantUML diagram types including UML (sequence, class, activity, state, component, deployment, use case, object, timing) and non-UML (ER diagrams, Gantt charts, JSON/YAML visualization, mindmaps, WBS, network diagrams, wireframes, and more).</description>
<location>.opencode/skill/plantuml/SKILL.md</location>
</skill>

<skill>
<name>pr-reviewer</name>
<description>></description>
<location>.opencode/skill/pr-reviewer/SKILL.md</location>
</skill>

<skill>
<name>project-memory</name>
<description>Set up and maintain a structured project memory system in docs/project_notes/ that tracks bugs with solutions, architectural decisions, key project facts, and work history. Use this skill when asked to "set up project memory", "track our decisions", "log a bug fix", "update project memory", or "initialize memory system". Configures both CLAUDE.md and AGENTS.md to maintain memory awareness across different AI coding tools.</description>
<location>.opencode/skill/project-memory/SKILL.md</location>
</skill>

<skill>
<name>sdd</name>
<description>This skill should be used when users want guidance on Spec-Driven Development methodology using GitHub's Spec-Kit. Guide users through executable specification workflows for both new projects (greenfield) and existing codebases (brownfield). After any SDD command generates artifacts, automatically provide structured 10-point summaries with feature status tracking, enabling natural language feature management and keeping users engaged throughout the process.</description>
<location>.opencode/skill/sdd/SKILL.md</location>
</skill>

<skill>
<name>mastering-hooks</name>
<description>Master Claude Context Hooks (CCH), the Rust-based runtime for controlling Claude Code behavior through hooks.yaml configuration. Use when asked to "install CCH", "create hooks", "debug hooks", "hook not firing", "configure context injection", "validate hooks.yaml", "PreToolUse", "PostToolUse", or "block dangerous commands". Covers installation, rule creation, troubleshooting, and optimization.</description>
<location>mastering-hooks/SKILL.md</location>
</skill>

<skill>
<name>release-cch</name>
<description>CCH release workflow automation. Use when asked to "release CCH", "create a release", "prepare release", "tag version", "hotfix release", or "publish CCH". Covers version management from Cargo.toml, changelog generation from conventional commits, PR creation, tagging, hotfix workflows, and GitHub Actions release monitoring.</description>
<location>.opencode/skill/release-cch/SKILL.md</location>
</skill>

<skill>
<name>using-claude-code-cli</name>
<description>Invoke Claude Code CLI from Python orchestrators and shell scripts. Use when asked to "spawn claude as subprocess", "automate claude cli", "run claude headless", "configure --allowedTools", "set up claude hooks", or "parallel claude invocation". Covers permissions, directory access (--add-dir), hooks, sandbox mode, and async patterns.</description>
<location>.opencode/skill/using-claude-code-cli/SKILL.md</location>
</skill>

</available_skills>
<!-- SKILLS_TABLE_END -->

</skills_system>

## Active Technologies
- Rust 2024 edition (no unsafe code blocks) + serde (JSON), clap (CLI), regex (pattern matching), tokio (async) (001-cch-binary-v1)
- File system (configuration files, logs), N/A for runtime data (001-cch-binary-v1)

## Recent Changes
- 001-cch-binary-v1: Added Rust 2024 edition (no unsafe code blocks) + serde (JSON), clap (CLI), regex (pattern matching), tokio (async)
- mastering-hooks: Added CCH mastery skill with comprehensive documentation and workflow guidance
