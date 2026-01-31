//! Edge case tests for the Dampen LSP server.
//!
//! Tests large files, concurrent documents, and boundary conditions.

use std::path::PathBuf;

use tower_lsp::lsp_types::*;
use url::Url;

use dampen_lsp::document::{DocumentCache, DocumentState};
use dampen_lsp::handlers::{completion, diagnostics, hover};

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

// ============================================================================
// Large File Tests
// ============================================================================

/// T086: Test parsing of very large files (3600+ lines)
#[test]
fn test_large_file_parsing_performance() {
    let content = load_fixture("large_file.dampen");
    let line_count = content.lines().count();

    assert!(
        line_count >= 1000,
        "Test fixture should have at least 1000 lines, got {}",
        line_count
    );

    let uri = test_uri("large_file.dampen");
    let start_time = std::time::Instant::now();
    let doc_state = DocumentState::new(uri, content, 1);
    let elapsed = start_time.elapsed();

    // Should parse within reasonable time (less than 100ms)
    assert!(
        elapsed.as_millis() < 100,
        "Large file parsing should complete within 100ms, took {}ms",
        elapsed.as_millis()
    );

    // Document should have content
    assert!(!doc_state.content.is_empty());
    println!("Parsed {} lines in {}ms", line_count, elapsed.as_millis());
}

/// T086: Test completion on large file at various positions
#[test]
fn test_large_file_completion_at_start() {
    let content = load_fixture("large_file.dampen");
    let uri = test_uri("large_completion_start.dampen");
    let doc_state = DocumentState::new(uri.clone(), content, 1);

    let position = Position::new(0, 0);
    let params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: None,
    };

    let start_time = std::time::Instant::now();
    let response = completion::completion(&doc_state, params);
    let elapsed = start_time.elapsed();

    // Should respond quickly even on large file
    assert!(
        elapsed.as_millis() < 100,
        "Completion on large file should be fast, took {}ms",
        elapsed.as_millis()
    );

    // Should return some results
    assert!(response.is_some());
}

/// T086: Test diagnostics on large file
#[test]
fn test_large_file_diagnostics_performance() {
    let content = load_fixture("large_file.dampen");
    let uri = test_uri("large_diagnostics.dampen");
    let doc_state = DocumentState::new(uri, content, 1);

    let start_time = std::time::Instant::now();
    let diagnostics = diagnostics::compute_diagnostics(&doc_state);
    let elapsed = start_time.elapsed();

    // Should compute diagnostics quickly
    assert!(
        elapsed.as_millis() < 100,
        "Diagnostics on large file should be fast, took {}ms",
        elapsed.as_millis()
    );

    println!(
        "Computed {} diagnostics in {}ms",
        diagnostics.len(),
        elapsed.as_millis()
    );
}

/// T086: Test hover on large file
#[test]
fn test_large_file_hover_performance() {
    let content = load_fixture("large_file.dampen");
    let uri = test_uri("large_hover.dampen");
    let doc_state = DocumentState::new(uri, content, 1);

    // Test hover at various positions
    let positions = vec![
        Position::new(0, 5),
        Position::new(10, 5),
        Position::new(50, 5),
    ];

    for position in positions {
        let start_time = std::time::Instant::now();
        let _response = hover::hover(&doc_state, position);
        let elapsed = start_time.elapsed();

        assert!(
            elapsed.as_millis() < 50,
            "Hover on large file should be fast, took {}ms at line {}",
            elapsed.as_millis(),
            position.line
        );
    }
}

// ============================================================================
// Concurrent Documents Tests
// ============================================================================

/// T086: Test cache with many concurrent documents
#[test]
fn test_concurrent_documents_cache() {
    let mut cache = DocumentCache::new(50);
    let doc_count = 50;

    // Insert 50 documents
    for i in 0..doc_count {
        let uri = test_uri(&format!("concurrent_{}.dampen", i));
        let content = format!("<column><text value=\"Document {}\"/></column>", i);
        let doc_state = DocumentState::new(uri.clone(), content, i as i32);
        cache.insert(uri, doc_state);
    }

    assert_eq!(cache.len(), doc_count);

    // Verify all documents are accessible
    for i in 0..doc_count {
        let uri = test_uri(&format!("concurrent_{}.dampen", i));
        let doc = cache.get(&uri);
        assert!(doc.is_some(), "Document {} should be in cache", i);
        assert_eq!(doc.unwrap().version, i as i32);
    }
}

/// T086: Test cache eviction with LRU policy
#[test]
fn test_cache_lru_eviction() {
    let mut cache = DocumentCache::new(5);

    // Insert 5 documents
    for i in 0..5 {
        let uri = test_uri(&format!("evict_{}.dampen", i));
        let content = format!("<text value=\"{}\"/>", i);
        let doc_state = DocumentState::new(uri.clone(), content, i as i32);
        cache.insert(uri, doc_state);
    }

    assert_eq!(cache.len(), 5);

    // Access documents 1 and 3 to make them recently used
    let uri1 = test_uri("evict_1.dampen");
    let uri3 = test_uri("evict_3.dampen");
    cache.get(&uri1);
    cache.get(&uri3);

    // Insert a 6th document - should evict the least recently used (0, 2, or 4)
    let uri6 = test_uri("evict_6.dampen");
    let doc6 = DocumentState::new(uri6.clone(), "<text value=\"6\"/>".to_string(), 6);
    cache.insert(uri6.clone(), doc6);

    assert_eq!(cache.len(), 5);

    // Documents 1 and 3 should still be there (recently accessed)
    assert!(
        cache.get(&uri1).is_some(),
        "Recently accessed doc 1 should remain"
    );
    assert!(
        cache.get(&uri3).is_some(),
        "Recently accessed doc 3 should remain"
    );

    // New document 6 should be there
    assert!(cache.get(&uri6).is_some(), "New doc 6 should be in cache");
}

/// T086: Test rapid document updates
#[test]
fn test_rapid_document_updates() {
    let mut cache = DocumentCache::new(10);
    let uri = test_uri("rapid_updates.dampen");

    // Simulate rapid updates (100 versions)
    for version in 0..100 {
        let content = format!("<column><text value=\"Version {}\"/></column>", version);
        let doc_state = DocumentState::new(uri.clone(), content, version);
        cache.insert(uri.clone(), doc_state);
    }

    // Only the latest version should be in cache
    let doc = cache.get(&uri).unwrap();
    assert_eq!(doc.version, 99);
    assert!(doc.content.contains("Version 99"));
}

/// T086: Test document cache with concurrent access patterns
#[test]
fn test_concurrent_access_patterns() {
    let mut cache = DocumentCache::new(20);

    // Simulate a workspace with multiple files being accessed
    let file_patterns = vec![
        ("main.dampen", 10),   // Accessed frequently
        ("header.dampen", 8),  // Accessed often
        ("footer.dampen", 5),  // Accessed sometimes
        ("sidebar.dampen", 3), // Rarely accessed
        ("modal.dampen", 1),   // Rarely accessed
    ];

    // Insert all files
    for (name, _) in &file_patterns {
        let uri = test_uri(name);
        let content = format!("<column><text value=\"{}\"/></column>", name);
        let doc_state = DocumentState::new(uri.clone(), content, 1);
        cache.insert(uri, doc_state);
    }

    // Simulate access patterns
    for (name, accesses) in &file_patterns {
        let uri = test_uri(name);
        for _ in 0..*accesses {
            let doc = cache.get(&uri);
            assert!(doc.is_some(), "{} should be accessible", name);
        }
    }

    // All files should still be in cache (under capacity)
    assert_eq!(cache.len(), file_patterns.len());
}

// ============================================================================
// Edge Case Tests
// ============================================================================

/// T086: Test empty document
#[test]
fn test_empty_document() {
    let uri = test_uri("empty.dampen");
    let content = "";

    let doc_state = DocumentState::new(uri, content.to_string(), 1);

    // Should handle empty document gracefully
    assert_eq!(doc_state.content, "");
    assert!(doc_state.ast.is_none() || doc_state.parse_errors.len() > 0);
}

/// T086: Test document with only whitespace
#[test]
fn test_whitespace_only_document() {
    let uri = test_uri("whitespace.dampen");
    let content = "   \n\t\n   ";

    let doc_state = DocumentState::new(uri, content.to_string(), 1);

    // Should handle whitespace-only document
    assert_eq!(doc_state.content, content);
}

/// T086: Test document with very long lines
#[test]
fn test_very_long_lines() {
    let uri = test_uri("long_lines.dampen");
    let long_value = "x".repeat(10000);
    let content = format!("<text value=\"{}\" />", long_value);

    let start_time = std::time::Instant::now();
    let doc_state = DocumentState::new(uri, content.clone(), 1);
    let elapsed = start_time.elapsed();

    // Should handle long lines within reasonable time
    assert!(
        elapsed.as_millis() < 50,
        "Long line parsing should be fast, took {}ms",
        elapsed.as_millis()
    );

    assert_eq!(doc_state.content.len(), content.len());
}

/// T086: Test document with deeply nested structure
#[test]
fn test_deeply_nested_document() {
    let uri = test_uri("nested.dampen");
    let mut content = String::new();

    // Create deeply nested structure (100 levels)
    for i in 0..100 {
        content.push_str(&format!("<column id=\"{}\">\n", i));
    }
    content.push_str("<text value=\"deep\"/>\n");
    for _ in 0..100 {
        content.push_str("</column>\n");
    }

    let start_time = std::time::Instant::now();
    let doc_state = DocumentState::new(uri, content.clone(), 1);
    let elapsed = start_time.elapsed();

    // Should handle deep nesting within reasonable time
    assert!(
        elapsed.as_millis() < 100,
        "Deep nesting parsing should be fast, took {}ms",
        elapsed.as_millis()
    );

    println!(
        "Parsed {} bytes with deep nesting in {}ms",
        content.len(),
        elapsed.as_millis()
    );
}

/// T086: Test document with many attributes
#[test]
fn test_many_attributes() {
    let uri = test_uri("many_attrs.dampen");
    let mut attrs = String::new();

    // Create widget with many attributes
    for i in 0..50 {
        attrs.push_str(&format!(" attr{}=\"value{}\"", i, i));
    }

    let content = format!("<column{} />", attrs);

    let start_time = std::time::Instant::now();
    let doc_state = DocumentState::new(uri, content.clone(), 1);
    let elapsed = start_time.elapsed();

    // Should handle many attributes within reasonable time
    assert!(
        elapsed.as_millis() < 50,
        "Many attributes parsing should be fast, took {}ms",
        elapsed.as_millis()
    );

    assert_eq!(doc_state.content.len(), content.len());
}

/// T086: Test completion at end of document
#[test]
fn test_completion_at_document_end() {
    let content = "<column>\n    <text value=\"test\" />\n</column>";
    let uri = test_uri("end_completion.dampen");
    let doc_state = DocumentState::new(uri.clone(), content.to_string(), 1);

    // Position at end of document
    let position = Position::new(2, 10);
    let params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: None,
    };

    let response = completion::completion(&doc_state, params);

    // Should handle gracefully (may return None or empty list)
    // The key is that it doesn't panic
    println!("Completion at end of document: {:?}", response.is_some());
}

/// T086: Test hover at invalid positions
#[test]
fn test_hover_at_invalid_positions() {
    let content = "<column>\n    <text value=\"test\" />\n</column>";
    let uri = test_uri("invalid_hover.dampen");
    let doc_state = DocumentState::new(uri, content.to_string(), 1);

    // Test various edge case positions
    let positions = vec![
        Position::new(100, 0),  // Beyond document end
        Position::new(0, 1000), // Beyond line end
        Position::new(0, 0),    // Start of document
    ];

    for position in positions {
        // Should not panic
        let _response = hover::hover(&doc_state, position);
    }
}

/// T086: Test cache clear operation
#[test]
fn test_cache_clear() {
    let mut cache = DocumentCache::new(10);

    // Insert some documents
    for i in 0..5 {
        let uri = test_uri(&format!("clear_{}.dampen", i));
        let doc_state = DocumentState::new(uri, "<text/>".to_string(), i as i32);
        cache.insert(test_uri(&format!("clear_{}.dampen", i)), doc_state);
    }

    assert_eq!(cache.len(), 5);

    // Clear cache
    cache.clear();

    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
}

/// T086: Test cache with special characters in URI
#[test]
fn test_cache_special_uri_characters() {
    let mut cache = DocumentCache::new(10);

    // URIs with special characters
    let special_uris = vec![
        "file:///path/with%20spaces/file.dampen",
        "file:///path/with-unicode-文件.dampen",
        "file:///path/with.dots/file.dampen",
    ];

    for (i, uri_str) in special_uris.iter().enumerate() {
        let uri = Url::parse(uri_str).unwrap();
        let doc_state = DocumentState::new(uri.clone(), "<text/>".to_string(), i as i32);
        cache.insert(uri.clone(), doc_state);

        // Should be retrievable
        let retrieved = cache.get(&uri);
        assert!(
            retrieved.is_some(),
            "Should retrieve document with URI: {}",
            uri_str
        );
    }
}

/// T086: Test document state with unicode content
#[test]
fn test_unicode_content() {
    let uri = test_uri("unicode.dampen");
    let content = r#"<text value="Hello 世界 🌍 Ñoño" />"#;

    let doc_state = DocumentState::new(uri, content.to_string(), 1);

    assert_eq!(doc_state.content, content);

    // Compute diagnostics - should handle unicode
    let diagnostics = diagnostics::compute_diagnostics(&doc_state);
    // Should not panic on unicode content
    println!("Unicode diagnostics count: {}", diagnostics.len());
}
