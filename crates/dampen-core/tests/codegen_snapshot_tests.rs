//! Codegen snapshot tests
//!
//! These tests verify that code generation produces consistent output.
//! They use insta for snapshot testing.

use dampen_core::{generate_application, parse, HandlerSignature};

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
