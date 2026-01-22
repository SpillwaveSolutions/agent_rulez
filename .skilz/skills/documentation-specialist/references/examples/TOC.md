# Documentation Examples - Table of Contents

---
TOKEN_BUDGET: 150
TIER: 3
LOAD_TRIGGER: On-demand when user needs examples or few-shot learning
DEPENDENCIES: None
---

## Overview

This directory contains **complete, working examples** of professional software documentation. Use these as few-shot examples when generating documentation.

**Load on-demand**: Reference specific examples by their links below, not all at once.

---

## 🟢 Greenfield Examples (Template-Based)

Examples of creating documentation from scratch using templates.

### Requirements Documentation

**1. Software Requirements Specification (SRS)**
- [greenfield/billing-srs.md](greenfield/billing-srs.md) - Payment processing system (IEEE 830 compliant)
- **Domain**: E-commerce, payments
- **Size**: ~1,200 tokens
- **Use for**: High-risk, compliance-critical features

**2. Product Requirements Document (PRD)**
- [greenfield/collaboration-prd.md](greenfield/collaboration-prd.md) - Team collaboration platform
- **Domain**: SaaS, productivity
- **Size**: ~1,000 tokens
- **Use for**: Agile feature development

### API Documentation

**3. OpenAPI 3.0 Specification**
- [greenfield/task-api-openapi.yaml](greenfield/task-api-openapi.yaml) - Task management REST API
- **Domain**: Task management
- **Size**: ~1,500 tokens
- **Use for**: REST API documentation

### Design Documentation

**4. Software Design Document (SDD)**
- [greenfield/ecommerce-sdd.md](greenfield/ecommerce-sdd.md) - Microservices e-commerce architecture
- **Domain**: E-commerce, microservices
- **Size**: ~2,000 tokens
- **Use for**: System architecture documentation

**5. Architecture Decision Record (ADR)**
- [greenfield/adr-microservices.md](greenfield/adr-microservices.md) - Choosing microservices over monolith
- **Domain**: Architecture
- **Size**: ~400 tokens
- **Use for**: Recording architectural decisions

---

## 🟤 Brownfield Examples (Code-to-Docs)

Examples of documentation reverse-engineered from existing codebases.

### Spring Boot Application
- [brownfield/spring-boot-petclinic/](brownfield/spring-boot-petclinic/) - Pet Clinic application
  - `sdd.md` - Software Design Document
  - `openapi.yaml` - Generated API specification
  - `diagrams/` - C4, ER, Sequence diagrams

### FastAPI Application
- [brownfield/fastapi-todo-app/](brownfield/fastapi-todo-app/) - Todo REST API
  - `api-docs.md` - API documentation
  - `openapi.yaml` - Generated OpenAPI spec

### Pulumi Infrastructure
- [brownfield/pulumi-aws-infra/](brownfield/pulumi-aws-infra/) - AWS infrastructure
  - `deployment-docs.md` - Deployment documentation
  - `architecture-diagram.md` - Infrastructure architecture

### React Dashboard
- [brownfield/react-dashboard/](brownfield/react-dashboard/) - Analytics dashboard
  - `component-docs.md` - Component documentation
  - `user-guide.md` - User documentation

### Python ETL Pipeline
- [brownfield/python-etl-pipeline/](brownfield/python-etl-pipeline/) - Data pipeline
  - `pipeline-docs.md` - Pipeline documentation
  - `deployment.md` - Deployment guide

---

## 📋 How to Use These Examples

### For Greenfield Documentation (Template-Based)

**When creating an SRS:**
1. Read `reference/02-requirements-srs-vs-prd.md` for guidance
2. Load `examples/greenfield/billing-srs.md` as few-shot example
3. Use template from `templates/markdown/requirements-srs.md`
4. Customize with user's context

**When creating a PRD:**
1. Read `reference/02-requirements-srs-vs-prd.md` for guidance
2. Load `examples/greenfield/collaboration-prd.md` as few-shot example
3. Use template from `templates/markdown/requirements-prd.md`
4. Customize with user's context

**When creating OpenAPI spec:**
1. Read `reference/05-api-openapi.md` for guidance
2. Load `examples/greenfield/task-api-openapi.yaml` as few-shot example
3. Use template from `templates/markdown/api-openapi.yaml`
4. Customize with user's API

**When creating an SDD:**
1. Read `reference/03-design-arc42.md` for guidance
2. Load `examples/greenfield/ecommerce-sdd.md` as few-shot example
3. Use template from `templates/markdown/design-sdd.md`
4. Customize with user's architecture

---

### For Brownfield Documentation (Code-to-Docs)

**When documenting a Spring Boot app:**
1. Read `reference/09-code-to-docs-workflow.md`
2. Load `examples/brownfield/spring-boot-petclinic/sdd.md` as example output
3. Follow brownfield-workflow.md extraction steps
4. Generate similar documentation

**When documenting a FastAPI app:**
1. Read `reference/09-code-to-docs-detection.md`
2. Load `examples/brownfield/fastapi-todo-app/openapi.yaml` as example output
3. Extract from code using brownfield-workflow.md
4. Generate similar documentation

---

## 🎯 Example Selection Guide

| User Request | Load This Example |
|--------------|-------------------|
| "Create an SRS for a payment system" | `greenfield/billing-srs.md` |
| "Create a PRD for a collaboration tool" | `greenfield/collaboration-prd.md` |
| "Generate OpenAPI spec for a REST API" | `greenfield/task-api-openapi.yaml` |
| "Document microservices architecture" | `greenfield/ecommerce-sdd.md` |
| "Record an architectural decision" | `greenfield/adr-microservices.md` |
| "Document my Spring Boot app" | `brownfield/spring-boot-petclinic/sdd.md` |
| "Extract API docs from FastAPI" | `brownfield/fastapi-todo-app/openapi.yaml` |
| "Document my infrastructure" | `brownfield/pulumi-aws-infra/deployment-docs.md` |

---

## 📊 Token Budgets

### Greenfield Examples
- SRS example: ~1,200 tokens
- PRD example: ~1,000 tokens
- OpenAPI example: ~1,500 tokens
- SDD example: ~2,000 tokens
- ADR example: ~400 tokens

**Total greenfield**: ~6,100 tokens (load selectively, not all at once)

### Brownfield Examples
- Spring Boot example: ~2,000 tokens
- FastAPI example: ~1,500 tokens
- Pulumi example: ~1,000 tokens
- React example: ~800 tokens
- Python ETL example: ~800 tokens

**Total brownfield**: ~6,100 tokens (load selectively, not all at once)

---

## 🔄 Progressive Loading Strategy

**Do NOT load all examples upfront!**

**Instead, use this pattern:**

1. **Classify intent**: CREATE_NEW or CODE_TO_DOCS
2. **Identify document type**: SRS, PRD, OpenAPI, SDD
3. **Load ONE relevant example**: Use table above
4. **Generate documentation**: Apply pattern from example

**Example workflow:**
```
User: "Create an SRS for a healthcare billing system"

Step 1: Intent = CREATE_NEW, DocType = SRS
Step 2: Load reference/02-requirements-srs-vs-prd.md (200 tokens)
Step 3: Load examples/greenfield/billing-srs.md (1,200 tokens)
Step 4: Load templates/markdown/requirements-srs.md (500 tokens)
Step 5: Generate customized SRS for healthcare billing
Total tokens: 1,900 (vs loading all 12,200 tokens)
```

---

**End of Examples TOC**
