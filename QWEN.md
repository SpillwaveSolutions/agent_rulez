# QWEN.md

**NOTE:** All project specifications, implementation plans, and feature tracking are now consolidated in the `.speckit/` directory. The legacy `specs/` and `.specify/` directories have been removed.

<skills_system priority="1">

## Available Skills

<!-- SKILLS_TABLE_START -->
<usage>
When users ask you to perform tasks, check if any of the available skills
below can help complete the task more effectively.

How to use skills:
- Invoke: Bash("skilz read <skill-name> --agent qwen")
- The skill content will load with detailed instructions
- Base directory provided in output for resolving bundled resources

Step-by-step process:
1. Identify a skill from <available_skills> that matches the user's request
2. Run the command above to load the skill's SKILL.md content
3. Follow the instructions in the loaded skill content
4. Skills may include bundled scripts, templates, and references

Usage notes:
- Only use skills listed in <available_skills> below
- Do not invoke a skill that is already loaded in your context
</usage>

<available_skills>

<skill>
<name>architect-agent</name>
<description>Coordinates planning, delegation, and evaluation across architect and code agent workspaces. Use when asked to "write instructions for code agent", "initialize architect workspace", "grade code agent work", "send instructions", or "verify code agent setup".</description>
<location>.skilz/skills/architect-agent/SKILL.md</location>
</skill>

<skill>
<name>design-doc-mermaid</name>
<description>Create Mermaid diagrams (activity, deployment, sequence, architecture) from text descriptions or source code. Use when asked to "create a diagram", "generate mermaid", "document architecture", "code to diagram", "create design doc", or "convert code to diagram". Supports hierarchical on-demand guide loading, Unicode semantic symbols, and Python utilities for diagram extraction and image conversion.</description>
<location>.skilz/skills/design-doc-mermaid/SKILL.md</location>
</skill>

<skill>
<name>documentation-specialist</name>
<description>|</description>
<location>.skilz/skills/documentation-specialist/SKILL.md</location>
</skill>

<skill>
<name>mastering-git-cli</name>
<description>Git CLI operations, workflows, and automation for modern development (2025). Use when working with repositories, commits, branches, merging, rebasing, worktrees, submodules, or multi-repo architectures. Includes parallel agent workflow patterns, merge strategies, conflict resolution, and large repo optimization. Triggers on git commands, version control, merge conflicts, worktree setup, submodule management, repository troubleshooting, branch strategy, rebase operations, cherry-pick decisions, and CI/CD git integration.</description>
<location>.skilz/skills/mastering-git-cli/SKILL.md</location>
</skill>

<skill>
<name>mastering-github-cli</name>
<description>|</description>
<location>.skilz/skills/mastering-github-cli/SKILL.md</location>
</skill>

<skill>
<name>mastering-python-skill</name>
<description>Modern Python coaching covering language foundations through advanced production patterns. Use when asked to "write Python code", "explain Python concepts", "set up a Python project", "configure Poetry or PDM", "write pytest tests", "create a FastAPI endpoint", "run uvicorn server", "configure alembic migrations", "set up logging", "process data with pandas", or "debug Python errors". Triggers on "Python best practices", "type hints", "async Python", "packaging", "virtual environments", "Pydantic validation", "dependency injection", "SQLAlchemy models".</description>
<location>.skilz/skills/mastering-python-skill/SKILL.md</location>
</skill>

<skill>
<name>mastering-typescript</name>
<description>|</description>
<location>.skilz/skills/mastering-typescript/SKILL.md</location>
</skill>

<skill>
<name>plantuml</name>
<description>Generate PlantUML diagrams from text descriptions and convert them to PNG/SVG images. Use when asked to "create a diagram", "generate PlantUML", "convert puml to image", "extract diagrams from markdown", or "prepare markdown for Confluence". Supports all PlantUML diagram types including UML (sequence, class, activity, state, component, deployment, use case, object, timing) and non-UML (ER diagrams, Gantt charts, JSON/YAML visualization, mindmaps, WBS, network diagrams, wireframes, and more).</description>
<location>.skilz/skills/plantuml/SKILL.md</location>
</skill>

<skill>
<name>pr-reviewer</name>
<description>></description>
<location>.skilz/skills/pr-reviewer/SKILL.md</location>
</skill>

<skill>
<name>project-memory</name>
<description>Set up and maintain a structured project memory system in docs/project_notes/ that tracks bugs with solutions, architectural decisions, key project facts, and work history. Use this skill when asked to "set up project memory", "track our decisions", "log a bug fix", "update project memory", or "initialize memory system". Configures both CLAUDE.md and AGENTS.md to maintain memory awareness across different AI coding tools.</description>
<location>.skilz/skills/project-memory/SKILL.md</location>
</skill>

<skill>
<name>sdd</name>
<description>This skill should be used when users want guidance on Spec-Driven Development methodology using GitHub's Spec-Kit. Guide users through executable specification workflows for both new projects (greenfield) and existing codebases (brownfield). After any SDD command generates artifacts, automatically provide structured 10-point summaries with feature status tracking, enabling natural language feature management and keeping users engaged throughout the process.</description>
<location>.skilz/skills/sdd/SKILL.md</location>
</skill>

</available_skills>
<!-- SKILLS_TABLE_END -->

</skills_system>
