# Diagram Generation Workflow

---
TOKEN_BUDGET: 350
TIER: 2
LOAD_TRIGGER: Intent = DIAGRAM
DEPENDENCIES: SKILL.md
---

## Overview

Generate visual documentation using Mermaid and PlantUML.

## Workflow Steps

### Step 1: Identify Diagram Type

| Keywords | Diagram | Tool |
|----------|---------|------|
| "C4 context", "system overview" | C4 Context | mermaid-architect |
| "C4 container", "services" | C4 Container | mermaid-architect |
| "C4 component", "modules" | C4 Component | mermaid-architect |
| "sequence", "flow" | Sequence | plantuml |
| "ER", "database", "entities" | ER Diagram | plantuml |
| "state", "lifecycle" | State Machine | plantuml |
| "activity", "process" | Activity | plantuml |
| "class", "inheritance" | Class | plantuml |
| "flowchart" | Flowchart | mermaid-architect |

### Step 2: Gather Content

**C4 Diagrams**: System name, actors, containers, relationships
**Sequence**: Participants, ordered steps
**ER**: Entities, attributes, relationships
**State Machine**: Entity, states, transitions

### Step 3: Invoke Skill

**C4 (mermaid-architect)**:
```
Task: Generate C4 [level] diagram
System: [name]
Elements: [list]
Relationships: [list]
Output: docs/diagrams/[filename].md
```

**UML (plantuml)**:
```
Task: Generate [type] diagram
Elements: [entities/participants/states]
Relationships: [connections]
Output: docs/diagrams/[filename].puml
```

### Step 4: Present Rendering Options

Present options to user:
```
Generated [diagram type]: [path]

Options:
1. Render to PNG/SVG
2. Generate related diagrams
3. Embed in documentation
```

## Quick Reference

| Show... | Use | Tool |
|---------|-----|------|
| External dependencies | C4 Context | mermaid-architect |
| Microservices/DBs | C4 Container | mermaid-architect |
| Internal modules | C4 Component | mermaid-architect |
| API call flow | Sequence | plantuml |
| Database schema | ER Diagram | plantuml |
| Entity lifecycle | State Machine | plantuml |
| Business process | Activity | plantuml |

## Selection Guide

**State Machine vs Activity**:
- **State Machine**: Entity states, transitions (Order: Pending → Shipped)
- **Activity**: Process steps, decisions (Registration flow)

For detailed guidance: [04-diagrams-selection.md](../reference/04-diagrams-selection.md)
