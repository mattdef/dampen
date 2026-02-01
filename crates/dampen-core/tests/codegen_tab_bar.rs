//! Codegen tests for TabBar widget

use dampen_core::{HandlerSignature, generate_application, parse};

#[test]
fn test_codegen_tab_bar_basic() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="0" on_select="on_tab_selected">
                <tab label="General" />
                <tab label="Appearance" />
                <tab label="Notifications" />
            </tab_bar>
        </dampen>
    "#;

    let doc = parse(xml).unwrap();
    let handlers = vec![HandlerSignature {
        name: "on_tab_selected".to_string(),
        param_type: Some("usize".to_string()),
        returns_command: false,
    }];

    let result = generate_application(&doc, "Model", "Message", &handlers);

    assert!(result.is_ok(), "Codegen should succeed for basic TabBar");

    let output = result.unwrap();
    let code = output.code.to_string();

    // Verify the generated code contains expected elements
    assert!(
        code.contains("TabBar"),
        "Generated code should contain TabBar"
    );
    assert!(
        code.contains("on_tab_selected"),
        "Generated code should reference the handler"
    );
}

#[test]
fn test_codegen_tab_bar_with_binding() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="{selected_tab}" on_select="on_tab_selected">
                <tab label="Tab 1" />
                <tab label="Tab 2" />
            </tab_bar>
        </dampen>
    "#;

    let doc = parse(xml).unwrap();
    let handlers = vec![HandlerSignature {
        name: "on_tab_selected".to_string(),
        param_type: Some("usize".to_string()),
        returns_command: false,
    }];

    let result = generate_application(&doc, "Model", "Message", &handlers);

    assert!(result.is_ok(), "Codegen should succeed for empty TabBar");
}

#[test]
fn test_codegen_tab_bar_with_icons() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="0" on_select="on_tab_selected">
                <tab label="Home" icon="home" />
                <tab label="Settings" icon="settings" />
                <tab icon="user" />
            </tab_bar>
        </dampen>
    "#;

    let doc = parse(xml).unwrap();
    let handlers = vec![HandlerSignature {
        name: "on_tab_selected".to_string(),
        param_type: Some("usize".to_string()),
        returns_command: false,
    }];

    let result = generate_application(&doc, "Model", "Message", &handlers);

    assert!(
        result.is_ok(),
        "Codegen should succeed for TabBar with icons"
    );

    let output = result.unwrap();
    let code = output.code.to_string();

    // Verify the generated code contains expected icon-related elements
    assert!(
        code.contains("TabLabel"),
        "Generated code should contain TabLabel"
    );
    assert!(
        code.contains("IconText"),
        "Generated code should contain IconText for icon+label tabs"
    );
    assert!(
        code.contains("Icon ("),
        "Generated code should contain Icon for icon-only tabs"
    );
}

#[test]
fn test_codegen_tab_bar_empty() {
    let xml = r#"
        <dampen version="1.1">
            <tab_bar selected="0" on_select="on_tab_selected">
            </tab_bar>
        </dampen>
    "#;

    let doc = parse(xml).unwrap();
    let handlers = vec![HandlerSignature {
        name: "on_tab_selected".to_string(),
        param_type: Some("usize".to_string()),
        returns_command: false,
    }];

    let result = generate_application(&doc, "Model", "Message", &handlers);

    assert!(result.is_ok(), "Codegen should succeed for empty TabBar");
}
