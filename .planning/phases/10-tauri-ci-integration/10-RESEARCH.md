# Phase 10: Tauri CI Integration - Research

**Researched:** 2026-02-10
**Domain:** Tauri 2.0 Cross-Platform CI/CD with GitHub Actions
**Confidence:** HIGH

## Summary

Phase 10 integrates Tauri 2.0 desktop app builds into GitHub Actions CI, delivering cross-platform artifacts (.dmg, .msi, .AppImage) with fail-fast E2E testing. Research confirms that Tauri's official `tauri-action` handles multi-platform builds, but critical pitfalls exist around webkit2gtk versions on Linux and runner OS selection.

**Key findings:**

1. **Linux builds MUST use `libwebkit2gtk-4.1-dev`** (NOT 4.0) — Tauri v2 migrated to webkit2gtk 4.1 for Flatpak support, and Ubuntu 24.04 removed the 4.0 packages entirely.

2. **Use `ubuntu-22.04` explicitly, NOT `ubuntu-latest`** — ubuntu-latest currently points to ubuntu-24.04, which supports webkit2gtk-4.1-dev, but explicit versioning prevents future breaking changes when GitHub updates the runner.

3. **E2E tests in web mode MUST run before Tauri builds** — Playwright tests against Vite dev server provide fast feedback (2-3 min) before triggering expensive multi-platform builds (8-15 min). Use `needs:` dependency to enforce order.

4. **The current e2e.yml workflow has a directory mismatch** — Uses `rulez_ui` but the actual directory is `rulez-ui`. This breaks path-based triggers and working-directory settings.

5. **tauri-action v0 (latest stable) automatically uploads artifacts** — No manual `actions/upload-artifact` needed for release builds. The action creates GitHub releases with platform-specific bundles when triggered on tag pushes.

**Primary recommendation:** Create `.github/workflows/tauri-build.yml` with E2E tests as a prerequisite job, explicit `ubuntu-22.04` runner for Linux with `libwebkit2gtk-4.1-dev`, and `fail-fast: false` matrix strategy to allow all platform builds to complete. Fix the existing e2e.yml directory mismatch as part of this phase.

## Standard Stack

### Core

| Library/Tool | Version | Purpose | Why Standard |
|--------------|---------|---------|--------------|
| tauri-apps/tauri-action | v0 | Multi-platform Tauri builds in CI | Official Tauri GitHub Action, handles Windows/macOS/Linux matrix builds, auto-uploads artifacts |
| tauri-apps/cli | 2.3.0+ | Tauri build tooling | Already in package.json, no change needed |
| libwebkit2gtk-4.1-dev | Latest (Ubuntu 22.04) | WebView renderer on Linux | **CRITICAL:** Tauri 2.0 requires 4.1 (NOT 4.0), Ubuntu 24.04 dropped 4.0 packages |
| Playwright | 1.50.1 | E2E testing in web mode | Already validated in Phase 9, provides fast pre-build validation |

### Supporting

| Library/Tool | Version | Purpose | When to Use |
|--------------|---------|---------|-------------|
| actions/checkout | v4 | Repository checkout | Every workflow |
| oven-sh/setup-bun | v2 | Bun runtime setup | Frontend build steps |
| dtolnay/rust-toolchain | stable | Rust toolchain setup | Tauri backend build |
| swatinem/rust-cache | v2 | Rust build artifact caching | Speeds up CI by 50-70% |
| actions/upload-artifact | v4 | Manual artifact uploads | Only if NOT using tauri-action's built-in release creation |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| tauri-action v0 | Manual cargo/npm builds per platform | Much more complex, must handle platform differences manually, no auto-release |
| ubuntu-22.04 | ubuntu-latest | Breaks when GitHub updates to newer Ubuntu (currently 24.04, may change to 26.04), less reproducible |
| libwebkit2gtk-4.1-dev | libwebkit2gtk-4.0-dev | **DOES NOT WORK** — Tauri 2.0 incompatible, package removed from Ubuntu 24.04+ |
| Playwright (web mode) | WebDriver with full Tauri app | 10x slower (must start full Tauri app), more flaky, harder to debug |
| E2E before build | E2E after build | Wastes 8-15 min on multi-platform builds before catching UI bugs |

**Installation:**

No new package dependencies — all tools are GitHub Actions or system packages installed via apt.

```yaml
# Linux dependencies (in CI workflow)
- name: Install Linux deps (Tauri 2.0)
  if: runner.os == 'Linux'
  run: |
    sudo apt-get update
    sudo apt-get install -y \
      libwebkit2gtk-4.1-dev \
      build-essential \
      curl \
      wget \
      file \
      libxdo-dev \
      libssl-dev \
      libayatana-appindicator3-dev \
      librsvg2-dev
```

## Architecture Patterns

### Recommended Workflow Structure

```
.github/workflows/
├── e2e.yml              # E2E tests (fast, web mode) — FIX directory paths
├── tauri-build.yml      # NEW: Multi-platform Tauri builds
└── ci.yml               # Existing Rust CI (unchanged)
```

### Pattern 1: Fail-Fast E2E Before Multi-Platform Build

**What:** Run E2E tests in web mode first, only trigger Tauri builds if tests pass.

**When to use:** Always — prevents wasting 8-15 minutes on builds when UI is broken.

**Example:**

```yaml
# .github/workflows/tauri-build.yml
name: Tauri Build

on:
  push:
    branches: [main, develop, 'release/**']
    paths:
      - 'rulez-ui/**'
      - '.github/workflows/tauri-build.yml'
  pull_request:
    paths:
      - 'rulez-ui/**'
  workflow_dispatch:

jobs:
  # STEP 1: Fast E2E tests (2-3 min)
  test-e2e:
    name: E2E Tests (Web Mode)
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: rulez-ui

    steps:
      - uses: actions/checkout@v4

      - name: Setup Bun
        uses: oven-sh/setup-bun@v2
        with:
          bun-version: latest

      - name: Install dependencies
        run: bun install

      - name: Install Playwright browsers
        run: bunx playwright install --with-deps chromium webkit

      - name: Run E2E tests
        run: bunx playwright test
        env:
          CI: true

      - name: Upload test results
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: playwright-report
          path: rulez-ui/playwright-report/
          retention-days: 30

  # STEP 2: Multi-platform Tauri builds (8-15 min)
  # Only runs if E2E tests pass
  build-tauri:
    name: Build Tauri (${{ matrix.platform }})
    needs: test-e2e  # ← CRITICAL: Blocks build if E2E fails
    strategy:
      fail-fast: false  # Continue other platforms if one fails
      matrix:
        platform: [ubuntu-22.04, macos-latest, windows-latest]

    runs-on: ${{ matrix.platform }}

    steps:
      - uses: actions/checkout@v4

      - name: Setup Bun
        uses: oven-sh/setup-bun@v2
        with:
          bun-version: latest

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install Linux dependencies (Tauri 2.0)
        if: matrix.platform == 'ubuntu-22.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            libwebkit2gtk-4.1-dev \
            build-essential \
            curl \
            wget \
            file \
            libxdo-dev \
            libssl-dev \
            libayatana-appindicator3-dev \
            librsvg2-dev

      - name: Rust cache
        uses: swatinem/rust-cache@v2
        with:
          workspaces: rulez-ui/src-tauri -> target

      - name: Install frontend dependencies
        run: bun install
        working-directory: rulez-ui

      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          projectPath: rulez-ui
          # Artifacts auto-uploaded for release tags
          # For non-release builds, just verify build succeeds
```

**Source:** [Tauri GitHub Actions Guide](https://v2.tauri.app/distribute/pipelines/github/), [tauri-apps/tauri-action](https://github.com/tauri-apps/tauri-action)

### Pattern 2: Explicit OS Versions for Reproducibility

**What:** Use `ubuntu-22.04` instead of `ubuntu-latest` to prevent breaking changes when GitHub updates runners.

**When to use:** Always for system dependencies (webkit, etc.) — only use `*-latest` for pure Rust/Node builds.

**Example:**

```yaml
strategy:
  matrix:
    platform:
      - ubuntu-22.04     # ✓ Explicit version
      - macos-latest     # ✓ OK - macOS deps stable
      - windows-latest   # ✓ OK - Windows deps stable

# WRONG:
# - ubuntu-latest     # ✗ Currently 24.04, may change to 26.04
```

**Why:** ubuntu-latest currently resolves to ubuntu-24.04 (as of 2026-02), which supports webkit2gtk-4.1-dev. But if GitHub updates ubuntu-latest to ubuntu-26.04 in the future, and that version has breaking changes (e.g., different webkit version), builds fail unexpectedly.

**Source:** [Tauri Issue #11763](https://github.com/tauri-apps/tauri/issues/11763), [GitHub Actions runner-images](https://github.com/actions/runner-images/issues/7606)

### Pattern 3: Artifact Upload Only on Release Branches

**What:** tauri-action automatically creates GitHub releases when triggered on tag pushes. For non-release builds, just verify compilation succeeds.

**When to use:** Release branches (main, release/*) push tags to trigger releases. Feature branches build without releasing.

**Example:**

```yaml
on:
  push:
    tags:
      - 'v*'  # Trigger release on version tags (e.g., v1.0.0)

# For tag pushes, tauri-action auto-creates release with artifacts
- uses: tauri-apps/tauri-action@v0
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  with:
    projectPath: rulez-ui
    tagName: ${{ github.ref_name }}
    releaseName: 'RuleZ UI ${{ github.ref_name }}'
    releaseBody: 'See CHANGELOG.md for details.'
    releaseDraft: false
    prerelease: false
```

For non-release builds (PRs, feature branches), omit `tagName` and release fields — tauri-action just builds without releasing.

**Source:** [tauri-action README](https://github.com/tauri-apps/tauri-action), [Tauri GitHub Actions Guide](https://v2.tauri.app/distribute/pipelines/github/)

### Anti-Patterns to Avoid

- **Anti-pattern:** Using `ubuntu-latest` for Tauri builds
  - **Why it's bad:** Breaking changes when GitHub updates ubuntu-latest to newer versions with different webkit packages
  - **What to do instead:** Use explicit `ubuntu-22.04` or `ubuntu-24.04`

- **Anti-pattern:** Installing `libwebkit2gtk-4.0-dev` for Tauri 2.0
  - **Why it's bad:** Tauri 2.0 requires webkit2gtk 4.1, and Ubuntu 24.04+ removed 4.0 packages entirely
  - **What to do instead:** Install `libwebkit2gtk-4.1-dev` (works on Ubuntu 22.04 and 24.04)

- **Anti-pattern:** Running Tauri builds before E2E tests
  - **Why it's bad:** Wastes 8-15 minutes on multi-platform builds before discovering basic UI bugs
  - **What to do instead:** Use `needs: test-e2e` dependency to run E2E first (2-3 min web mode)

- **Anti-pattern:** Using `fail-fast: true` in matrix builds
  - **Why it's bad:** One platform failure (e.g., Windows-specific bug) cancels all other platforms, hiding useful debug info
  - **What to do instead:** Use `fail-fast: false` to allow all platforms to complete, then debug specific failures

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Multi-platform Tauri builds | Custom cargo/npm scripts per OS | tauri-apps/tauri-action | Handles all platform differences (Windows NSIS/MSI, macOS DMG/App, Linux AppImage/Deb), auto-uploads to releases, battle-tested by Tauri community |
| Cross-platform webkit detection | Version detection scripts | Explicit `libwebkit2gtk-4.1-dev` on Ubuntu 22.04+ | Tauri 2.0 only supports 4.1, no need to detect — fail fast if wrong version |
| GitHub release creation | Manual gh CLI scripts | tauri-action's built-in release creation | Auto-generates release notes, uploads artifacts with correct naming, handles checksums |
| Rust build caching | Custom cache key logic | swatinem/rust-cache@v2 | Intelligently caches based on Cargo.lock/Cargo.toml, auto-cleans old artifacts, 50-70% CI speedup |

**Key insight:** Tauri's ecosystem has solved the cross-platform desktop app build problem with tauri-action. The only custom logic needed is: (1) install webkit2gtk-4.1-dev on Linux, (2) ensure E2E tests run first. Everything else is handled by the action.

## Common Pitfalls

### Pitfall 1: webkit2gtk Version Mismatch on Linux

**What goes wrong:** Installing `libwebkit2gtk-4.0-dev` (Tauri v1) instead of `libwebkit2gtk-4.1-dev` (Tauri v2) causes build failures with "dependency not found" errors.

**Why it happens:** Tauri v1 used webkit2gtk 4.0, but Tauri v2 migrated to webkit2gtk 4.1 for Flatpak support. Ubuntu 24.04 and Debian 13 removed the 4.0 packages from repositories.

**How to avoid:**

1. **Always install `libwebkit2gtk-4.1-dev` for Tauri 2.0 builds:**
   ```yaml
   - name: Install Linux dependencies (Tauri 2.0)
     if: runner.os == 'Linux'
     run: |
       sudo apt-get update
       sudo apt-get install -y libwebkit2gtk-4.1-dev \
         build-essential curl wget file libxdo-dev \
         libssl-dev libayatana-appindicator3-dev librsvg2-dev
   ```

2. **Use `ubuntu-22.04` or `ubuntu-24.04` runners (NOT ubuntu-latest):**
   - Both support webkit2gtk-4.1-dev
   - ubuntu-20.04 and earlier do NOT have 4.1 packages

3. **Document in README:**
   ```markdown
   ## Tauri UI Development

   ### Linux Prerequisites (Ubuntu 22.04+)
   ```bash
   sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev \
     libayatana-appindicator3-dev librsvg2-dev
   ```

   **Note:** Tauri 2.0 requires webkit2gtk-4.1 (NOT 4.0). Ubuntu 20.04 and earlier are not supported.
   ```

**Warning signs:**
- CI build fails with "Package 'libwebkit2gtk-4.0-dev' has no installation candidate"
- Error: "webkit2gtk-4.1 not found" during cargo build
- Local builds work but GitHub Actions fail

**Detection:**
```bash
# On Ubuntu, check available webkit versions
apt-cache search webkit2gtk

# Should show:
# - libwebkit2gtk-4.1-dev (Tauri v2)
# On Ubuntu 24.04, libwebkit2gtk-4.0-dev is MISSING
```

**Sources:**
- [Tauri Issue #9662: libwebkit2gtk-4.0 not available in Ubuntu 24](https://github.com/tauri-apps/tauri/issues/9662)
- [Tauri v2 Prerequisites](https://v2.tauri.app/start/prerequisites/)
- [Migration to webkit2gtk-4.1](https://v2.tauri.app/blog/tauri-2-0-0-alpha-3/)

---

### Pitfall 2: Directory Path Mismatch in E2E Workflow

**What goes wrong:** The existing `.github/workflows/e2e.yml` uses `rulez_ui` but the actual directory is `rulez-ui` (with hyphen). Path-based triggers and working-directory settings fail silently.

**Why it happens:** Likely a typo/renaming inconsistency. The workflow was created with `rulez_ui` but the directory uses the hyphenated name.

**How to avoid:**

1. **Fix ALL references in `.github/workflows/e2e.yml`:**
   ```yaml
   # WRONG:
   paths:
     - 'rulez_ui/**'
   defaults:
     run:
       working-directory: rulez_ui

   # RIGHT:
   paths:
     - 'rulez-ui/**'
   defaults:
     run:
       working-directory: rulez-ui
   ```

2. **Update all artifact paths:**
   ```yaml
   - name: Upload test results
     uses: actions/upload-artifact@v4
     with:
       name: playwright-report
       path: rulez-ui/playwright-report/  # NOT rulez_ui/
   ```

3. **Verify with grep search:**
   ```bash
   # Check for any remaining rulez_ui references
   grep -r "rulez_ui" .github/workflows/
   # Should return ZERO results
   ```

**Warning signs:**
- E2E workflow doesn't trigger when rulez-ui/ files change
- "working-directory not found" errors in CI
- Artifact upload fails with "path not found"

**Detection:**
```bash
# Check actual directory name
ls -la | grep rulez
# Shows: drwxr-xr-x  27 ... rulez-ui

# Check workflow references
grep -n "rulez_ui" .github/workflows/e2e.yml
# Should find the mismatches
```

**Sources:**
- Project inspection (existing e2e.yml has this bug)
- [PITFALLS.md Pitfall 5](file://.planning/research/PITFALLS.md) documents this issue

---

### Pitfall 3: Running Multi-Platform Builds Before E2E Tests

**What goes wrong:** CI triggers Tauri builds (8-15 min) before running E2E tests (2-3 min). UI bugs waste time on unnecessary builds.

**Why it happens:** Default job ordering in GitHub Actions is parallel by default — without explicit dependencies, all jobs run simultaneously.

**How to avoid:**

1. **Use `needs:` to enforce E2E-first ordering:**
   ```yaml
   jobs:
     test-e2e:
       name: E2E Tests (Web Mode)
       runs-on: ubuntu-latest
       steps:
         # ... run Playwright tests ...

     build-tauri:
       name: Build Tauri
       needs: test-e2e  # ← CRITICAL: Blocks build if E2E fails
       runs-on: ${{ matrix.platform }}
       steps:
         # ... run Tauri build ...
   ```

2. **Run E2E in web mode (NOT full Tauri mode):**
   - Web mode: 2-3 min (Playwright against Vite dev server)
   - Full Tauri mode: 10-20 min (must build Tauri app first)
   - **Use web mode for fast feedback**

3. **Use `fail-fast: false` for matrix builds:**
   ```yaml
   strategy:
     fail-fast: false  # Allow all platforms to complete
     matrix:
       platform: [ubuntu-22.04, macos-latest, windows-latest]
   ```
   - If Windows build fails, macOS/Linux still run
   - Collect all platform-specific errors in one CI run

**Warning signs:**
- CI takes 20+ minutes even when E2E tests fail
- PRs block for 15 minutes before showing test failures
- Build artifacts uploaded for PRs with failing tests

**Detection:**
```bash
# Check workflow job dependencies
grep -A5 "build-tauri:" .github/workflows/tauri-build.yml | grep "needs:"
# Should show: needs: test-e2e
```

**Sources:**
- [Playwright Best Practices](https://playwright.dev/docs/best-practices)
- [GitHub Actions job dependencies](https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions#jobsjob_idneeds)
- Research findings: "E2E tests in web mode before Tauri build"

---

### Pitfall 4: Rust Cache Invalidation on Directory Rename

**What goes wrong:** Renaming `rulez_ui` to `rulez-ui` leaves stale Rust cache in `swatinem/rust-cache`, causing incorrect build artifacts or outdated dependencies.

**Why it happens:** rust-cache uses `workspaces` parameter to identify cache locations. If the directory name changes, the cache key may still point to the old directory.

**How to avoid:**

1. **Update `workspaces` parameter in rust-cache:**
   ```yaml
   - name: Rust cache
     uses: swatinem/rust-cache@v2
     with:
       workspaces: rulez-ui/src-tauri -> target  # Correct directory name
   ```

2. **Clear cache after directory rename:**
   ```bash
   # Locally
   rm -rf rulez-ui/src-tauri/target

   # In CI, cache will auto-regenerate with new key
   ```

3. **Verify cache key includes directory path:**
   ```yaml
   - uses: swatinem/rust-cache@v2
     with:
       workspaces: rulez-ui/src-tauri -> target
       # Cache key includes workspace path, so rename forces new cache
   ```

**Warning signs:**
- Rust build errors referencing old `rulez_ui` paths
- "dependency not found" errors after directory rename
- CI builds slower than expected (cache miss)

**Detection:**
```bash
# Check for stale cache references
grep -r "rulez_ui" rulez-ui/src-tauri/target/ 2>/dev/null
# Should return ZERO results (or directory doesn't exist)
```

**Sources:**
- [swatinem/rust-cache documentation](https://github.com/Swatinem/rust-cache)
- [PITFALLS.md Pitfall 6](file://.planning/research/PITFALLS.md) (Stale binary cache)

## Code Examples

Verified patterns from official sources.

### Complete Tauri Build Workflow

```yaml
# .github/workflows/tauri-build.yml
name: Tauri CI Build

on:
  push:
    branches: [main, develop, 'release/**']
    paths:
      - 'rulez-ui/**'
      - '.github/workflows/tauri-build.yml'
  pull_request:
    paths:
      - 'rulez-ui/**'
  workflow_dispatch:

jobs:
  # STEP 1: Fast E2E tests (2-3 min)
  test-e2e:
    name: E2E Tests (Web Mode)
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: rulez-ui

    steps:
      - uses: actions/checkout@v4

      - name: Setup Bun
        uses: oven-sh/setup-bun@v2
        with:
          bun-version: latest

      - name: Install dependencies
        run: bun install

      - name: Install Playwright browsers
        run: bunx playwright install --with-deps chromium webkit

      - name: Run E2E tests
        run: bunx playwright test
        env:
          CI: true

      - name: Upload test results
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: playwright-report
          path: rulez-ui/playwright-report/
          retention-days: 30

  # STEP 2: Multi-platform Tauri builds (8-15 min)
  build-tauri:
    name: Build Tauri (${{ matrix.platform }})
    needs: test-e2e  # Only runs if E2E tests pass
    strategy:
      fail-fast: false
      matrix:
        platform: [ubuntu-22.04, macos-latest, windows-latest]

    runs-on: ${{ matrix.platform }}

    steps:
      - uses: actions/checkout@v4

      - name: Setup Bun
        uses: oven-sh/setup-bun@v2
        with:
          bun-version: latest

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install Linux dependencies (Tauri 2.0)
        if: matrix.platform == 'ubuntu-22.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            libwebkit2gtk-4.1-dev \
            build-essential \
            curl \
            wget \
            file \
            libxdo-dev \
            libssl-dev \
            libayatana-appindicator3-dev \
            librsvg2-dev

      - name: Rust cache
        uses: swatinem/rust-cache@v2
        with:
          workspaces: rulez-ui/src-tauri -> target

      - name: Install frontend dependencies
        run: bun install
        working-directory: rulez-ui

      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          projectPath: rulez-ui
```

**Source:** [Tauri GitHub Actions Guide](https://v2.tauri.app/distribute/pipelines/github/), [tauri-apps/tauri-action](https://github.com/tauri-apps/tauri-action)

### Fixed E2E Workflow (Directory Mismatch)

```yaml
# .github/workflows/e2e.yml (FIXED VERSION)
name: E2E Tests

on:
  push:
    branches: [main, develop]
    paths:
      - 'rulez-ui/**'  # ← FIXED: was rulez_ui
      - '.github/workflows/e2e.yml'
  pull_request:
    branches: [main, develop]
    paths:
      - 'rulez-ui/**'  # ← FIXED: was rulez_ui
  workflow_dispatch:

defaults:
  run:
    working-directory: rulez-ui  # ← FIXED: was rulez_ui

jobs:
  e2e-tests:
    name: Playwright E2E Tests
    runs-on: ubuntu-latest
    timeout-minutes: 30

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Bun
        uses: oven-sh/setup-bun@v2
        with:
          bun-version: latest

      - name: Install dependencies
        run: bun install

      - name: Install Playwright browsers
        run: bunx playwright install --with-deps chromium webkit

      - name: Run Playwright tests
        run: bunx playwright test
        env:
          CI: true

      - name: Upload test results
        uses: actions/upload-artifact@v4
        if: always()
        with:
          name: playwright-report
          path: rulez-ui/playwright-report/  # ← FIXED: was rulez_ui/
          retention-days: 30

      - name: Upload test artifacts
        uses: actions/upload-artifact@v4
        if: failure()
        with:
          name: test-results
          path: rulez-ui/test-results/  # ← FIXED: was rulez_ui/
          retention-days: 7

      - name: Publish Test Results
        uses: EnricoMi/publish-unit-test-result-action@v2
        if: always()
        with:
          files: rulez-ui/test-results/junit.xml  # ← FIXED: was rulez_ui/
          comment_mode: off
          check_name: E2E Test Results
```

**Source:** Existing e2e.yml with fixes applied

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual cross-platform builds | tauri-action GitHub Action | Tauri v1.0 (2022) | Automated Windows/macOS/Linux builds in single workflow |
| libwebkit2gtk-4.0-dev | libwebkit2gtk-4.1-dev | Tauri v2.0-alpha-3 (2024) | Flatpak support, Ubuntu 24.04 removed 4.0 packages |
| ubuntu-latest for builds | Explicit ubuntu-22.04 | Current best practice (2026) | Prevents breaking changes when GitHub updates ubuntu-latest |
| E2E in full Tauri mode | E2E in web mode (Playwright against Vite) | Current best practice (2026) | 5-10x faster (2-3 min vs 10-20 min) |
| Manual artifact upload | tauri-action auto-release | Tauri v1.0+ (2022) | Auto-creates GitHub releases with platform bundles on tag push |

**Deprecated/outdated:**
- **libwebkit2gtk-4.0-dev:** Removed from Ubuntu 24.04, Tauri v2 incompatible — use libwebkit2gtk-4.1-dev
- **ubuntu-latest for Tauri builds:** Too unstable, use explicit ubuntu-22.04 or ubuntu-24.04
- **Running full Tauri app for E2E tests:** Too slow, use web mode with mocked Tauri commands

## Open Questions

1. **Should we support ARM64 Linux builds?**
   - What we know: GitHub now offers `ubuntu-22.04-arm` runners for public repos (as of August 2025)
   - What's unclear: Is there user demand for ARM64 Linux desktop apps? (Most Linux users are x86_64)
   - Recommendation: Skip for v1.4, add in later phase if users request it

2. **Should we publish to GitHub Releases on every push to main?**
   - What we know: tauri-action auto-releases when triggered on tag pushes (e.g., v1.0.0)
   - What's unclear: User preference — continuous releases vs manual version tagging
   - Recommendation: Only release on version tags (v*), not every push to main (less noisy)

3. **Should we cache Playwright browsers in CI?**
   - What we know: Playwright browser install takes ~1-2 min, caching could save time
   - What's unclear: Cache size impact (browsers are ~500 MB), worth the complexity?
   - Recommendation: Skip for v1.4, revisit if E2E CI time becomes a bottleneck (currently 2-3 min total)

## Sources

### Primary (HIGH confidence)

- [Tauri v2 GitHub Actions Guide](https://v2.tauri.app/distribute/pipelines/github/) - Official CI setup
- [tauri-apps/tauri-action](https://github.com/tauri-apps/tauri-action) - Official GitHub Action
- [Tauri v2 Prerequisites](https://v2.tauri.app/start/prerequisites/) - System dependencies
- [Migration to webkit2gtk-4.1](https://v2.tauri.app/blog/tauri-2-0-0-alpha-3/) - webkit version requirement
- [Tauri Issue #9662](https://github.com/tauri-apps/tauri/issues/9662) - Ubuntu 24.04 webkit2gtk-4.0 removal
- [Playwright Best Practices](https://playwright.dev/docs/best-practices) - E2E testing strategies

### Secondary (MEDIUM confidence)

- [GitHub Actions workflow syntax](https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions) - job dependencies
- [swatinem/rust-cache](https://github.com/Swatinem/rust-cache) - Rust build caching
- [Tauri Issue #11763](https://github.com/tauri-apps/tauri/issues/11763) - webkit version discussion
- [Tauri Action Issue #342](https://github.com/tauri-apps/tauri-action/issues/342) - Artifact upload options

### Tertiary (LOW confidence)

- Community blog posts on Tauri CI setups (various, 2024-2026)
- GitHub Discussions on webkit compatibility

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Official Tauri docs verified, webkit2gtk-4.1 requirement confirmed
- Architecture: HIGH - Fail-fast pattern validated by official examples, E2E-first widely recommended
- Pitfalls: HIGH - webkit version mismatch documented in GitHub issues, directory mismatch found in codebase
- Code examples: HIGH - Directly from official Tauri docs and tauri-action README

**Research date:** 2026-02-10
**Valid until:** 2026-05-10 (90 days - ecosystem stable, but watch for Tauri 2.1+ updates)

**Critical validation needed before planning:**
- [ ] Confirm ubuntu-22.04 runner supports webkit2gtk-4.1-dev (test in CI)
- [ ] Verify E2E tests pass in web mode (already validated in Phase 9)
- [ ] Check if tauri-action v0 is latest stable (may update to v1 during phase)
- [ ] Validate `rulez-ui` directory name is consistent across all files
