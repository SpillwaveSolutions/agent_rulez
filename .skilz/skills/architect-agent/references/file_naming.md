# File Naming Convention

## Pattern Structure

```
<type>-<date>-<time>-<ticket_id>_<phase>_<description>.md
```

## Components

- **Type**: `instruct`, `grade`, `analysis`, or `human`
- **Date**: `YYYY_MM_DD` (underscores)
- **Time**: `HH_MM` (24-hour format)
- **Ticket ID**: Jira/Atlassian ticket ID (e.g., `tkt121`, `proj456`)
- **Phase**: Phase or step identifier if part of multi-phase work (e.g., `phase5`, `phase5b`, `step3`)
- **Description**: Brief descriptive words separated by underscores

## Pattern Rules

1. **Always include ticket ID** when working on a tracked ticket
2. **Include phase/step** when part of a multi-phase plan
3. **Omit ticket ID only** for general analysis/brainstorming not tied to a ticket
4. **Matching descriptions**: Instructions, grades, and human summaries for the same task MUST share the same ticket_id_phase_description (timestamps will differ)

## Examples

### Multi-phase ticket work:
```
instructions/instruct-2025_10_20-22_35-tkt121_phase5_infrastructure_deployment.md
human/human-2025_10_20-22_35-tkt121_phase5_infrastructure_deployment.md
grades/grade-2025_10_20-23_00-tkt121_phase5_infrastructure_deployment.md

instructions/instruct-2025_10_21-22_56-tkt121_phase5c_fix_scheduler_https.md
human/human-2025_10_21-22_56-tkt121_phase5c_fix_scheduler_https.md
grades/grade-2025_10_21-23_43-tkt121_phase5c_fix_scheduler_https.md
```

### Single-phase ticket work:
```
instructions/instruct-2025_10_04-14_30-proj123_implement_contact_api.md
grades/grade-2025_10_04-18_45-proj123_implement_contact_api.md
```

### General analysis (no ticket):
```
analysis/analysis-2025_10_04-14_00-database_schema_design.md
```

## Matching Rule

Instructions, human summaries, and grades for the same task MUST share:
- Same ticket ID
- Same phase identifier (if applicable)
- Same description (words after final underscore)
- Different timestamps (reflect actual creation time)
