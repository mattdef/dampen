# Implementation Plan: Harmonize Modes

**Branch**: `001-harmonize-modes` | **Date**: 2026-01-19 | **Spec**: [specs/001-harmonize-modes/spec.md](spec.md)
**Input**: Feature specification from `specs/001-harmonize-modes/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

This feature aims to achieve strict 100% parity between Dampen's Interpreted (Dev) and Codegen (Prod) modes. It involves standardizing XML attributes across all widgets, unifying layout behavior (width/height/padding support for all containers), and implementing state-aware styling (hover/focus/active) in the Codegen mode by statically generating `iced::StyleSheet` implementations. A visual regression testing infrastructure will be established to guarantee pixel-perfect consistency.

## Technical Context

**Language/Version**: Rust 2024 (MSRV 1.85)
**Primary Dependencies**: `iced` 0.14, `roxmltree` 0.19, `dampen-core` (IR), `dampen-macros` (codegen), `syn` 2.0, `quote` 1.0
**Storage**: N/A
**Testing**: `cargo test`, `insta` 1.0 (snapshots), `proptest` 1.0, `iced_renderer` (headless) for visual tests
**Target Platform**: Desktop (Linux, macOS, Windows)
**Project Type**: Rust Workspace (Library + CLI + Macros)
**Performance Goals**: XML parse time < 10ms, Codegen < 5s
**Constraints**: Zero `unsafe` code in generated output, maintain `dampen-core` backend agnosticism
**Scale/Scope**: Affects all standard widgets (~20) and core layout/styling logic

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Declarative-First**: Feature enforces a strict XML contract as the source of truth.
- [x] **Type Safety Preservation**: Codegen improvements rely on static Rust types (`impl StyleSheet`) rather than runtime erasure.
- [x] **Production Mode**: Explicitly enhances Codegen mode (Production) to match Dev capabilities.
- [x] **Backend Abstraction**: IR changes in `dampen-core` remain backend-agnostic; Iced-specifics confined to `dampen-iced`/`dampen-macros`.
- [x] **Test-First Development**: Visual regression harness is prioritized as Phase 1 foundation.

## Project Structure

### Documentation (this feature)

```text
specs/001-harmonize-modes/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/
├── dampen-core/         # IR updates for standardized layout/style
│   └── src/
│       ├── ir/
│       └── parser/
├── dampen-macros/       # Codegen updates for styling and layout wrapping
│   └── src/
│       ├── codegen/     # Style generation logic
│       └── ...
├── dampen-iced/         # Runtime interpreter updates to match standard
├── dampen-visual-tests/ # NEW: Visual regression test harness
└── dampen-cli/          # CLI support for new tests? (maybe)

tests/
├── visual/              # NEW: Visual regression test cases
└── ...
```

**Structure Decision**: Standard Rust workspace structure, adding a new crate `dampen-visual-tests` for the harness and a `tests/visual` directory for the actual test cases to keep them separate from unit/integration tests.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | | |
