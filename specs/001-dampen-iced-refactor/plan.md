# Implementation Plan: dampen-iced Crate Refactoring

**Branch**: `001-dampen-iced-refactor` | **Date**: 2026-01-21 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `/specs/001-dampen-iced-refactor/spec.md`

## Summary

Refactor the dampen-iced crate to eliminate code duplication (~370 lines), remove legacy unused code (128 lines), implement missing state-aware styling for 3 widgets, and optimize performance for large UIs (1000+ widgets). This is a code quality and maintainability improvement that preserves all existing functionality while improving developer experience and runtime performance.

**Primary Goal**: Reduce technical debt and improve codebase maintainability from 3/5 to 4/5+ rating.

**Technical Approach**: Incremental refactoring using test-driven validation at each step, extracting three major duplication patterns into generic helpers, removing legacy IcedBackend trait, and optimizing memory allocations via Rc wrappers.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.85  
**Primary Dependencies**: 
- `iced` 0.14+ (UI framework)
- `dampen-core` (workspace crate - IR types, parsing)
- `serde`, `serde_json` 1.0+ (serialization)
- `thiserror` (error handling)

**Storage**: N/A (in-memory widget tree)  
**Testing**: `cargo test` (148 existing tests, 100% pass rate required)  
**Target Platform**: Cross-platform (Linux, macOS, Windows) - desktop GUI applications  
**Project Type**: Workspace crate (`crates/dampen-iced/`) within monorepo  
**Performance Goals**: 
- XML parsing: < 10ms for 1000 widgets (currently ~0.284ms - well within budget)
- Widget rendering: 5% improvement after optimization (baseline: 0.284ms for 1000 widgets)
- Memory: 100KB reduction for 1000 widget UIs

**Constraints**: 
- MUST maintain 100% test pass rate (148 tests) throughout refactoring
- MUST produce zero clippy warnings (`cargo clippy --workspace -- -D warnings`)
- MUST preserve backward compatibility for DampenWidgetBuilder public API
- MUST NOT break Iced 0.14 compatibility
- MUST follow workspace coding standards (AGENTS.md)

**Scale/Scope**: 
- 10,265 lines of code in dampen-iced crate
- 33 modules across 6 directories
- 24 widget types (20 fully implemented)
- Target: reduce by 370+ lines through deduplication

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Core Principles Compliance

✅ **I. Declarative-First**: No impact - refactoring internal implementation, XML remains source of truth

✅ **II. Type Safety Preservation**: No impact - refactoring preserves all type signatures and message types

✅ **III. Production Mode**: No impact - refactoring affects both interpreted and codegen modes equally, maintains production codegen path

✅ **IV. Backend Abstraction**: ✅ COMPLIANT - Removing IcedBackend from dampen-iced strengthens abstraction by eliminating unused Backend trait implementation. All refactoring is within dampen-iced crate (backend-specific).

✅ **V. Test-First Development**: ✅ COMPLIANT - All 148 existing tests provide contract validation. Each refactoring step MUST pass full test suite before proceeding (TDD preservation).

### Quality Gates Compliance

✅ **Tests**: Baseline 148 tests pass, MUST remain passing after each incremental change

✅ **Linting**: Zero clippy warnings required (FR-010, SC-003)

✅ **Formatting**: `cargo fmt` compliance maintained via existing CI

✅ **Documentation**: FR-009, FR-011 require doc updates for new helpers (90% coverage target in SC-010)

### Technical Standards Compliance

✅ **Rust Edition 2024, MSRV 1.85**: No changes

✅ **Dependencies**: No new dependencies added (using existing `std::rc::Rc`)

✅ **Unsafe Code**: Zero unsafe code added

✅ **Performance Budgets**: 
- Current: 0.284ms for 1000 widgets ✅ (< 10ms budget)
- Target: 0.270ms after 5% improvement ✅ (still < 10ms budget)

### Constitution Violations

**NONE** - This refactoring is fully compliant with all constitution principles.

## Project Structure

### Documentation (this feature)

```text
specs/001-dampen-iced-refactor/
├── plan.md              # This file (/speckit.plan output)
├── research.md          # Phase 0: Pattern extraction research
├── data-model.md        # Phase 1: Helper function signatures
├── quickstart.md        # Phase 1: Migration guide for developers
├── contracts/           # Phase 1: Helper function APIs
│   ├── state-aware-styling-helper.md
│   ├── boolean-resolver-helper.md
│   └── handler-resolution-helper.md
├── checklists/
│   └── requirements.md  # Already created, validation complete
└── tasks.md             # Phase 2: Implementation tasks (NOT in this command)
```

### Source Code (repository root)

```text
crates/dampen-iced/
├── src/
│   ├── lib.rs                    # MODIFIED: Remove IcedBackend trait (lines 63-190)
│   ├── builder/
│   │   ├── mod.rs                # PUBLIC API: DampenWidgetBuilder
│   │   ├── helpers.rs            # NEW: Add 3 generic helper functions here
│   │   └── widgets/              # MODIFIED: 9 widget files refactored
│   │       ├── button.rs         # MODIFIED: Use boolean + handler helpers
│   │       ├── checkbox.rs       # MODIFIED: Use all 3 helpers
│   │       ├── radio.rs          # MODIFIED: Use all 3 helpers
│   │       ├── text_input.rs     # MODIFIED: Use state-aware + handler helpers
│   │       ├── toggler.rs        # MODIFIED: Use state-aware helper
│   │       ├── slider.rs         # MODIFIED: Implement state-aware styling
│   │       ├── pick_list.rs      # MODIFIED: Implement state-aware styling
│   │       ├── combo_box.rs      # MODIFIED: Implement state-aware styling
│   │       └── [16 other widgets unchanged]
│   ├── style_mapping.rs          # UNCHANGED: Status mapper functions exist
│   ├── convert.rs                # UNCHANGED
│   ├── theme_adapter.rs          # UNCHANGED
│   └── system_theme.rs           # UNCHANGED
└── tests/
    ├── builder_state_styles.rs   # MODIFIED: Add tests for 3 new widgets
    ├── status_mapping_tests.rs   # MODIFIED: Add tests for slider/pick_list/combo_box
    └── [other test files]        # UNCHANGED but must continue passing
```

**Structure Decision**: This is a refactoring within an existing workspace crate. No new directories are created. All changes are localized to `crates/dampen-iced/src/` with focus on `builder/helpers.rs` (new helper functions) and 9 widget files in `builder/widgets/`.

## Complexity Tracking

**No Constitution Violations** - This section is not applicable.

---

## Phase 0: Outline & Research

### Research Tasks

This phase resolves technical unknowns and establishes best practices for the refactoring.

#### R1: Generic Helper Function Design Pattern

**Question**: What is the optimal signature and design for a generic state-aware styling helper that works across 7 different widget types (checkbox, radio, text_input, toggler, slider, pick_list, combo_box)?

**Research Areas**:
- Rust generic programming patterns for widget styling
- Iced widget status enum mapping patterns
- Closure capture optimization techniques (clone vs Rc)
- Type parameter bounds for widget + status mapper functions

**Deliverable**: Design pattern with concrete type signatures for `apply_state_aware_style<W, S, M>` helper

---

#### R2: Boolean Attribute Parsing Standards

**Question**: What boolean value formats should be supported beyond "true"/"false"/"1"/"0"/"yes"/"no"/"on"/"off"?

**Research Areas**:
- HTML5 boolean attribute standards
- Common XML/XAML boolean conventions
- Case sensitivity handling (current: case-insensitive)
- Empty string handling (e.g., `disabled=""`)

**Deliverable**: Comprehensive boolean parsing specification with test cases

---

#### R3: Handler Resolution Error Context

**Question**: What contextual information should be included in error messages when handler parameter resolution fails?

**Research Areas**:
- Existing error message formats in dampen-core
- Iced error handling patterns
- Debugging information helpful for binding expression failures
- Span (line/column) integration for XML source locations

**Deliverable**: Error message format specification with examples

---

#### R4: Rc vs Arc for StyleClass Optimization

**Question**: Should `StyleClass` use `Rc` (single-threaded) or `Arc` (atomic, thread-safe) for reference counting?

**Research Areas**:
- Iced widget threading model (single-threaded event loop?)
- Performance impact of Arc atomic operations vs Rc
- Send/Sync trait requirements for Iced widgets
- Potential future multi-threaded rendering

**Deliverable**: Decision on Rc vs Arc with performance justification

---

#### R5: Verbose Logging Compile-Time Gating

**Question**: Should verbose logging use `#[cfg(debug_assertions)]` or a custom feature flag like `#[cfg(feature = "verbose")]`?

**Research Areas**:
- Rust conditional compilation best practices
- Impact on release binary size (current: 52 `if self.verbose` checks)
- Developer experience for enabling verbose logs in release mode (rare use case)
- Consistency with existing dampen workspace practices

**Deliverable**: Logging strategy with configuration recommendation

---

#### R6: Legacy IcedBackend Removal Impact Analysis

**Question**: Are there any external crates or undocumented usage of the IcedBackend trait that would break?

**Research Areas**:
- Search entire workspace for `IcedBackend` usage patterns
- Check if any examples/ or tests/ reference the legacy trait
- Review public API export in lib.rs
- Document migration path if external usage is found

**Deliverable**: Impact report and migration guide (if needed)

---

#### R7: Incremental Refactoring Order

**Question**: What is the optimal order to apply the 6 user stories (P1-P6) to minimize risk and maximize test coverage?

**Research Areas**:
- Dependency graph between user stories (e.g., P2 enables P3)
- Test coverage gaps that should be filled before refactoring
- Rollback strategy if a refactoring step fails
- Branch strategy (single branch vs incremental PRs)

**Deliverable**: Step-by-step refactoring sequence with validation checkpoints

---

### Research Output

**File**: `research.md` - Consolidated findings addressing all 7 research tasks above

**Format**:
```markdown
# Research: dampen-iced Refactoring

## R1: Generic Helper Function Design Pattern
**Decision**: [chosen approach]
**Rationale**: [why this approach]
**Alternatives Considered**: [other options evaluated]

[... repeat for R2-R7]
```

---

## Phase 1: Design & Contracts

### Prerequisites
- `research.md` complete with all 7 research tasks resolved
- Baseline benchmark run for performance comparison

### 1.1 Data Model

**File**: `data-model.md`

This refactoring doesn't introduce new domain entities, but does introduce new **code structure entities** (helper functions). Document these as "API entities":

**Entities**:

1. **StateAwareStyleHelper**
   - **Purpose**: Generic function that applies state-aware styling to widgets
   - **Type Signature**: `fn apply_state_aware_style<W, S, M>(widget: W, node: &WidgetNode, base_style: StyleProperties, style_class: Option<StyleClass>, mapper: fn(S) -> Option<WidgetState>) -> W`
   - **Constraints**: Widget W must have a `.style()` method accepting a closure
   - **Relations**: Used by 7 widget builders (checkbox, radio, text_input, toggler, slider, pick_list, combo_box)

2. **BooleanAttributeResolver**
   - **Purpose**: Parse AttributeValue into bool with multiple format support
   - **Type Signature**: `fn resolve_boolean_attribute(node: &WidgetNode, attr_name: &str, default: bool) -> bool`
   - **Validation Rules**: Case-insensitive, supports "true"/"1"/"yes"/"on" (and negations)
   - **Relations**: Used by 3 widget builders (button, radio, checkbox)

3. **HandlerResolutionHelper**
   - **Purpose**: Resolve event handler parameters from context or model bindings
   - **Type Signature**: `fn resolve_handler_param<M>(builder: &DampenWidgetBuilder, event_param: &str) -> Result<BindingValue, HandlerResolutionError>`
   - **Error Handling**: Returns HandlerResolutionError with widget type, binding expression, and suggestion
   - **Relations**: Used by 5 widget builders (button, checkbox, radio, slider, text_input)

4. **StyleClassWrapper**
   - **Purpose**: Rc-wrapped StyleClass for efficient clone in closures
   - **Type Signature**: `type StyleClassWrapper = Rc<StyleClass>`
   - **Performance**: Reduces 200-byte clone to 8-byte pointer clone
   - **Relations**: Used by StateAwareStyleHelper and all state-aware widgets

5. **VerboseLoggingGuard**
   - **Purpose**: Compile-time guard for verbose logging
   - **Implementation**: Macro or cfg-gated function
   - **Type Signature**: `macro_rules! verbose_log { ($self:expr, $($arg:tt)*) => { ... } }`
   - **Relations**: Replaces 52 `if self.verbose { eprintln!(...) }` calls

### 1.2 API Contracts

**Directory**: `contracts/`

#### Contract 1: State-Aware Styling Helper

**File**: `contracts/state-aware-styling-helper.md`

```markdown
# State-Aware Styling Helper Contract

## Function Signature

```rust
pub fn apply_state_aware_style<W, S, M>(
    widget: W,
    node: &WidgetNode,
    base_style_props: StyleProperties,
    style_class: Option<&StyleClass>,
    status_mapper: fn(S) -> Option<WidgetState>,
) -> W
where
    W: Widget<S, M>,  // Pseudo-trait for widgets with .style() method
    S: iced::widget::Status,  // Widget-specific status enum
    M: Clone + 'static,  // Message type
```

## Behavior

1. **Input Validation**:
   - `node`: Widget XML node with potential style attributes
   - `base_style_props`: Base styles to apply (before state variants)
   - `style_class`: Optional class with state-specific styles (hover, focus, etc.)
   - `status_mapper`: Widget-specific function mapping Status → WidgetState

2. **Processing**:
   - Clone `base_style_props` and `style_class` (or use Rc wrappers per R4 decision)
   - Return widget with `.style()` closure that:
     - Maps current status to WidgetState using mapper
     - Resolves state variant from style_class
     - Merges base styles with state-specific styles
     - Applies merged styles to widget appearance

3. **Error Handling**: No errors - invalid styles are ignored gracefully

## Test Cases

- Widget with base styles only (no state variants)
- Widget with hover state (status changes on mouse over)
- Widget with focus state (status changes on focus)
- Widget with disabled state
- Widget with both class and inline styles (inline overrides class)
```

---

#### Contract 2: Boolean Attribute Resolver

**File**: `contracts/boolean-resolver-helper.md`

```markdown
# Boolean Attribute Resolver Contract

## Function Signature

```rust
pub fn resolve_boolean_attribute(
    node: &WidgetNode,
    attr_name: &str,
    default: bool,
) -> bool
```

## Behavior

1. **Input Validation**:
   - `node`: Widget XML node
   - `attr_name`: Attribute name to resolve (e.g., "disabled", "enabled")
   - `default`: Value to return if attribute is missing

2. **Processing**:
   - Get attribute from node: `node.attributes.get(attr_name)`
   - If None → return `default`
   - If Some(AttributeValue::Static(s)):
     - Convert to lowercase
     - Match against truthy values: "true", "1", "yes", "on" → true
     - Match against falsy values: "false", "0", "no", "off" → false
     - Empty string → true (HTML5 boolean attribute convention)
     - Unknown value → default
   - If Some(AttributeValue::Binding(_)) → evaluate binding (future: may need model context)

3. **Error Handling**: Invalid boolean values default to `default` (no panic)

## Test Cases

- Missing attribute → default
- "true" → true
- "TrUe" (case variation) → true
- "1" → true
- "yes" → true
- "on" → true
- "false" → false
- "0" → false
- "no" → false
- "off" → false
- "" (empty) → true
- "invalid" → default
```

---

#### Contract 3: Handler Resolution Helper

**File**: `contracts/handler-resolution-helper.md`

```markdown
# Handler Resolution Helper Contract

## Function Signature

```rust
pub fn resolve_handler_param<M>(
    builder: &DampenWidgetBuilder<M>,
    event_param_expr: &str,
) -> Result<BindingValue, HandlerResolutionError>
where
    M: UiBindable + Clone + 'static,
```

## Behavior

1. **Input Validation**:
   - `builder`: DampenWidgetBuilder with context and model state
   - `event_param_expr`: Binding expression (e.g., "item.value", "model.count")

2. **Processing**:
   - First attempt: Resolve from context using `builder.resolve_from_context(event_param_expr)`
   - If Some(value) → return Ok(value)
   - Second attempt: Evaluate from model using `evaluate_binding_expr_with_shared(...)`
   - If Ok(value) → return Ok(value)
   - If Err(e) → return Err with context

3. **Error Handling**: 
   - Return `HandlerResolutionError` with:
     - Widget type name
     - Binding expression that failed
     - Original evaluation error
     - Suggestion (e.g., "Ensure 'model.count' exists in your model")

## Error Type

```rust
#[derive(Debug, Clone)]
pub struct HandlerResolutionError {
    pub widget_type: String,
    pub binding_expr: String,
    pub error: String,
    pub suggestion: Option<String>,
}
```

## Test Cases

- Context-based resolution succeeds
- Model-based resolution succeeds after context fails
- Both resolutions fail → error with context
- Binding expression with typo → helpful error message
```

---

### 1.3 Quickstart Guide

**File**: `quickstart.md`

```markdown
# Quickstart: Using Refactored dampen-iced Helpers

## Overview

The dampen-iced crate now provides three generic helper functions to simplify widget implementation:

1. `apply_state_aware_style` - For widgets with hover/focus/disabled states
2. `resolve_boolean_attribute` - For parsing boolean attributes
3. `resolve_handler_param` - For resolving event handler parameters

## For Widget Developers

### Adding State-Aware Styling to a New Widget

**Before** (60-70 lines of duplicated code):
```rust
// checkbox.rs - OLD APPROACH
let base_style = base_style_props.clone();
let class = style_class.cloned();
widget.style(move |_theme, status| {
    let state = match status {
        Status::Active => WidgetState::Base,
        Status::Hovered => WidgetState::Hover,
        // ... 50 more lines
    };
    // ... merge styles, apply properties
})
```

**After** (< 10 lines using helper):
```rust
// checkbox.rs - NEW APPROACH
use crate::builder::helpers::apply_state_aware_style;
use crate::style_mapping::map_checkbox_status;

apply_state_aware_style(
    widget,
    node,
    base_style_props,
    style_class.as_ref(),
    map_checkbox_status,  // Widget-specific mapper
)
```

### Parsing Boolean Attributes

**Before**:
```rust
let disabled = match node.attributes.get("disabled") {
    None => false,
    Some(AttributeValue::Static(s)) => match s.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        // ... 10 more lines
    }
};
```

**After**:
```rust
use crate::builder::helpers::resolve_boolean_attribute;

let disabled = resolve_boolean_attribute(node, "disabled", false);
```

### Resolving Handler Parameters

**Before**:
```rust
if let Some(param_expr) = &event.param {
    if let Some(value) = self.resolve_from_context(param_expr) {
        // ... 15 lines
    } else {
        match evaluate_binding_expr_with_shared(...) {
            // ... 20 more lines
        }
    }
}
```

**After**:
```rust
use crate::builder::helpers::resolve_handler_param;

if let Some(param_expr) = &event.param {
    match resolve_handler_param(self, param_expr) {
        Ok(value) => { /* use value */ },
        Err(e) => { /* error already has context */ },
    }
}
```

## For Framework Users

**No Breaking Changes** - All existing `.dampen` XML files work identically. This refactoring is internal only.

## Performance Improvements

- **Memory**: ~100KB reduction for UIs with 1000+ widgets
- **Rendering**: ~5% faster for large UIs
- **Binary Size**: ~5KB smaller in release mode (no verbose logging)

## Migration from Legacy IcedBackend

The `IcedBackend` trait has been removed. Use `DampenWidgetBuilder` instead:

**Before**:
```rust
let backend = IcedBackend::new(handler);
let widget = backend.render(document);
```

**After**:
```rust
let builder = DampenWidgetBuilder::new(model, handlers, verbose);
let widget = builder.build(&document);
```

See existing examples in `examples/` directory for complete patterns.
```

---

### 1.4 Agent Context Update

After completing Phase 1 design documents, run:

```bash
.specify/scripts/bash/update-agent-context.sh opencode
```

This updates `AGENTS.md` with:
- New helper function APIs
- Refactoring patterns
- Performance optimization notes
- Migration guidance

**New sections to add**:
```markdown
## dampen-iced Refactoring Helpers

### State-Aware Styling Helper
Location: `crates/dampen-iced/src/builder/helpers.rs`
[... function signature and usage ...]

### Boolean Attribute Resolver
[... documentation ...]

### Handler Resolution Helper
[... documentation ...]
```

---

## Phase 2: Task Breakdown

**NOT IMPLEMENTED IN THIS COMMAND**

This phase is handled by `/speckit.tasks` command, which will:
- Generate `tasks.md` with specific implementation tasks
- Create task checklist for each user story (P1-P6)
- Define validation checkpoints
- Provide code snippets and test templates

---

## Validation Checkpoints

After each user story implementation:

1. **Run Full Test Suite**: `cargo test --workspace`
   - MUST show 148/148 tests passing
   - Zero failures, zero panics

2. **Run Clippy**: `cargo clippy --workspace -- -D warnings`
   - MUST produce zero warnings
   - Check all lints from workspace `Cargo.toml`

3. **Run Benchmarks** (for P6 optimization):
   - `cargo bench -p benchmarks` (if exists)
   - Document baseline vs optimized performance

4. **Code Review Checklist**:
   - [ ] Documentation updated for new helpers
   - [ ] No TODO comments added without tickets
   - [ ] All helper functions have usage examples
   - [ ] Error messages include actionable suggestions

5. **Line Count Validation**:
   - Measure lines removed vs baseline
   - Target: 370+ lines reduced by end of all stories

---

## Success Metrics Tracking

Track these metrics throughout implementation:

| Metric | Baseline | Target | Actual |
|--------|----------|--------|--------|
| Total LOC | 10,265 | < 9,900 | _TBD_ |
| Duplicated LOC | ~400 | < 50 | _TBD_ |
| Test Pass Rate | 148/148 (100%) | 148/148 (100%) | _TBD_ |
| Clippy Warnings | 0 | 0 | _TBD_ |
| Render Time (1000w) | 0.284ms | < 0.270ms | _TBD_ |
| Memory (1000w) | _TBD_ | -100KB | _TBD_ |
| Binary Size | _TBD_ | -5KB | _TBD_ |
| Maintainability | 3/5 | 4/5+ | _TBD_ |

---

## Timeline Estimate

Based on refactoring scope:

- **Phase 0 (Research)**: 0.5 days (automated via agents)
- **Phase 1 (Design)**: 0.5 days (completed in this plan)
- **Phase 2 (Tasks)**: 0.25 days (automated via `/speckit.tasks`)
- **Implementation**:
  - P1 (Remove legacy): 0.5 days
  - P2 (Extract state-aware): 1 day
  - P3 (Implement missing styling): 0.5 days
  - P4 (Extract boolean helper): 0.25 days
  - P5 (Extract handler helper): 0.5 days
  - P6 (Optimize clones): 0.5 days
- **Testing & Validation**: 0.5 days
- **Documentation**: 0.25 days

**Total**: ~5 days (1 developer week)

---

## Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Test failures after refactor | Medium | High | Incremental approach, test after each widget |
| Generic helpers miss edge cases | Medium | Medium | Start with one widget, validate thoroughly |
| Performance regression | Low | Medium | Benchmark before/after, use Rc conditionally |
| Breaking external usage | Low | High | Search workspace, add deprecation warnings |
| Iced API limitations | Low | Medium | Review Iced docs before implementing |

---

## Next Steps

1. ✅ **Complete**: Feature specification
2. ✅ **Complete**: Implementation plan (this document)
3. **Next**: Run `/speckit.plan` to auto-generate Phase 0 research
4. **Then**: Review research.md outputs
5. **Finally**: Run `/speckit.tasks` to generate task breakdown
