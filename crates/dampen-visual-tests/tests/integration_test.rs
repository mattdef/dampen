//! Integration tests for visual testing infrastructure.

use dampen_visual_tests::{VisualTestCase, VisualTestResult};

#[test]
fn test_visual_test_case_creation() {
    let test_case = VisualTestCase::new("hello_world", "<Text value=\"Hello\" />");
    assert_eq!(test_case.name, "hello_world");
    assert_eq!(test_case.tolerance, 0.01);
}

#[test]
fn test_visual_test_case_with_custom_tolerance() {
    let test_case = VisualTestCase::new("test", "<Button />").with_tolerance(0.05);
    assert_eq!(test_case.tolerance, 0.05);
}

#[test]
fn test_visual_test_result_pass() {
    let result = VisualTestResult::pass("test", 0.001);
    assert!(result.passed);
    assert_eq!(result.difference, 0.001);
}

#[test]
fn test_visual_test_result_fail() {
    let result = VisualTestResult::fail("test", 0.1);
    assert!(!result.passed);
    assert_eq!(result.difference, 0.1);
}

#[test]
fn test_visual_test_result_with_paths() {
    let result = VisualTestResult::pass("test", 0.001).with_paths(
        "baseline.png",
        "actual.png",
        Some("diff.png"),
    );

    assert_eq!(result.baseline_path, Some("baseline.png".to_string()));
    assert_eq!(result.actual_path, Some("actual.png".to_string()));
    assert_eq!(result.diff_path, Some("diff.png".to_string()));
}

#[test]
fn test_complex_nested_layout_case_exists() {
    use std::path::Path;

    let case_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("tests/visual/cases/complex_nested_layout.dampen");

    assert!(
        case_path.exists(),
        "Complex nested layout test case should exist at {:?}",
        case_path
    );

    let content = std::fs::read_to_string(&case_path)
        .expect("Should be able to read complex_nested_layout.dampen");

    // Verify it contains key nested elements
    assert!(
        content.contains("<column"),
        "Should contain column elements"
    );
    assert!(content.contains("<row"), "Should contain row elements");
    assert!(
        content.contains("align_x"),
        "Should test horizontal alignment"
    );
    assert!(
        content.contains("align_y"),
        "Should test vertical alignment"
    );
    assert!(content.contains("padding"), "Should test padding");
    assert!(content.contains("spacing"), "Should test spacing");
}
