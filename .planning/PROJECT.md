# CCH (Claude Context Hooks)

## What This Is

CCH is a **first-class, auditable, local AI policy engine** for Claude Code. It intercepts Claude's tool invocations via hooks and applies user-defined YAML rules to block dangerous operations, inject helpful context, and maintain comprehensive audit trails.

The project encompasses:
1. **CCH Core** (v1.0.0 Released) - Rust-based policy engine binary
2. **Phase 2 Governance** (Complete) - Policy modes, metadata, priorities, enterprise features
3. **RuleZ UI** (In Progress) - Tauri desktop application for visual configuration

## Core Value

**LLMs do not enforce policy. LLMs are subject to policy.**

CCH positions itself as comparable to:
- OPA (but human-readable)
- Terraform Sentinel (but local)
- Kubernetes admission controllers (but for AI agents)

## Design Philosophy

- **Zero unsafe Rust code** - Memory safety guaranteed
- **Sub-10ms processing** - Hook events processed in under 10ms
- **Configuration-driven** - All behavior defined by user YAML, not hardcoded logic
- **Complete audit trail** - All decisions logged in JSON Lines format
- **No network access** - Pure local processing for security
- **No telemetry** - User privacy is paramount

## Current State

### Completed Features
- **cch-binary-v1** - Core policy engine (64+ tests, <3ms latency)
- **enhanced-logging** - Structured event details, debug mode
- **mastering-hooks** - Claude Code skill for CCH mastery
- **phase2-governance** - Policy modes (enforce/warn/audit), priority, metadata

### In Progress
- **rulez-ui** - Tauri desktop app (M1 complete, M2-M8 pending)

### Backlog
- **cch-advanced-rules** - Conditional matchers, inline injection
- **integration-testing** - IQ/OQ/PQ validation framework

## Technology Stack

### CCH Core (Rust)
- Rust 2021 edition, tokio async runtime
- serde (JSON/YAML), clap (CLI), regex (patterns)
- tracing (structured logging), chrono (time)

### RuleZ UI (Desktop)
- Tauri 2.0 (Rust backend + WebView)
- React 18 + TypeScript 5.7+
- Monaco Editor + monaco-yaml
- Tailwind CSS 4, Zustand, Bun

## Quality Gates

- Cold start: <5ms p95
- Rule matching: <1ms for 100 rules
- Memory: <50MB resident
- RuleZ UI launch: <2 seconds
- Editor input latency: <16ms (60fps)

## Git Workflow

- `main` - Production-ready, fully validated
- `develop` - Integration branch, fast CI
- `feature/*` - Short-lived working branches
- Never commit directly to main or develop

---

*Converted from .speckit/constitution.md on 2026-02-06*
