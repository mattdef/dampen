# Implementation Plan: Production Mode with Static Code Generation

**Branch**: `008-prod-codegen` | **Date**: 2026-01-08 | **Spec**: [link](../spec.md)
**Input**: Feature specification from `/specs/008-prod-codegen/spec.md`

## Summary

Implement production build mode for the Gravity framework that generates static Rust code at compile time, eliminating runtime XML parsing overhead. This enables high-performance production deployments while maintaining development mode for rapid iteration.

**Technical Approach**: Extend existing `#[ui_handler]` macro to emit handler metadata, create `build.rs` template for code generation, and integrate with `gravity build --prod` CLI command.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.75+  
**Primary Dependencies**: roxmltree (XML parsing), proc-macro2/syn/quote (macro generation), Cargo build.rs mechanism  
**Storage**: N/A (code generation, no runtime persistence)  
**Testing**: cargo test, insta (snapshot testing), proptest (property-based)  
**Target Platform**: Desktop (Windows, Linux, macOS)  
**Project Type**: CLI tool + Rust library crates  
**Performance Goals**: Build time <200% of dev mode, startup time 50%+ faster in production  
**Constraints**: Must maintain backward compatibility with `#[gravity_ui]` development mode  
**Scale/Scope**: 7 example projects to migrate, all existing widget types supported

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Declarative-First | ✅ PASS | XML remains source of truth; static code generation preserves semantics |
| II. Type Safety Preservation | ✅ PASS | Handler signatures type-checked at compile time; no type erasure |
| III. Dual-Mode Architecture | ✅ PASS | Production mode complements development mode; identical behavior |
| IV. Backend Abstraction | ✅ PASS | Core crate remains backend-agnostic; Iced is reference only |
| V. Test-First Development | ✅ PASS | Contract tests defined for code generation output (see research.md) |

**GATE STATUS**: PASS - All principles satisfied. Phase 0 complete.

## Phase 0: Research Tasks

### Unknowns Resolved

1. **Handler Metadata Extraction**: Rust module emission pattern (research.md, Decision 1)
2. **build.rs Code Generation**: Single file generation approach (research.md, Decision 2)
3. **Circular Dependency Detection**: Static analysis with DFS (research.md, Decision 3)

### Key Design Decisions

- Handler metadata: Static Rust module emitted by proc macro
- Code generation: Single ui_generated.rs file per project
- Error format: Structured messages with location and suggestions
- Code structure: Iced Widget Builder pattern for consistency

## Phase 1: Design Artifacts

### Deliverables

- [x] `research.md` - Technical decisions and patterns
- [x] `data-model.md` - HandlerInfo, HandlerSignatureType structures
- [x] `quickstart.md` - Developer guide for production mode
- [x] Agent context updated via `.specify/scripts/bash/update-agent-context.sh`
- [ ] `contracts/xml-schema.md` - Not needed (schema unchanged)
