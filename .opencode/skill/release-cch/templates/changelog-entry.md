## [${VERSION}] - ${DATE}

### Added

- New feature description

### Fixed

- Bug fix description

### Changed

- Change description

### Documentation

- Documentation update description

### BREAKING CHANGES

- Breaking change description (if any)

---

## Template Usage

Replace `${VERSION}` with the actual version (e.g., `1.1.0`)
Replace `${DATE}` with today's date in YYYY-MM-DD format

### Conventional Commit Types

| Type | Section |
|------|---------|
| `feat:` | Added |
| `fix:` | Fixed |
| `docs:` | Documentation |
| `chore:` | Changed |
| `refactor:` | Changed |
| `perf:` | Added (performance) |
| `feat!:` | BREAKING CHANGES |
| `fix!:` | BREAKING CHANGES |

### Example Entry

```markdown
## [1.1.0] - 2026-02-15

### Added

- Support for custom rule priorities
- New `cch status` command for quick health checks
- Environment variable override for log level

### Fixed

- Race condition in concurrent rule evaluation
- Incorrect path matching for Windows paths

### Changed

- Improved error messages for invalid YAML syntax
- Updated default timeout from 30s to 60s

### Documentation

- Added troubleshooting guide for common issues
- Updated CLI reference with new commands
```
