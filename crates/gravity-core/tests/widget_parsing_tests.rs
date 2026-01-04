use gravity_core::ir::{AttributeValue, EventKind, WidgetKind};
use gravity_core::parser::parse;

/// T024: Contract test for ComboBox XML parsing
#[test]
fn parse_combobox_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox options="Work,Personal,Shopping" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse valid combobox");
    let doc = result.unwrap();

    // Find combobox child
    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    assert_eq!(combobox.attributes.len(), 1);
    assert!(
        matches!(combobox.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "Work,Personal,Shopping")
    );
}

/// T025: Contract test for PickList XML parsing
#[test]
fn parse_picklist_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list options="All,Active,Completed" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse valid pick_list");
    let doc = result.unwrap();

    // Find pick_list child
    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    assert_eq!(picklist.attributes.len(), 1);
    assert!(
        matches!(picklist.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "All,Active,Completed")
    );
}

/// T026: Contract test for ComboBox with all attributes
#[test]
fn parse_combobox_with_all_attributes() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox 
        options="Work,Personal,Shopping,Other"
        selected="{category}"
        placeholder="Select category..."
        on_select="update_category"
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse combobox with all attributes");
    let doc = result.unwrap();

    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    // Check options
    assert!(
        matches!(combobox.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "Work,Personal,Shopping,Other")
    );

    // Check selected binding
    assert!(combobox.attributes.get("selected").is_some());

    // Check placeholder
    assert!(
        matches!(combobox.attributes.get("placeholder"), Some(AttributeValue::Static(s)) if s == "Select category...")
    );

    // Check event handler
    assert_eq!(combobox.events.len(), 1);
    assert!(matches!(combobox.events[0].event, EventKind::Select));
    assert_eq!(combobox.events[0].handler, "update_category");
}

/// T027: Contract test for PickList with all attributes
#[test]
fn parse_picklist_with_all_attributes() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list 
        options="All,Active,Completed"
        selected="{filter}"
        placeholder="Filter tasks..."
        on_select="apply_filter"
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse pick_list with all attributes");
    let doc = result.unwrap();

    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    // Check options
    assert!(
        matches!(picklist.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "All,Active,Completed")
    );

    // Check selected binding
    assert!(picklist.attributes.get("selected").is_some());

    // Check placeholder
    assert!(
        matches!(picklist.attributes.get("placeholder"), Some(AttributeValue::Static(s)) if s == "Filter tasks...")
    );

    // Check event handler
    assert_eq!(picklist.events.len(), 1);
    assert!(matches!(picklist.events[0].event, EventKind::Select));
    assert_eq!(picklist.events[0].handler, "apply_filter");
}

/// T028: Contract test for ComboBox missing required attributes
#[test]
fn parse_combobox_missing_options_errors() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox />
</column>"#;

    let result = parse(xml);

    // Should fail with parse error about missing required attribute
    assert!(result.is_err(), "Should fail when options is missing");
    let err = result.unwrap_err();
    assert!(err.to_string().to_lowercase().contains("options"));
}

/// T029: Contract test for PickList missing required attributes
#[test]
fn parse_picklist_missing_options_errors() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list />
</column>"#;

    let result = parse(xml);

    // Should fail with parse error about missing required attribute
    assert!(result.is_err(), "Should fail when options is missing");
    let err = result.unwrap_err();
    assert!(err.to_string().to_lowercase().contains("options"));
}

/// Additional test: ComboBox with empty options should error
#[test]
fn parse_combobox_empty_options_errors() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <combobox options="" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_err(), "Should fail when options is empty");
}

/// Additional test: PickList with single option
#[test]
fn parse_picklist_single_option() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <pick_list options="OnlyOption" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse pick_list with single option");
    let doc = result.unwrap();

    let picklist = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::PickList))
        .expect("Should have pick_list child");

    assert!(
        matches!(picklist.attributes.get("options"), Some(AttributeValue::Static(s)) if s == "OnlyOption")
    );
}

/// Additional test: ComboBox with many options
#[test]
fn parse_combobox_many_options() {
    let options = (1..=50)
        .map(|i| format!("Option{}", i))
        .collect::<Vec<_>>()
        .join(",");
    let xml = format!(
        r#"<?xml version="1.0"?>
<column>
    <combobox options="{}" />
</column>"#,
        options
    );

    let result = parse(&xml);
    assert!(result.is_ok(), "Should parse combobox with many options");
    let doc = result.unwrap();

    let combobox = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ComboBox))
        .expect("Should have combobox child");

    assert!(combobox.attributes.get("options").is_some());
}

/// T044: Contract test for ProgressBar XML parsing
#[test]
fn parse_progressbar_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <progress_bar min="0" max="100" value="{percent}" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse valid progress_bar");
    let doc = result.unwrap();

    let progressbar = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ProgressBar))
        .expect("Should have progress_bar child");

    assert_eq!(progressbar.attributes.len(), 3);
    assert!(
        matches!(progressbar.attributes.get("min"), Some(AttributeValue::Static(s)) if s == "0")
    );
    assert!(
        matches!(progressbar.attributes.get("max"), Some(AttributeValue::Static(s)) if s == "100")
    );
    assert!(progressbar.attributes.get("value").is_some());
}

/// T045: Contract test for Tooltip XML parsing
#[test]
fn parse_tooltip_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <tooltip message="Help text">
        <button label="Action" />
    </tooltip>
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse valid tooltip");
    let doc = result.unwrap();

    let tooltip = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Tooltip))
        .expect("Should have tooltip child");

    assert_eq!(tooltip.children.len(), 1);
    assert!(
        matches!(tooltip.attributes.get("message"), Some(AttributeValue::Static(s)) if s == "Help text")
    );
}

/// T046: Contract test for ProgressBar with style variants
#[test]
fn parse_progressbar_with_style() {
    let styles = ["primary", "success", "warning", "danger", "secondary"];

    for style in styles {
        let xml = format!(
            r#"<?xml version="1.0"?>
<column>
    <progress_bar value="50" style="{}" />
</column>"#,
            style
        );

        let result = parse(&xml);
        assert!(
            result.is_ok(),
            "Should parse progress_bar with style: {}",
            style
        );
        let doc = result.unwrap();

        let progressbar = doc
            .root
            .children
            .iter()
            .find(|c| matches!(c.kind, WidgetKind::ProgressBar))
            .expect("Should have progress_bar child");

        assert!(
            matches!(progressbar.attributes.get("style"), Some(AttributeValue::Static(s)) if s == style),
            "Style attribute should match: {}",
            style
        );
    }
}

/// T047: Contract test for Tooltip with position variants
#[test]
fn parse_tooltip_with_position() {
    let positions = ["follow_cursor", "top", "bottom", "left", "right"];

    for position in positions {
        let xml = format!(
            r#"<?xml version="1.0"?>
<column>
    <tooltip message="Tip" position="{}">
        <text value="Hover me" />
    </tooltip>
</column>"#,
            position
        );

        let result = parse(&xml);
        let doc = result.expect(&format!("Should parse tooltip with position: {}", position));

        let tooltip = doc
            .root
            .children
            .iter()
            .find(|c| matches!(c.kind, WidgetKind::Tooltip))
            .expect("Should have tooltip child");

        assert!(
            matches!(tooltip.attributes.get("position"), Some(AttributeValue::Static(s)) if s == position),
            "Position attribute should match: {}",
            position
        );
    }
}

/// T048: Contract test for Tooltip child count validation
#[test]
fn parse_tooltip_must_have_exactly_one_child() {
    // Test with no children
    let xml_empty = r#"<?xml version="1.0"?>
<column>
    <tooltip message="Tip" />
</column>"#;

    let result_empty = parse(xml_empty);
    assert!(
        result_empty.is_err(),
        "Should fail when tooltip has no children"
    );

    // Test with multiple children
    let xml_multiple = r#"<?xml version="1.0"?>
<column>
    <tooltip message="Tip">
        <text value="First" />
        <text value="Second" />
    </tooltip>
</column>"#;

    let result_multiple = parse(xml_multiple);
    assert!(
        result_multiple.is_err(),
        "Should fail when tooltip has multiple children"
    );
}

/// Additional test: ProgressBar with default values (no min/max)
#[test]
fn parse_progressbar_defaults() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <progress_bar value="{progress}" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse progress_bar with defaults");
    let doc = result.unwrap();

    let progressbar = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::ProgressBar))
        .expect("Should have progress_bar child");

    assert!(progressbar.attributes.get("value").is_some());
    assert!(progressbar.attributes.get("min").is_none());
    assert!(progressbar.attributes.get("max").is_none());
}

/// Additional test: Tooltip with delay attribute
#[test]
fn parse_tooltip_with_delay() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <tooltip message="Tip" delay="1000">
        <button label="Hover" />
    </tooltip>
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse tooltip with delay");
    let doc = result.unwrap();

    let tooltip = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Tooltip))
        .expect("Should have tooltip child");

    assert!(
        matches!(tooltip.attributes.get("delay"), Some(AttributeValue::Static(s)) if s == "1000"),
        "Delay attribute should be 1000ms"
    );
}

/// T061: Contract test for Grid XML parsing
#[test]
fn parse_grid_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <grid columns="3" spacing="10">
        <text value="Item 1" />
        <text value="Item 2" />
        <text value="Item 3" />
    </grid>
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse valid grid");
    let doc = result.unwrap();

    let grid = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Grid))
        .expect("Should have grid child");

    assert_eq!(grid.children.len(), 3, "Should have 3 child widgets");
    assert_eq!(grid.attributes.len(), 2);
    assert!(
        matches!(grid.attributes.get("columns"), Some(AttributeValue::Static(s)) if s == "3"),
        "Columns attribute should be 3"
    );
    assert!(
        matches!(grid.attributes.get("spacing"), Some(AttributeValue::Static(s)) if s == "10"),
        "Spacing attribute should be 10"
    );
}

/// T062: Contract test for Grid with varying child counts
#[test]
fn parse_grid_varying_children() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <grid columns="2" spacing="5" padding="10">
        <text value="A" />
        <text value="B" />
        <text value="C" />
    </grid>
</column>"#;

    let result = parse(xml);
    assert!(
        result.is_ok(),
        "Should parse grid with 3 children and 2 columns"
    );
    let doc = result.unwrap();

    let grid = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Grid))
        .expect("Should have grid child");

    assert_eq!(grid.children.len(), 3, "Should have 3 child widgets");
    assert!(
        matches!(grid.attributes.get("columns"), Some(AttributeValue::Static(s)) if s == "2"),
        "Columns attribute should be 2"
    );
    assert!(
        matches!(grid.attributes.get("spacing"), Some(AttributeValue::Static(s)) if s == "5"),
        "Spacing attribute should be 5"
    );
    assert!(
        matches!(grid.attributes.get("padding"), Some(AttributeValue::Static(s)) if s == "10"),
        "Padding attribute should be 10"
    );
}

/// T063: Contract test for Grid column validation (min 1, max 20)
#[test]
fn parse_grid_columns_validation() {
    let xml_min = r#"<?xml version="1.0"?>
<column>
    <grid columns="0">
        <text value="Test" />
    </grid>
</column>"#;

    let result_min = parse(xml_min);
    assert!(result_min.is_err(), "Should fail when columns < 1");

    let xml_max = r#"<?xml version="1.0"?>
<column>
    <grid columns="21">
        <text value="Test" />
    </grid>
</column>"#;

    let result_max = parse(xml_max);
    assert!(result_max.is_err(), "Should fail when columns > 20");

    let xml_valid = r#"<?xml version="1.0"?>
<column>
    <grid columns="5">
        <text value="Test" />
    </grid>
</column>"#;

    let result_valid = parse(xml_valid);
    assert!(
        result_valid.is_ok(),
        "Should succeed when columns in [1, 20] range"
    );
}

/// Test: Grid with no children should parse
#[test]
fn parse_grid_no_children() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <grid columns="3" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse grid with no children");
    let doc = result.unwrap();

    let grid = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Grid))
        .expect("Should have grid child");

    assert_eq!(grid.children.len(), 0, "Should have 0 child widgets");
}

/// Test: Grid missing columns attribute should error
#[test]
fn parse_grid_missing_columns_errors() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <grid spacing="10">
        <text value="Test" />
    </grid>
</column>"#;

    let result = parse(xml);
    assert!(result.is_err(), "Should fail when columns is missing");
    let err = result.unwrap_err();
    assert!(
        err.to_string().to_lowercase().contains("columns"),
        "Error should mention columns"
    );
}

// ============================================================================
// Canvas Widget Tests (T071-T072)
// ============================================================================

/// T071: Contract test for Canvas XML parsing
#[test]
fn parse_canvas_basic() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <canvas width="400" height="200" program="{statistics_chart}" />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse valid canvas");
    let doc = result.unwrap();

    // Find canvas child
    let canvas = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Canvas))
        .expect("Should have canvas child");

    assert!(
        matches!(canvas.attributes.get("width"), Some(AttributeValue::Static(s)) if s == "400"),
        "Width attribute should be 400"
    );
    assert!(
        matches!(canvas.attributes.get("height"), Some(AttributeValue::Static(s)) if s == "200"),
        "Height attribute should be 200"
    );
    assert!(
        matches!(canvas.attributes.get("program"), Some(AttributeValue::Binding(_))),
        "Program attribute should be a binding"
    );
}

/// T071: Contract test for Canvas with all attributes
#[test]
fn parse_canvas_with_all_attributes() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <canvas 
        width="800" 
        height="600" 
        program="{chart_program}"
        on_click="handle_canvas_click"
    />
</column>"#;

    let result = parse(xml);
    assert!(result.is_ok(), "Should parse canvas with all attributes");
    let doc = result.unwrap();

    let canvas = doc
        .root
        .children
        .iter()
        .find(|c| matches!(c.kind, WidgetKind::Canvas))
        .expect("Should have canvas child");

    assert!(
        matches!(canvas.attributes.get("width"), Some(AttributeValue::Static(s)) if s == "800"),
        "Width should be 800"
    );
    assert!(
        matches!(canvas.attributes.get("height"), Some(AttributeValue::Static(s)) if s == "600"),
        "Height should be 600"
    );
    assert!(
        matches!(canvas.attributes.get("program"), Some(AttributeValue::Binding(_))),
        "Program should be a binding"
    );

    // Check event handler
    assert_eq!(canvas.events.len(), 1, "Should have one event handler");
    let event = &canvas.events[0];
    assert!(
        matches!(event.event, EventKind::Click),
        "Event should be Click"
    );
    assert_eq!(event.handler, "handle_canvas_click");
}

/// T072: Contract test for Canvas size validation (min 50px, max 4000px)
#[test]
fn parse_canvas_size_validation_min() {
    // Test width too small
    let xml_width_small = r#"<?xml version="1.0"?>
<column>
    <canvas width="40" height="200" program="{chart}" />
</column>"#;

    let result = parse(xml_width_small);
    assert!(
        result.is_err(),
        "Should fail when width < 50px"
    );
    let err = result.unwrap_err();
    assert!(
        err.to_string().to_lowercase().contains("width") 
            || err.to_string().contains("50"),
        "Error should mention width or minimum size: {}",
        err
    );

    // Test height too small
    let xml_height_small = r#"<?xml version="1.0"?>
<column>
    <canvas width="200" height="40" program="{chart}" />
</column>"#;

    let result = parse(xml_height_small);
    assert!(
        result.is_err(),
        "Should fail when height < 50px"
    );
}

/// T072: Contract test for Canvas size validation (max)
#[test]
fn parse_canvas_size_validation_max() {
    // Test width too large
    let xml_width_large = r#"<?xml version="1.0"?>
<column>
    <canvas width="5000" height="200" program="{chart}" />
</column>"#;

    let result = parse(xml_width_large);
    assert!(
        result.is_err(),
        "Should fail when width > 4000px"
    );
    let err = result.unwrap_err();
    assert!(
        err.to_string().to_lowercase().contains("width")
            || err.to_string().contains("4000"),
        "Error should mention width or maximum size: {}",
        err
    );

    // Test height too large
    let xml_height_large = r#"<?xml version="1.0"?>
<column>
    <canvas width="200" height="5000" program="{chart}" />
</column>"#;

    let result = parse(xml_height_large);
    assert!(
        result.is_err(),
        "Should fail when height > 4000px"
    );
}

/// T072: Contract test for Canvas size validation (valid range)
#[test]
fn parse_canvas_size_validation_valid() {
    // Test minimum valid size
    let xml_min = r#"<?xml version="1.0"?>
<column>
    <canvas width="50" height="50" program="{chart}" />
</column>"#;

    let result = parse(xml_min);
    assert!(
        result.is_ok(),
        "Should succeed when width and height are exactly 50px"
    );

    // Test maximum valid size
    let xml_max = r#"<?xml version="1.0"?>
<column>
    <canvas width="4000" height="4000" program="{chart}" />
</column>"#;

    let result = parse(xml_max);
    assert!(
        result.is_ok(),
        "Should succeed when width and height are exactly 4000px"
    );

    // Test typical size
    let xml_typical = r#"<?xml version="1.0"?>
<column>
    <canvas width="800" height="600" program="{chart}" />
</column>"#;

    let result = parse(xml_typical);
    assert!(
        result.is_ok(),
        "Should succeed for typical canvas size"
    );
}

/// T072: Contract test for Canvas missing required attributes
#[test]
fn parse_canvas_missing_width_errors() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <canvas height="200" program="{chart}" />
</column>"#;

    let result = parse(xml);
    assert!(
        result.is_err(),
        "Should fail when width is missing"
    );
    let err = result.unwrap_err();
    assert!(
        err.to_string().to_lowercase().contains("width"),
        "Error should mention width: {}",
        err
    );
}

/// T072: Contract test for Canvas missing height
#[test]
fn parse_canvas_missing_height_errors() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <canvas width="400" program="{chart}" />
</column>"#;

    let result = parse(xml);
    assert!(
        result.is_err(),
        "Should fail when height is missing"
    );
    let err = result.unwrap_err();
    assert!(
        err.to_string().to_lowercase().contains("height"),
        "Error should mention height: {}",
        err
    );
}

/// T072: Contract test for Canvas missing program
#[test]
fn parse_canvas_missing_program_errors() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <canvas width="400" height="200" />
</column>"#;

    let result = parse(xml);
    assert!(
        result.is_err(),
        "Should fail when program is missing"
    );
    let err = result.unwrap_err();
    assert!(
        err.to_string().to_lowercase().contains("program"),
        "Error should mention program: {}",
        err
    );
}

/// T071: Contract test for Canvas children not allowed
#[test]
fn parse_canvas_with_children_errors() {
    let xml = r#"<?xml version="1.0"?>
<column>
    <canvas width="400" height="200" program="{chart}">
        <text value="Invalid child" />
    </canvas>
</column>"#;

    let result = parse(xml);
    assert!(
        result.is_err(),
        "Should fail when canvas has children"
    );
    let err = result.unwrap_err();
    assert!(
        err.to_string().to_lowercase().contains("canvas")
            || err.to_string().to_lowercase().contains("child"),
        "Error should mention canvas or children: {}",
        err
    );
}
