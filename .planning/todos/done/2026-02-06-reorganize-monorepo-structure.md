---
created: 2026-02-06T15:01
title: Reorganize codebase as proper monorepo
area: architecture
files:
  - cch_cli/src/
  - rulez_ui/
  - mastering-hooks/
  - Cargo.toml
---

## Problem

The codebase is meant to be a monorepo with three main components:

1. **CCH Core Binary** (Rust) - The primary product: manages hook callbacks, rule evaluation, policy enforcement
   - Located in `cch_cli/src/` with modules: cli, config, hooks, logging, models

2. **Mastering Hooks Skill** - Claude Code skill for CCH mastery (should become a plugin)
   - Located in `mastering-hooks/`

3. **RuleZ UI** (Tauri desktop app) - Visual configuration tool
   - Located in `rulez_ui/`
   - Currently has disproportionate focus despite being least critical component

Current structure doesn't clearly communicate the component hierarchy:
- CCH binary is the core product
- RuleZ UI is a supporting tool
- Mastering hooks is a skill/plugin

The roadmap incorrectly focuses on RuleZ UI phases when the core binary and skill/plugin ecosystem should be primary.

## Solution

1. Reorganize directory structure to reflect proper monorepo:
   ```
   packages/
     cch-core/        # Rust binary (primary)
     cch-skill/       # Mastering hooks skill → plugin
     rulez-ui/        # Desktop app (supporting)
   ```

2. Update Cargo.toml workspace configuration

3. Reassess roadmap priorities:
   - CCH Core enhancements first
   - Skill → Plugin conversion
   - RuleZ UI as lower priority

4. Ensure mastering-hooks becomes a proper plugin (if not already)

TBD: Exact directory naming and migration strategy
