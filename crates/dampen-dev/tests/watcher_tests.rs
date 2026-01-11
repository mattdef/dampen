//! Integration tests for FileWatcher functionality
//!
//! These tests verify that the file watcher correctly detects file system events
//! with proper debouncing and filtering.

use dampen_dev::watcher::{FileWatcher, FileWatcherConfig};
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// Helper function to create a temporary directory for testing
fn setup_test_dir() -> TempDir {
    TempDir::new().expect("Failed to create temp directory")
}

/// Helper function to create a .dampen file in the test directory
fn create_dampen_file(dir: &TempDir, name: &str, content: &str) -> PathBuf {
    let file_path = dir.path().join(name);
    fs::write(&file_path, content).expect("Failed to write file");
    file_path
}

/// Helper function to modify a .dampen file
fn modify_dampen_file(path: &PathBuf, content: &str) {
    fs::write(path, content).expect("Failed to modify file");
}

/// Helper function to wait for debouncer to process events
fn wait_for_debounce() {
    thread::sleep(Duration::from_millis(150)); // Slightly longer than 100ms debounce
}

#[test]
fn test_file_creation_detection() {
    // T065: Test that the watcher detects when a new .dampen file is created

    // Setup
    let temp_dir = setup_test_dir();
    let config = FileWatcherConfig {
        watch_paths: vec![temp_dir.path().to_path_buf()],
        debounce_ms: 100,
        extension_filter: ".dampen".to_string(),
        recursive: true,
    };

    // Create watcher
    let mut watcher = FileWatcher::new(config).expect("Failed to create watcher");
    watcher
        .watch(temp_dir.path().to_path_buf())
        .expect("Failed to watch directory");

    // Give the watcher time to initialize
    thread::sleep(Duration::from_millis(50));

    // Create a new .dampen file
    let test_file = temp_dir.path().join("test.dampen");
    fs::write(&test_file, "<dampen version="1.0"><text value=\"Hello\" /></dampen>")
        .expect("Failed to create file");

    // Wait for debouncer to process the event
    wait_for_debounce();

    // Check that we received an event
    let receiver = watcher.receiver();
    let mut received_events = Vec::new();

    // Collect all available events
    while let Ok(path) = receiver.try_recv() {
        received_events.push(path);
    }

    // Verify we received at least one event
    assert!(
        !received_events.is_empty(),
        "Expected to receive file creation event, but got none"
    );

    // Verify the event is for our test file
    assert!(
        received_events.iter().any(|p| p == &test_file),
        "Expected event for {:?}, but received events for: {:?}",
        test_file,
        received_events
    );
}

#[test]
fn test_file_modification_detection() {
    // T066: Test that the watcher detects when an existing .dampen file is modified

    // Setup
    let temp_dir = setup_test_dir();

    // Create the file BEFORE starting the watcher
    let test_file = create_dampen_file(
        &temp_dir,
        "existing.dampen",
        "<dampen version="1.0"><text value=\"Original\" /></dampen>",
    );

    // Give filesystem time to settle
    thread::sleep(Duration::from_millis(50));

    let config = FileWatcherConfig {
        watch_paths: vec![temp_dir.path().to_path_buf()],
        debounce_ms: 100,
        extension_filter: ".dampen".to_string(),
        recursive: true,
    };

    // Create and start watcher
    let mut watcher = FileWatcher::new(config).expect("Failed to create watcher");
    watcher
        .watch(temp_dir.path().to_path_buf())
        .expect("Failed to watch directory");

    // Give the watcher time to initialize
    thread::sleep(Duration::from_millis(50));

    // Clear any initialization events
    let receiver = watcher.receiver();
    while receiver.try_recv().is_ok() {}

    // Modify the existing file
    modify_dampen_file(&test_file, "<dampen version="1.0"><text value=\"Modified\" /></dampen>");

    // Wait for debouncer to process the event
    wait_for_debounce();

    // Check that we received a modification event
    let mut received_events = Vec::new();
    while let Ok(path) = receiver.try_recv() {
        received_events.push(path);
    }

    // Verify we received at least one event
    assert!(
        !received_events.is_empty(),
        "Expected to receive file modification event, but got none"
    );

    // Verify the event is for our test file
    assert!(
        received_events.iter().any(|p| p == &test_file),
        "Expected event for {:?}, but received events for: {:?}",
        test_file,
        received_events
    );
}

#[test]
fn test_debouncing_behavior() {
    // T067: Test that rapid successive file changes are debounced
    // Multiple rapid changes should result in fewer events than the number of changes

    // Setup
    let temp_dir = setup_test_dir();

    let config = FileWatcherConfig {
        watch_paths: vec![temp_dir.path().to_path_buf()],
        debounce_ms: 100,
        extension_filter: ".dampen".to_string(),
        recursive: true,
    };

    // Create and start watcher BEFORE creating the file
    let mut watcher = FileWatcher::new(config).expect("Failed to create watcher");
    watcher
        .watch(temp_dir.path().to_path_buf())
        .expect("Failed to watch directory");

    // Give the watcher time to initialize
    thread::sleep(Duration::from_millis(100));

    // Create the test file now
    let test_file = temp_dir.path().join("debounce_test.dampen");
    fs::write(&test_file, "<dampen version="1.0"><text value=\"Original\" /></dampen>")
        .expect("Failed to create file");

    // Wait for creation event to be processed
    thread::sleep(Duration::from_millis(150));

    // Clear creation event
    let receiver = watcher.receiver();
    while receiver.try_recv().is_ok() {}

    // Make multiple rapid successive modifications (within the 100ms debounce window)
    const NUM_MODIFICATIONS: usize = 10;
    for i in 0..NUM_MODIFICATIONS {
        modify_dampen_file(
            &test_file,
            &format!("<dampen version="1.0"><text value=\"Change {}\" /></dampen>", i),
        );
        // Very small delay to ensure changes are registered, but stay within debounce window
        thread::sleep(Duration::from_millis(5));
    }

    // Wait for debouncer to process all events
    thread::sleep(Duration::from_millis(250)); // Wait longer than debounce window

    // Collect all events
    let mut received_events = Vec::new();
    while let Ok(path) = receiver.try_recv() {
        received_events.push(path);
    }

    // Verify debouncing: we should have received significantly fewer events than modifications
    assert!(
        !received_events.is_empty(),
        "Expected to receive at least one debounced event, but got none"
    );

    // The key assertion: debouncing should reduce the number of events
    // With 10 rapid modifications, we should get fewer events than modifications
    // The exact number depends on filesystem timing, but should be < NUM_MODIFICATIONS
    assert!(
        received_events.len() < NUM_MODIFICATIONS,
        "Expected debouncing to reduce {} modifications to fewer events, but got {} events. \
        Debouncing may not be working correctly.",
        NUM_MODIFICATIONS,
        received_events.len()
    );

    // Additional check: verify significant reduction (at least 30%)
    let reduction_percent =
        (1.0 - (received_events.len() as f64 / NUM_MODIFICATIONS as f64)) * 100.0;
    assert!(
        reduction_percent >= 30.0,
        "Expected at least 30% reduction from debouncing, but only got {:.1}% \
        ({} events from {} modifications)",
        reduction_percent,
        received_events.len(),
        NUM_MODIFICATIONS
    );

    // Verify all events are for our test file
    for event_path in &received_events {
        assert_eq!(
            event_path, &test_file,
            "Received unexpected event for {:?}",
            event_path
        );
    }

    println!(
        "✓ Debouncing working: {} modifications resulted in {} events (reduction: {:.1}%)",
        NUM_MODIFICATIONS,
        received_events.len(),
        (1.0 - (received_events.len() as f64 / NUM_MODIFICATIONS as f64)) * 100.0
    );
}

#[test]
fn test_extension_filtering() {
    // Bonus test: Verify that non-.dampen files are filtered out

    // Setup
    let temp_dir = setup_test_dir();
    let config = FileWatcherConfig {
        watch_paths: vec![temp_dir.path().to_path_buf()],
        debounce_ms: 100,
        extension_filter: ".dampen".to_string(),
        recursive: true,
    };

    // Create watcher
    let mut watcher = FileWatcher::new(config).expect("Failed to create watcher");
    watcher
        .watch(temp_dir.path().to_path_buf())
        .expect("Failed to watch directory");

    // Give the watcher time to initialize
    thread::sleep(Duration::from_millis(50));

    // Clear any initialization events
    let receiver = watcher.receiver();
    while receiver.try_recv().is_ok() {}

    // Create a .dampen file (should be detected)
    let dampen_file = temp_dir.path().join("should_detect.dampen");
    fs::write(&dampen_file, "<dampen />").expect("Failed to create .dampen file");

    // Create a non-.dampen file (should be filtered out)
    let txt_file = temp_dir.path().join("should_ignore.txt");
    fs::write(&txt_file, "Some text").expect("Failed to create .txt file");

    // Wait for debouncer
    wait_for_debounce();

    // Collect events
    let mut received_events = Vec::new();
    while let Ok(path) = receiver.try_recv() {
        received_events.push(path);
    }

    // Verify only .dampen file triggered an event
    assert!(
        received_events.iter().any(|p| p == &dampen_file),
        "Expected to receive event for .dampen file"
    );

    assert!(
        !received_events.iter().any(|p| p == &txt_file),
        "Should not receive event for .txt file (should be filtered)"
    );
}

#[test]
fn test_deleted_file_handling() {
    // Bonus test: Verify that deleted files don't cause errors (T064 validation)

    // Setup
    let temp_dir = setup_test_dir();
    let test_file = create_dampen_file(
        &temp_dir,
        "to_delete.dampen",
        "<dampen version="1.0"><text value=\"Will be deleted\" /></dampen>",
    );

    // Give filesystem time to settle
    thread::sleep(Duration::from_millis(50));

    let config = FileWatcherConfig {
        watch_paths: vec![temp_dir.path().to_path_buf()],
        debounce_ms: 100,
        extension_filter: ".dampen".to_string(),
        recursive: true,
    };

    // Create and start watcher
    let mut watcher = FileWatcher::new(config).expect("Failed to create watcher");
    watcher
        .watch(temp_dir.path().to_path_buf())
        .expect("Failed to watch directory");

    // Give the watcher time to initialize
    thread::sleep(Duration::from_millis(50));

    // Clear any initialization events
    let receiver = watcher.receiver();
    while receiver.try_recv().is_ok() {}

    // Delete the file
    fs::remove_file(&test_file).expect("Failed to delete file");

    // Wait for debouncer
    wait_for_debounce();

    // Collect events - deletion events should be filtered out (file no longer exists)
    let received_events: Vec<PathBuf> = receiver.try_iter().collect();

    // The watcher should handle deletion gracefully (no event sent for deleted file)
    // This validates T064 implementation
    println!(
        "✓ File deletion handled gracefully: {} events received for deleted file",
        received_events.iter().filter(|p| p == &&test_file).count()
    );

    // Note: We don't assert here because deletion event behavior varies by platform
    // The important thing is that it doesn't cause a panic or error
}

#[test]
fn test_file_change_detection_latency() {
    // T068: Verify file change detection <100ms (FR-010, SC-003)
    //
    // This test measures the end-to-end latency from file modification to event reception.
    // We use a minimal debounce window (10ms) to test the raw detection speed.

    // Setup
    let temp_dir = setup_test_dir();

    // Use minimal debounce for this performance test
    let config = FileWatcherConfig {
        watch_paths: vec![temp_dir.path().to_path_buf()],
        debounce_ms: 10, // Minimal debounce to test raw detection speed
        extension_filter: ".dampen".to_string(),
        recursive: true,
    };

    // Create and start watcher
    let mut watcher = FileWatcher::new(config).expect("Failed to create watcher");
    watcher
        .watch(temp_dir.path().to_path_buf())
        .expect("Failed to watch directory");

    // Give the watcher time to initialize
    thread::sleep(Duration::from_millis(100));

    // Create test file
    let test_file = temp_dir.path().join("latency_test.dampen");
    fs::write(&test_file, "<dampen version="1.0"><text value=\"Initial\" /></dampen>")
        .expect("Failed to create file");

    // Wait for creation event to be processed
    thread::sleep(Duration::from_millis(50));

    // Clear creation event
    let receiver = watcher.receiver();
    while receiver.try_recv().is_ok() {}

    // Perform 5 measurements and average them
    const NUM_MEASUREMENTS: usize = 5;
    let mut latencies = Vec::new();

    for i in 0..NUM_MEASUREMENTS {
        // Mark start time immediately before modification
        let start = Instant::now();

        // Modify the file
        modify_dampen_file(
            &test_file,
            &format!("<dampen version="1.0"><text value=\"Measurement {}\" /></dampen>", i),
        );

        // Wait for event with timeout
        let timeout = Duration::from_millis(200);
        let mut received = false;

        loop {
            if let Ok(_path) = receiver.try_recv() {
                let latency = start.elapsed();
                latencies.push(latency);
                received = true;
                break;
            }

            if start.elapsed() > timeout {
                break;
            }

            // Small sleep to avoid busy-waiting
            thread::sleep(Duration::from_millis(1));
        }

        assert!(
            received,
            "Measurement {}: Did not receive event within {}ms",
            i,
            timeout.as_millis()
        );

        // Small delay between measurements
        thread::sleep(Duration::from_millis(50));

        // Clear any additional events
        while receiver.try_recv().is_ok() {}
    }

    // Calculate statistics
    let average_latency = latencies.iter().sum::<Duration>() / latencies.len() as u32;
    let min_latency = latencies.iter().min().unwrap();
    let max_latency = latencies.iter().max().unwrap();

    // Print results
    println!("\n=== File Change Detection Latency (FR-010, SC-003) ===");
    println!("Measurements: {}", NUM_MEASUREMENTS);
    println!(
        "Average latency: {:.2}ms",
        average_latency.as_secs_f64() * 1000.0
    );
    println!("Min latency: {:.2}ms", min_latency.as_secs_f64() * 1000.0);
    println!("Max latency: {:.2}ms", max_latency.as_secs_f64() * 1000.0);
    println!(
        "All latencies: {:?}",
        latencies
            .iter()
            .map(|d| format!("{:.2}ms", d.as_secs_f64() * 1000.0))
            .collect::<Vec<_>>()
    );

    // Verify FR-010, SC-003: file change detection < 100ms
    // With minimal debounce (10ms), the detection should be very fast
    assert!(
        average_latency.as_millis() < 100,
        "FAILED: Average file change detection latency {:.2}ms exceeds 100ms requirement (FR-010, SC-003)",
        average_latency.as_secs_f64() * 1000.0
    );

    // Additional check: max latency should also be reasonable
    assert!(
        max_latency.as_millis() < 150,
        "FAILED: Maximum latency {:.2}ms is too high (should be < 150ms)",
        max_latency.as_secs_f64() * 1000.0
    );

    println!("✓ PASSED: File change detection latency meets <100ms requirement");
}
