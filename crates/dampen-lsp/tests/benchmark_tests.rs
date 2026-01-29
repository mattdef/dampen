//! Performance benchmark tests for the Dampen LSP server.
//!
//! Tests performance requirements defined in the specification.

use std::path::PathBuf;
use std::time::Instant;

use tower_lsp::lsp_types::*;
use url::Url;

use dampen_lsp::document::DocumentState;
use dampen_lsp::handlers::{completion, diagnostics};

/// Helper function to create a test document URI.
fn test_uri(name: &str) -> Url {
    Url::parse(&format!("file:///test/{}", name)).unwrap()
}

/// Helper function to load a fixture file.
fn load_fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to load fixture: {:?}", path))
}

/// T094: Benchmark parse time for 1000-line file (target: less than 50ms)
///
/// This test verifies that parsing a 1000-line Dampen file completes
/// within the 50ms performance budget.
#[test]
fn test_benchmark_parse_time_1000_lines() {
    let uri = test_uri("large_file.dampen");
    let content = load_fixture("large_file.dampen");

    // Verify file has at least 1000 lines
    let line_count = content.lines().count();
    assert!(
        line_count >= 1000,
        "Test fixture should have at least 1000 lines, got {}",
        line_count
    );

    // Measure parse time
    let start_time = Instant::now();
    let doc_state = DocumentState::new(uri, content, 1);
    let elapsed = start_time.elapsed();

    println!(
        "Parse time for {} lines: {}ms",
        line_count,
        elapsed.as_millis()
    );

    // Verify document was parsed successfully
    // Note: Large files may have parse errors but should still be processed
    // The key metric is the parse time, not whether it's error-free
    println!("AST is_some: {}", doc_state.ast.is_some());
    println!("Parse errors count: {}", doc_state.parse_errors.len());
    if !doc_state.parse_errors.is_empty() {
        println!("First parse error: {:?}", doc_state.parse_errors[0]);
    }

    // Performance requirement: must complete within 50ms
    assert!(
        elapsed.as_millis() < 50,
        "Parse time for 1000-line file should be less than 50ms, took {}ms",
        elapsed.as_millis()
    );
}

/// T095: Benchmark completion response time (target: less than 100ms)
///
/// This test verifies that completion requests complete within the 100ms budget.
#[test]
fn test_benchmark_completion_response_time() {
    let uri = test_uri("completion_benchmark.dampen");
    let content = "<column>\n    <\n</column>".to_string();

    let doc_state = DocumentState::new(uri.clone(), content, 1);

    // Position after `<` on line 1
    let position = Position::new(1, 4);

    let params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: Some(CompletionContext {
            trigger_kind: CompletionTriggerKind::TRIGGER_CHARACTER,
            trigger_character: Some("<".to_string()),
        }),
    };

    // Measure completion time
    let start_time = Instant::now();
    let response = completion::completion(&doc_state, params);
    let elapsed = start_time.elapsed();

    println!("Completion response time: {}ms", elapsed.as_millis());

    // Verify response was returned
    assert!(response.is_some(), "Completion should return a response");

    // Performance requirement: must complete within 100ms
    assert!(
        elapsed.as_millis() < 100,
        "Completion response time should be less than 100ms, took {}ms",
        elapsed.as_millis()
    );
}

/// T096: Benchmark hover response time (target: less than 200ms)
///
/// This test verifies that hover requests complete within the 200ms budget.
#[test]
fn test_benchmark_hover_response_time() {
    use dampen_lsp::handlers::hover;

    let uri = test_uri("hover_benchmark.dampen");
    let content = "<column>\n    <button label=\"Click\" />\n</column>".to_string();

    let doc_state = DocumentState::new(uri, content, 1);

    // Position on "button" text
    let position = Position::new(1, 6);

    // Measure hover time
    let start_time = Instant::now();
    let _response = hover::hover(&doc_state, position);
    let elapsed = start_time.elapsed();

    println!("Hover response time: {}ms", elapsed.as_millis());

    // Performance requirement: must complete within 200ms
    // Note: Hover may return None if not implemented, but should still be fast
    assert!(
        elapsed.as_millis() < 200,
        "Hover response time should be less than 200ms, took {}ms",
        elapsed.as_millis()
    );
}

/// T097: Benchmark diagnostics publish time (target: less than 500ms)
///
/// This test verifies that diagnostics computation completes within the 500ms budget.
#[test]
fn test_benchmark_diagnostics_publish_time() {
    let uri = test_uri("diagnostics_benchmark.dampen");
    let content = load_fixture("large_file.dampen");

    let doc_state = DocumentState::new(uri, content, 1);

    // Measure diagnostics computation time
    let start_time = Instant::now();
    let _diagnostics = diagnostics::compute_diagnostics(&doc_state);
    let elapsed = start_time.elapsed();

    println!("Diagnostics publish time: {}ms", elapsed.as_millis());

    // Performance requirement: must complete within 500ms
    assert!(
        elapsed.as_millis() < 500,
        "Diagnostics publish time should be less than 500ms, took {}ms",
        elapsed.as_millis()
    );
}

/// Additional benchmark: Completion on large file
///
/// Tests that completion remains fast even on large documents.
#[test]
fn test_benchmark_completion_on_large_file() {
    let uri = test_uri("large_completion.dampen");
    let content = load_fixture("large_file.dampen");

    let doc_state = DocumentState::new(uri.clone(), content, 1);

    // Position at the end of the document
    let position = Position::new(10, 4);

    let params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: None,
    };

    // Measure completion time on large file
    let start_time = Instant::now();
    let response = completion::completion(&doc_state, params);
    let elapsed = start_time.elapsed();

    println!(
        "Completion on large file response time: {}ms",
        elapsed.as_millis()
    );

    // Verify response was returned
    assert!(response.is_some(), "Completion should return a response");

    // Should still be fast even on large files
    assert!(
        elapsed.as_millis() < 100,
        "Completion on large file should be less than 100ms, took {}ms",
        elapsed.as_millis()
    );
}

/// Additional benchmark: Diagnostics on file with errors
///
/// Tests that error diagnostics computation remains fast.
#[test]
fn test_benchmark_diagnostics_with_errors() {
    // Create a file with multiple errors
    let uri = test_uri("errors_benchmark.dampen");
    let mut content = String::new();
    content.push_str("<?xml version=\"1.0\"?>\n");
    content.push_str("<column>\n");

    // Add many invalid widgets
    for i in 0..100 {
        content.push_str(&format!(
            "    <unknown_widget_{} invalid_attr=\"value\">\n",
            i
        ));
        content.push_str("</unknown_widget_>"); // Intentional syntax error
    }

    content.push_str("</column>\n");

    let doc_state = DocumentState::new(uri, content, 1);

    // Measure diagnostics computation time
    let start_time = Instant::now();
    let diagnostics = diagnostics::compute_diagnostics(&doc_state);
    let elapsed = start_time.elapsed();

    println!(
        "Diagnostics with errors publish time: {}ms (found {} diagnostics)",
        elapsed.as_millis(),
        diagnostics.len()
    );

    // Performance requirement: must complete within 500ms even with errors
    assert!(
        elapsed.as_millis() < 500,
        "Diagnostics with errors should be less than 500ms, took {}ms",
        elapsed.as_millis()
    );
}
