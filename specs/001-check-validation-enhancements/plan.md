# Implementation Plan: Check Validation Enhancements

**Branch**: `001-check-validation-enhancements` | **Date**: 2026-01-08 | **Spec**: [spec.md](../spec.md)
**Input**: Feature specification from `/specs/001-check-validation-enhancements/spec.md`

## Summary

Enhance the `gravity check` command with rigorous XML validation including:
- Unknown attribute detection with Levenshtein distance suggestions (FR-001, FR-012)
- Handler registry validation against registered handlers (FR-002, FR-013)
- Binding validation against model fields (FR-003, FR-014)
- Cross-widget radio group validation (FR-004, FR-005)
- Theme property validation and circular dependency detection (FR-006, FR-007)
- Required attribute validation (FR-008)
- Strict mode for CI/CD quality gates (FR-009)
- Custom widget attribute configuration support (FR-017)

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV stable (per constitution)  
**Primary Dependencies**: gravity-core (parser, IR), serde_json (JSON handling), clap (CLI)  
**Storage**: JSON files for handler registry (`--handlers`) and model info (`--model`)  
**Testing**: cargo test (Rust test framework), proptest for property-based tests  
**Target Platform**: CLI tool (Linux, macOS, Windows - tier 1 per constitution)  
**Project Type**: CLI tool enhancement within gravity-cli crate  
**Performance Goals**: Validation of 100-500 widgets in under 1 second (SC-006)  
**Constraints**: Must not break existing validation; must collect all errors before reporting (FR-011)  
**Scale/Scope**: Backward compatible; adds new validations without removing existing ones

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Declarative-First | ✓ PASS | XML remains source of truth; validation enforces correctness |
| II. Type Safety Preservation | ✓ PASS | JSON schema validation; no runtime type erasure |
| III. Dual-Mode Architecture | ✓ PASS | CLI tool (dev mode validation); no production mode needed for check |
| IV. Backend Abstraction | ✓ PASS | gravity-cli has no backend deps; validation uses core only |
| V. Test-First Development | ⚠ REQUIRE TDD | All new features must have contract tests before implementation |

**Test Requirements**:
- Contract tests for each new validation type
- Integration tests for full check pipeline
- Property-based tests for Levenshtein distance algorithm
- Snapshot tests for error message formatting

## Project Structure

### Documentation (this feature)

```text
specs/001-check-validation-enhancements/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
crates/gravity-cli/src/commands/
├── check.rs             # Main check command (existing, to be enhanced)
├── mod.rs               # Command exports (existing)

# NEW: Validation modules to add
├── check/
│   ├── mod.rs           # Re-export all check validators
│   ├── attributes.rs    # Widget attribute schema and validation
│   ├── handlers.rs      # Handler registry loading and validation
│   ├── model.rs         # Model info loading and binding validation
│   ├── cross_widget.rs  # Radio group and cross-widget validation
│   ├── themes.rs        # Theme property and cycle detection
│   ├── suggestions.rs   # Levenshtein distance algorithm
│   └── errors.rs        # Enhanced error types

crates/gravity-cli/tests/
├── check_tests.rs       # Existing tests (to keep)
└── check/
    ├── attributes_tests.rs   # NEW: Attribute validation tests
    ├── handler_tests.rs      # NEW: Handler validation tests
    ├── binding_tests.rs      # NEW: Binding validation tests
    ├── radio_tests.rs        # NEW: Radio group tests
    ├── theme_tests.rs        # NEW: Theme validation tests
    └── integration_tests.rs  # NEW: Full pipeline tests
```

**Structure Decision**: Modular validation approach within gravity-cli. New check/ subdirectory separates concerns while keeping all validation logic co-located. Tests mirror source structure for discoverability.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations at this time. All technical choices align with constitution.

## Phase 0: Research

**Research Tasks** (from Technical Context unknowns):

1. **Levenshtein distance algorithm**: Find optimal implementation for typo suggestions (distance threshold <= 3)
2. **JSON schema for handler registry**: Define structure for handler metadata (name, param_type, returns_command)
3. **JSON schema for model info**: Define structure for field metadata (name, type, is_nested, children)
4. **Theme property validation**: Review existing theme schema to identify valid properties
5. **Widget attribute schema**: Document all valid attributes per widget type from IR

## Phase 1: Design & Contracts

**Deliverables**:
- `data-model.md`: HandlerRegistry, ModelInfo, WidgetAttributeSchema, ValidationError structures
- `quickstart.md`: Developer guide for using enhanced check command
- `/contracts/`: Error message formats, JSON schemas for registry/model files

## Phase 2: Tasks

Generated by `/speckit.tasks` command after Phase 1 design approval.
