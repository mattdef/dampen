// Unit tests for radio group validation
use dampen_cli::commands::check::cross_widget::RadioGroupValidator;
use dampen_cli::commands::check::errors::CheckError;

#[test]
fn test_duplicate_radio_value_detection() {
    // Create radio group with duplicate values
    let mut validator = RadioGroupValidator::new();

    // Add first radio button with value "option1"
    validator.add_radio("group1", "option1", "file.gravity", 10, 5, None);

    // Add second radio button with same value "option1" (should be an error)
    validator.add_radio("group1", "option1", "file.gravity", 15, 5, None);

    // Validate and expect duplicate value error
    let errors = validator.validate();
    assert_eq!(errors.len(), 1);

    match &errors[0] {
        CheckError::DuplicateRadioValue { value, group, .. } => {
            assert_eq!(value, "option1");
            assert_eq!(group, "group1");
        }
        _ => panic!("Expected DuplicateRadioValue error"),
    }
}

#[test]
fn test_inconsistent_handler_detection() {
    // Create radio group with different handlers
    let mut validator = RadioGroupValidator::new();

    // Add radio buttons with different handlers
    validator.add_radio(
        "group1",
        "opt1",
        "file.gravity",
        10,
        5,
        Some("handler1".to_string()),
    );
    validator.add_radio(
        "group1",
        "opt2",
        "file.gravity",
        15,
        5,
        Some("handler2".to_string()),
    );

    // Validate and expect inconsistent handler error
    let errors = validator.validate();
    assert_eq!(errors.len(), 1);

    match &errors[0] {
        CheckError::InconsistentRadioHandlers { group, .. } => {
            assert_eq!(group, "group1");
        }
        _ => panic!("Expected InconsistentRadioHandlers error"),
    }
}

#[test]
fn test_valid_radio_group() {
    // Create valid radio group
    let mut validator = RadioGroupValidator::new();

    // Add radio buttons with unique values and same handler
    validator.add_radio(
        "group1",
        "opt1",
        "file.gravity",
        10,
        5,
        Some("handler".to_string()),
    );
    validator.add_radio(
        "group1",
        "opt2",
        "file.gravity",
        15,
        5,
        Some("handler".to_string()),
    );
    validator.add_radio(
        "group1",
        "opt3",
        "file.gravity",
        20,
        5,
        Some("handler".to_string()),
    );

    // Validate and expect no errors
    let errors = validator.validate();
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_multiple_radio_groups() {
    // Test that different groups are validated independently
    let mut validator = RadioGroupValidator::new();

    // Group 1 with valid values
    validator.add_radio(
        "group1",
        "opt1",
        "file.gravity",
        10,
        5,
        Some("handler1".to_string()),
    );
    validator.add_radio(
        "group1",
        "opt2",
        "file.gravity",
        15,
        5,
        Some("handler1".to_string()),
    );

    // Group 2 with valid values
    validator.add_radio(
        "group2",
        "opt1",
        "file.gravity",
        20,
        5,
        Some("handler2".to_string()),
    );
    validator.add_radio(
        "group2",
        "opt2",
        "file.gravity",
        25,
        5,
        Some("handler2".to_string()),
    );

    // Validate and expect no errors
    let errors = validator.validate();
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_radio_group_with_no_handler() {
    // Test radio buttons without handlers (valid)
    let mut validator = RadioGroupValidator::new();

    validator.add_radio("group1", "opt1", "file.gravity", 10, 5, None);
    validator.add_radio("group1", "opt2", "file.gravity", 15, 5, None);

    // Should be valid - no handlers is fine
    let errors = validator.validate();
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_radio_group_mixed_handlers() {
    // Test some with handler, some without (should be an error)
    let mut validator = RadioGroupValidator::new();

    validator.add_radio(
        "group1",
        "opt1",
        "file.gravity",
        10,
        5,
        Some("handler".to_string()),
    );
    validator.add_radio("group1", "opt2", "file.gravity", 15, 5, None);

    // Should detect inconsistent handlers
    let errors = validator.validate();
    assert_eq!(errors.len(), 1);

    match &errors[0] {
        CheckError::InconsistentRadioHandlers { .. } => {}
        _ => panic!("Expected InconsistentRadioHandlers error"),
    }
}
