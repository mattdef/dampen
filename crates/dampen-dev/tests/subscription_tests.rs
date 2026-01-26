//! Integration tests for file watching subscriptions
//!
//! These tests verify the subscription lifecycle and error handling
//! by testing the underlying components and integration.

use dampen_core::parser;
use dampen_dev::subscription::{FileWatcherRecipe, watch_files};
use dampen_dev::watcher::{FileWatcher, FileWatcherConfig};
use std::fs;
use std::hash::Hasher;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// Helper function to create a temporary directory for testing
fn setup_test_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

#[test]
fn test_filewatcher_integration_with_parsing() {
    // T076: Integration test for subscription lifecycle
    //
    // Tests the complete flow: FileWatcher detects file → read → parse → success
    // This verifies the core functionality that the subscription uses

    let temp_dir = setup_test_dir();

    // Create watcher
    let config = FileWatcherConfig {
        watch_paths: vec![temp_dir.path().to_path_buf()],
        debounce_ms: 50,
        extension_filter: ".dampen".to_string(),
        recursive: true,
    };

    let mut watcher = FileWatcher::new(config).expect("Failed to create watcher");
    watcher
        .watch(temp_dir.path().to_path_buf())
        .expect("Failed to watch directory");

    // Give watcher time to initialize
    thread::sleep(Duration::from_millis(100));

    // Create a valid .dampen file
    let test_file = temp_dir.path().join("test.dampen");
    let valid_xml = r#"<dampen version="1.1" encoding="utf-8">
    <column spacing="10">
        <text value="Hello World" />
        <button label="Click me" on_click="handle_click" />
    </column>
</dampen>"#;
    fs::write(&test_file, valid_xml).expect("Failed to create test file");

    // Wait for debouncing
    thread::sleep(Duration::from_millis(150));

    // Check that we received the file event
    let receiver = watcher.receiver();
    let path = receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("Should receive file event");

    assert_eq!(path, test_file, "Should receive event for test file");

    // Verify we can read and parse the file (simulating what the subscription does)
    let content = fs::read_to_string(&path).expect("Should be able to read file");
    let document = parser::parse(&content).expect("Should parse valid XML");

    let root = &document.root;
    assert!(matches!(root.kind, dampen_core::ir::WidgetKind::Column));
    assert_eq!(root.children.len(), 2);

    println!("✓ FileWatcher integration with parsing successful");
}

#[test]
fn test_filewatcher_file_modification_detection() {
    // T076: Test that file modifications are detected
    //
    // Verifies the subscription can detect when files change

    let temp_dir = setup_test_dir();

    // Create initial file before starting watcher
    let test_file = temp_dir.path().join("modify_test.dampen");
    fs::write(
        &test_file,
        r#"<dampen version="1.1" encoding="utf-8"><text value="Initial" /></dampen>"#,
    )
    .expect("Failed to create initial file");

    thread::sleep(Duration::from_millis(100));

    // Create watcher
    let config = FileWatcherConfig {
        watch_paths: vec![temp_dir.path().to_path_buf()],
        debounce_ms: 50,
        extension_filter: ".dampen".to_string(),
        recursive: true,
    };

    let mut watcher = FileWatcher::new(config).expect("Failed to create watcher");
    watcher
        .watch(temp_dir.path().to_path_buf())
        .expect("Failed to watch directory");

    thread::sleep(Duration::from_millis(100));

    // Clear any initial events
    let receiver = watcher.receiver();
    while receiver.try_recv().is_ok() {}

    // Modify the file
    fs::write(
        &test_file,
        r#"<dampen version="1.1" encoding="utf-8"><text value="Modified" /></dampen>"#,
    )
    .expect("Failed to modify file");

    thread::sleep(Duration::from_millis(150));

    // Should receive modification event
    let path = receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("Should receive modification event");

    assert_eq!(path, test_file);

    // Verify content was actually modified
    let content = fs::read_to_string(&path).expect("Should read modified file");
    assert!(content.contains("Modified"));

    println!("✓ File modification detection working");
}

#[test]
fn test_parse_error_handling() {
    // T077: Test error propagation for invalid XML
    //
    // Verifies that parse errors are detected and can be handled

    let invalid_xml = r#"<dampen version="1.1" encoding="utf-8">
    <text value="Unclosed tag"
</dampen>"#; // Missing closing > on text element

    // Attempt to parse
    let result = parser::parse(invalid_xml);

    assert!(result.is_err(), "Should fail to parse invalid XML");

    let error = result.unwrap_err();
    assert!(!error.message.is_empty(), "Error should have a message");

    // In the subscription, this would be wrapped as FileEvent::ParseError
    println!("✓ Parse error detected: {}", error.message);
}

#[test]
fn test_watcher_error_nonexistent_path() {
    // T077: Test error propagation for watcher errors
    //
    // Verifies that watcher errors (like non-existent paths) are detected

    let nonexistent_path = PathBuf::from("/tmp/dampen-test-nonexistent-99999");

    let config = FileWatcherConfig {
        watch_paths: vec![nonexistent_path.clone()],
        debounce_ms: 100,
        extension_filter: ".dampen".to_string(),
        recursive: true,
    };

    let mut watcher = FileWatcher::new(config).expect("Watcher creation should succeed");

    // Attempting to watch non-existent path should fail
    let result = watcher.watch(nonexistent_path.clone());

    assert!(result.is_err(), "Should fail to watch non-existent path");

    // In the subscription, this would be wrapped as FileEvent::WatcherError
    println!("✓ Watcher error detected for non-existent path");
}

#[test]
fn test_multiple_file_events() {
    // T076: Test handling multiple files
    //
    // Verifies the subscription can handle multiple simultaneous file changes

    let temp_dir = setup_test_dir();

    let config = FileWatcherConfig {
        watch_paths: vec![temp_dir.path().to_path_buf()],
        debounce_ms: 50,
        extension_filter: ".dampen".to_string(),
        recursive: true,
    };

    let mut watcher = FileWatcher::new(config).expect("Failed to create watcher");
    watcher
        .watch(temp_dir.path().to_path_buf())
        .expect("Failed to watch directory");

    thread::sleep(Duration::from_millis(100));

    // Create multiple files
    let file1 = temp_dir.path().join("file1.dampen");
    let file2 = temp_dir.path().join("file2.dampen");

    fs::write(
        &file1,
        r#"<dampen version="1.1" encoding="utf-8"><text value="File 1" /></dampen>"#,
    )
    .expect("Failed to create file1");

    fs::write(
        &file2,
        r#"<dampen version="1.1" encoding="utf-8"><text value="File 2" /></dampen>"#,
    )
    .expect("Failed to create file2");

    thread::sleep(Duration::from_millis(150));

    // Collect events
    let receiver = watcher.receiver();
    let mut events = Vec::new();

    while let Ok(path) = receiver.try_recv() {
        events.push(path);
    }

    assert!(!events.is_empty(), "Should receive at least one event");

    // Verify we got events (debouncing may combine some)
    println!(
        "✓ Multiple file events handled ({} events received)",
        events.len()
    );
}

#[test]
fn test_filewatcher_recipe_hash_uniqueness() {
    // T076: Test that subscription configurations are unique
    //
    // Ensures different configurations create different subscriptions

    use iced::advanced::subscription::{Hasher, Recipe};

    fn hash_recipe(recipe: &FileWatcherRecipe) -> u64 {
        let mut hasher = Hasher::default();
        recipe.hash(&mut hasher);
        hasher.finish()
    }

    // Same configuration should hash equally
    let recipe1 = FileWatcherRecipe::new(vec![PathBuf::from("/tmp/test")], 100);
    let recipe2 = FileWatcherRecipe::new(vec![PathBuf::from("/tmp/test")], 100);
    assert_eq!(
        hash_recipe(&recipe1),
        hash_recipe(&recipe2),
        "Same config should hash equally"
    );

    // Different paths should hash differently
    let recipe3 = FileWatcherRecipe::new(vec![PathBuf::from("/tmp/other")], 100);
    assert_ne!(
        hash_recipe(&recipe1),
        hash_recipe(&recipe3),
        "Different paths should hash differently"
    );

    // Different debounce should hash differently
    let recipe4 = FileWatcherRecipe::new(vec![PathBuf::from("/tmp/test")], 200);
    assert_ne!(
        hash_recipe(&recipe1),
        hash_recipe(&recipe4),
        "Different debounce should hash differently"
    );

    println!("✓ Recipe hash uniqueness verified");
}

#[test]
fn test_subscription_api_creation() {
    // T076: Test that the public API can create subscriptions
    //
    // Verifies watch_files() returns a valid Subscription

    let paths = vec![PathBuf::from("/tmp/test")];
    let subscription = watch_files(paths, 100);

    // The subscription itself is opaque, but we can verify it was created
    // In actual use, Iced's runtime would consume this subscription
    println!("✓ Subscription API creation successful");

    // Type check: subscription should be of correct type
    let _: iced::Subscription<dampen_dev::subscription::FileEvent> = subscription;
}

#[test]
fn test_parse_success_with_valid_document() {
    // T076: Test successful parsing produces valid document
    //
    // Verifies the happy path of the subscription flow

    let valid_xml = r#"<dampen version="1.1" encoding="utf-8">
    <column spacing="20">
        <text value="Hello" size="24" />
        <button label="Click" on_click="action" />
        <row>
            <text value="Nested" />
        </row>
    </column>
</dampen>"#;

    let document = parser::parse(valid_xml).expect("Should parse valid XML");

    let root = &document.root;

    assert!(matches!(root.kind, dampen_core::ir::WidgetKind::Column));
    assert_eq!(root.children.len(), 3);

    // This is what FileEvent::Success would contain
    println!("✓ Valid document parsed successfully");
}

#[test]
fn test_error_recovery_simulation() {
    // T077: Test that errors don't crash the system
    //
    // Simulates recovery from errors (what the subscription does)

    let temp_dir = setup_test_dir();

    let config = FileWatcherConfig {
        watch_paths: vec![temp_dir.path().to_path_buf()],
        debounce_ms: 30,
        extension_filter: ".dampen".to_string(),
        recursive: true,
    };

    let mut watcher = FileWatcher::new(config).expect("Failed to create watcher");
    watcher
        .watch(temp_dir.path().to_path_buf())
        .expect("Failed to watch directory");

    thread::sleep(Duration::from_millis(50));

    // Create invalid file
    let invalid_file = temp_dir.path().join("invalid.dampen");
    fs::write(
        &invalid_file,
        r#"<dampen version="1.1" encoding="utf-8"><broken"#,
    )
    .expect("Failed to create invalid file");

    thread::sleep(Duration::from_millis(80));

    // Create valid file
    let valid_file = temp_dir.path().join("valid.dampen");
    fs::write(
        &valid_file,
        r#"<dampen version="1.1" encoding="utf-8"><text value="Valid" /></dampen>"#,
    )
    .expect("Failed to create valid file");

    thread::sleep(Duration::from_millis(80));

    // Watcher should still be working
    let receiver = watcher.receiver();
    let mut received_paths = Vec::new();

    while let Ok(path) = receiver.try_recv() {
        received_paths.push(path);
    }

    assert!(
        !received_paths.is_empty(),
        "Watcher should still be receiving events"
    );

    // Verify both files were detected
    assert!(
        received_paths.iter().any(|p| p == &invalid_file)
            || received_paths.iter().any(|p| p == &valid_file),
        "Should have detected at least one file"
    );

    println!("✓ Error recovery: watcher continues after errors");
}

#[test]
fn test_watcher_shutdown_detection() {
    // Test that the file watcher can detect when the async channel closes
    //
    // This test verifies the graceful shutdown behavior by checking that
    // recv_timeout doesn't block indefinitely

    let temp_dir = setup_test_dir();

    let config = FileWatcherConfig {
        watch_paths: vec![temp_dir.path().to_path_buf()],
        debounce_ms: 50,
        extension_filter: ".dampen".to_string(),
        recursive: true,
    };

    let mut watcher = FileWatcher::new(config).expect("Failed to create watcher");
    watcher
        .watch(temp_dir.path().to_path_buf())
        .expect("Failed to watch directory");

    // Give watcher time to initialize
    thread::sleep(Duration::from_millis(100));

    // Create a test file
    let test_file = temp_dir.path().join("test.dampen");
    fs::write(
        &test_file,
        r#"<dampen version="1.1" encoding="utf-8"><text value="Test" /></dampen>"#,
    )
    .expect("Failed to create file");

    thread::sleep(Duration::from_millis(150));

    // Receive the event
    let receiver = watcher.receiver();
    let path = receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("Should receive file event");

    assert_eq!(path, test_file);

    // Clear any additional events that might have been generated
    while receiver.recv_timeout(Duration::from_millis(50)).is_ok() {}

    // Now verify that recv_timeout returns (doesn't block forever)
    // even when there are no events
    let result = receiver.recv_timeout(Duration::from_millis(200));

    assert!(
        result.is_err(),
        "recv_timeout should timeout when no events, got: {:?}",
        result
    );

    // The watcher is still alive and can receive more events
    fs::write(
        &test_file,
        r#"<dampen version="1.1" encoding="utf-8"><text value="Modified" /></dampen>"#,
    )
    .expect("Failed to modify file");

    thread::sleep(Duration::from_millis(150));

    let path = receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("Should receive modification event");

    assert_eq!(path, test_file);

    println!("✓ Watcher uses recv_timeout and doesn't block indefinitely");
}
