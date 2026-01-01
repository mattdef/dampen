//! Integration tests for widget rendering with IR nodes

use gravity_core::{parse, AttributeValue, EventBinding, EventKind, WidgetKind, WidgetNode};
use gravity_iced::{render, IcedBackend};

/// Helper to create a test backend
fn create_backend() -> IcedBackend {
    IcedBackend::new(|handler_name, _value| Box::new(handler_name.to_string()))
}

#[test]
fn test_render_text_static() {
    let backend = create_backend();
    let xml = r#"<text value="Hello World" />"#;
    let doc = parse(xml).unwrap();

    // Should render without panicking
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_text_with_binding() {
    let backend = create_backend();
    let xml = r#"<text value="{message}" />"#;
    let doc = parse(xml).unwrap();

    // Should render with binding placeholder
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_text_interpolated() {
    let backend = create_backend();
    let xml = r#"<text value="Count: {count}" />"#;
    let doc = parse(xml).unwrap();

    // Should render with interpolation placeholder
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_button_static() {
    let backend = create_backend();
    let xml = r#"<button label="Click Me" on_click="handle_click" />"#;
    let doc = parse(xml).unwrap();

    // Should render button with handler
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_button_with_binding() {
    let backend = create_backend();
    let xml = r#"<button label="{button_text}" on_click="handle_click" />"#;
    let doc = parse(xml).unwrap();

    // Should render button with binding
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_column_layout() {
    let backend = create_backend();
    let xml = r#"
        <column spacing="10">
            <text value="Item 1" />
            <text value="Item 2" />
            <text value="Item 3" />
        </column>
    "#;
    let doc = parse(xml).unwrap();

    // Should render column with children
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_row_layout() {
    let backend = create_backend();
    let xml = r#"
        <row spacing="5">
            <text value="Left" />
            <text value="Right" />
        </row>
    "#;
    let doc = parse(xml).unwrap();

    // Should render row with children
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_container() {
    let backend = create_backend();
    let xml = r#"
        <container padding="20">
            <text value="Content" />
        </container>
    "#;
    let doc = parse(xml).unwrap();

    // Should render container with content
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_scrollable() {
    let backend = create_backend();
    let xml = r#"
        <scrollable>
            <column>
                <text value="Item 1" />
                <text value="Item 2" />
            </column>
        </scrollable>
    "#;
    let doc = parse(xml).unwrap();

    // Should render scrollable with content
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_stack() {
    let backend = create_backend();
    let xml = r#"
        <stack>
            <text value="Layer 1" />
            <text value="Layer 2" />
        </stack>
    "#;
    let doc = parse(xml).unwrap();

    // Should render stack with children
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_text_input() {
    let backend = create_backend();
    let xml = r#"<text_input value="{input_value}" on_input="update_input" placeholder="Enter text..." />"#;
    let doc = parse(xml).unwrap();

    // Should render text input
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_checkbox() {
    let backend = create_backend();
    let xml =
        r#"<checkbox label="Enable feature" checked="{is_enabled}" on_toggle="toggle_feature" />"#;
    let doc = parse(xml).unwrap();

    // Should render checkbox
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_slider() {
    let backend = create_backend();
    let xml = r#"<slider min="0" max="100" value="{volume}" on_change="update_volume" />"#;
    let doc = parse(xml).unwrap();

    // Should render slider
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_pick_list() {
    let backend = create_backend();
    let xml =
        r#"<pick_list options="Red,Green,Blue" selected="{color}" on_select="select_color" />"#;
    let doc = parse(xml).unwrap();

    // Should render pick list
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_toggler() {
    let backend = create_backend();
    let xml = r#"<toggler label="Dark Mode" active="{dark_mode}" on_toggle="toggle_dark" />"#;
    let doc = parse(xml).unwrap();

    // Should render toggler
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_image() {
    let backend = create_backend();
    let xml = r#"<image src="logo.png" />"#;
    let doc = parse(xml).unwrap();

    // Should render image
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_svg() {
    let backend = create_backend();
    let xml = r#"<svg src="icon.svg" />"#;
    let doc = parse(xml).unwrap();

    // Should render SVG
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_space() {
    let backend = create_backend();
    let xml = r#"<space />"#;
    let doc = parse(xml).unwrap();

    // Should render space
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_render_rule() {
    let backend = create_backend();
    let xml = r#"<rule />"#;
    let doc = parse(xml).unwrap();

    // Should render rule
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_complex_nested_layout() {
    let backend = create_backend();
    let xml = r#"
        <container padding="20">
            <column spacing="15">
                <row spacing="10" align="center">
                    <text value="Title" size="24" weight="bold" />
                    <space />
                    <button label="Action" on_click="handle_action" />
                </row>
                <rule />
                <scrollable>
                    <column spacing="8">
                        <text value="Item 1" />
                        <text value="Item 2" />
                        <text value="Item 3" />
                    </column>
                </scrollable>
                <row spacing="10">
                    <text_input value="{search}" on_input="update_search" placeholder="Search..." />
                    <button label="Search" on_click="perform_search" />
                </row>
            </column>
        </container>
    "#;
    let doc = parse(xml).unwrap();

    // Should render complex nested structure
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_widget_with_id_attribute() {
    let backend = create_backend();
    let xml = r#"<text id="header" value="Hello" />"#;
    let doc = parse(xml).unwrap();

    // Should parse and render with ID
    assert_eq!(doc.root.id, Some("header".to_string()));
    let _element = render(&doc.root, &backend);
}

#[test]
fn test_multiple_event_handlers() {
    let backend = create_backend();
    let xml = r#"<button label="Multi" on_click="click_handler" on_press="press_handler" />"#;
    let doc = parse(xml).unwrap();

    // Should parse multiple events
    assert_eq!(doc.root.events.len(), 2);
    assert!(doc.root.events.iter().any(|e| e.event == EventKind::Click));
    assert!(doc.root.events.iter().any(|e| e.event == EventKind::Press));

    let _element = render(&doc.root, &backend);
}

#[test]
fn test_empty_container() {
    let backend = create_backend();
    let xml = r#"<container></container>"#;
    let doc = parse(xml).unwrap();

    // Should handle empty container
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_deeply_nested_layouts() {
    let backend = create_backend();
    let xml = r#"
        <container>
            <column>
                <row>
                    <container>
                        <column>
                            <text value="Deep" />
                        </column>
                    </container>
                </row>
            </column>
        </container>
    "#;
    let doc = parse(xml).unwrap();

    // Should handle deep nesting
    let _element = render(&doc.root, &backend);
    assert!(true);
}

#[test]
fn test_attribute_parsing_all_types() {
    let backend = create_backend();
    let xml = r#"
        <column 
            spacing="10" 
            padding="20"
            width="300"
            height="200"
            align="center"
        >
            <text value="Test" />
        </column>
    "#;
    let doc = parse(xml).unwrap();

    // All attributes should be parsed
    assert!(doc.root.attributes.contains_key("spacing"));
    assert!(doc.root.attributes.contains_key("padding"));
    assert!(doc.root.attributes.contains_key("width"));
    assert!(doc.root.attributes.contains_key("height"));
    assert!(doc.root.attributes.contains_key("align"));

    let _element = render(&doc.root, &backend);
}

#[test]
fn test_binding_expression_in_attributes() {
    let backend = create_backend();
    let xml = r#"
        <column spacing="{spacing_value}">
            <text value="Test" />
        </column>
    "#;
    let doc = parse(xml).unwrap();

    // Should parse binding in attribute
    let spacing_attr = doc.root.attributes.get("spacing");
    assert!(matches!(spacing_attr, Some(AttributeValue::Binding(_))));

    let _element = render(&doc.root, &backend);
}

#[test]
fn test_interpolated_string_attributes() {
    let backend = create_backend();
    let xml = r#"<text value="Hello {name}, you have {count} items" />"#;
    let doc = parse(xml).unwrap();

    // Should parse interpolated string
    let value_attr = doc.root.attributes.get("value");
    assert!(matches!(value_attr, Some(AttributeValue::Interpolated(_))));

    let _element = render(&doc.root, &backend);
}

#[test]
fn test_all_widget_kinds_in_document() {
    let backend = create_backend();
    let xml = r#"
        <container>
            <column>
                <text value="Text" />
                <button label="Button" on_click="click" />
                <text_input value="input" on_input="input" />
                <checkbox label="Check" on_toggle="toggle" />
                <slider min="0" max="100" value="50" on_change="change" />
                <pick_list options="A,B,C" on_select="select" />
                <toggler label="Toggle" on_toggle="toggle2" />
                <image src="img.png" />
                <svg src="icon.svg" />
                <space />
                <rule />
                <scrollable><text value="Scroll" /></scrollable>
                <stack><text value="Stack" /></stack>
                <row><text value="Row" /></row>
            </column>
        </container>
    "#;
    let doc = parse(xml).unwrap();

    // Should parse all widget types
    let _element = render(&doc.root, &backend);
    assert!(true);
}
