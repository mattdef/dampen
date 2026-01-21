# Tasks: Fix Code Quality Issues in dampen-dev

**Input**: Design documents from `/specs/001-fix-code-quality-issues/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md
**Tests**: Tests are included as part of the implementation work (test fixes and new test creation)
**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `crates/dampen-dev/src/`, `crates/dampen-dev/tests/` at repository root
- Paths shown below assume single project structure from plan.md

---

## Phase 1: Setup (Prerequisite Verification)

**Purpose**: Verify branch and ensure clean build state before implementing fixes

- [ ] T001 Verify branch `001-fix-code-quality-issues` is checked out
- [ ] T002 Run `cargo build -p dampen-dev` to ensure clean build state
- [ ] T003 Run `cargo test -p dampen-dev` to verify current test state
- [ ] T004 Search codebase for any existing references to `FileDeleted` error variant

**Checkpoint**: Environment verified - ready to implement fixes

---

## Phase 2: Foundational (No Blocking Prerequisites)

**Purpose**: This feature consists entirely of independent fixes to existing code. No foundational infrastructure is needed.

> **Note**: Unlike typical features, this code quality improvement has no blocking prerequisites. Each fix is independent and can be implemented in any order. User story organization follows logical grouping for testing, not dependency constraints.

**Checkpoint**: Ready to implement user stories in priority order

---

## Phase 3: User Story 1 - Reliable Test Suite (Priority: P1) 🎯 MVP

**Goal**: Fix flaky `test_debouncing_behavior` test so it passes consistently when run with the full test suite

**Independent Test**: Run `cargo test -p dampen-dev` 10 times consecutively and verify all tests pass without intermittent failures

### Implementation for User Story 1

- [ ] T005 Add `TestTiming` struct with configurable debounce duration, wait multiplier, test timeout, and poll interval in crates/dampen-dev/tests/watcher_tests.rs
- [ ] T006 Implement `Default` trait for `TestTiming` with default values (100ms debounce, 1.5x multiplier, 500ms timeout, 5ms poll interval) in crates/dampen-dev/tests/watcher_tests.rs
- [ ] T007 [P] Implement `wait_for_debounce()` method on `TestTiming` that returns `debounce_duration * wait_multiplier` in crates/dampen-dev/tests/watcher_tests.rs
- [ ] T008 [P] Add `wait_for_events<T>()` helper function that uses `try_recv()` loop with timeout for active event polling in crates/dampen-dev/tests/watcher_tests.rs
- [ ] T009 Replace hardcoded `thread::sleep(Duration::from_millis(150))` in `wait_for_debounce()` with `thread::sleep(TestTiming::default().wait_for_debounce())` in crates/dampen-dev/tests/watcher_tests.rs
- [ ] T010 Update assertion in `test_debouncing_behavior` to use >= 20% threshold instead of >= 30% with improved error message explaining debouncing variability in crates/dampen-dev/tests/watcher_tests.rs
- [ ] T011 Use `wait_for_events()` helper to collect events with timeout in `test_debouncing_behavior` in crates/dampen-dev/tests/watcher_tests.rs
- [ ] T012 Run `test_debouncing_behavior` 10 times consecutively to verify it no longer fails intermittently

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Accurate Performance Metrics (Priority: P1) 🎯 MVP

**Goal**: Implement cache hit/miss tracking so that `calculate_cache_hit_rate()` returns accurate values instead of always returning 0.0

**Independent Test**: Trigger hot-reload multiple times with known hit/miss patterns and verify the reported cache hit rate correctly reflects the percentage of cache hits vs. misses

### Implementation for User Story 2

- [ ] T013 Add `use std::sync::atomic::{AtomicUsize, Ordering}` import at top of crates/dampen-dev/src/reload.rs
- [ ] T014 Add `cache_hits: AtomicUsize` field to `HotReloadContext<M>` struct in crates/dampen-dev/src/reload.rs
- [ ] T015 Add `cache_misses: AtomicUsize` field to `HotReloadContext<M>` struct in crates/dampen-dev/src/reload.rs
- [ ] T016 Initialize `cache_hits` to `AtomicUsize::new(0)` in `HotReloadContext::new()` constructor in crates/dampen-dev/src/reload.rs
- [ ] T017 Initialize `cache_misses` to `AtomicUsize::new(0)` in `HotReloadContext::new()` constructor in crates/dampen-dev/src/reload.rs
- [ ] T018 [P] Add `compute_content_hash(xml_source: &str) -> u64` helper function that computes hash using `DefaultHasher` in crates/dampen-dev/src/reload.rs
- [ ] T019 Update `get_cached_document()` to call `compute_content_hash(xml_source)` instead of inline hash computation in crates/dampen-dev/src/reload.rs
- [ ] T020 Add `self.cache_hits.fetch_add(1, Ordering::Relaxed)` when cache hit occurs in `get_cached_document()` in crates/dampen-dev/src/reload.rs
- [ ] T021 Add `self.cache_misses.fetch_add(1, Ordering::Relaxed)` when cache miss occurs in `get_cached_document()` in crates/dampen-dev/src/reload.rs
- [ ] T022 [P] Update `cache_document()` to call `compute_content_hash(xml_source)` instead of inline hash computation in crates/dampen-dev/src/reload.rs
- [ ] T023 Implement `calculate_cache_hit_rate()` method that loads hits and misses, calculates `hits / (hits + misses)` with zero-division handling (return 0.0 if total == 0) in crates/dampen-dev/src/reload.rs
- [ ] T024 Update `ReloadPerformanceMetrics::cache_hit_rate()` to delegate to `calculate_cache_hit_rate()` in crates/dampen-dev/src/reload.rs
- [ ] T025 Add unit test `test_cache_hit_rate_calculated_correctly()` that simulates hits/misses and verifies correct rate calculation in crates/dampen-dev/src/reload.rs
- [ ] T026 Add unit test `test_cache_hit_rate_zero_division()` that verifies return value is 0.0 when no reloads have occurred in crates/dampen-dev/src/reload.rs
- [ ] T027 Run `cargo test -p dampen-dev` to verify all cache metrics tests pass

**Checkpoint**: At this point, User Story 2 should be fully functional and testable independently

---

## Phase 5: User Story 3 - Robust File Event Handling (Priority: P2)

**Goal**: Upgrade file event channel from `mpsc::channel(100)` to `mpsc::channel(1000)` to handle bulk file operations without dropping events

**Independent Test**: Simulate 500+ rapid file changes and verify all change events are captured without loss

### Implementation for User Story 3

- [ ] T028 Find `mpsc::channel(100)` in crates/dampen-dev/src/subscription.rs (around line 160)
- [ ] T029 Replace `mpsc::channel(100)` with `mpsc::channel(1000)` to increase buffer capacity to 1000 events in crates/dampen-dev/src/subscription.rs
- [ ] T030 [P] Add optional channel health monitoring task that logs warning when channel is >80% full in crates/dampen-dev/src/subscription.rs
- [ ] T031 Run `cargo build -p dampen-dev` to verify channel buffer change compiles
- [ ] T032 Run `cargo test -p dampen-dev` to verify all tests pass after channel buffer change

**Checkpoint**: At this point, User Story 3 should be fully functional and testable independently

---

## Phase 6: User Story 4 - Optimized Performance (Priority: P2)

**Goal**: Eliminate unnecessary data duplication (Arc refactor) and compute content hash only once per cache operation to reduce memory usage and improve performance

**Independent Test**: Profile hot-reload operations on 500KB+ XML files and verify memory usage does not double during async parsing and hash is computed once

### Implementation for User Story 4

- [ ] T033 Add `use std::sync::Arc` import at top of crates/dampen-dev/src/reload.rs
- [ ] T034 Change `attempt_hot_reload_async()` function signature from `xml_source: String` to `xml_source: Arc<String>` in crates/dampen-dev/src/reload.rs
- [ ] T035 Replace `xml_source.clone()` with `Arc::clone(&xml_source)` in `attempt_hot_reload_async()` before passing to `spawn_blocking` in crates/dampen-dev/src/reload.rs
- [ ] T036 Update `get_cached_document(&xml_source)` call (Arc<String> automatically derefs to &str) in crates/dampen-dev/src/reload.rs
- [ ] T037 Update `cache_document(&xml_source, doc.clone())` call (Arc<String> automatically derefs to &str) in crates/dampen-dev/src/reload.rs
- [ ] T038 [P] Add optional `get_or_cache_document<F>()` method using Entry API pattern that computes hash once and returns cached or newly computed document in crates/dampen-dev/src/reload.rs
- [ ] T039 Run `cargo build -p dampen-dev` to verify Arc refactor and hash optimizations compile
- [ ] T040 Run `cargo test -p dampen-dev` to verify all tests pass after performance optimizations

**Checkpoint**: At this point, User Story 4 should be fully functional and testable independently

---

## Phase 7: User Story 5 - Clean and Documented Code (Priority: P3)

**Goal**: Remove dead code (`FileDeleted` error variant) and add comprehensive documentation for `FileWatcherState` state transitions

**Independent Test**: Review API documentation and verify no unused error variants exist, check that `cargo doc -p dampen-dev --open` shows complete state machine documentation

### Implementation for User Story 5

- [ ] T041 Find `FileWatcherError` enum in crates/dampen-dev/src/watcher.rs (around line 336)
- [ ] T042 Remove `FileDeleted(PathBuf)` variant from `FileWatcherError` enum in crates/dampen-dev/src/watcher.rs
- [ ] T043 Search codebase for `FileDeleted` usage to confirm it was never used: run `rg "FileDeleted" crates/dampen-dev/`
- [ ] T044 [P] Find `FileWatcherState` enum in crates/dampen-dev/src/watcher.rs (around line 39)
- [ ] T045 Replace `FileWatcherState` enum documentation with comprehensive state machine diagram and detailed state descriptions in crates/dampen-dev/src/watcher.rs
- [ ] T046 [P] Add example usage code to `FileWatcherState` documentation showing Idle → Watching → Failed transitions in crates/dampen-dev/src/watcher.rs
- [ ] T047 Run `cargo build -p dampen-dev` to verify documentation changes compile
- [ ] T048 Run `cargo doc -p dampen-dev --open` to verify updated documentation renders correctly

**Checkpoint**: At this point, User Story 5 should be fully functional and testable independently

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final validation and quality checks across all implemented fixes

- [ ] T049 Run `cargo build -p dampen-dev` to ensure clean build across all fixes
- [ ] T050 Run `cargo test -p dampen-dev` 10 times consecutively to verify no test flakiness remains
- [ ] T051 Run `cargo clippy -p dampen-dev -- -D warnings` to verify zero clippy warnings
- [ ] T052 Run `cargo fmt -p dampen-dev -- --check` to verify proper code formatting
- [ ] T053 Run `cargo doc -p dampen-dev --open` to verify all public documentation renders correctly
- [ ] T054 Review all 10 fixes against quickstart.md checklist to ensure completeness

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: No blocking tasks - ready to implement user stories immediately
- **User Stories (Phase 3-7)**: All independent, can implement in any order
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: No dependencies on other stories - fully independent
- **User Story 2 (P1)**: No dependencies on other stories - fully independent
- **User Story 3 (P2)**: No dependencies on other stories - fully independent
- **User Story 4 (P2)**: No dependencies on other stories - fully independent
- **User Story 5 (P3)**: No dependencies on other stories - fully independent

### Within Each User Story

- **User Story 1**: Tasks are sequential (TestTiming → wait_for_debounce → assertion → validation)
- **User Story 2**: AtomicUsize fields → initialization → hash helper → get_cached_document → cache_document → calculate_cache_hit_rate → tests (T013-T027 sequential)
- **User Story 3**: Single task with optional monitoring (T028-T032)
- **User Story 4**: Arc import → signature change → Arc::clone → call site updates → optional get_or_cache (T033-T040 sequential)
- **User Story 5**: Remove FileDeleted → verify unused → add FileWatcherState docs (T041-T048 sequential)

### Parallel Opportunities

- **User Story 1**: T005, T006, T007 can be implemented in parallel (different parts of TestTiming struct)
- **User Story 2**: T018 and T022 can be implemented in parallel (both add compute_content_hash calls)
- **User Story 4**: T038 can be implemented in parallel with T033-T037 (optional enhancement, not required for core fix)
- **User Story 5**: T041-T043 and T044-T046 can be implemented in parallel (different files/modules within the story)
- **Across user stories**: Since all stories are independent, multiple team members can work on different user stories in parallel

---

## Parallel Example: User Story 2

```bash
# Launch compute_content_hash helper tasks together (T018, T022):
# These can be worked on in parallel since they affect different methods

# Parallel execution of get_cached_document (T019-T021) and cache_document (T022):
# Can work on both methods simultaneously as they're in same file but independent
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (no blocking tasks)
3. Complete Phase 3: User Story 1 (Reliable Test Suite)
4. Complete Phase 4: User Story 2 (Accurate Performance Metrics)
5. **STOP and VALIDATE**: Run test suite 10 times, verify cache metrics work correctly
6. Deploy/demo if ready

**MVP Scope**: Fixes flaky test and implements cache hit/miss tracking - directly impacts developer productivity and confidence in test suite

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 (Fix Flaky Test) → Test 10x successfully → MVP increment 1
3. Add User Story 2 (Cache Metrics) → Verify hit rate calculations → MVP increment 2
4. Add User Story 3 (Channel Buffer) → Verify bulk operations → Increment 3
5. Add User Story 4 (Performance Optimization) → Profile and verify → Increment 4
6. Add User Story 5 (Clean Documentation) → Review docs → Increment 5
7. Each fix adds value without breaking previous fixes

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup together (T001-T004)
2. Once Setup is done, all user stories can proceed in parallel:
   - Developer A: User Story 1 (T005-T012)
   - Developer B: User Story 2 (T013-T027)
   - Developer C: User Story 3 (T028-T032)
   - Developer D: User Story 4 (T033-T040)
   - Developer E: User Story 5 (T041-T048)
3. Team completes Polish phase together (T049-T054)

---

## Notes

- [P] tasks = different files or independent operations, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable (no cross-story dependencies)
- Run tests after each user story to catch regressions early
- Stop at any user story checkpoint to validate story independently
- Tasks are specific and immediately executable - LLM can complete without additional context
- Format validation: All tasks follow checklist format with checkbox, ID, optional [P] marker, story label, and file path
