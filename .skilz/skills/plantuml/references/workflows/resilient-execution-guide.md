# Resilient PlantUML Execution Guide

## Overview

This guide documents the 4-step resilient workflow for diagram generation. Use this workflow for all PlantUML diagram creation to ensure consistent naming, proper error handling, and validated output.

**Token Budget**: ~1,500 tokens (load on demand for diagram creation tasks)

## Prerequisites

Before starting, verify environment:
```bash
python scripts/check_setup.py
```

This checks Java, Graphviz, and plantuml.jar availability.

---

## The 4-Step Resilient Workflow

### Step 1: Diagram Type Identification & Reference Lookup

**Automatic (Script)**:
```bash
python scripts/resilient_processor.py input.md
```
The script analyzes `@start` tags and content to identify diagram type.

**Manual (Agent Workflow)**:

1. Identify diagram type from user intent:
   | Intent Keywords | Diagram Type | Reference File |
   |-----------------|--------------|----------------|
   | API, messages, interactions | sequence | sequence_diagrams.md |
   | OOP, inheritance, classes | class | class_diagrams.md |
   | database, schema, entities | er | er_diagrams.md |
   | workflow, process, decision | activity | activity_diagrams.md |
   | states, transitions | state | state_diagrams.md |
   | architecture, modules | component | component_diagrams.md |
   | servers, infrastructure | deployment | deployment_diagrams.md |
   | timeline, schedule | gantt | gantt_diagrams.md |

2. Load appropriate reference:
   ```
   Read: references/[diagram_type]_diagrams.md
   ```

3. If ambiguous, consult `references/toc.md`

---

### Step 2: Structured File Creation

**Naming Convention**:
```
./diagrams/<markdown_name>_<num>_<type>_<title>.puml
```

**Components**:
| Component | Format | Example |
|-----------|--------|---------|
| markdown_name | source file (no ext) | architecture |
| num | zero-padded (001-999) | 001 |
| type | diagram type | sequence |
| title | first 10 chars, sanitized | user_authe |

**Examples**:
```
./diagrams/architecture_001_sequence_user_authe.puml
./diagrams/architecture_002_class_order_servi.puml
./diagrams/api_design_001_er_customer_dat.puml
```

**Directory Structure**:
```
project/
├── article.md
└── diagrams/
    ├── article_001_sequence_login_flow.puml
    ├── article_001_sequence_login_flow.png
    ├── article_002_class_user_model.puml
    └── article_002_class_user_model.png
```

**Script Usage**:
```bash
# Creates diagrams/ directory automatically
python scripts/resilient_processor.py article.md --format png
```

---

### Step 3: Error Handling & Recovery

**Error Resolution Flow**:
```
Conversion Failed
      |
      v
+---------------------------+
| 1. Parse error message    |
+-------------+-------------+
              |
              v
+---------------------------+
| 2. Load troubleshooting/  |
|    toc.md (~400 tokens)   |
+-------------+-------------+
              |
              v
+---------------------------+
| 3. Classify error:        |
|    - Setup/Installation   |
|    - Syntax               |
|    - Diagram-specific     |
|    - Styling              |
|    - Performance          |
+-------------+-------------+
              |
              v
+---------------------------+
| 4. Load specific guide    |
|    (~500-1000 tokens)     |
+-------------+-------------+
              |
              v
+---------------------------+
| 5. Apply fix & retry      |
|    (max 3 attempts)       |
+-------------+-------------+
              |
        Still failing?
              |
              v
+---------------------------+
| 6. Load common_syntax_    |
|    errors.md section      |
+-------------+-------------+
              |
        Still failing?
              |
              v
+---------------------------+
| 7. External search:       |
|    1. Perplexity          |
|    2. Brave Search        |
|    3. Gemini skill        |
|    4. WebSearch tool      |
+---------------------------+
```

**Error Pattern -> Guide Mapping**:

| Error Message | Troubleshooting Guide | Error # |
|---------------|----------------------|---------|
| "Cannot find java" | installation_setup_guide.md | 1 |
| "No Dot executable found" | installation_setup_guide.md | 2 |
| "GraphViz has crashed" | installation_setup_guide.md | 3 |
| "Unable to access jarfile" | installation_setup_guide.md | 4 |
| "HeadlessException" | installation_setup_guide.md | 6 |
| "No @startuml/@enduml found" | general_syntax_guide.md | 1 |
| "Syntax Error" (with line) | general_syntax_guide.md | - |
| "Duplicate participant" | sequence_diagrams_guide.md | 3 |
| "Empty alt group" | sequence_diagrams_guide.md | 11 |
| "Cannot include file" | preprocessor_includes_guide.md | 1 |
| "File already included" | preprocessor_includes_guide.md | 5 |
| "Stack overflow" | preprocessor_includes_guide.md | 8 |
| "Failed to generate image" | image_generation_guide.md | 1 |
| "NullPointerException" | general_syntax_guide.md | 15 |

**External Search Queries** (when internal guides fail):
```
"PlantUML error: [first 50 chars of error message]"
"How to fix PlantUML [error type] syntax error"
"PlantUML [error message] stackoverflow solution"
```

**Search Tool Priority**:
1. `mcp__perplexity-ask__perplexity_ask` - AI-powered, best for understanding
2. `mcp__brave-search__brave_web_search` - Web search, good coverage
3. `gemini` skill - Alternative AI perspective
4. `WebSearch` tool - Fallback

---

### Step 4: Validation & Markdown Integration

**Validation Checklist**:
- [ ] Image file exists at expected path
- [ ] Image file size > 0 bytes
- [ ] Image format matches request (PNG/SVG)

**Only after validation passes**:

1. Generate markdown link:
   ```markdown
   ![User Authentication Flow](diagrams/architecture_001_sequence_user_authe.png)
   ```

2. Replace original ```puml block with image link

3. **Keep .puml source file** (do NOT delete - needed for version control)

4. Save updated markdown as `*_with_images.md`

---

## Script Usage

### Process Markdown File (Recommended)
```bash
python scripts/resilient_processor.py article.md --format png
```

### Process Single Diagram
```bash
python scripts/resilient_processor.py diagram.puml --format svg
```

### Validate Only (No Conversion)
```bash
python scripts/resilient_processor.py article.md --validate-only
```

### Verbose Output
```bash
python scripts/resilient_processor.py article.md -v
```

---

## Agent Manual Workflow

When agents follow this workflow without the script:

### 1. Identify & Load Reference
```
Identify diagram type from user request
Read references/[type]_diagrams.md
```

### 2. Create File
```
Create diagrams/ directory if needed
Write diagram to: ./diagrams/<md>_<num>_<type>_<title>.puml
```

### 3. Convert & Handle Errors
```bash
python scripts/convert_puml.py diagrams/file.puml --format png --output-dir diagrams/
```

**If error occurs**:
```
Read references/troubleshooting/toc.md
Identify error category from decision tree
Read references/troubleshooting/[category]_guide.md
Apply suggested fix
Retry (max 3 times)
```

### 4. Validate & Integrate
```
Verify diagrams/file.png exists
Generate: ![title](diagrams/file.png)
Update markdown file
```

---

## Token Budget Summary

| Resource | Tokens | When to Load |
|----------|--------|--------------|
| This guide | ~1,500 | Workflow start |
| Diagram reference | 1,500-3,000 | Step 1 |
| troubleshooting/toc.md | ~400 | Error occurs |
| Specific troubleshooting guide | 500-1,000 | After classification |
| common_syntax_errors.md section | ~500 | Persistent errors |

**Target**: <10,000 tokens per diagram request

---

## Best Practices

1. **Always use structured naming** - Enables traceability and organization
2. **Keep .puml sources** - Version control friendly, enables future edits
3. **Check setup first** - Run check_setup.py before batch processing
4. **Default to PNG** - Use SVG only when user requests
5. **Log errors** - Track recurring issues for improvement
6. **Backup markdown** - Script creates *_with_images.md, preserves original
7. **Retry before searching** - Internal guides cover 90%+ of common errors
