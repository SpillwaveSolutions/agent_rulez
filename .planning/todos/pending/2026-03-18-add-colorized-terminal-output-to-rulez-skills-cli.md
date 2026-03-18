---
created: 2026-03-18T05:01:11.076Z
title: Add colorized terminal output to rulez skills CLI
area: cli
files:
  - rulez/src/cli/skills.rs
  - rulez/Cargo.toml
---

## Problem

`rulez skills` CLI (install, clean, sync, status, diff) uses plain `println!()` throughout with no color or progress indicators. Requirement DX-04 from v2.3.0 (Multi-Runtime Skill Portability) was left unimplemented — shipped as tech debt.

Specific gaps found during v2.3.0 milestone audit:
- `diff()` outputs `"  M ..."` / `"  + ..."` as plain ASCII — DX-02 requires colored diff (M=yellow, +=green)
- `status()` outputs a plain text table — installed rows should be green, missing/stale rows red
- `install()` and `sync()` emit a single `println!` after completion — no live progress during multi-file writes
- No color library in `Cargo.toml` (checked: `indicatif`, `console`, `termcolor`, `owo-colors`, `yansi` — none present)

## Solution

Add `owo-colors` (zero-cost, no_std compatible) or `console` to `rulez/Cargo.toml`.

In `rulez/src/cli/skills.rs`:
- `diff()`: color `M` prefix yellow, `+` prefix green, `-` prefix red
- `status()`: color installed runtime rows green, not-installed rows dimmed/red, stale rows yellow
- `install()`: print per-file progress as files are written (e.g., `  writing skill.md → ~/.config/opencode/skills/skill.md`)
- `sync()`: show per-runtime header as each runtime is processed

Keep changes self-contained to `skills.rs` and `Cargo.toml`. No architectural changes needed.
