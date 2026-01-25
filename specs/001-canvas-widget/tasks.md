# Tasks: Canvas Widget

**Input**: Design documents from `/specs/001-canvas-widget/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4, US5)
- Exact file paths included in descriptions

---

## Phase 1: Setup

**Purpose**: Extend existing project structure for canvas feature

- [x] T001 Add canvas module directory structure in crates/dampen-iced/src/canvas/
- [x] T002 [P] Add WidgetKind variants (CanvasRect, CanvasCircle, CanvasLine, CanvasText, CanvasGroup) in crates/dampen-core/src/ir/node.rs
- [x] T003 [P] Add EventKind variants (CanvasClick, CanvasDrag, CanvasMove, CanvasRelease) in crates/dampen-core/src/ir/node.rs
- [x] T004 [P] Create CanvasEvent and CanvasEventKind types in crates/dampen-core/src/handler/mod.rs
- [x] T005 [P] Update WidgetKind::all_standard() to include canvas shape tag names in crates/dampen-core/src/ir/node.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types and infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Shape Types (dampen-core)

- [x] T006 [P] Define RectShape struct with all attributes (x, y, width, height, fill, stroke, stroke_width, radius) in crates/dampen-core/src/ir/node.rs
- [x] T007 [P] Define CircleShape struct with all attributes (cx, cy, radius, fill, stroke, stroke_width) in crates/dampen-core/src/ir/node.rs
- [x] T008 [P] Define LineShape struct with all attributes (x1, y1, x2, y2, stroke, stroke_width) in crates/dampen-core/src/ir/node.rs
- [x] T009 [P] Define TextShape struct with all attributes (x, y, content, size, color) in crates/dampen-core/src/ir/node.rs
- [x] T010 [P] Define GroupShape struct with transform and children in crates/dampen-core/src/ir/node.rs
- [x] T011 [P] Define Transform enum (Translate, Rotate, Scale, ScaleXY, Matrix) in crates/dampen-core/src/ir/node.rs
- [x] T012 Define CanvasShape enum unifying all shape types in crates/dampen-core/src/ir/node.rs

### Schema Definitions

- [x] T013 [P] Add WidgetSchema for CanvasRect (required: x, y, width, height; optional: fill, stroke, stroke_width, radius) in crates/dampen-core/src/schema/mod.rs
- [x] T014 [P] Add WidgetSchema for CanvasCircle (required: cx, cy, radius; optional: fill, stroke, stroke_width) in crates/dampen-core/src/schema/mod.rs
- [x] T015 [P] Add WidgetSchema for CanvasLine (required: x1, y1, x2, y2; optional: stroke, stroke_width) in crates/dampen-core/src/schema/mod.rs
- [x] T016 [P] Add WidgetSchema for CanvasText (required: x, y; optional: size, color) in crates/dampen-core/src/schema/mod.rs
- [x] T017 [P] Add WidgetSchema for CanvasGroup (optional: transform) in crates/dampen-core/src/schema/mod.rs
- [x] T018 Update Canvas WidgetSchema to add on_click, on_drag, on_move, on_release events in crates/dampen-core/src/schema/mod.rs

### Handler System Extension

- [x] T019 Add WithCanvasEvent variant to HandlerEntry enum in crates/dampen-core/src/handler/mod.rs
- [x] T020 Implement register_canvas_event() method on HandlerRegistry in crates/dampen-core/src/handler/mod.rs
- [x] T021 Implement dispatch_canvas_event() method on HandlerRegistry in crates/dampen-core/src/handler/mod.rs

**Checkpoint**: Foundation ready - canvas types, schemas, and handler infrastructure complete

---

## Phase 3: User Story 1 - Draw Static Shapes Declaratively (Priority: P1) 🎯 MVP

**Goal**: Developers can define shapes (rect, circle, line, text, group) in XML and see them rendered

**Independent Test**: Create a .dampen file with canvas containing basic shapes and verify visual output

### Parser for User Story 1

- [x] T022 [US1] Implement parse_canvas_child() to detect shape elements inside canvas in crates/dampen-core/src/parser/mod.rs
- [x] T023 [US1] Implement parse_rect_shape() to extract rect attributes in crates/dampen-core/src/parser/mod.rs
- [x] T024 [US1] Implement parse_circle_shape() to extract circle attributes in crates/dampen-core/src/parser/mod.rs
- [x] T025 [US1] Implement parse_line_shape() to extract line attributes in crates/dampen-core/src/parser/mod.rs
- [x] T026 [US1] Implement parse_canvas_text() to extract text attributes and content in crates/dampen-core/src/parser/mod.rs
- [x] T027 [US1] Implement parse_group_shape() with transform parsing and recursive children in crates/dampen-core/src/parser/mod.rs
- [x] T028 [US1] Implement parse_transform() for translate/rotate/scale/matrix syntax in crates/dampen-core/src/parser/mod.rs
- [x] T029 [US1] Add validation: shape elements only valid inside canvas in crates/dampen-core/src/parser/mod.rs

### Canvas Module (dampen-iced)

- [x] T030 [P] [US1] Create shapes.rs with CanvasShape enum and shape structs (runtime types) in crates/dampen-iced/src/canvas/shapes.rs
- [x] T031 [P] [US1] Create mod.rs exporting shapes module in crates/dampen-iced/src/canvas/mod.rs
- [x] T032 [US1] Create program.rs with DeclarativeProgram struct implementing canvas::Program in crates/dampen-iced/src/canvas/program.rs
- [x] T033 [US1] Implement draw() method on DeclarativeProgram - iterate shapes and draw to Frame in crates/dampen-iced/src/canvas/program.rs
- [x] T034 [US1] Implement draw_rect() using Path::rectangle and Path::rounded_rectangle in crates/dampen-iced/src/canvas/program.rs
- [x] T035 [US1] Implement draw_circle() using Path::circle in crates/dampen-iced/src/canvas/program.rs
- [x] T036 [US1] Implement draw_line() using Path::line in crates/dampen-iced/src/canvas/program.rs
- [x] T037 [US1] Implement draw_text() using frame.fill_text() in crates/dampen-iced/src/canvas/program.rs
- [x] T038 [US1] Implement draw_group() with frame.with_translation/rotation/scale for transforms in crates/dampen-iced/src/canvas/program.rs
- [x] T039 [US1] Add cache field to DeclarativeProgram using iced::widget::canvas::Cache in crates/dampen-iced/src/canvas/program.rs

### Builder Integration for User Story 1

- [x] T040 [US1] Replace placeholder in build_canvas() to create DeclarativeProgram from parsed shapes in crates/dampen-iced/src/builder/widgets/canvas.rs
- [x] T041 [US1] Implement parse_canvas_shapes() to convert WidgetNode children to CanvasShape in crates/dampen-iced/src/builder/widgets/canvas.rs
- [x] T042 [US1] Export canvas module from dampen-iced lib.rs in crates/dampen-iced/src/lib.rs

### Example for User Story 1

- [x] T043 [P] [US1] Create examples/canvas-demo/Cargo.toml with dampen dependencies
- [x] T044 [P] [US1] Create examples/canvas-demo/src/ui/window.dampen with static shapes (rect, circle, line, text, group)
- [x] T045 [US1] Create examples/canvas-demo/src/main.rs with minimal model and application setup

**Checkpoint**: User Story 1 complete - static declarative shapes render correctly

---

## Phase 4: User Story 2 - Dynamic Shape Binding (Priority: P2)

**Goal**: Shape attributes can be bound to model state and update reactively

**Independent Test**: Create canvas with bound attributes, modify model, verify shapes update

### Binding Evaluation for User Story 2

- [x] T046 [US2] Extend parse_*_shape() functions to detect binding expressions in attributes in crates/dampen-core/src/parser/mod.rs
- [x] T047 [US2] Implement evaluate_shape_attributes() to resolve bindings at runtime in crates/dampen-iced/src/builder/widgets/canvas.rs
- [x] T048 [US2] Implement cache invalidation when bound values change in crates/dampen-iced/src/canvas/program.rs
- [x] T049 [US2] Add support for for-each loops inside canvas (iterating collections) in crates/dampen-iced/src/builder/widgets/canvas.rs
- [x] T050 [US2] Handle empty collections in for-each gracefully in crates/dampen-iced/src/builder/widgets/canvas.rs

### Example Update for User Story 2

- [x] T051 [US2] Add dynamic binding example to canvas-demo (bound circle position) in examples/canvas-demo/src/ui/window.dampen
- [x] T052 [US2] Update canvas-demo model with bindable state fields in examples/canvas-demo/src/main.rs

**Checkpoint**: User Story 2 complete - bound shapes update reactively

---

## Phase 5: User Story 3 - Canvas Interaction Events (Priority: P2)

**Goal**: Canvas emits click, drag, move, release events with coordinates to handlers

**Independent Test**: Define event handlers, interact with canvas, verify handlers receive correct coordinates

### Event Handling for User Story 3

- [x] T053 [P] [US3] Create events.rs with CanvasEventHandlers struct in crates/dampen-iced/src/canvas/events.rs
- [x] T054 [US3] Add CanvasState struct with is_dragging, last_position fields in crates/dampen-iced/src/canvas/events.rs
- [x] T055 [US3] Implement update() method on DeclarativeProgram for mouse event handling in crates/dampen-iced/src/canvas/program.rs
- [x] T056 [US3] Implement click event detection (ButtonPressed) with position_in(bounds) in crates/dampen-iced/src/canvas/program.rs
- [x] T057 [US3] Implement drag event detection (CursorMoved while dragging) with delta calculation in crates/dampen-iced/src/canvas/program.rs
- [x] T058 [US3] Implement move event detection (CursorMoved without button) in crates/dampen-iced/src/canvas/program.rs
- [x] T059 [US3] Implement release event detection (ButtonReleased) in crates/dampen-iced/src/canvas/program.rs
- [x] T060 [US3] Implement mouse_interaction() to return appropriate cursor (Crosshair when over canvas) in crates/dampen-iced/src/canvas/program.rs

### Builder Event Integration for User Story 3

- [x] T061 [US3] Parse on_click, on_drag, on_move, on_release attributes in build_canvas() in crates/dampen-iced/src/builder/widgets/canvas.rs
- [x] T062 [US3] Wire parsed event handlers to DeclarativeProgram event handlers in crates/dampen-iced/src/builder/widgets/canvas.rs
- [x] T063 [US3] Dispatch CanvasEvent through HandlerRegistry when events fire in crates/dampen-iced/src/canvas/program.rs

### Example Update for User Story 3

- [x] T064 [US3] Add interactive canvas example with on_click handler in examples/canvas-demo/src/ui/window.dampen
- [x] T065 [US3] Implement click handler in canvas-demo that adds points on click in examples/canvas-demo/src/main.rs

**Checkpoint**: User Story 3 complete - canvas events work with coordinates

---

## Phase 6: User Story 4 - Custom Drawing Program (Priority: P3)

**Goal**: Advanced users can bind custom Program implementations to canvas

**Independent Test**: Create custom Program impl, bind to canvas via program attribute, verify rendering

### Custom Program Support for User Story 4

- [x] T066 [US4] Implement program attribute binding resolution in build_canvas() in crates/dampen-iced/src/builder/widgets/canvas.rs
- [x] T067 [US4] Add CanvasContent enum (Declarative | Custom) to support both modes in crates/dampen-iced/src/canvas/program.rs
- [x] T068 [US4] Implement wrapper that allows custom Program with event handlers in crates/dampen-iced/src/canvas/program.rs
- [x] T069 [US4] Add validation: warn if canvas has both program and shape children in crates/dampen-core/src/parser/mod.rs

### Example for User Story 4

- [x] T070 [P] [US4] Add custom program example to canvas-demo in examples/canvas-demo/src/main.rs
- [x] T071 [US4] Create canvas with program binding in examples/canvas-demo/src/ui/window.dampen

**Checkpoint**: User Story 4 complete - custom programs can be bound to canvas

---

## Phase 7: User Story 5 - Development Mode Hot-Reload (Priority: P3)

**Goal**: Canvas changes in interpreted mode reflect immediately without restart

**Independent Test**: Run dampen run, modify canvas XML, verify changes appear instantly

### Hot-Reload Support for User Story 5

- [x] T072 [US5] Verify canvas shapes re-parse on file change in interpreted mode in crates/dampen-iced/src/builder/widgets/canvas.rs
- [x] T073 [US5] Ensure DeclarativeProgram cache clears on document reload in crates/dampen-iced/src/canvas/program.rs
- [x] T074 [US5] Test hot-reload with canvas-demo example in interpreted mode

**Checkpoint**: User Story 5 complete - hot-reload works for canvas

---

## Phase 8: Codegen Mode Support

**Purpose**: Production build compiles canvas to Rust code

- [x] T075 Implement generate_canvas() for codegen in crates/dampen-core/src/codegen/view.rs
- [x] T076 Implement generate_declarative_canvas() to emit shape construction code in crates/dampen-core/src/codegen/view.rs
- [x] T077 [P] Implement generate_rect_shape() to emit RectShape literal in crates/dampen-core/src/codegen/view.rs
- [x] T078 [P] Implement generate_circle_shape() to emit CircleShape literal in crates/dampen-core/src/codegen/view.rs
- [x] T079 [P] Implement generate_line_shape() to emit LineShape literal in crates/dampen-core/src/codegen/view.rs
- [x] T080 [P] Implement generate_text_shape() to emit TextShape literal in crates/dampen-core/src/codegen/view.rs
- [x] T081 [P] Implement generate_group_shape() to emit GroupShape with children in crates/dampen-core/src/codegen/view.rs
- [x] T082 Handle binding expressions in shape attributes for codegen in crates/dampen-core/src/codegen/view.rs
- [x] T083 Implement generate_custom_canvas() for program binding in codegen in crates/dampen-core/src/codegen/view.rs
- [x] T084 Test canvas-demo with dampen build (codegen mode)

**Checkpoint**: Codegen mode produces working canvas code

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Quality, documentation, and validation

- [x] T085 [P] Add rustdoc comments to all public types in canvas module in crates/dampen-iced/src/canvas/
- [x] T086 [P] Add rustdoc comments to shape types in dampen-core in crates/dampen-core/src/ir/node.rs
- [x] T087 [P] Add error messages with suggestions for invalid canvas/shape attributes in crates/dampen-core/src/parser/mod.rs
- [x] T088 Add edge case handling: negative dimensions, out-of-range coordinates in crates/dampen-core/src/parser/mod.rs
- [x] T089 Add edge case handling: empty canvas (no children, no program) in crates/dampen-iced/src/builder/widgets/canvas.rs
- [x] T090 Update canvas-demo README.md with usage examples in examples/canvas-demo/README.md
- [x] T091 Run cargo clippy --workspace -- -D warnings and fix any issues
- [x] T092 Run cargo fmt --all and verify formatting
- [x] T093 Run cargo test --workspace and verify all tests pass
- [x] T094 Run quickstart.md validation scenarios manually
