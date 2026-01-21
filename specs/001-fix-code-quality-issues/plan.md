# Implementation Plan: Fix Code Quality Issues in dampen-dev

**Branch**: `001-fix-code-quality-issues` | **Date**: January 21, 2026 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-fix-code-quality-issues/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement cache hit/miss tracking in `HotReloadContext` to provide accurate performance metrics, fix flaky watcher test with synchronization or tolerant assertions, remove unused `FileDeleted` error variant, optimize memory usage by eliminating unnecessary data duplication in async operations, compute content hash only once during cache operations, upgrade file event channel to handle bulk operations without dropping events, add comprehensive documentation for `FileWatcherState` state transitions, and make test timing configurable.

## Technical Context

**Language/Version**: Rust 2024, MSRV 1.85
**Primary Dependencies**: tokio (async runtime), notify-debouncer-full (file watching), iced (UI framework), std collections
**Storage**: In-memory cache (LHashMap)
**Testing**: cargo test with proptest (property-based), insta (snapshots)
**Target Platform**: Linux, macOS, Windows (development tools)
**Project Type**: single (dampen-dev crate in workspace)
**Performance Goals**: Eliminate unnecessary memory duplication (avoid 2x memory usage during async parsing), compute hash once per operation (reduce O(2n) to O(n) for hashing)
**Constraints**: Must maintain backward compatibility for public APIs, must not increase test flakiness, must not introduce `unsafe` code
**Scale/Scope**: 10 code quality issues across 4 modules (reload.rs, watcher.rs, subscription.rs, tests/), ~12 hours estimated effort

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Principle I: Declarative-First
**Status**: ✅ PASS - No changes to XML parsing or UI structure

### Principle II: Type Safety Preservation
**Status**: ✅ PASS - No runtime type erasure, all changes use existing Rust types

### Principle III: Production Mode
**Status**: ✅ PASS - No changes to codegen mode, all changes affect development mode only

### Principle IV: Backend Abstraction
**Status**: ✅ PASS - All changes are in dampen-dev crate, no Iced dependencies in core

### Principle V: Test-First Development
**Status**: ✅ PASS - Fixing flaky tests, adding new tests for cache metrics

### Technical Standards
**Status**: ✅ PASS - Using Rust 2024, MSRV 1.85, compliant dependencies

### Quality Gates
**Status**: ✅ PASS - Will maintain zero clippy warnings, all tests must pass, properly formatted code, documented public APIs

### Error Handling
**Status**: ✅ PASS - Will use Result types, custom errors with thiserror, no unwrap/expect/panic

### Performance Budgets
**Status**: ✅ PASS - Optimizations eliminate waste without adding overhead (AtomicUsize: ~5-10ns, Arc: 1000-10000x faster, single hash computation eliminates duplication)

**CONCLUSION**: All gates pass. Proceed to Phase 0 research.

## Project Structure

### Documentation (this feature)

```text
specs/001-fix-code-quality-issues/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command - not needed for internal fixes)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Single project (dampen-dev crate within workspace)
crates/dampen-dev/
├── src/
│   ├── reload.rs          # Cache metrics, hash optimization, Arc refactor
│   ├── watcher.rs         # Remove FileDeleted error variant
│   ├── subscription.rs   # Change mpsc::channel(100) to unbounded_channel
│   └── lib.rs             # Public API exports
├── tests/
│   └── watcher_tests.rs   # Fix flaky debounce test, configurable timing
└── Cargo.toml            # Dependencies (tokio, notify-debouncer-full)

# Existing structure (no changes)
crates/dampen-core/        # No changes
crates/dampen-iced/        # No changes
crates/dampen-macros/      # No changes
crates/dampen-cli/         # No changes
```

**Structure Decision**: Single project within dampen-dev crate. All 10 code quality issues are internal fixes to existing modules in dampen-dev, no new crate or workspace changes needed. Tests will be updated in-place in tests/watcher_tests.rs.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No constitution violations. All changes are internal optimizations and quality improvements that maintain backward compatibility and follow existing patterns.

---

## Phase 0: Research

### Research Tasks

Based on Technical Context, the following research tasks need to be completed:

1. **Atomic counter selection for cache metrics**: Determine whether to use `AtomicUsize` directly in `HotReloadContext` or pass mutable references for hit/miss tracking. Trade-offs: atomic avoids signature changes but has small overhead; mutable references require refactoring but are more idiomatic in Rust.

2. **Tokio channel best practices**: Research whether `mpsc::unbounded_channel()` is appropriate for file event handling or if `channel(1000)` with backpressure is better. Need to understand memory leak risks vs. event loss trade-offs.

3. **Arc vs. String clone for async parsing**: Evaluate whether changing `attempt_hot_reload_async` signature to accept `Arc<String>` is worth the API change for eliminating clones, or if checking cache before clone (existing pattern) is sufficient.

4. **Hash computation optimization**: Determine best approach for eliminating duplicate hashing - either extract `compute_content_hash()` helper function or combine get/cache into single method with closure.

5. **Test synchronization patterns**: Research best practices for testing debouncing behavior - whether to use `try_recv()` with timeout loops, channel synchronization, or just tolerant assertions.

### Expected Research Outcomes

1. Decision on cache metric tracking approach (atomic vs. mutable ref)
2. Recommendation for channel buffer size (unbounded vs. bounded with backpressure)
3. Decision on Arc refactor (worth API change or use existing pattern)
4. Hash optimization approach selected (helper function vs. combined method)
5. Test synchronization pattern chosen (active sync vs. tolerant assertions)

---

## Phase 1: Design & Contracts

### Data Model (Phase 1 Output)

Will be generated in `data-model.md` after research phase.

### API Contracts (Phase 1 Output)

Not applicable - all changes are internal to dampen-dev crate, no public API surface changes (except removing unused FileDeleted error variant).

### Quickstart Guide (Phase 1 Output)

Will be generated in `quickstart.md` after design phase.

---

## Phase 2: Task Generation (NOT created by /speckit.plan)

Task generation will be performed by `/speckit.tasks` command, not by this plan.
