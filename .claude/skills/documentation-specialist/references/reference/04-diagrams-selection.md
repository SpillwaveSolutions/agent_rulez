# Diagrams: Choosing the Right Diagram

---
TOKEN_BUDGET: 220
TIER: 3
LOAD_TRIGGER: On-demand when selecting diagram type for documentation
DEPENDENCIES: None
---

## 4.1 Choosing the Right Diagram

### Decision Matrix

| Need | Recommended Diagram | Tool |
|------|---------------------|------|
| Show system boundary and external actors | C4 Context or DFD Level 0 | Mermaid |
| Show deployable containers | C4 Container | Mermaid |
| Show internal components | C4 Component or UML Component | Mermaid/PlantUML |
| Show object interactions over time | Sequence Diagram | PlantUML |
| Show object lifecycle and states | State Machine Diagram | PlantUML |
| Show workflow or business process | Activity Diagram or Flowchart | PlantUML/Mermaid |
| Show database schema | ER Diagram | PlantUML |
| Show class relationships | Class Diagram | PlantUML |
| Show infrastructure topology | Deployment Diagram | PlantUML |

### Common Scenarios

**Scenario 1: Documenting a new microservices architecture**
1. Start with C4 Context (show system boundary)
2. Add C4 Container (show each microservice)
3. For complex services, add C4 Component (show internal structure)
4. Add Sequence diagrams for key workflows

**Scenario 2: Documenting a REST API**
1. OpenAPI specification (primary documentation)
2. Sequence diagrams for authentication flow
3. State Machine for resource lifecycle (e.g., Order states)

**Scenario 3: Documenting database design**
1. ER Diagram (entities and relationships)
2. Add notes for constraints, indexes

**Scenario 4: Onboarding developers to legacy code**
1. C4 Context (where does this fit in the ecosystem?)
2. C4 Component (what are the main modules?)
3. Class diagrams for complex OOP hierarchies
4. Sequence diagrams for critical workflows

---

**End of Diagram Selection Guide**
