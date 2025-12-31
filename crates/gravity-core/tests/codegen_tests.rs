//! Code generation snapshot tests

use gravity_core::{
    parse, generate_application, validate_handlers,
    HandlerSignature, WidgetKind, EventKind,
};

#[test]
fn test_simple_button_codegen() {
    let xml = r#"<column>
        <text value="Counter: {count}" />
        <button label="Increment" on_click="increment" />
    </column>"#;
    
    let doc = parse(xml).unwrap();
    
    let handlers = vec![HandlerSignature {
        name: "increment".to_string(),
        param_type: None,
        returns_command: false,
    }];
    
    // Validate handlers
    assert!(validate_handlers(&doc, &handlers).is_ok());
    
    // Generate code
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    
    // Verify generated code contains expected elements (with flexible whitespace)
    let code = &output.code;
    assert!(code.contains("enum") && code.contains("Message"));
    assert!(code.contains("increment"));
    assert!(code.contains("fn") && code.contains("view"));
    assert!(code.contains("fn") && code.contains("update"));
    assert!(code.contains("widget") && code.contains("button"));
    assert!(code.contains("widget") && code.contains("text"));
}

#[test]
fn test_binding_expression_codegen() {
    let xml = r#"<column>
        <text value="{name}" />
        <text value="Count: {count}" />
        <text value="{if count > 0 then 'Active' else 'Inactive'}" />
    </column>"#;
    
    let doc = parse(xml).unwrap();
    let handlers: Vec<HandlerSignature> = vec![];
    
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    
    // Should generate binding expressions
    let code = &output.code;
    assert!(code.contains("fn") && code.contains("view"));
    assert!(code.contains("name"));
    assert!(code.contains("count"));
}

#[test]
fn test_handler_with_value_codegen() {
    let xml = r#"<column>
        <text_input value="{input}" on_input="update_input" />
    </column>"#;
    
    let doc = parse(xml).unwrap();
    
    let handlers = vec![HandlerSignature {
        name: "update_input".to_string(),
        param_type: Some("String".to_string()),
        returns_command: false,
    }];
    
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    
    let code = &output.code;
    assert!(code.contains("update_input"));
    assert!(code.contains("String"));
}

#[test]
fn test_multiple_handlers_codegen() {
    let xml = r#"<column>
        <button label="Add" on_click="add" />
        <button label="Remove" on_click="remove" />
        <button label="Reset" on_click="reset" />
    </column>"#;
    
    let doc = parse(xml).unwrap();
    
    let handlers = vec![
        HandlerSignature {
            name: "add".to_string(),
            param_type: None,
            returns_command: false,
        },
        HandlerSignature {
            name: "remove".to_string(),
            param_type: None,
            returns_command: false,
        },
        HandlerSignature {
            name: "reset".to_string(),
            param_type: None,
            returns_command: false,
        },
    ];
    
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    
    let code = &output.code;
    assert!(code.contains("add"));
    assert!(code.contains("remove"));
    assert!(code.contains("reset"));
}

#[test]
fn test_nested_layout_codegen() {
    let xml = r#"<column spacing="10" padding="20">
        <text value="Header" />
        <row spacing="5">
            <button label="Left" on_click="left" />
            <button label="Right" on_click="right" />
        </row>
    </column>"#;
    
    let doc = parse(xml).unwrap();
    
    let handlers = vec![
        HandlerSignature {
            name: "left".to_string(),
            param_type: None,
            returns_command: false,
        },
        HandlerSignature {
            name: "right".to_string(),
            param_type: None,
            returns_command: false,
        },
    ];
    
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    
    let code = &output.code;
    assert!(code.contains("column"));
    assert!(code.contains("row"));
    assert!(code.contains("spacing"));
    assert!(code.contains("padding"));
}

#[test]
fn test_missing_handler_validation() {
    let xml = r#"<column>
        <button label="Click" on_click="nonexistent" />
    </column>"#;
    
    let doc = parse(xml).unwrap();
    let handlers: Vec<HandlerSignature> = vec![];
    
    let result = validate_handlers(&doc, &handlers);
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(e.to_string().contains("nonexistent"));
    }
}

#[test]
fn test_constant_folding() {
    use gravity_core::codegen::constant_folding;
    
    let input = "let x = 1 + 1; let y = 2 * 3;";
    let output = constant_folding(input);
    
    // For now, just verify it doesn't break
    assert_eq!(output, input);
}

#[test]
fn test_update_with_value() {
    let xml = r#"<column><text_input on_input="update_input" /></column>"#;
    let doc = parse(xml).unwrap();
    
    let handlers = vec![HandlerSignature {
        name: "update_input".to_string(),
        param_type: Some("String".to_string()),
        returns_command: false,
    }];
    
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    
    let code = &output.code;
    assert!(code.contains("update_input"));
    assert!(code.contains("String"));
}

#[test]
fn test_application_trait_generation() {
    let xml = r#"<column><text value="Test" /></column>"#;
    let doc = parse(xml).unwrap();
    let handlers: Vec<HandlerSignature> = vec![];
    
    let output = generate_application(&doc, "TestModel", "TestMessage", &handlers).unwrap();
    
    let code = &output.code;
    assert!(code.contains("impl") && code.contains("Application") && code.contains("TestModel"));
    assert!(code.contains("Message") && code.contains("TestMessage"));
}

#[test]
fn test_update_function_generation() {
    let xml = r#"<column><button label="Click" on_click="handle_click" /></column>"#;
    let doc = parse(xml).unwrap();
    
    let handlers = vec![HandlerSignature {
        name: "handle_click".to_string(),
        param_type: None,
        returns_command: false,
    }];
    
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    
    let code = &output.code;
    assert!(code.contains("fn") && code.contains("update"));
    assert!(code.contains("handle_click"));
}

#[test]
fn test_view_function_generation() {
    let xml = r#"<column>
        <text value="Hello" />
        <button label="OK" />
    </column>"#;
    
    let doc = parse(xml).unwrap();
    let handlers: Vec<HandlerSignature> = vec![];
    
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    
    let code = &output.code;
    assert!(code.contains("fn") && code.contains("view"));
    assert!(code.contains("text"));
    assert!(code.contains("button"));
}

#[test]
fn test_binding_with_method_call() {
    let xml = r#"<column>
        <text value="{items.len()}" />
    </column>"#;
    
    let doc = parse(xml).unwrap();
    let handlers: Vec<HandlerSignature> = vec![];
    
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    
    assert!(output.code.contains("items"));
    assert!(output.code.contains("len"));
}

#[test]
fn test_conditional_binding() {
    let xml = r#"<column>
        <text value="{if count > 0 then 'Has items' else 'Empty'}" />
    </column>"#;
    
    let doc = parse(xml).unwrap();
    let handlers: Vec<HandlerSignature> = vec![];
    
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    
    assert!(output.code.contains("if"));
    assert!(output.code.contains("count"));
    assert!(output.code.contains("Has items"));
    assert!(output.code.contains("Empty"));
}

#[test]
fn test_interpolated_string() {
    let xml = r#"<column>
        <text value="Hello, {name}! You have {count} items." />
    </column>"#;
    
    let doc = parse(xml).unwrap();
    let handlers: Vec<HandlerSignature> = vec![];
    
    let output = generate_application(&doc, "Model", "Message", &handlers).unwrap();
    
    assert!(output.code.contains("name"));
    assert!(output.code.contains("count"));
}

#[test]
fn test_empty_document() {
    let xml = r#"<column />"#;
    let doc = parse(xml).unwrap();
    let handlers: Vec<HandlerSignature> = vec![];
    
    let result = generate_application(&doc, "Model", "Message", &handlers);
    assert!(result.is_ok());
}

#[test]
fn test_complex_example() {
    // This test verifies a realistic example
    let xml = r#"<column spacing="10" padding="20">
        <text value="Todo List" size="24" />
        <text value="Total: {items.len()} items" />
        <row spacing="10">
            <button label="Add" on_click="add_item" />
            <button label="Clear" on_click="clear_all" />
        </row>
        <text value="{if items.len() == 0 then 'No items' else ''}" />
    </column>"#;
    
    let doc = parse(xml).unwrap();
    
    let handlers = vec![
        HandlerSignature {
            name: "add_item".to_string(),
            param_type: None,
            returns_command: false,
        },
        HandlerSignature {
            name: "clear_all".to_string(),
            param_type: None,
            returns_command: false,
        },
    ];
    
    let output = generate_application(&doc, "TodoModel", "TodoMessage", &handlers).unwrap();
    
    // Verify all components are present
    let code = &output.code;
    assert!(code.contains("TodoMessage"));
    assert!(code.contains("add_item"));
    assert!(code.contains("clear_all"));
    assert!(code.contains("fn") && code.contains("view"));
    assert!(code.contains("fn") && code.contains("update"));
    assert!(code.contains("items"));
    assert!(code.contains("len"));
    
    // Verify no warnings
    assert!(output.warnings.is_empty());
}
