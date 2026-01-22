# FastAPI Application to PlantUML Diagrams

This directory contains examples of converting FastAPI application code into various PlantUML diagrams.

## Overview

Common FastAPI patterns and their corresponding PlantUML representations:

- **Deployment Diagram**: Shows FastAPI deployment with async workers (Uvicorn/Gunicorn)
- **Component Diagram**: Illustrates routers, dependencies, middleware, and database connections
- **Sequence Diagram**: Documents async request flows, authentication, and background tasks
- **Class Diagram**: Maps Pydantic models, database schemas, and relationships

## Unicode Symbols for FastAPI

Use these semantic symbols in your diagrams:

- `⚡` - FastAPI/ASGI/Async processing
- `🚀` - Uvicorn/Hypercorn server
- `🔒` - OAuth2/JWT Security
- `📦` - SQLAlchemy ORM/Database
- `🎯` - API Router/Endpoint
- `💼` - Service/Business logic
- `🔌` - External API/HTTP client
- `💾` - PostgreSQL/Database
- `🔑` - API Key/Bearer token
- `⏱️` - Background tasks
- `📊` - Metrics/Monitoring
- `🐍` - Python async/await
- `📝` - Pydantic validation

## Example Application Structure

```
app/
├── main.py
├── api/
│   ├── __init__.py
│   ├── v1/
│   │   ├── endpoints/
│   │   │   ├── users.py
│   │   │   ├── orders.py
│   │   │   └── products.py
│   │   └── router.py
├── core/
│   ├── config.py
│   ├── security.py
│   └── dependencies.py
├── models/
│   └── domain.py
├── schemas/
│   ├── user.py
│   ├── order.py
│   └── product.py
├── db/
│   ├── base.py
│   └── session.py
└── services/
    ├── user_service.py
    └── order_service.py
```

See the example files in this directory for diagram mappings.
