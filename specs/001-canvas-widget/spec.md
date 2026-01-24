# Feature Specification: Canvas Widget

**Feature Branch**: `001-canvas-widget`  
**Created**: 2026-01-24  
**Status**: Draft  
**Input**: User description: "Implement Canvas Widget for Dampen with hybrid support for declarative XML shapes and custom Rust programs, plus complete event handling"

## Clarifications

### Session 2026-01-24

- Q: How should v1.1 versioning be signaled in `.dampen` files for backward compatibility? → A: Explicit root attribute `<dampen version="1.1">` required.
- Q: What behavior for invalid shape attribute values (negative width, radius < 0)? → A: Clamp to valid minimum + emit warning.
- Q: Should individual shapes support their own event handlers, or only canvas as a whole? → A: Canvas-level events only.
- Q: How should an empty canvas (no children, no program) behave? → A: Render empty area with optional background.
- Q: When declarative shapes and a custom program are both specified on the same canvas, what happens? → A: Mutually exclusive; parse error if both present.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Draw Static Shapes Declaratively (Priority: P1)

As a Dampen application developer, I want to define graphical shapes (rectangles, circles, lines, text) directly in my XML UI definition so that I can create visual diagrams, icons, and decorative elements without writing custom drawing code.

**Why this priority**: This is the foundational capability that enables all canvas-based UI creation. Without declarative shapes, the canvas widget provides no immediate value to developers.

**Independent Test**: Can be fully tested by creating a `.dampen` file with a canvas containing basic shapes and verifying they render correctly. Delivers immediate visual output that can be validated.

**Acceptance Scenarios**:

1. **Given** a canvas element with a rect child element specifying position, size, and fill color, **When** the application renders, **Then** a rectangle appears at the specified position with the specified dimensions and color.

2. **Given** a canvas element with a circle child element specifying center coordinates, radius, and fill color, **When** the application renders, **Then** a circle appears centered at the specified position with the specified radius and color.

3. **Given** a canvas element with a line child element specifying start and end coordinates and stroke color, **When** the application renders, **Then** a line appears connecting the two points with the specified color.

4. **Given** a canvas element with a text child element specifying position, content, size, and color, **When** the application renders, **Then** the text appears at the specified position with the specified styling.

5. **Given** a canvas element with nested shapes inside a group element with a transform attribute, **When** the application renders, **Then** all shapes within the group are transformed (translated, rotated, or scaled) together.

---

### User Story 2 - Dynamic Shape Binding (Priority: P2)

As a Dampen application developer, I want to bind shape attributes (position, color, size) to my application state so that shapes update automatically when data changes, enabling dynamic visualizations like charts, animations, and interactive graphics.

**Why this priority**: Dynamic binding transforms static drawings into reactive visualizations. This is essential for data-driven applications but depends on the foundational static shape support.

**Independent Test**: Can be tested by creating a canvas with bound attributes, modifying the application state, and verifying the shapes update to reflect the new values.

**Acceptance Scenarios**:

1. **Given** a circle element with position attributes bound to application state values, **When** the state values change, **Then** the circle moves to the new position without requiring a full re-render.

2. **Given** a rect element with fill color bound to an application state value, **When** the state value changes to a different color, **Then** the rectangle updates to display the new color.

3. **Given** a canvas with a for-each loop iterating over a collection in application state, **When** items are added to or removed from the collection, **Then** the corresponding shapes appear or disappear accordingly.

---

### User Story 3 - Canvas Interaction Events (Priority: P2)

As a Dampen application developer, I want to handle user interactions (clicks, drags, mouse movement) on the canvas with coordinate information so that I can build interactive drawing tools, drag-and-drop interfaces, and click-based selection.

**Why this priority**: Interactivity is essential for many canvas use cases (drawing apps, games, interactive charts). Ranked equally with dynamic binding as both extend the core functionality in different directions.

**Independent Test**: Can be tested by defining event handlers, performing mouse interactions on the canvas, and verifying the handlers receive the correct event data including coordinates.

**Acceptance Scenarios**:

1. **Given** a canvas element with a click event handler, **When** the user clicks on the canvas, **Then** the handler is invoked with the x and y coordinates of the click relative to the canvas.

2. **Given** a canvas element with a drag event handler, **When** the user clicks and drags on the canvas, **Then** the handler is invoked with the current position and the delta (change) in position since the last event.

3. **Given** a canvas element with a move event handler, **When** the user moves the mouse over the canvas, **Then** the handler is invoked with the current cursor position.

4. **Given** a canvas element with a release event handler, **When** the user releases a mouse button after clicking on the canvas, **Then** the handler is invoked with the final coordinates.

---

### User Story 4 - Custom Drawing Program (Priority: P3)

As an advanced Dampen application developer, I want to bind a custom drawing program to the canvas so that I can implement complex visualizations (charts, graphs, procedural graphics) that cannot be expressed as declarative shapes.

**Why this priority**: This is an advanced escape hatch for power users. Most applications will use declarative shapes; custom programs are for specialized use cases requiring maximum flexibility.

**Independent Test**: Can be tested by creating a custom program in application code, binding it to a canvas, and verifying the program's draw method is called with the correct context.

**Acceptance Scenarios**:

1. **Given** a canvas element with a program attribute bound to an application-provided drawing program, **When** the application renders, **Then** the canvas delegates all drawing to the custom program.

2. **Given** a custom program bound to a canvas that also has event handlers defined, **When** the user interacts with the canvas, **Then** both the custom program and the event handlers can respond to events.

---

### User Story 5 - Development Mode Hot-Reload (Priority: P3)

As a Dampen application developer, I want canvas changes to hot-reload during development so that I can iterate quickly on visual designs without restarting the application.

**Why this priority**: Hot-reload is a developer experience enhancement. Essential for productivity but not for the core functionality of the canvas widget.

**Independent Test**: Can be tested by running the application in development mode, modifying canvas XML, and verifying changes appear without application restart.

**Acceptance Scenarios**:

1. **Given** an application running in interpreted mode with a canvas definition, **When** the developer modifies the canvas XML (adds, removes, or changes shapes), **Then** the changes are reflected in the running application immediately.

---

### Edge Cases

- **Invalid shape attributes**: When a shape has invalid attribute values (negative dimensions, radius < 0), the system clamps values to the valid minimum (e.g., 0) and emits a warning. The shape still renders.
- **Empty canvas**: A canvas with no children and no bound program renders as an empty area with the specified dimensions. An optional `background` attribute can set the fill color. No warning is emitted.
- What happens when a bound collection for for-each is empty?
- How does the system handle overlapping shapes (z-order/draw order)?
- What happens when event coordinates fall outside the canvas bounds?
- How does the system handle rapid sequential events (e.g., fast mouse movement)?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support a canvas element that defines a rectangular drawing area with configurable width, height, and optional background color.
- **FR-002**: System MUST support rect shape elements with position (x, y), dimensions (width, height), fill color, stroke color, stroke width, and corner radius attributes.
- **FR-003**: System MUST support circle shape elements with center position (cx, cy), radius, fill color, stroke color, and stroke width attributes.
- **FR-004**: System MUST support line shape elements with start position (x1, y1), end position (x2, y2), stroke color, and stroke width attributes.
- **FR-005**: System MUST support text shape elements with position (x, y), content, font size, and color attributes.
- **FR-006**: System MUST support group elements that can contain multiple shapes and apply transformations (translate, rotate, scale) to all children.
- **FR-007**: System MUST support binding shape attributes to application state using the existing binding expression syntax.
- **FR-008**: System MUST support iteration over collections to generate multiple shapes using for-each elements.
- **FR-009**: System MUST support click events that provide x and y coordinates relative to the canvas.
- **FR-010**: System MUST support drag events that provide current position and delta values.
- **FR-011**: System MUST support move events that provide current cursor position.
- **FR-012**: System MUST support release events that provide final coordinates.
- **FR-013**: System MUST support binding a custom drawing program to a canvas via the program attribute. A canvas with a program attribute MUST NOT contain declarative shape children; specifying both is a parse error.
- **FR-014**: System MUST render shapes in document order (later shapes appear on top of earlier shapes).
- **FR-015**: System MUST validate that shape elements only appear as children of canvas elements.
- **FR-016**: System MUST support canvas rendering in both interpreted mode (hot-reload) and codegen mode (compiled).
- **FR-017**: System MUST provide clear error messages when canvas or shape definitions contain invalid attributes.
- **FR-018**: System MUST validate document version; canvas elements require `<dampen version="1.1">` and MUST produce a clear error if used in v1.0 documents.

### Key Entities

- **Canvas**: A drawing surface with defined dimensions that operates in one of two mutually exclusive modes: (1) declarative mode with shape children, or (2) program mode with a bound custom drawing program. Event handlers (click, drag, move, release) are defined at the canvas level only.
- **Shape**: A visual element (rectangle, circle, line, text) with position, styling, and optional dynamic bindings. Shapes exist only within a canvas context. Shapes do not have individual event handlers; all interaction is handled at the canvas level.
- **Group**: A container for shapes that applies a common transformation to all children.
- **Canvas Event**: An interaction event (click, drag, move, release) with associated coordinate data relative to the canvas origin.
- **Drawing Program**: An application-provided component that handles custom canvas rendering when declarative shapes are insufficient.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can create a canvas with multiple shapes in under 5 minutes using only XML, without writing any drawing code.
- **SC-002**: Canvas with 100 shapes renders without perceptible delay (under 16ms frame time for 60fps).
- **SC-003**: Shape attribute changes via bindings reflect visually within 1 frame (under 16ms).
- **SC-004**: Event handlers receive coordinate data accurate to within 1 pixel of actual cursor position.
- **SC-005**: Canvas hot-reload in development mode reflects changes within 500ms of file save.
- **SC-006**: All canvas features work identically in both interpreted and codegen modes (visual parity).
- **SC-007**: Invalid canvas/shape definitions produce error messages that identify the specific problem and location.

## Assumptions

- Canvas is a v1.1 widget; `.dampen` files using canvas MUST declare `<dampen version="1.1">` at the root element. Documents without version or with `version="1.0"` will reject canvas elements with a clear error message.
- The existing Dampen binding expression system supports the attribute types needed for canvas shapes (numbers, colors, strings).
- The underlying rendering framework provides canvas/drawing primitives that support the required shapes and transformations.
- Coordinate systems follow standard screen conventions (origin at top-left, y increasing downward).
- Colors are specified using standard web color formats (hex codes like #RRGGBB or #RRGGBBAA).
- Shape stroke widths are specified in pixels.
- Text rendering uses the application's default font unless explicitly configured.
