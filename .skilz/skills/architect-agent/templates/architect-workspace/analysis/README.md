# Analysis Directory

This directory contains investigation and analysis documents created during planning and problem-solving.

## Purpose

- **Root Cause Analysis** - Investigation into bugs, failures, or issues
- **Architecture Analysis** - Design decisions, trade-offs, alternatives
- **Performance Analysis** - Performance investigations and optimizations
- **Risk Analysis** - Identifying and mitigating risks
- **Technical Research** - Research into new technologies or approaches

## File Naming

```
analysis-YYYYMMDD_HHMMSS-brief_description.md
```

**Examples:**
- `analysis-20251120_143045-database_migration_options.md`
- `analysis-20251120_150000-vpc_connector_drift_root_cause.md`
- `analysis-20251120_160000-testing_strategy_comparison.md`

## Typical Contents

### Root Cause Analysis
- Problem statement
- Investigation steps
- Findings
- Root cause identified
- Proposed solutions
- Selected solution and rationale

### Architecture Analysis
- Current state
- Proposed changes
- Alternatives considered
- Trade-offs
- Decision and rationale
- Implementation approach

### Performance Analysis
- Performance metrics
- Bottlenecks identified
- Proposed optimizations
- Expected impact
- Implementation plan

## Workflow

1. Encounter complex problem or decision point
2. Create analysis document
3. Perform investigation/research
4. Document findings
5. Use findings to inform instruction creation
6. Reference analysis in instructions

## Integration with Instructions

Analysis documents often lead to or inform instruction files. Reference analysis documents in instructions to provide context.

**Example:**
```markdown
## Context

Based on analysis in `analysis/analysis-20251120_143045-database_migration_options.md`,
we selected Option 2 (Pulumi + Terraform) due to...
```
