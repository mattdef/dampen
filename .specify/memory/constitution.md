<!--
SYNC IMPACT REPORT
==================
Version Change: N/A (initial) -> 1.0.0
Modified Principles: N/A (initial constitution)
Added Sections:
  - Core Principles (5 principles)
  - Technical Constraints
  - Development Workflow
  - Governance
Removed Sections: N/A
Templates Requiring Updates:
  - .specify/templates/plan-template.md: OK (generic, no changes needed)
  - .specify/templates/spec-template.md: OK (generic, no changes needed)
  - .specify/templates/tasks-template.md: OK (generic, no changes needed)
Follow-up TODOs: None
-->

# Gravity Declarative Framework Constitution

## Core Principles

### I. Declarative-First

All user interface definitions MUST originate from declarative XML/markup files. The markup
serves as the single source of truth for UI structure, layout, and widget hierarchy.

- Markup files MUST be parseable without runtime Rust context
- The AST/IR representation MUST capture complete UI semantics
- Imperative Rust code defines behavior (handlers, state), not structure
- Separation of concerns: visual structure (XML) vs application logic (Rust)

**Rationale**: Declarative definitions enable tooling (visual editors, linters), simplify
reasoning about UI state, and support the dual-mode architecture requirement.

### II. Type Safety Preservation

Rust's strong type system MUST be preserved across the XML-to-Rust boundary. No runtime
type erasure or `Any`-based dispatch for core message and state types.

- Messages defined in XML MUST map to concrete Rust enum variants
- State bindings MUST be verified at compile-time in production mode
- Handler signatures MUST be type-checked against declared events
- Generic parameters and lifetimes MUST be expressible where needed

**Rationale**: Type safety is Rust's primary value proposition. Sacrificing it would
eliminate the advantage over dynamic UI frameworks.

### III. Dual-Mode Architecture

The framework MUST support two operational modes with identical semantic behavior:

**Development Mode**:
- Runtime interpretation of XML files
- Hot-reload on file changes (< 500ms target latency)
- No recompilation required for UI-only changes
- Diagnostic overlay for binding errors

**Production Mode**:
- Static Rust code generation via proc macros or build.rs
- Zero runtime XML parsing overhead
- Full compile-time verification of all bindings
- Generated code MUST be human-readable for debugging

**Rationale**: Fast iteration during development is essential for UI work. Production
deployments require maximum performance and compile-time guarantees.

### IV. Backend Abstraction

The framework MUST abstract over rendering backends through a trait-based interface.
Iced is the initial and primary target, but the architecture MUST NOT hardcode Iced types
in the core IR or public API.

- Core crate MUST NOT depend on any specific backend crate
- Backend trait MUST define widget primitives, layout, and event mapping
- Iced backend is the reference implementation
- Adding a new backend MUST NOT require modifying core crates

**Rationale**: While Iced is the initial target, backend abstraction future-proofs the
framework and enables community contributions for alternative renderers.

### V. Test-First Development

All features MUST follow TDD methodology. Tests define the contract before implementation.

- Contract tests for XML parsing and IR generation
- Integration tests for the full XML-to-rendered-UI pipeline
- Property-based tests for parser edge cases
- Hot-reload behavior MUST be testable without manual intervention
- Generated code MUST be deterministic for snapshot testing

**Rationale**: A declarative framework has well-defined input/output contracts that are
naturally suited to test-first development. Parser correctness is critical.

## Technical Constraints

### Language and Toolchain

- **Language**: Rust Edition 2024 or later
- **MSRV**: Stable Rust (no nightly-only features in public API)
- **Backend**: Iced 0.14+ as reference implementation
- **Platforms**: Windows, Linux, macOS (tier 1 support)

### Crate Architecture

The framework MUST be organized as a workspace with these crates:

| Crate | Purpose | Dependencies |
|-------|---------|--------------|
| `gravity-core` | Parser, AST, IR, trait definitions | None (backend-agnostic) |
| `gravity-macros` | Proc macros for production mode | `gravity-core` |
| `gravity-runtime` | Hot-reload interpreter, file watcher | `gravity-core` |
| `gravity-iced` | Iced backend implementation | `gravity-core`, `iced` |
| `gravity-cli` | Developer CLI (dev/build commands) | All above |

### Performance Budgets

- XML parse time: < 10ms for 1000-widget file
- Hot-reload latency: < 500ms from file save to UI update
- Production code generation: < 5s for typical application
- Runtime memory overhead (dev mode): < 50MB baseline

### API Stability

- Public API changes require CHANGELOG entry
- Breaking changes require major version bump
- Internal modules (`_internal`, `__private`) carry no stability guarantee

## Development Workflow

### Feature Development

1. Features MUST start with a specification in `/specs/`
2. Parser changes MUST include grammar documentation
3. New widgets MUST be implemented in core IR before any backend
4. Backend implementations MUST pass the core test suite

### Code Review Requirements

- All PRs MUST pass CI (tests, clippy, rustfmt)
- Parser changes require two approvals
- Public API changes require documentation updates
- Generated code changes require snapshot review

### Documentation Standards

- All public items MUST have rustdoc comments
- Examples MUST compile and run (`cargo test --doc`)
- XML schema MUST be documented with examples
- Error messages MUST be actionable

## Governance

This constitution supersedes all other development practices for the Gravity project.
Amendments require:

1. Written proposal with rationale
2. Impact analysis on existing code
3. Migration plan for breaking changes
4. Documentation update plan
5. Version bump according to semantic versioning

All code reviews MUST verify compliance with these principles. Deviations require explicit
justification documented in the PR description and approved by maintainers.

**Version**: 1.0.0 | **Ratified**: 2025-12-30 | **Last Amended**: 2025-12-30
