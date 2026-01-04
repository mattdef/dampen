//! Theme and styling tests

use gravity_core::ir::style::Color;
use gravity_core::ir::{Background, WidgetState};
use gravity_core::{parse, HandlerRegistry};
use gravity_runtime::{HotReloadInterpreter, StyleCascade, ThemeManager};

#[allow(dead_code)]
fn c(s: &str) -> Color {
    Color::parse(s).unwrap()
}

#[allow(dead_code)]
fn complete_palette(primary: &str, secondary: &str, background: &str, text: &str) -> String {
    format!(
        "primary=\"{}\" secondary=\"{}\" success=\"#27ae60\" warning=\"#f39c12\" danger=\"#e74c3c\" background=\"{}\" surface=\"#ffffff\" text=\"{}\" text_secondary=\"#7f8c8d\"",
        primary, secondary, background, text
    )
}

#[test]
fn test_theme_parsing() {
    let xml = "<gravity><themes><theme name=\"custom\"><palette primary=\"#3498db\" secondary=\"#2ecc71\" success=\"#27ae60\" warning=\"#f39c12\" danger=\"#e74c3c\" background=\"#ecf0f1\" surface=\"#ffffff\" text=\"#2c3e50\" text_secondary=\"#7f8c8d\" /><typography font_family=\"sans-serif\" font_size_base=\"16\" /><spacing unit=\"8\" /></theme></themes><global_theme name=\"custom\" /><column><text value=\"Test\" /></column></gravity>";

    let doc = parse(xml).unwrap();
    assert!(doc.themes.contains_key("custom"));
    let theme = doc.themes.get("custom").unwrap();
    assert_eq!(theme.name, "custom");
    assert_eq!(theme.palette.primary, c("#3498db"));
    assert_eq!(theme.palette.background, c("#ecf0f1"));
    assert_eq!(theme.spacing.unit, 8.0);
    assert_eq!(doc.global_theme, Some("custom".to_string()));
}

#[test]
fn test_builtin_themes() {
    let manager = ThemeManager::new();
    assert!(manager.get_theme("light").is_some());
    assert!(manager.get_theme("dark").is_some());

    let light = manager.get_theme("light").unwrap();
    assert_eq!(light.name, "light");
    assert_eq!(light.palette.background, c("#ecf0f1"));
}

#[test]
fn test_theme_switching() {
    let xml = format!("<gravity><themes><theme name=\"light\"><palette {} /><typography font_family=\"sans-serif\" font_size_base=\"16\" /><spacing unit=\"4\" /></theme><theme name=\"dark\"><palette primary=\"#5dade2\" secondary=\"#58d68d\" success=\"#27ae60\" warning=\"#f39c12\" danger=\"#e74c3c\" background=\"#2c3e50\" surface=\"#ffffff\" text=\"#ecf0f1\" text_secondary=\"#7f8c8d\" /><typography font_family=\"sans-serif\" font_size_base=\"16\" /><spacing unit=\"4\" /></theme></themes><global_theme name=\"light\" /><column><text value=\"Test\" /></column></gravity>", complete_palette("#3498db", "#2ecc71", "#ecf0f1", "#2c3e50"));

    let doc = parse(&xml).unwrap();
    let mut manager = ThemeManager::new();
    manager.load_from_document(&doc);

    assert_eq!(manager.get_current_theme().unwrap().name, "light");
    manager.set_theme("dark".to_string()).unwrap();
    assert_eq!(manager.get_current_theme().unwrap().name, "dark");
    assert_eq!(
        manager.get_current_theme().unwrap().palette.background,
        c("#2c3e50")
    );
}

#[test]
fn test_style_classes() {
    let xml = "<gravity><style_classes><style name=\"btn\" background=\"#3498db\" color=\"#ffffff\" /></style_classes><column><button class=\"btn\" label=\"Test\" /></column></gravity>";

    let doc = parse(xml).unwrap();
    assert!(doc.style_classes.contains_key("btn"));
    let class = doc.style_classes.get("btn").unwrap();
    assert_eq!(class.name, "btn");
    assert!(class.style.background.is_some());

    // Check if class attribute is extracted to classes field
    let button = &doc.root.children[0];
    assert_eq!(button.classes, vec!["btn".to_string()]);
}

#[test]
fn test_theme_validation() {
    use gravity_core::parser::theme_parser::parse_theme;
    use std::collections::HashMap;

    let mut palette = HashMap::new();
    palette.insert("primary".to_string(), "#3498db".to_string());
    palette.insert("secondary".to_string(), "#2ecc71".to_string());
    palette.insert("success".to_string(), "#27ae60".to_string());
    palette.insert("warning".to_string(), "#f39c12".to_string());
    palette.insert("danger".to_string(), "#e74c3c".to_string());
    palette.insert("background".to_string(), "#ecf0f1".to_string());
    palette.insert("surface".to_string(), "#ffffff".to_string());
    palette.insert("text".to_string(), "#2c3e50".to_string());
    palette.insert("text_secondary".to_string(), "#7f8c8d".to_string());

    let mut typography = HashMap::new();
    typography.insert("font_family".to_string(), "sans-serif".to_string());
    typography.insert("font_size_base".to_string(), "16".to_string());

    let result = parse_theme("test".to_string(), &palette, &typography, Some(4.0));
    assert!(result.is_ok());

    palette.remove("primary");
    let result = parse_theme("test".to_string(), &palette, &typography, Some(4.0));
    assert!(result.is_err());
}

#[test]
fn test_class_inheritance() {
    let xml = "<gravity><style_classes><style name=\"base\" background=\"#ffffff\" /><style name=\"primary\" extends=\"base\" color=\"#3498db\" /></style_classes><column><text value=\"Test\" /></column></gravity>";

    let doc = parse(xml).unwrap();
    let primary = doc.style_classes.get("primary").unwrap();
    assert_eq!(primary.extends, vec!["base".to_string()]);
}

#[test]
fn test_widget_theme_ref() {
    let xml = format!("<gravity><themes><theme name=\"global\"><palette {} /><typography font_family=\"sans-serif\" font_size_base=\"16\" /><spacing unit=\"4\" /></theme><theme name=\"local\"><palette primary=\"#e74c3c\" secondary=\"#e67e22\" success=\"#27ae60\" warning=\"#f39c12\" danger=\"#e74c3c\" background=\"#ecf0f1\" surface=\"#ffffff\" text=\"#2c3e50\" text_secondary=\"#7f8c8d\" /><typography font_family=\"sans-serif\" font_size_base=\"16\" /><spacing unit=\"4\" /></theme></themes><global_theme name=\"global\" /><column><text value=\"Global\" /><container theme_ref=\"local\"><text value=\"Local\" /></container></column></gravity>", complete_palette("#3498db", "#2ecc71", "#ecf0f1", "#2c3e50"));

    let doc = parse(&xml).unwrap();
    assert!(doc.themes.contains_key("global"));
    assert!(doc.themes.contains_key("local"));
    assert_eq!(doc.global_theme, Some("global".to_string()));
}

#[test]
fn test_full_theme_spec() {
    let xml = "<gravity><themes><theme name=\"full\"><palette primary=\"#3498db\" secondary=\"#2ecc71\" success=\"#27ae60\" warning=\"#f39c12\" danger=\"#e74c3c\" background=\"#ecf0f1\" surface=\"#ffffff\" text=\"#2c3e50\" text_secondary=\"#7f8c8d\" /><typography font_family=\"Inter, sans-serif\" font_size_base=\"16\" font_size_small=\"12\" font_size_large=\"24\" font_weight=\"bold\" line_height=\"1.5\" /><spacing unit=\"8\" /></theme></themes><global_theme name=\"full\" /><column><text value=\"Test\" /></column></gravity>";

    let doc = parse(xml).unwrap();
    let theme = doc.themes.get("full").unwrap();

    assert_eq!(theme.palette.primary, c("#3498db"));
    assert_eq!(theme.palette.success, c("#27ae60"));
    assert_eq!(theme.typography.font_family, "Inter, sans-serif");
    assert_eq!(
        theme.typography.font_weight,
        gravity_core::ir::theme::FontWeight::Bold
    );
    assert_eq!(theme.spacing.unit, 8.0);
}

#[test]
fn test_minimal_theme() {
    let xml = format!("<gravity><themes><theme name=\"minimal\"><palette {} /></theme></themes><global_theme name=\"minimal\" /><column><text value=\"Test\" /></column></gravity>", complete_palette("#3498db", "#2ecc71", "#ecf0f1", "#2c3e50"));

    let doc = parse(&xml).unwrap();
    let theme = doc.themes.get("minimal").unwrap();

    assert_eq!(theme.typography.font_family, "sans-serif");
    assert_eq!(theme.typography.font_size_base, 16.0);
    assert_eq!(theme.spacing.unit, 4.0);
}

// T112: Contract test - Inline style attributes → StyleProperties
#[test]
fn test_inline_style_parsing() {
    let xml = "<gravity><column><button background=\"#e74c3c\" color=\"#ffffff\" border_width=\"2\" border_color=\"#c0392b\" border_radius=\"4\" shadow=\"2 2 4 #00000040\" opacity=\"0.8\" label=\"Styled Button\" /></column></gravity>";

    let doc = parse(xml).unwrap();
    let button = &doc.root.children[0];

    // Verify inline style exists
    assert!(button.style.is_some());
    let style = button.style.as_ref().unwrap();

    // Verify background
    assert!(style.background.is_some());
    if let Background::Color(color) = style.background.as_ref().unwrap() {
        assert_eq!(color, &c("#e74c3c"));
    } else {
        panic!("Expected color background");
    }

    // Verify color
    assert_eq!(style.color, Some(c("#ffffff")));

    // Verify border
    assert!(style.border.is_some());
    let border = style.border.as_ref().unwrap();
    assert_eq!(border.width, 2.0);
    assert_eq!(border.color, c("#c0392b"));
    assert_eq!(border.radius.top_left, 4.0);

    // Verify shadow
    assert!(style.shadow.is_some());
    let shadow = style.shadow.as_ref().unwrap();
    assert_eq!(shadow.offset_x, 2.0);
    assert_eq!(shadow.offset_y, 2.0);
    assert_eq!(shadow.blur_radius, 4.0);

    // Verify opacity
    assert_eq!(style.opacity, Some(0.8));
}

// T113: Integration test - Inline styles override theme
#[test]
fn test_inline_style_overrides_theme() {
    let xml = format!("<gravity><themes><theme name=\"app\"><palette {} /><typography font_family=\"sans-serif\" font_size_base=\"16\" /><spacing unit=\"4\" /></theme></themes><global_theme name=\"app\" /><column><container background=\"#ffffff\" padding=\"20\"><text value=\"Override Test\" /></container></column></gravity>", complete_palette("#3498db", "#2ecc71", "#ecf0f1", "#2c3e50"));

    let doc = parse(&xml).unwrap();

    // Create theme manager
    let mut theme_manager = ThemeManager::new();
    theme_manager.load_from_document(&doc);

    // Get theme
    let theme = theme_manager.get_current_theme().unwrap();

    // Verify theme background is #ecf0f1
    assert_eq!(theme.palette.background, c("#ecf0f1"));

    // Get the container widget
    let container = &doc.root.children[0];

    // Verify container has inline background #ffffff
    assert!(container.style.is_some());
    let inline_bg = container
        .style
        .as_ref()
        .unwrap()
        .background
        .as_ref()
        .unwrap();

    if let Background::Color(color) = inline_bg {
        assert_eq!(
            color,
            &c("#ffffff"),
            "Inline background should override theme"
        );
    }

    // Test cascade resolution
    let cascade = StyleCascade::new(&doc);

    // Resolve styles for the container
    let resolved = cascade.resolve(
        container.style.as_ref(),
        &container.classes,
        Some(
            &theme
                .base_styles
                .get("container")
                .cloned()
                .unwrap_or_default(),
        ),
    );

    // Verify inline background wins
    if let Some(Background::Color(color)) = &resolved.background {
        assert_eq!(
            color,
            &c("#ffffff"),
            "Inline style should have highest priority"
        );
    } else {
        panic!("Expected inline background to be applied");
    }
}

// T114: Snapshot test - Generated code with inline styles
#[test]
fn test_inline_style_snapshot() {
    let xml = "<gravity><column><container background=\"linear-gradient(90deg, #3498db 0%, #2ecc71 100%)\" padding=\"20\" border_width=\"2\" border_color=\"#000000\" border_radius=\"8\" shadow=\"0 4 8 #00000020\"><text value=\"Gradient Container\" color=\"#ffffff\" /></container></column></gravity>";

    let doc = parse(xml).unwrap();
    let container = &doc.root.children[0];

    // Verify gradient parsing
    assert!(container.style.is_some());
    let style = container.style.as_ref().unwrap();

    if let Some(Background::Gradient(gradient)) = &style.background {
        match gradient {
            gravity_core::ir::style::Gradient::Linear { angle, stops } => {
                assert_eq!(*angle, 90.0);
                assert_eq!(stops.len(), 2);
                assert_eq!(stops[0].offset, 0.0);
                assert_eq!(stops[0].color, c("#3498db"));
                assert_eq!(stops[1].offset, 1.0);
                assert_eq!(stops[1].color, c("#2ecc71"));
            }
            _ => panic!("Expected linear gradient"),
        }
    } else {
        panic!("Expected gradient background");
    }

    // Verify all other inline styles
    assert!(style.border.is_some());
    let border = style.border.as_ref().unwrap();
    assert_eq!(border.width, 2.0);
    assert_eq!(border.radius.top_left, 8.0);

    assert!(style.shadow.is_some());

    // Verify text color
    let text = &container.children[0];
    assert!(text.style.is_some());
    assert_eq!(text.style.as_ref().unwrap().color, Some(c("#ffffff")));
}

// Test gradient mapping
#[test]
fn test_gradient_mapping() {
    use gravity_core::ir::style::{Background, ColorStop, Gradient};

    // Create a linear gradient
    let gradient = Gradient::Linear {
        angle: 45.0,
        stops: vec![
            ColorStop {
                offset: 0.0,
                color: c("#ff0000"),
            },
            ColorStop {
                offset: 1.0,
                color: c("#0000ff"),
            },
        ],
    };

    let background = Background::Gradient(gradient);

    // Verify gradient structure
    match background {
        Background::Gradient(Gradient::Linear { angle, stops }) => {
            assert_eq!(angle, 45.0);
            assert_eq!(stops.len(), 2);
        }
        _ => panic!("Expected linear gradient"),
    }
}

// Test opacity handling
#[test]
fn test_opacity_validation() {
    use gravity_core::parser::style_parser::parse_opacity;

    // Valid opacity
    assert_eq!(parse_opacity("0.5").unwrap(), 0.5);
    assert_eq!(parse_opacity("1.0").unwrap(), 1.0);
    assert_eq!(parse_opacity("0.0").unwrap(), 0.0);

    // Invalid opacity
    assert!(parse_opacity("1.5").is_err());
    assert!(parse_opacity("-0.1").is_err());
    assert!(parse_opacity("invalid").is_err());
}

// Test shadow parsing
#[test]
fn test_shadow_parsing() {
    use gravity_core::parser::style_parser::parse_shadow_attr;

    let shadow = parse_shadow_attr("2 4 8 #00000080").unwrap();
    assert_eq!(shadow.offset_x, 2.0);
    assert_eq!(shadow.offset_y, 4.0);
    assert_eq!(shadow.blur_radius, 8.0);
    assert_eq!(shadow.color, c("#00000080"));
}

// Test transform parsing
#[test]
fn test_transform_parsing() {
    use gravity_core::ir::style::Transform;
    use gravity_core::parser::style_parser::parse_transform;

    // Scale
    let transform = parse_transform("scale(1.5)").unwrap();
    assert_eq!(transform, Transform::Scale(1.5));

    // Rotate
    let transform = parse_transform("rotate(45)").unwrap();
    assert_eq!(transform, Transform::Rotate(45.0));

    // Translate
    let transform = parse_transform("translate(10, 20)").unwrap();
    assert_eq!(transform, Transform::Translate { x: 10.0, y: 20.0 });
}

// Test border parsing
#[test]
fn test_border_parsing() {
    use gravity_core::parser::style_parser::{
        parse_border_radius, parse_border_style, parse_border_width,
    };

    assert_eq!(parse_border_width("2").unwrap(), 2.0);
    assert_eq!(
        parse_border_style("solid").unwrap(),
        gravity_core::ir::style::BorderStyle::Solid
    );

    let radius = parse_border_radius("8").unwrap();
    assert_eq!(radius.top_left, 8.0);
    assert_eq!(radius.top_right, 8.0);

    // 4 values: top-left, top-right, bottom-right, bottom-left
    let radius = parse_border_radius("8 4 6 2").unwrap();
    assert_eq!(radius.top_left, 8.0);
    assert_eq!(radius.top_right, 4.0);
    assert_eq!(radius.bottom_right, 6.0);
    assert_eq!(radius.bottom_left, 2.0);
}

// T126: Contract test - Class XML → StyleClass struct
#[test]
fn test_class_parsing_contract() {
    let xml = "<gravity><style_classes><style name=\"btn\" background=\"#3498db\" color=\"#ffffff\" padding=\"12 24\" extends=\"base\" /></style_classes><column><button class=\"btn\" label=\"Test\" /></column></gravity>";

    let doc = parse(xml).unwrap();

    // Verify class exists
    assert!(doc.style_classes.contains_key("btn"));
    let class = doc.style_classes.get("btn").unwrap();

    // Verify properties
    assert_eq!(class.name, "btn");
    assert!(class.style.background.is_some());
    assert!(class.style.color.is_some());
    assert_eq!(class.extends, vec!["base".to_string()]);
}

// T127: Integration test - Class inheritance and merging
#[test]
fn test_class_inheritance_and_merging() {
    let xml = "<gravity><style_classes><style name=\"base\" background=\"#ffffff\" padding=\"8\" /><style name=\"primary\" extends=\"base\" background=\"#3498db\" color=\"#ffffff\" /><style name=\"large\" font_size=\"20\" /></style_classes><column><button class=\"primary large\" label=\"Test\" /></column></gravity>";

    let doc = parse(xml).unwrap();

    // Verify classes exist
    assert!(doc.style_classes.contains_key("base"));
    assert!(doc.style_classes.contains_key("primary"));
    assert!(doc.style_classes.contains_key("large"));

    // Verify widget has multiple classes
    let button = &doc.root.children[0];
    assert_eq!(
        button.classes,
        vec!["primary".to_string(), "large".to_string()]
    );

    // Test cascade resolution
    let cascade = StyleCascade::new(&doc);
    let resolved = cascade.resolve(None, &button.classes, None);

    // Should have merged properties from base → primary → large
    assert!(resolved.background.is_some());
    assert!(resolved.color.is_some());
}

// T128: Integration test - Hot-reload class updates
#[test]
fn test_class_hot_reload() {
    let xml1 = "<gravity><style_classes><style name=\"btn\" background=\"#3498db\" /></style_classes><column><button class=\"btn\" label=\"Test\" /></column></gravity>";
    let xml2 = "<gravity><style_classes><style name=\"btn\" background=\"#e74c3c\" /></style_classes><column><button class=\"btn\" label=\"Test\" /></column></gravity>";

    let registry = HandlerRegistry::new();
    let mut interpreter = HotReloadInterpreter::new(registry);

    // Load first version
    interpreter.reload_document(xml1).unwrap();
    let doc1 = interpreter.document().unwrap();
    let class1 = doc1.style_classes.get("btn").unwrap();

    // Verify first background
    if let Background::Color(color) = class1.style.background.as_ref().unwrap() {
        assert_eq!(color, &c("#3498db"));
    }

    // Reload with second version
    interpreter.reload_document(xml2).unwrap();
    let doc2 = interpreter.document().unwrap();
    let class2 = doc2.style_classes.get("btn").unwrap();

    // Verify second background
    if let Background::Color(color) = class2.style.background.as_ref().unwrap() {
        assert_eq!(color, &c("#e74c3c"));
    }
}

// T129: Error test - Circular dependency detection
#[test]
fn test_circular_dependency_detection() {
    use gravity_core::parser::theme_parser::parse_style_class;
    use std::collections::HashMap;

    // Create circular dependency: a → b → a
    // This will be caught by depth check (6 levels) or circular check
    let mut classes = HashMap::new();

    let class_a = parse_style_class(
        "a".to_string(),
        &HashMap::new(),
        vec!["b".to_string()],
        HashMap::new(),
        HashMap::new(),
        None,
    )
    .unwrap();

    let class_b = parse_style_class(
        "b".to_string(),
        &HashMap::new(),
        vec!["a".to_string()],
        HashMap::new(),
        HashMap::new(),
        None,
    )
    .unwrap();

    classes.insert("a".to_string(), class_a);
    classes.insert("b".to_string(), class_b);

    // Validation should fail (either depth or circular)
    let result = classes.get("a").unwrap().validate(&classes);
    assert!(result.is_err());
    let err = result.unwrap_err();
    // Should mention circular, dependency, or depth
    assert!(
        err.contains("Circular")
            || err.contains("circular")
            || err.contains("dependency")
            || err.contains("depth")
    );
}

// Test inheritance depth limit
#[test]
fn test_inheritance_depth_limit() {
    use gravity_core::parser::theme_parser::parse_style_class;
    use std::collections::HashMap;

    let mut classes = HashMap::new();

    // Create chain: c6 → c5 → c4 → c3 → c2 → c1 → c0 (7 levels)
    // Depth check: c6(0) → c5(1) → c4(2) → c3(3) → c2(4) → c1(5) → c0(6)
    // At c0, depth = 6, which exceeds 5
    for i in 0..7 {
        let name = format!("c{}", i);
        let extends = if i > 0 {
            vec![format!("c{}", i - 1)]
        } else {
            vec![]
        };

        let class = parse_style_class(
            name.clone(),
            &HashMap::new(),
            extends.clone(),
            HashMap::new(),
            HashMap::new(),
            None,
        )
        .unwrap();

        classes.insert(name, class);
    }

    // c6 should fail validation (depth exceeds 5)
    let result = classes.get("c6").unwrap().validate(&classes);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("depth") || err.contains("5 levels"));
}

// Test multiple classes merging order
#[test]
fn test_multiple_classes_merge_order() {
    let xml = "<gravity><style_classes><style name=\"a\" background=\"#ff0000\" color=\"#000000\" /><style name=\"b\" background=\"#00ff00\" /></style_classes><column><button class=\"a b\" label=\"Test\" /></column></gravity>";

    let doc = parse(xml).unwrap();
    let cascade = StyleCascade::new(&doc);
    let button = &doc.root.children[0];

    // Classes are applied in order: a then b
    // b's background should override a's
    let resolved = cascade.resolve(None, &button.classes, None);

    if let Some(Background::Color(color)) = &resolved.background {
        // b's background (#00ff00) should win
        assert_eq!(color, &c("#00ff00"));
    }

    // a's color should remain
    assert_eq!(resolved.color, Some(c("#000000")));
}

// Test state variants with classes
#[test]
fn test_class_state_variants() {
    // Use child elements for state variants (as per XML schema)
    let xml = "<gravity><style_classes><style name=\"btn\"><base background=\"#3498db\" /><hover background=\"#2980b9\" /><active background=\"#21618c\" /></style></style_classes><column><button class=\"btn\" label=\"Test\" /></column></gravity>";

    let doc = parse(xml).unwrap();
    let class = doc.style_classes.get("btn").unwrap();

    // Verify state variants exist
    assert!(class.state_variants.contains_key(&WidgetState::Hover));
    assert!(class.state_variants.contains_key(&WidgetState::Active));

    // Verify hover style
    let hover_style = class.state_variants.get(&WidgetState::Hover).unwrap();
    if let Some(Background::Color(color)) = &hover_style.background {
        assert_eq!(color, &c("#2980b9"));
    }
}

// Test class with layout constraints
#[test]
fn test_class_with_layout() {
    let xml = "<gravity><style_classes><style name=\"card\" background=\"#ffffff\" padding=\"20\" width=\"300\" height=\"200\" /></style_classes><column><container class=\"card\" /></column></gravity>";

    let doc = parse(xml).unwrap();
    let class = doc.style_classes.get("card").unwrap();

    // Verify layout exists
    assert!(class.layout.is_some());
    let layout = class.layout.as_ref().unwrap();

    assert_eq!(
        layout.width,
        Some(gravity_core::ir::layout::Length::Fixed(300.0))
    );
    assert_eq!(
        layout.height,
        Some(gravity_core::ir::layout::Length::Fixed(200.0))
    );
    assert!(layout.padding.is_some());
}

// Test widget class extraction
#[test]
fn test_widget_class_extraction() {
    let xml = "<gravity><style_classes><style name=\"btn\" background=\"#3498db\" /></style_classes><column><button class=\"btn primary large\" label=\"Test\" /></column></gravity>";

    let doc = parse(xml).unwrap();
    let button = &doc.root.children[0];

    // Verify all classes are extracted
    assert_eq!(button.classes.len(), 3);
    assert_eq!(button.classes[0], "btn");
    assert_eq!(button.classes[1], "primary");
    assert_eq!(button.classes[2], "large");
}

// Test empty class attribute
#[test]
fn test_empty_class_attribute() {
    let xml = "<gravity><column><button label=\"Test\" /></column></gravity>";

    let doc = parse(xml).unwrap();
    let button = &doc.root.children[0];

    // Should have no classes
    assert!(button.classes.is_empty());
}

// Test class attribute with extra whitespace
#[test]
fn test_class_whitespace_handling() {
    let xml = "<gravity><style_classes><style name=\"btn\" background=\"#3498db\" /></style_classes><column><button class=\"  btn   primary  \" label=\"Test\" /></column></gravity>";

    let doc = parse(xml).unwrap();
    let button = &doc.root.children[0];

    // Should handle whitespace correctly
    assert_eq!(button.classes.len(), 2);
    assert_eq!(button.classes[0], "btn");
    assert_eq!(button.classes[1], "primary");
}
