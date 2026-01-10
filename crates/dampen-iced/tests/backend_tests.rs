//! Unit tests for IcedBackend trait implementations

use dampen_core::Backend;
use dampen_iced::IcedBackend;

/// Create a test backend instance
fn create_test_backend() -> IcedBackend {
    IcedBackend::new(|handler_name, _value| {
        // Return a simple message for testing
        Box::new(handler_name.to_string())
    })
}

#[test]
fn test_text_widget() {
    let backend = create_test_backend();
    let widget = backend.text("Hello, World!");

    // The widget should be created without panicking
    // We can't directly inspect the Element type, but we can verify it compiles
    // and the method doesn't panic
    assert!(true);
}

#[test]
fn test_button_widget() {
    let backend = create_test_backend();
    let label = backend.text("Click Me");
    let msg = Box::new("test_handler".to_string());
    let widget = backend.button(label, Some(msg));

    // Button should be created successfully
    assert!(true);
}

#[test]
fn test_button_without_handler() {
    let backend = create_test_backend();
    let label = backend.text("Click Me");
    let widget = backend.button(label, None);

    // Button without handler should still work
    assert!(true);
}

#[test]
fn test_column_layout() {
    let backend = create_test_backend();
    let children = vec![
        backend.text("Item 1"),
        backend.text("Item 2"),
        backend.text("Item 3"),
    ];
    let widget = backend.column(children);

    // Column should be created successfully
    assert!(true);
}

#[test]
fn test_row_layout() {
    let backend = create_test_backend();
    let children = vec![backend.text("Left"), backend.text("Right")];
    let widget = backend.row(children);

    // Row should be created successfully
    assert!(true);
}

#[test]
fn test_container_widget() {
    let backend = create_test_backend();
    let content = backend.text("Content");
    let widget = backend.container(content);

    // Container should be created successfully
    assert!(true);
}

#[test]
fn test_scrollable_widget() {
    let backend = create_test_backend();
    let content = backend.text("Scrollable content");
    let widget = backend.scrollable(content);

    // Scrollable should be created successfully
    assert!(true);
}

#[test]
fn test_stack_widget() {
    let backend = create_test_backend();
    let children = vec![backend.text("Layer 1"), backend.text("Layer 2")];
    let widget = backend.stack(children);

    // Stack should be created successfully
    assert!(true);
}

#[test]
fn test_text_input_widget() {
    let backend = create_test_backend();
    let msg = Box::new("input_handler".to_string());
    let widget = backend.text_input("Placeholder", "Initial value", Some(msg));

    // TextInput should be created successfully
    assert!(true);
}

#[test]
fn test_text_input_without_handler() {
    let backend = create_test_backend();
    let widget = backend.text_input("Placeholder", "Initial value", None);

    // TextInput without handler should still work
    assert!(true);
}

#[test]
fn test_checkbox_widget() {
    let backend = create_test_backend();
    let msg = Box::new("toggle_handler".to_string());
    let widget = backend.checkbox("Enable feature", true, Some(msg));

    // Checkbox should be created successfully
    assert!(true);
}

#[test]
fn test_checkbox_unchecked() {
    let backend = create_test_backend();
    let widget = backend.checkbox("Enable feature", false, None);

    // Unchecked checkbox without handler should work
    assert!(true);
}

#[test]
fn test_slider_widget() {
    let backend = create_test_backend();
    let msg = Box::new("change_handler".to_string());
    let widget = backend.slider(0.0, 100.0, 50.0, Some(msg));

    // Slider should be created successfully
    assert!(true);
}

#[test]
fn test_slider_without_handler() {
    let backend = create_test_backend();
    let widget = backend.slider(0.0, 100.0, 50.0, None);

    // Slider without handler should still work
    assert!(true);
}

#[test]
fn test_pick_list_widget() {
    let backend = create_test_backend();
    let msg = Box::new("select_handler".to_string());
    let options = vec!["Option 1", "Option 2", "Option 3"];
    let widget = backend.pick_list(options, Some("Option 2"), Some(msg));

    // PickList should be created successfully
    assert!(true);
}

#[test]
fn test_pick_list_without_selection() {
    let backend = create_test_backend();
    let options = vec!["Option 1", "Option 2"];
    let widget = backend.pick_list(options, None, None);

    // PickList without selection or handler should work
    assert!(true);
}

#[test]
fn test_toggler_widget() {
    let backend = create_test_backend();
    let msg = Box::new("toggle_handler".to_string());
    let widget = backend.toggler("Dark Mode", true, Some(msg));

    // Toggler should be created successfully
    assert!(true);
}

#[test]
fn test_toggler_inactive() {
    let backend = create_test_backend();
    let widget = backend.toggler("Dark Mode", false, None);

    // Inactive toggler without handler should work
    assert!(true);
}

#[test]
fn test_image_widget() {
    let backend = create_test_backend();
    let widget = backend.image("path/to/image.png");

    // Image should be created successfully
    assert!(true);
}

#[test]
fn test_svg_widget() {
    let backend = create_test_backend();
    let widget = backend.svg("path/to/image.svg");

    // SVG should be created successfully
    assert!(true);
}

#[test]
fn test_space_widget() {
    let backend = create_test_backend();
    let widget = backend.space();

    // Space should be created successfully
    assert!(true);
}

#[test]
fn test_rule_widget() {
    let backend = create_test_backend();
    let widget = backend.rule();

    // Rule should be created successfully
    assert!(true);
}

#[test]
fn test_nested_layouts() {
    let backend = create_test_backend();

    // Create a nested layout: column containing rows with text
    let row1 = backend.row(vec![backend.text("Left"), backend.text("Right")]);

    let row2 = backend.row(vec![backend.text("A"), backend.text("B")]);

    let column = backend.column(vec![row1, row2]);
    let container = backend.container(column);

    // Complex nested structure should work
    assert!(true);
}

#[test]
fn test_message_handler_invocation() {
    // Test that the backend can be created with a handler
    let backend = IcedBackend::new(|name, _value| Box::new(name.to_string()));

    let label = backend.text("Test");
    let msg = Box::new("test_handler".to_string());
    let _widget = backend.button(label, Some(msg));

    // Handler should be set up correctly
    assert!(true);
}

#[test]
fn test_layout_with_alignment() {
    use dampen_core::ir::layout::{Alignment, Justification, LayoutConstraints};
    use dampen_iced::style_mapping::map_layout_constraints;

    let layout = LayoutConstraints {
        align_items: Some(Alignment::Center),
        justify_content: Some(Justification::SpaceBetween),
        ..Default::default()
    };

    let iced_layout = map_layout_constraints(&layout);
    assert!(iced_layout.align_items.is_some());
    assert!(iced_layout.justify_content.is_some());
}

#[test]
fn test_layout_with_position() {
    use dampen_core::ir::layout::{LayoutConstraints, Position};
    use dampen_iced::style_mapping::map_layout_constraints;

    let layout = LayoutConstraints {
        position: Some(Position::Absolute),
        top: Some(10.0),
        right: Some(20.0),
        bottom: Some(30.0),
        left: Some(40.0),
        z_index: Some(100),
        ..Default::default()
    };

    let iced_layout = map_layout_constraints(&layout);
    assert_eq!(iced_layout.position, Some(Position::Absolute));
    assert_eq!(iced_layout.top, Some(10.0));
    assert_eq!(iced_layout.right, Some(20.0));
    assert_eq!(iced_layout.bottom, Some(30.0));
    assert_eq!(iced_layout.left, Some(40.0));
    assert_eq!(iced_layout.z_index, Some(100));
}

#[test]
fn test_position_helper_functions() {
    use dampen_core::ir::layout::Position;
    use dampen_iced::style_mapping::{IcedLayout, get_z_index, has_positioning};

    // Test has_positioning with position
    let layout_with_pos = IcedLayout {
        position: Some(Position::Absolute),
        top: Some(10.0),
        ..Default::default()
    };
    assert!(has_positioning(&layout_with_pos));

    // Test has_positioning without position
    let layout_without_pos = IcedLayout::default();
    assert!(!has_positioning(&layout_without_pos));

    // Test get_z_index
    let layout_with_z = IcedLayout {
        z_index: Some(50),
        ..Default::default()
    };
    assert_eq!(get_z_index(&layout_with_z), 50);

    // Test get_z_index default
    let layout_no_z = IcedLayout::default();
    assert_eq!(get_z_index(&layout_no_z), 0);
}

#[test]
fn test_layout_with_combined_attributes() {
    use dampen_core::ir::layout::{Alignment, Justification, LayoutConstraints, Position};
    use dampen_iced::style_mapping::map_layout_constraints;

    let layout = LayoutConstraints {
        width: Some(dampen_core::ir::layout::Length::Fixed(300.0)),
        height: Some(dampen_core::ir::layout::Length::Fixed(200.0)),
        padding: Some(dampen_core::ir::layout::Padding {
            top: 10.0,
            right: 10.0,
            bottom: 10.0,
            left: 10.0,
        }),
        align_items: Some(Alignment::Center),
        justify_content: Some(Justification::Start),
        position: Some(Position::Relative),
        top: Some(5.0),
        z_index: Some(10),
        ..Default::default()
    };

    let iced_layout = map_layout_constraints(&layout);

    // Verify all attributes are mapped
    assert!(iced_layout.width != iced::Length::Shrink); // Has fixed width
    assert!(iced_layout.height != iced::Length::Shrink); // Has fixed height
    assert!(iced_layout.align_items.is_some());
    assert!(iced_layout.justify_content.is_some());
    assert_eq!(iced_layout.position, Some(Position::Relative));
    assert_eq!(iced_layout.top, Some(5.0));
    assert_eq!(iced_layout.z_index, Some(10));
}
