# Feature Management Guide

Complete guide to managing features in Spec-Driven Development projects using natural language commands and automated tracking.

## Overview

Claude automatically tracks feature status and provides natural language feature management for both greenfield and brownfield projects. This eliminates manual tracking and makes it easy to add, reorder, and monitor features throughout development.

## Natural Language Commands

### Adding Features

**Just describe what you want:**

```
"Add a new feature for user notifications"
"We need email integration"
"Let's add an admin dashboard"
"I want to build a reporting module"
```

**Claude will:**
1. Detect the feature request
2. Ask clarifying questions:
   - What's the priority? (High/Medium/Low)
   - What features does this depend on?
   - Brief description of the feature?
3. Create a feature entry
4. Show updated feature status dashboard

**Example interaction:**
```
User: "Add a feature for user notifications"

Claude: I'll add "user-notifications" as a new feature. Let me ask a few questions:

1. **Priority**: High, Medium, or Low?
2. **Dependencies**: Does this depend on any existing features? (e.g., user-authentication)
3. **Brief description**: What should this feature do?

[User answers...]

Claude:
✅ Feature added: user-notifications

📊 Updated Feature List:
1. user-authentication (In Progress - 80%)
2. profile-management (Pending)
3. user-notifications (Pending) ← NEW
4. admin-dashboard (Pending)

Would you like to start specifying this feature now, or continue with the current feature?
```

### Reordering Features

**Natural commands:**

```
"Move user-notifications before profile-management"
"Let's do admin-dashboard first"
"Swap the order of reporting and analytics"
"Prioritize email integration"
```

**Example interaction:**
```
User: "Let's do user-notifications before profile-management"

Claude:
Current order:
1. user-authentication (In Progress)
2. profile-management (Pending)
3. user-notifications (Pending)
4. admin-dashboard (Pending)

Proposed new order:
1. user-authentication (In Progress)
2. user-notifications (Pending) ← MOVED UP
3. profile-management (Pending)
4. admin-dashboard (Pending)

This will also update dependencies if needed.
Is this correct? (yes/no)

[User: yes]

Claude:
✅ Feature order updated!

📊 Feature Status: user-authentication (In Progress - 80%) → Next: user-notifications
```

### Removing Features

**Natural commands:**

```
"Remove the reporting feature"
"Delete admin-dashboard from the list"
"We don't need email integration anymore"
```

**Example interaction:**
```
User: "Remove the reporting feature"

Claude:
⚠️  Warning: This will remove 'reporting' feature from the project.

**Impact Analysis:**
- Features depending on this: None
- Progress lost: 0% (not started)
- Artifacts to remove: None (not yet created)

Are you sure you want to remove this feature? (yes/no)

[User: yes]

Claude:
✅ Feature 'reporting' removed.

📊 Updated Feature List:
1. user-authentication (Complete - 100%)
2. profile-management (In Progress - 60%)
3. admin-dashboard (Pending)
```

### Showing Feature Status

**Natural commands:**

```
"Show feature status"
"What's the current progress?"
"List all features"
"Show me the feature dashboard"
```

**Displays full dashboard:**
```
📊 Project Feature Status Dashboard

🎯 CURRENT FEATURE
├─ profile-management (Planning - 40% complete)
│  ├─ ✅ Requirements specified
│  ├─ 🔄 Implementation plan in progress
│  ├─ ⏸️  Tasks not started
│  └─ ⏸️  Implementation not started
│  Blockers: None
│  Dependencies: user-authentication ✅

✅ COMPLETED FEATURES (2)
├─ database-setup (100% complete)
└─ user-authentication (100% complete)

📋 UPCOMING FEATURES (2)
├─ user-notifications (depends on: user-authentication ✅)
└─ admin-dashboard (depends on: user-authentication ✅, profile-management ⏸️)

⚠️  BLOCKED FEATURES (0)

**Overall Progress:** 2 of 5 features complete (40%)
```

### Querying Dependencies

**Natural commands:**

```
"What features depend on user-authentication?"
"What does profile-management depend on?"
"Show me the dependency tree"
"Can we start admin-dashboard yet?"
```

**Example interaction:**
```
User: "What features depend on user-authentication?"

Claude:
📊 user-authentication Dependencies

**Features that depend on user-authentication:**
├─ profile-management (⏸️ Pending)
├─ user-notifications (⏸️ Pending)
└─ admin-dashboard (⏸️ Pending)

**Status:** user-authentication is ✅ Complete
**Impact:** 3 features are now unblocked and ready to start

Would you like to start one of these features now?
```

## Feature Status Tracking

### Progress Stages

Each feature automatically tracks its progress through the SDD workflow:

| Stage | Progress | Indicators | Next Step |
|-------|----------|------------|-----------|
| **Not Started** | 0% | No artifacts exist | Run `/speckit.specify` |
| **Specified** | 20% | `specify.md` exists | Run `/speckit.plan` |
| **Planned** | 40% | `plan.md` exists | Run `/speckit.tasks` |
| **Tasked** | 60% | `tasks.md` exists | Run `/speckit.implement` |
| **In Progress** | 80% | Implementation started | Complete implementation |
| **Complete** | 100% | Tests pass, feature done | Move to next feature |

### Status Indicators

- ✅ **Complete**: Feature finished and tested
- 🔄 **In Progress**: Currently being worked on
- ⏸️ **Pending**: Not started, waiting
- ⚠️ **Blocked**: Cannot proceed due to dependencies or issues

### Brief Status Line

After every SDD command, Claude shows a brief status line:

```
📊 **Feature Status:** profile-management (Planning) → Next: user-notifications
   Progress: [●●○○○] 40% | Completed: 2 of 5 features | Dependencies: user-authentication ✅
```

**Components:**
- **Current feature** and its stage
- **Next feature** in queue
- **Progress bar** visual (●○)
- **Completion ratio** (N of Total)
- **Dependencies** with status indicators

## Dependency Management

### Declaring Dependencies

**Natural language:**

```
"Profile-management depends on user-authentication"
"Admin-dashboard needs both user-auth and profile-management"
"User-notifications requires user-authentication"
```

**Claude automatically:**
1. Records the dependency relationship
2. Updates the dependency tree
3. Checks for circular dependencies
4. Calculates which features are blocked

### Checking Dependencies

**Before starting a feature:**

```
📊 Can we start profile-management?
   Checking dependencies...
   ✅ user-authentication (complete)
   ✅ database-setup (complete)

   All dependencies satisfied! Ready to proceed.
```

**If dependencies not met:**

```
📊 Can we start admin-dashboard?
   Checking dependencies...
   ✅ user-authentication (complete)
   ⏸️ profile-management (in progress - 60%)

   ⚠️  Not all dependencies satisfied.
   Blocked by: profile-management (needs 40% more progress)

   Estimated wait: Complete profile-management first
```

### Circular Dependency Detection

**Claude automatically detects circular dependencies:**

```
⚠️  Warning: Circular dependency detected!

feature-A depends on feature-B
feature-B depends on feature-C
feature-C depends on feature-A

**How to resolve:**
1. Remove one of the dependencies
2. Extract shared functionality into a new base feature
3. Reconsider the feature boundaries

Would you like help resolving this?
```

## Multi-Feature Project Structure

### File Organization

```
.speckit/
├── constitution.md           # Project-wide principles
├── features/
│   ├── 001-database-setup/
│   │   ├── specify.md        # ✅ Complete
│   │   ├── plan.md
│   │   ├── tasks.md
│   │   └── implementation/   # Code generated
│   ├── 002-user-auth/
│   │   ├── specify.md        # ✅ Complete
│   │   ├── plan.md
│   │   ├── tasks.md
│   │   └── implementation/
│   ├── 003-profile-mgmt/
│   │   ├── specify.md        # 🔄 In Progress
│   │   ├── plan.md
│   │   └── tasks.md          # ⏸️ Not started
│   ├── 004-user-notifications/
│   │   └── specify.md        # ⏸️ Pending
│   └── 005-admin-dashboard/
│       └── (empty)           # ⏸️ Not started
└── .claude/commands/         # SDD slash commands
```

### Feature Naming Convention

**Recommended pattern:**
- `001-feature-name` (numbered for ordering)
- `user-authentication` (descriptive, lowercase, hyphenated)
- `admin-dashboard` (clear, concise)

**Avoid:**
- Single words without context (`auth`, `admin`)
- Vague names (`feature1`, `module`)
- Inconsistent casing (`UserAuth`, `user_auth`)

## Advanced Feature Operations

### Bulk Operations

**Adding multiple features at once:**

```
User: "Add these features: user-notifications, email-integration, admin-dashboard, reporting"

Claude:
I'll add 4 new features. For each, I'll need some details:

**1. user-notifications**
   - Priority? (High/Medium/Low)
   - Dependencies?
   - Brief description?

[Repeat for each feature...]

Claude:
✅ Added 4 features:
1. user-notifications (High priority, depends on: user-auth)
2. email-integration (Medium priority, depends on: user-auth)
3. admin-dashboard (High priority, depends on: user-auth, profile-mgmt)
4. reporting (Low priority, depends on: admin-dashboard)

📊 Total features: 7 (3 complete, 1 in progress, 3 pending)
```

### Feature Templates

**For common feature types, Claude can use templates:**

```
User: "Add a CRUD feature for products"

Claude:
I'll create a CRUD feature using the standard template:

**Features to add:**
1. product-list (display all products)
2. product-create (create new product)
3. product-edit (update existing product)
4. product-delete (remove product)

Or would you prefer a single "product-management" feature with all CRUD operations?

**Choose:**
A) 4 separate features (more granular tracking)
B) 1 combined feature (simpler, faster)
```

### Feature Merging

**Combining related features:**

```
User: "Merge user-notifications and email-integration into one feature"

Claude:
I'll merge these features:

**Current:**
- user-notifications (Specified - 20%)
- email-integration (Pending - 0%)

**Merged feature:**
- user-communication (combines both)
  - Inherits progress from user-notifications (20%)
  - Includes email integration requirements
  - Dependencies: user-authentication ✅

**Artifacts to merge:**
- Combine specify.md files
- Update dependencies

Proceed with merge? (yes/no)
```

## Greenfield vs Brownfield Differences

### Greenfield Feature Management

**Starting fresh:**
- All features tracked from the beginning
- Clear dependency tree from design
- Progressive addition as project evolves
- No legacy features to document

**Example workflow:**
```
1. Initialize project
2. Create constitution
3. Specify first feature → Feature tracking begins
4. Add more features as needed
5. Track all features through completion
```

### Brownfield Feature Management

**Working with existing code:**
- **Existing features**: Discovered through reverse-engineering
- **New features**: Added via SDD workflow
- **Mixed tracking**: Both documented and new features
- **Integration awareness**: New features must integrate with existing

**Example workflow:**
```
1. Analyze existing codebase
2. Reverse-engineer major features (optional)
3. Add new feature → Track alongside existing
4. Show integration impact on status
5. Track dependencies on existing features
```

**Status display for brownfield:**
```
📊 Project Feature Status Dashboard

📚 EXISTING FEATURES (Documented)
├─ user-authentication (Existing - documented)
├─ profile-management (Existing - documented)
└─ basic-reporting (Existing - not documented)

🎯 CURRENT SDD FEATURE
├─ advanced-analytics (Planning - 40% complete)
│  Dependencies: basic-reporting (existing ✅)

📋 UPCOMING SDD FEATURES (2)
├─ email-notifications (depends on: user-authentication existing ✅)
└─ admin-dashboard (depends on: advanced-analytics ⏸️)
```

## Best Practices

### 1. Clear Naming

✅ **Good:**
- `user-authentication`
- `email-notifications`
- `advanced-reporting`

❌ **Avoid:**
- `auth`
- `emails`
- `reports`

### 2. Granularity

**Too granular:**
```
- login-form
- login-validation
- login-error-handling
- login-success-redirect
```
**Better:**
```
- user-authentication (includes all login functionality)
```

**Too coarse:**
```
- user-module (authentication, profile, preferences, notifications)
```
**Better:**
```
- user-authentication
- user-profile
- user-preferences
- user-notifications
```

### 3. Dependency Management

**Declare early:**
```
When specifying: "This depends on user-auth"
Not later: "Oh, we need user-auth first"
```

**Keep shallow:**
```
✅ Feature A → Feature B → Feature C (3 levels)
❌ Feature A → B → C → D → E → F (6 levels)
```

### 4. Progress Updates

**Let Claude track automatically:**
- Don't manually update progress
- Trust the artifact detection
- Inform Claude when features complete

**Example:**
```
User: "We finished implementing user-auth"

Claude:
✅ Updating user-authentication to 100% complete!

Running final checks:
- ✅ specify.md exists
- ✅ plan.md exists
- ✅ tasks.md exists
- ✅ Implementation complete
- 🔄 Checking tests...

Tests passing? (yes/no)

[User: yes]

Claude:
✅ user-authentication marked as COMPLETE!

📊 Features: 3 of 7 complete (43%)
🎯 Next feature: profile-management

Ready to start profile-management? (yes/no)
```

## Troubleshooting

### Feature Not Tracked

**Problem:** Claude doesn't recognize a feature

**Solution:**
```
User: "Why isn't email-integration showing in the status?"

Claude:
Let me check the feature list...

I don't see "email-integration" in the tracked features.

**Possible reasons:**
1. Feature was never added
2. Feature was removed
3. Feature has a different name

Would you like to:
A) Add "email-integration" as a new feature
B) Show me all tracked features
C) Search for similar feature names
```

### Dependency Confusion

**Problem:** Unclear what depends on what

**Solution:**
```
User: "What's blocking admin-dashboard?"

Claude:
📊 admin-dashboard Dependency Analysis

**Required dependencies:**
├─ user-authentication ✅ (complete)
├─ profile-management ⏸️ (in progress - 60%)
└─ reporting ⏸️ (pending - not started)

**Blocking status:**
⚠️  Blocked by 2 features:
1. profile-management (needs 40% more)
2. reporting (not started)

**Recommendation:**
Complete profile-management, then start reporting before admin-dashboard.

Estimated time to unblock: [Based on current velocity]
```

### Lost Progress

**Problem:** Feature showing incorrect progress

**Solution:**
```
User: "User-auth shows 40% but we finished it"

Claude:
Let me verify user-authentication status...

**Artifact check:**
- ✅ specify.md exists
- ✅ plan.md exists
- ✅ tasks.md exists
- ❌ Implementation not detected

**Issue:** Implementation not marked complete

**To fix:**
1. Confirm implementation is done
2. Confirm tests pass
3. I'll update status to 100%

Is implementation complete with passing tests? (yes/no)

[User: yes]

Claude:
✅ Updated user-authentication to 100% complete!
```

## Feature Management Reference

### Quick Command Reference

| User Intent | Example Command | Claude Action |
|-------------|----------------|---------------|
| Add feature | "Add user-notifications" | Ask questions, create feature |
| Reorder | "Move X before Y" | Show current/proposed order, update |
| Remove | "Remove feature X" | Warn about impact, remove if confirmed |
| Show status | "Show features" | Display full dashboard |
| Check deps | "What blocks X?" | Show dependency tree and blockers |
| Start feature | "Let's work on X" | Switch current feature to X |
| Mark complete | "We finished X" | Verify completion, update to 100% |

### Detection Patterns

Claude automatically detects these patterns:

- **Add:** "add", "create", "new feature", "build", "implement"
- **Move:** "move", "reorder", "prioritize", "before", "after", "first"
- **Remove:** "remove", "delete", "drop", "cancel"
- **Status:** "show", "list", "status", "progress", "dashboard"
- **Dependencies:** "depends", "requires", "needs", "blocks", "blocked by"
- **Complete:** "finished", "done", "completed", "ready"

## Integration with SDD Workflow

Feature management is automatically integrated with all SDD commands:

```
/speckit.specify → Asks about features, tracks new feature
/speckit.plan → Shows feature status, updates progress to 40%
/speckit.tasks → Updates progress to 60%
/speckit.implement → Updates progress to 80%
[Tests pass] → Updates progress to 100%, shows next feature
```

Every command includes a **brief status line** in its summary, and users can request **full status** anytime with option `[D]` or by asking "show feature status".

---

For more information, see:
- [SKILL.md](../SKILL.md) - Main skill file with feature tracking section
- [Greenfield Workflow](greenfield.md) - Feature management for new projects
- [Brownfield Workflow](brownfield.md) - Feature management for existing codebases
