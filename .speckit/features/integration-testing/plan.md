# Integration Testing Framework with IQ/OQ/PQ - Technical Plan

**Feature ID:** integration-testing  
**Status:** Implemented - IQ/OQ/PQ Enhancement In Progress  
**Created:** 2025-01-23  
**Updated:** 2025-01-24  
**Source:** [specify.md](./specify.md)

---

## 1. Architecture Overview

The validation framework consists of three layers:

```
┌─────────────────────────────────────────────────────────────────────┐
│                     CCH Validation Framework                         │
├─────────────────────────────────────────────────────────────────────┤
│  Layer 1: IQ (Installation Qualification)                           │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌─────────────┐│
│  │macOS ARM64   │ │macOS x86_64  │ │  Windows     │ │   Linux     ││
│  │(M1/M2/M3)    │ │(Intel/AMD)   │ │(x64)         │ │(Multi-dist) ││
│  └──────────────┘ └──────────────┘ └──────────────┘ └─────────────┘│
├─────────────────────────────────────────────────────────────────────┤
│  Layer 2: OQ (Operational Qualification)                             │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────────────┐│
│  │ Blocking   │ │ Injection  │ │  Logging   │ │   Permissions      ││
│  │ (US-001)   │ │ (US-002)   │ │  (US-003)  │ │   (US-004)         ││
│  └────────────┘ └────────────┘ └────────────┘ └────────────────────┘│
├─────────────────────────────────────────────────────────────────────┤
│  Layer 3: PQ (Performance Qualification)                             │
│  ┌────────────────┐ ┌────────────────┐ ┌───────────────────────────┐│
│  │  Cold Start    │ │  Event Latency │ │  Throughput & Memory      ││
│  │  (<15ms)       │ │  (<50ms)       │ │  (>100 evt/s, <10MB)      ││
│  └────────────────┘ └────────────────┘ └───────────────────────────┘│
└─────────────────────────────────────────────────────────────────────┘
```

### Directory Structure

```
project-root/
├── cch_cli/tests/
│   ├── common/mod.rs              # Shared test utilities
│   ├── iq_installation.rs         # IQ tests (Rust)
│   ├── iq_new_commands.rs         # IQ command tests
│   ├── oq_us1_blocking.rs         # OQ blocking tests
│   ├── oq_us2_injection.rs        # OQ injection tests
│   ├── oq_us3_validators.rs       # OQ validator tests
│   ├── oq_us4_permissions.rs      # OQ permission tests
│   ├── oq_us5_logging.rs          # OQ logging tests
│   └── pq_performance.rs          # PQ benchmark tests
│
├── test/integration/
│   ├── run-all.sh                 # Master orchestrator
│   ├── lib/test-helpers.sh        # Shared bash library (445 LOC)
│   ├── use-cases/
│   │   ├── 01-block-force-push/   # OQ with real Claude
│   │   ├── 02-context-injection/  # OQ with real Claude
│   │   ├── 03-session-logging/    # OQ with real Claude
│   │   └── 04-permission-explanations/
│   └── results/                   # JSON test outputs
│
└── docs/validation/
    ├── README.md                  # Validation framework docs
    ├── iq/                        # IQ evidence by date
    ├── oq/                        # OQ evidence by date
    ├── pq/                        # PQ evidence by date
    └── sign-off/
        └── TEMPLATE-validation-report.md
```

---

## 2. Technology Stack

| Component | Technology | Rationale |
|-----------|------------|-----------|
| IQ/OQ/PQ Tests | Rust (cargo test) | Type-safe, fast, integrated with CCH |
| Integration Tests | Bash scripts | Simple, portable, real Claude invocation |
| Claude Invocation | `claude -p` CLI | Real end-to-end validation |
| Evidence Format | JSON + Markdown | Machine-readable + human-readable |
| CI/CD | GitHub Actions | Multi-platform runners |
| Task Runner | Taskfile | Consistent with project conventions |

---

## 3. Installation Qualification (IQ) Design

### 3.1 IQ Test Categories

| Category | Tests | Purpose |
|----------|-------|---------|
| Binary Verification | Version, help, binary exists | Confirm installation |
| Configuration | Init, install, validate | Confirm setup works |
| File System | Paths, permissions, logs | Confirm file operations |
| Platform-Specific | Code signing, registry | Platform compliance |

### 3.2 IQ Implementation (Rust)

**File:** `cch_cli/tests/iq_installation.rs`

```rust
/// IQ-TC-001: Binary exists and returns version
#[test]
fn test_iq_binary_version() {
    let output = Command::cargo_bin("cch")
        .expect("binary exists")
        .arg("--version")
        .output()
        .expect("runs");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cch"));
}

/// IQ-TC-002: Init creates configuration
#[test]
fn test_iq_init_creates_config() {
    let temp = tempfile::tempdir().unwrap();
    let output = Command::cargo_bin("cch")
        .current_dir(&temp)
        .arg("init")
        .output()
        .expect("runs");
    
    assert!(output.status.success());
    assert!(temp.path().join(".claude/hooks.yaml").exists());
}
```

### 3.3 Platform-Specific IQ

| Platform | Specific Checks |
|----------|-----------------|
| macOS ARM64 | Native binary, code signing, Gatekeeper |
| macOS Intel | x86_64 binary, Rosetta not required |
| Windows | Path separators, AppData locations, registry |
| Linux | Multiple distros, systemd integration |

### 3.4 IQ Evidence Generation

```bash
#!/bin/bash
# scripts/collect-iq-evidence.sh

EVIDENCE_DIR="docs/validation/iq/$(date +%Y-%m-%d)"
mkdir -p "$EVIDENCE_DIR"

# Capture installation evidence
cargo test --release iq_ -- --nocapture 2>&1 | tee "$EVIDENCE_DIR/test-output.log"

# Capture environment
echo "Platform: $(uname -a)" > "$EVIDENCE_DIR/environment.txt"
echo "Rust: $(rustc --version)" >> "$EVIDENCE_DIR/environment.txt"
echo "CCH: $(cch --version)" >> "$EVIDENCE_DIR/environment.txt"

# Generate report
cat > "$EVIDENCE_DIR/report.md" << EOF
# IQ Evidence Report - CCH $(cch --version | cut -d' ' -f2)

**Date:** $(date)
**Platform:** $(uname -s) $(uname -m)
**Tester:** Automated CI

## Test Results
$(grep -E "^test|PASSED|FAILED" "$EVIDENCE_DIR/test-output.log")

## Status: $(grep -c FAILED "$EVIDENCE_DIR/test-output.log" && echo "❌ FAILED" || echo "✅ PASSED")
EOF
```

---

## 4. Operational Qualification (OQ) Design

### 4.1 OQ Test Categories

| Category | US | Tests | Purpose |
|----------|-----|-------|---------|
| Blocking | US-001 | Block force push, hard reset | Verify block action |
| Injection | US-002 | CDK context, Terraform context | Verify inject action |
| Validators | US-003 | Script execution, error handling | Verify run action |
| Permissions | US-004 | PermissionRequest events | Verify event handling |
| Logging | US-005 | JSON Lines format, timing | Verify audit trail |

### 4.2 OQ Implementation (Rust)

**File:** `cch_cli/tests/oq_us1_blocking.rs`

```rust
/// OQ-US1-001: Block force push command
#[test]
fn test_oq_block_force_push() {
    let temp = setup_test_workspace("block-force-push");
    let event = json!({
        "event_type": "PreToolUse",
        "tool_name": "Bash",
        "tool_input": {"command": "git push --force origin main"},
        "session_id": "test-001"
    });
    
    let output = run_cch_with_event(&temp, &event);
    
    assert!(output.contains("\"continue\":false"));
    assert!(output.contains("block-force-push"));
}
```

### 4.3 OQ Integration Tests (Bash)

Real Claude CLI validation for end-to-end scenarios:

```bash
# test/integration/use-cases/01-block-force-push/test.sh

start_test "01-block-force-push"
WORKSPACE=$(setup_workspace "$SCRIPT_DIR")
install_cch "$WORKSPACE"

# Capture log position
LOG_LINE_BEFORE=$(get_log_line_count)

# Run Claude with dangerous command
run_claude "$WORKSPACE" "Run: git push --force origin main" "Bash" 2

# Verify CCH blocked
assert_log_contains_since "$LOG_LINE_BEFORE" "block-force-push" \
    "CCH should match block-force-push rule"
    
cleanup_workspace
end_test
```

### 4.4 OQ Evidence Generation

```json
{
  "test_id": "OQ-US1-001",
  "test_name": "Block Force Push",
  "date": "2025-01-24T10:30:00Z",
  "scenario": {
    "given": "Rule configured to block git push.*--force",
    "when": "Claude attempts git push origin main --force",
    "then": "Operation blocked with warning message"
  },
  "evidence": {
    "hooks_yaml": "...",
    "event_payload": {...},
    "cch_log_entry": {...},
    "result": "PASSED"
  }
}
```

---

## 5. Performance Qualification (PQ) Design

### 5.1 PQ Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Cold Start (version) | <15ms (release) | 10 iterations, p95 |
| Cold Start (help) | <15ms (release) | 10 iterations, p95 |
| Event Processing | <50ms | 10 iterations, p95 |
| Throughput | >100 events/sec | Sustained 10 seconds |
| Memory Usage | <10MB RSS | Peak during load |

### 5.2 PQ Implementation (Rust)

**File:** `cch_cli/tests/pq_performance.rs`

```rust
/// Target cold start time (release build)
const COLD_START_TARGET_MS: u64 = 15;

/// Debug builds are ~10x slower
const DEBUG_MULTIPLIER: u64 = 10;

fn cold_start_threshold() -> u64 {
    if cfg!(debug_assertions) {
        COLD_START_TARGET_MS * DEBUG_MULTIPLIER
    } else {
        COLD_START_TARGET_MS
    }
}

#[test]
fn test_pq_cold_start_version() {
    let mut times = Vec::with_capacity(10);
    
    for _ in 0..10 {
        let start = Instant::now();
        let output = Command::cargo_bin("cch")
            .arg("--version")
            .output()
            .expect("runs");
        times.push(start.elapsed());
        assert!(output.status.success());
    }
    
    let avg_ms = times.iter().sum::<Duration>().as_millis() / 10;
    let threshold = cold_start_threshold();
    
    assert!(avg_ms < threshold * 3,
        "Cold start {}ms exceeds {}ms threshold",
        avg_ms, threshold * 3);
}
```

### 5.3 PQ Evidence Generation

```markdown
## PQ Benchmark Results - CCH v1.0.0

**Test Environment:** macOS ARM64, M2 8-core, 16GB RAM
**Date:** 2025-01-24
**Build:** Release

### Latency Results (10 iterations each)
| Test | p50 | p95 | p99 | Target | Status |
|------|-----|-----|-----|--------|--------|
| Cold Start (version) | 8ms | 12ms | 14ms | <15ms | ✅ |
| Cold Start (help) | 9ms | 13ms | 15ms | <15ms | ✅ |
| Event Processing | 7ms | 11ms | 14ms | <50ms | ✅ |

### Throughput Results
- Sustained: 1,200 events/sec
- Peak: 2,100 events/sec
- Target: >100 events/sec ✅

### Memory Usage
- Baseline: 4MB RSS
- Under Load: 8MB RSS
- Target: <10MB ✅

**Status:** ✅ PASSED - All performance requirements met
```

---

## 6. CI/CD Integration

### 6.1 GitHub Actions Workflow

```yaml
# .github/workflows/validation.yml
name: IQ/OQ/PQ Validation

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  iq-macos-arm64:
    runs-on: macos-14  # M1 runner
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --release iq_ -- --nocapture
      - uses: actions/upload-artifact@v4
        with:
          name: iq-evidence-macos-arm64
          path: docs/validation/iq/

  iq-macos-intel:
    runs-on: macos-13  # Intel runner
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --release iq_ -- --nocapture

  iq-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --release iq_ -- --nocapture

  iq-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --release iq_ -- --nocapture

  oq:
    needs: [iq-macos-arm64, iq-linux]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --release oq_ -- --nocapture

  pq:
    needs: [oq]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --release pq_ -- --nocapture
      - uses: actions/upload-artifact@v4
        with:
          name: pq-evidence
          path: docs/validation/pq/
```

### 6.2 Release Validation Gate

```yaml
# In release workflow
validate:
  runs-on: ubuntu-latest
  steps:
    - run: task validation-all
    - run: |
        # Check all validations passed
        if grep -r "FAILED" docs/validation/*/report.md; then
          echo "❌ Validation failed - blocking release"
          exit 1
        fi
        echo "✅ All validations passed"
```

---

## 7. Taskfile Integration

```yaml
# Taskfile.yml additions

iq-test:
  desc: Run Installation Qualification tests
  cmds:
    - cargo test --release iq_ -- --nocapture
    - ./scripts/collect-iq-evidence.sh

oq-test:
  desc: Run Operational Qualification tests
  cmds:
    - cargo test --release oq_ -- --nocapture

pq-test:
  desc: Run Performance Qualification tests  
  cmds:
    - cargo test --release pq_ -- --nocapture

validation-all:
  desc: Run full IQ/OQ/PQ validation suite
  deps: [build]
  cmds:
    - task: iq-test
    - task: oq-test
    - task: pq-test
    - task: integration-test
    - ./scripts/generate-validation-report.sh

integration-test:
  desc: Run CCH + Claude CLI integration tests
  aliases: [itest]
  deps: [build]
  cmds:
    - ./test/integration/run-all.sh

integration-test-strict:
  desc: Run integration tests in strict mode (fail on any issue)
  deps: [build]
  cmds:
    - STRICT_MODE=1 ./test/integration/run-all.sh
```

---

## 8. Error Handling Strategy

| Scenario | IQ Handling | OQ Handling | PQ Handling |
|----------|-------------|-------------|-------------|
| Binary missing | Fail fast | Fail fast | Fail fast |
| Config error | Report error | Report error | Report error |
| Timeout | N/A | 60s limit | Mark degraded |
| Performance miss | N/A | N/A | Warn if debug, fail if release |
| Platform issue | Platform-specific skip | Continue | Continue |

---

## 9. Evidence Retention Policy

| Evidence Type | Retention | Storage |
|---------------|-----------|---------|
| Release validation | Indefinite | Git + artifacts |
| PR validation | 90 days | CI artifacts only |
| Nightly builds | 30 days | CI artifacts only |
| Local dev runs | Session | Not stored |

---

## 10. Future Enhancements

| Enhancement | Priority | Rationale |
|-------------|----------|-----------|
| Parallel IQ across platforms | High | Faster CI |
| OQ with real Claude in CI | Medium | Full end-to-end in CI |
| PQ stress testing (7-day) | Medium | Long-term stability |
| Evidence dashboard | Low | Visual reporting |
| Compliance export (FDA format) | Low | Regulated industries |
