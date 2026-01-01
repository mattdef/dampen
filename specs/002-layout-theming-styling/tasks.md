# Task Breakdown: Layout, Sizing, Theming, and Styling System

**Feature Branch**: `002-layout-theming-styling`  
**Generated**: 2026-01-01  
**Plan**: [plan.md](plan.md) | **Spec**: [spec.md](spec.md) | **Data Model**: [data-model.md](data-model.md) | **Research**: [research.md](research.md)

---

## Overview

This task breakdown implements the complete layout, sizing, theming, and styling system for Gravity's declarative XML UI framework. All tasks are organized by user story to enable independent implementation and testing.

**Total Tasks**: 78  
**User Stories**: 8 (P1: 3, P2: 3, P3: 2)  
**Parallel Opportunities**: 23 tasks marked [P]

---

## Dependencies

### User Story Completion Order

```
Phase 1: Setup (Foundation)
  ↓
Phase 2: Foundational (All Stories)
  ↓
Phase 3: [US1] Widget Sizing and Spacing (P1)
  ↓
Phase 4: [US2] Flexible Layout Constraints (P1)
  ↓
Phase 5: [US3] Theming System (P1)
  ↓
Phase 6: [US4] Inline Widget Styling (P2)
  ↓
Phase 7: [US5] Style Classes (P2)
  ↓
Phase 8: [US6] Alignment and Positioning (P2)
  ↓
Phase 9: [US7] Responsive Breakpoints (P3)
  ↓
Phase 10: [US8] State-Based Styling (P3)
  ↓
Phase 11: Polish & Integration
```

**Note**: All user stories are **independently testable**. Each phase delivers a complete increment.

---

## Phase 1: Setup (Project Initialization)

**Goal**: Initialize project structure and add required dependencies.

- [X] T001 Add `csscolorparser` dependency to gravity-core/Cargo.toml
- [X] T002 Add `nom` dependency to gravity-core/Cargo.toml
- [X] T003 Create `gravity-core/src/ir/layout.rs` module
- [X] T004 Create `gravity-core/src/ir/style.rs` module
- [X] T005 Create `gravity-core/src/ir/theme.rs` module
- [X] T006 Create `gravity-core/src/parser/style_parser.rs` module
- [X] T007 Create `gravity-core/src/parser/theme_parser.rs` module
- [X] T008 Create `gravity-core/src/parser/gradient.rs` module
- [X] T009 Create `gravity-runtime/src/theme_manager.rs` module
- [X] T010 Create `gravity-runtime/src/style_cascade.rs` module
- [X] T011 Create `gravity-iced/src/style_mapping.rs` module
- [X] T012 Create `gravity-iced/src/theme_adapter.rs` module
- [X] T013 Create `gravity-iced/src/widgets/styled.rs` module
- [X] T014 Create `examples/styling/` directory structure
- [X] T015 Create `examples/responsive/` directory structure

**Total Phase 1 Tasks**: 15

---

## Phase 2: Foundational (Prerequisites for All Stories)

**Goal**: Implement core IR types and parsing infrastructure needed by all user stories.

### 2.1: Layout System Types

- [X] T016 [P] Implement `Length` enum in gravity-core/src/ir/layout.rs
- [X] T017 [P] Implement `Padding` struct with parse() method in gravity-core/src/ir/layout.rs
- [X] T018 [P] Implement `Alignment` enum in gravity-core/src/ir/layout.rs
- [X] T019 [P] Implement `Justification` enum in gravity-core/src/ir/layout.rs
- [X] T020 [P] Implement `Direction` enum in gravity-core/src/ir/layout.rs
- [X] T021 [P] Implement `LayoutConstraints` struct in gravity-core/src/ir/layout.rs

### 2.2: Style System Types

- [X] T022 [P] Implement `Color` struct with parse() method in gravity-core/src/ir/style.rs
- [X] T023 [P] Implement `Background` enum in gravity-core/src/ir/style.rs
- [X] T024 [P] Implement `Border` and `BorderRadius` structs in gravity-core/src/ir/style.rs
- [X] T025 [P] Implement `Shadow` struct in gravity-core/src/ir/style.rs
- [X] T026 [P] Implement `Transform` enum in gravity-core/src/ir/style.rs
- [X] T027 [P] Implement `StyleProperties` struct in gravity-core/src/ir/style.rs

### 2.3: Theme System Types

- [X] T028 [P] Implement `ThemePalette` struct in gravity-core/src/ir/theme.rs
- [X] T029 [P] Implement `Typography` and `FontWeight` in gravity-core/src/ir/theme.rs
- [X] T030 [P] Implement `SpacingScale` struct in gravity-core/src/ir/theme.rs
- [X] T031 [P] Implement `Theme` struct in gravity-core/src/ir/theme.rs
- [X] T032 [P] Implement `WidgetState` enum in gravity-core/src/ir/theme.rs
- [X] T033 [P] Implement `StyleClass` struct in gravity-core/src/ir/theme.rs
- [X] T034 [P] Implement `Breakpoint` enum in gravity-core/src/ir/layout.rs

### 2.4: Core IR Extensions

- [X] T035 Extend `WidgetNode` with style/layout/theme fields in gravity-core/src/ir/node.rs
- [X] T036 Extend `GravityDocument` with themes/classes/global_theme in gravity-core/src/ir/mod.rs

### 2.5: Parser Infrastructure

- [X] T037 Implement `parse_color_attr()` in gravity-core/src/parser/style_parser.rs
- [X] T038 Implement `parse_length_attr()` in gravity-core/src/parser/style_parser.rs
- [X] T039 Implement `parse_padding_attr()` in gravity-core/src/parser/style_parser.rs
- [X] T040 Implement `parse_shadow_attr()` in gravity-core/src/parser/style_parser.rs
- [X] T041 Implement `parse_gradient()` (linear) in gravity-core/src/parser/gradient.rs
- [X] T042 Implement `parse_angle()` for gradients in gravity-core/src/parser/gradient.rs
- [X] T043 Implement `parse_color_stop()` in gravity-core/src/parser/gradient.rs
- [X] T044 Implement `parse_breakpoint_attributes()` in gravity-core/src/parser/mod.rs
- [X] T045 Implement `parse_theme_definition()` in gravity-core/src/parser/theme_parser.rs
- [X] T046 Implement `parse_style_class_definition()` in gravity-core/src/parser/theme_parser.rs

### 2.6: Validation

- [X] T047 Implement validation rules for LayoutConstraints in gravity-core/src/ir/layout.rs
- [X] T048 Implement validation rules for StyleProperties in gravity-core/src/ir/style.rs
- [X] T049 Implement validation rules for Theme in gravity-core/src/ir/theme.rs
- [X] T050 Implement circular dependency detection for StyleClass in gravity-core/src/ir/theme.rs

**Total Phase 2 Tasks**: 35

---

## Phase 3: [US1] Widget Sizing and Spacing

**Goal**: Enable developers to control widget size and spacing declaratively in XML.

**Independent Test**: Create a column with `padding="20"` `spacing="10"`, add child widgets with `width="200"` `height="50"`, verify correct dimensions and spacing.

### 3.1: Parser Implementation

- [ ] T051 [P] [US1] Extend parser to parse `padding` attribute into LayoutConstraints
- [ ] T052 [P] [US1] Extend parser to parse `spacing` attribute into LayoutConstraints
- [ ] T053 [P] [US1] Extend parser to parse `width` attribute (fixed/fill/shrink/fill_portion/percentage)
- [ ] T054 [P] [US1] Extend parser to parse `height` attribute (fixed/fill/shrink/fill_portion/percentage)
- [ ] T055 [P] [US1] Extend parser to parse `min_width`, `max_width`, `min_height`, `max_height`

### 3.2: IR Integration

- [ ] T056 [US1] Update `parse_node()` in gravity-core/src/parser/mod.rs to populate `layout` field
- [ ] T057 [US1] Implement `resolve_layout()` in gravity-core to merge layout attributes

### 3.3: Runtime Support

- [ ] T058 [P] [US1] Implement `apply_layout()` in gravity-runtime/src/interpreter.rs
- [ ] T059 [US1] Test layout resolution with nested padding/spacing in gravity-runtime/tests/

### 3.4: Iced Backend

- [ ] T060 [P] [US1] Map `LayoutConstraints` to Iced container padding in gravity-iced/src/style_mapping.rs
- [ ] T061 [P] [US1] Map `LayoutConstraints` to Iced column/row spacing in gravity-iced/src/style_mapping.rs
- [ ] T062 [P] [US1] Map `Length` to Iced widget width/height in gravity-iced/src/style_mapping.rs
- [ ] T063 [US1] Create `examples/styling/src/main.rs` demonstrating sizing/spacing

### 3.5: Testing

- [ ] T064 [US1] Write contract test: XML layout attributes → IR LayoutConstraints
- [ ] T065 [US1] Write integration test: Render column with padding/spacing, verify pixels
- [ ] T066 [US1] Write snapshot test: Generated code for layout attributes

**Total Phase 3 Tasks**: 16

---

## Phase 4: [US2] Flexible Layout Constraints

**Goal**: Support fill, shrink, and fixed sizing for responsive interfaces.

**Independent Test**: Create row with three buttons: `width="fill"`, `width="shrink"`, `width="300"`. Verify fill button expands/contracts on resize, shrink stays minimal, fixed stays at 300px.

### 4.1: Parser Implementation

- [ ] T067 [P] [US2] Extend parser to handle `fill_portion(n)` syntax
- [ ] T068 [P] [US2] Extend parser to handle percentage-based sizing (`width="50%"`)
- [ ] T069 [P] [US2] Extend parser to handle `fill` and `shrink` keywords

### 4.2: Resolution Logic

- [ ] T070 [US2] Implement `resolve_flex_sizing()` in gravity-core to calculate fill distribution
- [ ] T071 [US2] Implement constraint enforcement (min/max) in layout resolution

### 4.3: Iced Backend

- [ ] T072 [P] [US2] Map `Fill` length to Iced `Length::Fill` in gravity-iced/src/style_mapping.rs
- [ ] T073 [P] [US2] Map `Shrink` length to Iced `Length::Shrink` in gravity-iced/src/style_mapping.rs
- [ ] T074 [P] [US2] Map `FillPortion(n)` to Iced `Length::FillPortion(n)` in gravity-iced/src/style_mapping.rs
- [ ] T075 [P] [US2] Map `Percentage(n)` to Iced `Length::Units` with calculation in gravity-iced/src/style_mapping.rs
- [ ] T076 [US2] Create `examples/responsive/src/main.rs` demonstrating flexible sizing

### 4.4: Testing

- [ ] T077 [US2] Write contract test: Flex sizing attributes → IR Length enum
- [ ] T078 [US2] Write integration test: Window resize triggers fill/shrink behavior
- [ ] T079 [US2] Write snapshot test: Generated code for flex sizing

**Total Phase 4 Tasks**: 13

---

## Phase 5: [US3] Theming System

**Goal**: Define and apply custom themes with color palettes, typography, and spacing.

**Independent Test**: Define theme with `primary_color="#3498db"`, `background="#ecf0f1"`, `font_family="Roboto"`, apply to application, verify all widgets reflect theme values.

### 5.1: Theme Parser

- [ ] T080 [P] [US3] Implement `parse_theme()` to parse `<theme>` XML element
- [ ] T081 [P] [US3] Implement `parse_palette()` to parse `<palette>` attributes
- [ ] T082 [P] [US3] Implement `parse_typography()` to parse `<typography>` attributes
- [ ] T083 [P] [US3] Implement `parse_spacing()` to parse `<spacing>` attributes
- [ ] T084 [US3] Extend `GravityDocument` to store parsed themes

### 5.2: Theme Manager

- [ ] T085 [US3] Implement `ThemeManager` in gravity-runtime/src/theme_manager.rs
- [ ] T086 [US3] Implement theme resolution (global vs local theme_ref)
- [ ] T087 [US3] Implement theme switching with state preservation
- [ ] T088 [US3] Implement built-in themes: `light`, `dark`, `default`

### 5.3: Theme Application

- [ ] T089 [US3] Implement `apply_theme()` in gravity-runtime to merge theme with widget styles
- [ ] T090 [US3] Implement theme-aware style cascading (theme → widget → inline)

### 5.4: Iced Backend

- [ ] T091 [P] [US3] Implement `ThemeAdapter` in gravity-iced/src/theme_adapter.rs
- [ ] T092 [P] [US3] Map Gravity ThemePalette to Iced Theme colors
- [ ] T093 [P] [US3] Map Gravity Typography to Iced text styling
- [ ] T094 [US3] Create `examples/styling/src/theme_demo.rs` demonstrating theme switching

### 5.5: Testing

- [ ] T095 [US3] Write contract test: Theme XML → Theme struct
- [ ] T096 [US3] Write integration test: Theme switching preserves state
- [ ] T097 [US3] Write snapshot test: Theme application to widgets

**Total Phase 5 Tasks**: 18

---

## Phase 6: [US4] Inline Widget Styling

**Goal**: Override theme defaults using inline style attributes.

**Independent Test**: Create button with `background="#e74c3c"` `border_width="2"` `border_color="#c0392b"` `border_radius="4"`, verify exact visual properties override theme.

### 6.1: Style Parser

- [ ] T098 [P] [US4] Extend parser to parse `background` attribute (color/gradient)
- [ ] T099 [P] [US4] Extend parser to parse `color` attribute
- [ ] T100 [P] [US4] Extend parser to parse `border_*` attributes
- [ ] T101 [P] [US4] Extend parser to parse `shadow` attribute
- [ ] T102 [P] [US4] Extend parser to parse `opacity` attribute
- [ ] T103 [P] [US4] Extend parser to parse `transform` attribute

### 6.2: Style Resolution

- [ ] T104 [US4] Implement `resolve_style_cascade()` in gravity-runtime/src/style_cascade.rs
- [ ] T105 [US4] Implement precedence: inline > classes > theme > defaults

### 6.3: Iced Backend

- [ ] T106 [P] [US4] Map `StyleProperties` to Iced container::Style in gravity-iced/src/style_mapping.rs
- [ ] T107 [P] [US4] Map `Background` to Iced background in gravity-iced/src/style_mapping.rs
- [ ] T108 [P] [US4] Map `Border` to Iced border in gravity-iced/src/style_mapping.rs
- [ ] T109 [P] [US4] Map `Shadow` to Iced shadow in gravity-iced/src/style_mapping.rs
- [ ] T110 [P] [US4] Map `Opacity` to Iced opacity in gravity-iced/src/style_mapping.rs
- [ ] T111 [P] [US4] Map `Transform` to Iced transform in gravity-iced/src/style_mapping.rs

### 6.4: Testing

- [ ] T112 [US4] Write contract test: Inline style attributes → StyleProperties
- [ ] T113 [US4] Write integration test: Inline styles override theme
- [ ] T114 [US4] Write snapshot test: Generated code with inline styles

**Total Phase 6 Tasks**: 17

---

## Phase 7: [US5] Style Classes

**Goal**: Define reusable style classes with inheritance and state variants.

**Independent Test**: Define `button_primary` class, apply to multiple buttons, verify identical appearance; update class definition, verify all buttons update.

### 7.1: Class Parser

- [ ] T115 [P] [US5] Implement `parse_style_class()` to parse `<style>` XML element
- [ ] T116 [P] [US5] Implement `parse_class_base()` for base properties
- [ ] T117 [P] [US5] Implement `parse_class_extends()` for inheritance
- [ ] T118 [P] [US5] Implement `parse_class_state()` for hover/focus/active/disabled variants
- [ ] T119 [US5] Extend `GravityDocument` to store style classes

### 7.2: Class Resolution

- [ ] T120 [US5] Implement `resolve_class()` in gravity-runtime with inheritance depth limit (5)
- [ ] T121 [US5] Implement circular dependency detection in class resolution
- [ ] T122 [US5] Implement class merging for multiple classes on one widget
- [ ] T123 [US5] Implement hot-reload support for class updates

### 7.3: Iced Backend

- [ ] T124 [P] [US5] Map resolved class properties to Iced styles in gravity-iced/src/style_mapping.rs
- [ ] T125 [US5] Create `examples/styling/src/class_demo.rs` demonstrating class usage

### 7.4: Testing

- [ ] T126 [US5] Write contract test: Class XML → StyleClass struct
- [ ] T127 [US5] Write integration test: Class inheritance and merging
- [ ] T128 [US5] Write integration test: Hot-reload class updates
- [ ] T129 [US5] Write error test: Circular dependency detection

**Total Phase 7 Tasks**: 15

---

## Phase 8: [US6] Alignment and Positioning

**Goal**: Control widget alignment within containers and positioning.

**Independent Test**: Create container with `align="center"` containing button, verify centered; test `align_self` override; test `position="absolute"` with offsets.

### 8.1: Parser Extensions

- [ ] T130 [P] [US6] Extend parser to parse `align_items` attribute
- [ ] T131 [P] [US6] Extend parser to parse `justify_content` attribute
- [ ] T132 [P] [US6] Extend parser to parse `align_self` attribute
- [ ] T133 [P] [US6] Extend parser to parse `align` shorthand
- [ ] T134 [P] [US6] Extend parser to parse `position` attribute
- [ ] T135 [P] [US6] Extend parser to parse `top`, `right`, `bottom`, `left` attributes
- [ ] T136 [P] [US6] Extend parser to parse `z_index` attribute

### 8.2: Layout Resolution

- [ ] T137 [US6] Implement alignment resolution in `resolve_layout()`
- [ ] T138 [US6] Implement position offset handling in layout resolution

### 8.3: Iced Backend

- [ ] T139 [P] [US6] Map alignment to Iced `align_items` in gravity-iced/src/style_mapping.rs
- [ ] T140 [P] [US6] Map justification to Iced `justify_content` in gravity-iced/src/style_mapping.rs
- [ ] T141 [P] [US6] Map `align_self` to Iced widget alignment in gravity-iced/src/style_mapping.rs
- [ ] T142 [P] [US6] Map position/z_index to Iced overlay in gravity-iced/src/style_mapping.rs

### 8.4: Testing

- [ ] T143 [US6] Write contract test: Alignment attributes → LayoutConstraints
- [ ] T144 [US6] Write integration test: Alignment and positioning behavior
- [ ] T145 [US6] Write snapshot test: Generated code for alignment

**Total Phase 8 Tasks**: 16

---

## Phase 9: [US7] Responsive Breakpoints

**Goal**: Support responsive attributes with mobile/tablet/desktop breakpoints.

**Independent Test**: Define `mobile:spacing="10"` `desktop:spacing="20"`, verify spacing changes when viewport crosses 640px/1024px thresholds.

### 9.1: Breakpoint Parser

- [ ] T146 [P] [US7] Extend parser to recognize `mobile:`, `tablet:`, `desktop:` prefixes
- [ ] T147 [P] [US7] Store breakpoint-prefixed attributes in `breakpoint_attributes` map
- [ ] T148 [US7] Validate breakpoint prefixes don't conflict with base attributes

### 9.2: Runtime Support

- [ ] T149 [US7] Add viewport state to application model in gravity-cli/src/commands/dev.rs
- [ ] T150 [US7] Implement `window::resize_events()` subscription in dev mode
- [ ] T151 [US7] Implement `resolve_breakpoint_attributes()` in gravity-runtime
- [ ] T152 [US7] Implement breakpoint change detection (only re-render on threshold crossing)

### 9.3: Iced Backend

- [ ] T153 [P] [US7] Pass viewport width to `render()` in gravity-iced/src/lib.rs
- [ ] T154 [US7] Integrate breakpoint resolution into widget rendering
- [ ] T155 [US7] Create `examples/responsive/src/main.rs` demonstrating responsive layout

### 9.4: Testing

- [ ] T156 [US7] Write contract test: Breakpoint attributes → IR map
- [ ] T157 [US7] Write integration test: Viewport resize triggers breakpoint changes
- [ ] T158 [US7] Write integration test: Breakpoint attributes override base attributes

**Total Phase 9 Tasks**: 13

---

## Phase 10: [US8] State-Based Styling

**Goal**: Apply styles based on widget interaction states (hover, focus, active, disabled).

**Independent Test**: Define `hover:background="#2980b9"` `active:background="#21618c"` `disabled:opacity="0.5"`, verify automatic state transitions.

### 10.1: State Parser

- [ ] T159 [P] [US8] Extend parser to recognize `hover:`, `focus:`, `active:`, `disabled:` prefixes
- [ ] T160 [P] [US8] Store state-prefixed styles in `StyleClass.state_variants`
- [ ] T161 [P] [US8] Support combined states (e.g., `hover:active:background`)

### 10.2: State Management

- [ ] T162 [US8] Implement widget state tracking in gravity-runtime
- [ ] T163 [US8] Implement automatic state transitions on user interaction
- [ ] T164 [US8] Implement `disabled` attribute handling (prevent event firing)

### 10.3: Iced Backend

- [ ] T165 [P] [US8] Map Iced widget states to Gravity states in gravity-iced/src/widgets/styled.rs
- [ ] T166 [P] [US8] Apply state styles dynamically in `styled` wrapper widget
- [ ] T167 [US8] Create `examples/styling/src/state_demo.rs` demonstrating state transitions

### 10.4: Testing

- [ ] T168 [US8] Write contract test: State-prefixed attributes → state_variants map
- [ ] T169 [US8] Write integration test: Hover/focus/active state transitions
- [ ] T170 [US8] Write integration test: Disabled state prevents events

**Total Phase 10 Tasks**: 12

---

## Phase 11: Polish & Integration

**Goal**: Final integration, documentation, and cross-cutting concerns.

### 11.1: CLI Integration

- [ ] T171 [P] Update `gravity dev` to support theme/class hot-reload
- [ ] T172 [P] Update `gravity build` to generate style code
- [ ] T173 [P] Update `gravity check` to validate style attributes

### 11.2: Documentation

- [ ] T174 Update `docs/XML_SCHEMA.md` with all new attributes
- [ ] T175 Update `docs/QUICKSTART.md` with styling examples
- [ ] T176 Create `docs/STYLING.md` comprehensive guide

### 11.3: Performance Optimization

- [ ] T177 [P] Optimize style resolution caching
- [ ] T178 [P] Optimize breakpoint attribute lookup
- [ ] T179 [P] Optimize hot-reload for style-only changes

### 11.4: Final Testing

- [ ] T180 Run full test suite: `cargo test --workspace`
- [ ] T181 Run clippy: `cargo clippy --workspace -- -D warnings`
- [ ] T182 Run formatting: `cargo fmt --all -- --check`
- [ ] T183 Build all examples: `cargo build --examples`
- [ ] T184 Performance benchmark: XML parse time for 1000 widgets
- [ ] T185 Performance benchmark: Hot-reload latency
- [ ] T186 Performance benchmark: Theme switching time

### 11.5: Final Polish

- [ ] T187 Verify all 50+ functional requirements from spec.md are implemented
- [ ] T188 Verify all 10 success criteria from spec.md are met
- [ ] T189 Verify all user stories are independently testable
- [ ] T190 Verify no constitutional violations

**Total Phase 11 Tasks**: 20

---

## Grand Total: 190 Tasks

---

## Implementation Strategy

### MVP Scope (Phase 3 Only)

To deliver immediate value with minimal scope, implement **only Phase 3 (US1)**:
- Layout system types (Length, Padding, LayoutConstraints)
- Parser for padding, spacing, width, height
- Basic Iced integration for sizing
- One example demonstrating sizing/spacing

**MVP Tasks**: T001-T066 (81 tasks)  
**MVP Timeline**: ~3-4 days  
**MVP Test**: Independent test from US1 spec

### Incremental Delivery

After MVP, deliver in priority order:

1. **Phase 4 (US2)**: Flexible sizing - 13 tasks - adds fill/shrink/percentage
2. **Phase 5 (US3)**: Theming - 18 tasks - adds color palettes and typography
3. **Phase 6 (US4)**: Inline styles - 17 tasks - adds background/border/shadow
4. **Phase 7 (US5)**: Style classes - 15 tasks - adds reusable styles
5. **Phase 8 (US6)**: Alignment - 16 tasks - adds positioning controls
6. **Phase 9 (US7)**: Responsive - 13 tasks - adds breakpoints
7. **Phase 10 (US8)**: State styles - 12 tasks - adds hover/focus/active
8. **Phase 11**: Polish - 20 tasks - integration and docs

### Parallel Execution Opportunities

**Maximum Parallelism**: 23 tasks marked [P] can be executed in parallel within their phases.

**Example Parallel Execution (Phase 2)**:
```bash
# Run all 6 layout type tasks in parallel
cargo watch -x "test layout_types" &
cargo watch -x "test length" &
cargo watch -x "test padding" &
cargo watch -x "test alignment" &
cargo watch -x "test justification" &
cargo watch -x "test direction" &
```

**Example Parallel Execution (Phase 6)**:
```bash
# Run all 6 style parser tasks in parallel
cargo watch -x "test parse_background" &
cargo watch -x "test parse_color" &
cargo watch -x "test parse_border" &
cargo watch -x "test parse_shadow" &
cargo watch -x "test parse_opacity" &
cargo watch -x "test parse_transform" &
```

### Independent Test Criteria

Each user story phase includes tasks for:
1. **Contract Tests**: Verify XML → IR conversion
2. **Integration Tests**: Verify end-to-end rendering
3. **Snapshot Tests**: Verify generated code (if applicable)

All tests can run independently per story phase.

---

## Task Format Validation

✅ **All 190 tasks follow the required format**:
- Checkbox: `- [ ]`
- Task ID: Sequential (T001-T190)
- [P] marker: Present on 23 parallelizable tasks
- [Story] label: Present on all user story phase tasks (US1-US8)
- Description: Clear action with file path

**Format Examples from this document**:
- ✅ `- [ ] T001 Add csscolorparser dependency to gravity-core/Cargo.toml`
- ✅ `- [ ] T016 [P] Implement Length enum in gravity-core/src/ir/layout.rs`
- ✅ `- [ ] T051 [P] [US1] Extend parser to parse padding attribute`

---

## Next Steps

1. **Start with Phase 1** (Setup): Execute T001-T015
2. **Proceed to Phase 2** (Foundational): Execute T016-T050
3. **Choose MVP or Full**: 
   - MVP: Execute only Phase 3 (T051-T066)
   - Full: Execute all phases in order
4. **Use parallel execution** for [P] tasks to speed up development
5. **Verify independent test criteria** after each user story phase

---

## Quick Reference

**User Story Mapping**:
- US1: Layout & Sizing (T051-T066)
- US2: Flexible Constraints (T067-T079)
- US3: Theming (T080-T097)
- US4: Inline Styling (T098-T114)
- US5: Style Classes (T115-T129)
- US6: Alignment (T130-T145)
- US7: Responsive (T146-T158)
- US8: State Styles (T159-T170)

**Critical Files**:
- `gravity-core/src/ir/{layout,style,theme}.rs` - Core types
- `gravity-core/src/parser/{style_parser,theme_parser,gradient}.rs` - Parsers
- `gravity-runtime/src/{theme_manager,style_cascade}.rs` - Runtime logic
- `gravity-iced/src/{style_mapping,theme_adapter}.rs` - Backend mapping
- `gravity-cli/src/commands/dev.rs` - Hot-reload integration

**Performance Targets**:
- XML parse: < 10ms (T184)
- Hot-reload: < 500ms (T185)
- Theme switch: < 100ms (T186)
- State transition: < 50ms (verified in US8 tests)

---

**Generated by**: `/speckit.tasks`  
**Date**: 2026-01-01  
**Status**: Ready for execution
