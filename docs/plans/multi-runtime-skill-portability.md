# Plan: Multi-Runtime Skill Portability Layer (v2.3.0)

## Context

RuleZ's **binary** already supports 5 AI coding runtimes (Claude Code, OpenCode, Gemini, Copilot, Codex) via adapters, install commands, and hook runners. However, the **skill/command layer** (14 skills in `.claude/skills/`, commands in `.claude/commands/`) is manually duplicated to `.opencode/skill/`, `.gemini/skills/`, `.codex/skills/`, `.skilz/skills/` with no conversion. Cross-references use Claude Code conventions (PascalCase tools, `~/.claude/` paths, dot-namespaced commands) that are incorrect for other runtimes.

**Goal:** Build an installer-based conversion pipeline (inspired by GSD's `bin/install.js`) that transforms canonical Claude Code source into runtime-specific installations. Author once in `.claude/`, convert at install time, run everywhere.

**Reference implementation:** GSD installer at `/Users/richardhightower/src/get-shit-done/bin/install.js` -- 113KB Node.js file with complete conversion logic for Claude, OpenCode, Gemini, Codex, Copilot, and Antigravity runtimes.

---

## Architecture

### Canonical Source (Claude Code format)

```
.claude/
  skills/           # 14 skills (architect-agent, pr-reviewer, etc.)
  commands/          # Commands (speckit.*, cch-release)
mastering-hooks/     # RuleZ documentation skill (special handling)
```

### Install-Time Conversion Pipeline

```
.claude/skills/ + .claude/commands/ + mastering-hooks/
        |
        v
    Discovery (scan sources, build inventory)
        |
        v
    Transform (per-runtime pipeline)
        |
        v
    Write (to target runtime directory)
        |
        v
    Config Gen (update GEMINI.md, AGENTS.md, etc.)
```

### CLI Interface

```bash
rulez skills install --runtime claude   [--scope project|global] [--dry-run]
rulez skills install --runtime opencode [--scope project|global] [--dry-run]
rulez skills install --runtime gemini   [--scope project|global] [--dry-run]
rulez skills install --runtime codex    [--scope project|global] [--dry-run]
rulez skills install --runtime skills   --dir .qwen/skills [--dry-run]
rulez skills sync                       # Install to all configured runtimes
rulez skills status                     # Show which runtimes are installed
rulez skills diff    --runtime <rt>     # Show what would change
rulez skills clean   --runtime <rt>     # Remove generated files
```

**Design decision:** New `rulez skills` subcommand family, not extending `rulez install`. Hook registration (binary + settings.json) and skill distribution are orthogonal concerns.

### Runtime Profiles

| Runtime | Skills Dir | Commands Dir | Command Sep | Config File | Tool Style | Path Prefix |
|---------|-----------|-------------|-------------|-------------|-----------|-------------|
| Claude | `.claude/skills/` | `.claude/commands/` | `.` (dot) | CLAUDE.md | PascalCase | `~/.claude/` |
| OpenCode | `.opencode/skill/` | `.opencode/command/` | `-` (hyphen) | AGENTS.md | lowercase | `~/.config/opencode/` |
| Gemini | `.gemini/skills/` | (none -- TOML) | `:` (colon) | GEMINI.md | snake_case | `~/.gemini/` |
| Codex | `.codex/skills/` | (skills only) | `-` (hyphen) | AGENTS.md | lowercase | `~/.codex/` |
| Custom | `<dir>/` | (skills only) | `-` (hyphen) | -- | lowercase | -- |

### Transformation Rules

Based on GSD's proven patterns (`bin/install.js`):

**1. Tool Name Mapping** (reuse existing `map_tool_name()` from `rulez/src/adapters/`)

| Claude (canonical) | OpenCode | Gemini |
|-------------------|----------|--------|
| Read | read | read_file |
| Write | write | write_file |
| Edit | edit | replace |
| Bash | bash | run_shell_command |
| Glob | glob | glob |
| Grep | grep | search_file_content |
| WebSearch | websearch | google_web_search |
| WebFetch | webfetch | web_fetch |
| TodoWrite | todowrite | write_todos |
| AskUserQuestion | question | ask_user |
| Task | task | (excluded -- auto-registered) |

**2. Path Reference Rewriting**
- `~/.claude/` -> `~/.config/opencode/` (OpenCode)
- `~/.claude/` -> `~/.gemini/` (Gemini)
- `~/.claude/` -> `~/.codex/` (Codex)
- `.claude/` -> `.opencode/` etc. for project-local paths

**3. Command Naming**
- Claude: `speckit.analyze.md` -> OpenCode: `speckit-analyze.md` (flatten dots to hyphens)
- Cross-references: `/speckit.plan` -> `/speckit-plan`

**4. Color Conversion** (OpenCode requires hex)
- `cyan` -> `#00FFFF`, `red` -> `#FF0000`, etc. (13 named colors)
- Gemini: strip `color:` field entirely

**5. Frontmatter Conversion**
- `allowed-tools:` array -> `tools:` object with `tool: true` entries (OpenCode)
- `allowed-tools:` -> `tools:` array with snake_case names (Gemini)
- Strip unsupported fields: `skills:`, `memory:`, `maxTurns:`, `permissionMode:` (OpenCode)
- Strip `color:`, `skills:` (Gemini)

**6. MCP Tool Handling**
- OpenCode: preserve `mcp__*` tool names as-is
- Gemini: exclude (auto-discovered at runtime)

---

## New Rust Module Structure

```
rulez/src/
  skills/
    mod.rs                   # Module root, re-exports
    profiles.rs              # Runtime enum, RuntimeProfile struct
    discovery.rs             # SkillInventory, scan .claude/skills/ + commands/
    transform.rs             # TransformPipeline, TransformContext
    transforms/
      mod.rs                 # SkillTransform trait
      path_refs.rs           # Path reference rewriting
      command_naming.rs      # Dot-to-hyphen, cross-ref rewriting
      tool_names.rs          # PascalCase -> runtime convention in markdown
      frontmatter.rs         # YAML frontmatter field conversion
      colors.rs              # Named color -> hex conversion
    writer.rs                # File writing with clean-install approach
    config_gen.rs            # Generate/update GEMINI.md, AGENTS.md sections
  cli/
    skills.rs                # CLI subcommand handler (new)
```

### Critical Existing Files to Modify

- `rulez/src/main.rs` -- Add `Skills` variant to `Commands` enum
- `rulez/src/cli.rs` -- Add `pub mod skills;` and dispatch
- `rulez/src/lib.rs` -- Add `pub mod skills;`

### Critical Existing Files to Reuse

- `rulez/src/adapters/gemini.rs` -- `map_tool_name()` mappings
- `rulez/src/adapters/opencode.rs` -- `map_tool_name()` mappings
- `rulez/src/cli/opencode_install.rs` -- Pattern for scope handling, path resolution
- `docs/TOOL-MAPPING.md` -- Authoritative cross-platform mapping table

---

## Key Decisions

- **Phase numbering:** Phases start at 34 under the new v2.3.0 milestone (independent from v2.2's phase 34-36)
- **Copilot:** Excluded from scope. Copilot uses a VSCode extension model that's fundamentally different from file-based skills
- **Workflow:** Store this plan at `docs/plans/multi-runtime-skill-portability.md`, then use GSD skills to create milestone structure
- **Transformations:** Hardcoded in Rust (not configurable YAML). The 4 supported runtimes have well-known conventions

## Phase Breakdown (Milestone v2.3.0)

### Phase 34: Runtime Profiles and Skill Discovery

**Goal:** Build data model and discovery layer -- no file writing yet.

**New files:** `rulez/src/skills/{mod,profiles,discovery}.rs`

**Key types:**
- `Runtime` enum: `Claude, OpenCode, Gemini, Codex, Custom(String)`
- `RuntimeProfile` struct: skills_dir, commands_dir, command_separator, tool_style, path_prefix
- `SkillInventory`: scans `.claude/skills/` and `.claude/commands/`, builds manifest
- `DiscoveredSkill`: name, source_dir, entry_point, resources list

**Tests:** Unit tests for profile construction, discovery with tempdir fixtures

### Phase 35: Transformation Engine

**Goal:** Build content transformation pipeline with all 6 transform types.

**New files:** `rulez/src/skills/{transform,transforms/*.rs}`

**Key types:**
- `SkillTransform` trait: `transform_content()`, `transform_filename()`
- `TransformPipeline`: ordered chain of transforms, constructed per-runtime
- `TransformContext`: source/target runtime, profiles

**Transforms:** path_refs, command_naming, tool_names, frontmatter, colors, cross_refs

**Reuse:** Tool name mappings from existing `adapters/*.rs` -- extract to shared constants

**Tests:** Unit tests for each transform in isolation, integration test with known skill

### Phase 36: CLI Integration and File Writer

**Goal:** Wire up `rulez skills install` CLI subcommand with file writing.

**New files:** `rulez/src/skills/writer.rs`, `rulez/src/cli/skills.rs`

**Modify:** `main.rs` (add Skills command), `cli.rs` (add dispatch), `lib.rs` (add module)

**Writer behavior:**
- Clean-install: remove existing target dir, write fresh
- Copy binary resources (scripts, images) without transformation
- Transform `.md` files through pipeline
- Print summary of files written
- `--dry-run` mode prints plan without writing

**Tests:** Integration tests for full pipeline with tempdir

### Phase 37: Config File Generation and Mastering-Hooks

**Goal:** Auto-generate runtime config files and handle mastering-hooks skill.

**New files:** `rulez/src/skills/config_gen.rs`

**Config generation:**
- After installing to `.gemini/skills/`, update `GEMINI.md` skill registry section
- Use `<!-- RULEZ_SKILLS_START -->` / `<!-- RULEZ_SKILLS_END -->` markers
- Preserve non-skill sections of config files
- Generate `AGENTS.md` for Codex with skill registry

**Mastering-hooks special handling:**
- Register as additional discovery source (lives at repo root, not in `.claude/skills/`)
- Same transform pipeline but with context-aware rewriting for platform references

**Tests:** Integration test for config generation roundtrip, mastering-hooks install

### Phase 38: Status, Diff, Sync, and DX Polish

**Goal:** Complete the `rulez skills` subcommand family with status/diff/sync/clean.

**Features:**
- `rulez skills status` -- table showing installed runtimes and freshness (mtime comparison)
- `rulez skills diff --runtime gemini` -- colored diff of what would change
- `rulez skills sync` -- install to all detected runtimes
- `rulez skills clean --runtime opencode` -- remove generated files
- Colorized output with progress indicators

**Tests:** Unit tests for status comparison, integration tests for sync/clean

---

## Testing Strategy

**Unit tests** (per module):
- RuntimeProfile construction and path generation
- Each transform in isolation (path refs, tool names, commands, frontmatter, colors)
- Discovery scanning with tempdir fixtures
- Config file section replacement

**Integration tests** (`rulez/tests/`):
- Full pipeline: discover -> transform -> write to tempdir -> verify output
- Per-runtime: OpenCode, Gemini, Codex output verification
- Dry-run produces correct plan without writing files
- Clean removes only generated files

**Pre-push:** Standard CI pipeline (fmt, clippy, test, coverage)

---

## Verification

After implementation, verify:

1. `rulez skills install --runtime opencode --scope project` produces correct `.opencode/skill/` and `.opencode/command/` with:
   - Flat hyphenated filenames
   - Lowercase tool names in content
   - `~/.config/opencode/` path references
   - `tools:` object format in frontmatter

2. `rulez skills install --runtime gemini --scope project` produces correct `.gemini/skills/` with:
   - Snake_case tool names
   - `~/.gemini/` path references
   - No `color:` fields
   - MCP tools excluded

3. `rulez skills status` shows accurate freshness for all runtimes

4. `rulez skills sync` installs to all detected runtimes in one command

5. All existing CI checks pass (fmt, clippy, test, coverage)

---

## GSD Workflow Integration

**Step 1:** Store the plan at `docs/plans/multi-runtime-skill-portability.md`

**Step 2 (follow-up conversation):** Use GSD skills to create the milestone and phases:

```bash
# Create milestone -- pass the plan as context
/gsd:new-milestone
# Input: v2.3.0 "Multi-Runtime Skill Portability"
# Context: @docs/plans/multi-runtime-skill-portability.md

# For each phase (34-38):
/gsd:plan-phase
# Input: Phase N description from the plan
# Context: @docs/plans/multi-runtime-skill-portability.md

# Execute each phase sequentially:
/gsd:execute-phase
# Phases: 34 -> 35 -> 36 -> 37 -> 38
```

The plan document serves as the **primary context input** for:
- **Milestone creation** (`gsd-roadmapper` agent maps requirements to phases)
- **Phase research** (`gsd-phase-researcher` agent reads the plan for scope)
- **Phase planning** (`gsd-planner` agent uses the plan for task breakdown)
- **Phase execution** (`gsd-executor` agent references the plan for architectural decisions)
