# Feature Specification: Layout, Sizing, Theming, and Styling System

**Feature Branch**: `002-layout-theming-styling`  
**Created**: 2026-01-01  
**Status**: Draft  
**Input**: User description: "Ajouter dans le projet Gravity une gestion complète de : - Layout - Sizing - Theming - Styling Tu peux t'aider des exemples dispo ici : https://github.com/iced-rs/iced/tree/master/examples#examples"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Define Widget Sizing and Spacing (Priority: P1)

A developer wants to control the size and spacing of UI elements declaratively in XML. They specify width, height, padding, and spacing attributes on widgets to create a properly laid-out interface.

**Why this priority**: Layout control is foundational to creating any usable UI. Without explicit sizing control, widgets cannot be positioned or sized appropriately, making the framework impractical for real applications.

**Independent Test**: Developer can create a column with `padding="20"` `spacing="10"`, add multiple child widgets with `width="200"` `height="50"`, and see them rendered with the correct dimensions and spacing.

**Acceptance Scenarios**:

1. **Given** a `<column padding="20" spacing="10">` with multiple child widgets, **When** rendered, **Then** there is 20px padding around the column content and 10px spacing between each child
2. **Given** a widget with `width="200" height="50"`, **When** rendered, **Then** the widget occupies exactly 200x50 pixels
3. **Given** a widget with `width="fill"`, **When** inside a container, **Then** the widget expands to fill available horizontal space
4. **Given** a widget with `width="shrink"`, **When** rendered, **Then** the widget uses only the minimum space needed for its content
5. **Given** nested layout widgets with different padding values, **When** rendered, **Then** padding is applied correctly at each nesting level

---

### User Story 2 - Apply Flexible Layout Constraints (Priority: P1)

A developer wants to use flexible layout systems like fill, shrink, and fixed sizing to create responsive interfaces that adapt to different window sizes.

**Why this priority**: Responsive layouts are essential for modern applications. Without flexible sizing, UIs would be rigid and break at different resolutions.

**Independent Test**: Developer creates a row with three buttons: first with `width="fill"`, second with `width="shrink"`, third with `width="300"`. When window is resized, first button expands/contracts, second stays minimal, third stays fixed at 300px.

**Acceptance Scenarios**:

1. **Given** a widget with `width="fill" height="fill"`, **When** parent container is resized, **Then** widget scales to fill all available space
2. **Given** a row with multiple `width="fill"` children, **When** rendered, **Then** available space is distributed equally among fill children
3. **Given** a widget with `max_width="500"`, **When** fill width would exceed 500px, **Then** widget is capped at 500px
4. **Given** a widget with `min_width="100"`, **When** shrink width would be less than 100px, **Then** widget maintains 100px minimum
5. **Given** percentage-based sizing like `width="50%"`, **When** rendered, **Then** widget occupies 50% of parent container width

---

### User Story 3 - Define and Apply Custom Themes (Priority: P1)

A developer wants to define a theme with custom colors, fonts, and visual properties, then apply it globally or per-widget to achieve a consistent look and feel.

**Why this priority**: Theming is critical for branding and user experience. Applications need consistent visual design, and themes enable switching between light/dark modes or custom brand styles.

**Independent Test**: Developer defines a theme with `primary_color="#3498db"`, `background="#ecf0f1"`, `font_family="Roboto"`, applies it to the application, and sees all themed widgets reflect these values.

**Acceptance Scenarios**:

1. **Given** a theme definition with color palette (primary, secondary, background, text), **When** applied to application, **Then** all widgets use theme colors by default
2. **Given** a widget with `theme="dark"` attribute, **When** global theme is "light", **Then** that specific widget renders with dark theme while others use light
3. **Given** a button with no explicit styling, **When** theme changes from light to dark, **Then** button automatically updates appearance to match new theme
4. **Given** custom theme properties like `border_radius="8"`, **When** applied, **Then** all styled widgets use 8px border radius
5. **Given** theme font definitions, **When** text widgets render, **Then** they use the theme's specified font family, size, and weight

---

### User Story 4 - Apply Inline Widget Styling (Priority: P2)

A developer wants to override theme defaults for specific widgets using inline style attributes like background color, border, shadow, and opacity.

**Why this priority**: While themes provide consistency, specific widgets often need custom styling for emphasis, states, or unique design requirements. This can be deferred after theme system works.

**Independent Test**: Developer creates a button with `background="#e74c3c"` `border_width="2"` `border_color="#c0392b"` `border_radius="4"`, and sees the button rendered with these exact visual properties, overriding theme defaults.

**Acceptance Scenarios**:

1. **Given** a widget with `background="#3498db"`, **When** rendered, **Then** widget has blue background regardless of theme
2. **Given** a widget with `border_width="2" border_color="#000" border_radius="8"`, **When** rendered, **Then** widget displays a 2px black border with 8px rounded corners
3. **Given** a widget with `opacity="0.5"`, **When** rendered, **Then** widget and all its children are rendered at 50% opacity
4. **Given** a widget with `shadow_offset="2,2" shadow_blur="4" shadow_color="#000000"`, **When** rendered, **Then** widget displays a drop shadow with specified parameters
5. **Given** conflicting inline style and theme, **When** rendered, **Then** inline style takes precedence over theme default

---

### User Story 5 - Define Reusable Style Classes (Priority: P2)

A developer wants to define reusable style classes (like CSS classes) that can be applied to multiple widgets, reducing duplication and enabling consistent styling patterns.

**Why this priority**: Style classes improve maintainability and reduce XML verbosity. However, basic inline styling can work initially without this abstraction.

**Independent Test**: Developer defines a style class `button_primary` with specific colors and sizing, applies `class="button_primary"` to multiple buttons, and all buttons share the same appearance.

**Acceptance Scenarios**:

1. **Given** a style class definition `<style name="card" background="#fff" padding="20" border_radius="8" shadow="..." />`, **When** widget uses `class="card"`, **Then** widget inherits all card style properties
2. **Given** multiple widgets with `class="button_primary"`, **When** rendered, **Then** all widgets have identical visual styling
3. **Given** a widget with `class="card" background="#f0f0f0"`, **When** rendered, **Then** inline background overrides class background while other class properties apply
4. **Given** a widget with `class="card highlighted"` (multiple classes), **When** rendered, **Then** both class styles are merged with later classes taking precedence on conflicts
5. **Given** style class update during hot-reload, **When** class definition changes, **Then** all widgets using that class update appearance

---

### User Story 6 - Use Alignment and Positioning Controls (Priority: P2)

A developer wants to control widget alignment within containers (left, center, right, top, bottom) and positioning (relative, absolute) to achieve precise layouts.

**Why this priority**: Alignment is essential for polished UIs, but basic layouts can function with default alignment initially.

**Independent Test**: Developer creates a container with `align="center"` containing a button, and the button is centered both horizontally and vertically within the container.

**Acceptance Scenarios**:

1. **Given** a column with `align_items="center"`, **When** children are added, **Then** all children are horizontally centered within the column
2. **Given** a row with `align_items="end"`, **When** children have different heights, **Then** all children are aligned to the bottom
3. **Given** a container with `justify_content="space_between"`, **When** multiple children exist, **Then** first child is at start, last at end, others evenly spaced between
4. **Given** a widget with `position="absolute" top="20" left="40"`, **When** rendered, **Then** widget is positioned 20px from top, 40px from left of nearest positioned ancestor
5. **Given** a widget with `align_self="end"`, **When** parent uses different alignment, **Then** this widget overrides parent alignment for itself

---

### User Story 7 - Define Responsive Breakpoints (Priority: P3)

A developer wants to define different layouts or styles based on viewport size (mobile, tablet, desktop) using responsive breakpoint attributes.

**Why this priority**: Responsive design is important for cross-device support, but initial version can target single-platform development. This can be added later for production applications.

**Independent Test**: Developer defines a column with `mobile:spacing="10" desktop:spacing="20"`, and when window width is below mobile breakpoint, spacing is 10px, otherwise 20px.

**Acceptance Scenarios**:

1. **Given** breakpoint definitions for mobile (<640px), tablet (<1024px), desktop (>=1024px), **When** window width changes, **Then** appropriate breakpoint becomes active
2. **Given** a widget with `mobile:width="fill" desktop:width="800"`, **When** on mobile viewport, **Then** widget fills width, **When** on desktop, **Then** widget is 800px
3. **Given** a layout with `mobile:direction="column" desktop:direction="row"`, **When** viewport changes, **Then** children reflow from column to row or vice versa
4. **Given** responsive attribute `tablet:visible="false"`, **When** on tablet viewport, **Then** widget is hidden
5. **Given** multiple breakpoint matches, **When** attribute is defined at multiple levels, **Then** most specific breakpoint takes precedence

---

### User Story 8 - Apply State-Based Styling (Priority: P3)

A developer wants widgets to change appearance based on interaction states (hover, focus, active, disabled) without writing custom event handlers.

**Why this priority**: State-based styling improves UX polish, but functional applications can work without it initially. This is more about refinement than core functionality.

**Independent Test**: Developer defines button styles `hover:background="#2980b9"` `active:background="#21618c"` `disabled:opacity="0.5"`, and button appearance changes automatically when user hovers, clicks, or when disabled state is true.

**Acceptance Scenarios**:

1. **Given** a button with `hover:background="#2980b9"`, **When** user hovers over button, **Then** background changes to #2980b9
2. **Given** a text_input with `focus:border_color="#3498db"`, **When** input receives focus, **Then** border color changes to blue
3. **Given** a widget with `active:transform="scale(0.95)"`, **When** user presses down on widget, **Then** widget scales to 95%
4. **Given** a button with `disabled="true" disabled:opacity="0.5"`, **When** rendered, **Then** button appears at 50% opacity and does not respond to clicks
5. **Given** nested state styles `hover:active:background="#1a5276"`, **When** user hovers and presses simultaneously, **Then** combined state style applies

---

### Edge Cases

- What happens when conflicting sizing constraints are specified (e.g., `width="fill" max_width="100"` where available space is 500px)? System applies fill first, then enforces max constraint, resulting in 100px width.
- How does the system handle percentage sizing when parent has no defined size? Percentage values fall back to shrink behavior with a warning in dev mode.
- What happens when a theme references colors that don't exist in the palette? System logs error and falls back to default theme color for that property.
- How are circular style class dependencies resolved (class A inherits from B, B inherits from A)? System detects cycles during parsing and returns error with cycle path.
- What happens when inline styles conflict with class styles and theme defaults? Precedence order: inline styles > class styles > theme defaults, with last value winning within same priority level.
- How does responsive breakpoint switching handle state preservation during layout changes? Widget state is preserved; only layout and styles are recomputed on breakpoint transitions.
- What happens with deeply nested themes (widget theme overrides section theme overrides global theme)? Theme properties cascade down, with most specific theme taking precedence for each property.

## Requirements *(mandatory)*

### Functional Requirements

#### Layout System

- **FR-001**: System MUST support layout attributes on container widgets: `padding`, `spacing`, `align_items`, `justify_content`, `direction`
- **FR-002**: System MUST support `padding` format as single value (all sides), two values (vertical, horizontal), or four values (top, right, bottom, left)
- **FR-003**: System MUST support `spacing` attribute controlling gap between child widgets in columns and rows
- **FR-004**: System MUST support `align_items` values: `start`, `center`, `end`, `stretch`
- **FR-005**: System MUST support `justify_content` values: `start`, `center`, `end`, `space_between`, `space_around`, `space_evenly`
- **FR-006**: System MUST support `direction` attribute for rows: `horizontal`, `horizontal_reverse`; and columns: `vertical`, `vertical_reverse`

#### Sizing System

- **FR-007**: System MUST support `width` and `height` attributes accepting: pixel values (e.g., `"200"`), `"fill"`, `"shrink"`, `"fill_portion"`, percentage values (e.g., `"50%"`)
- **FR-008**: System MUST support `min_width`, `max_width`, `min_height`, `max_height` constraint attributes
- **FR-009**: System MUST support `fill` sizing to expand widget to fill available parent space
- **FR-010**: System MUST support `shrink` sizing to minimize widget to content size
- **FR-011**: System MUST support `fill_portion` attribute for proportional space distribution among siblings
- **FR-012**: System MUST resolve sizing conflicts by applying constraints in order: min/max limits, then fill/shrink behavior
- **FR-013**: System MUST support percentage-based sizing calculated relative to parent container dimensions

#### Theming System

- **FR-014**: System MUST support theme definition with properties: `name`, `primary_color`, `secondary_color`, `background`, `text_color`, `font_family`, `font_size`, `border_radius`, `shadow`
- **FR-015**: System MUST support global theme application via application configuration or root widget attribute
- **FR-016**: System MUST support per-widget theme override using `theme="theme_name"` attribute
- **FR-017**: System MUST support light and dark theme variants with automatic system preference detection
- **FR-018**: System MUST provide default themes: `light`, `dark`, `default`
- **FR-019**: System MUST allow theme customization by extending base themes
- **FR-020**: System MUST support theme color palette with semantic color names: `primary`, `secondary`, `success`, `warning`, `danger`, `background`, `surface`, `text`, `text_secondary`
- **FR-021**: System MUST support theme typography settings: `font_family`, `font_size`, `font_weight`, `line_height`
- **FR-022**: System MUST support theme spacing scale (e.g., `spacing_unit=4`, multiples for consistent spacing)

#### Styling System

- **FR-023**: System MUST support inline style attributes: `background`, `color`, `border_width`, `border_color`, `border_radius`, `opacity`, `shadow`
- **FR-024**: System MUST support `background` formats: solid color (`"#3498db"`), gradient (`"linear-gradient(90deg, #3498db, #2ecc71)"`), image reference (`"url(/path/to/image.png)"`)
- **FR-025**: System MUST support `border` with sub-properties: `border_width`, `border_color`, `border_radius`, `border_style` (solid, dashed, dotted)
- **FR-026**: System MUST support `shadow` attribute with format: `"offset_x offset_y blur_radius color"` (e.g., `"2 2 4 #00000040"`)
- **FR-027**: System MUST support `opacity` attribute accepting values 0.0 to 1.0
- **FR-028**: System MUST support `transform` attribute with values: `scale(n)`, `rotate(deg)`, `translate(x, y)`
- **FR-029**: System MUST apply style precedence order: inline styles > style classes > theme defaults

#### Style Classes

- **FR-030**: System MUST support style class definitions in XML or separate style file
- **FR-031**: System MUST support `class` attribute accepting single or space-separated multiple class names
- **FR-032**: System MUST merge multiple class styles with later classes taking precedence on conflicts
- **FR-033**: System MUST support pseudo-selectors in class definitions: `:hover`, `:focus`, `:active`, `:disabled`
- **FR-034**: System MUST allow classes to extend other classes using inheritance or composition
- **FR-035**: System MUST detect and report circular class dependencies during parsing

#### Alignment and Positioning

- **FR-036**: System MUST support `align_self` attribute allowing individual child to override parent's `align_items`
- **FR-037**: System MUST support `align` shorthand combining `align_items` and `justify_content` (e.g., `align="center"` for both)
- **FR-038**: System MUST support `position` attribute with values: `relative` (default), `absolute`
- **FR-039**: System MUST support position offsets: `top`, `right`, `bottom`, `left` for positioned widgets
- **FR-040**: System MUST support `z_index` attribute controlling stacking order of positioned widgets

#### Responsive Design

- **FR-041**: System MUST support breakpoint prefix syntax: `mobile:`, `tablet:`, `desktop:` on any attribute
- **FR-042**: System MUST define default breakpoints: `mobile` (<640px), `tablet` (640-1024px), `desktop` (>=1024px)
- **FR-043**: System MUST allow custom breakpoint definitions via configuration
- **FR-044**: System MUST evaluate breakpoint-prefixed attributes based on current viewport width
- **FR-045**: System MUST re-evaluate and apply responsive attributes when viewport is resized

#### State-Based Styling

- **FR-046**: System MUST support state prefix syntax: `hover:`, `focus:`, `active:`, `disabled:` on style attributes
- **FR-047**: System MUST automatically apply state-prefixed styles when widget enters corresponding state
- **FR-048**: System MUST support combined state selectors (e.g., `hover:focus:border_color="#3498db"`)
- **FR-049**: System MUST support `disabled` attribute boolean controlling disabled state
- **FR-050**: System MUST prevent event handlers from firing on widgets in disabled state

### Key Entities

- **Theme**: Named collection of visual properties (colors, fonts, spacing, shadows) applied globally or per-widget
- **ThemePalette**: Set of semantic color definitions within a theme (primary, secondary, background, text, etc.)
- **StyleClass**: Reusable named style definition that can be applied to multiple widgets via `class` attribute
- **LayoutConstraints**: Sizing and positioning constraints for a widget (width, height, min/max, alignment)
- **StyleProperties**: Visual appearance properties (background, border, shadow, opacity, transform)
- **ResponsiveBreakpoint**: Named viewport size range with associated attribute overrides
- **WidgetState**: Interaction state affecting appearance (normal, hover, focus, active, disabled)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can create responsive layouts that adapt to three viewport sizes (mobile, tablet, desktop) using breakpoint attributes
- **SC-002**: Applications can switch between light and dark themes with all widgets updating appearance within 100ms
- **SC-003**: Developers can define a custom theme and apply it to entire application with less than 20 lines of configuration
- **SC-004**: Widgets with inline styles override theme defaults in 100% of cases with correct precedence
- **SC-005**: Layout changes (padding, spacing, sizing) during hot-reload reflect in UI within 500ms without losing application state
- **SC-006**: Style classes reduce XML verbosity by at least 40% for applications using repeated styling patterns
- **SC-007**: State-based styling (hover, focus, active) responds to user interaction within 50ms
- **SC-008**: Complex layouts with nested containers and flexible sizing render correctly across window sizes from 320px to 2560px width
- **SC-009**: Developers can recreate the visual appearance of any Iced example (styling, tour, etc.) using the declarative styling system
- **SC-010**: Parse time for XML with comprehensive layout, theming, and styling attributes remains under 10ms for files with 1000 widgets

### Assumptions

- Iced 0.14+ provides sufficient theming APIs for custom color palettes and typography
- Developers are familiar with CSS-like styling concepts (classes, pseudo-selectors, cascade)
- Responsive design targets web and desktop viewports, not mobile device rotation
- Performance impact of real-time responsive attribute evaluation is acceptable (<16ms per frame)
- Theme definitions can be embedded in XML or loaded from separate configuration files
- State-based styling requires Iced widget support for hover/focus/active state callbacks
