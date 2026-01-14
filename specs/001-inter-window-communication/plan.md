# Implementation Plan: Inter-Window Communication

**Branch**: `001-inter-window-communication` | **Date**: 2026-01-14 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-inter-window-communication/spec.md`

## Summary

Enable inter-view communication in Dampen multi-view applications through a **SharedContext** that wraps user-defined shared state. Views can access shared data via `{shared.field}` bindings in XML and modify it through handlers that receive the shared context. The implementation uses `Arc<RwLock<S>>` for thread-safe concurrent access while maintaining 100% backward compatibility as an opt-in feature.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.85 (aligned with Dampen constitution)
**Primary Dependencies**: `iced` 0.14+, `roxmltree` 0.19+, `syn` 2.0+, `quote` 1.0+, `serde` 1.0+
**Storage**: In-memory only (`Arc<RwLock<S>>`); persistent storage is out of scope
**Testing**: `cargo test`, `proptest` 1.0+, `insta` 1.0+ (snapshots), TDD mandatory
**Target Platform**: Cross-platform desktop (Linux, macOS, Windows) via Iced
**Project Type**: Rust workspace with multiple crates
**Performance Goals**: Shared state access < 1ms, propagation to views < 16ms (60fps frame budget)
**Constraints**: Zero breaking changes for existing apps, <5% memory overhead, parity between interpreted/codegen modes
**Scale/Scope**: Supports applications with 2-10 views sharing state

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Justification |
|-----------|--------|---------------|
| I. Declarative-First | PASS | Shared bindings use `{shared.field}` syntax in XML; XML remains source of truth |
| II. Type Safety Preservation | PASS | `SharedContext<S>` is generic over user's type; no runtime type erasure |
| III. Production Mode | PASS | Codegen mode will compile shared bindings to static accessor code |
| IV. Backend Abstraction | PASS | `SharedContext` lives in `dampen-core` with no Iced dependency |
| V. Test-First Development | PASS | Contract tests will be written before implementation per TDD |

## Project Structure

### Documentation (this feature)

```text
specs/001-inter-window-communication/
├── plan.md              # This file
├── research.md          # Phase 0 output: design decisions, alternatives analysis
├── data-model.md        # Phase 1 output: SharedContext, HandlerEntry variants
├── quickstart.md        # Phase 1 output: developer guide for shared state
├── contracts/           # Phase 1 output: XML schema, API contracts
│   ├── shared-binding-schema.md
│   └── handler-api-contract.md
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
crates/
├── dampen-core/
│   ├── src/
│   │   ├── shared/               # NEW: SharedContext implementation
│   │   │   ├── mod.rs            # SharedContext<S> struct
│   │   │   └── tests.rs          # Unit tests
│   │   ├── handler/
│   │   │   └── mod.rs            # Extended with WithShared variants
│   │   ├── state/
│   │   │   └── mod.rs            # AppState extended with shared_context
│   │   └── lib.rs                # Export shared module
│   └── tests/
│       └── shared_context_tests.rs
│
├── dampen-macros/
│   ├── src/
│   │   ├── dampen_app.rs         # Extended with shared_model attribute
│   │   └── ui_model.rs           # No changes needed
│   └── tests/
│       └── shared_macro_tests.rs
│
├── dampen-iced/
│   ├── src/
│   │   ├── builder.rs            # Extended for {shared.} binding resolution
│   │   └── lib.rs
│   └── tests/
│       └── shared_binding_tests.rs
│
└── dampen-cli/
    └── src/                      # No changes needed

examples/
└── shared-state/                 # NEW: Complete example
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   ├── shared.rs             # SharedState definition
    │   └── ui/
    │       ├── mod.rs
    │       ├── window.rs
    │       ├── window.dampen
    │       ├── settings.rs
    │       └── settings.dampen
    └── tests/
        └── integration.rs

tests/
├── contract/
│   └── shared_state_contracts.rs  # Contract tests
├── integration/
│   └── shared_state_e2e.rs        # End-to-end tests
└── parity/
    └── shared_mode_parity.rs      # Interpreted vs codegen parity tests
```

**Structure Decision**: Follows existing Dampen workspace structure. New code is isolated in `shared/` module within `dampen-core`, with extensions to existing modules (`handler/`, `state/`) and backends (`dampen-iced`). This maintains the backend abstraction principle.

## Complexity Tracking

> **No violations identified** - all implementation choices align with constitution principles.

| Design Choice | Justification | Simpler Alternative Rejected |
|---------------|---------------|------------------------------|
| `Arc<RwLock<S>>` for shared state | Thread-safe concurrent access required for multi-view apps | `Rc<RefCell<S>>` rejected: not `Send + Sync`, can't share across threads |
| Generic `SharedContext<S>` | Preserves type safety per Principle II | `Box<dyn Any>` rejected: requires runtime downcasting, loses compile-time safety |
| Opt-in via `shared_model` attribute | 100% backward compatibility per FR-008 | Always-on rejected: would break existing apps |

## Implementation Phases

### Phase 1: Infrastructure Core (2-3 days)
- Create `SharedContext<S>` in `dampen-core/src/shared/`
- Add `WithShared` variants to `HandlerEntry` enum
- Extend `HandlerRegistry` with `register_with_shared()` and `dispatch_with_shared()`
- Extend `AppState<M>` with optional `shared_context` field

### Phase 2: Macro Extension (2-3 days)
- Parse `shared_model` attribute in `#[dampen_app]`
- Generate `SharedContext` initialization in `init()`
- Modify `update()` to pass shared context to handlers

### Phase 3: XML Bindings (2-3 days)
- Extend `DampenWidgetBuilder` to resolve `{shared.field}` bindings
- Add codegen support for shared bindings
- Validate `{shared.}` syntax in parser

### Phase 4: Example & Documentation (1-2 days)
- Create `examples/shared-state/` with working demo
- Update `docs/USAGE.md` and `docs/XML_SCHEMA.md`
- Add CHANGELOG entry for v0.2.4

### Phase 5: Tests & Polish (1-2 days)
- Hot-reload preservation tests
- Interpreted/codegen parity tests
- Performance benchmarks

## Dependencies

- **Internal**: `AppState<M>`, `HandlerRegistry`, `HandlerEntry`, `UiBindable` trait, `DampenWidgetBuilder`, `#[dampen_app]` macro
- **External**: None (uses existing dependencies)

## Risks and Mitigations

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| RwLock deadlock | High | Low | Use `try_read`/`try_write` with timeouts in debug mode |
| Performance degradation | Medium | Low | Benchmark access patterns; consider read-heavy optimization |
| Breaking changes | High | Very Low | Extensive backward compatibility tests; type aliases |

## Estimated Effort

| Phase | Duration | Hours |
|-------|----------|-------|
| Phase 1: Core | 2-3 days | 16-24h |
| Phase 2: Macro | 2-3 days | 16-24h |
| Phase 3: Bindings | 2-3 days | 16-24h |
| Phase 4: Docs | 1-2 days | 8-16h |
| Phase 5: Tests | 1-2 days | 8-16h |
| **Total** | **8-13 days** | **64-104h** |
