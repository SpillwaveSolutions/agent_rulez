# Tasks: CRD-001 Enhanced Logging

**Input**: Design documents from `.specify/features/enhanced-logging/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are included as they are critical for validating the enhanced logging functionality.

**Organization**: Tasks are grouped by implementation phase to enable systematic implementation and testing.

## Format: `[ID] [P?] [Phase] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Phase]**: Implementation phase this task belongs to
- Include exact file paths in descriptions

---

## Phase 1: Core Data Structures (Priority: P1)

**Purpose**: Add enhanced logging data structures to models.rs

**Independent Test**: Compile successfully and basic struct creation works

### Tests for Phase 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T001 [P] [P1] Unit tests for EventDetails enum variants in cch_cli/src/models.rs
- [ ] T002 [P] [P1] Unit tests for ResponseSummary struct in cch_cli/src/models.rs
- [ ] T003 [P] [P1] Unit tests for RuleEvaluation and MatcherResults structs in cch_cli/src/models.rs
- [ ] T004 [P] [P1] Unit tests for DebugConfig struct in cch_cli/src/models.rs

### Implementation for Phase 1

- [ ] T005 [P] [P1] Add EventDetails enum with tool-specific variants in cch_cli/src/models.rs after LogMetadata
- [ ] T006 [P] [P1] Add ResponseSummary struct in cch_cli/src/models.rs
- [ ] T007 [P] [P1] Add RuleEvaluation and MatcherResults structs in cch_cli/src/models.rs
- [ ] T008 [P] [P1] Add DebugConfig struct with constructor in cch_cli/src/models.rs
- [ ] T009 [P] [P1] Extend LogEntry with new optional fields (event_details, response, raw_event, rule_evaluations) in cch_cli/src/models.rs

**Checkpoint**: At this point, all new data structures should compile and basic unit tests should pass

---

## Phase 2: Event Extraction Logic (Priority: P1)

**Purpose**: Implement EventDetails::extract() method and ResponseSummary::from_response()

**Independent Test**: EventDetails extraction works for all supported tool types

### Tests for Phase 2 ⚠️

- [ ] T010 [P] [P2] Test EventDetails::extract() for Bash tool events in cch_cli/src/models.rs
- [ ] T011 [P] [P2] Test EventDetails::extract() for Write tool events in cch_cli/src/models.rs
- [ ] T012 [P] [P2] Test EventDetails::extract() for Edit tool events in cch_cli/src/models.rs
- [ ] T013 [P] [P2] Test EventDetails::extract() for Read tool events in cch_cli/src/models.rs
- [ ] T014 [P] [P2] Test EventDetails::extract() for Glob tool events in cch_cli/src/models.rs
- [ ] T015 [P] [P2] Test EventDetails::extract() for Grep tool events in cch_cli/src/models.rs
- [ ] T016 [P] [P2] Test EventDetails::extract() for Session events in cch_cli/src/models.rs
- [ ] T017 [P] [P2] Test EventDetails::extract() for unknown tool events in cch_cli/src/models.rs
- [ ] T018 [P] [P2] Test ResponseSummary::from_response() in cch_cli/src/models.rs

### Implementation for Phase 2

- [ ] T019 [P] [P2] Implement EventDetails::extract() method for all supported tool types in cch_cli/src/models.rs
- [ ] T020 [P] [P2] Implement ResponseSummary::from_response() method in cch_cli/src/models.rs

**Checkpoint**: At this point, event extraction should work for all tool types and response summarization should be complete

---

## Phase 3: Hook Integration (Priority: P1)

**Purpose**: Integrate enhanced logging into the hook processing pipeline

**Independent Test**: Normal mode logs contain event_details and response fields

### Tests for Phase 3 ⚠️

- [ ] T021 [P] [P3] Integration test for normal mode logging in cch_cli/tests/oq_enhanced_logging.rs
- [ ] T022 [P] [P3] Integration test for debug mode logging in cch_cli/tests/oq_enhanced_logging.rs

### Implementation for Phase 3

- [ ] T023 [P] [P3] Modify process_event() signature to accept DebugConfig parameter in cch_cli/src/hooks.rs
- [ ] T024 [P] [P3] Extract EventDetails from event in process_event() in cch_cli/src/hooks.rs
- [ ] T025 [P] [P3] Build ResponseSummary from response in process_event() in cch_cli/src/hooks.rs
- [ ] T026 [P] [P3] Conditionally include raw_event in LogEntry when debug mode enabled in cch_cli/src/hooks.rs
- [ ] T027 [P] [P3] Track and include rule_evaluations in LogEntry when debug mode enabled in cch_cli/src/hooks.rs
- [ ] T028 [P] [P3] Update evaluate_rules() to optionally return rule evaluations for debug mode in cch_cli/src/hooks.rs

**Checkpoint**: At this point, enhanced logging should work in both normal and debug modes

---

## Phase 4: Debug Mode Plumbing (Priority: P2)

**Purpose**: Add CLI flags, environment variables, and config file support for debug mode

**Independent Test**: All three debug mode activation methods work (--debug-logs flag, CCH_DEBUG_LOGS env var, config setting)

### Tests for Phase 4 ⚠️

- [ ] T029 [P] [P4] Test --debug-logs CLI flag enables debug mode in cch_cli/tests/oq_enhanced_logging.rs
- [ ] T030 [P] [P4] Test CCH_DEBUG_LOGS environment variable enables debug mode in cch_cli/tests/oq_enhanced_logging.rs
- [ ] T031 [P] [P4] Test config file debug_logs setting enables debug mode in cch_cli/tests/oq_enhanced_logging.rs

### Implementation for Phase 4

- [ ] T032 [P] [P4] Add debug_logs field to Settings struct in cch_cli/src/config.rs
- [ ] T033 [P] [P4] Add --debug-logs global CLI flag to Cli struct in cch_cli/src/main.rs
- [ ] T034 [P] [P4] Update DebugConfig::new() to accept cli_flag and config_setting parameters in cch_cli/src/models.rs
- [ ] T035 [P] [P4] Build DebugConfig from CLI flag, env var, and config setting in main.rs
- [ ] T036 [P] [P4] Pass DebugConfig to process_event() call in main.rs

**Checkpoint**: At this point, debug mode should be activatable via CLI flag, environment variable, and config file

---

## Phase 5: Comprehensive Testing (Priority: P1)

**Purpose**: Add comprehensive unit and integration tests for enhanced logging

**Independent Test**: All tests pass and cover edge cases

### Tests for Phase 5 ⚠️

- [ ] T037 [P] [P5] Test backward compatibility with old log entries in cch_cli/tests/oq_enhanced_logging.rs
- [ ] T038 [P] [P5] Test large event serialization performance in cch_cli/tests/oq_enhanced_logging.rs
- [ ] T039 [P] [P5] Test Permission event recursion prevention in cch_cli/tests/oq_enhanced_logging.rs
- [ ] T040 [P] [P5] Test malformed tool_input handling in extraction in cch_cli/src/models.rs

### Implementation for Phase 5

- [ ] T041 [P] [P5] Add comprehensive unit tests to models.rs for all EventDetails variants
- [ ] T042 [P] [P5] Create cch_cli/tests/oq_enhanced_logging.rs with integration tests
- [ ] T043 [P] [P5] Add tests for debug mode activation methods
- [ ] T044 [P] [P5] Add tests for backward compatibility
- [ ] T045 [P] [P5] Add edge case tests for malformed inputs

**Checkpoint**: At this point, all tests should pass and coverage should be comprehensive

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Data Structures)**: No dependencies - can start immediately
- **Phase 2 (Extraction Logic)**: Depends on Phase 1 completion
- **Phase 3 (Hook Integration)**: Depends on Phase 1 and Phase 2 completion
- **Phase 4 (Debug Plumbing)**: Depends on Phase 1 completion - can run parallel to Phase 2/3
- **Phase 5 (Testing)**: Depends on all implementation phases (1-4) completion

### Within Each Phase

- Tests (if included) MUST be written and FAIL before implementation
- Data structures before logic
- Core functionality before plumbing
- Unit tests before integration tests

### Parallel Opportunities

- All test tasks marked [P] can run in parallel within their phase
- Phase 4 can run in parallel with Phase 2 and 3 once Phase 1 is complete
- Different test files can be worked on in parallel
- Implementation tasks within a phase can be parallelized where marked [P]

---

## Implementation Strategy

### MVP First Approach

1. Complete Phase 1: Core Data Structures
2. Complete Phase 2: Event Extraction Logic
3. Complete Phase 3: Hook Integration
4. **STOP and VALIDATE**: Test enhanced logging works in normal mode
5. Deploy/demo if ready

### Full Implementation

1. Complete Phases 1-3 → Basic enhanced logging functional
2. Add Phase 4 → Debug mode fully supported
3. Add Phase 5 → Comprehensive test coverage
4. **VALIDATE**: All tests pass, backward compatibility maintained

### Risk Mitigation Strategy

- **Phase 1-3 first**: Get core functionality working before debug features
- **Backward compatibility tests**: Ensure existing log parsers still work
- **Performance testing**: Validate debug mode doesn't impact normal operation

---

## Success Verification

After implementation:

1. **Normal Mode**: `event_details` and `response` fields appear in logs
2. **Debug Mode**: `raw_event` and `rule_evaluations` fields appear when enabled
3. **CLI Flag**: `--debug-logs` flag works
4. **Env Var**: `CCH_DEBUG_LOGS=1` works
5. **Config**: `debug_logs: true` in config works
6. **Backward Compatibility**: Old logs parse correctly
7. **All Tests Pass**: `cargo test` succeeds

---

## Notes

- [P] tasks = different test files or independent implementation units
- [Phase] label maps task to specific implementation phase
- Tests are written FIRST and must FAIL before implementation
- Each phase should be completable and testable independently
- Debug mode features are additive - normal mode works without them
- All new fields are optional with `skip_serializing_if` for backward compatibility</content>
<parameter name="filePath">.specify/features/enhanced-logging/tasks.md