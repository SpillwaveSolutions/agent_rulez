---
created: 2026-03-05T21:28:15.168Z
title: Auto-check and upgrade RuleZ binary to latest release
area: tooling
github_issue: https://github.com/SpillwaveSolutions/agent_rulez/issues/102
files:
  - mastering-hooks/SKILL.md
  - rulez/src/cli/
---

## Problem

When users invoke the RuleZ skill, it happily uses whatever binary version is installed locally — even if it's outdated. There's no mechanism to check whether the installed binary matches the latest GitHub release or to offer an upgrade path.

Users end up running stale versions without realizing it, missing bug fixes and new features.

## Solution

Add a version check step to the skill workflow that:

1. Checks the currently installed `rulez` binary version (`rulez --version`)
2. Queries GitHub releases via `gh release list --repo SpillwaveSolutions/agent_rulez --limit 5`
3. Compares versions — if outdated, prompts user with options:
   - Install latest release
   - Pick from last 5 releases
   - Stay on current version
4. Auto-detects platform (macOS ARM64, macOS x86, Linux) and selects correct asset
5. Downloads and replaces the binary at the detected path (e.g., `/Users/richardhightower/bin/rulez`)

Download pattern:
```bash
cd /tmp && \
gh release download <tag> --repo SpillwaveSolutions/agent_rulez --pattern '<asset>.tar.gz' && \
tar xzf <asset>.tar.gz && \
cp rulez <install-path>
```

Could be implemented as:
- A `rulez upgrade` CLI subcommand, or
- A check in the mastering-hooks skill that runs on init
