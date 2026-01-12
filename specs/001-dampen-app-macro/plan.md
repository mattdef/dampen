# Implementation Plan: Auto-Discovery Multi-View Application with #[dampen_app] Macro

**Branch**: `001-dampen-app-macro` | **Date**: 2026-01-12 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-dampen-app-macro/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement a procedural macro `#[dampen_app]` that automatically discovers `.dampen` UI files in the `src/ui` directory (recursively) and generates all boilerplate code for multi-view Dampen applications. This feature eliminates 85% of manual code (500 lines → <100 lines for a 20-view application) by auto-generating enums, struct fields, initialization, routing, view rendering, hot-reload subscription, and handler dispatch logic.

**Technical Approach**: Compile-time file discovery using `walkdir` crate within a proc-macro, parsing macro attributes with `syn`, and generating Rust code via `quote`. The macro runs during compilation, scanning the specified directory for `.dampen` files, validating naming conventions, and emitting type-safe code that integrates with existing `AppState<T>`, `dampen-dev` hot-reload, and Iced message handling.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.85 (aligned with Dampen constitution)  
**Primary Dependencies**: 
- Existing: `syn` 2.0+, `quote` 2.0+, `proc-macro2` 2.0+ (already in `dampen-macros`)
- New: `walkdir` 2.5+ (recursive directory traversal at compile time)
- Dev-only new: `trybuild` 1.0+ (compile-fail tests for macro error messages)
**Storage**: N/A (compile-time code generation, no runtime persistence)  
**Testing**: 
- Existing: `insta` 1.0+ (snapshot tests for generated code)
- Unit tests with `cargo test -p dampen-macros`
- Integration tests using `trybuild` for compile-fail scenarios
- Migration test with `widget-showcase` example
**Target Platform**: Rust proc-macro compilation (runs during `cargo build` on developer's machine)  
**Project Type**: Multi-crate workspace (adding functionality to existing `dampen-macros` crate)  
**Performance Goals**: 
- File discovery overhead < 200ms for 20 views (per SC-002)
- Zero runtime overhead (generated code identical to hand-written) (per SC-007)
- Compilation time for macro expansion < 500ms (reasonable for proc-macro)
**Constraints**: 
- MUST preserve type safety (no `Box<dyn>` or runtime type erasure) (Constitution Principle II)
- MUST work in both dev mode (interpreted) and production mode (codegen) (Constitution Principle III)
- MUST provide compile-time errors with file paths and suggestions (per FR-012, FR-013, FR-014)
**Scale/Scope**: 
- Expected: Applications with 3-20 views (primary use case)
- Edge case: Up to 50 views (should still compile reasonably)
- Typical project: `widget-showcase` with 20 views (~500 lines manual → <100 lines with macro)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Principle I: Declarative-First
✅ **COMPLIANT**: The macro discovers `.dampen` XML files (declarative UI definitions) and generates code to load them. Does not change the declarative-first paradigm - reinforces it by making multi-view XML apps easier.

### Principle II: Type Safety Preservation
✅ **COMPLIANT**: Generated code uses typed `AppState<ui::view_name::Model>` fields (no type erasure). Each view's Model type is statically known at compile time. The macro generates strongly-typed enums and match statements.

### Principle III: Production Mode
✅ **COMPLIANT**: The macro works with both dev mode (interpreted XML) and production mode (codegen). In production, the generated `AppState` fields will use pre-compiled view definitions. The macro itself is compile-time only (zero runtime cost).

### Principle IV: Backend Abstraction
✅ **COMPLIANT**: The macro lives in `dampen-macros` and generates code that uses `AppState<T>` (from `dampen-core`) and message types defined by the user. No direct Iced dependency in the macro. Generated code calls user-provided `view()` and `update()` methods that delegate to `AppState`, maintaining abstraction.

### Principle V: Test-First Development
⚠️ **REQUIRES TDD PLAN**: Must write tests before implementation. Plan includes:
1. Contract tests for file discovery (Phase 0)
2. Snapshot tests for code generation (Phase 1)
3. Compile-fail tests for error messages (Phase 1)
4. Integration test with `widget-showcase` migration (Phase 2)

**TDD Commitment**: All tests will be written first and must fail before implementation begins (red-green-refactor).

### Quality Gates Check
✅ **PLANNING**: Will ensure all gates pass:
- Tests: Unit tests for discovery, snapshot tests for generation, trybuild for errors
- Linting: `cargo clippy --workspace -- -D warnings` (zero warnings)
- Formatting: `cargo fmt --all -- --check` (properly formatted)
- Documentation: Rustdoc for `#[dampen_app]` macro and all public discovery types

**GATE STATUS**: ✅ PASSED - All principles compliant, TDD plan in place

## Project Structure

### Documentation (this feature)

```text
specs/001-dampen-app-macro/
├── spec.md              # Feature specification (DONE)
├── plan.md              # This file (IN PROGRESS)
├── research.md          # Phase 0 output (NEXT)
├── data-model.md        # Phase 1 output (PENDING)
├── quickstart.md        # Phase 1 output (PENDING)
├── contracts/           # Phase 1 output (PENDING)
│   └── macro-api.md     # Macro attribute API contract
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/dampen-macros/
├── src/
│   ├── lib.rs                # Export #[dampen_app] macro (MODIFY)
│   ├── ui_model.rs          # Existing #[derive(UiModel)] (NO CHANGE)
│   ├── ui_loader.rs         # Existing #[dampen_ui] (NO CHANGE)
│   ├── dampen_app.rs        # NEW: Main macro implementation
│   └── discovery.rs         # NEW: File discovery logic
├── tests/
│   ├── dampen_app_tests.rs  # NEW: Unit tests for macro
│   ├── fixtures/            # NEW: Test fixtures
│   │   ├── multi_view/      # Test project with multiple views
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs
│   │   │       └── ui/
│   │   │           ├── view1.dampen
│   │   │           ├── view1.rs
│   │   │           ├── view2.dampen
│   │   │           └── view2.rs
│   │   ├── nested_views/    # Test nested directory structure
│   │   └── edge_cases/      # Test empty dirs, missing files, etc.
│   └── ui/                  # NEW: trybuild compile-fail tests
│       ├── missing_rs_file.rs
│       ├── invalid_ui_dir.rs
│       └── naming_conflict.rs
└── Cargo.toml              # Add walkdir, trybuild dependencies (MODIFY)

crates/dampen-dev/
└── src/
    └── (NO CHANGES - macro integrates with existing FileEvent)

examples/widget-showcase/
├── src/
│   ├── main.rs             # MIGRATE: Apply #[dampen_app] macro
│   └── ui/                 # (20 existing .dampen and .rs files - NO CHANGE)
└── Cargo.toml              # (NO CHANGE)

tests/integration/
└── macro_integration_tests.rs  # NEW: End-to-end macro test
```

**Structure Decision**: Single workspace with modifications to existing `dampen-macros` crate. The macro is a natural extension of existing proc macros (`#[derive(UiModel)]`, `#[dampen_ui]`) and follows the same patterns. New files are isolated in `dampen_app.rs` and `discovery.rs` to avoid disrupting existing functionality.

## Complexity Tracking

> No constitution violations - this section intentionally left blank.

All constitution principles are compliant. The macro adds a new abstraction but it's justified:
- **Problem**: Manual boilerplate scales linearly with view count (500 lines for 20 views)
- **Why needed**: 85% code reduction, eliminates error-prone manual routing
- **Simpler alternatives rejected**: 
  - Manual code: Rejected due to maintenance burden and error-proneness
  - Runtime discovery: Rejected due to Constitution Principle III (production mode requires compile-time)
  - Code generation script: Rejected because proc-macro integrates better with Rust tooling and provides better error messages

## Phase 0: Research & Technology Selection

**Objective**: Resolve all technical unknowns and establish patterns for implementation.

### Research Tasks

**R1: Proc-Macro File System Access Patterns**
- **Question**: How do proc-macros access the file system at compile time? Are there any restrictions or best practices?
- **Investigation**: 
  - Research `std::fs` usage in proc-macro context
  - Investigate how other macros do file discovery (e.g., `include_str!`, build scripts)
  - Determine if `CARGO_MANIFEST_DIR` environment variable is accessible
  - Check for any sandboxing or security concerns
- **Deliverable**: Pattern for safe file system access in proc-macros

**R2: Walkdir Integration with Proc-Macros**
- **Question**: Does `walkdir` work correctly in proc-macro context? Any performance concerns?
- **Investigation**:
  - Test `walkdir` in a simple proc-macro
  - Measure overhead for 20 files (must be < 200ms per SC-002)
  - Research alternative directory traversal crates if needed
  - Determine caching strategy (if any) for repeated macro invocations
- **Deliverable**: Validated directory traversal approach with performance characteristics

**R3: Syn Attribute Parsing Patterns**
- **Question**: What's the best pattern for parsing complex macro attributes like `#[dampen_app(ui_dir = "src/ui", exclude = ["debug"])]`?
- **Investigation**:
  - Review `syn` documentation for attribute parsing
  - Study existing Dampen macros (`#[derive(UiModel)]`) for patterns
  - Research best practices for required vs optional attributes
  - Determine error reporting strategy for malformed attributes
- **Deliverable**: Attribute parsing pattern with clear error messages

**R4: Glob Pattern Matching for Exclusions**
- **Question**: Should we implement glob matching ourselves or use a library? Which library?
- **Investigation**:
  - Research `globset` crate (used by ripgrep)
  - Research `glob` crate (simpler but less features)
  - Consider hand-rolled simple matching (just `*` and literal strings)
  - Evaluate compile-time performance impact
- **Deliverable**: Exclusion pattern matching strategy

**R5: Quote Code Generation Best Practices**
- **Question**: What's the best way to generate large amounts of code with `quote!`? How to maintain readability?
- **Investigation**:
  - Review existing `dampen-macros` code generation patterns
  - Research `quote!` best practices for generating enums, structs, impl blocks
  - Determine strategy for formatting generated code
  - Investigate `prettyplease` crate for code formatting
- **Deliverable**: Code generation patterns with examples

**R6: Proc-Macro Error Reporting**
- **Question**: How to emit compile errors with file paths and suggestions (per FR-012, FR-013, FR-014)?
- **Investigation**:
  - Research `syn::Error` and `proc_macro2::Span` for error reporting
  - Study how to include file paths in error messages
  - Investigate `proc_macro_error` crate for enhanced error reporting
  - Determine format for multi-line error messages with suggestions
- **Deliverable**: Error reporting pattern with examples

**R7: Trybuild Testing Strategy**
- **Question**: How to set up compile-fail tests for macro error messages?
- **Investigation**:
  - Research `trybuild` setup and usage
  - Determine test organization (separate `tests/ui/` directory)
  - Investigate how to test error message content (not just failure)
  - Review `trybuild` best practices from popular macros (serde, tokio)
- **Deliverable**: Trybuild testing setup pattern

**R8: Integration with Existing AppState**
- **Question**: How should generated code integrate with the existing `AppState<T>` pattern?
- **Investigation**:
  - Review `dampen-core/src/state/mod.rs` to understand AppState API
  - Determine how to generate `AppState::new()` calls
  - Understand how to pass `document()` from `#[dampen_ui]` to generated code
  - Research how to handle optional hot-reload integration
- **Deliverable**: AppState integration pattern with code examples

### Research Output

All findings will be consolidated in `research.md` with the following sections:
1. **File System Access in Proc-Macros** (R1)
2. **Directory Traversal with Walkdir** (R2)
3. **Attribute Parsing with Syn** (R3)
4. **Exclusion Pattern Matching** (R4)
5. **Code Generation with Quote** (R5)
6. **Error Reporting Strategy** (R6)
7. **Compile-Fail Testing with Trybuild** (R7)
8. **AppState Integration** (R8)

Each section will include:
- **Decision**: What was chosen
- **Rationale**: Why chosen
- **Alternatives Considered**: What else was evaluated
- **Implementation Notes**: Key details for Phase 1

**GATE**: Research complete → Proceed to Phase 1

## Phase 1: Design & Contracts

**Prerequisites**: `research.md` complete

### Data Model Design

**Objective**: Define the internal data structures used by the macro during code generation.

**Output**: `data-model.md` with the following entities:

#### ViewInfo Struct
Represents a discovered view during file scanning.

**Fields**:
- `view_name: String` - Snake_case name from filename (e.g., "text_input")
- `variant_name: String` - PascalCase enum variant (e.g., "TextInput")
- `field_name: String` - Struct field name (e.g., "text_input_state")
- `module_path: String` - Rust module path (e.g., "ui::widgets::text_input")
- `dampen_file: PathBuf` - Path to .dampen file
- `rs_file: PathBuf` - Path to .rs file

**Validation Rules**:
- `view_name` must be valid Rust identifier (no spaces, starts with letter)
- `variant_name` must be unique across all discovered views
- `rs_file` must exist (enforced by FR-002)

**Relationships**:
- One ViewInfo per discovered .dampen file
- Collection of ViewInfo used to generate CurrentView enum, struct fields, and methods

#### MacroAttributes Struct
Represents parsed attributes from `#[dampen_app(...)]`.

**Fields**:
- `ui_dir: String` - Required: directory to scan (e.g., "src/ui")
- `message_type: String` - Required: user's Message enum name
- `handler_variant: String` - Required: Message variant for HandlerMessage
- `hot_reload_variant: Option<String>` - Optional: Message variant for FileEvent
- `dismiss_error_variant: Option<String>` - Optional: Message variant for error dismissal
- `exclude: Vec<String>` - Optional: Glob patterns to exclude

**Validation Rules**:
- `ui_dir` must be a valid path and must exist (FR-013)
- `message_type`, `handler_variant` must be valid Rust identifiers
- `exclude` patterns must be valid glob patterns

**Relationships**:
- One MacroAttributes per `#[dampen_app]` invocation
- Used to configure ViewInfo discovery and code generation

### API Contracts

**Objective**: Define the public API surface of the `#[dampen_app]` macro.

**Output**: `contracts/macro-api.md` with the following contract:

#### Macro Signature

```rust
#[dampen_app(
    ui_dir = "src/ui",                    // Required: String literal
    message_type = "Message",             // Required: Identifier
    handler_variant = "Handler",          // Required: Identifier
    hot_reload_variant = "HotReload",     // Optional: Identifier
    dismiss_error_variant = "DismissError", // Optional: Identifier
    exclude = ["debug_view", "experimental/*"] // Optional: Array of string literals
)]
```

#### Generated Code Contract

**Input**: Struct definition with `#[dampen_app]` attribute
```rust
#[dampen_app(/* attributes */)]
struct MyApp;
```

**Output**: Expanded code with:

1. **CurrentView Enum**
```rust
#[derive(Clone, Debug, PartialEq)]
enum CurrentView {
    View1,
    View2,
    // ... one variant per discovered .dampen file
}
```

2. **App Struct Fields**
```rust
struct MyApp {
    current_view: CurrentView,
    view1_state: AppState<ui::view1::Model>,
    view2_state: AppState<ui::view2::Model>,
    // ... one field per discovered view
    #[cfg(debug_assertions)]
    error_overlay: Option<ErrorOverlay>,
}
```

3. **Generated Methods**
- `fn init() -> (Self, Task<Message>)`
- `fn new() -> (Self, Task<Message>)`
- `fn update(app: &mut Self, message: Message) -> Task<Message>`
- `fn view(app: &Self) -> Element<'_, Message>`
- `fn subscription(app: &Self) -> Subscription<Message>` (if `hot_reload_variant` specified)
- `fn dispatch_handler(app: &mut Self, handler: &str, value: Option<String>)`

**Error Cases**:
- Missing `ui_dir`: Compile error "missing required attribute 'ui_dir'"
- Invalid `ui_dir`: Compile error "UI directory '{path}' does not exist"
- Missing .rs file: Compile error "No matching Rust module found for '{dampen_file}'"
- Naming conflict: Compile error "View naming conflict: '{name}' found in multiple locations"

### Quickstart Guide

**Objective**: Provide a minimal working example for users.

**Output**: `quickstart.md` with:

1. **Installation** (no changes needed - macro is in `dampen-macros`)
2. **Basic Example**: 3-view app with navigation
3. **File Structure**: Expected directory layout
4. **Common Pitfalls**: Missing .rs file, naming conflicts, etc.
5. **Migration Guide**: Converting existing manual code to macro

### Agent Context Update

After Phase 1 completion, run:
```bash
.specify/scripts/bash/update-agent-context.sh opencode
```

This will add to `AGENTS.md`:
```
- Rust Edition 2024, MSRV 1.85 + walkdir 2.5 (directory traversal), trybuild 1.0 (compile-fail tests) (001-dampen-app-macro)
- N/A (compile-time code generation, no runtime persistence) (001-dampen-app-macro)
```

**GATE**: Design artifacts complete → Proceed to Phase 2 (tasks breakdown via `/speckit.tasks`)

## Phase 2: Implementation Tasks (Generated via /speckit.tasks)

**Note**: This section is a preview. Detailed tasks will be generated by the `/speckit.tasks` command after Phase 1 completion.

### High-Level Task Groups

**TG-1: File Discovery Infrastructure**
- Implement `discovery.rs` with `walkdir` integration
- Add `ViewInfo` struct and discovery logic
- Write unit tests for discovery (TDD)
- Handle nested directories correctly

**TG-2: Attribute Parsing**
- Implement `MacroAttributes` parsing with `syn`
- Validate required/optional attributes
- Write tests for malformed attributes (TDD)
- Generate clear error messages for parsing failures

**TG-3: Code Generation**
- Generate `CurrentView` enum with `quote!`
- Generate app struct with `AppState` fields
- Generate `init()`, `new()`, `update()`, `view()`, `subscription()`, `dispatch_handler()`
- Write snapshot tests for generated code (TDD)

**TG-4: Error Handling**
- Implement error reporting for missing .rs files
- Implement error reporting for invalid `ui_dir`
- Implement error reporting for naming conflicts
- Write trybuild compile-fail tests (TDD)

**TG-5: Exclusion Patterns**
- Implement glob pattern matching for `exclude` attribute
- Filter discovered views based on exclusion patterns
- Write tests for exclusion logic (TDD)

**TG-6: Integration & Migration**
- Migrate `widget-showcase` example to use macro
- Verify all 20 views work identically
- Verify hot-reload works correctly
- Measure performance (discovery < 200ms)
- Write integration tests

**TG-7: Documentation & Polish**
- Write rustdoc for `#[dampen_app]` macro
- Update `docs/USAGE.md` with macro usage
- Write migration guide in `docs/migration/multi-view-macro.md`
- Create cookbook examples

### Task Dependencies

```
TG-1 (Discovery) → TG-2 (Parsing) → TG-3 (Generation)
                                   ↓
                          TG-4 (Error Handling)
                                   ↓
                          TG-5 (Exclusions)
                                   ↓
                          TG-6 (Integration)
                                   ↓
                          TG-7 (Documentation)
```

### Acceptance Criteria Mapping

Each task group maps to specific success criteria and functional requirements:

**TG-1 → FR-001, FR-010, FR-016, SC-002** (recursive scan, nested modules, alphabetical sorting, performance)

**TG-2 → FR-017, FR-018** (required/optional attributes)

**TG-3 → FR-003, FR-004, FR-005, FR-006, FR-007, FR-008, FR-009, SC-001, SC-003, SC-004** (code generation, boilerplate reduction)

**TG-4 → FR-012, FR-013, FR-014, FR-015, SC-006** (error messages with file paths and suggestions)

**TG-5 → FR-011** (exclusion patterns)

**TG-6 → FR-019, FR-020, SC-005, SC-007, SC-008** (hot-reload integration, performance validation, migration)

**TG-7 → Documentation requirements** (not in FRs but critical for adoption)

## Testing Strategy

### Test Pyramid

```
                    E2E
                 /       \
            Integration
          /               \
        Snapshot
      /                     \
   Unit                   Compile-Fail
```

**Unit Tests** (30% of tests):
- File discovery logic (fixtures with known directory structures)
- Attribute parsing (valid/invalid inputs)
- ViewInfo struct validation
- Exclusion pattern matching

**Snapshot Tests** (30% of tests):
- Generated `CurrentView` enum (compare with expected output)
- Generated struct fields (compare with expected output)
- Generated method implementations (compare with expected output)
- Use `insta` crate for snapshot management

**Compile-Fail Tests** (20% of tests):
- Missing .rs file error message
- Invalid `ui_dir` error message
- Naming conflict error message
- Malformed attributes error message
- Use `trybuild` crate for compile-fail testing

**Integration Tests** (15% of tests):
- Full macro expansion with 3-5 views
- Hot-reload subscription generation (when `hot_reload_variant` specified)
- Handler dispatch logic
- View switching logic

**E2E Test** (5% of tests):
- Migrate `widget-showcase` (20 views) to use macro
- Verify all views render correctly
- Verify hot-reload works
- Measure compilation time (< 200ms discovery overhead)

### TDD Workflow

For each task group:

1. **RED**: Write failing tests first
   - Unit tests for discovery
   - Snapshot tests for generation
   - Compile-fail tests for errors

2. **GREEN**: Implement minimal code to pass tests
   - Implement discovery logic
   - Implement code generation
   - Implement error handling

3. **REFACTOR**: Improve code quality
   - Extract common patterns
   - Improve error messages
   - Optimize performance

### Performance Testing

**Metrics to measure**:
- File discovery time for 20 views (must be < 200ms per SC-002)
- Macro expansion time (target < 500ms)
- Generated code size (compare to manual equivalent)
- Runtime performance (must be identical to manual code per SC-007)

**Benchmarking approach**:
- Use `criterion` crate for microbenchmarks (if needed)
- Measure in CI to detect regressions
- Compare generated assembly to manual code (zero-cost abstraction validation)

## Risk Mitigation

### Risk 1: Proc-Macro File System Access Issues
**Probability**: Low  
**Impact**: High (feature unusable)  
**Mitigation**: R1 research task validates file system access patterns. Fallback: Use build script instead of proc-macro.

### Risk 2: Performance Overhead
**Probability**: Medium  
**Impact**: Medium (compilation slowdown)  
**Mitigation**: R2 research task measures `walkdir` performance. Caching strategy if needed. Target < 200ms already generous.

### Risk 3: Complex Error Messages
**Probability**: Medium  
**Impact**: Medium (poor UX)  
**Mitigation**: R6 research task establishes error reporting patterns. Trybuild tests validate error message quality.

### Risk 4: Breaking Existing Code
**Probability**: Low  
**Impact**: High (breaks existing apps)  
**Mitigation**: Macro is opt-in. No changes to existing macros. Extensive integration tests.

### Risk 5: Nested Directory Edge Cases
**Probability**: Medium  
**Impact**: Low (affects advanced users)  
**Mitigation**: R2 research task tests nested directories. Unit tests cover edge cases. Clear documentation on limitations.

## Success Metrics

From spec.md success criteria:

- **SC-001**: 500 lines → <100 lines for 20-view app ✅ *Validated by widget-showcase migration*
- **SC-002**: Discovery < 200ms for 20 views ✅ *Measured in performance tests*
- **SC-003**: Zero wiring code for new views ✅ *Validated by quickstart example*
- **SC-004**: Zero manual routing logic ✅ *Validated by generated `update()` inspection*
- **SC-005**: Hot-reload < 500ms latency ✅ *Measured in integration tests*
- **SC-006**: 100% errors include paths and suggestions ✅ *Validated by trybuild tests*
- **SC-007**: Zero runtime overhead ✅ *Validated by assembly comparison*
- **SC-008**: Successful widget-showcase migration ✅ *Validated by E2E test*

## Next Steps

1. ✅ **Phase 0**: Generate `research.md` (resolves R1-R8 research tasks)
2. ⏳ **Phase 1**: Generate `data-model.md`, `contracts/macro-api.md`, `quickstart.md`
3. ⏳ **Phase 1**: Run agent context update script
4. ⏳ **Phase 2**: Run `/speckit.tasks` to generate detailed task breakdown
5. ⏳ **Implementation**: Execute tasks following TDD workflow
6. ⏳ **Validation**: Run all tests, verify success criteria
7. ⏳ **Documentation**: Complete rustdoc, usage guide, migration guide
8. ⏳ **Merge**: PR review and merge to main
