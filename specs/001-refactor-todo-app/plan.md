# Implementation Plan: Refactor Todo-App to Match Iced Example

**Branch**: `001-refactor-todo-app` | **Date**: 2026-01-22 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-refactor-todo-app/spec.md`

## Summary

Refactor the todo-app example application to match the visual and functional design of the Iced "todos" example while preserving Dampen's dual-mode architecture (interpreted dev mode with hot-reload, codegen production mode). The refactor simplifies the architecture from multi-view to single-view, updates the UI design to match Iced's aesthetic (simple styles, emoji icons, centered layout), implements inline task editing, adds keyboard navigation, and ensures both execution modes work correctly.

## Technical Context

**Language/Version**: Rust 2024, MSRV 1.85 (from constitution)
**Primary Dependencies**: iced 0.14+, roxmltree 0.19+, uuid, serde, serde_json, dampen-core, dampen-iced, dampen-macros
**Storage**: N/A (in-memory only, no persistence - acceptable for demo app)
**Testing**: cargo test with proptest 1.0+, insta 1.0+ for snapshot testing
**Target Platform**: Desktop (Linux/Mac/Windows via Iced framework)
**Project Type**: Single application (examples/todo-app)
**Performance Goals**: Hot-reload <2s for UI changes, startup <1s in production mode, task operations <100ms
**Constraints**: Max 1000 tasks, window size 500x800, task descriptions max 1000 chars
**Scale/Scope**: Demo/example application demonstrating Dampen framework capabilities in both interpreted and codegen modes

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Pre-Design Evaluation

| Principle | Status | Notes |
|-----------|--------|-------|
| **I. Declarative-First** | ✅ PASS | UI will be defined in `window.dampen` XML file. All UI structure declared declaratively. |
| **II. Type Safety Preservation** | ✅ PASS | Model uses `#[derive(UiModel)]` with typed fields. Handlers have validated signatures. No runtime type erasure. |
| **III. Production Mode** | ✅ PASS | Application must work in both interpreted mode (dev) and codegen mode (production). Codegen path validated. |
| **IV. Backend Abstraction** | ✅ PASS | Example code only uses dampen-core types and traits. No direct Iced dependencies in example app logic. |
| **V. Test-First Development** | ⚠️ NEEDS RESEARCH | Need to determine appropriate test strategy for example application (unit tests? integration tests? manual validation?) |

### Quality Gates Pre-Check

| Gate | Status | Notes |
|------|--------|-------|
| Tests Pass | ⚠️ NEEDS RESEARCH | Test approach for example app unclear |
| Clippy Clean | ✅ PASS | All code must pass `cargo clippy -- -D warnings` |
| Formatted | ✅ PASS | All code must pass `cargo fmt -- --check` |
| Documented | ✅ PASS | Public APIs must have rustdoc comments |

### Technical Standards Compliance

| Standard | Status | Notes |
|----------|--------|-------|
| Rust 2024 / MSRV 1.85 | ✅ PASS | Project uses Rust 2024 edition |
| Iced 0.14+ | ✅ PASS | Specified in AGENTS.md |
| roxmltree 0.19+ | ✅ PASS | Required for XML parsing |
| serde/serde_json 1.0+ | ✅ PASS | Used for serialization |
| Zero Unsafe | ✅ PASS | No unsafe unless justified |
| Performance Budgets | ⚠️ NEEDS RESEARCH | XML parse <10ms, codegen <5s, memory <50MB - validate for this example |

### Re-evaluation After Design

*Completed after Phase 1 design phase*

| Principle | Status | Notes |
|-----------|--------|-------|
| **I. Declarative-First** | ✅ PASS | UI defined in `window.dampen` XML. All structure declared declaratively. See `data-model.md` for entity definitions. |
| **II. Type Safety Preservation** | ✅ PASS | Model uses `#[derive(UiModel)]` with typed fields (String, i64, Vec<Task>, Filter enum). No runtime type erasure. |
| **III. Production Mode** | ✅ PASS | Design supports both modes. Interpreted: hot-reload. Codegen: static generation. See `quickstart.md` for mode comparison. |
| **IV. Backend Abstraction** | ✅ PASS | Model uses dampen-core types only. No direct Iced dependencies in example logic. |
| **V. Test-First Development** | ✅ PASS | Hybrid test strategy adopted (unit tests for model, manual integration for UI). Appropriate for example app. |

### Quality Gates Post-Check

| Gate | Status | Notes |
|------|--------|-------|
| Tests Pass | ✅ PASS | Unit tests for model logic, manual integration testing for UI. Strategy documented in `research.md`. |
| Clippy Clean | ✅ PASS | All code must pass `cargo clippy -- -D warnings`. Code will be linted before commit. |
| Formatted | ✅ PASS | All code must pass `cargo fmt -- --check`. Code will be formatted before commit. |
| Documented | ✅ PASS | Public APIs will have rustdoc comments. Internal code documented where appropriate. |

### Technical Standards Compliance

| Standard | Status | Notes |
|----------|--------|-------|
| Rust 2024 / MSRV 1.85 | ✅ PASS | Project uses Rust 2024 edition with MSRV 1.85 |
| Iced 0.14+ | ✅ PASS | Specified in dependencies |
| roxmltree 0.19+ | ✅ PASS | Required for XML parsing |
| serde/serde_json 1.0+ | ✅ PASS | Used for model serialization |
| Zero Unsafe | ✅ PASS | No unsafe code planned or justified |
| Performance Budgets | ✅ PASS | All budgets achievable: XML <5ms, codegen <2s, memory <40MB, startup <1s, ops <10ms. Validated in `research.md`. |

### Final Gate Status

✅ **ALL GATES PASSED** - Ready to proceed to Phase 2 (task breakdown)

**Summary**:
- All 5 constitution principles pass
- All 4 quality gates pass
- All 6 technical standards pass
- No violations requiring justification
- Complexity tracking not needed (simplification, not complexity)

**Next Step**: Run `/speckit.tasks` to generate task breakdown for implementation

## Project Structure

### Documentation (this feature)

```text
specs/001-refactor-todo-app/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (likely N/A for UI example)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
examples/todo-app/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point with #[dampen_app] macro
│   ├── shared.rs            # SharedState struct (simplified)
│   └── ui/
│       ├── window.rs        # Model, handlers, and app logic
│       └── window.dampen    # UI definition (single view)

# REMOVED FILES (simplification):
# ├── src/ui/add_task.rs    # Removed - inline editing instead
# ├── src/ui/add_task.dampen # Removed - merged into window
# ├── src/ui/statistics.rs  # Removed - simplified counter
# └── src/ui/statistics.dampen # Removed - merged into window
```

**Structure Decision**: Single example application structure (examples/todo-app). The refactor simplifies from multi-view to single-view architecture, removing separate add_task and statistics views. All functionality consolidated into window view with inline editing and simple counter. This aligns with the Iced todos example's single-screen design while demonstrating Dampen's declarative UI capabilities.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No constitution violations requiring complexity justification. The refactor simplifies the architecture rather than adding complexity.
