# PlantUML Troubleshooting Guide - Table of Contents

Comprehensive troubleshooting resources for common PlantUML errors, organized by category.

## Quick Navigation

### Setup & Environment
- [Installation & Setup Guide](installation_setup_guide.md) - Java, Graphviz, plantuml.jar configuration

### Syntax & Fundamentals
- [General Syntax Guide](general_syntax_guide.md) - Delimiters, comments, basic structure
- [Arrows & Relationships Guide](arrows_relationships_guide.md) - Arrow syntax, connectors, relationship types
- [Text & Labels Guide](text_labels_guide.md) - Quotes, special characters, text formatting

### Styling & Appearance
- [Styling & Themes Guide](styling_themes_guide.md) - skinparam, style blocks, colors, fonts

### Advanced Features
- [Preprocessor & Includes Guide](preprocessor_includes_guide.md) - !include, !define, !procedure, file paths

### Diagram-Specific Guides
- [Sequence Diagrams Guide](sequence_diagrams_guide.md) - Participants, arrows, activations, fragments
- [Class Diagrams Guide](class_diagrams_guide.md) - Classes, relationships, visibility, generics
- [ER Diagrams Guide](er_diagrams_guide.md) - Entities, relationships, cardinality
- [Activity Diagrams Guide](activity_diagrams_guide.md) - Flow control, partitions, forks, loops

### Output & Performance
- [Image Generation Guide](image_generation_guide.md) - Rendering issues, output formats
- [Performance Guide](performance_guide.md) - Timeouts, memory issues, large diagrams

## Error Decision Tree

**Start Here: What type of problem are you experiencing?**

### 1. PlantUML Won't Run At All
- "Cannot find java!" → [Installation & Setup Guide](installation_setup_guide.md) #1
- "Unable to access jarfile" → [Installation & Setup Guide](installation_setup_guide.md) #4
- "HeadlessException" → [Installation & Setup Guide](installation_setup_guide.md) #6

### 2. Syntax Errors
- "Some diagram description contains errors" → Check diagram-specific guide
- Missing delimiters (@startuml/@enduml) → [General Syntax Guide](general_syntax_guide.md) #1
- Arrow syntax errors → [Arrows & Relationships Guide](arrows_relationships_guide.md)
- Text/label errors → [Text & Labels Guide](text_labels_guide.md)

### 3. Diagram Type Specific Issues

**Sequence Diagrams:**
- Participant errors → [Sequence Diagrams Guide](sequence_diagrams_guide.md) #1-3
- Arrow/message problems → [Sequence Diagrams Guide](sequence_diagrams_guide.md) #4-7
- Fragment errors (alt/loop/opt) → [Sequence Diagrams Guide](sequence_diagrams_guide.md) #10-12

**Class Diagrams:**
- Relationship syntax → [Class Diagrams Guide](class_diagrams_guide.md) #1-5
- Visibility modifiers → [Class Diagrams Guide](class_diagrams_guide.md) #6-8
- Generics/interfaces → [Class Diagrams Guide](class_diagrams_guide.md) #9-12

**ER Diagrams:**
- Entity syntax → [ER Diagrams Guide](er_diagrams_guide.md) #1-3
- Cardinality notation → [ER Diagrams Guide](er_diagrams_guide.md) #6-9

**Activity Diagrams:**
- Flow control (if/while) → [Activity Diagrams Guide](activity_diagrams_guide.md) #5-9
- Fork/join errors → [Activity Diagrams Guide](activity_diagrams_guide.md) #10-12
- Partition syntax → [Activity Diagrams Guide](activity_diagrams_guide.md) #13-14

### 4. Styling Problems
- Colors not working → [Styling & Themes Guide](styling_themes_guide.md) #4-6
- skinparam vs style conflicts → [Styling & Themes Guide](styling_themes_guide.md) #1-3
- Font issues → [Styling & Themes Guide](styling_themes_guide.md) #7-10

### 5. Include & Preprocessor Errors
- "Cannot include file" → [Preprocessor & Includes Guide](preprocessor_includes_guide.md) #1-4
- Circular dependencies → [Preprocessor & Includes Guide](preprocessor_includes_guide.md) #8
- URL include failures → [Preprocessor & Includes Guide](preprocessor_includes_guide.md) #11-13

### 6. Rendering & Output Issues
- "No Dot executable found" → [Installation & Setup Guide](installation_setup_guide.md) #2
- Image generation failed → [Image Generation Guide](image_generation_guide.md) #1-5
- Wrong output format → [Image Generation Guide](image_generation_guide.md) #8-10

### 7. Performance Problems
- Diagram too slow → [Performance Guide](performance_guide.md) #1-5
- Memory errors → [Performance Guide](performance_guide.md) #6-8
- Timeout errors → [Performance Guide](performance_guide.md) #9-11

## Common Error Messages Quick Reference

| Error Message | Most Likely Cause | Guide Reference |
|---------------|-------------------|-----------------|
| "Cannot find java!" | Java not in PATH | [Installation](installation_setup_guide.md) #1 |
| "No Dot executable found" | Graphviz missing | [Installation](installation_setup_guide.md) #2 |
| "Some diagram description contains errors" | Syntax error (various) | Check diagram-specific guide |
| "Cannot include file" | Wrong file path | [Preprocessor](preprocessor_includes_guide.md) #1 |
| "Syntax Error" (with line number) | Check line syntax | [General Syntax](general_syntax_guide.md) |
| "Duplicate participant" | Participant defined twice | [Sequence](sequence_diagrams_guide.md) #3 |
| "Empty alt group" | Fragment missing content | [Sequence](sequence_diagrams_guide.md) #11 |
| "Unable to access jarfile" | Wrong plantuml.jar path | [Installation](installation_setup_guide.md) #4 |
| "HeadlessException" | Missing headless flag (Unix) | [Installation](installation_setup_guide.md) #6 |
| "For some reason, dot/GraphViz has crashed" | Graphviz corrupted/version | [Installation](installation_setup_guide.md) #3 |
| "File already included" | !include_once violation | [Preprocessor](preprocessor_includes_guide.md) #5 |
| "Failed to generate image" | Rendering failure | [Image Generation](image_generation_guide.md) #1 |
| "NullPointerException" | Internal PlantUML error | [General Syntax](general_syntax_guide.md) #15 |
| Stack overflow | Circular includes | [Preprocessor](preprocessor_includes_guide.md) #8 |

## Troubleshooting Workflow

### Step 1: Verify Environment
1. Check Java installation: `java -version`
2. Check Graphviz: `dot -V`
3. Check PlantUML: `java -jar plantuml.jar -version`

### Step 2: Isolate the Problem
1. Create minimal test case
2. Run with `-verbose` flag: `java -jar plantuml.jar -verbose test.puml`
3. Check PlantUML version compatibility

### Step 3: Use Specific Guides
1. Identify error category from decision tree above
2. Consult relevant guide
3. Apply solutions from most common to least common

### Step 4: Test Solutions
1. Test each solution individually
2. Verify with minimal example first
3. Apply to full diagram

## Additional Resources

- [PlantUML Official FAQ](https://plantuml.com/faq)
- [PlantUML Forum](https://forum.plantuml.net/)
- [Stack Overflow PlantUML Tag](https://stackoverflow.com/questions/tagged/plantuml)
- [PlantUML GitHub Issues](https://github.com/plantuml/plantuml/issues)

## Guide Structure

Each guide follows this format:
- **Error Message/Symptom**: What you see
- **Cause**: Why it happens
- **Solution**: How to fix it
- **Before/After Examples**: Code showing incorrect and correct versions

## Contributing

Found a common error not covered here? Please contribute by:
1. Documenting the error message
2. Providing a minimal reproduction case
3. Including the solution that worked
4. Adding before/after code examples
