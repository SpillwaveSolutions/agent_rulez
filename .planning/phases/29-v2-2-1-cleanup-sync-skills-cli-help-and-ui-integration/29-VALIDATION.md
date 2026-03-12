---
phase: 29
slug: v2-2-1-cleanup-sync-skills-cli-help-and-ui-integration
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-12
---

# Phase 29 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test (Rust) + grep/diff verification (docs/skills) |
| **Config file** | Cargo.toml |
| **Quick run command** | `cargo test --tests --all-features --workspace` |
| **Full suite command** | `cargo clippy --all-targets --all-features --workspace -- -D warnings && cargo test --tests --all-features --workspace` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test --tests --all-features --workspace`
- **After every plan wave:** Run full suite command
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 29-01-01 | 01 | 1 | Stale cch refs | grep | `grep -r "cch" .claude/skills/release-cch/ --include="*.md" --include="*.sh" \| wc -l` | ✅ | ⬜ pending |
| 29-01-02 | 01 | 1 | preflight-check.sh | grep | `grep "cch_cli" .claude/skills/release-cch/preflight-check.sh \| wc -l` | ✅ | ⬜ pending |
| 29-02-01 | 02 | 1 | CLI docs sync | diff | `diff <(cargo run -- --help 2>&1) /dev/null` | ✅ | ⬜ pending |
| 29-03-01 | 03 | 2 | UI routing | grep | `grep -r "ConfigDiffView\|config-diff" rulez-ui/src/ \| wc -l` | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

*Existing infrastructure covers all phase requirements.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| UI ConfigDiffView accessible | UI routing | Visual verification | Launch app, verify "Diff" button appears in header and opens diff view |
| Skill loads correctly | Skill rename | Claude Code skill loading | Run `/release-cch` (or renamed) and verify it loads |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
