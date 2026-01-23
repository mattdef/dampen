# Implementation Plan: Window State Persistence

**Branch**: `001-window-persistence` | **Date**: 2026-01-24 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-window-persistence/spec.md`

## Summary

Implement opt-in window state persistence for Dampen applications, enabling windows to
restore their size, position, and maximized state across application restarts. The feature
integrates at the `dampen-dev` crate level (following the existing `theme_loader.rs` pattern)
with helpers for `main.rs` and optional macro integration for automated save-on-close.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.88
**Primary Dependencies**: `iced` 0.14+, `serde`/`serde_json`, `directories` 5.0, `tokio::fs`
**Storage**: JSON files in platform-specific config directories (XDG_CONFIG_HOME, AppData/Roaming, Application Support)
**Testing**: `cargo test --workspace`, unit tests with temp directories, integration tests
**Target Platform**: Linux (X11/Wayland), Windows, macOS
**Project Type**: Workspace with multiple crates (dampen-core, dampen-dev, dampen-macros, dampen-iced, dampen-cli)
**Performance Goals**: <100ms additional startup time for state loading, <10ms for state saving
**Constraints**: Non-blocking I/O, graceful fallback on errors, opt-in only
**Scale/Scope**: Single-instance window state per application identifier

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Compliance | Notes |
|-----------|------------|-------|
| I. Declarative-First | ✅ PASS | Window persistence is runtime config, not UI structure. XML remains source of truth for UI. |
| II. Type Safety Preservation | ✅ PASS | `WindowState` struct is fully typed with serde derives. No runtime type erasure. |
| III. Production Mode | ✅ PASS | Persistence works identically in interpreted and codegen modes. No XML parsing at runtime for this feature. |
| IV. Backend Abstraction | ✅ PASS | Feature resides in `dampen-dev`, not `dampen-core`. Core crate remains Iced-free. |
| V. Test-First Development | ✅ PASS | Contract tests will be written before implementation (see tasks.md). |

**Quality Gates**:
- Tests: All tests must pass with `cargo test --workspace`
- Linting: Zero warnings with `cargo clippy --workspace -- -D warnings`
- Formatting: Must pass `cargo fmt --all -- --check`
- Documentation: All public items will have rustdoc comments

**Error Handling**:
- All file I/O uses `Result<T, E>` with custom error types
- Errors include actionable messages (e.g., "Failed to write config to ~/.config/app/window.json: permission denied")
- Non-blocking: failures are logged as warnings, never panic

## Project Structure

### Documentation (this feature)

```text
specs/001-window-persistence/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (API contracts)
└── tasks.md             # Phase 2 output (created by /speckit.tasks)
```

### Source Code (repository root)

```text
crates/
├── dampen-core/         # NO CHANGES - maintains Iced-free guarantee
├── dampen-dev/
│   └── src/
│       ├── lib.rs                    # MODIFY: Export persistence module
│       ├── theme_loader.rs           # REFERENCE: Pattern for project root discovery
│       └── persistence/              # NEW: Window persistence module
│           ├── mod.rs                # Module exports
│           ├── window_state.rs       # WindowState struct and serde
│           ├── storage.rs            # Load/save operations with directories crate
│           └── monitor.rs            # Monitor validation utilities
├── dampen-macros/
│   └── src/
│       └── dampen_app.rs             # MODIFY: Add persistence attribute (optional)
├── dampen-iced/         # NO CHANGES - widget building unaffected
└── dampen-cli/
    └── templates/
        └── main.rs.template          # MODIFY: Add persistence boilerplate

examples/
├── counter/             # NO CHANGES - reference example stays minimal
├── hello-world/
│   └── src/main.rs                   # MODIFY: Add persistence integration
├── todo-app/
│   └── src/main.rs                   # MODIFY: Add persistence integration
└── theming/
    └── src/main.rs                   # MODIFY: Add persistence integration

tests/
└── integration/
    └── persistence_tests.rs          # NEW: Integration tests
```

**Structure Decision**: Window persistence is implemented as a new module in `dampen-dev`
(following the `theme_loader.rs` pattern). This maintains the Constitution's backend
abstraction principle - `dampen-core` remains Iced-free, and persistence is a dev/runtime
concern rather than core IR functionality.

## Complexity Tracking

No constitution violations requiring justification. The implementation follows established
patterns (similar to theme_loader.rs) and uses existing dependencies (`directories`, `serde_json`).
