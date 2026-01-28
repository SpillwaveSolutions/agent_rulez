# PDA Reorganization Plan: Documentation Specialist Skill

**Version**: 3.0-PDA
**Date**: 2025-12-01
**Status**: DRAFT - Awaiting Approval

---

## Executive Summary

This plan transforms the documentation-specialist skill from a **content-heavy** design (~9,000 tokens typical load) to a **true Progressive Disclosure Architecture** (~1,500-2,500 tokens typical load).

**Key Changes**:
1. Slim SKILL.md from 595 → ~180 lines (pure router + quick start)
2. Split large workflows into focused step files
3. Extract embedded examples to standalone files
4. Add workflow TOC files for navigation

**Expected Token Reduction**: 60-75%

---

## Current State Analysis

### File Sizes (Problems)

| File | Lines | Est. Tokens | Target | Delta |
|------|-------|-------------|--------|-------|
| SKILL.md | 595 | ~2,500 | 400-500 | -80% needed |
| greenfield-workflow.md | 771 | ~3,000 | 300-400 | -87% needed |
| brownfield-workflow.md | ~500 | ~2,000 | 300-400 | -80% needed |
| audit-workflow.md | ~300 | ~1,200 | 300-400 | -66% needed |
| convert-workflow.md | ~200 | ~800 | 200-300 | -62% needed |
| diagram-workflow.md | ~250 | ~1,000 | 300-400 | -60% needed |

### Content Duplication

**SKILL.md contains**:
- ❌ Quick Start (duplicates Overview)
- ❌ Overview (duplicates Quick Start)
- ❌ Core Capabilities (duplicates workflow files)
- ❌ Execution Instructions (duplicates workflows)
- ❌ Command Patterns (duplicates examples)
- ❌ Token Budget Management (meta-info, belongs in CLAUDE.md)
- ✅ Decision Tree (keep - this is routing logic)
- ✅ Resource Links (keep - this is routing)
- ✅ Integration with Other Skills (keep - important context)

**greenfield-workflow.md contains**:
- ✅ Workflow Steps (keep)
- ❌ 300-line SRS example (extract to examples/)
- ❌ Advanced Features (extract to separate file)
- ✅ Quality Checklist (keep - compact)

---

## Proposed Structure

```
documentation-specialist/
├── SKILL.md                              # Tier 1: Router (~180 lines, ~450 tokens)
│
├── references/
│   ├── workflows/
│   │   ├── TOC.md                        # NEW: Workflow navigation index (~50 tokens)
│   │   │
│   │   ├── greenfield/                   # Expanded greenfield workflow
│   │   │   ├── greenfield-overview.md    # Entry point + steps (~150 tokens)
│   │   │   ├── greenfield-srs-guide.md   # SRS-specific guidance (~200 tokens)
│   │   │   ├── greenfield-prd-guide.md   # PRD-specific guidance (~200 tokens)
│   │   │   ├── greenfield-openapi-guide.md # OpenAPI-specific (~200 tokens)
│   │   │   └── greenfield-advanced.md    # Multi-project, packages (~150 tokens)
│   │   │
│   │   ├── brownfield/                   # Expanded brownfield workflow
│   │   │   ├── brownfield-overview.md    # Entry point + steps (~150 tokens)
│   │   │   ├── brownfield-springboot.md  # Spring Boot specifics (~200 tokens)
│   │   │   ├── brownfield-fastapi.md     # FastAPI specifics (~200 tokens)
│   │   │   └── brownfield-extraction.md  # Extraction patterns (~200 tokens)
│   │   │
│   │   ├── audit-workflow.md             # Compact audit workflow (~200 tokens)
│   │   ├── convert-workflow.md           # Compact convert workflow (~150 tokens)
│   │   ├── diagram-workflow.md           # Compact diagram workflow (~200 tokens)
│   │   ├── user-docs-workflow.md         # Compact user docs workflow (~200 tokens)
│   │   ├── tutorial-workflow.md          # Compact tutorial workflow (~200 tokens)
│   │   └── runbook-workflow.md           # Compact runbook workflow (~200 tokens)
│   │
│   ├── examples/                         # Enhanced examples
│   │   ├── TOC.md                        # Examples navigation (~100 tokens)
│   │   ├── greenfield/
│   │   │   ├── billing-srs.md            # Full SRS example (keep as-is)
│   │   │   ├── billing-srs-snippet.md    # NEW: 30-line snippet for inline use
│   │   │   ├── collaboration-prd.md      # Full PRD example (keep as-is)
│   │   │   ├── collaboration-prd-snippet.md # NEW: 30-line snippet
│   │   │   └── ... (other examples)
│   │   └── brownfield/
│   │       └── ... (keep as-is)
│   │
│   ├── templates/                        # Keep as-is (good structure)
│   │   └── markdown/
│   │
│   ├── reference/                        # Keep as-is (excellent PDA structure!)
│   │   └── (27 small files)
│   │
│   └── mappings/                         # Keep as-is
│       └── backend/
│
└── docs/                                 # Meta-documentation (not loaded by skill)
    ├── README.md                         # User-facing overview
    ├── USER_GUIDE.md                     # Detailed usage guide
    └── CLAUDE.md                         # Developer guidance (move token budget here)
```

---

## Detailed Changes

### 1. SKILL.md Transformation

**Remove** (move elsewhere or delete):
- Lines 99-164: Overview section (redundant with Quick Start)
- Lines 279-321: Command Patterns (already in examples)
- Lines 322-393: Execution Instructions (redundant with workflows)
- Lines 394-417: Best Practices (move to reference/01-philosophy.md)
- Lines 509-525: Token Budget Management (move to CLAUDE.md)
- Lines 578-589: Quick Reference table (redundant)

**Keep** (refine):
- Lines 1-6: Frontmatter
- Lines 8-95: Quick Start (condense to 40 lines)
- Lines 165-278: Decision Tree & Routing Logic (core function)
- Lines 418-507: Knowledge Base Organization + Resource Links
- Lines 467-507: Integration with Other Skills + Error Handling

**Resulting SKILL.md**: ~180 lines, ~450 tokens

### 2. Greenfield Workflow Split

**Current**: Single 771-line file with everything

**Proposed Split**:

| New File | Content | Tokens |
|----------|---------|--------|
| `greenfield/greenfield-overview.md` | Steps 1-7 outline, quality checklist | ~150 |
| `greenfield/greenfield-srs-guide.md` | SRS-specific generation guidance | ~200 |
| `greenfield/greenfield-prd-guide.md` | PRD-specific generation guidance | ~200 |
| `greenfield/greenfield-openapi-guide.md` | OpenAPI-specific guidance | ~200 |
| `greenfield/greenfield-advanced.md` | Multi-project, packages, continuous docs | ~150 |
| `examples/greenfield/billing-srs-snippet.md` | 30-line SRS snippet for inline use | ~100 |

**Total**: ~1,000 tokens (vs. ~3,000 current) = **67% reduction**

### 3. Example Snippets (New)

Create small "snippet" versions of examples for inline workflow use:

```markdown
# billing-srs-snippet.md (~30 lines)

## 3.1 User Authentication

**FR-AUTH-001**: User Login

**Description**: The system shall authenticate users with email/password.

**Acceptance Criteria**:
- **Given** a registered user with verified email
- **When** they enter correct credentials
- **Then** the system grants access with JWT token (1-hour expiry)
- **And** redirects to dashboard

**Priority**: Must Have
```

Workflows reference snippets inline, full examples available via TOC.md.

### 4. Workflow TOC.md (New)

```markdown
# Workflow Navigation

| Intent | Workflow | Est. Tokens |
|--------|----------|-------------|
| Create new docs | [greenfield/greenfield-overview.md](greenfield/greenfield-overview.md) | ~150 |
| Document existing code | [brownfield/brownfield-overview.md](brownfield/brownfield-overview.md) | ~150 |
| Audit documentation | [audit-workflow.md](audit-workflow.md) | ~200 |
| Convert formats | [convert-workflow.md](convert-workflow.md) | ~150 |
| Generate diagrams | [diagram-workflow.md](diagram-workflow.md) | ~200 |

## Sub-Workflows (Load on demand)

### Greenfield
- [greenfield-srs-guide.md](greenfield/greenfield-srs-guide.md) - SRS creation
- [greenfield-prd-guide.md](greenfield/greenfield-prd-guide.md) - PRD creation
...
```

---

## Token Budget Analysis

### Before (Current)

| Request Type | Files Loaded | Tokens |
|--------------|--------------|--------|
| Create SRS | SKILL.md + greenfield-workflow + template | ~6,300 |
| Document Spring Boot | SKILL.md + brownfield-workflow + mapping | ~5,500 |
| Audit docs | SKILL.md + audit-workflow | ~3,700 |

**Average**: ~5,100 tokens

### After (Proposed)

| Request Type | Files Loaded | Tokens |
|--------------|--------------|--------|
| Create SRS | SKILL.md + greenfield-overview + srs-guide + template | ~1,500 |
| Document Spring Boot | SKILL.md + brownfield-overview + springboot + mapping | ~1,500 |
| Audit docs | SKILL.md + audit-workflow | ~650 |

**Average**: ~1,200 tokens

**Improvement**: **76% reduction** in typical token load

---

## Implementation Phases

### Phase 1: SKILL.md Refactoring
1. Create backup: `SKILL.md.backup-v2.1`
2. Remove redundant sections
3. Condense Quick Start
4. Verify routing logic intact
5. Test: Intent classification still works

### Phase 2: Greenfield Workflow Split
1. Create `references/workflows/greenfield/` directory
2. Extract SRS example → `examples/greenfield/billing-srs-snippet.md`
3. Split workflow into 5 focused files
4. Update SKILL.md references
5. Test: Create SRS flow works

### Phase 3: Brownfield Workflow Split
1. Create `references/workflows/brownfield/` directory
2. Split into overview + framework-specific guides
3. Update SKILL.md references
4. Test: Document Spring Boot flow works

### Phase 4: Other Workflows
1. Compact audit-workflow.md to ~200 tokens
2. Compact convert-workflow.md to ~150 tokens
3. Compact diagram-workflow.md to ~200 tokens
4. Create TOC.md for workflows
5. Test: All workflows functional

### Phase 5: Documentation Update
1. Update CLAUDE.md with new structure
2. Update README.md
3. Archive old PDA_COMPLETE_SUMMARY.md
4. Create new v3.0 release notes

---

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Breaking routing logic | High | Keep Decision Tree intact, test each intent |
| Missing content after split | Medium | Create checklist, test full workflows |
| Reference links break | Medium | Update all links systematically |
| User confusion with new structure | Low | Clear TOC.md files, updated docs |

---

## Success Criteria

- [ ] SKILL.md ≤ 200 lines
- [ ] All workflow files ≤ 300 tokens individually
- [ ] Typical request loads ≤ 2,000 tokens
- [ ] All 8 intent types route correctly
- [ ] All existing functionality preserved
- [ ] CLAUDE.md updated with new structure
- [ ] README.md reflects v3.0 changes

---

## Appendix: SKILL.md Target Structure

```markdown
---
name: "documentation-specialist"
description: "..."
version: "3.0-PDA"
allowed-tools: ["Read", "Write", "Edit", "Glob", "Grep", "Bash", "Skill"]
---

# Documentation Specialist Skill

## Quick Start (~40 lines)
- Primary Capabilities (8 bullet points)
- How It Works (5 steps, 1 line each)
- Command Examples (1 example per intent, total 8)

## Intent Classification (~50 lines)
- Intent table with keywords and workflow links
- IMPORTANT note about loading only needed workflow

## Document Type Tables (~40 lines)
- Requirements & Design types → templates
- User Documentation types → templates
- Developer Documentation types → templates
- Operational Documentation types → templates

## Framework Detection (~20 lines)
- Detection files/patterns table
- Link to brownfield workflow

## Resource Links (~30 lines)
- Workflow guides (links only)
- Reference guides (link to comprehensive-guide.md)
- Templates (link to templates/)
- Examples (link to examples/TOC.md)
- Mappings (links only)

## Integration & Error Handling (~20 lines)
- Other skills (docx, pdf, plantuml, mermaid-architect)
- Common error patterns

---
**Total**: ~200 lines, ~500 tokens
```

---

## Approval

**To proceed with implementation, please confirm:**

1. ✅ SKILL.md target structure acceptable?
2. ✅ Greenfield split approach (5 files) acceptable?
3. ✅ Snippet concept for inline examples acceptable?
4. ✅ TOC.md navigation pattern acceptable?
5. ✅ Implementation phases acceptable?

---

**End of Plan**
