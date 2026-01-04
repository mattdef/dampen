# Research: Advanced Widgets for Modern Todo App

**Feature**: 004-advanced-widgets-todo  
**Date**: 2026-01-04  
**Status**: Complete

## Overview

This document consolidates research on implementing 8 advanced Iced widgets in Gravity to enable a fully functional, modern todo application. The widgets are categorized by priority based on their importance for todo app functionality.

---

## Widget API Research

### 1. ComboBox (Priority: P1 - Essential)

**Iced Documentation**: https://docs.rs/iced/latest/iced/widget/combo_box/

**Decision**: Support ComboBox for searchable dropdown selections

**API Analysis**:
```rust
// Iced API signature
pub fn combo_box<'a, T, Message>(
    state: &'a combo_box::State<T>,
    placeholder: &str,
    selection: Option<&'a T>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> ComboBox<'a, T, Message>
where
    T: std::fmt::Display + Clone
```

**Key Characteristics**:
- Requires `combo_box::State<T>` to manage options and search state
- Selection is `Option<&T>` for current value
- Placeholder text for empty state
- Type `T` must implement `Display` + `Clone`
- User can type to search and select from filtered options

**Gravity XML Mapping**:
```xml
<combobox 
    options="Work,Personal,Shopping,Other"
    selected="{current_category}"
    placeholder="Select category..."
    on_select="update_category"
/>
```

**Implementation Requirements**:
- Parse comma-separated `options` attribute into `Vec<String>`
- Maintain `combo_box::State` in Gravity runtime (requires state management enhancement)
- Convert `selected` binding to `Option<&String>`
- Map `on_select` event to handler with selected value

**Alternatives Considered**: 
- Using PickList instead → Rejected because PickList doesn't support search/typing
- Custom text input + dropdown → Rejected because ComboBox provides this natively

---

### 2. PickList (Priority: P1 - Essential)

**Iced Documentation**: https://docs.rs/iced/latest/iced/widget/pick_list/

**Decision**: Support PickList for simple dropdown selections

**API Analysis**:
```rust
// Iced API signature
pub fn pick_list<'a, T, L, V, Message>(
    options: L,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> PickList<'a, T, L, V, Message>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]>,
    V: Borrow<T>
```

**Key Characteristics**:
- Simpler than ComboBox (no search/typing)
- Options can be array, Vec, or slice
- No state management required
- Placeholder can be set with `.placeholder()` method
- Type `T` must implement `ToString` + `PartialEq` + `Clone`

**Gravity XML Mapping**:
```xml
<pick_list 
    options="All,Active,Completed"
    selected="{current_filter}"
    placeholder="Filter by..."
    on_select="apply_filter"
/>
```

**Implementation Requirements**:
- Parse comma-separated `options` into `Vec<String>`
- Convert `selected` binding to `Option<String>`
- Map `on_select` event to handler with selected value
- Support optional `placeholder` attribute

**Alternatives Considered**:
- Using ComboBox everywhere → Rejected because PickList is simpler for known fixed options
- Radio buttons → Rejected for space efficiency (dropdowns take less vertical space)

---

### 3. Canvas (Priority: P1 - Essential but Complex)

**Iced Documentation**: https://docs.rs/iced/latest/iced/widget/canvas/

**Decision**: Support Canvas with Rust-side rendering via `Program` trait

**API Analysis**:
```rust
// Iced API - requires implementing Program trait
pub trait Program<Message> {
    type State;
    
    fn draw(&self, state: &Self::State, renderer: &Renderer, 
            theme: &Theme, bounds: Rectangle, cursor: mouse::Cursor) 
            -> Vec<Geometry>;
    
    fn update(&self, state: &mut Self::State, event: Event, 
              bounds: Rectangle, cursor: mouse::Cursor) 
              -> (event::Status, Option<Message>);
}

pub fn canvas<P, Message>(program: P) -> Canvas<P, Message>
where
    P: Program<Message>
```

**Key Characteristics**:
- Cannot be fully declarative in XML (requires Rust code for drawing logic)
- Uses `Program` trait for custom rendering
- Supports mouse events, drawing primitives, paths, fills, strokes
- Provides `Frame` for drawing operations
- Can cache geometry for performance

**Gravity XML Mapping**:
```xml
<canvas 
    width="400" 
    height="200" 
    program="{statistics_canvas}"
    on_click="canvas_clicked"
/>
```

**Implementation Strategy**:
- Canvas requires hybrid approach: XML declares placement, Rust implements rendering
- User provides `impl Program<Message>` in Rust code
- Gravity binds the program implementation via model field
- Canvas size controlled by `width`/`height` attributes
- Events can be handled normally

**Alternatives Considered**:
- Declarative SVG-like drawing in XML → Rejected as too complex and limiting
- Skip Canvas entirely → Rejected because visualization is valuable for modern UIs
- Custom Gravity drawing DSL → Rejected due to implementation complexity

**Rationale**: Canvas is inherently imperative for custom graphics. The hybrid approach (declarative placement + Rust rendering) maintains Gravity's declarative principles while enabling powerful visualizations.

---

### 4. ProgressBar (Priority: P2 - Visual Enhancement)

**Iced Documentation**: https://docs.rs/iced/latest/iced/widget/progress_bar/

**Decision**: Support ProgressBar for completion tracking

**API Analysis**:
```rust
// Iced API signature
pub fn progress_bar<'a>(
    range: RangeInclusive<f32>,
    value: f32,
) -> ProgressBar<'a>
```

**Key Characteristics**:
- Simple widget with range and current value
- Range defaults to `0.0..=1.0` but can be customized
- Value is automatically clamped to range
- Supports styling (primary, success, warning, danger)

**Gravity XML Mapping**:
```xml
<progress_bar 
    min="0" 
    max="100" 
    value="{completion_percentage}"
    style="success"
/>
```

**Implementation Requirements**:
- Parse `min` and `max` attributes (default: 0.0 and 1.0)
- Evaluate `value` binding to get current progress
- Map `style` attribute to Iced progress bar styles
- Auto-clamp value to range

**Alternatives Considered**:
- Custom styled container → Rejected because ProgressBar provides native theming
- Slider in read-only mode → Rejected due to different UX semantics

---

### 5. Tooltip (Priority: P2 - Visual Enhancement)

**Iced Documentation**: https://docs.rs/iced/latest/iced/widget/tooltip/

**Decision**: Support Tooltip as wrapper widget for contextual help

**API Analysis**:
```rust
// Iced API signature
pub fn tooltip<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    tooltip: impl Into<Element<'a, Message>>,
    position: Position,
) -> Tooltip<'a, Message>
```

**Key Characteristics**:
- Wraps another widget to add hover tooltip
- Position can be Top, Bottom, Left, Right, or FollowCursor
- Default delay of 2 seconds before showing (configurable)
- Tooltip content is any Element

**Gravity XML Mapping**:
```xml
<tooltip message="Delete all completed tasks" position="top">
    <button label="Clear Completed" on_click="clear_completed" />
</tooltip>
```

**Implementation Requirements**:
- Parse `message` attribute for simple text tooltips
- Support child element as the base widget
- Parse `position` attribute (default: FollowCursor)
- Optional `delay` attribute in milliseconds

**Alternatives Considered**:
- Title attribute on widgets → Rejected because Iced doesn't support this pattern
- Custom hover overlay → Rejected because Tooltip provides this natively

---

### 6. Image (Priority: P2 - Visual Enhancement)

**Iced Documentation**: https://docs.rs/iced/latest/iced/widget/image/

**Decision**: Verify and document existing Image support (already in WidgetKind)

**API Analysis**:
```rust
// Iced API - Image already supported in Gravity
pub fn image<Handle>(handle: impl Into<Handle>) -> Image
where
    Handle: Clone + Hash

// Handle types
pub enum Handle {
    Path(PathBuf),           // File path
    Bytes(Bytes),            // Raw bytes
    Rgba { width, height, pixels },  // RGBA data
}
```

**Key Characteristics**:
- Already in `WidgetKind::Image`
- Supports file paths, byte arrays, RGBA data
- Automatic aspect ratio maintenance
- Supports width/height constraints
- Content fit strategies: Fill, Contain, Cover, ScaleDown

**Current Gravity Support**:
```xml
<image path="assets/icon.png" width="32" height="32" />
```

**Implementation Status**: 
- ✅ Already supported in gravity-core IR (`WidgetKind::Image`)
- ✅ Already supported in gravity-iced builder
- ⚠️ Verify attribute parsing and handle creation

**Action Required**:
- Audit existing Image implementation for completeness
- Test with different image formats (PNG, JPG, SVG via Svg widget)
- Ensure error handling for missing files

---

### 7. Grid (Priority: P2 - Layout Enhancement)

**Iced Documentation**: https://docs.rs/iced/latest/iced/widget/grid/

**Decision**: Support Grid for multi-column responsive layouts

**API Analysis**:
```rust
// Iced API signature
pub fn grid<'a, Message>(
    children: impl IntoIterator<Item = Element<'a, Message>>,
) -> Grid<'a, Message>

// Grid supports:
grid.columns(5)              // Set column count
grid.spacing(10)             // Gap between items
grid.padding(20)             // Outer padding
```

**Key Characteristics**:
- Responsive grid layout with automatic wrapping
- Configurable column count
- Items flow left-to-right, top-to-bottom
- Spacing applies between all grid cells
- Children can be any Element

**Gravity XML Mapping**:
```xml
<grid columns="5" spacing="10" padding="20">
    <text value="{task.name}" />
    <text value="{task.category}" />
    <text value="{task.priority}" />
    <text value="{task.due_date}" />
    <button label="Edit" on_click="edit_task" />
</grid>
```

**Implementation Requirements**:
- Parse `columns` attribute (required)
- Parse `spacing` and `padding` attributes
- Collect all child widgets into iterator
- Map to `iced::widget::grid()`

**Alternatives Considered**:
- Nested Row/Column → Rejected due to verbosity and lack of responsive wrapping
- HTML-like table → Rejected because Grid is more flexible
- CSS Grid-like syntax → Rejected for complexity

---

### 8. Float (Priority: P3 - Advanced UX)

**Iced Documentation**: https://docs.rs/iced/latest/iced/widget/float/

**Decision**: Support Float for positioned overlay elements

**API Analysis**:
```rust
// Iced API signature - NOT CONFIRMED (based on module listing)
// Need to verify exact API in actual docs

// Expected API pattern:
pub fn float<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    floating: impl Into<Element<'a, Message>>,
) -> Float<'a, Message>
```

**Key Characteristics**:
- Positions content at fixed coordinates
- Can layer content with z-index
- Useful for modals, floating buttons, notifications
- **⚠️ API NEEDS VERIFICATION** - Float is listed but specifics unclear

**Gravity XML Mapping (Tentative)**:
```xml
<float position="bottom-right" offset_x="20" offset_y="20" z_index="100">
    <button label="+" on_click="show_add_dialog" />
</float>
```

**Implementation Requirements** (pending API verification):
- Parse `position` attribute (top-left, top-right, bottom-left, bottom-right, custom)
- Parse `offset_x` and `offset_y` for positioning
- Parse `z_index` for layering
- Support visibility toggling via binding

**Action Required**:
- **Fetch detailed Float documentation to confirm API**
- Determine if Float is the right widget or if we need `overlay` or `pin` instead
- Consider using `pin` widget as alternative for fixed positioning

**Alternatives Considered**:
- Iced `pin` widget → May be better for fixed positioning
- Iced `overlay` → May be better for modals
- Container with absolute positioning → Not supported in Iced's layout model

**Research Note**: Float module exists but detailed API unclear. May need to pivot to `pin` or `overlay` based on actual capabilities.

---

## State Management Analysis

### Challenge: Widget State Requirements

Several new widgets require state management beyond simple model bindings:

1. **ComboBox** requires `combo_box::State<T>`:
   - Manages search input text
   - Filters options based on input
   - Tracks dropdown open/closed state

2. **Canvas** requires `Program::State`:
   - User-defined state for rendering
   - Can be any type implementing required traits

**Decision**: Extend Gravity runtime state management

**Rationale**: 
- Current Gravity model only supports user-defined `#[derive(UiModel)]` fields
- Widget-specific state (like ComboBox::State) must be managed separately
- Solution: Add `widget_states: HashMap<String, Box<dyn Any>>` to runtime

**Implementation Strategy**:
```rust
// In gravity-runtime/src/state.rs
pub struct GravityState {
    pub user_model: Box<dyn UiBindable>,
    pub widget_states: HashMap<String, Box<dyn Any>>, // NEW
}

// In gravity-iced/src/builder.rs
impl GravityWidgetBuilder {
    fn build_combobox(&self, node: &WidgetNode) -> Element<'_, Message> {
        // Get or create ComboBox::State for this widget
        let state_id = node.id.as_ref().unwrap_or(&"unnamed");
        let state = self.get_widget_state::<combo_box::State<String>>(state_id);
        
        combo_box(state, placeholder, selection, on_select)
    }
}
```

**Alternatives Considered**:
- Require users to add widget states to model → Rejected (boilerplate, breaks declarative principle)
- Use global static state → Rejected (not thread-safe, limits multiple instances)
- Generate state fields in #[derive(UiModel)] → Rejected (complex macro logic)

---

## Event Handling Extension

### New Event Types Required

Current `EventKind` enum:
```rust
pub enum EventKind {
    Click, Press, Release, Change, Input, Submit, Select, Toggle, Scroll
}
```

**Required Additions**:
- ✅ `Select` - Already exists! Can be used for ComboBox and PickList
- ⚠️ May need `CanvasEvent` for custom canvas interactions

**Decision**: Reuse existing `EventKind::Select` for dropdown selections

**Canvas Event Handling**:
- Canvas events are handled by `Program::update()` trait method
- Gravity will pass events through to user-provided Program implementation
- No new EventKind needed for basic canvas support

---

## XML Parser Updates

### Attribute Parsing Requirements

**New Attributes to Support**:

| Widget | Attributes | Type | Default |
|--------|-----------|------|---------|
| ComboBox | `options` | String (comma-separated) | Required |
| | `selected` | Binding | `None` |
| | `placeholder` | String | `""` |
| | `on_select` | EventHandler | Optional |
| PickList | `options` | String (comma-separated) | Required |
| | `selected` | Binding | `None` |
| | `placeholder` | String | `""` |
| | `on_select` | EventHandler | Optional |
| Canvas | `width` | f32 | Required |
| | `height` | f32 | Required |
| | `program` | Binding (to Program impl) | Required |
| | `on_click` | EventHandler | Optional |
| ProgressBar | `min` | f32 | `0.0` |
| | `max` | f32 | `1.0` |
| | `value` | Binding (f32) | Required |
| | `style` | Enum | `"primary"` |
| Tooltip | `message` | String | Required |
| | `position` | Enum | `"follow_cursor"` |
| | `delay` | u64 (ms) | `2000` |
| Grid | `columns` | u32 | Required |
| | `spacing` | f32 | `0.0` |
| | `padding` | f32 | `0.0` |
| Float | `position` | Enum | TBD |
| | `offset_x` | f32 | `0.0` |
| | `offset_y` | f32 | `0.0` |
| | `z_index` | u32 | `0` |

**Parser Enhancement Strategy**:
- Extend `parse_attributes()` in `gravity-core/src/parser/mod.rs`
- Add validation for required attributes
- Add enum parsing for `position`, `style` attributes
- Support comma-separated list parsing for `options`

---

## Performance Considerations

### Rendering Budget Analysis

**Current Performance** (from AGENTS.md):
- 100 widgets: ~0.027ms
- 1000 widgets: ~0.284ms

**Expected Impact of New Widgets**:

| Widget | Rendering Cost | Notes |
|--------|---------------|-------|
| ComboBox | ~0.5ms | Dropdown rendering, text filtering |
| PickList | ~0.3ms | Simple dropdown |
| Canvas | Variable | Depends on drawing complexity; caching critical |
| ProgressBar | ~0.1ms | Simple bar fill |
| Tooltip | ~0.2ms | Only when visible |
| Grid | ~0.05ms × items | Layout calculation |
| Image | ~0.2ms | One-time decode, then cached |
| Float | ~0.1ms | Positioning overhead |

**Success Criteria**: < 50ms for typical todo app UI (50-100 widgets)

**Estimated Todo App Widget Count**:
- 1 ComboBox (category selector)
- 2 PickLists (filter, sorting)
- 1 Canvas (statistics chart)
- 1 ProgressBar (completion)
- 5 Tooltips (help text)
- 1 Grid (task list, ~20 tasks)
- 10 Images (priority icons)

**Total Estimate**: ~15ms base + Canvas (depends on chart complexity)

**Mitigation**:
- Use Canvas `Cache` for geometry caching
- Lazy-render tooltips (only when hovered)
- Image caching in Iced handles this automatically

---

## Testing Strategy

### Test Coverage Requirements

**Unit Tests**:
1. Parse each new widget type from XML
2. Validate required attributes enforcement
3. Verify attribute value parsing (enums, numbers, comma-separated lists)
4. Test binding evaluation for widget properties
5. Test event handler registration

**Integration Tests**:
1. Render each widget with GravityWidgetBuilder
2. Verify widgets appear with correct properties
3. Test event firing and handler invocation
4. Test state persistence across hot-reloads
5. Verify theme application to new widgets

**Example Tests**:
1. Complete todo-app example (visual & functional testing)
2. Individual widget showcases
3. Performance benchmarks for rendering

**Property-Based Tests**:
- ComboBox search filtering with random strings
- Grid layout with varying column counts and item counts
- ProgressBar value clamping with out-of-range inputs

---

## Dependencies

### Crate Dependency Analysis

**No new external dependencies required**:
- ✅ Iced 0.14+ already provides all 8 widgets
- ✅ All widgets are in `iced::widget` module
- ✅ Canvas requires `canvas` feature flag (already enabled)
- ✅ Image requires `image` feature flag (need to verify)

**Cargo.toml Verification**:
```toml
[workspace.dependencies]
iced = { version = "0.14", features = ["tokio"] }

# May need to add:
# iced = { version = "0.14", features = ["tokio", "canvas", "image"] }
```

**Action Required**: Verify Iced feature flags in workspace Cargo.toml

---

## Migration Path

### Impact on Existing Code

**Breaking Changes**: None
- All new widgets are additive
- Existing widgets remain unchanged
- Existing examples continue to work

**Non-Breaking Changes**:
- New `WidgetKind` variants (8 new)
- New attributes in XML schema
- New event types (potentially)
- New widget state management (internal to runtime)

**Migration Guide**:
- Users can adopt new widgets incrementally
- No changes required to existing `.gravity` files
- Hot-reload continues to work with new widgets

---

## Risks & Mitigation

### Identified Risks

**Risk 1: Canvas Complexity**
- **Impact**: High - Canvas requires Rust code, breaks full declarative model
- **Likelihood**: Certain
- **Mitigation**: 
  - Document hybrid approach clearly
  - Provide example `Program` implementations
  - Consider future declarative DSL for simple shapes

**Risk 2: Float Widget API Unclear**
- **Impact**: Medium - May need to use different widget (pin/overlay)
- **Likelihood**: High - Documentation insufficient
- **Mitigation**:
  - Test Float widget in isolation
  - Have fallback plan to use `pin` or `overlay`
  - Document actual behavior in contracts/

**Risk 3: Widget State Management Complexity**
- **Impact**: Medium - ComboBox state complicates runtime
- **Likelihood**: Medium
- **Mitigation**:
  - Encapsulate state management in runtime
  - Use widget IDs for state tracking
  - Thoroughly test state persistence across hot-reloads

**Risk 4: Todo App Example Scope Creep**
- **Impact**: Low - Example could become too complex
- **Likelihood**: Medium
- **Mitigation**:
  - Focus on demonstrating widgets, not building production app
  - Keep to 300-400 lines of Rust code
  - Prioritize clarity over features

---

## Conclusion

All 8 widgets are available in Iced 0.14 and can be integrated into Gravity with the following approach:

**Straightforward Widgets** (Can be fully declarative):
- ✅ PickList
- ✅ ProgressBar
- ✅ Tooltip
- ✅ Image (already supported)
- ✅ Grid

**Complex Widgets** (Require state or Rust code):
- ⚠️ ComboBox (needs State management)
- ⚠️ Canvas (needs Rust Program implementation)
- ⚠️ Float (needs API verification, may use Pin instead)

**Key Technical Decisions**:
1. Extend runtime with widget state management
2. Use hybrid declarative/imperative approach for Canvas
3. Reuse existing `EventKind::Select` for dropdowns
4. Create comprehensive todo-app example demonstrating all widgets

**Next Steps**:
1. Define data model for todo app entities
2. Create XML schema contracts for all widgets
3. Write quickstart guide for using new widgets
4. Proceed to implementation planning
