# Research: Inline State Styles & Responsive Design

**Feature**: 002-inline-state-responsive  
**Date**: 2026-01-19

## Executive Summary

Research confirms the Dampen codebase has extensive existing infrastructure that can be leveraged for both features. The implementation primarily involves connecting already-built components rather than creating new systems from scratch.

---

## Research Question 1: Parser Detection of State-Prefixed Attributes

**Question**: How should the parser detect and handle state-prefixed attributes like `hover:background`?

**Decision**: Use colon (`:`) as separator for state prefixes, distinct from hyphen (`-`) used for breakpoints.

**Rationale**:
- Breakpoints already use hyphen: `mobile-spacing`, `tablet-width`
- Colon provides clear visual distinction: `hover:background`, `focus:border`
- Colon is a common convention (Tailwind CSS uses similar patterns)
- XML allows colons in attribute names (unlike namespace prefixes which require `xmlns` declarations)

**Alternatives Considered**:
1. **Hyphen for both** (`hover-background`): Rejected - conflicts with existing breakpoint pattern, ambiguous parsing
2. **Underscore** (`hover_background`): Rejected - inconsistent with XML naming conventions, less readable
3. **Double hyphen** (`hover--background`): Rejected - unusual syntax, harder to type

**Implementation**:
```rust
// In parser/mod.rs after breakpoint check
if let Some((state_prefix, attr_name)) = name.split_once(':') {
    if let Some(state) = WidgetState::from_prefix(state_prefix) {
        // Store in inline_state_variants
    }
}
```

---

## Research Question 2: WidgetState.from_prefix() Implementation

**Question**: Does `WidgetState::from_prefix()` exist, and what should it return?

**Decision**: Method needs to be added to `WidgetState` enum in `dampen-core/src/ir/theme.rs`.

**Rationale**:
- Current `WidgetState` enum has no parsing method
- Centralized parsing logic ensures consistency
- Returns `Option<WidgetState>` to handle invalid prefixes gracefully

**Implementation**:
```rust
impl WidgetState {
    pub fn from_prefix(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "hover" => Some(WidgetState::Hover),
            "focus" => Some(WidgetState::Focus),
            "active" => Some(WidgetState::Active),
            "disabled" => Some(WidgetState::Disabled),
            _ => None,
        }
    }
}
```

---

## Research Question 3: Inline State Variants Storage Structure

**Question**: How should inline state styles be stored in `WidgetNode`?

**Decision**: Add `inline_state_variants: HashMap<WidgetState, StyleProperties>` field to `WidgetNode`.

**Rationale**:
- Mirrors the existing `state_variants` field in `StyleClass`
- `StyleProperties` is the established type for style data
- HashMap allows efficient lookup by state
- Separates inline state styles from base `style` field for clear precedence

**Alternatives Considered**:
1. **Merge into existing `style` field**: Rejected - loses state information, complicates resolution
2. **Use `HashMap<WidgetState, HashMap<String, AttributeValue>>`**: Rejected - raw attributes require re-parsing, `StyleProperties` is already the resolved type
3. **Store in `attributes` with prefixed keys**: Rejected - pollutes general attributes, requires parsing at resolution time

---

## Research Question 4: Style Precedence Order

**Question**: What is the correct precedence for merging styles from multiple sources?

**Decision**: Theme → Class → Class State Variants → Inline Base → Inline State Variants

**Rationale**:
- Follows CSS-like specificity: more specific overrides less specific
- Inline styles are most specific (directly on element)
- State variants override their respective base styles
- Class state variants apply before inline but after class base

**Precedence Chain**:
```
1. Theme styles (lowest priority)
2. Class styles (from `class="btn-primary"`)
3. Class state variants (from `.btn-primary { hover { ... } }`)
4. Inline base styles (from `background="#blue"`)
5. Inline state variants (highest priority, from `hover:background="#red"`)
```

**Implementation in resolve_complete_styles_with_states()**:
```rust
// For each state, build the full style:
// merged_base = theme → class → inline_base
// final_state_style = merged_base → class_state_variant → inline_state_variant
```

---

## Research Question 5: Viewport Width Injection for Responsive Design

**Question**: How should viewport width be provided to the builder for breakpoint resolution?

**Decision**: Add optional `viewport_width: Option<f32>` parameter to `DampenWidgetBuilder::new()`.

**Rationale**:
- Non-breaking change (Option type)
- Builder already receives context (document, model, theme)
- Viewport width can be obtained from Iced's `window::Settings` or resize events
- Default to Desktop breakpoint if None (safe fallback)

**Alternatives Considered**:
1. **Global static/thread-local**: Rejected - not thread-safe, hidden dependency
2. **Pass through model**: Rejected - pollutes user's model with framework concerns
3. **Subscription to window events**: Rejected - adds complexity, builder may be called outside event context

**Implementation**:
```rust
impl<'a, M> DampenWidgetBuilder<'a, M> {
    pub fn new(
        document: &'a Document,
        model: &'a M,
        theme_document: Option<&'a ThemeDocument>,
        viewport_width: Option<f32>,  // NEW
    ) -> Self { ... }
    
    fn active_breakpoint(&self) -> Breakpoint {
        self.viewport_width
            .map(Breakpoint::from_viewport_width)
            .unwrap_or(Breakpoint::Desktop)
    }
}
```

---

## Research Question 6: Breakpoint Attribute Resolution

**Question**: How should breakpoint-specific attributes be merged with base attributes?

**Decision**: Breakpoint attributes override base attributes for the active breakpoint only.

**Rationale**:
- Matches CSS media query behavior
- Base attributes provide fallback
- Only active breakpoint's overrides apply

**Resolution Order**:
```
1. Base attribute value (e.g., `spacing="20"`)
2. Active breakpoint override (e.g., if mobile: `mobile-spacing="10"` → spacing=10)
```

**Edge Cases**:
- No base attribute + no active breakpoint match → use widget default
- Multiple breakpoints defined + one active → only active applies
- All breakpoints defined + none active → use base or default

---

## Research Question 7: Codegen Strategy for State-Aware Closures

**Question**: How should dampen-macros generate code for inline state styles?

**Decision**: Generate match expressions on Iced status enums, similar to existing class-based state styling.

**Rationale**:
- Iced requires closures for stateful styling: `|theme, status| -> Style`
- Match expressions are idiomatic Rust
- Compile-time evaluation ensures type safety
- Generated code mirrors hand-written closures

**Generated Code Pattern**:
```rust
// For <button hover:background="#ff0000" label="Click" />
let button = button(text("Click"))
    .style(|_theme, status| {
        let base = iced::widget::button::Style {
            background: Some(iced::Background::Color(/* base color */)),
            ..Default::default()
        };
        match status {
            Status::Hovered => iced::widget::button::Style {
                background: Some(iced::Background::Color(
                    iced::Color::from_rgb8(0xff, 0x00, 0x00)
                )),
                ..base
            },
            _ => base,
        }
    })
    .on_press(Message::Click);
```

---

## Research Question 8: Widgets Without State Support

**Question**: What happens when state-prefixed attributes are used on non-interactive widgets?

**Decision**: Parser accepts them (valid syntax) but builder ignores them with a debug log.

**Rationale**:
- Container, Column, Row, Text, Space, Rule have no `Status` type in Iced
- Silent acceptance prevents breaking builds
- Debug log helps developers identify mistakes
- Future Iced versions might add state support

**Affected Widgets**: Container, Column, Row, Stack, Text, Space, Rule, Scrollable

**Implementation**:
```rust
// In builder when processing non-interactive widget
if !node.inline_state_variants.is_empty() {
    log::debug!(
        "Widget {:?} has inline state styles but doesn't support states; ignoring",
        node.kind
    );
}
```

---

## Research Question 9: Combined State + Breakpoint Attributes

**Question**: Should `mobile:hover:background` (combined breakpoint + state) be supported?

**Decision**: Not in initial implementation. Defer to future iteration.

**Rationale**:
- Adds significant complexity to parser and resolution
- No immediate user demand documented
- Can be added later without breaking changes
- Current feature scope is already substantial

**Future Pattern** (if implemented):
```xml
<!-- Potential future syntax -->
<button mobile:hover:background="#red" desktop:hover:background="#blue" />
```

---

## Research Question 10: Performance Impact

**Question**: What is the performance impact of state style resolution?

**Decision**: Acceptable - closures are created once per widget build, not per frame.

**Analysis**:
- **Interpreted Mode**: Closure created during `build()` call; called by Iced on state change
- **Codegen Mode**: Closure compiled into binary; zero runtime overhead
- **HashMap lookup**: O(1) for state variant retrieval
- **Style merging**: Linear in number of style properties (typically < 20)

**Benchmarks Needed**:
- Parse time with many state-prefixed attributes
- Builder time with state-aware closures
- Memory usage with inline_state_variants populated

---

## Summary of Decisions

| Topic | Decision | Impact |
|-------|----------|--------|
| State prefix separator | Colon (`:`) | Parser changes |
| WidgetState.from_prefix | Add new method | dampen-core/ir/theme.rs |
| Storage structure | HashMap<WidgetState, StyleProperties> | WidgetNode field |
| Style precedence | Theme → Class → Inline → State | Builder helpers |
| Viewport injection | Optional parameter to builder | DampenWidgetBuilder::new() |
| Breakpoint resolution | Override base attributes | Builder helpers |
| Codegen strategy | Match expressions on status | dampen-macros |
| Non-interactive widgets | Ignore with debug log | Builder |
| Combined state+breakpoint | Deferred | N/A |
| Performance | Acceptable | Monitor in tests |

---

## Open Items Resolved

All NEEDS CLARIFICATION items from initial context have been resolved:
- State prefix syntax: Colon separator
- Storage format: HashMap<WidgetState, StyleProperties>
- Precedence order: Documented above
- Viewport tracking: Optional builder parameter
- Codegen approach: Match expressions
- Non-interactive widgets: Ignore with logging
