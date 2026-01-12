# Task Breakdown Summary: #[dampen_app] Macro

**Generated**: 2026-01-12
**Feature**: Auto-Discovery Multi-View Application with #[dampen_app] Macro
**Location**: specs/001-dampen-app-macro/tasks.md

---

## Overview

**Total Tasks**: 116 tasks across 9 phases
**TDD Approach**: 47 test tasks (40% of total) - must FAIL before implementation
**Parallel Opportunities**: 47 tasks marked [P] - can run concurrently
**MVP Scope**: US1 + US2 = 37 tasks (discovery + routing core functionality)

---

## Phase Breakdown

| Phase | Description | Tasks | Story Focus |
|-------|-------------|-------|-------------|
| **Phase 1** | Setup | 4 | Infrastructure (dependencies, directories) |
| **Phase 2** | Foundational | 9 | Core data structures (BLOCKS all stories) |
| **Phase 3** | User Story 1 (P1) | 26 | View Discovery & Initialization |
| **Phase 4** | User Story 2 (P1) | 11 | View Switching Without Routing |
| **Phase 5** | User Story 3 (P2) | 12 | Hot-Reload Support |
| **Phase 6** | User Story 4 (P3) | 9 | Selective View Exclusion |
| **Phase 7** | User Story 5 (P2) | 16 | Clear Compile-Time Errors |
| **Phase 8** | Integration | 16 | widget-showcase Migration |
| **Phase 9** | Polish | 13 | Documentation & Quality |

---

## User Story Distribution

### Priority 1 (Must Have - MVP)

**US1: View Discovery & Initialization** (26 tasks)
- File discovery with glob patterns
- ViewInfo validation and deduplication
- Code generation (Message enum, AppState struct, initialization)
- Contract tests: 3 views → correct ViewInfo

**US2: View Switching Without Routing** (11 tasks)
- View switching message generation
- update() method with view enum matching
- view() method with view enum rendering
- Contract tests: widget-showcase structure

**MVP Total**: 37 tasks (Phases 3-4)

### Priority 2 (Should Have)

**US3: Hot-Reload Support** (12 tasks)
- File watcher subscription in generated code
- Error overlay widget rendering
- Hot-reload testing with multi-view fixtures
- Contract tests: file change → reload

**US5: Clear Error Messages** (16 tasks)
- 7 compile-fail test cases (trybuild)
- Error reporting with spans and suggestions
- Contract tests: all errors show paths + fixes

**Priority 2 Total**: 28 tasks (Phases 5, 7)

### Priority 3 (Could Have)

**US4: Selective Exclusion** (9 tasks)
- Glob pattern parsing for #[dampen_app(exclude = "...")]
- Exclusion filtering in discovery logic
- Contract tests: excluded views not generated

**Priority 3 Total**: 9 tasks (Phase 6)

---

## Critical Path & Dependencies

```
Phase 1: Setup (4 tasks)
    ↓
Phase 2: Foundational (9 tasks) ← BLOCKS ALL STORIES
    ↓
Phase 3: US1 - Discovery (26 tasks) ← MVP Core
    ↓
Phase 4: US2 - Routing (11 tasks) ← MVP Complete
    ↓
Phase 5: US3 - Hot-Reload (12 tasks)
    ↓
Phase 8: Integration (16 tasks)
    ↓
Phase 9: Polish (13 tasks)

Independent Paths (can run after Phase 2):
- Phase 6: US4 - Exclusion (9 tasks) [no dependencies]
- Phase 7: US5 - Errors (16 tasks) [no dependencies]
```

**Key Dependencies**:
- US2 REQUIRES US1 (routing needs discovered views)
- US3 REQUIRES US2 (hot-reload needs complete routing)
- US4 and US5 are INDEPENDENT (can run in parallel after Phase 2)

---

## Test Tasks Distribution

**Test-First Tasks**: 47 total (must FAIL before implementation)

| Phase | Test Tasks | Focus |
|-------|------------|-------|
| Phase 2 | 4 | ViewInfo, MacroAttributes unit tests |
| Phase 3 (US1) | 9 | Discovery, validation, code generation |
| Phase 4 (US2) | 4 | View switching, update/view methods |
| Phase 5 (US3) | 4 | Hot-reload subscription |
| Phase 6 (US4) | 3 | Exclusion patterns |
| Phase 7 (US5) | 7 | Compile-fail error messages |
| Phase 8 | 10 | Integration tests |
| Phase 9 | 6 | Documentation examples |

---

## Parallel Execution Opportunities

**47 tasks marked [P]** (same phase, different files, no dependencies)

Examples:
- **Phase 2**: T005-T008 (ViewInfo, MacroAttributes, helpers - all independent structs)
- **Phase 3**: T014-T018 (5 contract tests can run in parallel)
- **Phase 7**: T072-T078 (7 compile-fail tests can run in parallel)
- **Phase 9**: T104-T111 (8 documentation tasks can run in parallel)

**Optimization**: Use parallel test execution (`cargo test -j N`) for test tasks

---

## Success Criteria Checkpoints

| ID | Criterion | Verification Task | Phase |
|----|-----------|-------------------|-------|
| SC-001 | 85% boilerplate reduction (500→<100 LOC) | T102 | Phase 8 |
| SC-002 | Discovery < 200ms for 20 views | T103 | Phase 9 |
| SC-003 | New view = 2 files only | T024 | Phase 3 |
| SC-004 | Zero manual match statements | T044 | Phase 4 |
| SC-005 | Hot-reload < 500ms | T055 | Phase 5 |
| SC-006 | 100% errors include paths | T086 | Phase 7 |
| SC-007 | Zero runtime overhead | T103 | Phase 9 |
| SC-008 | widget-showcase migration | T088 | Phase 8 |

---

## File Structure Generated

New files to create:
```
crates/dampen-macros/
├── src/
│   ├── dampen_app.rs         # T012: Main macro implementation
│   └── discovery.rs           # T011: File discovery logic
├── tests/
│   ├── dampen_app_tests.rs   # T014: Unit + snapshot tests
│   ├── fixtures/             # T003: Test fixtures
│   │   ├── multi_view/       # T015: 3-view test case
│   │   ├── single_view/      # T016: Minimal test case
│   │   └── edge_cases/       # T017: Empty, invalid tests
│   └── ui/                   # T004: Compile-fail tests
│       ├── err_missing_ui_dir.rs    # T072
│       ├── err_invalid_message.rs   # T073
│       └── ... (7 total)
└── Cargo.toml                # T001-T002: Dependencies
```

Modified files:
```
crates/dampen-macros/src/lib.rs  # T013: Export #[dampen_app] macro
```

---

## Next Steps

### Step 1: Commit Tasks File
```bash
git add specs/001-dampen-app-macro/tasks.md
git commit -m "feat(spec): Complete task breakdown for #[dampen_app] macro"
```

### Step 2: Start Phase 1 (Setup)
```bash
# T001: Add glob dependency
# T002: Add trybuild dev-dependency
# T003: Create fixtures directory
# T004: Create UI tests directory
```

### Step 3: Complete Phase 2 (Foundational)
This phase BLOCKS all user stories - must complete all 9 tasks first.

### Step 4: Choose Implementation Path

**Option A - MVP First (Recommended)**:
1. Phase 3: US1 (26 tasks) - Get basic discovery working
2. Phase 4: US2 (11 tasks) - Add routing
3. Validate MVP with integration tests
4. Continue with US3, US4, US5

**Option B - Story-by-Story**:
1. Complete US1 entirely (Phase 3)
2. Test independently with 3-view fixture
3. Complete US2 (Phase 4)
4. Test independently with widget-showcase structure
5. Add US3, US4, US5 incrementally

**Option C - Parallel Development**:
1. Complete Phases 1-4 (MVP)
2. Run Phases 6-7 in parallel (US4 and US5 are independent)
3. Integrate with Phase 8
4. Polish with Phase 9

---

## Validation Checklist

Before starting implementation:
- [ ] All design docs reviewed (spec, plan, data-model, contracts, quickstart, research)
- [ ] Task dependencies understood (US2→US1, US3→US2)
- [ ] TDD workflow accepted (tests MUST fail first)
- [ ] Phase 2 recognized as blocker for all stories
- [ ] MVP scope agreed (US1 + US2 = 37 tasks)
- [ ] Success criteria understood (8 checkpoints)
- [ ] File structure planned (2 new source files, test infrastructure)

---

## Estimated Effort

**Rough time estimates** (assumes experienced Rust dev, TDD workflow):

| Phase | Tasks | Est. Time | Cumulative |
|-------|-------|-----------|------------|
| Phase 1 | 4 | 0.5h | 0.5h |
| Phase 2 | 9 | 2-3h | 3-3.5h |
| Phase 3 (US1) | 26 | 6-8h | 9-11.5h |
| Phase 4 (US2) | 11 | 3-4h | 12-15.5h ← MVP |
| Phase 5 (US3) | 12 | 3-4h | 15-19.5h |
| Phase 6 (US4) | 9 | 2-3h | 17-22.5h |
| Phase 7 (US5) | 16 | 4-5h | 21-27.5h |
| Phase 8 | 16 | 4-6h | 25-33.5h |
| Phase 9 | 13 | 3-4h | 28-37.5h |

**Total**: ~28-38 hours (3.5-5 days full-time)
**MVP Only**: ~12-16 hours (1.5-2 days full-time)

---

## Constitution Compliance

✅ **Principle I: Declarative-First** - Macro reinforces XML as source of truth
✅ **Principle II: Type Safety** - Generated code preserves Message<M> types
✅ **Principle III: Production Mode** - Compile-time code generation only
✅ **Principle IV: Backend Abstraction** - No Iced dependency in macro crate
✅ **Principle V: TDD** - 47 test tasks (40%) written before implementation

---

**Report Generated**: /tmp/tasks_summary.md
**Ready for**: Phase 1 execution
