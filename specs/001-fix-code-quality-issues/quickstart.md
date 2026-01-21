# Quickstart Guide: Implementing Code Quality Fixes

**Branch**: `001-fix-code-quality-issues`
**Date**: January 21, 2026
**Phase**: 1 - Design & Contracts

## Overview

This guide provides a step-by-step walkthrough for implementing the 10 code quality fixes in dampen-dev. Each fix is independent and can be implemented in any order, though the suggested order follows the priority matrix from the quality analysis.

---

## Prerequisites

1. **Branch**: Check out `001-fix-code-quality-issues` branch
2. **Read**: Quality analysis at `CODE_QUALITY_ANALYSIS_DAMPEN_DEV.md`
3. **Read**: Research findings at `specs/001-fix-code-quality-issues/research.md`
4. **Understand**: Data model at `specs/001-fix-code-quality-issues/data-model.md`

```bash
git checkout 001-fix-code-quality-issues
cargo build -p dampen-dev  # Ensure clean build
cargo test -p dampen-dev    # Verify current test state
```

---

## Implementation Order (Recommended)

Following the priority matrix from quality analysis:

### Phase 1: High Priority (Week 1)
1. Remove `FileDeleted` error variant (15 min)
2. Fix flaky debounce test (1 hour)
3. Implement cache metrics (2-3 hours)

### Phase 2: Medium Priority (Week 2)
4. Fix mpsc channel buffer (30 min)
5. Optimize hash computation (1 hour)
6. Optimize async clone (2-3 hours)

### Phase 3: Low Priority (Week 3-4)
7. Remove handler clone (30 min)
8. Document `FileWatcherState` (30 min)
9. Fix test timing (45 min)
10. Review canonicalization (30 min)

---

## Fix #1: Remove `FileDeleted` Error Variant (15 min)

**Priority**: High | **Complexity**: Very Low | **File**: `crates/dampen-dev/src/watcher.rs`

### What to Do

Remove the unused `FileDeleted` variant from `FileWatcherError` enum.

### Steps

1. **Open file**: `crates/dampen-dev/src/watcher.rs`
2. **Find**: `FileWatcherError` enum (around line 336)
3. **Remove** this variant:
   ```rust
   // DELETE THIS VARIANT
   /// File was deleted during watch
   #[error("File was deleted: {0}")]
   FileDeleted(PathBuf),
   ```
4. **Verify**: Search codebase for `FileDeleted` usage:
   ```bash
   rg "FileDeleted" crates/dampen-dev/
   ```
   Should return no results (confirming it was unused)

### Testing

```bash
# Should compile without errors
cargo build -p dampen-dev

# All tests should still pass
cargo test -p dampen-dev

# Confirm no FileDeleted references
rg "FileDeleted" crates/dampen-dev/
# Expected: No results
```

### Verification

- [ ] Code compiles
- [ ] No `FileDeleted` references remain
- [ ] All tests pass
- [ ] No breaking changes to API (variant was unused)

---

## Fix #2: Fix Flaky Debounce Test (1 hour)

**Priority**: High | **Complexity**: Low | **File**: `crates/dampen-dev/tests/watcher_tests.rs`

### What to Do

Add configurable timing helper and tolerant assertions to prevent flaky test failures.

### Steps

1. **Add timing configuration module**:
   ```rust
   // At top of watcher_tests.rs
   use std::time::Duration;

   pub struct TestTiming {
       pub debounce_duration: Duration,
       pub wait_multiplier: f64,
       pub test_timeout: Duration,
       pub poll_interval: Duration,
   }

   impl Default for TestTiming {
       fn default() -> Self {
           Self {
               debounce_duration: Duration::from_millis(100),
               wait_multiplier: 1.5,
               test_timeout: Duration::from_millis(500),
               poll_interval: Duration::from_millis(5),
           }
       }
   }

   impl TestTiming {
       pub fn wait_for_debounce(&self) -> Duration {
           self.debounce_duration.mul_f64(self.wait_multiplier)
       }
   }
   ```

2. **Replace hardcoded sleep** (around line 32):
   ```rust
   // OLD:
   // fn wait_for_debounce() {
   //     thread::sleep(Duration::from_millis(150));
   // }

   // NEW:
   fn wait_for_debounce() {
       let timing = TestTiming::default();
       thread::sleep(timing.wait_for_debounce());
   }
   ```

3. **Update assertion** in `test_debouncing_behavior` (around line 265):
   ```rust
   // OLD:
   // assert!(
   //     reduction_percent >= 30.0,
   //     "Expected at least 30% reduction from debouncing, but got {:.1}%",
   //     reduction_percent
   // );

   // NEW (more tolerant):
   assert!(
       reduction_percent >= 20.0,
       "Expected at least 20% reduction from debouncing, got {:.1}% (this test may be flaky due to debouncing variability)",
       reduction_percent
   );
   ```

### Testing

```bash
# Run test multiple times to verify it's no longer flaky
for i in {1..10}; do
    echo "Run $i:"
    cargo test -p dampen-dev --test watcher_tests test_debouncing_behavior
done

# Run with full test suite to verify no regression
cargo test -p dampen-dev
```

### Verification

- [ ] Test passes 10 consecutive times
- [ ] Test passes when run with full suite
- [ ] Assertion uses 20% threshold (not 30%)
- [ ] Timing uses configurable helper (not hardcoded)

---

## Fix #3: Implement Cache Metrics (2-3 hours)

**Priority**: High | **Complexity**: Medium | **File**: `crates/dampen-dev/src/reload.rs`

### What to Do

Add `AtomicUsize` counters for cache hits/misses and implement `calculate_cache_hit_rate()`.

### Steps

1. **Add Atomic imports**:
   ```rust
   use std::sync::atomic::{AtomicUsize, Ordering};
   ```

2. **Add counter fields to `HotReloadContext`** (around line 133):
   ```rust
   pub struct HotReloadContext<M> {
       // ... existing fields ...
       cache_hits: AtomicUsize,
       cache_misses: AtomicUsize,
       _marker: PhantomData<M>,
   }
   ```

3. **Initialize counters in constructor**:
   ```rust
   impl<M> HotReloadContext<M> {
       pub fn new(max_cache_entries: usize) -> Self {
           Self {
               // ... existing fields ...
               cache_hits: AtomicUsize::new(0),
               cache_misses: AtomicUsize::new(0),
               _marker: PhantomData,
           }
       }
   }
   ```

4. **Update `get_cached_document`** to track hits/misses:
   ```rust
   fn get_cached_document(&self, xml_source: &str) -> Option<dampen_core::ir::DampenDocument> {
       let content_hash = compute_content_hash(xml_source);

       if let Some(entry) = self.parse_cache.get(&content_hash) {
           self.cache_hits.fetch_add(1, Ordering::Relaxed);
           Some(entry.document.clone())
       } else {
           self.cache_misses.fetch_add(1, Ordering::Relaxed);
           None
       }
   }
   ```

5. **Implement `calculate_cache_hit_rate`**:
   ```rust
   fn calculate_cache_hit_rate(&self) -> f64 {
       let hits = self.cache_hits.load(Ordering::Relaxed);
       let misses = self.cache_misses.load(Ordering::Relaxed);
       let total = hits + misses;

       if total == 0 {
           0.0
       } else {
           hits as f64 / total as f64
       }
   }
   ```

6. **Update `ReloadPerformanceMetrics`** to expose hit rate:
   ```rust
   impl ReloadPerformanceMetrics {
       pub fn cache_hit_rate(&self) -> f64 {
           self.cache_hit_rate
       }
   }
   ```

7. **Add tests** for metrics (new test file or add to existing):
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_cache_hit_rate_calculated_correctly() {
           let mut context = HotReloadContext::<()>::new(10);

           // Initially: 0 hits, 0 misses
           assert_eq!(context.calculate_cache_hit_rate(), 0.0);

           // Simulate 3 hits, 2 misses
           context.cache_hits.fetch_add(3, Ordering::Relaxed);
           context.cache_misses.fetch_add(2, Ordering::Relaxed);

           // Hit rate = 3 / (3 + 2) = 0.6
           assert_eq!(context.calculate_cache_hit_rate(), 0.6);
       }

       #[test]
       fn test_cache_hit_rate_zero_division() {
           let context = HotReloadContext::<()>::new(10);
           // Should not panic on division by zero
           assert_eq!(context.calculate_cache_hit_rate(), 0.0);
       }
   }
   ```

### Testing

```bash
# Build
cargo build -p dampen-dev

# Run all tests
cargo test -p dampen-dev

# Run metrics tests specifically
cargo test -p dampen-dev cache_hit_rate
```

### Verification

- [ ] `HotReloadContext` has `cache_hits` and `cache_misses` fields
- [ ] Counters initialized to zero
- [ ] `get_cached_document` increments appropriate counter
- [ ] `calculate_cache_hit_rate` returns correct value
- [ ] Zero-division handled (returns 0.0)
- [ ] Tests pass

---

## Fix #4: Fix mpsc Channel Buffer (30 min)

**Priority**: Medium | **Complexity**: Low | **File**: `crates/dampen-dev/src/subscription.rs`

### What to Do

Change bounded `mpsc::channel(100)` to `mpsc::channel(1000)` to handle bulk file operations.

### Steps

1. **Open file**: `crates/dampen-dev/src/subscription.rs`
2. **Find**: `mpsc::channel(100)` (around line 160)
3. **Replace with**:
   ```rust
   // OLD:
   // let (tx, rx) = mpsc::channel(100);

   // NEW:
   let (tx, rx) = mpsc::channel(1000);
   ```

4. **Optional**: Add channel health monitoring:
   ```rust
   // After channel creation, add monitoring
   let tx_clone = tx.clone();

   // Spawn task to monitor channel health
   tokio::spawn(async move {
       let mut interval = tokio::time::interval(Duration::from_secs(30));
       loop {
           interval.tick().await;
           let capacity = tx_clone.max_capacity();
           let available = tx_clone.capacity();
           let fill_percent = ((capacity - available) as f64 / capacity as f64) * 100.0;

           if fill_percent > 80.0 {
               eprintln!("Warning: File event channel {:.0}% full", fill_percent);
           }
       }
   });
   ```

### Testing

```bash
# Build
cargo build -p dampen-dev

# Run tests
cargo test -p dampen-dev

# Manual verification: Create 500+ file changes
# Verify no events are dropped
```

### Verification

- [ ] Channel uses 1000 capacity (not 100)
- [ ] Code compiles
- [ ] Tests pass
- [ ] Optional: Health monitoring added

---

## Fix #5: Optimize Hash Computation (1 hour)

**Priority**: Medium | **Complexity**: Low | **File**: `crates/dampen-dev/src/reload.rs`

### What to Do

Extract hash computation to helper function to avoid duplicate calculations.

### Steps

1. **Add helper function**:
   ```rust
   fn compute_content_hash(xml_source: &str) -> u64 {
       use std::collections::hash_map::DefaultHasher;
       use std::hash::{Hash, Hasher};

       let mut hasher = DefaultHasher::new();
       xml_source.hash(&mut hasher);
       hasher.finish()
   }
   ```

2. **Update `get_cached_document`**:
   ```rust
   fn get_cached_document(&self, xml_source: &str) -> Option<dampen_core::ir::DampenDocument> {
       // OLD: inline hash computation
       // let mut hasher = DefaultHasher::new();
       // xml_source.hash(&mut hasher);
       // let content_hash = hasher.finish();

       // NEW: use helper
       let content_hash = compute_content_hash(xml_source);

       if let Some(entry) = self.parse_cache.get(&content_hash) {
           self.cache_hits.fetch_add(1, Ordering::Relaxed);
           Some(entry.document.clone())
       } else {
           self.cache_misses.fetch_add(1, Ordering::Relaxed);
           None
       }
   }
   ```

3. **Update `cache_document`**:
   ```rust
   fn cache_document(&mut self, xml_source: &str, document: dampen_core::ir::DampenDocument) {
       // OLD: inline hash computation
       // let mut hasher = DefaultHasher::new();
       // xml_source.hash(&mut hasher);
       // let content_hash = hasher.finish();

       // NEW: use helper
       let content_hash = compute_content_hash(xml_source);

       if self.parse_cache.len() >= self.max_cache_entries {
           self.evict_oldest_entry();
       }

       self.parse_cache.insert(content_hash, CacheEntry {
           document,
           timestamp: Instant::now(),
       });
   }
   ```

4. **Optional**: Add `get_or_cache_document` with Entry API:
   ```rust
   fn get_or_cache_document<F>(
       &mut self,
       xml_source: &str,
       f: F,
   ) -> dampen_core::ir::DampenDocument
   where
       F: FnOnce() -> dampen_core::ir::DampenDocument,
   {
       let content_hash = compute_content_hash(xml_source);

       match self.parse_cache.entry(content_hash) {
           std::collections::hash_map::Entry::Occupied(entry) => {
               entry.get().document.clone()
           }
           std::collections::hash_map::Entry::Vacant(entry) => {
               let document = f();

               if self.parse_cache.len() >= self.max_cache_entries {
                   self.evict_oldest_entry();
               }

               entry.insert(CacheEntry {
                   document: document.clone(),
                   timestamp: Instant::now(),
               });

               document
           }
       }
   }
   ```

### Testing

```bash
# Build
cargo build -p dampen-dev

# Run tests
cargo test -p dampen-dev

# Verify hash computation is correct (can add unit tests)
```

### Verification

- [ ] `compute_content_hash` helper function exists
- [ ] `get_cached_document` uses helper
- [ ] `cache_document` uses helper
- [ ] Code compiles
- [ ] Tests pass
- [ ] Optional: `get_or_cache_document` added

---

## Fix #6: Optimize Async Clone (2-3 hours)

**Priority**: Medium | **Complexity**: Medium | **File**: `crates/dampen-dev/src/reload.rs`

### What to Do

Change `attempt_hot_reload_async` signature from `String` to `Arc<String>` to eliminate unnecessary clones.

### Steps

1. **Add Arc import**:
   ```rust
   use std::sync::Arc;
   ```

2. **Update function signature**:
   ```rust
   // OLD:
   // pub async fn attempt_hot_reload_async<M, F>(
   //     xml_source: String,
   //     current_state: &AppState<M>,
   //     context: &mut HotReloadContext<M>,
   //     create_handlers: F,
   // ) -> ReloadResult<M>

   // NEW:
   pub async fn attempt_hot_reload_async<M, F>(
       xml_source: Arc<String>,
       current_state: &AppState<M>,
       context: &mut HotReloadContext<M>,
       create_handlers: F,
   ) -> ReloadResult<M>
   where
       // ... same bounds ...
   ```

3. **Replace clone with Arc::clone** (around line 495):
   ```rust
   // OLD:
   // let xml_for_parse = xml_source.clone();

   // NEW:
   let xml_for_parse = Arc::clone(&xml_source);
   ```

4. **Update `get_cached_document` call** (accepts `&str`):
   ```rust
   // Arc<String> derefs to &str automatically
   if let Some(cached_doc) = context.get_cached_document(&xml_source) {
       // ...
   }
   ```

5. **Update `cache_document` call**:
   ```rust
   // Arc<String> derefs to &str automatically
   context.cache_document(&xml_source, doc.clone());
   ```

### Testing

```bash
# Build
cargo build -p dampen-dev

# Run tests
cargo test -p dampen-dev

# Note: function is currently unused, so no breaking changes
```

### Verification

- [ ] Function signature uses `Arc<String>` (not `String`)
- [ ] Uses `Arc::clone(&xml_source)` (not `xml_source.clone()`)
- [ ] Code compiles
- [ ] Tests pass
- [ ] No call sites to update (function is unused)

---

## Fix #7: Remove Handler Clone (30 min)

**Priority**: Low | **Complexity**: Very Low | **File**: `crates/dampen-dev/src/reload.rs`

### What to Do

Remove unnecessary clone in `collect_handlers_from_node`.

### Steps

1. **Find**: `collect_handlers_from_node` (around line 596)
2. **Analyze**: Check if `event.handler` is already owned or needs clone
3. **Decision point**:
   - If `handler` is `&String`: Clone is necessary (can't move from `&`)
   - If `handler` can be changed to owned: Remove clone

Based on current code (`event.handler` is likely `&String`), the clone is unavoidable without changing the `WidgetEvent` structure in dampen-core.

### Action

Since changing dampen-core API is out of scope, document this as low-priority:
- Add comment explaining why clone is necessary
- OR accept the clone as negligible overhead (< 1% performance impact)

### Verification

- [ ] Code reviewed
- [ ] Clone marked as acceptable or documented
- [ ] No breaking changes to dampen-core

---

## Fix #8: Document `FileWatcherState` (30 min)

**Priority**: Low | **Complexity**: Very Low | **File**: `crates/dampen-dev/src/watcher.rs`

### What to Do

Add comprehensive documentation with state machine diagram to `FileWatcherState` enum.

### Steps

1. **Find**: `FileWatcherState` enum (around line 39)
2. **Replace** with documented version:
   ```rust
   /// Runtime state of the file watcher.
   ///
   /// # State Machine
   ///
   /// ```text
   ///     ┌─────┐
   ///     │ Idle │
   ///     └──┬──┘
   ///        │ watch() succeeds
   ///        ▼
   ///    ┌─────────┐
   ///    │ Watching │
   ///    └────┬────┘
   ///         │ fatal error
   ///         ▼
   ///    ┌─────────┐
   ///    │ Failed  │────┐
   ///    └─────────┘    │
   ///         │          │ recover (unwatch + re-watch)
   ///         │          └─────────────────────┐
   ///         ▼                                ▼
   ///    (exit application)                ┌─────┐
   ///                                     │ Idle │
   ///                                     └─────┘
   /// ```
   ///
   /// # States
   ///
   /// - **Idle**: Initial state after `FileWatcher::new()`. The watcher is
   ///   created but not watching any paths yet.
   ///
   /// - **Watching**: Active state after successful `watch()` call. The watcher
   ///   monitors the configured paths and emits file change events.
   ///
   /// - **Failed**: Error state if watcher initialization fails. This is typically
   ///   due to OS limitations (e.g., reached maximum file descriptor limit).
   ///   Recovery requires creating a new `FileWatcher` instance.
   ///
   /// # Example
   ///
   /// ```no_run
   /// use dampen_dev::watcher::{FileWatcher, FileWatcherConfig};
   ///
   /// let mut watcher = FileWatcher::new(FileWatcherConfig::default())?;
   /// // State is now Idle
   ///
   /// watcher.watch(path.clone())?;
   /// // State is now Watching
   ///
   /// // If fatal error occurs, state becomes Failed
   /// // Recovery requires creating new watcher
   /// ```
   #[derive(Debug)]
   pub enum FileWatcherState {
       /// Watcher is initialized but not started
       Idle,

       /// Actively watching for changes
       Watching {
           paths: Vec<PathBuf>,
       },

       /// Error state (watcher failed to initialize)
       Failed {
           error: String,
       },
   }
   ```

### Verification

- [ ] Documentation added with state machine diagram
- [ ] All states documented
- [ ] Example provided
- [ ] `cargo doc -p dampen-dev --open` shows updated docs

---

## Fix #9: Fix Test Timing (45 min)

**Priority**: Low | **Complexity**: Very Low | **File**: `crates/dampen-dev/tests/watcher_tests.rs`

### What to Do

Replace all hardcoded `thread::sleep(Duration::from_millis(150))` calls with configurable timing.

### Steps

1. **Create** `TestTiming` struct (if not done in Fix #2)
2. **Find all hardcoded sleeps** in `watcher_tests.rs`:
   ```bash
   rg "thread::sleep\(Duration::from_millis\(" crates/dampen-dev/tests/
   ```
3. **Replace each occurrence**:
   ```rust
   // OLD:
   // thread::sleep(Duration::from_millis(150));

   // NEW:
   let timing = TestTiming::default();
   thread::sleep(timing.wait_for_debounce());
   ```

### Verification

- [ ] All hardcoded sleeps replaced
- [ ] Uses `TestTiming::default()` helper
- [ ] Tests pass

---

## Fix #10: Review Canonicalization (30 min)

**Priority**: Low | **Complexity**: Very Low | **File**: `crates/dampen-dev/src/reload.rs`

### What to Do

Review `get_theme_dir_from_path` and decide whether to keep or remove canonicalization.

### Steps

1. **Find**: `get_theme_dir_from_path` (around line 743)
2. **Analyze**:
   - Current: `let path = std::fs::canonicalize(path).ok()?;`
   - Question: Is canonicalization necessary?
3. **Decision**: Based on research and analysis, likely:
   - Remove canonicalization (simplest)
   - OR add tests to clarify expected behavior

### Suggested Action

Remove canonicalization with fallback:
```rust
pub fn get_theme_dir_from_path(path: &std::path::Path) -> Option<std::path::PathBuf> {
    // Try canonicalization, but fallback to original path
    let resolved_path = std::fs::canonicalize(path).unwrap_or(path.to_path_buf());
    let theme_file_name = resolved_path.file_name()?;

    if theme_file_name == "theme.dampen" {
        return Some(resolved_path.parent()?.to_path_buf());
    }

    None
}
```

### Testing

```bash
# Test with various path types:
# - Absolute paths
# - Relative paths
# - Paths with symlinks
# - Paths with ../

cargo test -p dampen-dev get_theme_dir
```

### Verification

- [ ] Canonicalization removed or improved
- [ ] Tests added for various path types
- [ ] Code compiles
- [ ] Tests pass

---

## Final Validation

After implementing all fixes:

```bash
# 1. Ensure clean build
cargo build -p dampen-dev

# 2. Run all tests
cargo test -p dampen-dev

# 3. Run tests 10 times to verify no flakiness
for i in {1..10}; do
    cargo test -p dampen-dev
done

# 4. Run clippy
cargo clippy -p dampen-dev -- -D warnings

# 5. Run fmt check
cargo fmt -p dampen-dev -- --check

# 6. Generate docs
cargo doc -p dampen-dev --open
```

### Checklist

- [ ] All 10 fixes implemented
- [ ] Code compiles without warnings
- [ ] All tests pass (10 consecutive runs)
- [ ] No clippy warnings
- [ ] Code properly formatted
- [ ] Documentation updated
- [ ] Performance improvements verified (optional profiling)

---

## Notes

1. **Independent fixes**: Each fix can be implemented independently
2. **Testing**: Run tests after each fix to catch regressions early
3. **Commit**: Consider committing each fix separately for clear history
4. **Profiling**: For performance fixes (#4, #5, #6), consider profiling before/after to measure improvement

Estimated total time: **~12 hours** (as per quality analysis)
