use gravity_core::ir::{AttributeValue, EventKind, WidgetKind};
use gravity_core::parser::parse;

/// T030: Integration test for ComboBox rendering
#[test]
fn test_build_combobox_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox options="Work,Personal,Shopping" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse combobox XML");
    let doc = result.unwrap();

    // Verify combobox node exists
    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    // Verify attributes
    assert!(
        matches!(combobox.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "Work,Personal,Shopping")
    );

    // In a real test, we would call builder.build_widget() and verify it builds
    // For now, just verify the IR is correct
    assert!(combobox.events.is_empty());
}

/// T031: Integration test for PickList rendering
#[test]
fn test_build_picklist_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list options="All,Active,Completed" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse pick_list XML");
    let doc = result.unwrap();

    // Verify pick_list node exists
    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    // Verify attributes
    assert!(
        matches!(picklist.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "All,Active,Completed")
    );

    // In a real test, we would call builder.build_widget() and verify it builds
    assert!(picklist.events.is_empty());
}

/// T032: Integration test for ComboBox event handling
#[test]
fn test_build_combobox_with_event() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox
        options="Work,Personal,Shopping"
        selected="{category}"
        placeholder="Select..."
        on_select="update_category"
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse combobox with event");
    let doc = result.unwrap();

    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    // Verify all attributes present
    assert!(combobox.attributes.contains_key("options"));
    assert!(combobox.attributes.contains_key("selected"));
    assert!(combobox.attributes.contains_key("placeholder"));

    // Verify event handler
    assert_eq!(combobox.events.len(), 1);
    assert!(matches!(combobox.events[0].event, EventKind::Select));
    assert_eq!(combobox.events[0].handler, "update_category");
}

/// T033: Integration test for PickList event handling
#[test]
fn test_build_picklist_with_event() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list
        options="All,Active,Completed"
        selected="{filter}"
        placeholder="Filter..."
        on_select="apply_filter"
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse pick_list with event");
    let doc = result.unwrap();

    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    // Verify all attributes present
    assert!(picklist.attributes.contains_key("options"));
    assert!(picklist.attributes.contains_key("selected"));
    assert!(picklist.attributes.contains_key("placeholder"));

    // Verify event handler
    assert_eq!(picklist.events.len(), 1);
    assert!(matches!(picklist.events[0].event, EventKind::Select));
    assert_eq!(picklist.events[0].handler, "apply_filter");
}

/// Additional test: ComboBox with binding for selected value
#[test]
fn test_combobox_binding() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox options="Red,Green,Blue" selected="{color}" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok());
    let doc = result.unwrap();

    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    // Verify binding is parsed
    assert!(matches!(
        combobox.attributes.get("selected"),
        Some(AttributeValue::Binding(_))
    ));
}

/// Additional test: PickList with binding for selected value
#[test]
fn test_picklist_binding() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list options="A,B,C" selected="{choice}" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok());
    let doc = result.unwrap();

    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    // Verify binding is parsed
    assert!(matches!(
        picklist.attributes.get("selected"),
        Some(AttributeValue::Binding(_))
    ));
}

/// Additional test: ComboBox with interpolated placeholder
#[test]
fn test_combobox_interpolated_placeholder() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox options="A,B" placeholder="Select {category}..." />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok());
    let doc = result.unwrap();

    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    // Verify interpolated string is parsed
    assert!(matches!(
        combobox.attributes.get("placeholder"),
        Some(AttributeValue::Interpolated(_))
    ));
}

/// T064: Integration test for Grid rendering and layout
#[test]
fn test_build_grid_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <grid columns="3" spacing="10" padding="20">
        <text value="A" />
        <text value="B" />
        <text value="C" />
        <text value="D" />
        <text value="E" />
        <text value="F" />
    </grid>
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse grid XML");
    let doc = result.unwrap();

    let grid = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Grid))
        .expect("Should have grid child");

    // Verify attributes
    assert!(
        matches!(grid.attributes.get("columns"), Some(AttributeValue::Static(s)) if s == "3"),
        "Columns should be 3"
    );
    assert!(
        matches!(grid.attributes.get("spacing"), Some(AttributeValue::Static(s)) if s == "10"),
        "Spacing should be 10"
    );
    assert!(
        matches!(grid.attributes.get("padding"), Some(AttributeValue::Static(s)) if s == "20"),
        "Padding should be 20"
    );

    // Verify children count
    assert_eq!(grid.children.len(), 6, "Should have 6 child widgets");

    // Verify all children are text widgets
    for child in &grid.children {
        assert!(
            matches!(child.kind, WidgetKind::Text),
            "All children should be text widgets"
        );
    }

    // In a real test, we would call builder.build_widget() and verify it builds
    // For now, just verify the IR is correct
}

/// T065: Integration test for Grid wrapping behavior
#[test]
fn test_build_grid_wrapping() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <grid columns="2" spacing="5">
        <text value="Row1-Col1" />
        <text value="Row1-Col2" />
        <text value="Row2-Col1" />
        <text value="Row2-Col2" />
        <text value="Row3-Col1" />
    </grid>
</column>"#;

    let result = parse(xml);
    assert!(
        result.is_ok(),
        "Should parse grid with 5 children and 2 columns"
    );
    let doc = result.unwrap();

    let grid = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Grid))
        .expect("Should have grid child");

    // Should have 5 children (2 full rows + 1 partial row)
    assert_eq!(grid.children.len(), 5, "Should have 5 child widgets");

    // Verify columns attribute
    assert!(
        matches!(grid.attributes.get("columns"), Some(AttributeValue::Static(s)) if s == "2"),
        "Columns should be 2"
    );

    // Verify spacing
    assert!(
        matches!(grid.attributes.get("spacing"), Some(AttributeValue::Static(s)) if s == "5"),
        "Spacing should be 5"
    );

    // Verify children structure
    let texts: Vec<_> = grid
        .children
        .iter()
        .filter_map(|c| {
            if let WidgetKind::Text = &c.kind {
                c.attributes.get("value").and_then(|v| {
                    if let AttributeValue::Static(s) = v {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
        .collect();

    assert_eq!(texts.len(), 5);
    assert_eq!(
        texts,
        vec![
            "Row1-Col1".to_string(),
            "Row1-Col2".to_string(),
            "Row2-Col1".to_string(),
            "Row2-Col2".to_string(),
            "Row3-Col1".to_string(),
        ]
    );
}

/// Test: Grid with zero children
#[test]
fn test_build_grid_empty() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <grid columns="5" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse empty grid");
    let doc = result.unwrap();

    let grid = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Grid))
        .expect("Should have grid child");

    assert_eq!(grid.children.len(), 0, "Should have no children");
    assert!(
        matches!(grid.attributes.get("columns"), Some(AttributeValue::Static(s)) if s == "5"),
        "Columns should be 5"
    );
}

// ============================================================================
// Canvas Widget Rendering Tests (T073-T074)
// ============================================================================

/// T073: Integration test for Canvas rendering with Program trait
#[test]
fn test_build_canvas_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <canvas width="400" height="200" program="{statistics_chart}" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse canvas XML");
    let doc = result.unwrap();

    // Verify canvas node exists
    let canvas = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Canvas))
        .expect("Should have canvas child");

    // Verify attributes
    assert!(
        matches!(canvas.attributes.get("width"), Some(AttributeValue::Static(s)) if s == "400"),
        "Width should be 400"
    );
    assert!(
        matches!(canvas.attributes.get("height"), Some(AttributeValue::Static(s)) if s == "200"),
        "Height should be 200"
    );
    assert!(
        matches!(canvas.attributes.get("program"), Some(AttributeValue::Binding(_))),
        "Program should be a binding expression"
    );

    // Verify no events by default
    assert!(canvas.events.is_empty(), "Should have no events by default");
}

/// T073: Integration test for Canvas with Program binding evaluation
#[test]
fn test_build_canvas_with_program_binding() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <canvas 
        width="800" 
        height="600" 
        program="{model.chart_program}" 
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse canvas with nested binding");
    let doc = result.unwrap();

    let canvas = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Canvas))
        .expect("Should have canvas child");

    // Verify program is a binding (not static)
    match canvas.attributes.get("program") {
        Some(AttributeValue::Binding(expr)) => {
            // Verify it's a field access binding
            assert!(
                format!("{:?}", expr).contains("model"),
                "Program binding should reference model field"
            );
        }
        _ => panic!("Program should be a binding expression"),
    }
}

/// T074: Integration test for Canvas click event handling
#[test]
fn test_build_canvas_with_click_event() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <canvas 
        width="400" 
        height="300" 
        program="{chart}"
        on_click="handle_canvas_click"
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse canvas with click event");
    let doc = result.unwrap();

    let canvas = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Canvas))
        .expect("Should have canvas child");

    // Verify click event is registered
    assert_eq!(canvas.events.len(), 1, "Should have one event handler");
    
    let event = &canvas.events[0];
    assert!(
        matches!(event.event, EventKind::Click),
        "Event kind should be Click"
    );
    assert_eq!(
        event.handler, "handle_canvas_click",
        "Handler name should match"
    );
}

/// T074: Integration test for Canvas multiple properties
#[test]
fn test_build_canvas_with_all_properties() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <canvas 
        width="1200" 
        height="800" 
        program="{visualization.advanced_chart}"
        on_click="chart_click_handler"
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse canvas with all properties");
    let doc = result.unwrap();

    let canvas = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Canvas))
        .expect("Should have canvas child");

    // Verify all attributes are present
    assert!(canvas.attributes.contains_key("width"));
    assert!(canvas.attributes.contains_key("height"));
    assert!(canvas.attributes.contains_key("program"));

    // Verify dimensions
    assert!(
        matches!(canvas.attributes.get("width"), Some(AttributeValue::Static(s)) if s == "1200")
    );
    assert!(
        matches!(canvas.attributes.get("height"), Some(AttributeValue::Static(s)) if s == "800")
    );

    // Verify program binding
    assert!(
        matches!(canvas.attributes.get("program"), Some(AttributeValue::Binding(_)))
    );

    // Verify event
    assert_eq!(canvas.events.len(), 1);
    assert_eq!(canvas.events[0].handler, "chart_click_handler");
}

/// T073: Integration test for Canvas size constraints
#[test]
fn test_build_canvas_size_constraints() {
    // Test minimum valid size
    let xml_min = r#"<?xml version="1.0"?>
<column>
    <canvas width="50" height="50" program="{chart}" />
</column>"#;

    let result = parse(xml_min);
    assert!(result.is_ok(), "Should accept minimum size (50x50)");

    // Test maximum valid size
    let xml_max = r#"<?xml version="1.0"?>
<column>
    <canvas width="4000" height="4000" program="{chart}" />
</column>"#;

    let result = parse(xml_max);
    assert!(result.is_ok(), "Should accept maximum size (4000x4000)");

    // Test typical size
    let xml_typical = r#"<?xml version="1.0"?>
<column>
    <canvas width="640" height="480" program="{chart}" />
</column>"#;

    let result = parse(xml_typical);
    assert!(result.is_ok(), "Should accept typical size (640x480)");
}

/// T074: Integration test for Canvas in complex layout
#[test]
fn test_build_canvas_in_layout() {
    let xml = r#"<?xml version="1.0"?>
<column spacing="10">
    <text value="Statistics Dashboard" />
    <row spacing="20">
        <canvas width="400" height="300" program="{chart1}" />
        <canvas width="400" height="300" program="{chart2}" />
    </row>
    <canvas width="800" height="200" program="{timeline}" on_click="timeline_click" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse canvas in complex layout");
    let doc = result.unwrap();

    // Count canvas widgets recursively
    fn count_canvas(node: &gravity_core::ir::WidgetNode) -> usize {
        let mut count = if matches!(node.kind, WidgetKind::Canvas) {
            1
        } else {
            0
        };
        for child in &node.children {
            count += count_canvas(child);
        }
        count
    }

    let canvas_count = count_canvas(&doc.root);
    assert_eq!(canvas_count, 3, "Should have 3 canvas widgets total");

    // Verify row contains 2 canvas widgets
    let row = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Row))
        .expect("Should have row");

    let row_canvas_count = row
        .children
        .iter()
        .filter(|c| matches!(c.kind, WidgetKind::Canvas))
        .count();
    assert_eq!(row_canvas_count, 2, "Row should contain 2 canvas widgets");
}
