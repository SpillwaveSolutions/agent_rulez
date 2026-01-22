# Brownfield Workflow: Bringing SDD to Existing Codebases

**Note:** All summarization sections in this guide should use the **10-point summary template** from SKILL.md, with feature status tracking when applicable.

## When to Use Brownfield Workflow

Use this workflow when:
- Adding features to an existing codebase
- Modernizing or refactoring legacy systems
- Adopting SDD methodology in an ongoing project
- Reverse-engineering existing code into SDD format
- Migrating from traditional development to spec-driven development

For new projects from scratch, see [Greenfield Workflow](greenfield.md).

## Core Concepts

### Brownfield Philosophy

Brownfield SDD respects and builds upon existing code:
- **Analyze first**: Understand current architecture before making changes
- **Respect patterns**: Constitution should acknowledge existing design decisions
- **Incremental adoption**: Start small, prove value, expand usage
- **Integration-aware**: New features must mesh with existing code

### Analysis Depth Options

Choose the appropriate analysis depth based on your needs:

- **Surface Level** (~5-10 minutes)
  - File structure and directory organization
  - Dependencies and tech stack
  - Basic code patterns (naming, file organization)
  - **Use when**: Quick bootstrap to start adding features

- **Moderate Depth** (~20-40 minutes) - **DEFAULT**
  - Architecture and design patterns
  - API contracts and interfaces
  - Data models and schemas
  - Business logic organization
  - Configuration and environment setup
  - **Use when**: Most brownfield scenarios

- **Deep Analysis** (~1-3 hours)
  - Complete system comprehension
  - Edge cases and error handling
  - Technical debt documentation
  - Implicit requirements discovery
  - Decision rationale and history
  - **Use when**: Legacy modernization, major refactoring, compliance requirements

## The 7-Step Brownfield Workflow

**Workflow Overview:**
1. Analyze Existing Codebase
2. Initialize SDD in Existing Project
3. Generate Constitution from Analysis
4. Choose Artifact Generation Strategy
5. Document Existing Features
   - **5a. Validate Reverse-Engineering (Optional)**
6. Specify New Feature
7. Integration-Aware Implementation

### Step 1: Analyze Existing Codebase

**Command:**
```
/speckit.brownfield
```

**Purpose**: Initialize brownfield workflow and analyze existing code

**Example prompt to give user:**
```
/speckit.brownfield Analyze this codebase at moderate depth, focusing on:
- Current architecture and design patterns
- Technology stack and dependencies
- API structure and data models
- Coding standards and conventions
```

**For different depths:**
```
# Surface analysis
/speckit.brownfield Perform a surface-level analysis of this codebase

# Deep analysis
/speckit.brownfield Perform a deep analysis including technical debt,
edge cases, and architectural decision rationale
```

**What this produces:**
- Codebase analysis report
- Identified patterns and conventions
- Tech stack inventory
- Architecture overview

**After this step - Summarize the analysis report:**

Use the **10-point summary template** from SKILL.md to present the codebase analysis.

**Example enhanced summary:**
```
## ✅ Brownfield Analysis Completed - Here's What Just Happened

### 🎯 Key Decisions Made
1. **Microservices architecture identified (5 services)** - Rationale: Modular design allows independent feature development
2. **TypeScript with strict mode across codebase** - Rationale: Strong type safety culture; maintain this in new features
3. **Event-driven communication via RabbitMQ** - Rationale: Async messaging pattern; new features should follow this pattern

### 📋 What Was Generated
- Codebase analysis report: Architecture patterns, tech stack, coding standards, technical debt assessment

### 🔍 Important Items to Review
1. **65% test coverage (target: 80%)** - New features should increase coverage; plan for testing investment
2. **Some services lack error handling** - When adding features, prioritize services with weak error handling
3. **API documentation needs updates** - Document new endpoints comprehensively as part of feature work

### ⚠️ Watch Out For
- **Redux state management complexity** - How to avoid: Consider simpler state solutions for new features or plan Redux refactor
- **RabbitMQ dependency** - How to avoid: Ensure local RabbitMQ setup for development; document in constitution

### 🔄 What This Enables Next
- **Option 1:** Run `specify init --here --force` to set up SDD - Best if: Analysis looks complete
- **Option 2:** Deep dive into specific services - Best if: Need more architectural details
- **Option 3:** Analyze dependencies and integration points - Best if: Planning complex feature integration

📊 **Feature Status:** Analyzing existing codebase (No SDD features yet)
   Progress: Analysis complete → Next: Initialize SDD infrastructure

**Your options:** [A] Initialize SDD [B] Deep dive specific areas [C] Analyze dependencies [D] Adjust analysis depth
```

### Step 2: Initialize SDD in Existing Project

**Command:**
```bash
specify init --here --ai claude
# or for non-empty directories
specify init --here --force --ai claude
```

**Purpose**: Set up SDD infrastructure without disrupting existing code

**What happens:**
- Creates `.speckit/` directory structure
- Installs slash commands in `.claude/commands/`
- Preserves all existing code and configuration
- Initializes Git tracking if not already present

### Step 3: Generate Constitution from Analysis

**Command:**
```
/speckit.analyze-codebase
```

**Purpose**: Create constitution that reflects existing architecture and principles

**Approach**: Hybrid (automated + interactive)
1. AI analyzes code and proposes principles
2. User reviews and refines proposed constitution
3. AI generates final constitution.md

**Example interaction:**
```
AI: Based on my analysis, I observe these patterns:
- RESTful API design with Express.js
- PostgreSQL with Sequelize ORM
- JWT authentication
- MVC architecture
- React frontend with Redux

Should the constitution include these as established patterns? [Y/n]

User: Yes, but we're planning to migrate from Redux to Zustand

AI: Understood. I'll document Redux as current state with Zustand as future direction.
```

**Example prompt to give user:**
```
/speckit.analyze-codebase Generate a constitution that:
- Documents existing architecture patterns
- Acknowledges current tech stack
- Maintains consistency with existing code style
- Notes any planned migrations or changes
```

**After this step - Summarize constitution.md:**

Read `.speckit/constitution.md` and present a summary highlighting:
- Core architectural principles derived from existing code
- Established patterns and conventions
- Technology constraints and standards
- Future direction or planned migrations

**Example summary:**
```
## Constitution Generated from Existing Code ✓

I've created constitution.md based on the existing codebase patterns:

**Architectural Principles:**
- Microservices: Each service owns its domain and database
- API-first: All inter-service communication via RESTful APIs
- Event-driven: Async operations via RabbitMQ message queues
- Stateless services: Session state in Redis, not in-memory

**Technology Standards:**
- TypeScript with strict mode for type safety
- Express.js for API servers
- PostgreSQL for relational data, Redis for caching
- Docker containers, Kubernetes orchestration

**Coding Conventions:**
- Repository pattern for data access
- Service layer for business logic
- Controller layer for API endpoints
- DTOs for request/response validation (Joi schemas)

**Testing Standards:**
- Minimum 80% code coverage (current: 65%)
- Unit tests with Jest
- Integration tests for API endpoints
- E2E tests for critical user flows

**Planned Evolution:**
- Migrate from Redux to Zustand (frontend state management)
- Adopt OpenAPI 3.0 for API documentation
- Implement distributed tracing (Jaeger)

**Next Step:** Choose artifact generation strategy (Step 4)

Would you like to:
A) Proceed to choose documentation strategy
B) Refine any principles
C) Add additional constraints
```

### Step 4: Choose Artifact Generation Strategy

**Interactive decision point** - Ask user which approach:

#### Option A: Constitution Only
```
Generate constitution.md only, use for new features going forward
```
**Best for**: Teams wanting to adopt SDD for future work only

#### Option B: Constitution + Baseline Specs
```
Generate constitution.md and high-level specs for existing major features
```
**Best for**: Documentation backfill while starting new SDD work

#### Option C: Full Artifact Suite
```
Generate constitution, specifications, and technical plans for all existing features
```
**Best for**: Complete SDD adoption, compliance documentation, legacy modernization

#### Option D: Mixed Approach (Recommended)
```
- Constitution for all code
- Full specs for 2-3 core features
- Baseline specs for remaining features
```
**Best for**: Balanced documentation without overwhelming effort

**Command after user chooses:**
```
/speckit.reverse-engineer [option-chosen]
```

**Example prompts:**
```
# Constitution only
/speckit.reverse-engineer constitution-only

# Mixed approach
/speckit.reverse-engineer mixed --core-features authentication,user-management,api-gateway
```

### Step 5: Document Existing Features (Optional)

If user chose Options B, C, or D in Step 4:

**Command:**
```
/speckit.reverse-engineer
```

**Purpose**: Generate SDD artifacts for existing features

**Example prompt to give user:**
```
/speckit.reverse-engineer Document the authentication system:
- User registration and login flows
- JWT token management
- Password reset functionality
- OAuth integration
```

**What this produces:**
```
.speckit/
└── features/
    └── existing/
        ├── 001-authentication/
        │   ├── specify.md      # Reverse-engineered requirements
        │   └── plan.md         # Current implementation details
        ├── 002-user-management/
        └── 003-api-gateway/
```

**After this step - Summarize reverse-engineered specs:**

Read the generated specs for documented features and present a summary highlighting:
- Features that were documented
- Key functionality captured
- Integration points identified
- Any gaps or areas needing clarification

**Example summary:**
```
## Existing Features Documented ✓

I've reverse-engineered specifications for 3 core features:

**1. Authentication System (001-authentication/):**
- User registration with email verification
- Login with JWT tokens (15-minute access, 7-day refresh)
- Password reset via email
- OAuth providers: Google, GitHub
- Rate limiting: 5 failed attempts = 15-minute lockout

**2. User Management (002-user-management/):**
- CRUD operations for user profiles
- Role-based access control (User, Admin, Super Admin)
- User search and filtering
- Bulk operations (export, deactivate)
- Admin endpoints for user impersonation

**3. API Gateway (003-api-gateway/):**
- Request routing to microservices
- Authentication middleware
- Rate limiting (100 req/minute per user)
- Request/response logging
- Error handling and retry logic

**Integration Points Identified:**
- All features use shared PostgreSQL database
- Redis for session storage and rate limiting
- RabbitMQ for async notifications
- Shared authentication middleware

**Gaps to Address:**
- Payment processing workflow incomplete (45% documented)
- Background job system not yet documented
- Admin dashboard missing specifications

**Next Step:** Optionally validate reverse-engineering with `/speckit.validate-reverse-engineering`, then proceed to Step 6 to specify your new feature

Would you like to:
A) Validate the reverse-engineering accuracy
B) Document additional existing features
C) Proceed to specify new feature (Step 6)
```

### Step 5a: Validate Reverse-Engineering (Optional but Recommended)

> **Note**: Validation commands are proposed enhancements to spec-kit. If commands are not yet available, you can perform manual validation by reviewing generated specs against actual code, using code search to verify completeness, cross-checking constitution principles with actual patterns, or creating custom validation scripts.

**When to use**: After documenting existing features, before adding new features

**Purpose**: Verify that reverse-engineering work is accurate, complete, and traceable

This optional step allows you to request validation of the reverse-engineering work to ensure quality before proceeding with new feature development.

#### Reverse-Engineering Accuracy Validation

**Command:**
```
/speckit.validate-reverse-engineering
```

**Purpose**: Verify that generated specs match actual code behavior and implementation

**Example prompt to give user:**
```
/speckit.validate-reverse-engineering Check:
- Do API specs match actual endpoints and contracts?
- Are data models accurately documented?
- Do workflows reflect actual business logic?
- Are technical plans consistent with implementation?
```

**What this validates:**
- Generated specifications describe actual code behavior
- All public APIs are accurately documented
- Data models match actual database schemas
- Architecture diagrams reflect actual structure
- Feature boundaries are correctly identified
- Business logic workflows are accurate

**Example validation report:**
```
✓ Authentication API: 12/12 endpoints documented accurately
✗ User Management: Missing 3 admin endpoints
✓ Data Models: 8/8 models match database schema
⚠ Payment Processing: Workflow partially documented (60% coverage)

Issues Found:
1. Admin endpoints not documented in user-management specs
2. Payment refund workflow missing from specifications
3. Database migration history not captured

Recommendations:
- Run /speckit.reverse-engineer for admin endpoints
- Document payment refund process
- Add migration history to technical plan
```

#### Coverage and Completeness Check

**Command:**
```
/speckit.coverage-check
```

**Purpose**: Report documentation coverage and identify gaps

**Example prompt to give user:**
```
/speckit.coverage-check Report:
- Percentage of codebase documented
- Undocumented features and modules
- Files without corresponding specs
- API endpoint coverage
```

**What this reports:**
- Percentage of codebase with SDD documentation
- List of undocumented features and components
- Source files without corresponding specs
- Gaps in API documentation
- Database tables/models not documented
- Configuration and infrastructure coverage

**Example coverage report:**
```
Overall Coverage: 68%

Feature Coverage:
✓ Authentication: 100% (fully documented)
✓ User Management: 90% (admin section partial)
✗ Payment Processing: 45% (significant gaps)
✗ Reporting: 0% (not documented)
✓ API Gateway: 85% (rate limiting not documented)

File Coverage:
- Documented: 45 files
- Undocumented: 21 files
- Total: 66 files

Undocumented Features:
1. Reporting system (src/reports/)
2. Background job processing (src/jobs/)
3. Email templates (src/templates/email/)
4. Admin dashboard (src/admin/)

Priority: Document payment processing and reporting next
```

#### Constitution Consistency Validation

**Command:**
```
/speckit.validate-constitution
```

**Purpose**: Verify that constitution principles match actual code patterns

**Example prompt to give user:**
```
/speckit.validate-constitution Check for:
- Stated patterns vs actual code patterns
- Tech stack completeness
- Contradictions between principles and code
- Coding standards accuracy
```

**What this validates:**
- Stated design patterns match actual usage
- Technology stack inventory is complete and accurate
- No contradictions between constitution principles and code
- Coding standards reflect actual conventions
- Architecture principles match implementation
- Documented dependencies are current

**Example validation report:**
```
Constitution Accuracy: 85%

✓ Design Patterns:
  - MVC architecture: Correctly identified
  - Repository pattern: Correctly documented

✗ Technology Stack Issues:
  - Constitution says "Redux" but code uses Zustand
  - Missing dependency: bull (job queue)
  - Outdated: PostgreSQL 12 (actually using 14)

✓ Coding Standards:
  - ESLint rules match actual .eslintrc
  - Naming conventions accurate

⚠ Architecture Principles:
  - Constitution mentions "microservices" but codebase is monolithic
  - Claims "stateless API" but sessions are used

Recommendations:
- Update constitution to reflect Zustand usage
- Add bull to tech stack inventory
- Clarify architecture: monolithic with service-oriented design
- Document session usage or migrate to stateless auth
```

#### Spec-to-Code Traceability

**Command:**
```
/speckit.trace [feature-name]
```

**Purpose**: Map specifications to source code bidirectionally

**Example prompt to give user:**
```
/speckit.trace authentication

/speckit.trace Show mapping between payment specs and implementation
```

**What this shows:**
- Which source files implement each feature
- Which specs describe each code module
- Bidirectional navigation (spec → code, code → spec)
- Orphaned specs (documented but not implemented)
- Orphaned code (implemented but not documented)

**Example traceability report:**
```
Feature: Authentication

Specifications:
- .speckit/features/existing/001-authentication/specify.md
- .speckit/features/existing/001-authentication/plan.md

Implementation Files:
✓ src/auth/login.ts (documented)
✓ src/auth/register.ts (documented)
✓ src/auth/jwt.ts (documented)
✓ src/auth/password-reset.ts (documented)
✗ src/auth/oauth-providers.ts (NOT in specs)
✓ src/middleware/auth-middleware.ts (documented)

Database Models:
✓ src/models/User.ts (documented)
✓ src/models/Session.ts (documented)

API Routes:
✓ POST /api/auth/login (documented)
✓ POST /api/auth/register (documented)
✓ POST /api/auth/logout (documented)
✓ POST /api/auth/reset-password (documented)
✗ GET /api/auth/oauth/:provider (NOT in specs)

Issues:
- OAuth integration implemented but not documented
- Consider adding to specifications or marking as deprecated

Traceability Score: 85% (11/13 components mapped)
```

#### Validation Workflow Example

Complete validation workflow after reverse-engineering:

```bash
# Step 1: Validate accuracy of reverse-engineering
/speckit.validate-reverse-engineering

# Review validation report - found 3 missing admin endpoints
# and incomplete payment workflow

# Step 2: Check coverage metrics
/speckit.coverage-check

# Coverage is 68% - reporting system and background jobs not documented
# Decision: Document payment processing now, defer reporting for later

# Step 3: Fill identified gaps
/speckit.reverse-engineer Document payment processing including:
- Payment refund workflow
- Admin endpoints for user management

# Step 4: Validate constitution accuracy
/speckit.validate-constitution

# Found tech stack mismatch (Redux vs Zustand)
# Update constitution.md manually to reflect actual tech stack

# Step 5: Check traceability for critical features
/speckit.trace authentication
/speckit.trace payment-processing

# OAuth integration not documented - add to specs
/speckit.reverse-engineer Document OAuth integration in authentication

# Step 6: Final validation
/speckit.validate-reverse-engineering
# All critical features now validated ✓

# Step 7: Generate validation report for team
/speckit.validation-report
```

#### When to Skip Validation

You can skip validation if:
- Using "constitution-only" strategy (no features to validate)
- Adding small feature to well-understood codebase
- Time-constrained and willing to accept risk
- Plan to validate incrementally as you work

#### When Validation is Critical

Always validate if:
- Using "full artifact suite" strategy
- Working on regulated/compliance-required system
- Large or unfamiliar codebase
- Multiple team members will rely on specs
- High risk of integration issues

### Step 6: Specify New Feature (Standard SDD)

**Command:**
```
/speckit.specify
```

**Purpose**: Define new feature to add to existing codebase

**Example prompt to give user:**
```
/speckit.specify Add a user notification system that:
- Sends email and in-app notifications
- Allows users to configure notification preferences
- Integrates with existing authentication
- Supports notification templates
```

**After this step - Summarize specify.md:**

Follow the same summarization approach as greenfield (see [Greenfield Workflow Step 3](greenfield.md#step-3-create-the-specification)). Highlight:
- Main functional requirements for the new feature
- User stories specific to this feature
- Success criteria
- Integration requirements with existing systems

**Example summary:**
```
## New Feature Specification Created ✓

I've generated specify.md for the notification system:

**Core Features:**
- Email notifications via existing SMTP service
- In-app notification center with real-time updates
- User preference management (per-notification-type settings)
- Template-based notifications with variable substitution

**Integration Requirements:**
- Must use existing authentication/authorization
- Leverage current PostgreSQL database
- Integrate with RabbitMQ for async delivery
- Reuse existing email service configuration

**Success Criteria:**
- Notifications delivered within 5 seconds
- Support 10,000 concurrent users
- 99.9% delivery success rate
- GDPR-compliant preference storage

**Next Step:** Run `/speckit.integration-plan` to plan integration with existing code

Would you like to:
A) Proceed to integration planning
B) Refine requirements
C) Add more integration constraints
```

### Step 7: Integration-Aware Implementation

#### Step 7a: Integration Planning

**Command:**
```
/speckit.integration-plan
```

**Purpose**: Plan how new feature integrates with existing code

**Example prompt to give user:**
```
/speckit.integration-plan How should the notification system integrate with:
- Existing user authentication
- Current database schema
- Email service configuration
- Frontend routing and components
```

**What this produces:**
- Integration points identified
- Required modifications to existing code
- Dependency analysis
- Migration strategy if needed

**After this step - Summarize integration-plan:**

Present a summary highlighting:
- Key integration points with existing systems
- Modifications required to existing code
- Dependencies and sequencing
- Risks and mitigation strategies

**Example summary:**
```
## Integration Plan Created ✓

I've created an integration plan for the notification system:

**Integration Points:**
1. **Authentication:** Reuse JWT middleware for API authentication
2. **Database:** Add `notifications` and `notification_preferences` tables to existing PostgreSQL
3. **Message Queue:** Create new RabbitMQ exchange `notifications` with routing keys
4. **Email Service:** Use existing `EmailService` class, extend with template support

**Required Modifications to Existing Code:**
1. **User Model:** Add `notification_preferences` relationship (non-breaking)
2. **Auth Middleware:** No changes needed (reuse as-is)
3. **Email Service:** Extend with `sendTemplatedEmail()` method
4. **Frontend:** Add NotificationCenter component, integrate with existing layout

**Task Sequencing:**
- Phase 1: Database migrations and models (2 days)
- Phase 2: Backend notification service (3 days)
- Phase 3: RabbitMQ integration (2 days)
- Phase 4: Frontend components (3 days)
- Phase 5: Testing and integration (2 days)

**Dependencies:**
- Must complete database migrations before backend service
- Email template service before notification delivery
- Backend API before frontend components

**Risks & Mitigation:**
- **Risk:** RabbitMQ queue overload during high traffic
  **Mitigation:** Implement rate limiting and queue monitoring
- **Risk:** Email service rate limits
  **Mitigation:** Batch emails, implement retry logic
- **Risk:** Database schema conflicts
  **Mitigation:** Review schema with DBA, use Sequelize migrations

**Next Step:** Run `/speckit.tasks` to break down into actionable tasks

Would you like to:
A) Proceed to task breakdown
B) Adjust integration approach
C) Address specific risks first
```

#### Step 7b: Create Tasks

**Command:**
```
/speckit.tasks
```

**Purpose**: Break down implementation considering integration

**Enhanced for brownfield**: Tasks include integration steps

**After this step - Summarize tasks.md:**

Follow the same summarization approach as greenfield (see [Greenfield Workflow Step 5](greenfield.md#step-5-break-down-into-tasks)). Additionally highlight:
- Tasks that modify existing code (mark clearly)
- Integration-specific tasks
- Testing requirements for integration points

#### Step 7c: Execute Implementation

**Command:**
```
/speckit.implement
```

**Purpose**: Build the feature with integration awareness

## Brownfield-Specific Commands

### `/speckit.brownfield`
Initializes brownfield analysis workflow

**Usage:**
```
/speckit.brownfield [surface|moderate|deep]
```

**Examples:**
```
/speckit.brownfield
/speckit.brownfield deep
/speckit.brownfield Analyze this Express.js API focusing on routing patterns
```

### `/speckit.analyze-codebase`
Deep dive analysis and constitution generation

**Usage:**
```
/speckit.analyze-codebase [focus-areas]
```

**Examples:**
```
/speckit.analyze-codebase
/speckit.analyze-codebase Focus on API design and data models
/speckit.analyze-codebase Include security patterns and authentication flows
```

### `/speckit.reverse-engineer`
Generate SDD artifacts from existing code

**Usage:**
```
/speckit.reverse-engineer [strategy] [options]
```

**Strategies:**
- `constitution-only`: Just generate constitution.md
- `baseline`: Constitution + high-level feature specs
- `full`: Complete artifact suite
- `mixed`: Constitution + full specs for selected features

**Examples:**
```
/speckit.reverse-engineer constitution-only
/speckit.reverse-engineer mixed --core-features auth,payments
/speckit.reverse-engineer full
```

### `/speckit.integration-plan`
Plan new feature integration with existing code

**Usage:**
```
/speckit.integration-plan [context]
```

**Examples:**
```
/speckit.integration-plan
/speckit.integration-plan Consider backward compatibility requirements
/speckit.integration-plan Minimize changes to existing database schema
```

## Artifact Generation Strategies Explained

### Constitution Only
```
.speckit/
└── constitution.md           # Existing patterns & principles
```

**Pros:**
- Quick setup
- Minimal documentation burden
- Focus on new work

**Cons:**
- No documentation for existing features
- Historical context not captured

**Best for:** Small teams, simple codebases, new feature focus

### Constitution + Feature Specs
```
.speckit/
├── constitution.md
└── features/
    └── existing/
        ├── 001-auth/
        │   └── specify.md        # High-level overview
        └── 002-api/
            └── specify.md
```

**Pros:**
- Documents what exists
- Moderate effort
- Provides context for new work

**Cons:**
- Less detail than full suite
- No implementation plans

**Best for:** Medium-sized teams, moderate complexity, documentation backfill

### Full Artifact Suite
```
.speckit/
├── constitution.md
└── features/
    └── existing/
        ├── 001-auth/
        │   ├── specify.md        # Detailed requirements
        │   ├── plan.md           # Implementation details
        │   └── architecture.md   # Design decisions
        └── 002-api/
            └── ...
```

**Pros:**
- Complete documentation
- Deep understanding captured
- Compliance-ready
- Excellent for onboarding

**Cons:**
- Significant time investment
- Can be overwhelming

**Best for:** Large teams, complex systems, regulated industries, legacy modernization

### Baseline for New Work (Mixed)
```
.speckit/
├── constitution.md           # All existing code
└── features/
    ├── existing/
    │   └── 001-core-auth/    # Full specs for 2-3 key features
    │       ├── specify.md
    │       ├── plan.md
    │       └── architecture.md
    └── new/
        └── 001-notifications/ # Standard SDD for new work
```

**Pros:**
- Balanced approach
- Key features documented deeply
- Manageable scope
- Supports both existing and new

**Cons:**
- Requires choosing what to document
- Inconsistent depth

**Best for:** Most brownfield scenarios, pragmatic teams

## Best Practices for Brownfield

### 1. Start Small, Prove Value
```bash
# Don't try to document everything at once
# Pick one new feature to add using SDD
specify init --here --ai claude
/speckit.brownfield surface
/speckit.analyze-codebase
/speckit.specify Add password strength indicator
/speckit.integration-plan
/speckit.tasks
/speckit.implement
```

### 2. Respect Existing Patterns
Your constitution should acknowledge current architecture:
```markdown
## Existing Architecture Principles
- RESTful API design with Express.js (maintain)
- PostgreSQL with Sequelize ORM (maintain)
- JWT authentication (maintain)
- React frontend with Redux (migrating to Zustand)

## New Feature Guidelines
- Follow existing REST conventions
- Integrate with current auth system
- Use Sequelize for data access
- Prefer Zustand for new state management
```

### 3. Incremental Constitution Evolution
```markdown
## Constitution Version History

### v1.0 - Initial (2024-01-15)
- Documented existing patterns
- Established code quality baselines

### v1.1 - First New Feature (2024-02-01)
- Added notification system principles
- Defined event-driven patterns

### v1.2 - Tech Stack Evolution (2024-03-15)
- Documented Redux → Zustand migration
- Updated state management guidelines
```

### 4. Integration Testing Focus
Brownfield implementations need extra testing:
```markdown
## Integration Test Requirements
- Verify compatibility with existing auth
- Test database migration rollback
- Validate API backward compatibility
- Check frontend routing integration
```

### 5. Validate Before Building
Before adding new features, validate your reverse-engineering work:
```bash
# After reverse-engineering existing features
/speckit.validate-reverse-engineering
# Review report, identify inaccuracies

/speckit.coverage-check
# Check coverage percentage, find gaps

/speckit.validate-constitution
# Ensure constitution matches reality

/speckit.trace authentication
# Verify traceability for critical features

# Fix any identified issues before proceeding
/speckit.reverse-engineer --fill-gaps [missed-features]
```

**Benefits:**
- Catch documentation errors early
- Ensure specs match reality
- Build confidence in SDD artifacts
- Prevent integration issues later

### 6. Document Technical Debt
Use SDD to track and plan debt reduction:
```markdown
## Known Technical Debt
- Auth system lacks rate limiting → Plan in 002-security-hardening
- Database queries not optimized → Address in 003-performance
- No input validation on older endpoints → Fix in 004-validation
```

## Common Brownfield Scenarios

### Scenario 1: Adding Feature to Production App

```bash
# Setup
cd existing-app
specify init --here --force --ai claude

# Workflow
/speckit.brownfield moderate
/speckit.analyze-codebase Focus on payment processing and user auth
/speckit.reverse-engineer constitution-only
/speckit.specify Add subscription management feature
/speckit.integration-plan Consider existing payment gateway integration
/speckit.tasks
/speckit.implement
```

### Scenario 2: Legacy System Modernization

```bash
# Setup
cd legacy-system
specify init --here --ai claude

# Workflow
/speckit.brownfield deep --include-tech-debt
/speckit.analyze-codebase Document all major subsystems
/speckit.reverse-engineer full
# Review all generated specs
/speckit.specify Modernize authentication to OAuth 2.0
/speckit.integration-plan Migration strategy with zero downtime
/speckit.tasks
/speckit.implement
```

### Scenario 3: Microservice Extraction

```bash
# Setup
cd monolith
specify init --here --ai claude

# Workflow
/speckit.brownfield moderate
/speckit.analyze-codebase Focus on notification subsystem
/speckit.reverse-engineer mixed --core-features notifications
/speckit.specify Extract notifications as independent microservice
/speckit.integration-plan API contracts and message queues
/speckit.tasks Include data migration and API versioning
/speckit.implement
```

### Scenario 4: Compliance Documentation

```bash
# Setup (existing compliant system needs documentation)
cd production-app
specify init --here --ai claude

# Workflow
/speckit.brownfield deep
/speckit.analyze-codebase Include security controls and data handling
/speckit.reverse-engineer full --compliance-mode
# Generate comprehensive documentation for audit
```

## Brownfield Troubleshooting

### Issue: Constitution Conflicts with Existing Code

**Problem**: Generated constitution suggests patterns that don't match existing code

**Solution**:
1. Use `/speckit.analyze-codebase` with more specific guidance
2. Manually review and edit constitution.md
3. Add "Existing Patterns" vs "New Patterns" sections
4. Document migration path if changing patterns

**Example Fix**:
```markdown
## Current State (Maintain)
- Class-based React components
- Redux for state management

## Future Direction (New Features)
- Functional components with hooks
- Zustand for state management

## Migration Strategy
- Maintain existing code as-is
- New features use modern patterns
- Refactor opportunistically
```

### Issue: Too Many Features to Document

**Problem**: Large codebase with dozens of features, overwhelming to document all

**Solution**:
1. Use `constitution-only` strategy initially
2. Document features incrementally as you touch them
3. Prioritize: core features → frequently changed → rarely touched

**Example Approach**:
```bash
# Start minimal
/speckit.reverse-engineer constitution-only

# Add core features only
/speckit.reverse-engineer Document just authentication and payment processing

# Add more as you go
# When working on user-profile feature next month:
/speckit.reverse-engineer Document user profile management
```

### Issue: Integration Points Not Clear

**Problem**: `/speckit.integration-plan` doesn't identify all integration points

**Solution**:
1. Run deeper analysis: `/speckit.brownfield deep`
2. Explicitly list integration concerns in prompt
3. Review generated plan and add missing points manually

**Example**:
```
/speckit.integration-plan Consider:
- Database schema changes (users table, notifications table)
- Existing API routes (/api/users)
- Authentication middleware
- Frontend routing (React Router)
- Email service configuration
- Background job processing
```

### Issue: Existing Code Quality Issues

**Problem**: Existing code has quality issues, constitution documents them

**Solution**:
1. Use constitution to document current state honestly
2. Add "Quality Improvement Plan" section
3. Address issues incrementally with new features

**Example Constitution Section**:
```markdown
## Current Code Quality Issues
- Inconsistent error handling
- Missing input validation in older endpoints
- No automated testing for legacy features

## Quality Improvement Plan
- New features: 100% test coverage required
- When modifying existing code: Add tests for modified functions
- Quarterly: Address one legacy area with full testing

## New Feature Standards
- Full unit and integration test coverage
- Input validation on all endpoints
- Consistent error handling with error codes
```

### Issue: Validation Reveals Inaccuracies

**Problem**: `/speckit.validate-reverse-engineering` shows specs don't match actual code

**Solution**:
1. Review validation report for specific discrepancies
2. Re-run `/speckit.reverse-engineer` with more specific guidance
3. Manually edit specs in `.speckit/features/existing/`
4. Re-validate until accuracy is acceptable

**Example**:
```bash
# Initial validation shows issues
/speckit.validate-reverse-engineering
# Report: "User Management: Missing 3 admin endpoints"

# Fix the gap
/speckit.reverse-engineer Document admin endpoints in user management:
- POST /api/admin/users/:id/suspend
- POST /api/admin/users/:id/restore
- DELETE /api/admin/users/:id/permanent-delete

# Re-validate
/speckit.validate-reverse-engineering
# Report: "User Management: All endpoints documented ✓"
```

### Issue: Low Coverage Percentage

**Problem**: `/speckit.coverage-check` shows <50% coverage

**Solution**:
1. Decide on target coverage based on strategy (constitution-only vs full)
2. Use `/speckit.reverse-engineer --fill-gaps` for priority features
3. Document incrementally as you work on each area
4. Focus on documenting features you'll be modifying

**Example**:
```bash
# Check coverage
/speckit.coverage-check
# Report: "Overall Coverage: 45%"

# Decide on strategy
# Option A: Acceptable for constitution-only approach
# Option B: Need higher coverage for full artifact suite

# Fill gaps for features you'll work on
/speckit.reverse-engineer --fill-gaps payment-processing,reporting
/speckit.coverage-check
# Report: "Overall Coverage: 68%"
```

### Issue: Constitution Validation Fails

**Problem**: `/speckit.validate-constitution` shows contradictions between principles and code

**Solution**:
1. Review validation report for specific conflicts
2. Decide: Update constitution or plan to change code
3. Document migration path if planning to change code
4. Re-validate after updates

**Example**:
```bash
# Validate constitution
/speckit.validate-constitution
# Report: "Constitution says 'Redux' but code uses Zustand"

# Fix: Update constitution to match reality
# Edit .speckit/constitution.md:
# Change: "Redux for state management"
# To: "Zustand for state management (migrated from Redux in v2.0)"

# Re-validate
/speckit.validate-constitution
# Report: "Constitution Accuracy: 95% ✓"
```

### Issue: Traceability Gaps

**Problem**: `/speckit.trace` shows orphaned code or orphaned specs

**Solution**:
1. For orphaned code (implemented but not documented):
   - Document with `/speckit.reverse-engineer`
   - Or mark as deprecated if planned for removal
2. For orphaned specs (documented but not implemented):
   - Remove from specs if feature was removed
   - Or flag as planned future implementation

**Example**:
```bash
# Check traceability
/speckit.trace authentication
# Report: "OAuth integration implemented but not documented"

# Fix: Document the orphaned code
/speckit.reverse-engineer Document OAuth integration in authentication

# Or if OAuth should be removed:
# Update specs to mark OAuth as deprecated
# Add to technical debt list
```

## Command Reference for Brownfield

| Command | Purpose | When to Use |
|---------|---------|-------------|
| `specify init --here --force` | Initialize in existing project | First step in brownfield |
| `/speckit.brownfield` | Analyze codebase | After init, before constitution |
| `/speckit.analyze-codebase` | Deep analysis & constitution | After brownfield analysis |
| `/speckit.reverse-engineer` | Generate artifacts for existing code | Documenting existing features |
| `/speckit.validate-reverse-engineering` | Verify spec accuracy | After reverse-engineering, before new work |
| `/speckit.coverage-check` | Check documentation coverage | After reverse-engineering |
| `/speckit.validate-constitution` | Verify constitution consistency | After constitution generation |
| `/speckit.trace [feature]` | Map specs to code | Any time, for traceability |
| `/speckit.specify` | Define new feature | After constitution, start new work |
| `/speckit.integration-plan` | Plan integration | After specify, before tasks |
| `/speckit.tasks` | Break down implementation | After integration planning |
| `/speckit.implement` | Execute tasks | Final implementation step |

## Next Steps After Brownfield Setup

1. **Validate Setup**: Review generated constitution and artifacts
2. **Add First Feature**: Use standard SDD workflow for new feature
3. **Iterate Constitution**: Refine principles as you learn
4. **Expand Documentation**: Gradually document more existing features
5. **Team Adoption**: Share workflows with team members
6. **Measure Impact**: Track time saved, bugs reduced, onboarding speed
