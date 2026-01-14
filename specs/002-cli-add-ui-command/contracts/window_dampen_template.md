# Contract: Dampen XML Template (window.dampen.template)

**Feature**: 002-cli-add-ui-command  
**Template**: `crates/dampen-cli/templates/add/window.dampen.template`  
**Purpose**: Generate XML UI definition file for new UI window

## Template Contract

### Placeholders

| Placeholder | Description | Example Value |
|-------------|-------------|---------------|
| `{{WINDOW_NAME_TITLE}}` | Title Case window name (for display text) | `Settings` |

**Note**: `{{WINDOW_NAME}}` and `{{WINDOW_NAME_PASCAL}}` are not used in XML files (only in Rust).

### Required Elements

The template MUST contain:

1. **XML declaration**
   - Version: `1.0`
   - Encoding: `UTF-8`

2. **Root `<dampen>` element**
   - Attribute: `version="1.0"`

3. **Basic UI structure**
   - Container widget (e.g., `<column>`)
   - At least one text element
   - At least one interactive widget (e.g., `<button>`)
   - At least one data binding example (e.g., `{field_name}`)

4. **Example functionality**
   - Button with `on_click` handler
   - Text with binding to model field
   - Demonstrates basic interaction pattern

### Template Content

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<dampen version="1.0">
    <column padding="40" spacing="20">
        <text value="{{WINDOW_NAME_TITLE}}" size="32" weight="bold" />
        <button label="Click me!" on_click="example_handler" />
        <text value="{message}" size="24" />
    </column>
</dampen>
```

### Layout Guidelines

**Container widget** should use:
- `padding="40"` - Comfortable spacing from edges
- `spacing="20"` - Visual separation between children

**Text elements** should use:
- Title text: `size="32"`, `weight="bold"`
- Body text: `size="24"` (default weight)

**Interactive widgets** should:
- Reference handler names matching those in `.rs` file
- Use descriptive labels

**Data bindings** should:
- Reference field names from Model struct
- Use curly brace syntax: `{field_name}`

### Validation Rules

**After rendering**, the generated file MUST:

1. ✅ Be valid XML (parseable by roxmltree)
2. ✅ Pass `dampen check` validation
3. ✅ Contain valid Dampen widgets (column, text, button)
4. ✅ Have valid attribute values (numeric sizes, valid handler names)
5. ✅ Include at least one data binding
6. ✅ Reference handler names that exist in template `.rs` file

### Widget Requirements

**Supported widgets** (from existing Dampen):
- `<column>` - Vertical layout
- `<row>` - Horizontal layout
- `<text>` - Display text
- `<button>` - Interactive button
- `<checkbox>` - Boolean input
- `<radio>` - Single selection
- `<text_input>` - Text entry

**Attributes** (from existing schemas):
- Layout: `padding`, `spacing`, `align`
- Text: `value`, `size`, `weight`, `color`
- Interactive: `label`, `on_click`, `on_change`, `disabled`
- Binding: Curly brace syntax `{field}`

### Contract Tests

```rust
#[test]
fn test_dampen_template_structure() {
    let window_name = WindowName::new("settings").unwrap();
    let template = WindowTemplate::load(TemplateKind::DampenXml);
    let rendered = template.render(&window_name);
    
    // Must be valid XML
    let doc = roxmltree::Document::parse(&rendered).unwrap();
    
    // Must have dampen root
    let root = doc.root_element();
    assert_eq!(root.tag_name().name(), "dampen");
    assert_eq!(root.attribute("version"), Some("1.0"));
    
    // Must have container widget
    let container = root.first_element_child().unwrap();
    assert!(["column", "row"].contains(&container.tag_name().name()));
    
    // Must have at least 3 children (title, button, text with binding)
    assert!(container.children().filter(|n| n.is_element()).count() >= 3);
}

#[test]
fn test_dampen_template_widgets() {
    let window_name = WindowName::new("settings").unwrap();
    let template = WindowTemplate::load(TemplateKind::DampenXml);
    let rendered = template.render(&window_name);
    
    // Must contain text widget with title
    assert!(rendered.contains(r#"<text"#));
    assert!(rendered.contains("Settings")); // Title case name
    
    // Must contain button with handler
    assert!(rendered.contains(r#"<button"#));
    assert!(rendered.contains(r#"on_click="#));
    
    // Must contain data binding
    assert!(rendered.contains("{message}"));
}

#[test]
fn test_dampen_template_placeholder_replacement() {
    let window_name = WindowName::new("UserProfile").unwrap();
    let template = WindowTemplate::load(TemplateKind::DampenXml);
    let rendered = template.render(&window_name);
    
    // Title case in display text
    assert!(rendered.contains("User Profile"));
    
    // No unreplaced placeholders
    assert!(!rendered.contains("{{"));
    assert!(!rendered.contains("}}"));
}

#[test]
fn test_dampen_template_validates() {
    // Integration test: write generated file and run dampen check
    let temp_dir = tempfile::tempdir().unwrap();
    let window_name = WindowName::new("test_window").unwrap();
    
    // Generate files
    generate_window_files(&temp_dir.path(), &window_name).unwrap();
    
    // Should validate with dampen check
    let output = std::process::Command::new("dampen")
        .arg("check")
        .arg(temp_dir.path().join("test_window.dampen"))
        .output()
        .unwrap();
    
    assert!(output.status.success(), "Generated .dampen file failed validation");
}
```

### Example Variations

**Minimal template**:
```xml
<?xml version="1.0" encoding="UTF-8" ?>
<dampen version="1.0">
    <column padding="40" spacing="20">
        <text value="{{WINDOW_NAME_TITLE}}" size="32" />
    </column>
</dampen>
```

**Rich template** (for future enhancement):
```xml
<?xml version="1.0" encoding="UTF-8" ?>
<dampen version="1.0">
    <column padding="40" spacing="20">
        <text value="{{WINDOW_NAME_TITLE}}" size="32" weight="bold" />
        
        <row spacing="10">
            <button label="Action 1" on_click="handler_one" />
            <button label="Action 2" on_click="handler_two" />
        </row>
        
        <text_input placeholder="Enter text..." value="{input_text}" />
        
        <text value="{status_message}" size="18" />
        
        <checkbox label="Enable feature" checked="{feature_enabled}" />
    </column>
</dampen>
```

### XML Formatting

**Indentation**: 4 spaces per level
**Line breaks**: Between sibling elements
**Attributes**: 
- One attribute per widget for simple cases
- Multiple attributes on separate lines for complex widgets (not in basic template)

### Change History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-01-13 | Initial template contract |

### Related Contracts

- [window.rs.template](./window_rs_template.md) - Rust module template
- [data-model.md](../data-model.md) - WindowName structure
- `/specs/001-framework-technical-specs/contracts/xml-schema.md` - Dampen XML specification
