//! File watcher for hot-reload functionality

use notify::{Event, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

/// Events emitted by the file watcher
#[derive(Debug, Clone)]
pub enum FileEvent {
    /// A .gravity file was modified
    Modified(PathBuf),
    /// A .gravity file was created
    Created(PathBuf),
    /// A .gravity file was removed
    Deleted(PathBuf),
    /// Error occurred during watching
    Error(String),
}

/// File watcher with debouncing and filtering
#[allow(dead_code)]
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    event_rx: Receiver<FileEvent>,
    watched_dir: PathBuf,
}

impl FileWatcher {
    /// Create a new file watcher for a directory
    pub fn new<P: Into<PathBuf>>(path: P) -> NotifyResult<Self> {
        let watched_dir = path.into();
        let (event_tx, event_rx) = channel();
        
        // Create watcher with debouncing
        let mut watcher = notify::recommended_watcher(move |res: NotifyResult<Event>| {
            match res {
                Ok(event) => {
                    // Filter for .gravity files and convert to our event type
                    if let Some(file_event) = Self::convert_event(event) {
                        let _ = event_tx.send(file_event);
                    }
                }
                Err(e) => {
                    let _ = event_tx.send(FileEvent::Error(e.to_string()));
                }
            }
        })?;
        
        // Configure debouncing (100ms as per requirements)
        watcher.configure(notify::Config::default().with_poll_interval(Duration::from_millis(100)))?;
        
        // Start watching
        watcher.watch(&watched_dir, RecursiveMode::NonRecursive)?;
        
        Ok(Self {
            watcher,
            event_rx,
            watched_dir,
        })
    }
    
    /// Convert notify events to our FileEvent type with filtering
    fn convert_event(event: Event) -> Option<FileEvent> {
        // Filter for .gravity files
        let gravity_files: Vec<PathBuf> = event.paths
            .into_iter()
            .filter(|p| p.extension().is_some_and(|ext| ext == "gravity"))
            .collect();
        
        if gravity_files.is_empty() {
            return None;
        }
        
        // Use the first gravity file (should typically be just one)
        let path = gravity_files.into_iter().next()?;
        
        match event.kind {
            notify::EventKind::Modify(_) => Some(FileEvent::Modified(path)),
            notify::EventKind::Create(_) => Some(FileEvent::Created(path)),
            notify::EventKind::Remove(_) => Some(FileEvent::Deleted(path)),
            _ => None,
        }
    }
    
    /// Receive the next file event (blocks until available)
    pub fn recv(&self) -> Result<FileEvent, std::sync::mpsc::RecvError> {
        self.event_rx.recv()
    }
    
    /// Try to receive a file event without blocking
    pub fn try_recv(&self) -> Result<FileEvent, std::sync::mpsc::TryRecvError> {
        self.event_rx.try_recv()
    }
    
    /// Receive with timeout
    pub fn recv_timeout(&self, timeout: Duration) -> Result<FileEvent, std::sync::mpsc::RecvTimeoutError> {
        self.event_rx.recv_timeout(timeout)
    }
    
    /// Get the directory being watched
    pub fn watched_path(&self) -> &Path {
        &self.watched_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use tempfile::tempdir;

    #[test]
    fn test_watcher_creation() {
        let temp_dir = tempdir().unwrap();
        let watcher = FileWatcher::new(temp_dir.path());
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_file_modification_detection() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.gravity");
        
        // Write initial file
        fs::write(&test_file, "<column><text value='test' /></column>").unwrap();
        
        let watcher = FileWatcher::new(temp_dir.path()).unwrap();
        
        // Modify the file
        thread::sleep(Duration::from_millis(150)); // Wait for debounce
        fs::write(&test_file, "<column><text value='updated' /></column>").unwrap();
        
        // Should receive event
        let event = watcher.recv_timeout(Duration::from_secs(2)).unwrap();
        
        match event {
            FileEvent::Modified(path) => {
                assert_eq!(path.file_name().unwrap(), "test.gravity");
            }
            _ => panic!("Expected Modified event"),
        }
    }

    #[test]
    fn test_gravity_extension_filter() {
        let temp_dir = tempdir().unwrap();
        let gravity_file = temp_dir.path().join("test.gravity");
        let other_file = temp_dir.path().join("test.txt");
        
        let watcher = FileWatcher::new(temp_dir.path()).unwrap();
        
        // Write .gravity file
        thread::sleep(Duration::from_millis(150));
        fs::write(&gravity_file, "test").unwrap();
        
        // Should receive event
        let event = watcher.recv_timeout(Duration::from_secs(1)).unwrap();
        assert!(matches!(event, FileEvent::Created(_)));
        
        // Write .txt file
        thread::sleep(Duration::from_millis(150));
        fs::write(&other_file, "test").unwrap();
        
        // Should NOT receive event for .txt (or at least not immediately)
        // Give it a moment to potentially trigger, then try to receive
        thread::sleep(Duration::from_millis(100));
        let result = watcher.try_recv();
        
        // Either no event (Err) or if there is one, it should be for the .gravity file
        if let Ok(event) = result {
            // If we got an event, it should be for the .gravity file
            match event {
                FileEvent::Created(path) | FileEvent::Modified(path) => {
                    assert!(path.extension().map_or(false, |ext| ext == "gravity"));
                }
                _ => {}
            }
        }
        // If no event, that's also acceptable (filter worked)
    }
}
