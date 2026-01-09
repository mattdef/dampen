// Unit tests for theme validation
use dampen_cli::commands::check::errors::CheckError;
use dampen_cli::commands::check::themes::ThemeValidator;

#[test]
fn test_invalid_theme_property_detection() {
    // Test detection of invalid theme property (e.g., negative font size)
    let mut validator = ThemeValidator::new();

    // Add a theme with invalid property
    let result = validator.add_invalid_theme_property(
        "test_theme",
        "font_size_base",
        "-16.0",
        "test.gravity",
        10,
        5,
    );

    // Should detect the invalid property
    assert!(result.is_err());

    let errors = validator.validate();
    assert!(!errors.is_empty());

    // Check that we have an InvalidThemeProperty error
    let has_invalid_property = errors
        .iter()
        .any(|e| matches!(e, CheckError::InvalidThemeProperty { .. }));
    assert!(
        has_invalid_property,
        "Expected invalid theme property error"
    );
}

#[test]
fn test_circular_dependency_detection() {
    // Test detection of circular dependencies in style class inheritance
    let mut validator = ThemeValidator::new();

    // Create style classes with circular dependency:
    // class_a extends class_b
    // class_b extends class_c
    // class_c extends class_a (circular!)

    validator.add_style_class("class_a", vec!["class_b".to_string()], "test.gravity", 5, 1);
    validator.add_style_class(
        "class_b",
        vec!["class_c".to_string()],
        "test.gravity",
        10,
        1,
    );
    validator.add_style_class(
        "class_c",
        vec!["class_a".to_string()],
        "test.gravity",
        15,
        1,
    );

    // Validate should detect circular dependency
    let errors = validator.validate();
    assert!(!errors.is_empty());

    // Check that we have a circular dependency error
    let has_circular_error = errors
        .iter()
        .any(|e| matches!(e, CheckError::ThemeCircularDependency { .. }));
    assert!(has_circular_error, "Expected circular dependency error");
}

#[test]
fn test_valid_theme() {
    // Test that a valid theme passes validation
    let validator = ThemeValidator::new();

    // No invalid themes or style classes added
    let errors = validator.validate();
    assert!(errors.is_empty(), "Expected no errors for valid theme");
}

#[test]
fn test_style_class_extends_nonexistent() {
    // Test that extending a non-existent style class produces an error
    let mut validator = ThemeValidator::new();

    validator.add_style_class(
        "child_class",
        vec!["nonexistent_parent".to_string()],
        "test.gravity",
        5,
        1,
    );

    // Validate should detect missing parent class
    let errors = validator.validate();
    assert!(!errors.is_empty());

    let has_invalid_property = errors
        .iter()
        .any(|e| matches!(e, CheckError::InvalidThemeProperty { .. }));
    assert!(
        has_invalid_property,
        "Expected invalid theme property error for missing parent"
    );
}

#[test]
fn test_multiple_circular_dependencies() {
    // Test detection of multiple circular dependencies
    let mut validator = ThemeValidator::new();

    // First circular dependency: a -> b -> a
    validator.add_style_class("class_a", vec!["class_b".to_string()], "test.gravity", 5, 1);
    validator.add_style_class(
        "class_b",
        vec!["class_a".to_string()],
        "test.gravity",
        10,
        1,
    );

    // Second circular dependency: c -> d -> e -> c
    validator.add_style_class(
        "class_c",
        vec!["class_d".to_string()],
        "test.gravity",
        15,
        1,
    );
    validator.add_style_class(
        "class_d",
        vec!["class_e".to_string()],
        "test.gravity",
        20,
        1,
    );
    validator.add_style_class(
        "class_e",
        vec!["class_c".to_string()],
        "test.gravity",
        25,
        1,
    );

    let errors = validator.validate();

    // Should have at least 2 circular dependency errors
    let circular_error_count = errors
        .iter()
        .filter(|e| matches!(e, CheckError::ThemeCircularDependency { .. }))
        .count();
    assert!(
        circular_error_count >= 2,
        "Expected at least 2 circular dependency errors, got {}",
        circular_error_count
    );
}

#[test]
fn test_deep_inheritance_without_cycle() {
    // Test that deep inheritance without cycles is valid
    let mut validator = ThemeValidator::new();

    // Create a chain: a -> b -> c -> d (no cycle)
    validator.add_style_class("class_a", vec!["class_b".to_string()], "test.gravity", 5, 1);
    validator.add_style_class(
        "class_b",
        vec!["class_c".to_string()],
        "test.gravity",
        10,
        1,
    );
    validator.add_style_class(
        "class_c",
        vec!["class_d".to_string()],
        "test.gravity",
        15,
        1,
    );
    validator.add_style_class("class_d", vec![], "test.gravity", 20, 1);

    let errors = validator.validate();

    // Should have no circular dependency errors
    let has_circular_error = errors
        .iter()
        .any(|e| matches!(e, CheckError::ThemeCircularDependency { .. }));
    assert!(
        !has_circular_error,
        "Expected no circular dependency error for valid inheritance chain"
    );
}
