# Data Model: Inline State Styles & Responsive Design

**Feature**: 002-inline-state-responsive  
**Date**: 2026-01-19

## Entity Overview

This feature modifies existing entities rather than introducing new ones. The primary changes are to `WidgetNode` and `WidgetState`.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              WidgetNode                                  │
├─────────────────────────────────────────────────────────────────────────┤
│ kind: WidgetKind                                                        │
│ id: Option<String>                                                      │
│ attributes: HashMap<String, AttributeValue>                             │
│ events: Vec<EventBinding>                                               │
│ children: Vec<WidgetNode>                                               │
│ span: Span                                                              │
│ style: Option<StyleProperties>              ← Base inline styles        │
│ layout: Option<LayoutConstraints>                                       │
│ theme_ref: Option<AttributeValue>                                       │
│ classes: Vec<String>                                                    │
│ breakpoint_attributes: HashMap<Breakpoint, HashMap<String, AttrValue>>  │ (EXISTS)
│ inline_state_variants: HashMap<WidgetState, StyleProperties>            │ (NEW)
└─────────────────────────────────────────────────────────────────────────┘
          │
          │ references
          ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                              WidgetState                                 │
├─────────────────────────────────────────────────────────────────────────┤
│ Hover                                                                   │
│ Focus                                                                   │
│ Active                                                                  │
│ Disabled                                                                │
├─────────────────────────────────────────────────────────────────────────┤
│ + from_prefix(s: &str) -> Option<WidgetState>                    (NEW)  │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                              Breakpoint                                  │
├─────────────────────────────────────────────────────────────────────────┤
│ Mobile    (< 640px)                                              (EXISTS)│
│ Tablet    (640px - 1024px)                                       (EXISTS)│
│ Desktop   (>= 1024px)                                            (EXISTS)│
├─────────────────────────────────────────────────────────────────────────┤
│ + from_viewport_width(width: f32) -> Breakpoint                  (EXISTS)│
│ + parse(s: &str) -> Result<Breakpoint, String>                   (EXISTS)│
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Entity: WidgetNode (Modified)

**Location**: `crates/dampen-core/src/ir/node.rs`

### Existing Fields (unchanged)

| Field | Type | Description |
|-------|------|-------------|
| `kind` | `WidgetKind` | Widget type (Button, Column, etc.) |
| `id` | `Option<String>` | Optional DOM-like identifier |
| `attributes` | `HashMap<String, AttributeValue>` | General attributes |
| `events` | `Vec<EventBinding>` | Event handlers |
| `children` | `Vec<WidgetNode>` | Child widgets |
| `span` | `Span` | Source location for error reporting |
| `style` | `Option<StyleProperties>` | Base inline styles |
| `layout` | `Option<LayoutConstraints>` | Layout constraints |
| `theme_ref` | `Option<AttributeValue>` | Theme reference |
| `classes` | `Vec<String>` | CSS-like class names |
| `breakpoint_attributes` | `HashMap<Breakpoint, HashMap<String, AttributeValue>>` | Responsive overrides (already exists) |

### New Field

| Field | Type | Description |
|-------|------|-------------|
| `inline_state_variants` | `HashMap<WidgetState, StyleProperties>` | State-specific style overrides from inline attributes |

### Validation Rules

1. `inline_state_variants` keys must be valid `WidgetState` variants
2. `StyleProperties` values must contain valid style properties
3. State variants are independent (hover style doesn't affect focus style)

### State Transitions

`WidgetNode` is immutable after parsing. No state transitions apply.

---

## Entity: WidgetState (Modified)

**Location**: `crates/dampen-core/src/ir/theme.rs`

### Existing Definition

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum WidgetState {
    Hover,
    Focus,
    Active,
    Disabled,
}
```

### New Method

```rust
impl WidgetState {
    /// Parse a state prefix string to a WidgetState.
    /// 
    /// # Examples
    /// ```
    /// assert_eq!(WidgetState::from_prefix("hover"), Some(WidgetState::Hover));
    /// assert_eq!(WidgetState::from_prefix("FOCUS"), Some(WidgetState::Focus));
    /// assert_eq!(WidgetState::from_prefix("unknown"), None);
    /// ```
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

### Validation Rules

1. Prefix matching is case-insensitive
2. Unknown prefixes return `None` (not an error)
3. No whitespace trimming (caller responsibility)

---

## Entity: Breakpoint (Unchanged)

**Location**: `crates/dampen-core/src/ir/layout.rs`

Exists and is fully implemented. No changes required.

### Current Definition

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Breakpoint {
    Mobile,    // < 640px
    Tablet,    // 640px - 1024px  
    Desktop,   // >= 1024px
}

impl Breakpoint {
    pub fn from_viewport_width(width: f32) -> Self { ... }
    pub fn parse(s: &str) -> Result<Self, String> { ... }
}
```

---

## Entity: StyleProperties (Unchanged)

**Location**: `crates/dampen-core/src/ir/style.rs`

Existing struct used for styling. No changes required.

### Key Fields (subset)

| Field | Type | Description |
|-------|------|-------------|
| `background` | `Option<Background>` | Background color/gradient |
| `color` | `Option<Color>` | Text/foreground color |
| `border` | `Option<Border>` | Border styling |
| `shadow` | `Option<Shadow>` | Box shadow |
| `opacity` | `Option<f32>` | Opacity value |

---

## Relationships

```
                    Parser
                      │
                      │ creates
                      ▼
┌──────────────────────────────────────────────────────────┐
│                     WidgetNode                            │
│  ┌─────────────────┐    ┌────────────────────────────┐   │
│  │ style           │    │ inline_state_variants      │   │
│  │ (base styles)   │    │ HashMap<WidgetState, Style>│   │
│  └────────┬────────┘    └──────────────┬─────────────┘   │
│           │                            │                  │
│           │ breakpoint_attributes      │                  │
│           │ HashMap<BP, Attrs>         │                  │
└───────────┼────────────────────────────┼──────────────────┘
            │                            │
            │                            │
            ▼                            ▼
┌───────────────────┐        ┌─────────────────────┐
│    Breakpoint     │        │    WidgetState      │
│ Mobile│Tablet│Desk│        │ Hover│Focus│Active  │
└───────────────────┘        │ Disabled            │
                             └─────────────────────┘
```

---

## Data Flow

### Parsing Phase

```
XML Input: <button hover:background="#f00" mobile-spacing="10" />
                        │                       │
                        ▼                       ▼
              split_once(':')           split_once('-')
              state_prefix="hover"      breakpoint="mobile"
              attr_name="background"    attr_name="spacing"
                        │                       │
                        ▼                       ▼
              inline_state_variants     breakpoint_attributes
              { Hover: { bg: #f00 } }   { Mobile: { spacing: 10 } }
```

### Building Phase (Interpreted Mode)

```
WidgetNode
    │
    ├─── resolve_theme_styles() ──────────────────┐
    │                                              │
    ├─── resolve_class_styles() ──────────────────┤
    │                                              │
    ├─── node.style (base inline) ────────────────┤
    │                                              │
    ├─── class.state_variants ────────────────────┤
    │                                              │
    └─── node.inline_state_variants ──────────────┤
                                                   │
                                                   ▼
                                    ┌─────────────────────────┐
                                    │ Style Closure           │
                                    │ |theme, status| {       │
                                    │   match status {        │
                                    │     Hovered => ...      │
                                    │     Active => ...       │
                                    │     _ => base           │
                                    │   }                     │
                                    │ }                       │
                                    └─────────────────────────┘
```

### Codegen Phase

```
WidgetNode.inline_state_variants
    │
    ▼
dampen-macros/src/dampen_app.rs
    │
    ▼
Generated Rust Code:
    button(text("..."))
        .style(|_theme, status| {
            match status {
                Status::Hovered => Style { background: ... },
                Status::Active => Style { ... },
                _ => base_style,
            }
        })
```

---

## Migration Notes

### Backward Compatibility

- `inline_state_variants` defaults to empty `HashMap` via `#[derive(Default)]`
- Existing XML without state-prefixed attributes continues to work
- No breaking changes to public API
- `breakpoint_attributes` already exists and is populated by parser

### Database/Storage

N/A - This is a UI framework, no persistent storage affected.

### Serialization

`WidgetNode` uses `#[derive(Serialize, Deserialize)]`. The new field will be serialized automatically. JSON representation:

```json
{
  "kind": "Button",
  "style": { "background": "#blue" },
  "inline_state_variants": {
    "Hover": { "background": "#red" },
    "Active": { "background": "#green" }
  },
  "breakpoint_attributes": {
    "Mobile": { "width": "100%" }
  }
}
```
