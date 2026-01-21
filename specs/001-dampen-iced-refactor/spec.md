# Feature Specification: dampen-iced Crate Refactoring

**Feature Branch**: `001-dampen-iced-refactor`  
**Created**: 2026-01-21  
**Status**: Draft  
**Input**: User description: "Refactor dampen-iced crate to remove legacy code, extract duplicated patterns, implement missing state-aware styling, and optimize performance based on code analysis report"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Remove Legacy IcedBackend Code (Priority: P1)

As a framework maintainer, I need to remove the obsolete IcedBackend rendering system so that the codebase only contains actively used code and reduces confusion for contributors.

**Why this priority**: This removes 128 lines of unused code that creates confusion about which rendering approach to use. The legacy system has zero test coverage and zero usage in the modern codebase, making it safe to remove with high impact on code clarity.

**Independent Test**: Can be fully tested by removing lib.rs:63-190, running the full test suite (148 tests), and verifying all tests still pass with zero compilation errors.

**Acceptance Scenarios**:

1. **Given** the legacy IcedBackend trait exists in lib.rs, **When** it is removed, **Then** all 148 existing tests continue to pass
2. **Given** the legacy render() function with placeholder widgets exists, **When** it is removed, **Then** no compilation errors occur in any crate
3. **Given** the codebase uses DampenWidgetBuilder exclusively, **When** legacy code is removed, **Then** no functionality is lost for any supported widget
4. **Given** documentation references to IcedBackend exist, **When** legacy code is removed, **Then** documentation is updated to reference only DampenWidgetBuilder

---

### User Story 2 - Extract State-Aware Styling Pattern (Priority: P2)

As a framework developer, I need duplicated state-aware styling code (~200 lines across 4 widgets) extracted into a reusable helper so that adding new widgets is faster and styling behavior remains consistent.

**Why this priority**: This refactoring reduces ~200 lines of duplication and makes the codebase easier to maintain. It directly enables P3 (implementing missing state-aware styling) by providing a single source of truth for the styling pattern.

**Independent Test**: Can be tested by creating the generic helper function, migrating one widget (e.g., checkbox) to use it, verifying the widget's state-aware tests still pass, then repeating for other widgets.

**Acceptance Scenarios**:

1. **Given** checkbox.rs has 60-70 lines of state-aware styling code, **When** it is migrated to use the generic helper, **Then** the checkbox state-aware tests pass identically
2. **Given** the helper function is implemented, **When** radio.rs, text_input.rs, and toggler.rs are migrated, **Then** all state-aware styling tests pass for all widgets
3. **Given** the styling pattern is extracted, **When** calculating lines of code, **Then** at least 150 lines of duplication are removed
4. **Given** a new widget needs state-aware styling, **When** a developer uses the helper, **Then** they write less than 10 lines of widget-specific styling code

---

### User Story 3 - Implement Missing State-Aware Styling (Priority: P3)

As a framework user, I need slider, pick_list, and combo_box widgets to respond visually to hover, focus, and disabled states so that my application provides consistent interactive feedback across all widgets.

**Why this priority**: Completes the state-aware styling feature for all interactive widgets. Currently these 3 widgets don't respond to user interactions visually, creating an inconsistent user experience. This builds on P2's extracted pattern.

**Independent Test**: Can be tested by creating XML files with slider/pick_list/combo_box widgets, applying hover/focus/disabled styles, running the application, and visually verifying state transitions.

**Acceptance Scenarios**:

1. **Given** a slider widget with hover styling defined, **When** user hovers over it, **Then** the slider visually changes to the hover style
2. **Given** a pick_list widget with focus styling defined, **When** user focuses it, **Then** the pick_list visually changes to the focus style
3. **Given** a combo_box widget with disabled="true" attribute, **When** the widget renders, **Then** it displays the disabled style and doesn't respond to interactions
4. **Given** all three widgets have state-aware styling, **When** integration tests run, **Then** status mapping tests pass for slider, pick_list, and combo_box

---

### User Story 4 - Extract Boolean Attribute Resolution Pattern (Priority: P4)

As a framework developer, I need the repeated boolean attribute parsing code (~50 lines across 3 widgets) extracted into a helper so that attribute handling is consistent and maintainable.

**Why this priority**: This is a smaller refactoring that reduces duplication and prevents bugs from inconsistent boolean parsing logic across widgets.

**Independent Test**: Can be tested by creating the helper, migrating button.rs to use it, verifying button tests pass, then migrating radio.rs and checkbox.rs.

**Acceptance Scenarios**:

1. **Given** button.rs parses "disabled" attribute with multiple boolean variations, **When** migrated to use resolve_boolean_attribute helper, **Then** all boolean formats (true/1/yes/on) are correctly handled
2. **Given** the helper supports default values, **When** an attribute is missing, **Then** the default value is returned correctly
3. **Given** all three widgets use the helper, **When** calculating code, **Then** approximately 50 lines of duplication are removed
4. **Given** a developer adds a new boolean attribute, **When** they use the helper, **Then** all standard boolean formats are automatically supported

---

### User Story 5 - Extract Handler Resolution Pattern (Priority: P5)

As a framework developer, I need the repeated event handler resolution code (~120 lines across 5 widgets) extracted into a generic helper so that event handling logic is centralized and bugs are easier to fix.

**Why this priority**: This reduces the largest duplication block but is lower priority because the existing code works correctly and this is purely a maintenance improvement.

**Independent Test**: Can be tested by creating the helper, migrating one widget at a time, and verifying that event binding tests pass for each migrated widget.

**Acceptance Scenarios**:

1. **Given** button.rs has handler resolution with context and model binding fallback, **When** migrated to use the generic helper, **Then** all button event tests pass
2. **Given** the helper handles both context-based and model-based parameter resolution, **When** used by checkbox, radio, slider, and text_input, **Then** all event binding tests pass
3. **Given** all five widgets use the helper, **When** calculating code, **Then** approximately 120 lines of duplication are removed
4. **Given** a binding error occurs, **When** the helper handles it, **Then** error messages include helpful context about which widget and binding failed

---

### User Story 6 - Optimize Clone Performance (Priority: P6)

As a framework user building large UIs with 1000+ widgets, I need reduced memory allocation during style closure creation so that my application starts faster and uses less memory.

**Why this priority**: This is an optimization that provides measurable benefits only for large UIs. Current performance is already good (1000 widgets in ~0.284ms), making this lower priority than code quality improvements.

**Independent Test**: Can be tested by running benchmarks before and after the optimization, measuring both time and memory allocations for 1000+ widget UIs.

**Acceptance Scenarios**:

1. **Given** StyleClass is wrapped in Rc instead of cloned, **When** rendering 1000 widgets, **Then** memory allocations are reduced by at least 150KB (200 bytes × 1000 - overhead)
2. **Given** benchmarks run for 1000 widgets, **When** using Rc-based approach, **Then** rendering time improves by at least 5%
3. **Given** the optimization is implemented, **When** all tests run, **Then** zero functionality changes and all 148 tests pass
4. **Given** verbose logging is guarded by cfg(debug_assertions), **When** release builds are created, **Then** no verbose logging code is included in the binary

---

### Edge Cases

- What happens when state-aware styling is applied to widgets that don't support it? System should gracefully ignore unsupported style properties without errors.
- How does the system handle migrating from legacy IcedBackend if external code depends on it? Document deprecation path and provide migration guide.
- What happens when a widget has both inline styles and class-based state-aware styles? Inline styles should override class styles following CSS precedence rules.
- How does boolean attribute parsing handle edge cases like "TrUe", "0", "false"? Case-insensitive parsing with comprehensive test coverage for all variations.
- What happens when handler resolution fails for context and model bindings? Clear error message with binding expression and widget type information.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST remove all legacy IcedBackend trait code (lib.rs:63-190) without breaking any existing tests
- **FR-002**: System MUST extract state-aware styling pattern into a generic helper function that works for checkbox, radio, text_input, and toggler widgets
- **FR-003**: System MUST implement state-aware styling for slider, pick_list, and combo_box widgets with hover, focus, active, and disabled state support
- **FR-004**: System MUST extract boolean attribute resolution into a reusable helper that handles "true"/"1"/"yes"/"on" and their negations case-insensitively
- **FR-005**: System MUST extract handler resolution pattern into a generic helper supporting both context-based and model-based parameter resolution
- **FR-006**: System MUST optimize StyleClass handling to use reference-counted pointers (Rc) instead of cloning in style closures
- **FR-007**: System MUST guard verbose logging with compile-time conditionals to exclude it from release builds
- **FR-008**: All refactoring MUST maintain 100% test pass rate (148 tests) throughout the process
- **FR-009**: System MUST update documentation to reflect removed legacy code and new helper functions
- **FR-010**: System MUST pass clippy linting with zero warnings after all refactoring is complete
- **FR-011**: Extracted helpers MUST be thoroughly documented with usage examples and type signatures
- **FR-012**: State-aware styling helpers MUST support custom status-to-state mapping functions for widget-specific behavior

### Key Entities *(optional - included for code structure clarity)*

- **Generic Style Helper**: Reusable function that applies state-aware styling by accepting a widget, node, and status mapper function, returning the styled widget
- **Boolean Attribute Resolver**: Helper function that parses AttributeValue into bool, supporting multiple string representations and defaults
- **Handler Resolution Helper**: Generic function that resolves event handler parameters from context or model bindings, handling errors consistently
- **StyleClass Wrapper**: Rc-wrapped style class that can be cheaply cloned for closure captures
- **Status Mapper**: Function type that converts widget-specific Status enums to WidgetState for style resolution

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Code duplication is reduced by at least 350 lines across all refactoring tasks (200 + 50 + 120 from patterns, plus legacy code removal)
- **SC-002**: All 148 existing tests pass after each incremental refactoring step, maintaining 100% test success rate throughout
- **SC-003**: Clippy produces zero warnings when running `cargo clippy --workspace -- -D warnings` after refactoring is complete
- **SC-004**: Rendering performance for 1000 widgets improves by at least 5% after clone optimizations (baseline: ~0.284ms)
- **SC-005**: Memory allocations for 1000 widgets are reduced by at least 100KB after Rc optimization
- **SC-006**: Release binary size is reduced by at least 5KB after removing verbose logging from release builds
- **SC-007**: Adding a new widget with state-aware styling requires less than 15 lines of widget-specific code after helper extraction
- **SC-008**: Slider, pick_list, and combo_box widgets correctly respond to all four interaction states (base, hover, focus, disabled) after state-aware styling implementation
- **SC-009**: Code maintainability score improves from 3/5 to 4/5 or higher as measured by reduced duplication and clearer architecture
- **SC-010**: Documentation is updated for all public helper functions with at least 90% coverage of parameters, return values, and usage examples

## Assumptions

- The 148 passing tests provide sufficient coverage to detect regressions during refactoring
- No external code depends on the legacy IcedBackend trait (verified by zero test usage)
- Performance benchmarks can be run using existing benchmark infrastructure
- The existing status mapping functions (map_slider_status, map_picklist_status) are correct and just need to be integrated
- Rust 2024 edition and MSRV 1.85 remain unchanged during refactoring
- Breaking changes to public APIs are acceptable as this is framework development, not library consumption
- The verbose logging flag is primarily used during development and can be compile-time gated
- Iced 0.14 limitations (like Container lacking Status parameter) remain unchanged

## Scope Boundaries

### In Scope
- Removing legacy IcedBackend trait and render() function
- Extracting three main duplication patterns (state-aware styling, boolean resolution, handler resolution)
- Implementing state-aware styling for slider, pick_list, combo_box
- Optimizing clone usage in style closures via Rc
- Improving verbose logging with compile-time guards
- Updating documentation for affected code
- Maintaining all existing test coverage

### Out of Scope
- Adding new widgets beyond the existing 20
- Implementing canvas::Program access from model bindings (noted as TODO but separate feature)
- Changing the overall architecture of DampenWidgetBuilder
- Performance optimizations beyond clone reduction (e.g., allocation pooling in merge_styles)
- Upgrading Iced framework version
- Adding new state-aware styling features beyond hover/focus/active/disabled
- Refactoring scrollable and stack from their current column-based fallback implementations
- Adding new boolean attribute formats beyond current standard variations

## Dependencies

- Existing test suite must remain comprehensive enough to catch regressions
- Benchmark infrastructure must be available for performance validation
- Code analysis tools (clippy, rustfmt) must be configured and available
- Documentation review process for updated public APIs
- The report's line count analysis accuracy (verified against actual file inspection)

## Risks

- **Risk**: Removing legacy IcedBackend might break undocumented external usage
  - **Mitigation**: Search entire workspace for usage, add deprecation warning before removal, document migration path
  
- **Risk**: Generic helper functions might not accommodate all widget-specific edge cases
  - **Mitigation**: Implement incrementally, one widget at a time, with full test validation at each step
  
- **Risk**: Rc overhead might negate performance benefits for small UIs
  - **Mitigation**: Benchmark both small (100 widgets) and large (1000+ widgets) scenarios
  
- **Risk**: State-aware styling implementation might expose Iced 0.14 limitations
  - **Mitigation**: Review Iced documentation for each widget's Status enum before implementation
  
- **Risk**: Refactoring might introduce subtle behavioral changes undetected by tests
  - **Mitigation**: Review test coverage before refactoring, add integration tests for state-aware behavior if needed
