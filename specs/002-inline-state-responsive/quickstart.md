# Quickstart: Inline State Styles & Responsive Design

**Feature**: 002-inline-state-responsive  
**Date**: 2026-01-19

## Overview

This guide helps developers implement inline state styles (`hover:background`) and responsive design (`mobile-spacing`) in the Dampen UI framework.

---

## Prerequisites

- Rust 1.85+ (Edition 2024)
- Familiarity with Dampen codebase structure
- Understanding of Iced's style closure pattern

---

## Quick Implementation Checklist

```
Phase 1: Core Changes (dampen-core)
[ ] Add WidgetState::from_prefix() method
[ ] Add inline_state_variants field to WidgetNode
[ ] Update parser to detect state-prefixed attributes
[ ] Add unit tests for parser

Phase 2: Builder Changes (dampen-iced)
[ ] Add viewport_width parameter to DampenWidgetBuilder::new()
[ ] Implement resolve_complete_styles_with_states()
[ ] Implement resolve_breakpoint_attributes()
[ ] Update button builder with state-aware closure
[ ] Update remaining interactive widgets
[ ] Add integration tests

Phase 3: Codegen Changes (dampen-macros)
[ ] Update code generation for state-aware closures
[ ] Add breakpoint resolution code generation
[ ] Add snapshot tests
```

---

## Implementation Guide

### Step 1: Add WidgetState::from_prefix()

**File**: `crates/dampen-core/src/ir/theme.rs`

```rust
impl WidgetState {
    /// Parse a state prefix string (e.g., "hover", "focus") to WidgetState.
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

### Step 2: Add inline_state_variants to WidgetNode

**File**: `crates/dampen-core/src/ir/node.rs`

```rust
use crate::ir::theme::WidgetState;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WidgetNode {
    // ... existing fields ...
    
    /// State-specific styles from inline attributes (e.g., hover:background="#f00")
    #[serde(default)]
    pub inline_state_variants: HashMap<WidgetState, StyleProperties>,
}
```

### Step 3: Update Parser

**File**: `crates/dampen-core/src/parser/mod.rs`

In the `parse_node()` function, after the breakpoint check:

```rust
// Check for state-prefixed attributes (e.g., "hover:background")
if let Some((state_prefix, attr_name)) = name.split_once(':') {
    if let Some(state) = crate::ir::theme::WidgetState::from_prefix(state_prefix) {
        // Parse the attribute value
        let attr_value = parse_attribute_value(value, get_span(node, source))?;
        
        // Get or create StyleProperties for this state
        let state_style = inline_state_variants
            .entry(state)
            .or_insert_with(StyleProperties::default);
        
        // Apply the style attribute
        apply_style_attribute(state_style, attr_name, &attr_value, get_span(node, source))?;
        continue;
    }
    // Unknown prefix - could warn or treat as regular attribute
}
```

### Step 4: Update Builder Constructor

**File**: `crates/dampen-iced/src/builder/mod.rs`

```rust
pub struct DampenWidgetBuilder<'a, M> {
    // ... existing fields ...
    viewport_width: Option<f32>,
}

impl<'a, M> DampenWidgetBuilder<'a, M> {
    pub fn new(
        document: &'a Document,
        model: &'a M,
        theme_document: Option<&'a ThemeDocument>,
        viewport_width: Option<f32>,
    ) -> Self {
        Self {
            document,
            model,
            theme_document,
            viewport_width,
            // ... other fields ...
        }
    }
    
    fn active_breakpoint(&self) -> Breakpoint {
        self.viewport_width
            .map(Breakpoint::from_viewport_width)
            .unwrap_or(Breakpoint::Desktop)
    }
}
```

### Step 5: Implement Style Resolution Helper

**File**: `crates/dampen-iced/src/builder/helpers.rs`

```rust
/// Resolve complete styles including inline state variants.
/// Returns (base_style, state_variants_map).
pub(super) fn resolve_complete_styles_with_states(
    &self,
    node: &WidgetNode,
) -> (Option<StyleProperties>, HashMap<WidgetState, StyleProperties>) {
    // Resolve base style (theme → class → inline)
    let base = self.resolve_complete_styles(node);
    
    // Build state variant map
    let mut state_map = HashMap::new();
    
    for state in [
        WidgetState::Hover,
        WidgetState::Focus,
        WidgetState::Active,
        WidgetState::Disabled,
    ] {
        let mut state_style = base.clone().unwrap_or_default();
        
        // Apply class state variants if present
        if let Some(class_state) = self.resolve_class_state_variant(node, state) {
            state_style = merge_style_properties(&state_style, &class_state);
        }
        
        // Apply inline state variants (highest priority)
        if let Some(inline_state) = node.inline_state_variants.get(&state) {
            state_style = merge_style_properties(&state_style, inline_state);
        }
        
        state_map.insert(state, state_style);
    }
    
    (base, state_map)
}
```

### Step 6: Update Widget Builders

**File**: `crates/dampen-iced/src/builder/button.rs`

```rust
pub(super) fn build_button<'a, M>(
    &self,
    node: &WidgetNode,
) -> Element<'a, M> {
    let (base_style, state_variants) = self.resolve_complete_styles_with_states(node);
    
    // Clone for move into closure
    let base_for_closure = base_style.clone();
    let variants_for_closure = state_variants.clone();
    
    let style_closure = move |_theme: &Theme, status: button::Status| {
        use dampen_core::ir::theme::WidgetState;
        
        let dampen_state = match status {
            button::Status::Hovered => Some(WidgetState::Hover),
            button::Status::Pressed => Some(WidgetState::Active),
            button::Status::Disabled => Some(WidgetState::Disabled),
            button::Status::Active => None,
        };
        
        let style_props = dampen_state
            .and_then(|s| variants_for_closure.get(&s))
            .or(base_for_closure.as_ref());
        
        style_props
            .map(|sp| convert_style_properties_to_button_style(sp))
            .unwrap_or_default()
    };
    
    button(content)
        .style(style_closure)
        .on_press(message)
        .into()
}
```

---

## Testing

### Unit Test Example

**File**: `crates/dampen-core/tests/parser_inline_states.rs`

```rust
#[test]
fn test_parse_inline_hover_style() {
    let xml = r#"<button hover:background="#ff0000" label="Test" />"#;
    let doc = parse(xml).expect("Should parse");
    
    assert!(
        doc.root.inline_state_variants.contains_key(&WidgetState::Hover),
        "Should have hover state variant"
    );
    
    let hover_style = doc.root.inline_state_variants.get(&WidgetState::Hover).unwrap();
    assert!(hover_style.background.is_some(), "Should have background in hover");
}

#[test]
fn test_parse_multiple_states() {
    let xml = r#"<button 
        hover:background="#ff0000" 
        active:background="#00ff00" 
        disabled:opacity="0.5" 
        label="Test" 
    />"#;
    let doc = parse(xml).expect("Should parse");
    
    assert_eq!(doc.root.inline_state_variants.len(), 3);
}
```

### Integration Test Example

**File**: `crates/dampen-iced/tests/builder_state_styles.rs`

```rust
#[test]
fn test_button_hover_style_applied() {
    let xml = r#"<button hover:background="#ff0000" label="Hover Me" />"#;
    let doc = parse(xml).unwrap();
    let model = TestModel::default();
    
    let builder = DampenWidgetBuilder::new(&doc, &model, None, None);
    let _element = builder.build();
    
    // Note: Testing actual style application requires Iced test utilities
    // or snapshot testing of rendered output
}
```

---

## Common Patterns

### State Priority Override

```xml
<!-- Class hover is overridden by inline hover -->
<button class="btn-primary" hover:background="#custom" />
```

### Responsive + State (Future)

```xml
<!-- Not yet supported, but planned pattern -->
<button mobile:hover:background="#red" desktop:hover:background="#blue" />
```

### Fallback Behavior

```xml
<!-- If no hover:background, uses base background on hover -->
<button background="#blue" hover:border="2px solid white" />
```

---

## Debugging Tips

1. **Parser issues**: Check `node.inline_state_variants` after parsing
2. **Builder issues**: Add logging in `resolve_complete_styles_with_states()`
3. **State mapping**: Verify Iced status → WidgetState mapping in `style_mapping.rs`
4. **Visual issues**: Use interpreted mode for faster iteration, then test codegen

---

## Related Files

| Component | Primary Files |
|-----------|---------------|
| Parser | `crates/dampen-core/src/parser/mod.rs` |
| IR Types | `crates/dampen-core/src/ir/node.rs`, `theme.rs` |
| Builder | `crates/dampen-iced/src/builder/helpers.rs`, `mod.rs` |
| Style Mapping | `crates/dampen-iced/src/style_mapping.rs` |
| Codegen | `crates/dampen-macros/src/dampen_app.rs` |
