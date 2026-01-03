# Alternative Backend Pattern

**Feature**: 003-widget-builder  
**Date**: 2026-01-03  
**Task**: T089 - Document pattern for alternative backends

---

## Overview

Gravity's architecture is designed to be **backend-agnostic**. The centralized builder pattern ensures that:
1. **gravity-core** contains no backend-specific code (no Iced, GTK, Qt, etc.)
2. Backend implementations are isolated in separate crates (gravity-iced, gravity-gtk, etc.)
3. The same XML UI definitions work across all backends
4. Examples can switch backends by changing dependencies

---

## Architecture Principles

### Constitution Compliance

**Principle IV: Backend Abstraction**
> "Core crate has no Iced dependency"

**Verification**:
```bash
# ✅ PASS: No Iced imports in gravity-core
$ grep -r "use iced::" crates/gravity-core/src/
# (no results)
```

### Shared Components

**gravity-core** (shared across all backends):
- `WidgetNode` IR structure
- `WidgetKind` enum
- `AttributeValue` evaluation
- `UiBindable` trait
- `HandlerRegistry`
- XML parser

**gravity-iced** (Iced backend only):
- `GravityWidgetBuilder` (Iced widgets)
- `HandlerMessage` (Iced message type)
- Style mapping to Iced types
- Iced-specific widget implementations

---

## Creating a New Backend

### Step 1: Create Backend Crate

**Directory**: `crates/gravity-{backend}/`

```toml
# crates/gravity-gtk/Cargo.toml
[package]
name = "gravity-gtk"
version = "0.1.0"
edition = "2024"

[dependencies]
gravity-core = { path = "../gravity-core" }
gtk4 = "0.9"  # Example: GTK backend
```

**File Structure**:
```
crates/gravity-gtk/
├── src/
│   ├── lib.rs          # Public API
│   ├── builder.rs      # GravityWidgetBuilder (GTK version)
│   ├── convert.rs      # IR → GTK type conversions
│   └── message.rs      # GTK message types
├── Cargo.toml
└── README.md
```

---

### Step 2: Implement Builder

**File**: `crates/gravity-gtk/src/builder.rs`

```rust
use gravity_core::binding::UiBindable;
use gravity_core::expr::evaluate_binding_expr;
use gravity_core::handler::HandlerRegistry;
use gravity_core::ir::node::{AttributeValue, WidgetNode};
use gravity_core::ir::WidgetKind;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Widget};

/// Builder for creating GTK widgets from Gravity markup
pub struct GravityWidgetBuilder<'a, Message> {
    node: &'a WidgetNode,
    model: &'a dyn UiBindable,
    handler_registry: Option<&'a HandlerRegistry>,
    verbose: bool,
    message_factory: Box<dyn Fn(&str) -> Message + 'a>,
}

impl<'a, Message> GravityWidgetBuilder<'a, Message> {
    /// Create a new GTK widget builder
    pub fn new(
        node: &'a WidgetNode,
        model: &'a dyn UiBindable,
        handler_registry: Option<&'a HandlerRegistry>,
        message_factory: impl Fn(&str) -> Message + 'a,
    ) -> Self {
        Self {
            node,
            model,
            handler_registry,
            verbose: false,
            message_factory: Box::new(message_factory),
        }
    }

    /// Build the widget tree
    pub fn build(self) -> Widget {
        self.build_widget(self.node)
    }

    /// Recursively build a widget
    fn build_widget(&self, node: &WidgetNode) -> Widget {
        match node.kind {
            WidgetKind::Text => self.build_text(node),
            WidgetKind::Button => self.build_button(node),
            WidgetKind::Column => self.build_column(node),
            WidgetKind::Row => self.build_row(node),
            // ... other widgets
            _ => Label::new(Some("Unsupported widget")).into(),
        }
    }

    /// Build a GTK Label (Text widget)
    fn build_text(&self, node: &WidgetNode) -> Widget {
        let value = node
            .attributes
            .get("value")
            .map(|v| self.evaluate_attribute(v))
            .unwrap_or_default();

        let label = Label::new(Some(&value));

        // Apply styling
        if let Some(size_attr) = node.attributes.get("size") {
            if let Ok(size) = self.evaluate_attribute(size_attr).parse::<i32>() {
                label.set_height_request(size);
            }
        }

        label.into()
    }

    /// Build a GTK Button
    fn build_button(&self, node: &WidgetNode) -> Widget {
        let label = node
            .attributes
            .get("label")
            .map(|v| self.evaluate_attribute(v))
            .unwrap_or_default();

        let button = Button::with_label(&label);

        // Connect event handler
        if let Some(on_click_attr) = node.attributes.get("on_click") {
            let handler_name = self.evaluate_attribute(on_click_attr);
            if let Some(_) = self.handler_registry {
                let message = (self.message_factory)(&handler_name);
                // GTK-specific event connection here
                // button.connect_clicked(move |_| { /* dispatch message */ });
            }
        }

        button.into()
    }

    /// Build a GTK Box (Column widget)
    fn build_column(&self, node: &WidgetNode) -> Widget {
        let vbox = Box::new(gtk4::Orientation::Vertical, 0);

        // Process children
        for child in &node.children {
            let child_widget = self.build_widget(child);
            vbox.append(&child_widget);
        }

        // Apply spacing
        if let Some(spacing_attr) = node.attributes.get("spacing") {
            if let Ok(spacing) = self.evaluate_attribute(spacing_attr).parse::<i32>() {
                vbox.set_spacing(spacing);
            }
        }

        vbox.into()
    }

    /// Evaluate attribute (same as Iced version)
    fn evaluate_attribute(&self, attr: &AttributeValue) -> String {
        match attr {
            AttributeValue::Static(value) => value.clone(),
            AttributeValue::Binding(expr) => {
                evaluate_binding_expr(expr, self.model)
                    .map(|v| v.to_display_string())
                    .unwrap_or_default()
            }
            AttributeValue::Interpolated(parts) => {
                // Same interpolation logic as Iced
                // ... (code omitted for brevity)
                String::new()
            }
        }
    }
}
```

---

### Step 3: Implement Message Type

**File**: `crates/gravity-gtk/src/message.rs`

```rust
/// GTK-compatible message type
#[derive(Clone, Debug)]
pub enum GtkMessage {
    Handler(String, Option<String>),
    Custom(Box<dyn std::any::Any>),
}
```

---

### Step 4: Export Public API

**File**: `crates/gravity-gtk/src/lib.rs`

```rust
mod builder;
mod convert;
mod message;

pub use builder::GravityWidgetBuilder;
pub use message::GtkMessage;
```

---

## Using the Alternative Backend

### Example Application (GTK)

```rust
use gravity_core::{parse, HandlerRegistry};
use gravity_gtk::{GravityWidgetBuilder, GtkMessage};
use gravity_macros::UiModel;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

#[derive(Default, UiModel, Serialize, Deserialize, Clone)]
struct Model {
    count: i32,
}

fn build_ui(app: &Application) {
    // Parse UI definition
    let xml = include_str!("../ui/main.gravity");
    let document = parse(xml).expect("Failed to parse XML");

    // Setup model and handlers
    let model = Model::default();
    let handler_registry = HandlerRegistry::new();

    // Register handlers
    handler_registry.register_simple("increment", |m: &mut dyn Any| {
        let model = m.downcast_mut::<Model>().unwrap();
        model.count += 1;
    });

    // Build GTK UI using builder
    let widget = GravityWidgetBuilder::new(
        &document.root,
        &model,
        Some(&handler_registry),
        |name| GtkMessage::Handler(name.to_string(), None),
    )
    .build();

    // Create window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Gravity GTK Example")
        .child(&widget)
        .build();

    window.present();
}

fn main() {
    let app = Application::new(Some("com.example.gravity"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}
```

**XML** (same as Iced version):
```xml
<column padding="20" spacing="10">
    <text value="GTK Counter" size="24" />
    <text value="Count: {count}" />
    <button label="Increment" on_click="increment" />
</column>
```

**Result**: The same XML works for both Iced and GTK backends!

---

## Backend Comparison

### Iced Backend

**Pros**:
- Pure Rust, no C dependencies
- Cross-platform (Windows, Linux, macOS, Web)
- Modern, GPU-accelerated rendering
- Reactive architecture

**Example**:
```rust
use gravity_iced::GravityWidgetBuilder;

let element = GravityWidgetBuilder::new(
    &document.root,
    &model,
    Some(&registry)
).build();
```

### GTK Backend (Hypothetical)

**Pros**:
- Native Linux integration
- Accessibility support (a11y)
- Rich widget ecosystem
- System theme integration

**Example**:
```rust
use gravity_gtk::GravityWidgetBuilder;

let widget = GravityWidgetBuilder::new(
    &document.root,
    &model,
    Some(&registry),
    |name| GtkMessage::Handler(name, None)
).build();
```

### Qt Backend (Hypothetical)

**Pros**:
- Cross-platform native look
- Enterprise support
- Rich tooling (Qt Designer, etc.)

**Example**:
```rust
use gravity_qt::GravityWidgetBuilder;

let widget = GravityWidgetBuilder::new(
    &document.root,
    &model,
    Some(&registry),
    |name| QtMessage::Signal(name)
).build();
```

---

## Shared vs Backend-Specific

### Shared (gravity-core)

✅ **XML Parsing**: Same parser for all backends  
✅ **IR Structure**: `WidgetNode`, `WidgetKind`, `AttributeValue`  
✅ **Binding Evaluation**: `evaluate_binding_expr()`  
✅ **Handler Registry**: Event dispatch mechanism  
✅ **Type Safety**: `UiBindable` trait  

**Example** (works everywhere):
```xml
<text value="{user.name}" />
<button label="Click" on_click="handle_click" />
```

### Backend-Specific

❌ **Widget Implementation**: Iced vs GTK vs Qt widgets  
❌ **Message Types**: `Element<Message>` vs `gtk::Widget`  
❌ **Style Application**: Iced themes vs GTK CSS vs Qt stylesheets  
❌ **Event Handling**: Backend-specific dispatch  

**Example** (Iced-specific):
```rust
use iced::Element;  // Backend-specific type
let element: Element<'_, Message> = builder.build();
```

**Example** (GTK-specific):
```rust
use gtk4::Widget;  // Backend-specific type
let widget: Widget = builder.build();
```

---

## Testing Alternative Backends

### Unit Tests

**File**: `crates/gravity-gtk/tests/builder_tests.rs`

```rust
#[test]
fn test_gtk_text_widget() {
    let xml = r#"<text value="Hello GTK" />"#;
    let document = parse(xml).unwrap();
    
    let model = EmptyModel;
    let widget = GravityWidgetBuilder::new(&document.root, &model, None, |_| GtkMessage::Custom)
        .build();
    
    assert!(widget.is::<Label>());
}
```

### Integration Tests

**File**: `crates/gravity-gtk/tests/integration_tests.rs`

```rust
#[test]
fn test_gtk_ui_from_xml() {
    #[derive(UiModel, Serialize, Deserialize, Clone)]
    struct TestModel {
        count: i32,
    }
    
    let xml = r#"
        <column>
            <text value="Count: {count}" />
            <button label="Inc" on_click="increment" />
        </column>
    "#;
    
    let document = parse(xml).unwrap();
    let model = TestModel { count: 5 };
    
    let widget = GravityWidgetBuilder::new(&document.root, &model, None, |_| GtkMessage::Custom)
        .build();
    
    assert!(widget.is::<Box>());
}
```

---

## Migration Between Backends

### Example: Iced → GTK

**Before** (Iced):
```toml
[dependencies]
gravity-iced = { path = "../../crates/gravity-iced" }
iced = "0.14"
```

```rust
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};

fn view(state: &AppState) -> Element<'_, HandlerMessage> {
    GravityWidgetBuilder::new(
        &state.document.root,
        &state.model,
        Some(&state.registry)
    ).build()
}
```

**After** (GTK):
```toml
[dependencies]
gravity-gtk = { path = "../../crates/gravity-gtk" }
gtk4 = "0.9"
```

```rust
use gravity_gtk::{GravityWidgetBuilder, GtkMessage};

fn build_ui(model: &Model, registry: &HandlerRegistry) -> gtk4::Widget {
    GravityWidgetBuilder::new(
        &document.root,
        model,
        Some(registry),
        |name| GtkMessage::Handler(name.to_string(), None)
    ).build()
}
```

**XML**: No changes needed!

---

## Best Practices for Backend Implementers

### 1. Follow Iced Pattern

Use `gravity-iced` as reference implementation:
- Same builder structure
- Same attribute evaluation logic
- Same error handling approach
- Same verbose logging pattern

### 2. Maintain API Compatibility

Public API should mirror Iced version:
```rust
pub struct GravityWidgetBuilder<'a, Message> { ... }

impl<'a, Message> GravityWidgetBuilder<'a, Message> {
    pub fn new(...) -> Self;
    pub fn with_verbose(self, bool) -> Self;
    pub fn build(self) -> BackendWidget;
}
```

### 3. Reuse gravity-core

Never duplicate parsing/evaluation logic:
```rust
// ✅ Good: Reuse core
use gravity_core::expr::evaluate_binding_expr;

// ❌ Bad: Duplicate logic
fn my_evaluate_binding(...) { /* custom implementation */ }
```

### 4. Document Differences

Clearly document backend-specific features:
```rust
/// Build a GTK button
///
/// # GTK-Specific Notes
/// - Uses GtkButton instead of iced::widget::button
/// - Styling applied via CSS classes
/// - Events connected via GObject signals
fn build_button(&self, node: &WidgetNode) -> gtk4::Widget { ... }
```

---

## Summary

**Architecture**: Backend-agnostic core, backend-specific builders  
**Reusability**: Same XML works across all backends  
**Isolation**: Zero coupling between backends  
**Extensibility**: Add backends without modifying core  
**Compatibility**: Examples switch backends via dependencies only  

**Verification**:
- ✅ gravity-core has no backend imports
- ✅ Builder pattern consistent across backends
- ✅ XML compatibility guaranteed
- ✅ Examples portable with minimal changes

**Conclusion**: The centralized builder architecture successfully enables multiple backend support while preserving type safety and avoiding code duplication.
