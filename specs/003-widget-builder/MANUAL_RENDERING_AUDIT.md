# Manual Rendering Code Audit

**Date**: 2026-01-03  
**Phase**: 5 - User Story 2 (Centralize Interpretation Logic)  
**Task**: T081 - Document any remaining manual rendering code

---

## Executive Summary

**Status**: 3 of 7 examples still use manual rendering logic

**Examples Using Builder**: ✅ 4/7
- `styling/src/main.rs` (109 lines)
- `styling/src/state_demo.rs` (111 lines)
- `counter/src/main.rs` (103 lines)
- `todo-app/src/main.rs` (207 lines)

**Examples with Manual Rendering**: ⚠️ 3/7
- `responsive/src/main.rs` (207 lines)
- `class-demo/src/main.rs` (415 lines)
- `hello-world/src/main.rs` (79 lines)

---

## Detailed Findings

### 1. responsive/src/main.rs

**Lines**: 207 total, ~110 lines of manual rendering logic

**Manual Rendering Code**:
- Custom `render_node()` function (lines 79-190)
- Manual pattern matching on `WidgetKind` (Text, Button, Column, Row, Container)
- Manual attribute extraction using `node.attributes.get()`
- Manual style application using `map_style_properties()`
- Manual layout handling (width, height, padding, spacing)
- Custom breakpoint resolution logic

**Why Not Using Builder**:
- Uses breakpoint system via `resolve_tree_breakpoint_attributes()`
- Custom viewport width tracking
- Window resize subscription

**Refactoring Strategy**:
- Builder should support breakpoint-aware rendering
- Could add `GravityWidgetBuilder::with_viewport_width(width)`
- Would enable builder usage while keeping breakpoint logic

---

### 2. class-demo/src/main.rs

**Lines**: 415 total, ~250 lines of manual rendering logic

**Manual Rendering Code**:
- Custom `render_node()` function (lines 138-336)
- Manual pattern matching on `WidgetKind` (Text, Button, Column, Row, Container)
- Manual binding evaluation for `{count}` and `{clicks}` (lines 152-158)
- Manual style cascade resolution (lines 175-180, 214-219, 282-287)
- Manual theme-aware styling (lines 166-185)
- Manual state-based button styling (lines 224-235)
- Custom helper module for style mapping (lines 374-414)

**Why Not Using Builder**:
- Uses `ThemeManager` and `StyleCascade` from gravity-runtime
- Demonstrates class-based styling with inheritance
- Shows state-based styling (hover, active)
- Example of theme integration

**Refactoring Strategy**:
- Builder should integrate with `ThemeManager` and `StyleCascade`
- Could add `GravityWidgetBuilder::with_theme_manager(theme_manager)`
- Could add `GravityWidgetBuilder::with_style_cascade(style_cascade)`
- Would enable builder usage while keeping theme/class logic

---

### 3. hello-world/src/main.rs

**Lines**: 79 total, ~30 lines of manual rendering logic

**Manual Rendering Code**:
- Custom `render_node()` function (lines 37-69)
- Manual pattern matching on `WidgetKind` (Text, Button, Column, Row)
- Manual attribute extraction using `node.attributes.get()`
- No styling or layout (basic example)
- Hardcoded `Message::Greet` for button

**Why Not Using Builder**:
- Originally created before builder existed
- Intended as minimal example (no dependencies beyond iced)
- Shows basic parsing and rendering

**Refactoring Strategy**:
- Simple conversion to `GravityWidgetBuilder::new()`
- Would reduce from 79 to ~30 lines
- Would demonstrate builder simplicity

---

## Impact Analysis

### Code Duplication

**Manual Rendering Logic** (duplicated across examples):
- Pattern matching on `WidgetKind`: 3 examples (responsive, class-demo, hello-world)
- Attribute extraction (`node.attributes.get()`): 3 examples
- Layout handling (width, height, padding, spacing): 2 examples (responsive, class-demo)
- Style application: 2 examples (responsive, class-demo)

**Total Lines of Duplication**: ~390 lines across 3 examples

### Benefits of Refactoring

1. **Code Reduction**: ~390 lines → ~50 lines (87% reduction)
2. **Consistency**: All examples use same rendering approach
3. **Maintainability**: Widget changes only affect builder.rs
4. **Discoverability**: New users see builder pattern immediately
5. **Feature Parity**: Builder gains breakpoint/theme support

---

## Refactoring Priority

### High Priority (P0)
- **hello-world**: Simple conversion, biggest user impact

### Medium Priority (P1)
- **responsive**: Requires breakpoint integration
- **class-demo**: Requires theme/cascade integration

### Blockers

1. **Breakpoint Support**: Builder needs viewport width awareness
2. **Theme Integration**: Builder needs ThemeManager/StyleCascade integration
3. **State-Based Styling**: Builder needs widget state tracking

---

## Recommendations

### Phase 1: Simple Refactoring
- ✅ Refactor `hello-world` to use builder (no blockers)
- Document pattern for others to follow

### Phase 2: Feature Additions
- Add `GravityWidgetBuilder::with_viewport_width()` for breakpoints
- Add `GravityWidgetBuilder::with_theme_manager()` for themes
- Add `GravityWidgetBuilder::with_style_cascade()` for class styling

### Phase 3: Complete Migration
- ✅ Refactor `responsive` using new breakpoint support
- ✅ Refactor `class-demo` using new theme support
- Remove all manual `render_node()` functions

---

## Success Criteria

- [ ] All examples use `GravityWidgetBuilder`
- [ ] No custom `render_node()` functions in examples
- [ ] No manual pattern matching on `WidgetKind`
- [ ] No manual attribute extraction
- [ ] Builder supports all features needed by examples

---

## Next Steps

See **T082** for refactoring implementation plan.
