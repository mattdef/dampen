//! Codegen snapshot tests
//!
//! These tests verify that code generation produces consistent output.
//! They use insta for snapshot testing.

use dampen_core::{HandlerSignature, generate_application, parse};

#[test]
fn test_codegen_simple_button() {
    let xml = r#"<button label="Click" on_click="handle_click" />"#;
    let doc = parse(xml).unwrap();

    let handlers = vec![HandlerSignature {
        name: "handle_click".to_string(),
        param_type: None,
        returns_command: false,
    }];

    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();

    // Use insta to snapshot test
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_text_with_binding() {
    let xml = r#"<text value="{counter}" size="24" />"#;
    let doc = parse(xml).unwrap();

    let handlers = vec![];

    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();

    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_column_with_children() {
    let xml = r#"
        <column spacing="10">
            <text value="Header" />
            <button label="Submit" on_click="submit" />
        </column>
    "#;
    let doc = parse(xml).unwrap();

    let handlers = vec![HandlerSignature {
        name: "submit".to_string(),
        param_type: None,
        returns_command: false,
    }];

    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();

    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_conditional_enabled() {
    let xml = r#"<button label="Submit" on_click="submit" enabled="{count > 0}" />"#;
    let doc = parse(xml).unwrap();

    let handlers = vec![HandlerSignature {
        name: "submit".to_string(),
        param_type: None,
        returns_command: false,
    }];

    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();

    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_handler_with_value() {
    let xml = r#"<text_input value="{name}" on_input="update_name" />"#;
    let doc = parse(xml).unwrap();

    let handlers = vec![HandlerSignature {
        name: "update_name".to_string(),
        param_type: Some("String".to_string()),
        returns_command: false,
    }];

    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();

    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_formatted_binding() {
    let xml = r#"<text value="Count: {count}, Name: {name}" />"#;
    let doc = parse(xml).unwrap();

    let handlers = vec![];

    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();

    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_complex_layout() {
    let xml = r#"
        <column padding="20" spacing="15">
            <text value="Todo App" size="32" weight="bold" />
            <row spacing="10">
                <text_input 
                    value="{new_item}"
                    on_input="update_new_item"
                    placeholder="Add todo..."
                    width="fill"
                />
                <button label="Add" on_click="add_item" enabled="{new_item.len() > 0}" />
            </row>
            <rule />
            <text value="{items.len()} items" />
        </column>
    "#;
    let doc = parse(xml).unwrap();

    let handlers = vec![
        HandlerSignature {
            name: "update_new_item".to_string(),
            param_type: Some("String".to_string()),
            returns_command: false,
        },
        HandlerSignature {
            name: "add_item".to_string(),
            param_type: None,
            returns_command: false,
        },
    ];

    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();

    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_method_calls() {
    let xml = r#"
        <column>
            <text value="{items.len()}" />
            <text value="{name.to_uppercase()}" />
            <button label="Clear" on_click="clear" enabled="{items.len() > 0}" />
        </column>
    "#;
    let doc = parse(xml).unwrap();

    let handlers = vec![HandlerSignature {
        name: "clear".to_string(),
        param_type: None,
        returns_command: false,
    }];

    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();

    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_conditional_expressions() {
    let xml = r#"
        <column>
            <text value="{if is_loading then 'Loading...' else 'Ready'}" />
            <text value="{if error then 'Error' else 'Success'}" />
            <button 
                label="Submit" 
                on_click="submit" 
                enabled="{count > 0}" 
            />
        </column>
    "#;
    let doc = parse(xml).unwrap();

    let handlers = vec![HandlerSignature {
        name: "submit".to_string(),
        param_type: None,
        returns_command: false,
    }];

    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();

    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_zero_runtime_dependencies() {
    let xml = r#"
        <column>
            <text value="{name}" />
            <text value="Count: {count + 1}" />
            <button label="Click" on_click="handle_click" />
        </column>
    "#;
    let doc = parse(xml).unwrap();

    let handlers = vec![HandlerSignature {
        name: "handle_click".to_string(),
        param_type: None,
        returns_command: false,
    }];

    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    let code = output.code;

    assert!(
        !code.contains("to_binding_value"),
        "Generated code should not contain to_binding_value() calls"
    );
    assert!(
        !code.contains("BindingValue"),
        "Generated code should not contain BindingValue type"
    );
    assert!(
        code.contains("to_string"),
        "Generated code should use to_string() for string conversion"
    );
}

// ============================================================================
// Comprehensive Widget Type Coverage Tests
// ============================================================================

#[test]
fn test_codegen_container() {
    let xml =
        r#"<container width="300" height="200" padding="20"><text value="Inside" /></container>"#;
    let doc = parse(xml).unwrap();
    let handlers = vec![];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_scrollable() {
    let xml = r#"
        <scrollable height="400">
            <column>
                <text value="Line 1" />
                <text value="Line 2" />
                <text value="Line 3" />
            </column>
        </scrollable>
    "#;
    let doc = parse(xml).unwrap();
    let handlers = vec![];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_stack() {
    let xml = r#"
        <stack>
            <container><text value="Background" /></container>
            <container><text value="Overlay" /></container>
        </stack>
    "#;
    let doc = parse(xml).unwrap();
    let handlers = vec![];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_checkbox() {
    let xml = r#"<checkbox label="Accept Terms" checked="{accepted}" on_toggle="toggle_accept" />"#;
    let doc = parse(xml).unwrap();
    let handlers = vec![HandlerSignature {
        name: "toggle_accept".to_string(),
        param_type: Some("bool".to_string()),
        returns_command: false,
    }];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_slider() {
    let xml = r#"<slider min="0" max="100" value="{volume}" on_change="set_volume" />"#;
    let doc = parse(xml).unwrap();
    let handlers = vec![HandlerSignature {
        name: "set_volume".to_string(),
        param_type: Some("f32".to_string()),
        returns_command: false,
    }];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_picklist() {
    let xml = r#"<pick_list placeholder="Select option" options="Option1,Option2,Option3" selected="{selected_option}" on_select="handle_select" />"#;
    let doc = parse(xml).unwrap();
    let handlers = vec![HandlerSignature {
        name: "handle_select".to_string(),
        param_type: Some("String".to_string()),
        returns_command: false,
    }];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_toggler() {
    let xml = r#"<toggler label="Enable Feature" is_toggled="{feature_enabled}" on_toggle="toggle_feature" />"#;
    let doc = parse(xml).unwrap();
    let handlers = vec![HandlerSignature {
        name: "toggle_feature".to_string(),
        param_type: Some("bool".to_string()),
        returns_command: false,
    }];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_space() {
    let xml = r#"<column><text value="Before" /><space width="50" height="20" /><text value="After" /></column>"#;
    let doc = parse(xml).unwrap();
    let handlers = vec![];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_image() {
    let xml = r#"<image src="assets/logo.png" width="200" height="100" />"#;
    let doc = parse(xml).unwrap();
    let handlers = vec![];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_svg() {
    let xml = r#"<svg path="assets/icon.svg" width="24" height="24" />"#;
    let doc = parse(xml).unwrap();
    let handlers = vec![];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_radio() {
    let xml = r#"
        <column>
            <radio label="Option A" value="a" selected="{option}" on_select="select_option" />
            <radio label="Option B" value="b" selected="{option}" on_select="select_option" />
        </column>
    "#;
    let doc = parse(xml).unwrap();
    let handlers = vec![HandlerSignature {
        name: "select_option".to_string(),
        param_type: Some("String".to_string()),
        returns_command: false,
    }];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_progress_bar() {
    let xml = r#"<progress_bar value="{progress}" min="0" max="100" />"#;
    let doc = parse(xml).unwrap();
    let handlers = vec![];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_tooltip() {
    let xml = r#"
        <tooltip message="Click to perform action">
            <button label="Action" on_click="perform_action" />
        </tooltip>
    "#;
    let doc = parse(xml).unwrap();
    let handlers = vec![HandlerSignature {
        name: "perform_action".to_string(),
        param_type: None,
        returns_command: false,
    }];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

// ComboBox test removed - widget not implemented yet

#[test]
fn test_codegen_float() {
    let xml = r#"
        <float>
            <column>
                <button label="Floating Button" on_click="float_action" />
            </column>
        </float>
    "#;
    let doc = parse(xml).unwrap();
    let handlers = vec![HandlerSignature {
        name: "float_action".to_string(),
        param_type: None,
        returns_command: false,
    }];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_complex_nested() {
    let xml = r#"
        <scrollable height="600">
            <column padding="20" spacing="10">
                <text value="Dashboard" size="32" weight="bold" />
                <rule />
                <row spacing="20">
                    <container width="300" padding="15">
                        <column spacing="10">
                            <text value="Stats" size="20" weight="bold" />
                            <text value="Total: {total}" />
                            <text value="Active: {active}" />
                            <progress_bar value="{progress}" min="0" max="100" />
                        </column>
                    </container>
                    <container width="400" padding="15">
                        <column spacing="10">
                            <text value="Actions" size="20" weight="bold" />
                            <checkbox label="Enable feature" checked="{enabled}" on_toggle="toggle" />
                            <slider min="0" max="100" value="{value}" on_change="update_value" />
                            <row spacing="10">
                                <button label="Save" on_click="save" />
                                <button label="Cancel" on_click="cancel" />
                            </row>
                        </column>
                    </container>
                </row>
            </column>
        </scrollable>
    "#;
    let doc = parse(xml).unwrap();
    let handlers = vec![
        HandlerSignature {
            name: "toggle".to_string(),
            param_type: Some("bool".to_string()),
            returns_command: false,
        },
        HandlerSignature {
            name: "update_value".to_string(),
            param_type: Some("f32".to_string()),
            returns_command: false,
        },
        HandlerSignature {
            name: "save".to_string(),
            param_type: None,
            returns_command: false,
        },
        HandlerSignature {
            name: "cancel".to_string(),
            param_type: None,
            returns_command: false,
        },
    ];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}

#[test]
fn test_codegen_all_binding_types() {
    let xml = r#"
        <column>
            <text value="{field}" />
            <text value="{field.subfield}" />
            <text value="{field.method()}" />
            <text value="{a + b}" />
            <text value="{a - b}" />
            <text value="{a * b}" />
            <text value="{a / b}" />
            <text value="{a == b}" />
            <text value="{a != b}" />
            <text value="{a &lt; b}" />
            <text value="{a &gt; b}" />
            <text value="{if cond then 'yes' else 'no'}" />
        </column>
    "#;
    let doc = parse(xml).unwrap();
    let handlers = vec![];
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    insta::assert_snapshot!(output.code);
}
