# PlantUML Common Syntax Errors by Diagram Type

This guide documents the most common syntax errors specific to each of the 19 PlantUML diagram types, based on GitHub issues, Stack Overflow discussions, and PlantUML community forums.

## Table of Contents

1. [Sequence Diagrams](#sequence-diagrams)
2. [Class Diagrams](#class-diagrams)
3. [Activity Diagrams](#activity-diagrams)
4. [Use Case Diagrams](#use-case-diagrams)
5. [State Diagrams](#state-diagrams)
6. [Component Diagrams](#component-diagrams)
7. [Deployment Diagrams](#deployment-diagrams)
8. [Object Diagrams](#object-diagrams)
9. [Timing Diagrams](#timing-diagrams)
10. [ER Diagrams](#er-diagrams)
11. [Gantt Charts](#gantt-charts)
12. [MindMap Diagrams](#mindmap-diagrams)
13. [WBS Diagrams](#wbs-diagrams)
14. [JSON/YAML Diagrams](#jsonyaml-diagrams)
15. [Network Diagrams](#network-diagrams)
16. [Archimate Diagrams](#archimate-diagrams)
17. [Salt Wireframes](#salt-wireframes)
18. [Ditaa Diagrams](#ditaa-diagrams)
19. [Timeline Diagrams](#timeline-diagrams)

---

## Sequence Diagrams

### 1. Participant Declaration Errors

**Error:** Forgetting to declare participants before using them in messages.

**Incorrect:**
```plantuml
Alice -> Bob: Init
```

**Correct:**
```plantuml
participant Alice
participant Bob
Alice -> Bob: Init
```

**Cause:** PlantUML requires explicit participant declarations for proper rendering.

### 2. Arrow Direction Confusion

**Error:** Using reversed arrow direction, sending messages in the wrong direction.

**Incorrect:**
```plantuml
participant Alice
participant Bob
Bob <- Alice: Sends message  ' This says message goes TO Alice
```

**Correct:**
```plantuml
participant Alice
participant Bob
Alice -> Bob: Sends message
```

**Cause:** `<-` flips the message direction. Use `->` for left-to-right messages.

### 3. Forbidden Whitespace Characters

**Error:** Non-breaking spaces (NBSP) or tabs in diagram code, especially after v1.2025.

**Cause:** Copy-pasting from other sources can introduce invisible invalid characters. Recent PlantUML versions reject these characters.

**Fix:** Clean whitespace by retyping or using a text editor that shows invisible characters.

### 4. Missing Activation/Deactivation

**Error:** Not using `activate`/`deactivate` for process ownership clarity.

**Incorrect:**
```plantuml
participant Service
Service -> Other: Call
```

**Correct:**
```plantuml
participant Service
activate Service
Service -> Other: Call
deactivate Service
```

### 5. Fragment Block Errors

**Error:** Missing `end` keyword or misplaced fragment delimiters.

**Incorrect:**
```plantuml
alt Approved
    Alice -> Bob: Confirmed
else Rejected
    Alice -> Bob: Declined
' Missing end
```

**Correct:**
```plantuml
alt Approved
    Alice -> Bob: Confirmed
else Rejected
    Alice -> Bob: Declined
end
```

**Cause:** Every `alt`, `loop`, `opt`, `par`, etc. must have a matching `end`.

---

## Class Diagrams

### 1. Relationship Arrow Errors

**Error:** Using wrong arrow symbols for inheritance, composition, or aggregation.

**Incorrect:**
```plantuml
class Parent
class Child
Child --> Parent  ' Wrong arrow for inheritance
```

**Correct:**
```plantuml
class Parent
class Child
Parent <|-- Child  ' Inheritance arrow
```

**Common Arrow Types:**
- `<|--` : Inheritance
- `*--` : Composition
- `o--` : Aggregation
- `--` : Association

### 2. Cardinality Notation Errors

**Error:** Using non-standard cardinality symbols or incorrect placement.

**Incorrect:**
```plantuml
ClassA "1" -- "many" ClassB
```

**Correct:**
```plantuml
ClassA }o-- "many" ClassB  ' Zero or many
ClassA ||-- "1" ClassB     ' Exactly one
```

### 3. Abstract Class Declaration

**Error:** Omitting `abstract` keyword for abstract classes.

**Incorrect:**
```plantuml
class Shape
```

**Correct:**
```plantuml
abstract class Shape
```

### 4. Attribute Syntax Errors

**Error:** Missing colon between attribute name and type.

**Incorrect:**
```plantuml
class Example {
  myField int
}
```

**Correct:**
```plantuml
class Example {
  myField: int
}
```

### 5. Visibility Modifier Errors

**Error:** Omitting visibility symbols or placing them incorrectly.

**Incorrect:**
```plantuml
class Example {
  publicField: int
}
```

**Correct:**
```plantuml
class Example {
  +publicField: int
  -privateField: int
  #protectedField: int
  ~packageField: int
}
```

**Visibility Symbols:**
- `+` : Public
- `-` : Private
- `#` : Protected
- `~` : Package/Internal

---

## Activity Diagrams

### 1. Start/Stop Node Errors

**Error:** Omitting start (*) or stop markers.

**Incorrect:**
```plantuml
:Initialize;
:Process;
```

**Correct:**
```plantuml
start
:Initialize;
:Process;
stop
```

Or using legacy syntax:
```plantuml
(*) --> :Initialize;
:Process;
--> (*)
```

### 2. Decision Diamond Syntax

**Error:** Using incorrect syntax for conditions and branches.

**Incorrect:**
```plantuml
:Test value;
-->[yes] :If true;
-->[no] :If false;
```

**Correct:**
```plantuml
if (Test value?) then (yes)
  :If true;
else (no)
  :If false;
endif
```

### 3. Fork/Join Errors

**Error:** Missing `end fork` or `end split` after parallel branches.

**Incorrect:**
```plantuml
:Do A;
fork
  :Branch 1;
fork again
  :Branch 2;
' Missing end fork
```

**Correct:**
```plantuml
:Do A;
fork
  :Branch 1;
fork again
  :Branch 2;
end fork
```

### 4. Partition Syntax Errors

**Error:** Not using correct `|Partition|` syntax for swimlanes.

**Incorrect:**
```plantuml
partition P1 {
  :Do something;
}
```

**Correct:**
```plantuml
|P1|
:Do something;
|P2|
:Do something else;
```

### 5. Arrow Label Errors

**Error:** Placing labels incorrectly on arrows without brackets.

**Incorrect:**
```plantuml
:Start;
--> yes :Process;
```

**Correct:**
```plantuml
:Start;
-->[yes] :Process;
```

---

## Use Case Diagrams

### 1. Actor Declaration Errors

**Error:** Omitting the `actor` keyword.

**Incorrect:**
```plantuml
User
```

**Correct:**
```plantuml
actor User
actor Customer as C
```

### 2. Use Case Syntax Errors

**Error:** Missing parentheses or quotes for multi-word names.

**Incorrect:**
```plantuml
Login UseCase
```

**Correct:**
```plantuml
(Login)
usecase "Register Account" as Register
```

### 3. Extends/Includes Relationship Errors

**Error:** Wrong syntax for extension or inclusion relationships.

**Incorrect:**
```plantuml
(Login) -extends-> (Authenticate)
```

**Correct:**
```plantuml
(Login) .> (Authenticate) : <<extends>>
(Register) .> (Validate Email) : <<include>>
```

### 4. System Boundary Errors

**Error:** Placing actors inside the system boundary.

**Incorrect:**
```plantuml
rectangle System {
  actor User
  (Login)
}
```

**Correct:**
```plantuml
rectangle "Authentication System" {
  (Login)
  (Register)
}
actor User
User -- (Login)
```

---

## State Diagrams

### 1. State Declaration Errors

**Error:** Omitting `state` keyword when using aliases.

**Incorrect:**
```plantuml
"Processing" as Process
```

**Correct:**
```plantuml
state "Processing" as Process
```

### 2. Transition Arrow Errors

**Error:** Using single dash instead of double dash.

**Incorrect:**
```plantuml
Waiting -> Processing
```

**Correct:**
```plantuml
Waiting --> Processing
```

### 3. Initial State Errors

**Error:** Missing arrow from initial state.

**Incorrect:**
```plantuml
[*] Waiting
```

**Correct:**
```plantuml
[*] --> Waiting
```

### 4. Composite State Errors

**Error:** Incorrect nesting syntax for composite states.

**Incorrect:**
```plantuml
state CompositeState
  state Substate1
  state Substate2
end state
```

**Correct:**
```plantuml
state CompositeState {
  state Substate1
  state Substate2
}
```

### 5. Fork/Join Syntax

**Error:** Not using `<<fork>>` or `<<join>>` stereotypes.

**Incorrect:**
```plantuml
state fork1
State1 --> fork1
fork1 --> State2
```

**Correct:**
```plantuml
state fork1 <<fork>>
State1 --> fork1
fork1 --> State2
fork1 --> State3
```

---

## Component Diagrams

### 1. Component Declaration Errors

**Error:** Using component names with spaces without brackets or quotes.

**Incorrect:**
```plantuml
component User Service
```

**Correct:**
```plantuml
[User Service]
' OR
component "User Service"
' OR
component "User Service" as US
```

### 2. Interface Syntax Errors

**Error:** Using class diagram interface notation instead of lollipop notation.

**Incorrect:**
```plantuml
interface Auth
Auth - [User Service]
```

**Correct:**
```plantuml
[User Service] - [() Auth]
' OR
[User Service] --|> [Auth]
```

### 3. Relationship Arrow Errors

**Error:** Using wrong arrow syntax for dependencies.

**Incorrect:**
```plantuml
[Frontend] => [Backend]
```

**Correct:**
```plantuml
[Frontend] --> [Backend]
' OR for dependency
[Frontend] ..> [Backend]
```

### 4. Missing Relationship Labels

**Error:** Forgetting colon when adding labels.

**Incorrect:**
```plantuml
[Service] --> [Database] DataFetch
```

**Correct:**
```plantuml
[Service] --> [Database] : DataFetch
```

---

## Deployment Diagrams

### 1. Node Declaration Errors

**Error:** Omitting `node` keyword or missing quotes.

**Incorrect:**
```plantuml
"Web Server" as web
```

**Correct:**
```plantuml
node "Web Server" as web
```

### 2. Artifact Syntax Errors

**Error:** Not using `artifact` keyword or missing aliases.

**Incorrect:**
```plantuml
artifact app
node "Server"
```

**Correct:**
```plantuml
node "Server" as srv
artifact "App" as app
srv -- app
```

### 3. Deployment Relationship Errors

**Error:** Using directional arrows (`-l->`, `-r->`) which are not supported in deployment diagrams.

**Incorrect:**
```plantuml
node Server
node Database
Server -l-> Database
```

**Correct:**
```plantuml
node Server
node Database
Server --> Database
```

**Cause:** Directional qualifiers are only valid in state diagrams.

### 4. Nesting Issues

**Error:** Non-unique IDs causing element merging.

**Cause:** PlantUML may merge elements with the same name if not clearly separated with unique aliases.

**Fix:** Use unique names or the `set separator` command with `::` for namespaces.

---

## Object Diagrams

### 1. Object Name Errors

**Error:** Using spaces in object names without quotes.

**Incorrect:**
```plantuml
object John Smith
```

**Correct:**
```plantuml
object "John Smith"
' OR
object "John Smith" as js
```

### 2. Attribute Syntax Errors

**Error:** Using colons instead of equals signs for attribute values.

**Incorrect:**
```plantuml
object p1 {
  name: "Alice"
  age:
}
```

**Correct:**
```plantuml
object p1 {
  name = "Alice"
  age = 30
}
```

### 3. Missing Attribute Braces

**Error:** Forgetting braces around attributes.

**Incorrect:**
```plantuml
object p1
  name = "Alice"
  age = 30
```

**Correct:**
```plantuml
object p1 {
  name = "Alice"
  age = 30
}
```

### 4. Invalid Relationship Operators

**Error:** Using class diagram-only operators.

**Incorrect:**
```plantuml
p1 ==> p2
```

**Correct:**
```plantuml
p1 -- p2
' OR
p1 *-- p2  ' Composition
p1 o-- p2  ' Aggregation
```

---

## Timing Diagrams

### 1. Wrong Participant Keywords

**Error:** Using `participant` keyword from sequence diagrams.

**Incorrect:**
```plantuml
@startuml
participant Foo
```

**Correct:**
```plantuml
@startuml
robust "Sensor" as S
concise "Signal" as Sig
binary "Switch" as Sw
clock "Clock" as Clk
@enduml
```

**Keywords:** Use `robust`, `concise`, `binary`, or `clock` for timing diagrams.

### 2. State Transition Syntax Errors

**Error:** Using assignment operator instead of `is` verb.

**Incorrect:**
```plantuml
robust "Device" as D
@0 D = Start
```

**Correct:**
```plantuml
robust "Device" as D
@0 D is Start
@1 D is Running
```

### 3. Timing Constraint Errors

**Error:** Wrong notation or missing braces for constraints.

**Incorrect:**
```plantuml
@2 L is On (1..3s)
' OR
@2 L is On {wrong-notification}
```

**Correct:**
```plantuml
@2 L is On {1..3s}
@3 L is Off {<=2s}
```

### 4. Missing Time Markers

**Error:** Omitting `@` time marker for transitions.

**Incorrect:**
```plantuml
robust "Light" as L
L is Off
2 L is On
```

**Correct:**
```plantuml
robust "Light" as L
@0 L is Off
@2 L is On
```

---

## ER Diagrams

### 1. Entity Declaration Errors

**Error:** Missing `entity` keyword or forgetting braces.

**Incorrect:**
```plantuml
Employee {
  +emp_id: INT
}
```

**Correct:**
```plantuml
entity Employee {
  +emp_id: INT
  +name: VARCHAR
}
```

### 2. Invalid Entity Names

**Error:** Spaces or special characters without quotes.

**Incorrect:**
```plantuml
entity employee record {
  +id: INT
}
```

**Correct:**
```plantuml
entity "employee record" {
  +id: INT
}
```

### 3. Cardinality Errors

**Error:** Using non-standard cardinality symbols or wrong order.

**Incorrect:**
```plantuml
Employee o|--o| Department
```

**Correct:**
```plantuml
Employee ||--o{ Department
' ||   = exactly one
' o{   = zero or many
' }|   = one or many
```

### 4. Primary/Foreign Key Notation

**Error:** Missing `<pk>` or `<fk>` markers.

**Incorrect:**
```plantuml
entity Department {
  +dept_id: INT
  +dept_name: VARCHAR
}
```

**Correct:**
```plantuml
entity Department {
  +dept_id: INT <<pk>>
  +dept_name: VARCHAR
}
' OR
entity Department {
  +dept_id: INT <pk>
  +dept_name: VARCHAR
}
```

### 5. Attribute Type Errors

**Error:** Missing colon between attribute name and type.

**Incorrect:**
```plantuml
entity Example {
  +emp_id INT
}
```

**Correct:**
```plantuml
entity Example {
  +emp_id: INT
}
```

---

## Gantt Charts

### 1. Task Declaration Errors

**Error:** Omitting square brackets around task names.

**Incorrect:**
```plantuml
Task A starts 2025-11-01
```

**Correct:**
```plantuml
@startgantt
[Task A] starts 2025-11-01
@endgantt
```

### 2. Date Format Errors

**Error:** Using incorrect date formats (slashes, missing zeros).

**Incorrect:**
```plantuml
[A] starts 11/01/2025
' OR
[B] starts 2025-1-5
```

**Correct:**
```plantuml
[A] starts 2025-11-01
[B] starts 2025-01-05
```

**Required Format:** `YYYY-MM-DD`

### 3. Milestone Syntax Errors

**Error:** Using wrong keywords or adding durations to milestones.

**Incorrect:**
```plantuml
[M1] milestone 2025-11-15
' OR
[M1] happens at 2025-11-15 lasts 1 day
```

**Correct:**
```plantuml
[M1] happens at 2025-11-15
' OR
[M1] happens at [Task A]'s end
```

### 4. Dependency Errors

**Error:** Wrong keywords or missing arrows.

**Incorrect:**
```plantuml
[Task B] after [Task A]
```

**Correct:**
```plantuml
[Task A] -> [Task B]
' OR
[Task B] starts at [Task A]'s end
```

### 5. Duration Syntax Errors

**Error:** Missing units or mixing duration with end dates.

**Incorrect:**
```plantuml
[Task B] lasts 5
' OR
[Task C] ends 2025-11-10 and lasts 3 days
```

**Correct:**
```plantuml
[Task B] lasts 5 days
' OR
[Task B] lasts 1w
' Don't mix end date and duration - pick one
```

---

## MindMap Diagrams

### 1. Node Hierarchy Errors

**Error:** Incorrect number of asterisks or broken hierarchy levels.

**Incorrect:**
```plantuml
@startmindmap
* Root
*** Grandchild  ' Skipped level 2
@endmindmap
```

**Correct:**
```plantuml
@startmindmap
* Root
** Child
*** Grandchild
@endmindmap
```

### 2. Multi-line Node Errors

**Error:** Attempting to use multi-line text in nodes (not supported).

**Incorrect:**
```plantuml
** This is a
multi-line node
```

**Correct:**
```plantuml
** This is a single-line node
```

**Fix:** Use `skinparam maxWidth` for text wrapping instead of actual line breaks.

### 3. Direction Notation Errors

**Error:** Mixing `+` and `-` on same node or using inconsistent depth.

**Incorrect:**
```plantuml
+ Root
+- Node  ' Mixed symbols
```

**Correct:**
```plantuml
+ Root
++ Right Child
-- Left Child
+++ Right Grandchild
--- Left Grandchild
```

### 4. Multi-line with Sided Nodes

**Error:** Combining multi-line attempts with left/right side notation.

**Incorrect:**
```plantuml
++ A right node
++ With a second line
```

**Cause:** Multi-line nodes are not supported with direction notation.

---

## WBS Diagrams

### 1. Diagram Declaration Errors

**Error:** Missing or misspelled start/end tags.

**Incorrect:**
```plantuml
@startwb
* Project
@endwbs
```

**Correct:**
```plantuml
@startwbs
* Project
** Task 1
** Task 2
@endwbs
```

### 2. Multiple Root Nodes

**Error:** Defining multiple top-level root nodes.

**Incorrect:**
```plantuml
@startwbs
* Project A
* Project B
@endwbs
```

**Correct:**
```plantuml
@startwbs
* Main Project
** Sub-project A
** Sub-project B
@endwbs
```

**Cause:** WBS diagrams require exactly one root node.

### 3. Line Continuation Errors

**Error:** Using backslash for line continuation in included files.

**Incorrect:**
```plantuml
@startwbs
* Business Process Modelling WBS
** Launch \
the project
@endwbs
```

**Cause:** Backslash continuation can cause parsing failures when including files.

**Fix:** Avoid backslash line continuations or don't include such files.

### 4. Inconsistent Hierarchy Markers

**Error:** Mixing indentation styles or using wrong number of asterisks.

**Incorrect:**
```plantuml
@startwbs
* Root
** Level 2
* Level 1 again  ' Wrong - should be **
@endwbs
```

**Correct:**
```plantuml
@startwbs
* Root
** Level 2
*** Level 3
** Another Level 2
@endwbs
```

---

## JSON/YAML Diagrams

### 1. JSON Formatting Errors

**Error:** Improper quotes, missing commas, incorrect bracket nesting.

**Incorrect:**
```plantuml
@startjson
{
  name: "John"
  age: 30
  city: "NYC"
}
@endjson
```

**Correct:**
```plantuml
@startjson
{
  "name": "John",
  "age": 30,
  "city": "NYC"
}
@endjson
```

### 2. YAML Indentation Errors

**Error:** Incorrect spacing or mixing tabs with spaces.

**Incorrect:**
```plantuml
@startyaml
person:
name: John
  age: 30
@endyaml
```

**Correct:**
```plantuml
@startyaml
person:
  name: John
  age: 30
@endyaml
```

**Important:** Use consistent spaces (not tabs) for YAML indentation.

### 3. Missing Diagram Delimiters

**Error:** Omitting `@startjson`/`@endjson` or `@startyaml`/`@endyaml`.

**Incorrect:**
```plantuml
{
  "name": "John"
}
```

**Correct:**
```plantuml
@startjson
{
  "name": "John"
}
@endjson
```

### 4. Type Declaration Errors (YAML)

**Error:** Missing or incorrect `type` field for diagram specification.

**Cause:** When using YAML to define diagram types, the `type` field must correctly specify the diagram format.

---

## Network Diagrams

### 1. Network ID Errors (Hyphens)

**Error:** Using hyphens in network or node identifiers.

**Incorrect:**
```plantuml
nwdiag {
  network net-1 {
    node1;
  }
}
```

**Correct:**
```plantuml
nwdiag {
  network net1 {
    node1;
  }
}
```

**Cause:** Network and node IDs cannot contain hyphens. Use underscores instead.

### 2. Node Placement Errors

**Error:** Defining nodes outside any network block.

**Incorrect:**
```plantuml
nwdiag {
  group {
    web01;
    db01;
  }
  ' No network block for these nodes
}
```

**Correct:**
```plantuml
nwdiag {
  network internal {
    web01;
    db01;
  }
}
```

**Cause:** Nodes must be inside a `network` block to avoid NullPointerException.

### 3. Group Order Errors

**Error:** Declaring groups before networks or nesting incorrectly.

**Incorrect:**
```plantuml
nwdiag {
  group { web01; db01; }
  network internal {
    web01; db01;
  }
}
```

**Correct:**
```plantuml
nwdiag {
  network internal {
    web01;
    db01;
  }
  group {
    web01;
    db01;
  }
}
```

**Cause:** Network blocks should be defined before groups.

### 4. Address Notation Errors

**Error:** Unquoted or malformed IP addresses.

**Incorrect:**
```plantuml
nwdiag {
  network DMZ {
    web01 [address = y.x.x.1];
  }
}
```

**Correct:**
```plantuml
nwdiag {
  network DMZ {
    web01 [address = "y.x.x.1"];
  }
}
```

---

## Archimate Diagrams

### 1. Element Declaration Errors

**Error:** Missing or misordered parameters in macro calls.

**Incorrect:**
```plantuml
Business_Actor("Customer")
```

**Correct:**
```plantuml
Business_Actor(Customer, "Customer")
' alias first, then description
```

### 2. Relationship Syntax Errors

**Error:** Using text labels instead of aliases or wrong macro names.

**Incorrect:**
```plantuml
Rel_Composition("Customer", "Order")
```

**Correct:**
```plantuml
Business_Actor(Customer, "Customer")
Business_Object(Order, "Order")
Rel_Composition(Customer, Order, "composes")
```

### 3. Directional Relationship Errors

**Error:** Not using directional relationship macros when needed.

**Correct:**
```plantuml
Rel_Composition_Right(Customer, Order, "Customer places Order")
Rel_Association_Left(Order, Product, "contains")
```

### 4. Nesting Errors

**Error:** Attempting to nest unsupported elements, especially in special-shaped nodes.

**Incorrect:**
```plantuml
Technology_Node(MainServer) {
    Business_Service(AppService)
}
```

**Cause:** Special-shaped elements may not support nesting. Check documentation for allowed nesting.

### 5. Missing Library Include

**Error:** Not including the Archimate macro library.

**Incorrect:**
```plantuml
@startuml
Business_Actor(Customer, "Customer")
@enduml
```

**Correct:**
```plantuml
@startuml
!include <archimate/Archimate>
Business_Actor(Customer, "Customer")
@enduml
```

---

## Salt Wireframes

### 1. Widget Syntax Errors

**Error:** Incorrect casing or misspelling widget names.

**Incorrect:**
```plantuml
@startsalt
[textfield]
[btn]
@endsalt
```

**Correct:**
```plantuml
@startsalt
{
  [TextField]
  [Button]
}
@endsalt
```

**Note:** Widget names are case-sensitive.

### 2. Layout Bracket Errors

**Error:** Using wrong bracket types or unmatched brackets.

**Incorrect:**
```plantuml
@startsalt
(
  [Label]
  [TextField]
]
@endsalt
```

**Correct:**
```plantuml
@startsalt
{
  [Label]
  [TextField]
}
@endsalt
```

### 3. Button Notation Errors

**Error:** Missing brackets or not enclosing button text.

**Incorrect:**
```plantuml
@startsalt
{
  Submit
}
@endsalt
```

**Correct:**
```plantuml
@startsalt
{
  [Submit]
  [<b>Bold Button</b>]
}
@endsalt
```

### 4. Text Field Syntax Errors

**Error:** Omitting brackets or missing pipe for default values.

**Incorrect:**
```plantuml
@startsalt
{
  TextField
}
@endsalt
```

**Correct:**
```plantuml
@startsalt
{
  "Username:" | [TextField]
  "Email:" | [TextField|Enter email]
}
@endsalt
```

### 5. Grid Alignment Errors

**Error:** Missing pipe separators for table cells.

**Incorrect:**
```plantuml
@startsalt
{
  [Name [TextField]]
  [Email [TextField]]
}
@endsalt
```

**Correct:**
```plantuml
@startsalt
{
  Name | [TextField]
  Email | [TextField]
}
@endsalt
```

### 6. Whitespace Errors

**Error:** Leading whitespace before `salt` keyword when using `@startuml`.

**Incorrect:**
```plantuml
@startuml
  salt
{
  [Button]
}
@enduml
```

**Correct:**
```plantuml
@startuml
salt
{
  [Button]
}
@enduml
' OR
@startsalt
{
  [Button]
}
@endsalt
```

---

## Ditaa Diagrams

### 1. ASCII Art Formatting Errors

**Error:** Trailing backslashes causing line fusion.

**Incorrect:**
```plantuml
@startditaa
/-----------------\
| Some box text   |
\-----------------/\
@endditaa
```

**Cause:** Trailing `\` is interpreted as line continuation when using `@startuml`.

**Fix:** Use `@startditaa` and `@endditaa` instead of `@startuml`/`@enduml`.

### 2. Box Syntax Errors

**Error:** Using incorrect corner characters for boxes.

**Incorrect:**
```plantuml
(--------)
|        |
(--------)
```

**Correct:**
```plantuml
/--------\
|        |
\--------/
' OR
+--------+
|        |
+--------+
```

### 3. Arrow Notation Errors

**Error:** Mixing symbols or using wrong arrow characters.

**Incorrect:**
```plantuml
|--
```

**Correct:**
```plantuml
|-->
<--|
|
v
```

### 4. Color Coding Errors

**Error:** Misspelled color codes or wrong placement.

**Incorrect:**
```plantuml
|cRED RED |cBLU BLU |
```

**Correct:**
```plantuml
|cRED|
| Red Box |
+--------+

|cBLU|
| Blue Box |
+---------+
```

**Valid Color Codes:** `cRED`, `cBLU`, `cGRE`, `cYEL`, etc.

---

## Timeline Diagrams

### 1. Period Declaration Errors

**Error:** Using incorrect syntax for period definitions.

**Incorrect:**
```plantuml
period 2025/01/01-2025/01/31 "Q1"
```

**Correct:**
```plantuml
period 2025-01-01 to 2025-01-31 as "Q1"
```

### 2. Event Syntax Errors

**Error:** Missing colon between date and event description.

**Incorrect:**
```plantuml
2025-01-01 Start Event
```

**Correct:**
```plantuml
2025-01-01 : Start Event
```

### 3. Date Formatting Errors

**Error:** Using wrong date format or delimiters.

**Incorrect:**
```plantuml
25-01-2025 : Milestone
2025/01/01 : Milestone
```

**Correct:**
```plantuml
2025-01-25 : Milestone
2025-01-01 : Start
```

**Required Format:** ISO 8601 `YYYY-MM-DD`

### 4. Chronological Ordering Errors

**Error:** Events not listed in chronological order.

**Incorrect:**
```plantuml
timeline
  2025-01-05 : First Event
  2025-01-01 : Earlier Event
```

**Correct:**
```plantuml
timeline
  2025-01-01 : Earlier Event
  2025-01-05 : First Event
```

**Cause:** Some timeline implementations expect strict chronological order.

---

## General Tips to Avoid Syntax Errors

### 1. Always Use Proper Delimiters
- Start every diagram with `@start[type]` and end with `@end[type]`
- Examples: `@startuml`/`@enduml`, `@startgantt`/`@endgantt`, `@startsalt`/`@endsalt`

### 2. Watch for Whitespace
- Avoid non-breaking spaces (NBSP) and tabs, especially after PlantUML v1.2025
- Use regular spaces for indentation
- Don't add leading whitespace before diagram type keywords

### 3. Check Bracket/Brace Matching
- Every opening bracket/brace must have a closing one
- Use the correct bracket type for each context

### 4. Use Consistent Naming
- PlantUML is case-sensitive
- Use quotes for names with spaces or special characters
- Define elements before referencing them in relationships

### 5. Validate Syntax with Error Messages
- Read error messages carefully - they usually indicate the line number
- Check for typos in keywords and element names
- Verify parameter order for macros and functions

### 6. Test Incrementally
- Build diagrams step by step
- Test after adding each major element
- Comment out sections to isolate syntax errors

### 7. Consult Documentation
- Refer to the official PlantUML Language Reference Guide
- Check version-specific changes in the changelog
- Review diagram-specific syntax in reference documentation

---

## Version-Specific Issues

### PlantUML v1.2025+ Changes
- Stricter whitespace handling (NBSP and tabs now rejected)
- Some legacy syntax deprecated
- Enhanced error reporting with better line number accuracy

### Common Migration Issues
- Old diagrams with tabs or NBSP may suddenly fail
- Copy-pasted code from other sources needs whitespace cleaning
- Some skinparam commands may have changed

---

## Resources

- [PlantUML Official Documentation](https://plantuml.com)
- [PlantUML Language Reference Guide (PDF)](https://pdf.plantuml.net/PlantUML_Language_Reference_Guide_en.pdf)
- [PlantUML GitHub Issues](https://github.com/plantuml/plantuml/issues)
- [PlantUML Forum](https://forum.plantuml.net)
- [Stack Overflow PlantUML Tag](https://stackoverflow.com/questions/tagged/plantuml)

---

**Last Updated:** 2025-11-13
