# CCH Quality Checklists

This directory contains comprehensive quality checklists for all CCH features.

## Available Checklists

| Checklist | Feature | Status |
|-----------|---------|--------|
| [rulez-ui-checklist.md](rulez-ui-checklist.md) | RuleZ UI Desktop App | Pre-Implementation |
| [phase2-governance-checklist.md](phase2-governance-checklist.md) | Phase 2 Governance | Pre-Implementation |

## Checklist Categories

Each checklist includes:

### 1. Pre-Implementation
- Environment setup verification
- Dependency checks
- Understanding verification

### 2. User Story Acceptance
- Functional requirements per story
- Edge cases to test
- Performance criteria

### 3. Technical Quality
- Code quality standards
- Testing requirements
- Performance benchmarks
- Security considerations

### 4. Pre-Merge (Per PR)
- Code review checklist
- Testing verification
- Documentation updates
- CI/CD validation

### 5. Pre-Release
- Full functionality verification
- Cross-platform testing
- Documentation completeness
- Release preparation

### 6. Regression Testing
- Critical paths
- Edge cases
- Error scenarios

## Usage

### During Development
1. Review pre-implementation checklist before starting
2. Check off acceptance criteria as you implement
3. Use pre-merge checklist before creating PR

### Before Release
1. Complete all pre-release items
2. Run full regression test suite
3. Verify cross-platform (if applicable)

## Quick Reference

### RuleZ UI Pre-Commit
```bash
cd rulez_ui
bun run lint        # Biome linting
bun run typecheck   # TypeScript
bun run test        # Unit tests
```

### Phase 2 Governance Pre-Commit
```bash
cd cch_cli
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Updating Checklists

Checklists should be updated when:
- New requirements added to spec
- Edge cases discovered during testing
- Bugs found that should be regression tested
- Performance targets change
