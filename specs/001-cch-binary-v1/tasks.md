# Tasks: CCH Binary v1

**Input**: Design documents from `/specs/001-cch-binary-v1/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are OPTIONAL - not requested in the feature specification, so skipped.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- Paths shown below assume single project - adjust based on plan.md structure

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Create Rust project structure per implementation plan
- [X] T002 Initialize Cargo.toml with serde, clap, regex, tokio dependencies
- [X] T003 [P] Configure rustfmt and clippy linting tools

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T004 Setup basic binary structure with main.rs and lib.rs
- [X] T005 [P] Implement core data models (Rule, Event, Response, LogEntry) in src/models/
- [X] T006 [P] Implement configuration loading from YAML files in src/config/
- [X] T007 [P] Setup JSON Lines logging infrastructure in src/logging/
- [X] T008 Implement stdin/stdout JSON processing pipeline

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Block Dangerous Operations (Priority: P1) 🎯 MVP

**Goal**: Automatically block dangerous git operations like force push

**Independent Test**: Configure a block rule for `git push --force` and verify the operation is blocked with a clear message

### Implementation for User Story 1

- [X] T009 [US1] Implement rule matching logic for command patterns in src/hooks.rs (matches_rule function)
- [X] T010 [US1] Add blocking action handler in src/hooks.rs (execute_rule_actions function)
- [X] T011 [US1] Integrate blocking into PreToolUse event processing
- [X] T012 [US1] Add reason message output for blocked operations (Response::block with descriptive message)

**Checkpoint**: ✅ User Story 1 is fully functional and tested - force push and hard reset blocking verified

---

## Phase 4: User Story 2 - Inject Context for Skill Triggers (Priority: P1)

**Goal**: Automatically inject relevant skill documentation when editing files in specific directories

**Independent Test**: Configure an inject rule for CDK files and verify the SKILL.md content appears in Claude's context

### Implementation for User Story 2

- [X] T013 [US2] Implement file path matching for directories in src/hooks.rs (matches_rule function with glob patterns)
- [X] T014 [US2] Add context injection action handler in src/hooks.rs (execute_rule_actions with inject action)
- [X] T015 [US2] Integrate injection into PreToolUse event processing
- [X] T016 [US2] Add file reading and context formatting logic (read_context_file async function)

**Checkpoint**: ✅ User Stories 1 AND 2 both work independently - blocking and injection verified

---

## Phase 5: User Story 3 - Run Custom Validators (Priority: P2)

**Goal**: Execute custom Python scripts to enforce complex rules beyond pattern matching

**Independent Test**: Create a validator script that checks for console.log and verify it blocks when the pattern is found

### Implementation for User Story 3

- [X] T017 [US3] Implement async script execution with timeout in src/hooks.rs (execute_validator_script function)
- [X] T018 [US3] Add validator script runner in src/hooks.rs (spawn, stdin piping, timeout handling)
- [X] T019 [US3] Integrate validator execution into PreToolUse processing (via run action)
- [X] T020 [US3] Handle script exit codes and stdout/stderr processing (exit 0=allow with stdout context, exit 1=block with stderr message)

**Checkpoint**: ✅ User Stories 1, 2, and 3 all work independently - blocking, injection, and validation verified

---

## Phase 6: User Story 4 - Explain Commands Before Permission (Priority: P2)

**Goal**: Provide structured explanations before asking permission for commands

**Independent Test**: Configure a PermissionRequest rule with inject template and verify explanation is included

### Implementation for User Story 4

- [X] T021 [US4] Implement PermissionRequest event handling in src/hooks.rs (EventType::PermissionRequest supported)
- [X] T022 [US4] Add explanation template injection logic in src/hooks.rs (inject action with operations matcher)
- [X] T023 [US4] Add operations matcher for event type filtering in matches_rule function
- [X] T024 [US4] Add context formatting for permission requests (explain-command.md template)

**Checkpoint**: ✅ All P1 and P2 user stories now independently functional

---

## Phase 7: User Story 5 - Query Logs for Troubleshooting (Priority: P3)

**Goal**: Query CCH logs to understand why rules did or didn't fire

**Independent Test**: Run `cch logs` after hook events and verify events are recorded

### Implementation for User Story 5

- [X] T025 [US5] Implement log querying CLI command in src/cli/logs.rs (with filtering, limit, since options)
- [X] T026 [US5] Add log file reading and JSON Lines parsing in src/logging.rs (LogQuery with QueryFilters)
- [X] T027 [US5] Implement rule explanation command in src/cli/explain.rs (session-based query with summary)
- [X] T028 [US5] Initialize global logger in main.rs for audit trail

**Checkpoint**: ✅ ALL user stories are now independently functional and tested

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T029 [P] Add comprehensive error handling and validation
- [ ] T030 Performance optimization for <5ms cold start requirement
- [ ] T031 [P] Security hardening and input validation
- [ ] T032 Cross-platform testing and compatibility fixes
- [ ] T033 [P] Documentation updates in docs/
- [ ] T034 Final integration testing across all user stories

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 4 (P2)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 5 (P3)**: Can start after Foundational (Phase 2) - No dependencies on other stories

### Within Each User Story

- Core implementation tasks are sequential within each story
- Stories should be independently testable
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch foundational tasks together:
Task: "Implement core data models (Rule, Event, Response, LogEntry) in src/models/"
Task: "Implement configuration loading from YAML files in src/config/"
Task: "Setup JSON Lines logging infrastructure in src/logging/"

# Launch user stories in parallel:
Task: "Implement rule matching logic for command patterns in src/hooks/matching.rs"
Task: "Implement file path matching for directories in src/hooks/matching.rs"
```

---

## Implementation Strategy

### MVP First (User Stories 1 & 2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phases 3-4: User Stories 1 & 2
4. **STOP and VALIDATE**: Test User Stories 1 & 2 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Stories 1 & 2 → Test independently → Deploy/Demo (MVP!)
3. Add User Stories 3 & 4 → Test independently → Deploy/Demo
4. Add User Story 5 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Stories 1 & 2 (blocking/injection core features)
   - Developer B: User Stories 3 & 4 (validation/explanation features)
   - Developer C: User Story 5 (logging features)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence</content>
<parameter name="filePath">specs/001-cch-binary-v1/tasks.md